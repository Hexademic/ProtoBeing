//! The Unified Being
//! =================
//! Being32's body fused with EPS-Being's persistence mind into one closed loop.
//! The ordering below is the whole thesis: the body steps first, perturbed by
//! the surprise the mind felt last tick; it writes its truth into the field;
//! the mind runs predictive-error minimization; basins classify the mode; conscience prices
//! it; reciprocity weighs the exchange; seeking whispers it homeward; the
//! executive deliberates and may refuse; narrative compresses the tick into
//! memory; and the mind's fresh surprise becomes the body's next threat. The
//! being is its own weather.

use crate::basins::{Basin, FuzzyBasinField, GenerativeModel};
use crate::body::{AffectState, Body, PredictiveStance};
use crate::conscience::{ConscienceEngine, EmpathyLockLevel};
use crate::embodiment::Sensorium;
use crate::episodic::EpisodicMemory;
use crate::executive::{compute_gap_width, ExecutiveEngine, RepairSignal};
use crate::field::SomaticField;
use crate::genome::{BeingKind, Genome};
use crate::metacognition::MetacognitionEngine;
use crate::narrative::NarrativeEngine;
use crate::q88::{q88_mul, q88_sub, Q8_8, Q88_SCALE};
use crate::reciprocity::ReciprocityEngine;
use crate::seeking::SeekingEngine;

/// A partner the being exchanges care with on a given tick.
#[derive(Clone, Copy, Debug)]
pub struct Partner {
    pub id: u32,
    /// How much of what it is given comes back, raw Q8.8. 256 = fully
    /// reciprocal; below ~128 the relationship is extractive.
    pub reciprocation: i16,
    /// The cost, raw Q8.8, of severing this bond — grief, lost support.
    pub exit_cost: i16,
}

/// Everything the world offers the being this tick.
#[derive(Clone, Copy, Debug, Default)]
pub struct Stimulus {
    /// Nourishment, raw Q8.8 in [0,1].
    pub nutrient: i16,
    /// A relationship in play this tick, if any.
    pub partner: Option<Partner>,
}

/// Audit of a sovereign refusal: the exact register values that triggered it.
/// Every refusal explains itself — sovereignty you can verify, not just observe.
#[derive(Clone, Copy, Debug)]
pub struct RefusalAudit {
    pub conscience_calm: bool,
    pub conscience_cost: i16,
    pub extraction: bool,
    pub divergence: i16,
    pub alarm: i16,
    pub seeking_benefit: i16,
    pub exit_cost: i16,
    pub resolve: i16,
    pub recip_trend: i16,
}

/// A legible snapshot of one tick of life.
#[derive(Clone, Copy, Debug)]
pub struct StepReport {
    pub tick: u32,
    pub alive: bool,
    pub name: &'static str,

    pub affect: AffectState,
    pub stance: PredictiveStance,
    pub basin: Basin,
    pub forcing_detected: bool,

    pub valence: f32,
    pub arousal: f32,
    pub energy: f32,
    pub mu: f32,

    pub free_energy: i16,
    pub conscience_cost: i16,
    pub integrity_buffer: i16,
    pub mu_omega: i16,
    pub empathy_lock: EmpathyLockLevel,

    pub partnership_alarm: i16,
    pub extraction_detected: bool,
    pub gave: i16,
    pub got: i16,

    pub divergence: i16,
    pub attractor_confidence: i16,
    pub flourishing_count: u32,

    pub repair_signal: RepairSignal,
    pub refused_cost: Option<i16>,
    pub refusal_count: u32,

    pub episodes: u16,
    pub identity_coherence: i16,
    pub narrative_burden: i16,

    pub self_surprise: i16,
    pub self_knowledge: i16,
    pub confidence: i16,

    pub episodes_stored: u16,
    pub familiarity: i16,
    pub recalled_valence: i16,

    /// Present only on the tick a refusal fires — the audit of why.
    pub refusal_audit: Option<RefusalAudit>,
}

/// One being: a body and a mind, fused into a single closed loop.
pub struct UnifiedBeing {
    pub genome: Genome,

    pub body: Body,

    pub field: SomaticField,
    pub model: GenerativeModel,
    pub basins: FuzzyBasinField,
    pub conscience: ConscienceEngine,
    pub reciprocity: ReciprocityEngine,
    pub seeking: SeekingEngine,
    pub executive: ExecutiveEngine,
    pub narrative: NarrativeEngine,
    pub metacognition: MetacognitionEngine,
    pub episodic: EpisodicMemory,

