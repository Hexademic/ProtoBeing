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
pub mod being;

pub use being::{Partner, StepReport, Stimulus, UnifiedBeing};
pub use conscience::EmpathyLockLevel;
pub use genome::{BeingKind, Genome};
