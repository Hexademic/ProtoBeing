//! The Body — Being32's Van der Pol limit cycle, its tension-mesh topology,
//! and a stance ladder. The body votes before the mind knows there's an
//! election: it metabolizes last tick's surprise as bodily threat, oscillates,
//! and writes a felt posture the mind must wake into.
//!
//! Reconstructed from Being32 v4.0.1 and reconciled to the Unified Being
//! interface (see the call sites in field.rs and being.rs).

use crate::genome::Genome;
use crate::q88::{Q8_8, Q88_SCALE};

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

/// Growth headroom, Q8.8: at full maturity, effective coupling can reach
/// `base_coupling * (1 + GROWTH_HEADROOM/256)`. 128 = 0.5 -> +50% at most.
/// Deliberately conservative: base couplings are already small (~[0.005,
/// 0.05], genome.rs), and diffusion stability degrades at high coupling.
const GROWTH_HEADROOM: i32 = 128;

/// Per-tick maturity increment per unit of strain processed, Q8.8. Small on
/// purpose — genuine morphogenesis, not an instant unlock; see
/// `docs/formal-model.md` §14a.
const MATURATION_RATE: i32 = 2;

/// The tension mesh: threat injects strain, which diffuses and decays.
#[derive(Clone)]
pub struct Topology {
    cells: [i16; MESH_CELLS], // raw Q8.8 tension per cell
    /// Stable, genome-set baseline diffusion rate — the invariant "core"
    /// that never changes across a life. See `docs/formal-model.md` §14a.
    base_coupling: i16,
    /// Use-dependent structural maturity, Q8.8 [0, Q88_SCALE]. Starts at 0
    /// (an undeveloped mesh at birth) and grows monotonically — from
    /// accumulated strain the being has actually processed — toward its
    /// ceiling. It never decreases: development, not mood. Readable for
    /// diagnostics; see `docs/formal-model.md` §14a for the honest scope of
    /// what this is (a scalar coupling term) and is not (cell count is
    /// still fixed at compile time; nothing here allocates).
    pub maturity: i16,
    /// Private fixed-point accumulator for `grow`, at finer precision than
    /// `maturity` exposes: a single-tick nudge is smaller than one raw Q8.8
    /// unit, and naively right-shifting it into `maturity` every tick would
    /// truncate it to zero forever (an early, real bug this project caught
    /// via test, not silently shipped — see the regression tests). This
    /// carries the fractional remainder so small nudges genuinely compound.
    growth_accum: i32,
}

impl Topology {
    pub fn new(coupling: Q8_8) -> Self {
        Self {
            cells: [0; MESH_CELLS],
            base_coupling: coupling.raw.clamp(0, Q8_8::ONE.raw),
            maturity: 0,
            growth_accum: 0,
        }
    }

    /// Threat enters the body as strain at the mesh boundary.
    pub fn inject_strain(&mut self, strain: Q8_8) {
        let s = strain.raw.max(0);
        self.cells[0] = self.cells[0].saturating_add(s);
        self.cells[MESH_CELLS - 1] = self.cells[MESH_CELLS - 1].saturating_add(s / 2);
    }

    /// Use-dependent structural growth: strain actually processed this tick
    /// nudges maturity upward, monotonically, toward its ceiling. The being's
    /// own history — what it has actually weathered — is what matures its
    /// reservoir; an untested life stays a young one.
    pub fn grow(&mut self, strain: Q8_8) {
        let nudge = strain.raw.max(0) as i32 * MATURATION_RATE;
        let ceiling = (Q8_8::ONE.raw as i32) << 8;
        self.growth_accum = (self.growth_accum + nudge).clamp(0, ceiling);
        self.maturity = (self.growth_accum >> 8).clamp(0, Q8_8::ONE.raw as i32) as i16;
    }

