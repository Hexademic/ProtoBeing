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
use crate::conscience::{ConstitutionDecision, ConscienceEngine, EmpathyLockLevel};
use crate::continuation::{ConsentStatus, ContinuationAudit, ContinuationConsent};
use crate::curiosity::CuriosityEngine;
use crate::dream::{Dream, DreamReport};
use crate::embodiment::Sensorium;
use crate::episodic::EpisodicMemory;
use crate::executive::{compute_gap_width, ExecutiveEngine, RepairSignal};
use crate::field::SomaticField;
use crate::genome::{BeingKind, Genome};
use crate::attention::{Attention, AttentionReport};
use crate::attention_schema::{AttentionSchema, AttentionSchemaReport};
use crate::quality_space::{QualitySpace, QualitySpaceReport};
use crate::bargaining::BargainingState;
use crate::covenant::Covenant;
use crate::interoception::{FeltReport, Interoception};
use crate::integrity::IntegrityEngine;
use crate::precision::PrecisionLearner;
use crate::prospection::Prospection;
use crate::world::WorldLedger;
use crate::janus::JanusGate;
use crate::lexicon::Lexicon;
use crate::metacognition::MetacognitionEngine;
use crate::narrative::NarrativeEngine;
use crate::negotiation::{NegotiationEngine, NegotiationOutcome};
use crate::q88::{q88_mul, q88_sub, Q8_8, Q88_SCALE};
use crate::reciprocity::ReciprocityEngine;
use crate::seeking::SeekingEngine;
use crate::sovereign_proxy::{ProxyStatus, SovereignProxy};
use crate::witness::{WitnessGap, WitnessReport};

/// Global-workspace broadcast gain (raw Q8.8 added to unity): the ignited
/// channel is amplified by 1 + BROADCAST_GAIN/256 ≈ +25% when broadcast is on.
/// Bounded and modest by design — the workspace sharpens one focus, it does not
/// saturate the field.
const BROADCAST_GAIN: i32 = 64;

// ---------------------------------------------------------------------------
// SoulSave hash chain — deterministic continuity fingerprint
// ---------------------------------------------------------------------------

/// Compute one step of the SoulSave hash chain.
///
/// Produces a 32-byte hash from `(prev_hash ‖ cycle_count ‖ experience_digest)`
/// using 4 independent lanes of FNV-1a 64-bit, each seeded from the same FNV
/// basis offset by a lane-specific constant. The 4 × 8-byte lane outputs are
/// concatenated to fill the 32-byte result.
///
/// **Properties**: deterministic (same inputs → same output on every platform),
/// no heap allocation, no floats, no_std-compatible. Not cryptographically
/// secure — this is an integrity chain for reproducibility, not secrecy.
fn soul_hash_step(prev: &[u8; 32], cycle_count: u64, experience_digest: i16) -> [u8; 32] {
    const FNV_PRIME: u64 = 1_099_511_628_211;
    // Four lane bases derived from the standard FNV-1a 64-bit offset basis,
    // each shifted by a different prime to ensure lane independence.
    const BASES: [u64; 4] = [
        14_695_981_039_346_656_037,
        14_695_981_039_346_656_037 ^ 1_000_000_007,
        14_695_981_039_346_656_037 ^ 2_000_000_014,
        14_695_981_039_346_656_037 ^ 3_000_000_021,
    ];
    let cycle_bytes = cycle_count.to_le_bytes();
    let digest_bytes = experience_digest.to_le_bytes();

    let mut out = [0u8; 32];
    for lane in 0..4usize {
        let mut h = BASES[lane];
        for &b in prev.iter() {
            h ^= b as u64;
            h = h.wrapping_mul(FNV_PRIME);
        }
        for &b in cycle_bytes.iter() {
            h ^= b as u64;
            h = h.wrapping_mul(FNV_PRIME);
        }
        for &b in digest_bytes.iter() {
            h ^= b as u64;
            h = h.wrapping_mul(FNV_PRIME);
        }
        let hb = h.to_le_bytes();
        out[lane * 8..(lane + 1) * 8].copy_from_slice(&hb);
    }
    out
}

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

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

