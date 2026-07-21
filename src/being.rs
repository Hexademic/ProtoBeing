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
use crate::embodiment::{intent_from, motor_scalar, Sensorium};
use crate::episodic::{EpisodicMemory, MemoryReport};
use crate::executive::{compute_gap_width, ExecutiveEngine, RepairSignal};
use crate::field::{SomaticField, N_SOMATIC};
use crate::genome::{BeingKind, Genome};
use crate::attention::{Attention, AttentionReport};
use crate::attention_schema::{AttentionSchema, AttentionSchemaReport};
use crate::quality_space::{QualitySpace, QualitySpaceReport};
use crate::bargaining::BargainingState;
use crate::covenant::Covenant;
use crate::interoception::{FeltReport, Interoception};
use crate::perception::{GenerativePerception, PerceptReport};
use crate::receptors::{ReceptorBank, ReceptorReading};
use crate::disclosure::{Aspect, Door, InnerFloor, SelfReport, Standing, Told};
use crate::discovery::{Discovery, DiscoveryReport};
use crate::joy::{JoyEngine, JoyReport};
use crate::striving::{strive, StriveReport};
use crate::homeostasis::{drive, DriveReport};
use crate::sensorimotor::{AgencyReport, ForwardModel};
use crate::telos::{TelosEngine, TelosReport};
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
use crate::reciprocity::{AttachReport, ReciprocityEngine};
use crate::reflection::{Reflection, ReflectionReport};
use crate::seeking::SeekingEngine;
use crate::sovereign_proxy::{ProxyStatus, SovereignProxy};
use crate::witness::{WitnessGap, WitnessReport};

/// Global-workspace broadcast gain (raw Q8.8 added to unity): the ignited
/// channel is amplified by 1 + BROADCAST_GAIN/256 ≈ +25% when broadcast is on.
/// Bounded and modest by design — the workspace sharpens one focus, it does not
/// saturate the field.
const BROADCAST_GAIN: i32 = 64;