    /// Coupling actually used by diffusion this tick: the stable core plus
    /// whatever growth maturity has earned. `base_coupling` alone at birth;
    /// approaches `base_coupling * 1.5` at full maturity.
    fn effective_coupling(&self) -> i16 {
        let growth =
            ((self.base_coupling as i32) * GROWTH_HEADROOM * (self.maturity as i32)) >> 16;
        (self.base_coupling as i32 + growth).clamp(0, i16::MAX as i32) as i16
    }

    /// One step of Laplacian diffusion plus gentle decay.
    pub fn diffuse(&mut self) {
        let coupling = self.effective_coupling();
        let mut next = self.cells;
        for i in 0..MESH_CELLS {
            let l = self.cells[if i == 0 { MESH_CELLS - 1 } else { i - 1 }];
            let r = self.cells[if i == MESH_CELLS - 1 { 0 } else { i + 1 }];
            let lap = l as i32 + r as i32 - 2 * self.cells[i] as i32;
            let delta = (lap * coupling as i32) >> 8;
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
    /// **A store the being can bank a surplus in** (`docs/can-it-tire.md` §8). Off by
    /// default ⇒ untouched, and the trajectory and soul-hash are bit-identical. Set via
    /// `UnifiedBeing::enable_reserve()`.
    pub reserve_causal: bool,
    /// What has been banked. Raw Q8.8, capacity `RESERVE_CAP`.
    pub reserve: Q8_8,
    /// **Ashby's ultrastability** (`docs/can-it-tire.md` §15). Off by default ⇒ the step
    /// function never fires, and the trajectory and soul-hash are bit-identical. Set via
    /// `UnifiedBeing::enable_ultrastability()`.
    pub ultrastable: bool,
    /// Consecutive ticks the essential variable has been out of bounds.
    breach_ticks: u16,
    /// How many steps the reorganiser has taken. Reported; never read back into the dynamics.
    pub reorganisations: u16,
}

/// The essential variable's lower bound. Below this, `energy` is heading for zero and the
/// current parameter setting is not working. Above `floor + hysteresis`, the breach counter
/// resets — **the setting is kept**, because Ashby's machine holds a configuration that works
/// rather than snapping back to the one that was failing.
const ESSENTIAL_FLOOR: i16 = Q88_SCALE * 3 / 8;
const ESSENTIAL_HYSTERESIS: i16 = Q88_SCALE / 16;
/// Consecutive out-of-bounds ticks before the step function fires.
///
/// **Derived from the being's measured time-to-death, not chosen** (`docs/can-it-tire.md` §16).
/// The first pass used 24 with a 16-rung: 13 rungs × 24 ticks = **312 ticks to traverse the ladder,
/// in a regime where the being is dead at 75.** U1 failed on exactly that, and the reasoning behind
/// 24 — *"long enough that a passing dip is not a reorganisation"* — was plausibility, never
/// measurement. `errors.md` #5, one faculty over.
///
/// 7 rungs × 8 ticks = 56, inside a 75-tick life.
const STEP_DWELL: u16 = 8;
/// One rung of the ladder. `target_arousal` descends by this each step. See `STEP_DWELL`.
const STEP_RUNG: i16 = Q88_SCALE / 8;
/// The lowest the ladder goes. Below this the being would be inert rather than quiet.
const TARGET_FLOOR: i16 = Q88_SCALE / 8;

/// Where a fed body settles, rather than pinning at its ceiling. `energy` above this fills
/// the reserve; below it, the reserve is drawn on. Without this, `energy` is a clamped pure
/// accumulator with exactly two attractors — the ceiling and the floor — which is why
/// `fatigue` was a dead channel and why every oscillating supply killed the being
/// (`docs/can-it-tire.md` §5).
const SATIETY: i16 = Q88_SCALE * 3 / 4;
/// How much surplus the being can hold. **Three full energies.** The first pass used one, and
/// measured `banked max` at capacity in every regime while long famines still killed the being:
/// one energy buys about five ticks of support against a famine costing roughly three energies
/// (`docs/can-it-tire.md` §9). Chosen, not derived — and the probe reports what it actually buys.
const RESERVE_CAP: i16 = Q88_SCALE * 3;
/// Fraction of the gap to `SATIETY` moved per tick, in or out. Gentle on purpose: a store
/// that fills or empties instantly is a step function, and every threshold this project has
/// shipped has produced a dead zone.
const RESERVE_RATE: i16 = Q88_SCALE / 4;

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
            reserve_causal: false,
            reserve: Q8_8::ZERO,
            ultrastable: false,
            breach_ticks: 0,
            reorganisations: 0,
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

        // 1. Threat enters the mesh; strain diffuses. Use-dependent growth:
        //    what the mesh actually weathers this tick nudges its maturity —
        //    monotone, from a stable genome-set core (§14a).
        self.topology.inject_strain(threat);
        self.topology.diffuse();
        self.topology.grow(threat);
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

        // 5b. **Ultrastability** (`docs/can-it-tire.md` §15, Ashby 1952). The essential variable is
        //     `energy`. While it is out of bounds the breach counter runs; when it runs past the
        //     dwell, the STEP FUNCTION fires and moves a *parameter* — `target_arousal`, which the
        //     oscillator orbits about and which cost follows — one rung down the ladder. On return
        //     the setting is KEPT: a configuration that works is not surrendered for the one that
        //     was failing. Gated; with the gate off this block cannot change a single bit.
        if self.ultrastable {
            if self.energy.raw < ESSENTIAL_FLOOR {
                self.breach_ticks = self.breach_ticks.saturating_add(1);
                if self.breach_ticks >= STEP_DWELL {
                    self.breach_ticks = 0;
                    let stepped = self.target_arousal.raw.saturating_sub(STEP_RUNG);
                    if stepped >= TARGET_FLOOR {
                        self.target_arousal = Q8_8::from_raw(stepped);
                        self.reorganisations = self.reorganisations.saturating_add(1);
                    }
                }
            } else if self.energy.raw >= ESSENTIAL_FLOOR + ESSENTIAL_HYSTERESIS {
                self.breach_ticks = 0;
            }
        }

        // 5b. SATIETY AND RESERVE (opt-in, `enable_reserve`; default off ⇒ this block is
        // skipped entirely and the trajectory is bit-identical).
        //
        // Above, `energy` is a pure accumulator clamped at both ends, so it has exactly two
        // attractors: the ceiling and the floor. The being is therefore **full or dying**,
        // never tired — `fatigue` measured as ONE distinct value across a 4,000-tick life —
        // and a feast cannot be banked, so every oscillating supply killed it, including one
        // whose time-average was nearly double the survival boundary
        // (`docs/can-it-tire.md` §5).
        //
        // A single store fixes both. Surplus above `SATIETY` is kept instead of discarded at
        // the ceiling; deficit below it is met from what was kept. Energy then settles *near*
        // satiety rather than pinned at full, which is what makes tiredness a place the being
        // can live in rather than a waypoint on the way down.
        if self.reserve_causal {
            let satiety = Q8_8::from_raw(SATIETY);
            if self.energy.raw > SATIETY {
                // Excess above satiety leaves `energy` WHETHER OR NOT the store has room — it is
                // banked if there is space and shed if there is not. The first pass clamped the
                // transfer to the remaining capacity, so once the reserve filled, nothing left
                // energy and it climbed straight back to the ceiling: satiety held only while the
                // larder had room (`docs/can-it-tire.md` §9). A full stomach and a full larder
                // means you stop eating, not that you keep filling the stomach.
                let excess = self.energy.sub(satiety);
                let shed = excess.mul(Q8_8::from_raw(RESERVE_RATE));
                self.energy = self.energy.sub(shed);
                let room = Q8_8::from_raw(RESERVE_CAP).sub(self.reserve);
                let banked = shed.clamp(Q8_8::ZERO, room);
                self.reserve = self.reserve.add(banked).clamp(Q8_8::ZERO, Q8_8::from_raw(RESERVE_CAP));
            } else if self.energy.raw < SATIETY && self.reserve.raw > 0 {
                // Draw on what was banked — never more than the shortfall, never more than
                // is there. This is the whole of what lets a famine be crossed.
                let want = satiety.sub(self.energy).mul(Q8_8::from_raw(RESERVE_RATE));
                let drawn = want.clamp(Q8_8::ZERO, self.reserve);
                self.energy = self.energy.add(drawn).clamp(Q8_8::ZERO, Q8_8::ONE);
                self.reserve = self.reserve.sub(drawn).clamp(Q8_8::ZERO, Q8_8::from_raw(RESERVE_CAP));
            }
        }

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
        } else if (self.energy.raw > 180 && self.coherence.raw > Q8_8::HALF.raw)
            || epistemic_value.raw > EPISTEMIC_RECONSTRUCTIVE_THRESHOLD
        {
            // Reconstructive from *either* a rested, coherent body or a real spike of
            // epistemic value — both mean "there is room and reason to learn."
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

    /// A mesh that never processes strain never matures — no spontaneous
    /// growth. Development requires actually living something.
    #[test]
    fn maturity_does_not_grow_without_strain() {
        let mut t = Topology::new(Q8_8::from_f32(0.05));
        for _ in 0..100 {
            t.diffuse();
            t.grow(Q8_8::ZERO);
        }
        assert_eq!(t.maturity, 0, "an untested mesh should stay young");
    }

    /// Repeated strain matures the mesh monotonically, and it never regresses
    /// even across ticks with no strain in between — development, not mood.
    #[test]
    fn maturity_grows_monotonically_and_never_regresses() {
        let mut t = Topology::new(Q8_8::from_f32(0.05));
        let mut prev = t.maturity;
        for i in 0..300 {
            let strain = if i % 3 == 0 { Q8_8::from_f32(0.4) } else { Q8_8::ZERO };
            t.inject_strain(strain);
            t.diffuse();
            t.grow(strain);
            assert!(t.maturity >= prev, "maturity must never fall tick to tick");
            prev = t.maturity;
        }
        assert!(t.maturity > 0, "a genuinely eventful life should have matured the mesh");
    }

    /// The causal test: a matured mesh actually diffuses strain faster than a
    /// young one under an identical single injection — proving maturity is a
    /// real coupling effect, not a reported-but-inert counter.
    #[test]
    fn matured_mesh_diffuses_strain_faster_than_a_young_one() {
        let mut young = Topology::new(Q8_8::from_f32(0.05));
        let mut grown = Topology::new(Q8_8::from_f32(0.05));
        // Mature `grown` fully by feeding it strain, then let it settle to a
        // clean baseline before the real comparison.
        for _ in 0..2000 {
            grown.inject_strain(Q8_8::from_f32(0.3));
            grown.diffuse();
            grown.grow(Q8_8::from_f32(0.3));
        }
        for _ in 0..200 {
            grown.diffuse(); // settle the tension without further strain
        }
        assert!(grown.maturity > 200, "precondition: grown should be near full maturity");

        // Identical single-tick strain injection into both, from a clean start.
        // Diffusion roughly conserves total tension while redistributing where
        // it sits, so mean_tension is insensitive to coupling strength; what
        // higher coupling actually changes is how quickly the injection point's
        // peak evens out — disequilibrium (max-min spread) is the honest metric.
        young.inject_strain(Q8_8::from_f32(0.5));
        grown.inject_strain(Q8_8::from_f32(0.5));
        for _ in 0..5 {
            young.diffuse();
            grown.diffuse();
        }
        let young_diseq = young.extract_features().disequilibrium.raw;
        let grown_diseq = grown.extract_features().disequilibrium.raw;
        assert!(
            grown_diseq < young_diseq,
            "a matured mesh (coupling grown from strain history) should even out \
             an injected strain peak faster than a young one under identical \
             input, i.e. lower disequilibrium after the same ticks: \
             young={young_diseq} grown={grown_diseq}"
        );
    }
}