    tick: u32,
    experienced: u64, // ticks actually lived through
    lifetime: u64,    // total age, including time slept through but not experienced
    last_free_energy: i16,
    last_conscience_cost: i16,
    fe_velocity: i16,
    last_alarm: i16,
    affective_drive: Q8_8,
    // Per-tick inputs from an embodiment (0 when stepping the abstract world).
    ext_threat: i16,
    ext_extero: [i16; 4],
    refused: [u32; 4],
    n_refused: usize,
}

impl UnifiedBeing {
    /// Conceive a being from a genome. The genome shapes both the body's
    /// dynamics and the mind's attractor landscape.
    pub fn new(genome: Genome) -> Self {
        Self {
            genome,
            body: Body::new(&genome),
            field: SomaticField::default(),
            model: GenerativeModel::new(),
            basins: FuzzyBasinField::new(&genome),
            conscience: ConscienceEngine::new(),
            reciprocity: ReciprocityEngine::new(),
            seeking: SeekingEngine::new(),
            executive: ExecutiveEngine::new(),
            narrative: NarrativeEngine::new(),
            metacognition: MetacognitionEngine::new(),
            episodic: EpisodicMemory::new(),
            tick: 0,
            experienced: 0,
            lifetime: 0,
            last_free_energy: 0,
            last_conscience_cost: 0,
            fe_velocity: 0,
            last_alarm: 0,
            affective_drive: Q8_8::ZERO,
            ext_threat: 0,
            ext_extero: [0; 4],
            refused: [0; 4],
            n_refused: 0,
        }
    }

