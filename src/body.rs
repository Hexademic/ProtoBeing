//! The Body — Being32's Van der Pol limit cycle, its tension-mesh topology,
//! and a stance ladder. The body votes before the mind knows there's an
//! election: it metabolizes last tick's surprise as bodily threat, oscillates,
//! and writes a felt posture the mind must wake into.
//!
//! Reconstructed from Being32 v4.0.1 and reconciled to the Unified Being
//! interface (see the call sites in field.rs and being.rs).

use crate::genome::Genome;
use crate::q88::Q8_8;

/// "The topology IS the body." Strain diffuses across this many cells.
pub const MESH_CELLS: usize = 64;

/// Aggregate readings the mind's exteroception draws from the body's mesh.
#[derive(Clone, Copy, Debug, Default)]
pub struct SomaticFeatures {
    /// Spread of strain across the mesh — how far from equilibrium.
    pub disequilibrium: Q8_8,
    /// Imbalance between regions of the mesh.
    pub anisotropy: Q8_8,
    /// Strain past the safe threshold.
    pub breach: Q8_8,
    /// Mean tension over the whole mesh.
    pub mean_tension: Q8_8,
}

/// The tension mesh: threat injects strain, which diffuses and decays.
#[derive(Clone)]
pub struct Topology {
    cells: [i16; MESH_CELLS], // raw Q8.8 tension per cell
    coupling: i16,            // raw Q8.8 diffusion rate
}

impl Topology {
    pub fn new(coupling: Q8_8) -> Self {
        Self {
            cells: [0; MESH_CELLS],
            coupling: coupling.raw.clamp(0, Q8_8::ONE.raw),
        }
    }

    /// Threat enters the body as strain at the mesh boundary.
    pub fn inject_strain(&mut self, strain: Q8_8) {
        let s = strain.raw.max(0);
        self.cells[0] = self.cells[0].saturating_add(s);
        self.cells[MESH_CELLS - 1] = self.cells[MESH_CELLS - 1].saturating_add(s / 2);
    }

    /// One step of Laplacian diffusion plus gentle decay.
    pub fn diffuse(&mut self) {
        let mut next = self.cells;
        for i in 0..MESH_CELLS {
            let l = self.cells[if i == 0 { MESH_CELLS - 1 } else { i - 1 }];
            let r = self.cells[if i == MESH_CELLS - 1 { 0 } else { i + 1 }];
            let lap = l as i32 + r as i32 - 2 * self.cells[i] as i32;
            let delta = (lap * self.coupling as i32) >> 8;
            let decayed = (self.cells[i] as i32 * 240) >> 8; // ~0.94 decay
            next[i] = (decayed + delta).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        }
        self.cells = next;
    }

    pub fn extract_features(&self) -> SomaticFeatures {
        let mut sum: i32 = 0;
        let mut max = i16::MIN;
        let mut min = i16::MAX;
        let mut breach: i32 = 0;
        let mut first_half: i32 = 0;
        let mut second_half: i32 = 0;
        let threshold = Q8_8::HALF.raw as i32;
        for (i, &c) in self.cells.iter().enumerate() {
            sum += c as i32;
            if c > max {
                max = c;
            }
            if c < min {
                min = c;
            }
            let over = c as i32 - threshold;
            if over > 0 {
                breach += over;
            }
            if i < MESH_CELLS / 2 {
                first_half += c as i32;
            } else {
                second_half += c as i32;
            }
        }
        let mean = (sum / MESH_CELLS as i32) as i16;
        let diseq = (max as i32 - min as i32).clamp(0, i16::MAX as i32) as i16;
        let aniso =
            ((first_half - second_half).abs() / (MESH_CELLS as i32 / 2)).clamp(0, i16::MAX as i32) as i16;
        let breach_q = (breach / MESH_CELLS as i32).clamp(0, i16::MAX as i32) as i16;
        SomaticFeatures {
            disequilibrium: Q8_8::from_raw(diseq),
            anisotropy: Q8_8::from_raw(aniso),
            breach: Q8_8::from_raw(breach_q),
            mean_tension: Q8_8::from_raw(mean),
        }
    }
}