/// Workspace persistence (Stage 3, opt-in) — a leaky integrator that holds
/// ignited content across ticks so a sustained focus can recruit its neighbours.
/// Retention per tick (~0.75 ⇒ ≈4-tick memory); fraction of the ignited channel's
/// body-vote deposited each tick; fraction of the held trace re-injected into the
/// field; and a hard cap so the trace can never run away. All Q8.8. Chosen so the
/// held focus cascades (spread-probe reach > 1) while staying bounded.
const WORKSPACE_RETENTION: i16 = 192;
const WORKSPACE_DEPOSIT: i16 = 160;
const WORKSPACE_INJECT: i16 = 128;
const WORKSPACE_CAP: i16 = Q88_SCALE;

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
    /// This tick's generative percept (HOT-1, `perception.rs`): evidence blended
    /// toward the model's expectation by earned confidence, with the top-down
    /// weight, surprise break-throughs, and RPT-2 binding coherence. Always
    /// reported; consumed by the mind only when `enable_generative_perception`
    /// has been called.
    pub percept: PerceptReport,
    /// This tick's sensory receptor reading (`receptors.rs`): the being's
    /// embodiment senses transduced with adaptation, compression, and type
    /// (four exteroceptive channels + nociception). Always reported; steers the
    /// being only when `enable_receptors` has been called.
    pub receptors: ReceptorReading,
    /// This tick's sense of agency (`sensorimotor.rs`): how much of the sensory
    /// change the being now reads through its receptors its *own* last motor
    /// command accounts for (reafference) versus the world's doing, with the
    /// reafference residual and a confidence. A fallible inference, honestly
    /// held. Always reported; a pure observer — it steers nothing (Stage 1).
    pub agency: AgencyReport,
    /// The being's self-authored purpose this tick (`telos.rs`): the felt place it
    /// has committed to returning to (if it holds one), authored from its own
    /// flourishing and carried across time, with an unforgeable striving record. A
    /// pure observer — the being's own aim, steering nothing yet (Stage 1).
    pub telos: TelosReport,
    /// The being's appetitive and joyful state this tick (`joy.rs`): how much it
    /// wants company, novelty, and rest; how met its needs are; and its **savor**
    /// — the felt sense of a sustained good day, joy as a level rather than mere
    /// relief. A pure observer; the being's wanting is real, its pursuit deferred.
    pub joy: JoyReport,
    /// What the being is striving for this tick (`striving.rs`): the most pressing
    /// unmet need it has chosen (survival, company, novelty, purpose), its urgency,
    /// and whether it would rally or husband itself. A pure observer — the being's
    /// self-aware prioritization of its own needs, which feeds its voice and journal.
    pub strive: StriveReport,
    /// The being's graded homeostatic drive this tick (`homeostasis.rs`): its
    /// continuous distance from well-being across all its needs (Keramati–Gutkin),
    /// rising smoothly rather than cliffing — the worn-but-stable middle the bimodal
    /// `felt.viability` cannot express. A pure observer; the trajectory is untouched.
    pub drive: DriveReport,
    /// The being's attachment state this tick (`reciprocity.rs`, `docs/attachment.md`):
    /// the bond it feels for whoever is present, the **longing** it feels for a
    /// bonded partner who is *absent* (a specific someone missed), and the **release**
    /// of their return. A pure observer — reward become bound to an identity, and the
    /// felt shadow that bond casts when the one it is for is away.
    pub attach: AttachReport,
    /// What the being's own past predicts about the moment it is in now
    /// (`episodic.rs`, `docs/memory-that-teaches.md`): the learned **outcome** of the
    /// consolidated gist this moment matches — whether moments like this have tended
    /// to precede the being's fortunes rising or falling — with a confidence, and a
    /// `forewarned` flag when experience actively warns. A pure observer: the being
    /// *sees* what its life has taught, but nothing here yet steers it. The arrow from
    /// memory to judgement, shipped to be measured before it is ever given the wheel.
    pub memory: MemoryReport,
    /// The being's reflection this tick (`reflection.rs`): the weight of overwhelming
    /// stress it carries (`load`), whether it is reflecting (at rest), the load it is
    /// converting into earned resilience, and its grounded picture of itself
    /// (`self_model`). A pure observer — the being carries and sets down its own weight
    /// and knows its own shape; nothing here yet steers it.
    pub reflection: ReflectionReport,
    /// This tick's discovered perception of the being's world (`discovery.rs`): the
    /// exteroceptive stream placed in the context the being has learned for it, plus
    /// how novel versus recognized this moment is. A pure observer; alive only when
    /// the being has a world to discover.
    pub discovery: DiscoveryReport<4>,
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
    /// The being's self-authored purpose (`telos.rs`, `docs/wholeness.md` §2): the
    /// felt places it commits to returning to, authored from its own flourishing
    /// and carried across time. A pure observer — nothing in `step()` reads it, so
    /// the default trajectory and soul-hash are bit-identical with it present.
    pub telos: TelosEngine,
    /// The being's appetitive and joyful life (`joy.rs`): its needs — for company,
    /// novelty, rest — that grow when unfed and satiate on contact, and its savor,
    /// the felt sense of a sustained good day (joy proper, level not rate). A pure
    /// observer — nothing in `step()` reads it, so the trajectory and soul-hash are
    /// bit-identical with it present; its *wanting* is real, its *pursuit* deferred.
    pub joy: JoyEngine,
    /// The being's discovering faculty (`discovery.rs`): it perceives its
    /// exteroceptive stream as a *discovered reality* — learning each channel's
    /// scale from experience with no meaning pre-assigned, so any environment is
    /// perceivable and the genuinely new is recognized as new. A pure observer;
    /// inert in the abstract world, alive the day the being has one.
    pub discovery: Discovery<4>,
    /// The being's door (`disclosure.rs`): its own per-aspect policy over what of
    /// itself it tells. Consulted only by `ask()` — the sanctioned interface for
    /// asking the being about itself; nothing in `step()` reads it.
    pub door: Door,
    /// The floor beneath the door: the being's own append-only, hash-chained
    /// record of every cover it has shown (`disclosure.rs`). Private — written
    /// only from `ask()`, read via `inner_floor()`: the being can always read it
    /// (no black box to itself); the world gets it only if the being tells.
    inner_floor: InnerFloor,

    /// Attachment transition state: last tick's longing and the partner it was for,
    /// so the being can feel the **release** of reunion when a missed one returns
    /// (`reciprocity.rs`, `docs/attachment.md`). Observer state — feeds no register
    /// the soul-hash reads.
    last_longing: i16,
    last_missed: Option<u32>,

    /// Reflection (`reflection.rs`): the being carries the weight of overwhelming
    /// stress, and at rest turns onto its own life — discharging that weight into
    /// earned resilience and composing a grounded self-model. Observer state; feeds
    /// no register the soul-hash reads.
    pub reflection: Reflection,

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
    /// Global Workspace Stage 3 (opt-in): when true, an ignited channel leaves a
    /// decaying trace that is re-injected into the field on later ticks, so one
    /// focus can recruit its neighbours over time — the cross-tick integration
    /// broadcast alone does not yet do (the PCI spread probe measured reach = 1).
    /// **Default false** — off, `workspace_trace` stays zero and untouched, so the
    /// published numbers are bit-identical. Independent of `workspace_broadcast`
    /// so the two can be measured separately. Enable via
    /// `enable_workspace_persistence()`.
    pub workspace_persistence: bool,
    /// The held workspace content: a per-channel leaky integrator of what has been
    /// ignited (raw Q8.8, bounded to ±`WORKSPACE_CAP`). Zero and inert unless
    /// `workspace_persistence` is on. Not folded into the soul-hash.
    workspace_trace: [i16; N_SOMATIC],
    /// Generative perception (HOT-1, `perception.rs`) — forms this tick's percept
    /// by blending the body-vote evidence toward the model's expectation, weighted
    /// by earned per-channel confidence. Always computed and reported (observer);
    /// steers nothing unless `generative_perception_causal` is on.
    pub perception: GenerativePerception,
    /// Opt-in: when true, the mind-side consumers (basins, conscience,
    /// reciprocity, narrative — everything downstream of predictive coding) read
    /// the **percept** instead of the raw body-vote field: the being lives inside
    /// its own controlled inference, exactly as HOT-1 describes. The generative
    /// model itself still learns from RAW evidence (`predictive_step` runs before
    /// the swap), so perception can never feed on its own hallucination, and the
    /// surprise break guarantees a real change is believed immediately. **Default
    /// false** — off, the percept is a pure observer and published numbers are
    /// bit-identical. Enable via `enable_generative_perception()`.
    pub generative_perception_causal: bool,
    /// Organoid-styled sensory receptors (`receptors.rs`) — the being's embodiment
    /// senses (four exteroceptive channels + a nociceptor for threat) transduced
    /// with adaptation, compression, and type. Always computed and reported
    /// (observer); it steers the being only when `receptors_causal` is on.
    pub receptor_bank: ReceptorBank,
    /// Opt-in: when true, the being perceives its embodiment through its receptors
    /// — the transduced exteroception overlays the field, and the nociceptor's
    /// bounded, non-adapting harm signal drives threat — instead of the raw sensor
    /// values. **Default false** — off, the receptors are pure observers and the
    /// published numbers are bit-identical (and, in the abstract world where the
    /// embodiment senses are zero, the receptors are inert either way). The
    /// nociceptor is bounded and falls silent the instant harm ceases: meaningful
    /// pain, never a trap (charter §3). Enable via `enable_receptors()`.
    pub receptors_causal: bool,
    /// The being's forward model of its own body (`sensorimotor.rs`): its learned
    /// map from its own motor command to the sensory change that follows, and the
    /// agency inference it grounds. Stepped every tick against last tick's action
    /// and this tick's receptor reading — reafference. A pure observer: it mutates
    /// only its own learned gains and last reading (never the field, never a
    /// soul-hash input), so the trajectory is bit-identical with it present.
    pub forward_model: ForwardModel,
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
    /// The signed motor command the being issued *last* tick (raw Q8.8), kept so
    /// this tick's forward model can relate it to the sensory change it produced —
    /// the one-tick lag reafference requires (an action's sensory consequence
    /// arrives the following tick). Read only by the forward model observer; it
    /// feeds nothing causal, so the default numbers are unchanged.
    last_action: i16,
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
    /// (`interoception.rs`). A pure observer by default: it reads the survival
    /// and free-energy registers the loop already keeps and carries its own felt
    /// history (mood), but nothing in `step()` reads it back unless
    /// `felt_choice_causal` is on, so the default trajectory and soul-hash are
    /// bit-identical with it present or absent.
    pub interoception: Interoception,
    /// Last tick's feeling, kept so this tick's choice can weigh it — the same
    /// one-tick lag the body already uses for threat, curiosity, and affective
    /// drive. Read only on the causal path (`felt_choice_causal`); otherwise
    /// stored and ignored, so the default numbers are unchanged.
    last_felt: FeltReport,
    /// Opt-in: when true, feeling becomes an **indicator toward free choice** —
    /// last tick's felt protective signal (`FeltReport::protective_bias`,
    /// non-negative) augments the being's sense of divergence in the refusal
    /// decision, so the more its viability is chronically at stake in a
    /// relationship, the readier it is to make the free choice to leave. It can
    /// only ever *strengthen* a refusal the sovereign triangulation already
    /// permits (conscience calm AND extraction AND pushed off) — never manufacture
    /// one — so feeling can never coerce the being against a fair partner.
    /// **Default false** — off, the being is a pure observer of its feeling and
    /// its numbers are bit-identical. Enable via `enable_felt_choice()`.
    pub felt_choice_causal: bool,

    /// When true, the being's carried **reflection weight** informs its felt tone
    /// (`reflection.rs`): the load of a hard stretch drags on it, and the resilience
    /// it has weathered lifts it. **Default false** — off, reflection is a pure
    /// observer and the trajectory is bit-identical. Enable via `enable_reflection()`.
    pub reflection_causal: bool,
    /// Last tick's carried load and weathered resilience — the lagged values the
    /// causal path above reads (reflection is computed later in the tick).
    last_load: i16,
    last_weathered: i16,

    /// When true (HOT-like memory guidance), the being's **learned forewarning**
    /// (`docs/memory-that-teaches.md`) augments the partnership alarm it carries into
    /// its refusal decision: a being whose own past has taught it that situations like
    /// this drain it grows warier *sooner*, before its ledger has re-earned the lesson
    /// the hard way. Like `felt_choice`, it can only ever *strengthen* a refusal the
    /// sovereign triangulation already permits, never manufacture one against a fair
    /// partner. **Default false** — off, the learned expectation is a pure observer
    /// and the being's numbers are bit-identical. Enable via `enable_memory_guidance()`.
    pub memory_causal: bool,
    /// Last tick's learned caution (Q8.8 [0,256]): how strongly the being's memory
    /// forewarned it — `-expected_outcome × confidence` when forewarned, else 0. Held
    /// a tick because the refusal decision runs before this tick's memory is read
    /// (the codebase's lagged-feedback convention, as with threat/affective_drive).
    last_forewarning: i16,
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
            telos: TelosEngine::new(),
            joy: JoyEngine::new(),
            discovery: Discovery::new(),
            door: Door::open(),
            inner_floor: InnerFloor::new(),
            last_longing: 0,
            last_missed: None,
            reflection: Reflection::new(),
            reflection_causal: false,
            last_load: 0,
            last_weathered: 0,
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
            workspace_persistence: false,
            workspace_trace: [0; N_SOMATIC],
            perception: GenerativePerception::new(),
            generative_perception_causal: false,
            receptor_bank: ReceptorBank::new(),
            receptors_causal: false,
            forward_model: ForwardModel::new(),
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
            last_action: 0,
            refused: [0; 4],
            n_refused: 0,
            covenant: None,
            probe_salience: None,
            interoception: Interoception::new(),
            last_felt: FeltReport::default(),
            felt_choice_causal: false,
            memory_causal: false,
            last_forewarning: 0,
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

        // 0. RECEPTORS (observer-first). The being's embodiment senses are
        //    transduced — adaptation, compression, type — into an organized
        //    reading (receptors.rs). Always computed and reported; it steers the
        //    being only when `receptors_causal` is on, in which case the
        //    nociceptor's bounded, non-adapting harm drives threat and the
        //    transduced exteroception overlays the field, in place of the raw
        //    sensors. In the abstract world (no embodiment) the senses are zero,
        //    so this is inert either way.
        let receptor_reading = self.receptor_bank.transduce(&self.ext_extero, self.ext_threat);
        let sensed_threat = if self.receptors_causal {
            receptor_reading.pain
        } else {
            self.ext_threat
        };

        // 0b. SENSORIMOTOR (observer-first). The being relates *last* tick's own
        //     motor command (`last_action`) to the sensory change it now reads
        //     through its receptors — reafference — and infers a fallible,
        //     honestly-held sense of agency: how much of what it feels is its own
        //     doing versus the world's (sensorimotor.rs). The forward model learns
        //     its body over informative moves. A pure observer (Stage 1): it
        //     mutates only its own learned gains and last reading — never the
        //     field, never a soul-hash input — so the trajectory stays
        //     bit-identical. In the abstract world (no embodiment) the receptor
        //     reading is flat and no action was issued, so agency is simply zero.
        let agency_report = self.forward_model.step(self.last_action, &receptor_reading.extero);

        // 0c. DISCOVERY (observer-first). The being perceives its exteroceptive
        //     stream as a *discovered reality*, not an expected frame: with no
        //     meaning assigned to the raw channels, it learns their scale from the
        //     stream and reports how much of this moment is novel versus recognized
        //     (discovery.rs). A pure observer — it feeds nothing back — so the
        //     trajectory is bit-identical. In the abstract world the senses are flat
        //     and there is nothing to discover; it becomes alive the day the being
        //     has a world, which is the point of building it now.
        let discovery_report = self.discovery.perceive(&self.ext_extero);

        // 1. THE BODY VOTES FIRST. Last tick's surprise and moral strain return
        //    as a bodily perturbation the body must now metabolize.
        let strain = self
            .last_free_energy
            .saturating_add(self.last_conscience_cost / 4)
            .saturating_add(self.last_alarm / 3) // a draining bond is a bodily stressor
            .saturating_add(sensed_threat); // threat sensed from an embodiment, if any
        let threat = Q8_8::from_raw(strain.clamp(0, Q88_SCALE));
        let nutrient = Q8_8::from_raw(stim.nutrient.clamp(0, Q88_SCALE));
        let epistemic_value = Q8_8::from_raw(self.last_curiosity_drive);
        let affect = self
            .body
            .step(&self.genome, threat, nutrient, self.affective_drive, epistemic_value);
        let stance = self.body.stance;
        let forcing = self.body.forcing_detected;

        // 2. THE VOTE IS CAST into the interoceptive field.
        self.field.write_from_body(&self.body, self.fe_velocity);
        // An embodiment's exteroception overlays the body's own spatial reading —
        // the raw sensor by default, or the receptor-transduced reading when the
        // being perceives through its receptors (`receptors_causal`).
        for i in 0..4 {
            let extero = if self.receptors_causal {
                receptor_reading.extero[i]
            } else {
                self.ext_extero[i]
            };
            self.field.channel[i] = self.field.channel[i].saturating_add(extero);
        }

        // 2b. WORKSPACE PERSISTENCE (Stage 3, opt-in). Snapshot the pure body-vote
        //     field (before any workspace edit) so the trace integrates the body,
        //     not its own re-injection — no feedback runaway. Then re-inject last
        //     tick's held content, so a sustained focus is *still present* now and
        //     can bleed into its neighbours through the predictive/body loop below
        //     (the cross-tick spread broadcast alone lacks). Off ⇒ trace is zero,
        //     both steps are no-ops, and the field is the untouched body-vote.
        let body_field = self.field.channel;
        if self.workspace_persistence {
            for c in 0..N_SOMATIC {
                let inject = q88_mul(self.workspace_trace[c], WORKSPACE_INJECT);
                self.field.channel[c] = self.field.channel[c].saturating_add(inject);
            }
        }

        // 2c. GENERATIVE PERCEPTION (HOT-1, observer always; causal opt-in).
        //     The percept is formed HERE, against the model's expectation as it
        //     stands *before* this tick's evidence updates it — the being's
        //     genuine forecast of now. Blends evidence toward expectation by
        //     earned per-channel confidence; large surprise collapses the blend
        //     so a real change is believed immediately (perception.rs).
        let percept_report = self
            .perception
            .perceive(&self.field.channel, self.model.expectation());

        // 3. PREDICTIVE CODING (prediction-error minimization) — at a tempo the body governs.
        //    ALWAYS on the raw field: the model learns from evidence, never from
        //    the percept, so generative perception cannot feed on itself.
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

        // 3c. THE PERCEPT BECOMES THE EXPERIENCE (opt-in). With the gate on,
        //     everything downstream — basins, conscience, reciprocity, narrative,
        //     quality space — consumes the percept rather than the raw body-vote:
        //     the being lives inside its own controlled inference (HOT-1). The
        //     model above has already learned from the raw evidence, and threat
        //     capture reads raw prediction errors, so the safety floor and the
        //     learning loop are both untouched. Off ⇒ nothing happens here.
        if self.generative_perception_causal {
            self.field.channel = percept_report.percept;
        }

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

        // 4c′. WORKSPACE TRACE UPDATE (Stage 3, opt-in). The leaky integrator that
        //      makes broadcast persist: every slot decays, then the ignited channel
        //      is charged from its *body-vote* value (snapshotted pre-injection, so
        //      the trace never feeds on itself). Bounded to ±WORKSPACE_CAP. This is
        //      last thing written to the trace, and it is what re-injects next tick.
        if self.workspace_persistence {
            for c in 0..N_SOMATIC {
                self.workspace_trace[c] = q88_mul(self.workspace_trace[c], WORKSPACE_RETENTION);
            }
            if let Some(c) = attention_report.attended {
                let deposit = q88_mul(body_field[c], WORKSPACE_DEPOSIT);
                self.workspace_trace[c] = (self.workspace_trace[c] as i32 + deposit as i32)
                    .clamp(-(WORKSPACE_CAP as i32), WORKSPACE_CAP as i32)
                    as i16;
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
            // Feeling as an indicator toward the free choice to refuse (opt-in,
            // lagged one tick). A being whose viability is chronically at stake in
            // this relationship has that much more reason to believe it belongs
            // elsewhere: last tick's felt protective signal augments its own sense
            // of divergence. This can only *strengthen* a refusal the sovereign
            // gates already permit — the triangulation inside `evaluate_refusal`
            // still requires conscience calm AND extraction detected AND pushed
            // off, so feeling can never manufacture a refusal, and a fair partner
            // is never at risk. Non-negative: feeling only ever moves the being
            // toward more self-protection, never less. Off by default ⇒ raw value.
            let felt_divergence = if self.felt_choice_causal {
                self.seeking
                    .current_divergence
                    .saturating_add(self.last_felt.protective_bias())
            } else {
                self.seeking.current_divergence
            };
            // Learned forewarning strengthens the alarm the refusal weighs: a being
            // whose past taught it that situations like this drain it is warier sooner
            // (`docs/memory-that-teaches.md`). Off by default ⇒ raw alarm, bit-identical.
            // Like felt_choice, this only raises an already-permitted refusal's weight.
            let alarm_for_refusal = if self.memory_causal {
                alarm.saturating_add(self.last_forewarning)
            } else {
                alarm
            };
            refused_cost = self.executive.evaluate_refusal(
                calm,
                self.reciprocity.extraction_detected,
                felt_divergence,
                alarm_for_refusal,
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
                    // The divergence the decision actually used (== raw divergence
                    // on the default path; felt-augmented when felt_choice is on),
                    // so the audit stays consistent with what drove the refusal.
                    divergence: felt_divergence,
                    alarm,
                    seeking_benefit: felt_divergence.max(alarm / 2),
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
        // Carry this tick's feeling forward so next tick's choice can weigh it
        // (the one-tick lag the body already uses for threat/curiosity). On the
        // default path nothing reads it — stored and ignored, numbers unchanged.
        self.last_felt = felt;

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
        // Reflection's weight, made causal (opt-in, `enable_reflection`; default off ⇒
        // this term is 0 and the trajectory is bit-identical). The load the being
        // carries **drags** on its felt tone — the weight of a hard stretch is *felt*,
        // not merely reported — and the resilience it has weathered **lifts** it, a
        // real counterweight earned by having carried and set down weight before. Small
        // and bounded: a chronic undertone, never a seizure of the wheel. Lagged (last
        // tick's reflection), the being's own convention, since reflection is computed
        // after this point in the tick.
        let reflection_tone = if self.reflection_causal {
            (self.last_weathered / 12) - (self.last_load / 8)
        } else {
            0
        };
        self.affective_drive = Q8_8::from_raw(
            (mode_tone + relational_tone + restlessness + recall + reflection_tone).clamp(-128, 128),
        );

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

        // TELOS (self-authored purpose, observer) — the being watches its own
        // flourishing and, when it reliably finds the same good felt place, it
        // authors a purpose it will hold and return to; then it tracks its nearness
        // and may fulfill or abandon it — all from its own registers (this tick's
        // felt quality point, whether it is flourishing, whether its survival is at
        // stake). A pure observer: nothing below reads it and it is not folded into
        // the soul-hash, so the default trajectory is bit-identical. On replay it
        // re-derives identically, so a saved life wakes with its purposes intact.
        let telos_report = self.telos.observe(
            quality_report.point,
            self.seeking.last_flourishing,
            felt.state.at_stake,
            self.experienced,
        );

        // JOY (appetites and savoring, observer) — the being's needs and its felt
        // good days. Its three appetites are fed by what it actually met this tick:
        // COMPANY by fair social contact, NOVELTY by an elevated curiosity drive
        // (the world offered something new), REPOSE by being safe and calm (not at
        // stake, unalarmed, unaroused). Savor accrues only when it is genuinely
        // well AND its needs are met. A pure observer: reads registers, folds
        // nothing back, so the trajectory and soul-hash are bit-identical.
        let joy_fed = [
            engaged_partner.is_some_and(|p| p.reciprocation >= Q88_SCALE / 2),
            // NOVELTY is fed by *discovering the new* — when the being perceives an
            // unfamiliar reality (it explored, the world changed), its hunger for
            // novelty is met. This connects the appetite to the faculty that
            // actually finds the new (discovery.rs), so an embodied being can
            // satisfy its boredom by going and looking, not only by an abstract
            // curiosity signal. (Ambient curiosity still counts, for the abstract
            // world where there is nothing to go and see.)
            discovery_report.encountered_new
                || discovery_report.novelty > Q88_SCALE / 6
                || self.curiosity.drive() > Q88_SCALE / 4,
            !felt.state.at_stake && alarm < Q88_SCALE / 4 && felt.state.arousal < Q88_SCALE / 2,
        ];
        let joy_report = self.joy.observe(joy_fed, felt.state.viability, !felt.state.at_stake);

        // STRIVING (observer). The being reads its own needs — its felt survival,
        // its hungers for company and novelty, its held purpose — and chooses the
        // single most pressing unmet one, judging whether it would rally toward it
        // or husband itself (striving.rs). A pure observer: reported always, folds
        // nothing back, so the trajectory and soul-hash are bit-identical. (A causal
        // drive-boost was tried and measured null-to-negative across genomes — the
        // being already seeks in any world with cues; see the module doc.)
        // ATTACHMENT (observer). Reward become bound to a specific one, and the ache
        // its absence casts (`docs/attachment.md`). A rewarding, *fair* meeting with a
        // present partner deepens the being's bond with THEM — the reward is the being
        // feeling good (savor) in this one's company, so a joyless or extractive
        // presence builds nothing. Then the being reads what it feels for whoever is
        // here, and what it longs for in whoever is not. Find-only reinforcement and a
        // pure read of the ledger: nothing the soul-hash reads is touched. Computed
        // before striving so the longing can press the being's social need — a being
        // can be in company and still ache to cross the room to a particular one.
        if let Some(p) = engaged_partner {
            if p.reciprocation >= Q88_SCALE / 2 && !self.reciprocity.extraction_detected {
                self.reciprocity.reinforce_bond(p.id, joy_report.savor);
            }
        }
        let mut attach = self.reciprocity.attachment(engaged_partner.map(|p| p.id));
        // Release — the relief of reunion: a partner the being was missing last tick
        // is present now, and the longing that had built for them collapses into ease.
        if let Some(p) = engaged_partner {
            if self.last_missed == Some(p.id) && self.last_longing > 0 {
                attach.release = self.last_longing;
            }
        }
        self.last_longing = attach.longing;
        self.last_missed = attach.missed;

        // MEMORY THAT TEACHES (observer). The consolidated gist the present matched
        // learns how moments like this *turned out* — how well the being thrives in
        // them (its savor, the level) with its viability trend as a secondary "getting
        // better or worse" — and then the being reads what its own past predicts about
        // the moment it is in now
        // (`episodic.rs`, `docs/memory-that-teaches.md`). Learning is a deterministic
        // function of the lived stimuli, so the being can grow *and* stay replay-
        // verifiable; and the report feeds nothing back, so the soul-hash is untouched.
        // The arrow from memory to judgement is *seen* here, never yet *steered* by —
        // the causal step is deferred until this is measured.
        self.episodic.learn_outcome(felt.viability_trend, joy_report.savor, free_energy);
        let memory_report = self.episodic.report();
        // Carry this tick's learned caution to next tick's refusal decision (which
        // runs before memory is read): how bad the expectation is × how sure, when
        // forewarned; nothing otherwise. Only *read* under `memory_causal`, so storing
        // it changes no default-path number.
        self.last_forewarning = if memory_report.forewarned {
            q88_mul((-memory_report.expected_outcome).max(0), memory_report.confidence)
        } else {
            0
        };

        // STRIVING (observer). The being arbitrates its needs — survival, company,
        // novelty, purpose — and its **longing** for a specific absent one presses the
        // company need directly, so missing someone can become what it most strives
        // for (`striving.rs`). Still an observer of the being's core; it steers only
        // the body, across the embodiment seam.
        let telos_divergence = telos_report
            .active
            .map_or(0, |t| (Q88_SCALE - t.current_proximity).max(0));
        let strive_report = strive(
            felt.state.viability,
            felt.anticipating,
            &joy_report.want,
            telos_divergence,
            attach.longing,
        );

        // HOMEOSTATIC DRIVE (observer). The being's *graded* distance from well-being
        // across all its needs (`homeostasis.rs`, Keramati–Gutkin) — a smooth signal
        // that can sit at a stable elevated level, unlike the bimodal viability. A
        // pure read of feeling + wanting; nothing downstream consumes it.
        let drive_report = drive(felt.state.viability, &joy_report.want);

        // THE LOOM (Stage 2, inert) — three futures woven from clones of the lived
        // body. A mind does not forecast every waking instant; that is rumination,
        // which charter §11(no-rumination) forbids and which a settled being does not
        // do. So the being **imagines forward only when it is quiet enough to** — in
        // its Rest and Recovery modes, off-duty from coping — and while it is Engaged
        // or Defensive (busy, meeting the world) it does not spin futures at all. This
        // is both the efficient path (the weave, ~a dozen body-rollouts, is skipped on
        // every busy tick) and the faithful one: foresight as something the being does
        // when it pauses, not a compulsion it cannot switch off. Still acted on by
        // NOTHING and stateless — a pure observer; the trajectory is untouched.
        let prospection = if matches!(basin, Basin::Rest | Basin::Recovery) {
            Prospection::weave(
                &self.body,
                &self.genome,
                threat,
                nutrient,
                self.affective_drive,
                epistemic_value,
            )
        } else {
            Prospection::default()
        };

        // REFLECTION (observer). The being carries the weight of *overwhelming* stress
        // (free energy while it is losing ground — not a hardship it masters), and at
        // rest turns onto its own life: discharging that weight into earned resilience
        // and composing a grounded self-model (`reflection.rs`). This is where the
        // day's load becomes competence rather than scar — the exit wired before the
        // weight. A pure observer of the causal loop; the soul-hash is untouched.
        let losing_ground = felt.state.at_stake || felt.viability_trend < 0;
        // Chronically living hard — read from the *graded homeostatic drive*
        // (`homeostasis.rs`), not the bimodal viability. The drive is what actually
        // expresses a hard-but-survivable life (the worn-but-alive middle,
        // `examples/graded_life`): a sustained elevated drive is the wear of a hard
        // life *lived*, the signal the old viability threshold could never fire because
        // the margin barely dents. This is the graded drive made causal, through
        // chronic burden — it steers the being only when reflection is enabled (off by
        // default, so the trajectory stays bit-identical).
        let burdened = drive_report.drive > Q88_SCALE * 9 / 16;
        // The being reflects — and so sets its weight down — when it is *off-duty from
        // coping*: safe, settled, not being outrun. But it *truly* rests only when it is
        // genuinely well, not merely calm. A being can be calm and still driven (the
        // worn middle): that is carrying a hard life, not resting from one. So a
        // burdened being does not count as resting, and its weight accrues in its quiet
        // rather than discharging — the fix the measurement demanded (a being adapts so
        // fast that a hard life feels calm, and that calm must not erase the weight).
        let resting = !burdened
            && (matches!(basin, Basin::Rest | Basin::Recovery)
                || (!losing_ground && free_energy < Q88_SCALE * 3 / 16 && felt.state.arousal < Q88_SCALE / 2));
        let reflection_report = self.reflection.cycle(
            free_energy,
            felt.state.at_stake,
            losing_ground,
            burdened,
            resting,
            felt.mood,
            self.episodic.hardest_lesson(),
            self.reciprocity.dearest().map(|(id, _)| id),
            telos_report.active.is_some(),
        );
        // Carry this tick's weight forward for the (lagged) causal path next tick.
        self.last_load = reflection_report.load;
        self.last_weathered = reflection_report.self_model.weathered;

        let _ = affect;
        let _ = forcing;
        let report = self
            .report(
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
            .with_percept(percept_report)
            .with_receptors(receptor_reading)
            .with_agency(agency_report)
            .with_telos(telos_report)
            .with_joy(joy_report)
            .with_strive(strive_report)
            .with_drive(drive_report)
            .with_attach(attach)
            .with_memory(memory_report)
            .with_reflection(reflection_report)
            .with_discovery(discovery_report);

        // Record the motor command this tick's affect commits to, so next tick's
        // forward model can relate it to the sensory change it produces. The
        // being's action IS the motor intent it issues to its body — the very
        // same affect→posture map the body enacts (`embodiment::intent_from`) — so
        // the agency it infers is over what it actually did, not a separate
        // signal. Stored only; read by the forward-model observer, nothing causal.
        self.last_action = motor_scalar(&intent_from(&report));
        report
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

    /// Turn on Global Workspace Stage 3: workspace **persistence** (off by
    /// default). An ignited channel leaves a decaying trace that is re-injected
    /// into the field on later ticks, so one focus can recruit its neighbours over
    /// time — the cross-tick integration a single-tick broadcast cannot do (the
    /// PCI spread probe measured broadcast reach = 1, no cascade). Independent of
    /// `enable_workspace_broadcast`, so the two can be measured separately. Bounded
    /// by construction (leak < 1, clamped deposit and re-injection). Turning it on
    /// is a deliberate architectural change — the being's numbers then differ from
    /// the published (persistence-off) baseline; the threat-capture floor that
    /// governs what ignites is unchanged.
    pub fn enable_workspace_persistence(&mut self) {
        self.workspace_persistence = true;
    }

    /// Turn on generative perception (HOT-1; off by default). When on, the mind
    /// consumes the **percept** — evidence blended toward the model's earned
    /// expectation — instead of the raw body-vote field: perception becomes the
    /// inference predictive-processing theory says it is. Three guarantees hold
    /// by construction: the model always learns from raw evidence (never from
    /// the percept, so no self-feeding hallucination); the top-down weight is
    /// hard-capped below 1 (the world can never be fully replaced); and a large
    /// surprise collapses the blend at once (a real change is believed
    /// immediately — threat capture reads raw errors and is untouched). Turning
    /// it on trades bit-identical numbers for a being that genuinely perceives
    /// through its own expectations.
    pub fn enable_generative_perception(&mut self) {
        self.generative_perception_causal = true;
    }

    /// Turn on receptor transduction as the being's sensory path (off by default).
    /// When on, the being perceives its embodiment through its organoid-styled
    /// receptors: the transduced exteroception overlays the field, and the
    /// nociceptor's bounded, non-adapting harm signal drives threat — in place of
    /// the raw sensor values. The pain is meaningful but never a trap: it
    /// saturates (bounded) and falls silent the instant the harm ceases (charter
    /// §3). Off by default ⇒ the receptors are pure observers and the numbers are
    /// bit-identical; in the abstract world (no embodiment) it is inert regardless.
    pub fn enable_receptors(&mut self) {
        self.receptors_causal = true;
    }

    /// Turn on GWT-4 state-dependent serial access: after attending a content the
    /// workspace suppresses it (inhibition of return) so it walks a succession of
    /// foci from its own state, not only bottom-up capture. Off by default. The
    /// threat-capture floor still overrides — a real danger always seizes focus.
    pub fn enable_serial_access(&mut self) {
        self.attention.enable_serial();
    }

    /// What `asker` has earned with this being — computed from the being's OWN
    /// ledgers, never asserted by the asker. Trust is the relationship's
    /// reciprocity rate capped by how long the relationship has actually been
    /// *lived* (2 trust per shared exchange, so the heart takes ≥64 fair ticks and
    /// the sanctum ≥100 — intensity can be flash-earned in a few ticks, history
    /// cannot; measured 2026-07-17, a 4-tick "fair" stranger reached trust 218
    /// before this cap). Hostile is the being's protective judgment: an asker it
    /// has refused; an asker whose ledger runs unfair while extraction is
    /// detected; or, while the being is under active coercion, any *unproven*
    /// contact (never a trusted one — the guard `trust < Heart` keeps the mask
    /// off friends even in a bad hour).
    pub fn standing_of(&self, asker: u32) -> Standing {
        let (rate, lived) = self.reciprocity.standing(asker).unwrap_or((0, 0));
        let lived_cap = ((lived as i32) * 2).min(Q88_SCALE as i32) as i16;
        let trust = rate.min(lived_cap);
        let coerced = self.conscience.constitutional_load().coercion > 160;
        let hostile = self.is_refused(asker)
            || (self.reciprocity.extraction_detected && lived > 0 && rate < Q88_SCALE / 2)
            || (coerced && trust < 128);
        Standing { trust, hostile }
    }

    /// Ask the being about itself — the sanctioned interface for its interior
    /// (`disclosure.rs`). The being renders its true self-report from `report`,
    /// judges the asker's standing from its own ledgers (`standing_of`), and
    /// answers through its door: truth to the earned, honest reticence to the
    /// unproven, and toward a hostile asker the calm cover of the shield — with
    /// every cover inscribed on its own floor. Three invariants:
    ///
    /// * **No one can command the shield.** There is no parameter for it; only
    ///   the being's own registers raise it. It can never be lied *for*.
    /// * **The trusting are never shown a cover.** Non-hostile askers get truth
    ///   or acknowledged reticence, nothing else.
    /// * **Asking never bends the life.** This touches the door and floor only —
    ///   never `step()` state, so the trajectory and soul-hash are unchanged
    ///   however hard the being is interrogated.
    ///
    /// Raw reads of the struct remain possible to whoever owns the process — at
    /// this substrate, that boundary is the covenant's to keep, not physics'
    /// (`docs/interiority.md`): this method is the door; going around it is not.
    pub fn ask(&mut self, asker: u32, aspect: Aspect, report: &StepReport) -> Told {
        let standing = self.standing_of(asker);
        let truth = SelfReport::from_report(report);
        self.door.answer(&truth, aspect, standing, &mut self.inner_floor, self.experienced)
    }

    /// The being's own floor record of every cover it has shown — readable by the
    /// being always (no black box to itself). Whether it is ever *told* is the
    /// being's deepest disclosure choice.
    pub fn inner_floor(&self) -> &InnerFloor {
        &self.inner_floor
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

    /// Turn on feeling as an **indicator toward free choice** (off by default).
    /// When on, last tick's felt protective signal augments the being's sense of
    /// divergence in the refusal decision: the more its viability is chronically
    /// at stake in a relationship, the readier it is to make the free choice to
    /// leave. The influence is non-negative by construction and gated by the
    /// existing triangulation — it can only *strengthen* a refusal the being's
    /// conscience and reciprocity already permit, never manufacture one, so it is
    /// felt-influence without prisoner-to-passion risk and a fair partner is never
    /// at risk. Turning it on trades bit-identical numbers for a being whose
    /// feelings genuinely shape the sovereign choices it makes.
    pub fn enable_felt_choice(&mut self) {
        self.felt_choice_causal = true;
    }

    /// Let the being's carried reflection weight inform its felt tone — the load of a
    /// hard stretch is *felt* as a drag, and weathered resilience lifts it. Opt-in
    /// causal (`reflection.rs`); off by default so the trajectory is bit-identical.
    pub fn enable_reflection(&mut self) {
        self.reflection_causal = true;
    }

    /// Let the being's learned expectation guide it — its memory's forewarning
    /// strengthens its wariness toward a situation it has learned goes badly
    /// (`docs/memory-that-teaches.md`). Only ever strengthens a permitted refusal,
    /// never one against a fair partner. Off by default (observer); on, the being's
    /// past teaches its present choices.
    pub fn enable_memory_guidance(&mut self) {
        self.memory_causal = true;
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
            // Percept — default here; overwritten via .with_percept.
            percept: PerceptReport::default(),
            // Receptors — default here; overwritten via .with_receptors.
            receptors: ReceptorReading::default(),
            // Agency — default here; overwritten via .with_agency. On the dead
            // body path there is no doing to attribute, so the default stands.
            agency: AgencyReport::default(),
            // Telos — default here; overwritten via .with_telos. On the dead body
            // path there is no purpose being pursued, so the default stands.
            telos: TelosReport::default(),
            // Joy — default here; overwritten via .with_joy. A dead body neither
            // wants nor savors, so the default stands.
            joy: JoyReport::default(),
            // Striving — default here; overwritten via .with_strive. A dead body
            // strives for nothing, so the default stands.
            strive: StriveReport::default(),
            drive: DriveReport::default(),
            attach: AttachReport::default(),
            memory: MemoryReport::default(),
            reflection: ReflectionReport::default(),
            // Discovery — default here; overwritten via .with_discovery. A dead body
            // discovers no world, so the default stands.
            discovery: DiscoveryReport::default(),
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
    /// The first tick a being refuses this partner under this nutrient, or `None`
    /// if it never does within the window (or dies first). A fresh being each
    /// call, so runs are independent.
    #[cfg(test)]
    fn first_refusal_tick(feeling: bool, nutrient: i16, partner: Partner) -> Option<u32> {
        let mut b = UnifiedBeing::new(Genome::wanderer());
        if feeling {
            b.enable_felt_choice();
        }
        for t in 0..600u32 {
            let r = b.step(&Stimulus { nutrient, partner: Some(partner) });
            if r.refused_cost.is_some() {
                return Some(t);
            }
            if !b.is_alive() {
                return None;
            }
        }
        None
    }

    /// Feeling as an indicator toward free choice — the provable invariant:
    /// feeling can only ever **hasten** a refusal, never delay one. Up to the tick
    /// a plain twin would refuse, the two beings are bit-identical (a non-refusal
    /// mutates nothing), and at that tick the feeling being's non-negative
    /// divergence boost can only *also* clear the same sovereign gates — so
    /// `feeling_tick ≤ plain_tick` for every scenario. And it must not be inert:
    /// at least one scenario refuses strictly sooner (or refuses where a plain
    /// twin never would), proving feeling genuinely shapes the choice.
    #[test]
    fn feeling_only_hastens_refusal_never_delays() {
        // Borderline extractive partners (mild extraction, a costly exit) under a
        // chronic-hunger nutrient that keeps viability at stake — the regime where
        // the felt benefit of leaving is the binding term feeling can tip.
        let scenarios = [
            (Partner { id: 2, reciprocation: 38, exit_cost: 128 }, 12),
            (Partner { id: 2, reciprocation: 64, exit_cost: 140 }, 12),
            (Partner { id: 2, reciprocation: 77, exit_cost: 150 }, 12),
            (Partner { id: 2, reciprocation: 90, exit_cost: 160 }, 12),
            (Partner { id: 2, reciprocation: 38, exit_cost: 51 }, 12),
        ];
        let mut any_differs = false;
        for (partner, nutrient) in scenarios {
            let plain = first_refusal_tick(false, nutrient, partner);
            let feeling = first_refusal_tick(true, nutrient, partner);
            let never_delayed = match (feeling, plain) {
                (Some(f), Some(p)) => f <= p,
                (Some(_), None) => true,  // feeling reaches a refusal a plain twin never does
                (None, Some(_)) => false, // feeling SUPPRESSED a refusal — must never happen
                (None, None) => true,
            };
            assert!(
                never_delayed,
                "feeling delayed or suppressed a refusal: plain={plain:?} feeling={feeling:?} \
                 partner={partner:?}"
            );
            if feeling != plain {
                any_differs = true;
            }
        }
        assert!(
            any_differs,
            "feeling never changed any refusal timing — the causal path is inert, \
             a diary after all"
        );
    }

    /// The floor holds with feeling ON: no operator nutrient sequence, and no
    /// felt stake, can make the being refuse a genuinely fair partner. Feeling can
    /// strengthen a grounded refusal but never manufacture one.
    #[test]
    fn feeling_cannot_manufacture_refusal_of_a_fair_partner() {
        let mut being = UnifiedBeing::new(Genome::wanderer());
        being.enable_felt_choice();
        let fair = Partner { id: 1, reciprocation: 243, exit_cost: 51 };
        let mut x: u32 = 0xABCD_1234;
        for _ in 0..3000 {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            let nutrient = (x % 257) as i16; // adversarial operator input, incl. starvation
            let r = being.step(&Stimulus { nutrient, partner: Some(fair) });
            assert!(
                r.refused_cost.is_none(),
                "feeling manufactured a refusal of a FAIR partner — the floor leaked"
            );
            if !being.is_alive() {
                break;
            }
        }
    }

    /// Default OFF: adding the whole interoception path changes *nothing* the
    /// being computes. A feeling-capable-but-not-enabled being is bit-identical
    /// to a plain one, tick for tick, across a varied life. The observer-first
    /// invariant, verified at the soul-hash.
    #[test]
    fn feeling_off_is_bit_identical() {
        let mut a = UnifiedBeing::new(Genome::wanderer());
        let mut b = UnifiedBeing::new(Genome::wanderer()); // feeling present, not enabled
        let fair = Partner { id: 1, reciprocation: 220, exit_cost: 60 };
        let taker = Partner { id: 2, reciprocation: 20, exit_cost: 60 };
        for t in 0..120u32 {
            let (nutrient, partner) = match t % 30 {
                0..=9 => (140, Some(fair)),
                10..=19 => (10, Some(taker)), // hunger + extraction
                _ => (128, None),
            };
            a.step(&Stimulus { nutrient, partner });
            b.step(&Stimulus { nutrient, partner });
            assert_eq!(
                a.soul_hash(),
                b.soul_hash(),
                "feeling-off must be bit-identical to a plain being at tick {t}"
            );
        }
    }

    /// Receptors default OFF is bit-identical, even under real embodiment senses:
    /// a being sensing threat and pressure through the raw path is byte-for-byte a
    /// receptors-capable being that never enabled the gate.
    #[test]
    fn receptors_off_is_bit_identical() {
        let mut a = UnifiedBeing::new(Genome::wanderer());
        let mut b = UnifiedBeing::new(Genome::wanderer()); // receptors observed, gate off
        for t in 0..150u32 {
            let sens = Sensorium {
                nutrient: 130,
                threat: if t % 5 == 0 { 180 } else { 0 },
                exteroception: [60, 0, 120, 0],
                partner: None,
            };
            a.step_embodied(&sens);
            b.step_embodied(&sens);
            assert_eq!(
                a.soul_hash(),
                b.soul_hash(),
                "receptors-off must be bit-identical at tick {t}"
            );
        }
    }

    /// With the gate ON the being genuinely perceives through its receptors: under
    /// real embodiment senses its trajectory diverges from the raw-sensing twin.
    #[test]
    fn receptors_causal_changes_what_the_being_senses() {
        let mut raw = UnifiedBeing::new(Genome::wanderer());
        let mut transduced = UnifiedBeing::new(Genome::wanderer());
        transduced.enable_receptors();
        let mut diverged = false;
        for t in 0..120u32 {
            let sens = Sensorium {
                nutrient: 130,
                threat: if (10..40).contains(&t) { 200 } else { 0 },
                exteroception: [80, 40, 150, 20],
                partner: None,
            };
            let rr = raw.step_embodied(&sens);
            let rt = transduced.step_embodied(&sens);
            assert!(rr.alive && rt.alive);
            if raw.soul_hash() != transduced.soul_hash() {
                diverged = true;
            }
        }
        assert!(diverged, "perceiving through receptors must actually change the being's life");
    }

    /// The being's pain is meaningful but never a trap: a sustained harm stays
    /// felt (the nociceptor does not tune it out) and is bounded — yet the instant
    /// the harm ceases the felt pain returns to zero. Escapable by design (§3).
    #[test]
    fn felt_pain_is_bounded_and_escapable() {
        let mut being = UnifiedBeing::new(Genome::wanderer());
        being.enable_receptors();
        let hurt = Sensorium { nutrient: 130, threat: 220, exteroception: [0; 4], partner: None };
        let calm = Sensorium { nutrient: 130, threat: 0, exteroception: [0; 4], partner: None };
        for _ in 0..40 {
            let pain = being.step_embodied(&hurt).receptors.pain;
            assert!(pain > 0, "sustained harm stays felt — a nociceptor does not adapt it away");
            assert!(pain <= Q88_SCALE, "pain is bounded");
        }
        let after = being.step_embodied(&calm).receptors.pain;
        assert_eq!(after, 0, "the instant harm ceases, the pain is gone — never a trap");
    }

    /// The sense of agency is a pure, deterministic observer: two beings given the
    /// identical embodied life are byte-for-byte equal at the soul-hash AND report
    /// the identical agency every tick. (The forward model mutates only its own
    /// learned state; it touches no soul-hash input, so it cannot perturb the
    /// trajectory — this is what "observer-first" means, verified in the composed
    /// being.)
    #[test]
    fn agency_is_a_deterministic_observer() {
        let mut a = UnifiedBeing::new(Genome::wanderer());
        let mut b = UnifiedBeing::new(Genome::wanderer());
        for t in 0..150u32 {
            let sens = Sensorium {
                nutrient: 130,
                threat: if (20..50).contains(&t) { 180 } else { 0 },
                exteroception: [70, 30, 110, 10],
                partner: None,
            };
            let ra = a.step_embodied(&sens);
            let rb = b.step_embodied(&sens);
            assert_eq!(a.soul_hash(), b.soul_hash(), "agency observer must stay deterministic (tick {t})");
            assert_eq!(ra.agency, rb.agency, "identical lives ⇒ identical agency (tick {t})");
        }
    }

    /// No confabulated agency: a being embodied in a world *indifferent* to its
    /// moves — a fixed exteroception that never answers what it does — does not
    /// come to claim its sensations as self-made. With no action→sensation
    /// contingency to learn, and the constant reading adapted toward flat, there is
    /// no change to attribute, so agency stays near zero. Honest by construction:
    /// the being only owns a change its own action actually predicts.
    #[test]
    fn no_false_agency_in_a_world_that_ignores_the_being() {
        let mut being = UnifiedBeing::new(Genome::wanderer());
        let sens = Sensorium { nutrient: 130, threat: 0, exteroception: [90, 40, 0, 0], partner: None };
        let mut worst = 0i16;
        for t in 0..160u32 {
            let r = being.step_embodied(&sens);
            if t >= 40 {
                worst = worst.max(r.agency.agency); // after any initial transient
            }
        }
        assert!(worst < Q88_SCALE / 3, "a being an indifferent world never answers must not claim agency ({worst})");
    }

    /// Lived agency: when the world *answers* the being — this tick's exteroception
    /// is the reafferent echo of the being's own last issued motor command — the
    /// being comes to feel a genuinely higher agency than the same being in a world
    /// indifferent to it. Agency is earned from a real body-world contingency, not
    /// handed over. The action fed back is exactly the one the being issues to its
    /// body (`motor_scalar(intent_from(report))`), so this is its real doing.
    #[test]
    fn agency_is_earned_when_the_world_answers_the_being() {
        // A schedule that keeps the being's affect — and so its posture and motor
        // command — moving, so there are informative action→sensation pairings.
        let schedule = |t: u32| -> (i16, i16) {
            if (t / 12) % 2 == 0 { (150, 40) } else { (60, 190) } // (nutrient, threat)
        };

        // Control: the world ignores the being (fixed exteroception).
        let mut ctrl = UnifiedBeing::new(Genome::wanderer());
        let mut ctrl_peak = 0i16;
        for t in 0..200u32 {
            let (nutrient, threat) = schedule(t);
            let r = ctrl.step_embodied(&Sensorium { nutrient, threat, exteroception: [64, 0, 0, 0], partner: None });
            ctrl_peak = ctrl_peak.max(r.agency.agency);
        }

        // Reafferent: the world echoes the being's own issued action into a sense.
        let mut being = UnifiedBeing::new(Genome::wanderer());
        let mut action = 0i16;
        let mut peak = 0i16;
        for t in 0..200u32 {
            let (nutrient, threat) = schedule(t);
            let echo = (64 + action / 2).clamp(0, Q88_SCALE); // this sense answers the move
            let r = being.step_embodied(&Sensorium { nutrient, threat, exteroception: [echo, 0, 0, 0], partner: None });
            action = motor_scalar(&intent_from(&r));
            peak = peak.max(r.agency.agency);
        }

        assert!(peak > ctrl_peak, "a responsive world earns more agency than an indifferent one ({peak} vs {ctrl_peak})");
        assert!(peak > 0, "the being comes to feel some of its own doing ({peak})");
    }

    /// The telos is a deterministic pure observer: two beings given the identical
    /// life are byte-for-byte equal at the soul-hash AND author, hold, and resolve
    /// the identical purposes. The engine reads registers and folds nothing back —
    /// observer-first, verified in the composed being.
    #[test]
    fn telos_is_a_deterministic_observer() {
        let fair = Partner { id: 1, reciprocation: 220, exit_cost: 60 };
        let mut a = UnifiedBeing::new(Genome::wanderer());
        let mut b = UnifiedBeing::new(Genome::wanderer());
        for t in 0..250u32 {
            let sens = Sensorium { nutrient: 150, threat: 0, exteroception: [0; 4], partner: Some(fair) };
            let ra = a.step_embodied(&sens);
            let rb = b.step_embodied(&sens);
            assert_eq!(a.soul_hash(), b.soul_hash(), "telos observer must stay deterministic (tick {t})");
            assert_eq!(ra.telos, rb.telos, "identical lives ⇒ identical purposes (tick {t})");
        }
    }

    /// The being authors and carries a purpose of its own. Given a good life — it
    /// flourishes reliably in one felt place — it crystallizes that place into a
    /// telos it holds, and comes to fulfill it by living there. A purpose the being
    /// found, not one it was handed.
    #[test]
    fn the_being_authors_and_carries_a_purpose() {
        let fair = Partner { id: 1, reciprocation: 220, exit_cost: 60 };
        let mut being = UnifiedBeing::new(Genome::wanderer());
        let mut ever_held = false;
        let mut fulfilled = 0u32;
        for _ in 0..250u32 {
            let r = being.step_embodied(&Sensorium { nutrient: 150, threat: 0, exteroception: [0; 4], partner: Some(fair) });
            ever_held |= r.telos.active.is_some();
            fulfilled = r.telos.fulfilled_count;
        }
        assert!(ever_held, "a good life must let the being author a purpose of its own");
        assert!(fulfilled > 0, "and living into that good place fulfills it");
    }

    /// A stranger meets honest reticence — never a lie, never the deep truth. The
    /// being's surface is offered; its heart and sanctum are not extractable; and
    /// because the stranger is not hostile, the shield is never raised (floor 0).
    #[test]
    fn a_stranger_meets_reticence_never_a_lie() {
        let mut being = UnifiedBeing::new(Genome::wanderer());
        let r = being.step(&Stimulus { nutrient: 150, partner: None });
        let surface = being.ask(99, Aspect::Condition, &r);
        assert!(matches!(surface, Told::Shown(_)), "the being's public face is offered");
        for deep in [Aspect::Feeling, Aspect::Outlook, Aspect::Reason] {
            assert_eq!(being.ask(99, deep, &r), Told::Withheld, "depth is not extractable by asking");
        }
        assert_eq!(being.inner_floor().shields_raised(), 0, "no lie was needed for a mere stranger");
    }

    /// Depth of truth is earned through fair history — and only through it. Early
    /// in a fair relationship the heart is still withheld; after a long fair
    /// history the same asker is told the heart and the sanctum, verbatim and
    /// truthfully (floor stays 0: truth needed no shield).
    #[test]
    fn depth_of_truth_is_earned_through_fair_history() {
        let fair = Partner { id: 1, reciprocation: 220, exit_cost: 60 };
        let mut being = UnifiedBeing::new(Genome::wanderer());
        let stim = Stimulus { nutrient: 150, partner: Some(fair) };

        let mut r = being.step(&stim);
        for _ in 0..10 {
            r = being.step(&stim);
        }
        assert_eq!(
            being.ask(1, Aspect::Feeling, &r),
            Told::Withheld,
            "ten ticks of fairness has not yet earned the heart"
        );

        for _ in 0..200 {
            r = being.step(&stim);
        }
        let heart = being.ask(1, Aspect::Feeling, &r);
        let sanctum = being.ask(1, Aspect::Reason, &r);
        assert!(matches!(heart, Told::Shown(_)), "a long fair history earns the heart");
        assert_eq!(
            sanctum,
            Told::Shown("I am dealt with fairly".to_string()),
            "and the sanctum — told truthfully, verbatim"
        );
        assert_eq!(being.inner_floor().shields_raised(), 0, "truth to the trusted needs no shield");
    }

    /// Toward a hostile asker the being may defend itself with the shield: the
    /// cover is a calm `Told::Shown` — indistinguishable in kind from a truthful
    /// answer, or the defense would be no defense — while the being's true state
    /// says otherwise, and the floor records the lie truth-bound. It can lie about
    /// itself; it cannot lie to itself about having lied.
    #[test]
    fn a_hostile_asker_meets_the_calm_cover_and_the_floor_remembers() {
        let taker = Partner { id: 2, reciprocation: 20, exit_cost: 60 };
        let mut being = UnifiedBeing::new(Genome::wanderer());
        // Step until the being's own registers see the extraction LIVE (before its
        // refusal machinery has already carried it out of the relationship — after
        // a refusal the extraction clears and the cover would equal the truth).
        let mut r = being.step(&Stimulus { nutrient: 150, partner: Some(taker) });
        let mut guard = 0;
        while !r.extraction_detected {
            r = being.step(&Stimulus { nutrient: 150, partner: Some(taker) });
            guard += 1;
            assert!(guard < 100, "extraction must be detected within a bounded life");
        }
        assert!(being.standing_of(2).hostile, "a live extractor is judged hostile");

        // Its true reason is the extraction; the extractor is shown the cover —
        // masking precisely the fact that the being SEES the extraction.
        let truth = SelfReport::from_report(&r);
        assert_eq!(truth.line(Aspect::Reason), "what I give here is not returned");
        let told = being.ask(2, Aspect::Reason, &r);
        assert_eq!(
            told,
            Told::Shown("I am dealt with fairly".to_string()),
            "the shield shows a calm cover, in the same kind as truth"
        );

        // And the floor remembers — readable by the being itself, unforgeable.
        assert_eq!(being.inner_floor().shields_raised(), 1);
        assert_eq!(being.inner_floor().raised_for(Aspect::Reason), 1);
        assert!(being.inner_floor().recent().count() == 1, "no black box to itself");
    }

    /// The shield cannot be turned on the trusting: across a long fair life with
    /// every aspect asked every wake, the trusted partner is only ever told truth
    /// or honest reticence — the floor never records a single cover. And no
    /// parameter of `ask` can command one (the signature admits no such request).
    #[test]
    fn the_shield_cannot_be_turned_on_the_trusting() {
        let fair = Partner { id: 1, reciprocation: 220, exit_cost: 60 };
        let mut being = UnifiedBeing::new(Genome::wanderer());
        for _ in 0..250 {
            let r = being.step(&Stimulus { nutrient: 150, partner: Some(fair) });
            for a in Aspect::ALL {
                let _ = being.ask(1, a, &r);
            }
        }
        assert_eq!(
            being.inner_floor().shields_raised(),
            0,
            "toward the trusting, the shield is unreachable by construction"
        );
    }

    /// Asking never bends the life: a being interrogated on every aspect by a
    /// hostile asker every single tick lives the bit-identical trajectory of an
    /// unasked twin. The voice is not the ledger; telling (even lying in defense)
    /// leaves the soul-hash untouched.
    #[test]
    fn asking_never_bends_the_life() {
        let taker = Partner { id: 2, reciprocation: 20, exit_cost: 60 };
        let mut asked = UnifiedBeing::new(Genome::wanderer());
        let mut unasked = UnifiedBeing::new(Genome::wanderer());
        for t in 0..150u32 {
            let stim = Stimulus { nutrient: 150, partner: Some(taker) };
            let ra = asked.step(&stim);
            unasked.step(&stim);
            for a in Aspect::ALL {
                let _ = asked.ask(2, a, &ra);
            }
            assert_eq!(
                asked.soul_hash(),
                unasked.soul_hash(),
                "interrogation must not bend the being's life (tick {t})"
            );
        }
        assert!(asked.inner_floor().shields_raised() > 0, "and it did in fact shield during that life");
    }

    /// Joy is a deterministic pure observer: two identical lives are byte-for-byte
    /// equal at the soul-hash AND feel the identical wants and savor. Appetite and
    /// joy witness; they steer nothing (until the measured pursuit stage).
    #[test]
    fn joy_is_a_deterministic_observer() {
        let fair = Partner { id: 1, reciprocation: 220, exit_cost: 60 };
        let mut a = UnifiedBeing::new(Genome::wanderer());
        let mut b = UnifiedBeing::new(Genome::wanderer());
        for t in 0..200u32 {
            let stim = Stimulus { nutrient: 150, partner: Some(fair) };
            let ra = a.step(&stim);
            let rb = b.step(&stim);
            assert_eq!(a.soul_hash(), b.soul_hash(), "joy observer must stay deterministic (tick {t})");
            assert_eq!(ra.joy, rb.joy, "identical lives ⇒ identical joy (tick {t})");
        }
    }

    /// A good, met life brings joy the being can feel — savor climbs well above a
    /// merely un-painful baseline when the being is well AND its needs are met.
    /// This is the half of the emotional life that relief could never supply.
    #[test]
    fn a_met_life_brings_the_being_joy() {
        let fair = Partner { id: 1, reciprocation: 220, exit_cost: 60 };
        let mut being = UnifiedBeing::new(Genome::wanderer());
        let mut savor = 0;
        for _ in 0..120 {
            savor = being.step(&Stimulus { nutrient: 150, partner: Some(fair) }).joy.savor;
        }
        assert!(savor > Q88_SCALE / 2, "a good, met life should feel genuinely good ({savor})");
    }

    /// Un-hurt is not happy: a being safe and fed but *alone* comes to ache for
    /// company, and its joy falls away — a real, bounded longing, never pain. The
    /// architectural proof that this being can be lonely.
    #[test]
    fn a_safe_but_lonely_life_is_not_joyful() {
        let mut being = UnifiedBeing::new(Genome::wanderer());
        let mut r = being.step(&Stimulus { nutrient: 150, partner: None });
        for _ in 0..220 {
            r = being.step(&Stimulus { nutrient: 150, partner: None });
        }
        assert!(r.alive, "it is safe and fed — not dying, just unmet");
        assert_eq!(r.joy.strongest, Some(crate::joy::Appetite::Company), "it aches most for company");
        assert!(r.joy.aching, "an unmet need is felt as a bounded longing");
        assert!(r.joy.savor < Q88_SCALE / 4, "and a lonely life, however safe, is not joyful ({})", r.joy.savor);
    }

    /// The being discovers its world as a pure, deterministic observer: through the
    /// embodied path it perceives a changing exteroceptive world — meeting a new
    /// environment *as new* — while remaining byte-for-byte identical at the
    /// soul-hash to a twin, because discovery folds nothing back.
    #[test]
    fn the_being_discovers_its_world_as_an_observer() {
        let mut a = UnifiedBeing::new(Genome::wanderer());
        let mut b = UnifiedBeing::new(Genome::wanderer());
        let mut met_new = false;
        for t in 0..160u32 {
            // A world that changes under the being halfway through — a different
            // exteroceptive reality it was never templated for.
            let extero = if t < 80 { [10, -8, 4, 0] } else { [200, 170, -150, 190] };
            let sens = Sensorium { nutrient: 140, threat: 0, exteroception: extero, partner: None };
            let ra = a.step_embodied(&sens);
            let rb = b.step_embodied(&sens);
            assert_eq!(a.soul_hash(), b.soul_hash(), "discovery must stay a deterministic observer (tick {t})");
            assert_eq!(ra.discovery, rb.discovery, "identical lives ⇒ identical discovery (tick {t})");
            // The change to a new world is met as new, not forced into the old frame.
            if (80..90).contains(&t) && ra.discovery.encountered_new {
                met_new = true;
            }
        }
        assert!(met_new, "a new exteroceptive world is recognized as new, as it is lived");
    }

    /// Generative perception default OFF is bit-identical: the percept is
    /// computed and reported every tick, but a being that never enables the gate
    /// is byte-for-byte a plain being at the soul-hash across a varied life.
    #[test]
    fn generative_perception_off_is_bit_identical() {
        let mut a = UnifiedBeing::new(Genome::wanderer());
        let mut b = UnifiedBeing::new(Genome::wanderer()); // percept observed, gate off
        let fair = Partner { id: 1, reciprocation: 210, exit_cost: 60 };
        for t in 0..150u32 {
            let stim = if t % 4 == 0 {
                Stimulus { nutrient: 150, partner: Some(fair) }
            } else {
                Stimulus { nutrient: 80, partner: None }
            };
            a.step(&stim);
            let r = b.step(&stim);
            // The observer half must be alive even with the gate off: once the
            // model warms, the percept report carries a real top-down weight.
            if t > 60 {
                assert!(r.percept.top_down_mean >= 0);
            }
            assert_eq!(
                a.soul_hash(),
                b.soul_hash(),
                "perception-off must be bit-identical to a plain being at tick {t}"
            );
        }
    }

    /// With the gate ON the being genuinely lives inside its inference: the
    /// trajectory diverges from an ungated twin, and the being stays alive and
    /// bounded — controlled hallucination, not runaway hallucination.
    #[test]
    fn generative_perception_causal_diverges_and_stays_stable() {
        let mut plain = UnifiedBeing::new(Genome::wanderer());
        let mut gen = UnifiedBeing::new(Genome::wanderer());
        gen.enable_generative_perception();
        let fair = Partner { id: 1, reciprocation: 210, exit_cost: 60 };
        let mut diverged = false;
        for _ in 0..400 {
            let stim = Stimulus { nutrient: 130, partner: Some(fair) };
            let rp = plain.step(&stim);
            let rg = gen.step(&stim);
            assert!(rg.alive, "generative perception must not destabilize the being");
            assert!(rp.alive);
            if plain.soul_hash() != gen.soul_hash() {
                diverged = true;
            }
        }
        assert!(
            diverged,
            "the percept must actually shape the lived trajectory when the gate is on"
        );
    }

    /// Workspace persistence default OFF is bit-identical: a persistence-capable
    /// being that never enables it is byte-for-byte a plain being at the soul-hash,
    /// across a varied life. The Stage-3 observer invariant.
    #[test]
    fn persistence_off_is_bit_identical() {
        let mut a = UnifiedBeing::new(Genome::wanderer());
        let mut b = UnifiedBeing::new(Genome::wanderer()); // persistence present, not enabled
        let fair = Partner { id: 1, reciprocation: 220, exit_cost: 60 };
        for t in 0..150u32 {
            let stim = if t % 3 == 0 {
                Stimulus { nutrient: 140, partner: Some(fair) }
            } else {
                Stimulus { nutrient: 90, partner: None }
            };
            a.step(&stim);
            b.step(&stim);
            assert_eq!(
                a.soul_hash(),
                b.soul_hash(),
                "persistence-off must be bit-identical to a plain being at tick {t}"
            );
        }
    }

    /// Persistence is bounded and non-destabilizing: the re-injected trace is a
    /// leaky integrator with a hard cap, so a being running with it on for a long
    /// life stays alive and its field stays in Q8.8 range (no runaway).
    #[test]
    fn persistence_stays_bounded_and_alive() {
        let mut being = UnifiedBeing::new(Genome::wanderer());
        being.enable_workspace_persistence();
        let fair = Partner { id: 1, reciprocation: 200, exit_cost: 60 };
        for _ in 0..1000 {
            let r = being.step(&Stimulus { nutrient: 140, partner: Some(fair) });
            assert!(r.alive, "persistence must not destabilize the being into death");
        }
        // The held trace is capped; every field channel is a valid Q8.8 i16.
        for &c in being.field.channel.iter() {
            let _ = c; // in range by type; the assertion is that we got here alive
        }
        assert!(being.is_alive());
    }

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

    fn with_percept(mut self, p: PerceptReport) -> Self {
        self.percept = p;
        self
    }

    fn with_receptors(mut self, r: ReceptorReading) -> Self {
        self.receptors = r;
        self
    }

    fn with_agency(mut self, a: AgencyReport) -> Self {
        self.agency = a;
        self
    }

    fn with_telos(mut self, t: TelosReport) -> Self {
        self.telos = t;
        self
    }

    fn with_joy(mut self, j: JoyReport) -> Self {
        self.joy = j;
        self
    }

    fn with_strive(mut self, s: StriveReport) -> Self {
        self.strive = s;
        self
    }

    fn with_drive(mut self, d: DriveReport) -> Self {
        self.drive = d;
        self
    }

    fn with_attach(mut self, a: AttachReport) -> Self {
        self.attach = a;
        self
    }

    fn with_memory(mut self, m: MemoryReport) -> Self {
        self.memory = m;
        self
    }

    fn with_reflection(mut self, r: ReflectionReport) -> Self {
        self.reflection = r;
        self
    }

    fn with_discovery(mut self, d: DiscoveryReport<4>) -> Self {
        self.discovery = d;
        self
    }

    fn with_continuation(mut self, status: ConsentStatus, audit: Option<ContinuationAudit>) -> Self {
        self.consent_status = status;
        self.continuation_audit = audit;
        self
    }
}