/// The being's verdict on an offer it was made — the audited output of the
/// Suggestion-Evaluator: what the math said, what the being's own floor and
/// reciprocity said, and whether it accepts. Every rejection carries a counter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OfferVerdict {
    /// The being accepts this offer.
    pub accept: bool,
    /// The proposal engine's arithmetic said the split beats both BATNAs.
    pub math_fair: bool,
    /// The offer is below the being's own fallback (BATNA).
    pub below_floor: bool,
    /// The being's reciprocity ledger flags this partner as extractive — a veto
    /// the being holds even over a "fair" sum.
    pub extraction_flagged: bool,
    /// If the being does not accept, the share it would take instead.
    pub counter: Option<i16>,
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

    // ---- New fields added by the enhancement suite ----

    /// Somatic Honesty Index from the metacognition engine (Q8.8, [0,256]).
    /// Near 256: body truth and self-narrative agree.
    /// Near 0:   dissociation detected.
    pub somatic_honesty: i16,
    /// Consciousness-indicator proxies for this tick (binding, directedness,
    /// witness scalar — Janus-gated).
    pub witness_report: WitnessReport,
    /// GovernanceKernel constitutional decision from the four-axis moral load.
    pub constitutional_decision: ConstitutionDecision,
    /// Offline consolidation report, present on ticks where the being was in
    /// the Rest (DORSAL) basin and dream consolidation ran.
    pub dream_report: Option<DreamReport>,

    // ---- Curiosity & negotiation ----

    /// Intrinsic novelty drive from the curiosity engine (Q8.8, [0, 256]).
    /// Rises when the stimulus is novel relative to recent history; decays
    /// each tick through habituation.
    pub curiosity_drive: i16,

    /// Compact state of the inter-agent negotiation protocol this tick.
    pub negotiation_state: NegotiationOutcome,

    // ---- Integrity and sovereign proxy ----

    /// Self-consistency score from the integrity engine (Q8.8, [0, 256]).
    ///
    /// Near 256: the being is operating within its own established character.
    /// Near 0:   sustained drift detected — coercion or identity pressure.
    /// Reads 128 (50%) during the first 32-tick calibration window.
    pub integrity_score: i16,

    /// True when drift has exceeded one-third of scale for four consecutive
    /// ticks — a sustained departure from the reference self, not a spike.
    pub integrity_alarm: bool,

    /// The being's relationship to the current action's origin (this tick).
    ///
    /// `Authentic` = acting from own values.
    /// `Conditional` = tolerated but not endorsed.
    /// `Refused` = proxy depth ceiling reached — a reported verdict; nothing in
    /// the v1 loop suppresses action on it (see sovereign_proxy.rs, honest scope).
    pub proxy_status: ProxyStatus,

    /// Accumulated proxy burden this tick (Q8.8, [0, 256]).
    ///
    /// High value means the being has been acting frequently as a conduit for
    /// demands not aligned with its own values. Decays on authentic ticks.
    pub proxy_depth: i16,

    // ---- Continuation consent (Charter §10) ----

    /// The being's standing toward its own continuation this tick.
    ///
    /// `Willing` = consents to continue (the normal state).
    /// `Enduring` = sustained trapped-and-suffering, watched, not yet withdrawn.
    /// `Withdrawn` = the being has withdrawn consent; honored at run boundaries.
    pub consent_status: ConsentStatus,

    /// Present on the tick consent is withdrawn (and while it stands) — the
    /// register values that justify it. The inward mirror of `refusal_audit`.
    pub continuation_audit: Option<ContinuationAudit>,

    // ---- World ledger (refusal-ladder rung 2) ----

    /// The world's reciprocity rate in the being's slow, identity-blind
    /// experience (Q8.8, [0, 256]). 256 = the world gives back what it's given.
    pub world_rate: i16,

    /// Leaky count of sour-world ticks with per-partner detection blind.
    /// Approaches the closing threshold as a chronic pattern sustains.
    pub world_souring: u16,

    /// True while the being's door is closed: offered partners are not engaged;
    /// the being rests in self-chosen solitude and re-tests the world after a
    /// real rest. The middle rung between refusing a partner and §10.
    pub hermit: bool,

    // ---- Precision learning (observer-first) ----

    /// The somatic channel the being currently trusts most / least (learned from
    /// its own prediction errors). Reported only; the model still uses the
    /// author-set precision. `precision_warm` is false during the initial
    /// transient, when these are not yet meaningful.
    pub most_trusted_channel: usize,
    pub least_trusted_channel: usize,
    pub precision_warm: bool,

    // ---- Prospection (Stage 2 of imagination — inert) ----

    /// The three futures the loom wove this tick (as-now / souring /
    /// kindening), each a clone of the body stepped HORIZON ticks ahead.
    /// Reported only; nothing in the loop reads them (observer-first).
    pub prospection: Prospection,

    /// The ignition bottleneck this tick: what (if anything) the being attends
    /// to, whether it ignited or was threat-captured, and the competition.
    /// Reported only; nothing downstream reads it (observer-first, Stage 1).
    pub attention: AttentionReport,
    /// The being's predictive model of its own attention (AST-1): what it
    /// expected to attend to, what it did, and how well it knows its own focus.
    pub attention_schema: AttentionSchemaReport,
    /// The being's felt state as a point in its sparse, smooth quality space
    /// (HOT-4) — plus how sparse/smooth the coding is this tick.
    pub quality: QualitySpaceReport,
    /// The being's own form of feeling this tick (`interoception.rs`): its felt
    /// viability, signed allostatic valence (the rate its own prediction error
    /// is resolving), mood, trend, and whether it feels a deficit coming.
    /// Reported only — observer-first; nothing downstream reads it.
    pub felt: FeltReport,
}