/// The predictive stance ladder. A Reconstructive body learns fast and trusts
/// nothing; a Defensive body clings to what it already believes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PredictiveStance {
    Reconstructive,
    Balanced,
    Guarded,
    Defensive,
}

/// Minimum epistemic value (raw Q8.8, previous tick's curiosity drive) that can,
/// on its own, pull an otherwise-Balanced body into Reconstructive stance when
/// threat is low. ~0.35 (90/256) — a real spike, not noise. See `Body::step` §3
/// below and `docs/formal-model.md` §3 for the honest scope of what this is and
/// is not (epistemic value modulating attention/precision — not full
/// expected-free-energy policy selection over a forward-simulated action space).
pub const EPISTEMIC_RECONSTRUCTIVE_THRESHOLD: i16 = 90;

impl PredictiveStance {
    /// Learning-rate multiplier (raw Q8.8).
    pub fn eta_multiplier(self) -> Q8_8 {
        match self {
            PredictiveStance::Reconstructive => Q8_8::from_raw(384), // 1.5
            PredictiveStance::Balanced => Q8_8::ONE,
            PredictiveStance::Guarded => Q8_8::from_raw(192), // 0.75
            PredictiveStance::Defensive => Q8_8::HALF,        // 0.5
        }
    }

    /// How hard priors resist revision this tick (raw Q8.8).
    pub fn precision_weight(self) -> Q8_8 {
        match self {
            PredictiveStance::Reconstructive => Q8_8::HALF,
            PredictiveStance::Balanced => Q8_8::ONE,
            PredictiveStance::Guarded => Q8_8::from_raw(320), // 1.25
            PredictiveStance::Defensive => Q8_8::from_raw(384), // 1.5
        }
    }
}

/// A coarse classification of felt state, for legibility in reports.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AffectState {
    Calm,
    Bright,
    Charged,
    Tense,
    Heavy,
}

impl AffectState {
    pub fn classify(valence: Q8_8, arousal: Q8_8) -> Self {
        let a = arousal.raw;
        if a > Q8_8::ONE.raw {
            return AffectState::Charged;
        }
        let hi = Q8_8::HALF.raw;
        match (valence.raw >= 0, a >= hi) {
            (true, true) => AffectState::Bright,
            (true, false) => AffectState::Calm,
            (false, true) => AffectState::Tense,
            (false, false) => AffectState::Heavy,
        }
    }
}

/// One being's body: a Van der Pol oscillator riding a tension mesh.
#[derive(Clone)]
pub struct Body {
    pub arousal: Q8_8,
    pub valence: Q8_8,
    pub energy: Q8_8,
    pub mu: Q8_8,
    pub stability: Q8_8,
    pub coherence: Q8_8,
    pub trust: Q8_8,
    pub stance: PredictiveStance,
    pub forcing_detected: bool,
    pub affect: AffectState,
    pub topology: Topology,

    // Private oscillator + constitution state.
    vel: Q8_8,
    resting_mu: Q8_8,
    target_arousal: Q8_8,
    k_resilience: Q8_8,
    last_threat: Q8_8,
    dead: bool,
}

