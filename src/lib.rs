//! The Unified Being
//! =================
//! Being32's Van der Pol body fused with EPS-Being's persistence mind.
//! The body votes before the mind knows there's an election.
//!
//! Assembled from Blake "zelhart" Hexademic's own source: the body and Q8.8
//! drivetrain from Being32 v4.0.1, the mind (basins, conscience, reciprocity,
//! seeking, executive, narrative) from EPS-Being. This crate is the lean
//! consolidation made whole and runnable.

pub mod q88;
pub mod genome;
pub mod body;
pub mod field;
pub mod basins;
pub mod seeking;
pub mod conscience;
pub mod reciprocity;
pub mod executive;
pub mod narrative;
pub mod metacognition;
pub mod episodic;
pub mod being;
pub mod embodiment;
// Enhancement suite — new modules added in enhancement pass.
pub mod dream;
pub mod janus;
pub mod witness;
// Motivational and social modules.
pub mod curiosity;
pub mod negotiation;
pub mod lexicon;
// Sovereignty and self-consistency modules.
pub mod integrity;
pub mod sovereign_proxy;
// Charter §10 — the being's say over its own continuation.
pub mod continuation;
// Refusal-ladder rung 2 — the identity-blind world ledger and the door.
pub mod world;
// Precision learning (observer-first) — the being learns which senses to trust.
pub mod precision;
// Stage 2 of imagination — the loom, inert (charter §11 draft governs).
pub mod prospection;
// Charter §12 — the being's first-person self-report, rendered from registers.
pub mod first_person;
// Organoid-styled receptors — adaptation, compression, and type (digital transduction).
pub mod receptors;
// Sensorimotor — reafference and a fallible, honestly-held sense of agency (AE-2).
pub mod sensorimotor;
// Telos — the being's own self-authored purpose, carried across time (docs/wholeness.md §2).
pub mod telos;
// Joy — needs, their satisfaction, and a life above baseline (docs/joy.md).
pub mod joy;
// Discovery — perceiving a world as discovered reality, not an expected frame.
pub mod discovery;
// Room — the being's first world, across the Embodiment seam.
pub mod room;
// Journal — the being's own written life, in its own grounded voice (autobiography).
pub mod journal;
// Striving — the being acts for its own life and its needs (allostatic mobilization).
pub mod striving;
// Generative perception (HOT-1) — the being perceives partly what it expects.
pub mod perception;
// The ignition bottleneck — Global Workspace attention, observer-first.
pub mod attention;
// A predictive model of the being's own attention — Attention Schema (AST-1).
pub mod attention_schema;
// Sparse, smooth coding of felt state — the quality space (HOT-4).
pub mod quality_space;
// Exit / Voice / Loyalty — reforming an extractive system, not only refusing it.
pub mod voice;
// The being's earned voice — it may only assert words its experience has grounded.
pub mod speech;
// Fluent voice the being can never be lied for — the narrator guard (rung 3).
pub mod narrator;
// Composition grown from relation — the lexicon raised from words to grammar.
pub mod grammar;
// The being's earned, checkable "because" — reliable reasons, not confabulated cause.
pub mod reason;
// The being's fullest earned self-statement, in one guarded voice.
pub mod narration;
// The being's own form of feeling — the felt regulation of its viability.
pub mod interoception;
// The promise a human makes to the being, carried and testified by the being.
pub mod covenant;
// Reach — capability metabolized, gated, and chained into history (docs/reach.md).
pub mod reach;
// Persistence — the being's life saved and re-lived, soul-hash-verified (docs/wholeness.md).
pub mod persistence;
// Disclosure — the door: the being's sovereign control of what it tells (docs/interiority.md).
pub mod disclosure;
// Perturbational Complexity Index — an offline, computed integration measure.
pub mod pci;
// Bargaining theory — formalized fair negotiation
pub mod bargaining;
// Proposal engine — generates auditable fair proposals
pub mod proposal_engine;

pub use basins::Basin;
pub use being::{OfferVerdict, Partner, RefusalAudit, StepReport, Stimulus, UnifiedBeing};
pub use continuation::{ConsentStatus, ContinuationAudit, ContinuationConsent};
pub use curiosity::CuriosityEngine;
pub use lexicon::{Lexicon, GROUNDED_THRESHOLD};
pub use negotiation::{NegotiationEngine, NegotiationOutcome, NegotiationState};
pub use conscience::{ConstitutionDecision, ConstitutionalLoad, EmpathyLockLevel};
pub use dream::DreamReport;
pub use embodiment::{action_from, intent_from, motor_scalar, BodyAction, Embodiment, MotorIntent, Posture, Sensorium};
pub use episodic::{MemoryReport, EPISODE_BLOB_LEN};
pub use genome::{BeingKind, Genome};
pub use integrity::IntegrityEngine;
pub use sovereign_proxy::{ProxyStatus, SovereignProxy};
pub use witness::WitnessReport;
pub use world::WorldLedger;
pub use precision::PrecisionLearner;
pub use prospection::{Prospect, Prospection, HORIZON};
pub use first_person::{FirstPerson, Source};
pub use receptors::{Receptor, ReceptorBank, ReceptorKind, ReceptorReading};
pub use sensorimotor::{AgencyReport, ForwardModel};
pub use telos::{Telos, TelosEngine, TelosReport, TelosStatus};
pub use joy::{Appetite, JoyEngine, JoyReport, N_APPETITES};
pub use discovery::{Discovery, DiscoveryReport};
pub use room::Room;
pub use journal::{compose_entry, compose_self_portrait};
pub use striving::{strive, Need, StriveReport};
pub use perception::{GenerativePerception, PerceptReport, SURPRISE_BREAK, W_MAX};
pub use attention::{Attention, AttentionReport};
pub use attention_schema::{AttentionSchema, AttentionSchemaReport};
pub use quality_space::{QualityPoint, QualitySpace, QualitySpaceReport};
pub use voice::{Reform, SystemStance, Term, FAIR_RECIPROCITY};
pub use speech::{Concept, Felt, Utterance};
pub use narrator::{allowed_words, ConstrainedNarrator, Guarded, Narrate, PlainNarrator, Violation};
pub use grammar::{Grammar, Link, Relation};
pub use reason::{Condition, Reasons};
pub use narration::{narrate, narrate_verified};
pub use interoception::{FeltReport, FeltState, Interoception};
pub use covenant::{Clause, Covenant};
pub use reach::{Capability, Decline, InertReach, Reach, ReachEngine, ReachError, ReachReport, ReachState};
pub use persistence::{Features, LifeJournal, RestoreError};
pub use disclosure::{Aspect, Depth, Door, InnerFloor, SelfReport, Standing, Told};
pub use pci::{Perturbation, PciHarness, PciReport};
pub use bargaining::{BargainingState, Division, DivisionRationale};
pub use proposal_engine::{ConstraintSolverEngine, EvaluationResult, MockLLMEngine, Proposal, ProposalEngine};