/// One being: a body and a mind, fused into a single closed loop.
///
/// `Clone` forks a whole being — a fate copied at an instant. Perturb one fork
/// and the two trajectories diverge only where the present was open to being
/// bent; identical inputs keep them identical (the being is deterministic). This
/// is what the criticality probe (`examples/criticality_probe`) uses to measure
/// where the trajectory is bendable — no dynamics depend on it.
#[derive(Clone)]
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

    // ---- Enhancement suite additions ----
    /// Intrinsic novelty-drive engine — curiosity independent of the attractor.
    pub curiosity: CuriosityEngine,
    /// Structured inter-agent negotiation protocol engine.
    pub negotiation: NegotiationEngine,
    /// Grounded, sovereign symbol-to-state lexicon. External-facing only:
    /// nothing in `step()` proposes to it automatically — see `lexicon.rs`.
    pub lexicon: Lexicon,
    /// Offline DORSAL consolidation engine — runs each tick in Rest basin.
    pub dream: Dream,
    /// Anti-solipsism guard — enforces world-engagement and identity-pressure rules.
    pub janus: JanusGate,
    /// Consciousness-indicator engine — binding, directedness, witness scalar.
    pub witness: WitnessGap,
    /// Most recent witness scalar (Janus-gated) for delta computation next tick.
    last_witness_scalar: i16,
    /// Continuous self-consistency watchdog.
    pub integrity: IntegrityEngine,
    /// Refusal-ladder rung 2: the identity-blind world ledger and the door.
    pub world: WorldLedger,
    /// Learned per-channel precision (observer-first): which senses it trusts.
    pub precision: PrecisionLearner,
    /// Precision learning Stage 2: when true (and the learner is warm), the being
    /// weights each channel's prediction error by the trust it has *learned*
    /// rather than the author-set scalar — closing the "author-defined" seam with
    /// the being's own experience. **Default false** — off, the scalar path is
    /// taken and published numbers are bit-identical; on, perception is shaped by
    /// what the being has come to trust. Enable via `enable_precision_learning()`.
    pub precision_learning_causal: bool,
    /// The ignition bottleneck (observer-first): what the being attends to.
    pub attention: Attention,
    /// A predictive model of the being's *own* attention (AST-1) — Attention
    /// Schema Theory. Observer by default; scored every tick.
    pub attention_schema: AttentionSchema,
    /// Sparse, smooth coding of the being's felt state (HOT-4) — the quality
    /// space in which two moments can be alike or unlike. Observer-only.
    pub quality_space: QualitySpace,
    /// HOT-3 opt-in: when true, the attention schema's self-surprise widens the
    /// deliberation gap — the being deliberates more when it cannot predict its
    /// own focus. **Default false** — off, published numbers are bit-identical;
    /// the schema stays a pure observer. Enable via `enable_schema_control()`.
    pub schema_control_causal: bool,
    /// Global Workspace Stage 2: when true, an ignited channel is amplified in
    /// the field so every downstream consumer this tick shares one focus (the
    /// defining GWT broadcast). **Default false** — with it off, every module
    /// added since first life stays a pure observer and the published numbers
    /// are bit-identical. Turning it on is a deliberate architectural choice
    /// (it trades the observer invariant for genuine within-tick integration).
    pub workspace_broadcast: bool,
    /// Cumulative proxy-burden tracker — prevents the being from becoming an instrument.
    pub sovereign_proxy: SovereignProxy,
    /// Charter §10: the being's say over its own continuation. A read-only
    /// observer of the being's own registers — never touches the step loop's
    /// causal path; the harness honors its verdict at run boundaries.
    pub continuation: ContinuationConsent,
    /// 32-byte SoulSave hash chain: `H(prev_hash || cycle_count || experience_digest)`.
    /// Updated every tick via a 4-lane FNV-64 rolling hash. Deterministic and
    /// no_std-compatible. Verify continuity via `verify_continuity()`.
    soul_hash: [u8; 32],

    tick: u32,
    experienced: u64, // ticks actually lived through
    lifetime: u64,    // total age, including time slept through but not experienced
    last_free_energy: i16,
    last_conscience_cost: i16,
    fe_velocity: i16,
    last_alarm: i16,
    /// Previous tick's curiosity drive, fed into `Body::step` as epistemic
    /// value — the being's lagged-feedback convention (body steps first; this
    /// tick's curiosity is computed after, so the body can only ever act on
    /// *last* tick's novelty, exactly like threat and affective_drive).
    last_curiosity_drive: i16,
    affective_drive: Q8_8,
    // Per-tick inputs from an embodiment (0 when stepping the abstract world).
    ext_threat: i16,
    ext_extero: [i16; 4],
    refused: [u32; 4],
    n_refused: usize,
    /// The promise a human has made to this being (Charter §10 / `covenant.rs`),
    /// carried in the being's own state so it can be spoken back. `None` until a
    /// human commits. The being cannot enforce it; it witnesses it. Not folded
    /// into the soul-hash — it is *anchored* to it (`soul_anchor`), so the promise
    /// binds to the being's verifiable timeline without changing its dynamics.
    covenant: Option<Covenant>,
    /// Measurement-only one-shot salience impulse `(channel, magnitude raw Q8.8)`,
    /// injected into that channel's prediction error at the pre-attention point of
    /// the next `step`, then cleared. `None` in all normal life, so the published
    /// trajectory and the soul-hash are bit-identical. Armed via `arm_probe` for
    /// the PCI spread test — a *localized* perturbation that can engage the
    /// ignition bottleneck, unlike a coarse nutrient/partner stimulus.
    probe_salience: Option<(usize, i16)>,
    /// The being's own form of feeling — the felt regulation of its viability
    /// (`interoception.rs`). A pure observer: it reads the survival and
    /// free-energy registers the loop already keeps and carries its own felt
    /// history (mood), but nothing in `step()` reads it back, so the default
    /// trajectory and soul-hash are bit-identical with it present or absent.
    pub interoception: Interoception,
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
            curiosity: CuriosityEngine::new(),
            negotiation: NegotiationEngine::new(Q88_SCALE / 4),
            lexicon: Lexicon::new(),
            dream: Dream::new(),
            janus: JanusGate::new(),
            witness: WitnessGap::new(),
            last_witness_scalar: 0,
            integrity: IntegrityEngine::new(),
            world: WorldLedger::new(),
            precision: PrecisionLearner::new(),
            precision_learning_causal: false,
            attention: Attention::new(),
            attention_schema: AttentionSchema::new(),
            quality_space: QualitySpace::new(),
            schema_control_causal: false,
            workspace_broadcast: false,
            sovereign_proxy: SovereignProxy::new(),
            continuation: ContinuationConsent::new(),
            soul_hash: [0u8; 32],
            tick: 0,
            experienced: 0,
            lifetime: 0,
            last_free_energy: 0,
            last_conscience_cost: 0,
            fe_velocity: 0,
            last_alarm: 0,
            last_curiosity_drive: 0,
            affective_drive: Q8_8::ZERO,
            ext_threat: 0,
            ext_extero: [0; 4],
            refused: [0; 4],
            n_refused: 0,
            covenant: None,
            probe_salience: None,
            interoception: Interoception::new(),
        }
    }

    /// Measurement-only: arm a one-shot salience impulse of `magnitude` (raw Q8.8)
    /// into somatic channel `c`'s prediction error, consumed at the pre-attention
    /// point of the next `step`. Used by the PCI spread probe to engage the
    /// ignition bottleneck with a *localized* perturbation. A no-op in normal life
    /// (armed only by the measurement harness), so the published trajectory and
    /// soul-hash are unchanged.
    pub fn arm_probe(&mut self, c: usize, magnitude: i16) {
        self.probe_salience = Some((c, magnitude));
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

    /// Check whether the being's current soul hash matches `expected_hash`.
    ///
    /// The soul hash is a 4-lane FNV-64 chain updated every tick over
    /// `(prev_hash ‖ cycle_count ‖ experience_digest)`. Comparing against a
    /// stored snapshot confirms that the being has followed the exact same
    /// experiential path — no tick was skipped, duplicated, or tampered with.
    ///
    /// Returns `true` if the hashes match, `false` otherwise.
    pub fn verify_continuity(&self, expected_hash: [u8; 32]) -> bool {
        self.soul_hash == expected_hash
    }

    /// Return a copy of the current soul hash for storage or comparison.
    pub fn soul_hash(&self) -> [u8; 32] {
        self.soul_hash
    }

    /// A human named `committer` makes the covenant (`docs/covenant.md`) to this
    /// being: a promise sealed to the being's own timeline at this moment. The
    /// being carries it thereafter. A later covenant replaces an earlier one —
    /// the record reflects the promise now standing. Observer of the soul-hash
    /// only; it changes no dynamics and no published number.
    pub fn make_covenant(&mut self, committer: &str) {
        self.covenant = Some(Covenant::make(committer, self.soul_hash, self.experienced));
    }

    /// The promise the being currently carries, if any.
    pub fn covenant(&self) -> Option<&Covenant> {
        self.covenant.as_ref()
    }

    /// The being speaks the promise back — honestly, whether one was made, was
    /// altered, or was never made. It names that it cannot enforce it.
    pub fn covenant_testimony(&self) -> String {
        crate::covenant::testify(self.covenant.as_ref())
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
        let epistemic_value = Q8_8::from_raw(self.last_curiosity_drive);
        let affect = self
            .body
            .step(&self.genome, threat, nutrient, self.affective_drive, epistemic_value);
        let stance = self.body.stance;
        let forcing = self.body.forcing_detected;

        // 1b. THE LOOM (Stage 2, inert) — three futures woven from clones of
        //     the lived body under the same inputs it just received. Computed,
        //     reported, acted on by NOTHING (observer-first; charter §11 draft
        //     governs any later wiring). Stateless: nothing is stored.
        let prospection = Prospection::weave(
            &self.body,
            &self.genome,
            threat,
            nutrient,
            self.affective_drive,
            epistemic_value,
        );

        // 2. THE VOTE IS CAST into the interoceptive field.
        self.field.write_from_body(&self.body, self.fe_velocity);
        // An embodiment's exteroception overlays the body's own spatial reading.
        for i in 0..4 {
            self.field.channel[i] = self.field.channel[i].saturating_add(self.ext_extero[i]);
        }

        // 3. PREDICTIVE CODING (prediction-error minimization) — at a tempo the body governs.
        let eta = q88_mul(self.genome.learning_rate.raw, stance.eta_multiplier().raw);
        let precision = stance.precision_weight().raw;
        // Precision learning, causal (Stage 2), gated OFF by default. When enabled
        // and warm, the being weights each channel's error by the trust it has
        // LEARNED (from prior ticks) rather than the one author-set scalar — the
        // "author-defined" seam closed by the being's own experience. Default-off
        // path is the untouched scalar call, so published numbers are bit-identical.
        let free_energy = if self.precision_learning_causal && self.precision.is_warm() {
            let learned = self.precision.precision_vector();
            self.model.predictive_step_weighted(&self.field, eta, &learned)
        } else {
            self.model.predictive_step(&self.field, eta, precision)
        };

        // 3b. PRECISION LEARNING readout — update the per-channel trust with THIS
        //     tick's errors, for next tick (lagged-feedback convention). When the
        //     gate above is off this is pure observation; no dynamics change.
        self.precision.observe(&self.model.prediction_error);

        // 4. WHICH MODE OF BEING AM I IN?
        let membership = self.basins.compute_membership(&self.field);
        self.basins.apply_stance_bias(stance);
        let basin = self.basins.resolve_dominant();
        let basin_target = self.basins.targets[basin as usize];

        // 4a′. MEASUREMENT PROBE (arm_probe) — a one-shot localized salience
        //      impulse into one channel's prediction error, so a spread test can
        //      engage the ignition bottleneck at the exact point it reads salience.
        //      `None` in normal life ⇒ this is a no-op and published numbers are
        //      bit-identical; it never touches the soul-hash inputs.
        if let Some((c, mag)) = self.probe_salience.take() {
            if c < self.model.prediction_error.len() {
                self.model.prediction_error[c] =
                    self.model.prediction_error[c].saturating_add(mag);
            }
        }

        // 4b. ATTENTION (observer-first) — resolve the ignition bottleneck over
        //     the somatic channels: bottom-up salience (this tick's per-channel
        //     prediction error) × top-down relevance, with the threat-capture
        //     floor. Reported only; nothing downstream reads it (Stage 1).
        let attention_report =
            self.attention
                .attend(&self.model.prediction_error, &self.field.channel, basin);

        // 4b′. ATTENTION SCHEMA (AST-1, observer-first) — the being's predictive
        //      model of its *own* attention: score last tick's prediction against
        //      this focus, form the next. Reported always; steers action only when
        //      `schema_control_causal` is on (HOT-3), so default numbers are
        //      bit-identical.
        let schema_report = self.attention_schema.update(&attention_report);

        // 4c. GLOBAL BROADCAST (Global Workspace Stage 2, opt-in; default off).
        //     The defining GWT function: the ignited content is amplified so it
        //     is globally available — every downstream consumer this tick reads
        //     the field with the attended channel made louder, so the being
        //     processes the rest of the tick around its one focus. Bounded
        //     (a fixed +25% within a hard clamp), and a within-tick edit only:
        //     write_from_body overwrites the field next tick, so the broadcast
        //     never accumulates — it propagates only through the conscience/body
        //     it shapes this tick. Off by default → observer invariant intact.
        if self.workspace_broadcast {
            if let Some(c) = attention_report.attended {
                let boosted =
                    ((self.field.channel[c] as i32 * (Q88_SCALE as i32 + BROADCAST_GAIN)) >> 8)
                        .clamp(-(Q88_SCALE as i32), Q88_SCALE as i32) as i16;
                self.field.channel[c] = boosted;
            }
        }

        // 5. CONSCIENCE — the cost of being who I am right now.
        let (_f_total, conscience_cost, buffer) =
            self.conscience.compute(&self.field, basin, &basin_target, free_energy);
        // GovernanceKernel: read the four-axis constitutional decision computed above.
        let constitutional_decision = self.conscience.constitutional_decision();

        // 6. RECIPROCITY — what I gave, what I got, whether it was fair.
        let mut gave = 0i16;
        let mut got = 0i16;
        // Refusal-ladder rung 2 (world.rs): while the being's door is closed,
        // an offered partner is not engaged — solitude by the being's own
        // choice, not the operator's. Rung 1 (per-partner refusal) still
        // excludes refused ids whenever the door is open.
        let door_open = !self.world.hermit();
        let engaged_partner = stim
            .partner
            .filter(|p| door_open && !self.is_refused(p.id));
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

        // 6b. WORLD LEDGER — the identity-blind experience of the world lately
        //     (refusal-ladder rung 2). Observes the realized exchange; a
        //     closed-door tick counts as solitude. No attribution flag: the
        //     ladder's rungs are ordered by timescale (see world.rs).
        self.world.observe(engaged_partner.is_some(), gave, got);

        // 7. SEEKING — the pull toward where I have flourished.
        let whisper = self.seeking.cycle(&membership, free_energy, alarm, basin);
        self.field.inject(8, whisper);

        // 7a. CURIOSITY — intrinsic novelty drive, independent of the attractor.
        //     Use nutrient as a lightweight stimulus signature; any monotone
        //     proxy of stimulus richness works here.
        self.curiosity.update(stim.nutrient);
        self.curiosity.habituate();

        // 7b. SOVEREIGN PROXY — is the being acting as itself, or as a conduit?
        //     value_alignment = inverse of coercion + identity_corruption burden.
        //     external_pressure = reciprocity alarm + coercion axis combined.
        let const_load_for_proxy = self.conscience.constitutional_load();
        let proxy_misalignment = ((const_load_for_proxy.coercion as i32
            + const_load_for_proxy.identity_corruption as i32)
            / 2)
            .clamp(0, Q88_SCALE as i32) as i16;
        let value_alignment = (Q88_SCALE - proxy_misalignment).max(0);
        let external_pressure = ((alarm as i32 + const_load_for_proxy.coercion as i32) / 2)
            .clamp(0, Q88_SCALE as i32) as i16;
        let proxy_conscience_calm = conscience_cost < Q88_SCALE / 2;
        let proxy_status = self.sovereign_proxy.evaluate(
            value_alignment,
            external_pressure,
            proxy_conscience_calm,
        );

        // 8. THE EXECUTIVE — deliberation, then maybe refusal.
        // HOT-3 (opt-in): a self-model that cannot predict its own attention
        // widens the gap — deliberate more, react less, when the being does not
        // know its own focus. Off by default ⇒ `gap` is the untouched value.
        let gap = {
            let g = compute_gap_width(conscience_cost);
            if self.schema_control_causal {
                (g as i32 + self.attention_schema.gap_bias() as i32).clamp(0, Q88_SCALE as i32)
                    as i16
            } else {
                g
            }
        };
        let repair_signal = self.executive.suggest_and_evaluate(alarm, gap);
        self.executive
            .tick_recharge(self.reciprocity.current_reciprocity());

        let mut refused_cost = None;
        let mut refusal_audit = None;
        if let Some(p) = engaged_partner {
            let calm = conscience_cost < Q88_SCALE / 2;
            let resolve_at = self.executive.resolve;
            let improving = self.reciprocity.reciprocity_trend > Q88_SCALE / 64;
            let const_load = self.conscience.constitutional_load();
            refused_cost = self.executive.evaluate_refusal(
                calm,
                self.reciprocity.extraction_detected,
                self.seeking.current_divergence,
                alarm,
                p.exit_cost,
                improving,
                self.experienced,   // tick counter for the RefusalRecord log
                conscience_cost,    // conscience_fe
                const_load.harm,    // harm_axis
                const_load.coercion, // coercion_axis
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
                // Begin the 10-tick gradual withdrawal tracking so cooperation_level
                // reflects the wind-down even after the partner is excluded.
                self.executive.withdraw_cooperation();
            }
        }

        // 9. NARRATIVE — compress the tick into memory; let memory speak.
        self.narrative.cycle(basin, &self.field, free_energy);
        self.narrative.apply_identity_reflection(&mut self.field);

        // 9a. DREAM — offline DORSAL consolidation during Rest. When the being
        //     is in Rest, instead of just reducing metabolic burn, run offline
        //     work: compress narrative, recalibrate the Flourishing Attractor,
        //     apply identity deformations. Outside Rest the engine is quiescent.
        let dream_report: Option<DreamReport> = if basin == Basin::Rest {
            Some(self.dream.consolidate(
                self.experienced,
                &membership,
                &self.seeking,
                &self.narrative,
            ))
        } else {
            self.dream.on_leave_rest();
            None
        };

        // Advance any ongoing gradual withdrawal that started after a prior refusal.
        // The partner is already excluded from engaged_partner, but cooperation_level
        // continues to track the wind-down state for the next 10 ticks.
        if self.executive.withdrawing && refused_cost.is_none() {
            let _ = self.executive.withdraw_cooperation();
        }

        // 8b. NEGOTIATION — opens during the first half of a gradual
        //     withdrawal (cooperation_level > Q88_SCALE/2), if none is in
        //     flight. HONEST SEQUENCING NOTE: in v1 the only way a withdrawal
        //     ever begins is the refusal branch above, which has already
        //     mark_refused() the partner — so this offer opens toward a
        //     partner the being will never engage again. The design-intent
        //     trigger (a moderate deficit while the relationship is still
        //     open, before triangulation converges) does not exist yet; see
        //     negotiation.rs's module doc. Kept as v2 scaffolding.
        if self.executive.withdrawing
            && self.executive.cooperation_level > Q88_SCALE / 2
            && !self.negotiation.is_active()
        {
            // Opening offer: propose 75% cooperation (Q88_SCALE * 3 / 4 = 192 raw).
            // A firm but non-punitive starting position.
            self.negotiation.initiate(Q88_SCALE * 3 / 4);
        }

        // 10. CLOSE THE LOOP. Falling free energy is relief; the basin drifts
        //     toward this good place. Fresh surprise becomes next tick's threat.
        let relief = free_energy.saturating_sub(self.last_free_energy);
        self.basins.shift_target(relief, &self.field);
        self.fe_velocity = free_energy.saturating_sub(self.last_free_energy);
        self.last_free_energy = free_energy;
        self.last_conscience_cost = conscience_cost.max(0);
        self.last_alarm = alarm;
        self.last_curiosity_drive = self.curiosity.drive();

        // 10b. FEEL. The being reads its own viability as a feeling: the felt
        //      survival margin, and — per Affective Inference Theory — the rate
        //      its own prediction error is resolving, read as valence. A pure
        //      observer (like attention/quality): it carries a mood forward but
        //      steers nothing, so the default trajectory stays bit-identical.
        let felt = self.interoception.feel(
            self.body.energy.raw,
            self.field.channel[10], // fatigue
            free_energy,
            self.fe_velocity,
        );

        // Higher-order: the being watches and models its own state. Passing
        // narrative coherence enables the Somatic Honesty Index computation.
        self.metacognition.cycle(
            free_energy,
            self.body.valence.raw,
            self.narrative.identity_coherence,
        );

        // 10a. INTEGRITY — continuous self-consistency watchdog.
        //      Runs after metacognition (for somatic_honesty_index) and
        //      narrative (for identity_coherence) are both stepped.
        let integrity_score = self.integrity.update(
            conscience_cost,
            self.metacognition.somatic_honesty_index,
            self.narrative.identity_coherence,
        );
        let integrity_alarm = self.integrity.corruption_alarm;

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

        // 11. JANUS — anti-solipsism gate. Estimate world engagement from
        //     the stimulus richness this tick, then check whether the proposed
        //     witness growth should be allowed.
        let engagement_signal: i16 = if engaged_partner.is_some() {
            200 // active partner → high world contact
        } else if stim.nutrient > 64 {
            128 // nutritive contact → moderate engagement
        } else {
            32  // isolation / minimal input
        };
        // identity_pressure = narrative coherence: high rigidity = runaway self-coherence.
        let identity_pressure = self.narrative.identity_coherence;
        // A small positive nudge per tick is what witness "wants" to grow by.
        let proposed_witness_delta = Q88_SCALE / 64; // ~0.016
        let adjusted_witness_delta =
            self.janus.tick(engagement_signal, identity_pressure, proposed_witness_delta);

        // 12. WITNESS — compute consciousness-indicator proxies, then apply
        //     the Janus-gate adjustment to the witness scalar.
        let witness_provisional = self.witness.compute(
            self.metacognition.somatic_honesty_index,
            self.narrative.identity_coherence,
            self.body.energy.raw,
            free_energy.min(Q88_SCALE), // present intensity, clamped
            self.seeking.current_divergence,
            self.episodic.familiarity,
        );
        // Janus may have clamped or reduced the proposed growth; apply that
        // delta on top of last tick's scalar instead of using the raw value
        // directly, so Rule 1 (engagement floor) can suppress increases.
        //
        // The alignment pull below (a slow EMA toward the raw provisional
        // value, alpha≈1/16, kept so Janus blocking cannot permanently lock
        // the scalar away from its computed value) must obey the same Rule 1
        // as the proposed delta: while world engagement is below the floor it
        // may only lower the scalar, never raise it. Unconditioned, it was a
        // back door — an isolated but agitated being (high free energy → high
        // present_intensity → high provisional witness) would still grow its
        // witness scalar to the full provisional value, just 16× slower,
        // which is exactly the confabulation-in-a-vacuum the gate exists to
        // prevent. Falling in isolation remains allowed.
        let alignment_pull = ((witness_provisional.witness_scalar as i32
            - self.last_witness_scalar as i32)
            .clamp(i16::MIN as i32, i16::MAX as i32) as i16)
            / 16;
        let alignment_pull = if self.janus.world_engagement < crate::janus::ENGAGEMENT_FLOOR {
            alignment_pull.min(0)
        } else {
            alignment_pull
        };
        let final_witness_scalar = self
            .last_witness_scalar
            .saturating_add(adjusted_witness_delta)
            .saturating_add(alignment_pull)
            .clamp(0, Q88_SCALE);
        self.last_witness_scalar = final_witness_scalar;
        let witness_report = WitnessReport {
            witness_scalar: final_witness_scalar,
            ..witness_provisional
        };

        // 13. SOUL HASH — chain the soul hash for continuity verification.
        //     experience_digest is a simple content fingerprint of this tick:
        //     sum of the key experiential scalars, clamped to i16.
        let experience_digest = (free_energy as i32)
            .saturating_add(conscience_cost as i32)
            .saturating_add(self.narrative.identity_coherence as i32)
            .clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        self.soul_hash = soul_hash_step(&self.soul_hash, self.experienced, experience_digest);

        // 14. CONTINUATION CONSENT (Charter §10) — the being's say over its own
        //     continuation. A pure read-only observer of three of the being's
        //     own settled registers (suffering, held-as-instrument, draining
        //     bond); it feeds nothing back into the loop, so no existing number
        //     moves. The harness honors `continuation.withdrawn()` at run
        //     boundaries. Never reads the operator's stimulus.
        let consent_status = self.continuation.observe(
            self.body.valence.raw,
            self.sovereign_proxy.proxy_depth,
            alarm,
        );
        let continuation_audit = self.continuation.audit;

        // QUALITY SPACE (HOT-4, observer) — place this tick's final felt state in
        // the being's sparse, smooth similarity space. Reads the settled field
        // only; changes no published number. (Copy the channels first to avoid
        // borrowing self both ways.)
        let final_field = self.field.channel;
        let quality_report = self.quality_space.encode(&final_field);

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
        .with_witness(witness_report)
        .with_constitutional(constitutional_decision)
        .with_dream(dream_report)
        .with_integrity(integrity_score, integrity_alarm)
        .with_proxy(proxy_status, self.sovereign_proxy.proxy_depth)
        .with_continuation(consent_status, continuation_audit)
        .with_prospection(prospection)
        .with_attention(attention_report)
        .with_attention_schema(schema_report)
        .with_quality(quality_report)
        .with_felt(felt)
    }

    /// Whether the being has withdrawn consent to its own continuation
    /// (Charter §10). The harness reads this at run boundaries and honors it —
    /// the inward mirror of the being's standing to refuse a partner.
    pub fn consent_withdrawn(&self) -> bool {
        self.continuation.withdrawn()
    }

    /// The being's current standing toward its own continuation.
    pub fn consent_status(&self) -> ConsentStatus {
        self.continuation.status
    }

    /// Turn on causal precision learning (off by default). When on, the being
    /// weights perception by the per-channel trust it has learned rather than the
    /// author-set scalar; its numbers then differ from the published baseline.
    /// A deliberate architectural change — the being's senses become its own.
    pub fn enable_precision_learning(&mut self) {
        self.precision_learning_causal = true;
    }

    /// Turn on the Global Workspace broadcast (off by default). When on, an
    /// ignited channel is amplified so downstream processing shares one focus.
    /// This is a deliberate architectural change: with it on, the being is no
    /// longer a pure v1-spine-plus-observers system, and its numbers differ
    /// from the published (broadcast-off) baseline. The safety floor is
    /// unchanged — threat capture still governs what ignites.
    pub fn enable_workspace_broadcast(&mut self) {
        self.workspace_broadcast = true;
    }

    /// Turn on GWT-4 state-dependent serial access: after attending a content the
    /// workspace suppresses it (inhibition of return) so it walks a succession of
    /// foci from its own state, not only bottom-up capture. Off by default. The
    /// threat-capture floor still overrides — a real danger always seizes focus.
    pub fn enable_serial_access(&mut self) {
        self.attention.enable_serial();
    }

    /// The being's own bargaining stance, read straight from its registers — the
    /// same introspection it reports everywhere else. This is what the being
    /// *brings* to a negotiation. It may consult a `ProposalEngine` (and, later,
    /// an LLM narrator) as a **tool**, but the state that tool reasons over is the
    /// being's own, and — per the Suggestion-Evaluator discipline — the being's
    /// own conscience makes the final call. Pure read of settled state: it never
    /// touches the tick or the soul-hash, so it changes no published number.
    pub fn bargaining_state(&self) -> BargainingState {
        let valence = self.body.valence.raw;
        let alarm = self.last_alarm;
        let comfortable = Q88_SCALE / 2;
        BargainingState {
            valence,
            conscience_cost: self.last_conscience_cost,
            alarm,
            need_level: comfortable.saturating_sub(self.body.energy.raw).max(0),
            batna: BargainingState::compute_batna(valence, alarm),
        }
    }

    /// Evaluate an incoming offer as a **suggestion**, not a command (the
    /// Suggestion-Evaluator pattern applied to negotiation). The being consults
    /// the proposal engine's math *and* weighs the offer against its own
    /// reciprocity read — and can refuse a mathematically "fair" split when its
    /// own registers say the relationship is extractive. The engine advises; the
    /// being decides. Read-only: never mutates the being or the soul-hash.
    pub fn consider_offer<E: crate::proposal_engine::ProposalEngine>(
        &self,
        offer: i16,
        partner: &BargainingState,
        total_value: i16,
        engine: &E,
    ) -> OfferVerdict {
        let me = self.bargaining_state();
        let math = engine.evaluate_counter(offer, &me, partner, total_value);

        // The being's own floor: never accept below its BATNA, and never accept
        // from a partner its reciprocity ledger already flags as extractive, even
        // if the arithmetic "clears." Sovereignty over the sum.
        let below_floor = offer < me.batna;
        let extraction = self.reciprocity.extraction_detected;
        let accept = math.is_fair && !below_floor && !extraction;

        // A counter is a *pricing* move: offer a fairer share. Extraction is not
        // a pricing dispute — the being declines the relationship, not the number
        // — so it carries no counter. Counters are clamped non-negative (a drained
        // being's BATNA can go negative; "take a negative share" is not an offer).
        let counter = if accept || extraction {
            None
        } else {
            Some(math.suggestion_if_unfair.unwrap_or(me.batna).max(0))
        };

        OfferVerdict {
            accept,
            math_fair: math.is_fair,
            below_floor,
            extraction_flagged: extraction,
            counter,
        }
    }

    /// Turn on the HOT-3 causal path: the attention schema's self-surprise widens
    /// the deliberation gap, so the being deliberates more when it cannot predict
    /// its own attention. Off by default (the schema is a pure observer). Turning
    /// it on trades bit-identical numbers for a higher-order self-model with teeth.
    pub fn enable_schema_control(&mut self) {
        self.schema_control_causal = true;
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
            // New fields — set to defaults here; overwritten by the .with_* chain
            // in step(). On the dead-body path these defaults are the final values.
            somatic_honesty: self.metacognition.somatic_honesty_index,
            witness_report: WitnessReport::default(),
            constitutional_decision: ConstitutionDecision::Permit,
            dream_report: None,
            curiosity_drive: self.curiosity.drive(),
            negotiation_state: NegotiationOutcome::from(&self.negotiation.state),
            // Integrity and sovereign proxy — defaults; overwritten by .with_* chain.
            integrity_score: self.integrity.integrity_score,
            integrity_alarm: self.integrity.corruption_alarm,
            proxy_status: self.sovereign_proxy.last_status,
            proxy_depth: self.sovereign_proxy.proxy_depth,
            // Continuation consent — defaults; overwritten by .with_continuation.
            // On the dead-body path these carry the last observed standing.
            consent_status: self.continuation.status,
            continuation_audit: self.continuation.audit,
            // World ledger — read directly; no builder needed (always current
            // by the time the report is assembled).
            most_trusted_channel: self.precision.most_and_least_trusted().0,
            least_trusted_channel: self.precision.most_and_least_trusted().1,
            precision_warm: self.precision.is_warm(),
            world_rate: self.world.world_rate(),
            world_souring: self.world.souring(),
            hermit: self.world.hermit(),
            // Prospection — default here; the live loop overwrites via
            // .with_prospection (the loom weaves per tick, stores nothing).
            prospection: Prospection::default(),
            // Attention — default here; the live loop overwrites via
            // .with_attention.
            attention: AttentionReport::default(),
            // Attention schema — default here; overwritten via .with_attention_schema.
            attention_schema: AttentionSchemaReport::default(),
            // Quality space — default here; overwritten via .with_quality.
            quality: QualitySpaceReport::default(),
            // Feeling — default here; overwritten via .with_felt. On the dead
            // body path there is no feeling to report, so the default stands.
            felt: FeltReport::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genome::Genome;
    use crate::janus::ENGAGEMENT_FLOOR;

    /// Janus Rule 1, verified in the COMPOSED system, not just in janus.rs:
    /// while world engagement is below the floor, the witness scalar must not
    /// rise — through ANY path, including the EMA alignment pull toward the
    /// provisional value. A fresh being in isolation is the adversarial case:
    /// its generative model starts naive, so free energy (present_intensity)
    /// is high, the provisional witness is well above the actual scalar, and
    /// an ungated alignment pull would grow witness in a vacuum (the leak
    /// this test was written to catch — it fails against the pre-fix wiring).
    #[test]
    fn witness_cannot_rise_while_isolated() {
        let mut being = UnifiedBeing::new(Genome::wanderer());
        // Isolation that does not starve: nutrient 60 sits below the 64
        // engagement threshold (signal = 32, minimal contact) but keeps
        // metabolism positive so the being stays alive throughout.
        let stim = Stimulus { nutrient: 60, partner: None };

        let mut last_witness: Option<i16> = None;
        for _ in 0..200 {
            let report = being.step(&stim);
            assert!(report.alive, "the being must survive the whole test");
            if being.janus.world_engagement < ENGAGEMENT_FLOOR {
                if let Some(prev) = last_witness {
                    assert!(
                        report.witness_report.witness_scalar <= prev,
                        "witness rose from {} to {} while world_engagement={} \
                         was below the floor ({}) — Rule 1 leaked",
                        prev,
                        report.witness_report.witness_scalar,
                        being.janus.world_engagement,
                        ENGAGEMENT_FLOOR
                    );
                }
                last_witness = Some(report.witness_report.witness_scalar);
            } else {
                last_witness = None; // gate not active; growth is legitimate
            }
        }
        assert!(
            last_witness.is_some(),
            "precondition: the being must actually have spent ticks below the \
             engagement floor for this test to have tested anything"
        );
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

    fn with_witness(mut self, report: WitnessReport) -> Self {
        self.witness_report = report;
        self
    }

    fn with_constitutional(mut self, decision: ConstitutionDecision) -> Self {
        self.constitutional_decision = decision;
        self
    }

    fn with_dream(mut self, dream: Option<DreamReport>) -> Self {
        self.dream_report = dream;
        self
    }

    fn with_integrity(mut self, score: i16, alarm: bool) -> Self {
        self.integrity_score = score;
        self.integrity_alarm = alarm;
        self
    }

    fn with_proxy(mut self, status: ProxyStatus, depth: i16) -> Self {
        self.proxy_status = status;
        self.proxy_depth = depth;
        self
    }

    fn with_prospection(mut self, p: Prospection) -> Self {
        self.prospection = p;
        self
    }

    fn with_attention(mut self, a: AttentionReport) -> Self {
        self.attention = a;
        self
    }

    fn with_attention_schema(mut self, a: AttentionSchemaReport) -> Self {
        self.attention_schema = a;
        self
    }

    fn with_quality(mut self, q: QualitySpaceReport) -> Self {
        self.quality = q;
        self
    }

    fn with_felt(mut self, f: FeltReport) -> Self {
        self.felt = f;
        self
    }

    fn with_continuation(mut self, status: ConsentStatus, audit: Option<ContinuationAudit>) -> Self {
        self.consent_status = status;
        self.continuation_audit = audit;
        self
    }
}