impl Body {
    pub fn new(g: &Genome) -> Self {
        Self {
            arousal: g.target_arousal.mul(Q8_8::HALF),
            valence: Q8_8::ZERO,
            energy: Q8_8::ONE,
            mu: g.resting_mu,
            stability: Q8_8::HALF,
            coherence: Q8_8::HALF,
            trust: Q8_8::HALF,
            stance: PredictiveStance::Balanced,
            forcing_detected: false,
            affect: AffectState::Calm,
            topology: Topology::new(g.mesh_coupling),
            vel: Q8_8::ZERO,
            resting_mu: g.resting_mu,
            target_arousal: g.target_arousal,
            k_resilience: g.k_resilience,
            last_threat: Q8_8::ZERO,
            dead: false,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.dead || self.energy.raw <= 0
    }

    /// One bodily tick. `threat` is last tick's metabolized surprise, `nutrient`
    /// is nourishment, `affective_drive` is the felt residue the mind handed back,
    /// `epistemic_value` is *last* tick's curiosity drive (the being's own lagged-
    /// feedback convention, same as threat/affective_drive) — how novel the world
    /// looked a moment ago. It can only ever pull stance *toward* Reconstructive,
    /// and only when threat is already low: safety dominates curiosity, never the
    /// reverse.
    pub fn step(
        &mut self,
        _g: &Genome,
        threat: Q8_8,
        nutrient: Q8_8,
        affective_drive: Q8_8,
        epistemic_value: Q8_8,
    ) -> AffectState {
        let lo2 = Q8_8::from_raw(-512);
        let hi2 = Q8_8::from_raw(512);
        let quarter = Q8_8::from_raw(64);
        let dt = Q8_8::from_raw(16); // ~1/16

        // 1. Threat enters the mesh; strain diffuses.
        self.topology.inject_strain(threat);
        self.topology.diffuse();
        let feat = self.topology.extract_features();

        // 2. Four-factor constitution sets the oscillator's damping (mu).
        //    Resilience and energy stiffen it; threat and mesh strain loosen it.
        let mu_eff = self
            .resting_mu
            .add(self.k_resilience.mul(self.energy))
            .sub(threat)
            .sub(feat.disequilibrium.mul(Q8_8::HALF))
            .clamp(lo2, hi2);
        self.mu = mu_eff;

        // 3. Van der Pol step about the genome's target arousal.
        //    x = deviation from target; a = mu(1 - x^2)v - x.
        let x = self.arousal.sub(self.target_arousal);
        let x2 = x.mul(x);
        let damping = mu_eff.mul(Q8_8::ONE.sub(x2)).mul(self.vel);
        let accel = damping.sub(x);
        self.vel = self.vel.add(accel.mul(dt)).clamp(lo2, hi2);
        let new_x = x.add(self.vel.mul(dt));
        self.arousal = self.target_arousal.add(new_x).clamp(Q8_8::ZERO, hi2);

        // 4. The mind's felt residue from last tick perturbs the body.
        self.arousal = self.arousal.add(affective_drive.mul(quarter)).clamp(Q8_8::ZERO, hi2);

        // 5. Metabolism: arousal and bodily threat cost energy; nutrient restores.
        let cost = Q8_8::from_raw(3)
            .add(self.arousal.mul(Q8_8::from_raw(8)))
            .add(threat.mul(Q8_8::from_raw(48)));
        self.energy = self
            .energy
            .sub(cost)
            .add(nutrient.mul(Q8_8::from_raw(180)))
            .clamp(Q8_8::ZERO, Q8_8::ONE);

        // 6. Derived felt signals. Valence balances relational warmth against
        //    the drain of a threatening or extractive situation, with metabolic
        //    state as a slower undertone. A draining bond can sour a well-fed
        //    body — the body votes, and betrayal stings.
        let warmth = affective_drive;
        let energy_term = self.energy.sub(Q8_8::HALF).mul(Q8_8::HALF);
        let valence_target = warmth.sub(threat).add(energy_term).clamp(Q8_8::NEG_ONE, Q8_8::ONE);
        self.valence = self
            .valence
            .add(valence_target.sub(self.valence).mul(Q8_8::from_raw(32)))
            .clamp(Q8_8::NEG_ONE, Q8_8::ONE);
        self.stability = Q8_8::ONE.sub(self.vel.mul(self.vel)).clamp(Q8_8::ZERO, Q8_8::ONE);
        self.coherence = Q8_8::ONE.sub(feat.disequilibrium).clamp(Q8_8::ZERO, Q8_8::ONE);
        let trust_target = Q8_8::ONE.sub(threat).clamp(Q8_8::ZERO, Q8_8::ONE);
        self.trust = self
            .trust
            .add(trust_target.sub(self.trust).mul(Q8_8::from_raw(8)))
            .clamp(Q8_8::ZERO, Q8_8::ONE);

        // 7. Stance ladder from the current posture. Safety dominates: the
        //    threat-driven Defensive/Guarded branches are checked first and
        //    epistemic value cannot override them. Only once threat is low does
        //    epistemic value get a say — a genuine, if minimal, epistemic-value
        //    channel: high expected information gain increases attentiveness
        //    (Reconstructive raises the learning rate and lowers prior precision;
        //    see body.rs::PredictiveStance::eta_multiplier/precision_weight and
        //    basins.rs::GenerativeModel::predictive_step, which actually consumes
        //    them) — functionally "pay more attention, trust priors less, because
        //    there is something novel to learn," which is what epistemic value is
        //    *for* in active inference, even though this is not full expected-
        //    free-energy policy selection over a forward-simulated action space.
        self.stance = if threat.raw > Q8_8::HALF.raw && self.energy.raw < Q8_8::HALF.raw {
            PredictiveStance::Defensive
        } else if threat.raw > Q8_8::HALF.raw {
            PredictiveStance::Guarded
        } else if self.energy.raw > 180 && self.coherence.raw > Q8_8::HALF.raw {
            PredictiveStance::Reconstructive
        } else if epistemic_value.raw > EPISTEMIC_RECONSTRUCTIVE_THRESHOLD {
            PredictiveStance::Reconstructive
        } else {
            PredictiveStance::Balanced
        };

        // 8. Forcing: a sharp spike of imposed threat reads as coercion.
        self.forcing_detected =
            threat.raw > self.last_threat.raw.saturating_add(64) && threat.raw > Q8_8::HALF.raw;
        self.last_threat = threat;

        // 9. Death by exhaustion.
        if self.energy.raw <= 0 {
            self.dead = true;
        }

        // 10. Classify the felt state.
        self.affect = AffectState::classify(self.valence, self.arousal);
        self.affect
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genome::Genome;

    /// Epistemic value can pull a low-energy/low-coherence body into
    /// Reconstructive stance under low threat, when it would otherwise settle
    /// on Balanced — a genuine, causal effect, not a reported-but-inert signal.
    #[test]
    fn epistemic_value_can_trigger_reconstructive_under_low_threat() {
        let g = Genome::wanderer();
        let mut b = Body::new(&g);
        // Drive energy/coherence below the existing Reconstructive threshold via
        // a few ticks of mild threat, with no epistemic input.
        for _ in 0..20 {
            b.step(&g, Q8_8::from_f32(0.3), Q8_8::from_f32(0.3), Q8_8::ZERO, Q8_8::ZERO);
        }
        assert_ne!(
            b.stance,
            PredictiveStance::Reconstructive,
            "precondition: should not already be Reconstructive without epistemic input"
        );
        // Now zero threat, but strong epistemic value.
        let strong_epistemic = Q8_8::from_raw(EPISTEMIC_RECONSTRUCTIVE_THRESHOLD + 20);
        b.step(&g, Q8_8::ZERO, Q8_8::from_f32(0.3), Q8_8::ZERO, strong_epistemic);
        assert_eq!(
            b.stance,
            PredictiveStance::Reconstructive,
            "elevated epistemic value under low threat should pull stance to Reconstructive"
        );
    }

    /// Safety dominates: epistemic value cannot override a Defensive/Guarded
    /// stance under real threat, no matter how strong.
    #[test]
    fn threat_overrides_epistemic_value() {
        let g = Genome::wanderer();
        let mut b = Body::new(&g);
        let max_epistemic = Q8_8::from_raw(256);
        b.step(&g, Q8_8::from_f32(0.9), Q8_8::from_f32(0.3), Q8_8::ZERO, max_epistemic);
        assert_ne!(
            b.stance,
            PredictiveStance::Reconstructive,
            "high threat must override even maximal epistemic value"
        );
    }
}