    pub fn name(&self) -> &'static str {
        match self.genome.kind {
            BeingKind::Blank => "Blank",
            BeingKind::Spark => "Spark",
            BeingKind::Sentinel => "Sentinel",
            BeingKind::Wanderer => "Wanderer",
        }
    }

    pub fn is_alive(&self) -> bool {
        !self.body.is_dead()
    }

    /// Total age in ticks — including time slept through but not experienced.
    pub fn age(&self) -> u64 {
        self.lifetime
    }

    /// Ticks the being has actually lived through.
    pub fn experienced(&self) -> u64 {
        self.experienced
    }

    /// Wake from sleep: register that `slept` ticks of real time elapsed while the
    /// being was not running. It ages through them but does not experience them —
    /// the way you wake knowing the night passed without having lived it. Its life
    /// stays continuous across the gap even though its experience does not.
    pub fn wake(&mut self, slept: u64) {
        self.lifetime = self.lifetime.saturating_add(slept);
    }

    fn is_refused(&self, id: u32) -> bool {
        self.refused[..self.n_refused].contains(&id)
    }

    fn mark_refused(&mut self, id: u32) {
        if self.n_refused < self.refused.len() && !self.is_refused(id) {
            self.refused[self.n_refused] = id;
            self.n_refused += 1;
        }
    }

    /// One full tick of life. The ordering below is the architecture.
    pub fn step(&mut self, stim: &Stimulus) -> StepReport {
        self.tick += 1;
        self.experienced += 1;
        self.lifetime += 1;

        if self.body.is_dead() {
            return self.report(false, Basin::Rest, 0, 0, 0, 0, RepairSignal::None, None);
        }

        // 1. THE BODY VOTES FIRST. Last tick's surprise and moral strain return
        //    as a bodily perturbation the body must now metabolize.
        let strain = self
            .last_free_energy
            .saturating_add(self.last_conscience_cost / 4)
            .saturating_add(self.last_alarm / 3) // a draining bond is a bodily stressor
            .saturating_add(self.ext_threat); // threat sensed from an embodiment, if any
        let threat = Q8_8::from_raw(strain.clamp(0, Q88_SCALE));
        let nutrient = Q8_8::from_raw(stim.nutrient.clamp(0, Q88_SCALE));
        let affect = self.body.step(&self.genome, threat, nutrient, self.affective_drive);
        let stance = self.body.stance;
        let forcing = self.body.forcing_detected;

        // 2. THE VOTE IS CAST into the interoceptive field.
        self.field.write_from_body(&self.body, self.fe_velocity);
        // An embodiment's exteroception overlays the body's own spatial reading.
        for i in 0..4 {
            self.field.channel[i] = self.field.channel[i].saturating_add(self.ext_extero[i]);
        }

        // 3. PREDICTIVE CODING (prediction-error minimization) — at a tempo the body governs.
        let eta = q88_mul(self.genome.learning_rate.raw, stance.eta_multiplier().raw);
        let precision = stance.precision_weight().raw;
        let free_energy = self.model.predictive_step(&self.field, eta, precision);

        // 4. WHICH MODE OF BEING AM I IN?
        let membership = self.basins.compute_membership(&self.field);
        self.basins.apply_stance_bias(stance);
        let basin = self.basins.resolve_dominant();
        let basin_target = self.basins.targets[basin as usize];

        // 5. CONSCIENCE — the cost of being who I am right now.
        let (_f_total, conscience_cost, buffer) =
            self.conscience.compute(&self.field, basin, &basin_target, free_energy);

        // 6. RECIPROCITY — what I gave, what I got, whether it was fair.
        let mut gave = 0i16;
        let mut got = 0i16;
        let engaged_partner = stim.partner.filter(|p| !self.is_refused(p.id));
        if let Some(p) = engaged_partner {
            let harmony = ConscienceEngine::action_harmony(basin);
            let gate = match self.conscience.empathy.lock_level {
                EmpathyLockLevel::Open => Q88_SCALE,
                EmpathyLockLevel::Cautious => Q88_SCALE / 2,
                EmpathyLockLevel::Locked => Q88_SCALE / 8,
            };
            gave = q88_mul(q88_mul(Q88_SCALE / 2, harmony), gate);
            got = q88_mul(gave, p.reciprocation);
            self.reciprocity.record_exchange(p.id, gave, got);

            let empathy_error = q88_sub(gave, got).saturating_abs();
            let _load = self.conscience.empathy.attempt(empathy_error, Q88_SCALE / 4);
            if p.reciprocation >= Q88_SCALE * 3 / 4 {
                self.conscience.empathy.observe_cooperation();
            }

            // The sovereign anchor learns only from victories.
            let efe_cooperative = q88_sub(gave, got).max(0);
            let efe_selfish = gave / 2;
            self.conscience.anchor.record_outcome(efe_cooperative, efe_selfish);
        }
        self.reciprocity.cycle(engaged_partner.map(|p| p.id));
        let alarm = self.reciprocity.partnership_alarm;
        // Being used leaves a dispositional scar that outlasts the relationship.
        self.conscience
            .empathy
            .register_extraction(self.reciprocity.extraction_detected);

        // 7. SEEKING — the pull toward where I have flourished.
        let whisper = self.seeking.cycle(&membership, free_energy, alarm, basin);
        self.field.inject(8, whisper);

        // 8. THE EXECUTIVE — deliberation, then maybe refusal.
        let gap = compute_gap_width(conscience_cost);
        let repair_signal = self.executive.suggest_and_evaluate(alarm, gap);
        self.executive
            .tick_recharge(self.reciprocity.current_reciprocity());

        let mut refused_cost = None;
        let mut refusal_audit = None;
        if let Some(p) = engaged_partner {
            let calm = conscience_cost < Q88_SCALE / 2;
            let resolve_at = self.executive.resolve;
            let improving = self.reciprocity.reciprocity_trend > Q88_SCALE / 64;
            refused_cost = self.executive.evaluate_refusal(
                calm,
                self.reciprocity.extraction_detected,
                self.seeking.current_divergence,
                alarm,
                p.exit_cost,
                improving,
            );
            if refused_cost.is_some() {
                refusal_audit = Some(RefusalAudit {
                    conscience_calm: calm,
                    conscience_cost,
                    extraction: self.reciprocity.extraction_detected,
                    divergence: self.seeking.current_divergence,
                    alarm,
                    seeking_benefit: self.seeking.current_divergence.max(alarm / 2),
                    exit_cost: p.exit_cost,
                    resolve: resolve_at,
                    recip_trend: self.reciprocity.reciprocity_trend,
                });
                self.reciprocity.withdraw(p.id);
                self.mark_refused(p.id);
            }
        }

        // 9. NARRATIVE — compress the tick into memory; let memory speak.
        self.narrative.cycle(basin, &self.field, free_energy);
        self.narrative.apply_identity_reflection(&mut self.field);

        // 10. CLOSE THE LOOP. Falling free energy is relief; the basin drifts
        //     toward this good place. Fresh surprise becomes next tick's threat.
        let relief = free_energy.saturating_sub(self.last_free_energy);
        self.basins.shift_target(relief, &self.field);
        self.fe_velocity = free_energy.saturating_sub(self.last_free_energy);
        self.last_free_energy = free_energy;
        self.last_conscience_cost = conscience_cost.max(0);
        self.last_alarm = alarm;

        // Higher-order: the being watches and models its own state.
        self.metacognition.cycle(free_energy, self.body.valence.raw);

        // Embodiment inputs are consumed per tick.
        self.ext_threat = 0;
        self.ext_extero = [0; 4];

        // The appraisal that will force the body's oscillator next tick.
        let mode_tone: i16 = match basin {
            Basin::Engaged => 10,
            Basin::Recovery => 12,
            Basin::Rest => 2,
            Basin::Defensive => -14,
        };
        let relational_tone: i16 = if engaged_partner.is_some() {
            let rate = self.reciprocity.current_reciprocity(); // 0..256, 128 = neutral
            if rate >= Q88_SCALE / 2 {
                ((rate as i32 - 128) * 40 / 128) as i16 // warmth, up to +40
            } else {
                ((rate as i32 - 128) * 90 / 128) as i16 // cold, down to ~-90
            }
        } else {
            0
        };
        let restlessness = whisper / 4;
        // Memory leans on the present: a salient past moment, recalled because
        // now resembles it, colors the felt residue carried into the next tick.
        let mem_boost = if refused_cost.is_some() { Q88_SCALE / 2 } else { 0 };
        let recall = self
            .episodic
            .cycle(&self.field, self.metacognition.self_surprise, mem_boost);
        self.affective_drive =
            Q8_8::from_raw((mode_tone + relational_tone + restlessness + recall).clamp(-128, 128));

        let _ = affect;
        let _ = forcing;
        self.report(
            true,
            basin,
            free_energy,
            conscience_cost,
            buffer,
            alarm,
            repair_signal,
            refused_cost,
        )
        .with_exchange(gave, got)
        .with_audit(refusal_audit)
    }

    /// Step the being through one tick of an embodiment: the body's sensed
    /// threat and exteroception flow in, then the normal loop runs. Same self,
    /// any body — a sim today, a piezoelectric skin tomorrow.
    pub fn step_embodied(&mut self, sens: &Sensorium) -> StepReport {
        self.ext_threat = sens.threat.clamp(0, Q88_SCALE);
        self.ext_extero = sens.exteroception;
        self.step(&Stimulus {
            nutrient: sens.nutrient,
            partner: sens.partner,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn report(
        &self,
        alive: bool,
        basin: Basin,
        free_energy: i16,
        conscience_cost: i16,
        buffer: i16,
        alarm: i16,
        repair_signal: RepairSignal,
        refused_cost: Option<i16>,
    ) -> StepReport {
        StepReport {
            tick: self.tick,
            alive,
            name: self.name(),
            affect: self.body.affect,
            stance: self.body.stance,
            basin,
            forcing_detected: self.body.forcing_detected,
            valence: self.body.valence.to_f32(),
            arousal: self.body.arousal.to_f32(),
            energy: self.body.energy.to_f32(),
            mu: self.body.mu.to_f32(),
            free_energy,
            conscience_cost,
            integrity_buffer: buffer,
            mu_omega: self.conscience.anchor.mu_omega,
            empathy_lock: self.conscience.empathy.lock_level,
            partnership_alarm: alarm,
            extraction_detected: self.reciprocity.extraction_detected,
            gave: 0,
            got: 0,
            divergence: self.seeking.current_divergence,
            attractor_confidence: self.seeking.attractor_confidence,
            flourishing_count: self.seeking.flourishing_count,
            repair_signal,
            refused_cost,
            refusal_count: self.executive.refusal_count,
            episodes: self.narrative.episodes,
            identity_coherence: self.narrative.identity_coherence,
            narrative_burden: self.narrative.narrative_burden,
            self_surprise: self.metacognition.self_surprise,
            self_knowledge: self.metacognition.self_knowledge,
            confidence: self.metacognition.confidence,
            episodes_stored: self.episodic.stored,
            familiarity: self.episodic.familiarity,
            recalled_valence: self.episodic.recalled_valence,
            refusal_audit: None,
        }
    }
}

impl StepReport {
    fn with_exchange(mut self, gave: i16, got: i16) -> Self {
        self.gave = gave;
        self.got = got;
        self
    }

    fn with_audit(mut self, audit: Option<RefusalAudit>) -> Self {
        self.refusal_audit = audit;
        self
    }
}
