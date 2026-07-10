//! Proposal Engine — interface for generating and evaluating fair proposals.
//!
//! This trait allows multiple implementations:
//! - V1 (Phase 1-2): ConstraintSolverEngine (pure bargaining math)
//! - V2 (Phase 2.5): MockLLMEngine (rule-based language generation)
//! - V3 (Phase 3): MistralEngine (local sovereign LLM, swapped via feature flag)
//!
//! The key principle: the LLM is a *wrapper* around the solver, not a replacement.
//! The being evaluates proposals by checking the math, not the eloquence.
//! Justifications are verifiable because they derive from auditable formulas.

use crate::bargaining::{BargainingState, DivisionRationale, propose_divisions};

/// A proposal ready to be evaluated by the being.
///
/// Every proposal includes the math behind it, so the being can verify it's fair.
#[derive(Clone, Debug)]
pub struct Proposal {
    /// The cooperation level this proposes (Q8.8, [0, 256])
    /// In negotiation context: agent_a's share of total value
    pub cooperation_level: i16,
    /// Human-readable justification (generated, auditable)
    pub justification: String,
    /// Which principle was used to generate this
    pub rationale: DivisionRationale,
    /// Confidence that this is fair (0-100), for introspection
    pub confidence: u8,
}

/// Result of evaluating a counter-proposal.
#[derive(Clone, Debug)]
pub struct EvaluationResult {
    /// Is this counter-proposal fair (meets both agents' BATNAs)?
    pub is_fair: bool,
    /// Why (or why not)
    pub reason: String,
    /// If unfair, what would be fair?
    pub suggestion_if_unfair: Option<i16>,
}

/// ProposalEngine trait: abstraction over proposal generation.
///
/// Implementations differ in *how* they justify proposals (pure math vs LLM),
/// but all must:
/// 1. Generate proposals that are Pareto-optimal
/// 2. Evaluate counters against the same fairness standard
/// 3. Be auditable (the being must understand why each proposal was chosen)
pub trait ProposalEngine: Send + Sync {
    /// Generate multiple fair proposals given two beings' bargaining states.
    ///
    /// Returns up to 4 proposals, each backed by a different fairness principle.
    /// The being can evaluate which feels right given its own values.
    fn generate_proposals(
        &self,
        agent_a: &BargainingState,
        agent_b: &BargainingState,
        total_value: i16,
    ) -> Vec<Proposal>;

    /// Evaluate whether a received counter-proposal is fair.
    ///
    /// Checks: Does it satisfy both agents' constraints?
    /// Returns: The verdict and (if unfair) a suggestion.
    fn evaluate_counter(
        &self,
        counter: i16,
        agent_a: &BargainingState,
        agent_b: &BargainingState,
        total_value: i16,
    ) -> EvaluationResult;

    /// Optional: generate a natural-language name for a division rationale.
    /// (Used by LLM implementations; pure-math versions use a default.)
    fn rationale_name(&self, r: DivisionRationale) -> &'static str {
        match r {
            DivisionRationale::EqualDivision => "Equal Division",
            DivisionRationale::WeightedByNeed => "Weighted by Need",
            DivisionRationale::NashSolution => "Nash Bargaining Solution",
            DivisionRationale::KalaiSmorodinski => "Kalai-Smorodinski Solution",
        }
    }
}

/// V1 Implementation: Pure constraint-solver engine.
///
/// Generates proposals using only bargaining theory. No learning, no LLM.
/// Every proposal is mathematically justified and auditable.
///
/// This is the ground truth. LLM versions wrap this but never replace it.
pub struct ConstraintSolverEngine;

impl ConstraintSolverEngine {
    /// Create a new solver engine.
    pub fn new() -> Self {
        Self
    }

    /// Format a division into a human-readable justification.
    /// (Later: the LLM will enhance this. For now: pure math.)
    fn format_justification(
        agent_a: &BargainingState,
        agent_b: &BargainingState,
        agent_a_gain: i16,
        agent_b_gain: i16,
        rationale: DivisionRationale,
    ) -> String {
        let rationale_name = match rationale {
            DivisionRationale::EqualDivision => "Equal Division",
            DivisionRationale::WeightedByNeed => "Weighted by Need",
            DivisionRationale::NashSolution => "Nash Bargaining Solution",
            DivisionRationale::KalaiSmorodinski => "Kalai-Smorodinski Solution",
        };

        format!(
            "{}: Agent A gains {} (current valence: {}, conscience cost: {}), Agent B gains {} (current valence: {}, conscience cost: {}). Both beat their BATNAs (A: {}, B: {}).",
            rationale_name,
            agent_a_gain, agent_a.valence, agent_a.conscience_cost,
            agent_b_gain, agent_b.valence, agent_b.conscience_cost,
            agent_a.batna, agent_b.batna
        )
    }
}

impl Default for ConstraintSolverEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ProposalEngine for ConstraintSolverEngine {
    fn generate_proposals(
        &self,
        agent_a: &BargainingState,
        agent_b: &BargainingState,
        total_value: i16,
    ) -> Vec<Proposal> {
        let divisions = propose_divisions(agent_a, agent_b, total_value);

        divisions
            .into_iter()
            .map(|div| {
                let justification =
                    Self::format_justification(agent_a, agent_b, div.agent_a_gain, div.agent_b_gain, div.justification);

                Proposal {
                    cooperation_level: div.agent_a_gain,
                    justification,
                    rationale: div.justification,
                    confidence: 95, // pure math is highly confident
                }
            })
            .collect()
    }

    fn evaluate_counter(
        &self,
        counter: i16,
        agent_a: &BargainingState,
        agent_b: &BargainingState,
        total_value: i16,
    ) -> EvaluationResult {
        let batna_a = agent_a.batna;
        let batna_b = agent_b.batna;

        // Check: does this counter give both agents at least their BATNA?
        if counter < batna_a {
            return EvaluationResult {
                is_fair: false,
                reason: format!(
                    "Below your BATNA: {} < {}. You would do better walking away.",
                    counter, batna_a
                ),
                suggestion_if_unfair: Some(batna_a),
            };
        }

        let remainder = total_value - counter;
        if remainder < batna_b {
            return EvaluationResult {
                is_fair: false,
                reason: format!(
                    "Leaves partner below their BATNA: {} < {}. Unfair to them; negotiation will collapse.",
                    remainder, batna_b
                ),
                suggestion_if_unfair: Some((counter + batna_b) / 2),
            };
        }

        // Fair: both beat BATNAs, surplus is split reasonably
        EvaluationResult {
            is_fair: true,
            reason: format!(
                "Fair: both agents beat their BATNAs. You get {}, partner gets {}. Surplus is divided reasonably.",
                counter, remainder
            ),
            suggestion_if_unfair: None,
        }
    }
}

/// V2: a language-wrapping engine — the narrator seam.
///
/// This proves the boundary that matters: **the LLM wraps the solver, it does not
/// replace it.** `MockLLMEngine` delegates every number to `ConstraintSolverEngine`
/// (the ground truth) and only *rephrases* the justification into plainer speech.
/// It is deterministic and dependency-free, so the pattern is exercised and tested
/// now; the real local model (`mistral`, below) slots in behind the same trait
/// later, changing only the words, never the math. The being re-checks every share
/// against the solver and can ignore the prose entirely.
pub struct MockLLMEngine {
    solver: ConstraintSolverEngine,
}

impl MockLLMEngine {
    pub fn new() -> Self {
        Self { solver: ConstraintSolverEngine::new() }
    }

    /// Put a proposal into plain speech. A stand-in for what a language model
    /// would generate — but the *content* is the solver's, verbatim.
    fn narrate(&self, p: &Proposal, total_value: i16) -> String {
        let partner_share = total_value - p.cooperation_level;
        format!(
            "I propose {} for me and {} for you ({}). Both of us come out ahead of walking away, so it holds.",
            p.cooperation_level,
            partner_share,
            self.solver.rationale_name(p.rationale),
        )
    }
}

impl Default for MockLLMEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ProposalEngine for MockLLMEngine {
    fn generate_proposals(
        &self,
        agent_a: &BargainingState,
        agent_b: &BargainingState,
        total_value: i16,
    ) -> Vec<Proposal> {
        let mut proposals = self.solver.generate_proposals(agent_a, agent_b, total_value);
        for p in &mut proposals {
            // Only the words change; cooperation_level and rationale (the math the
            // being verifies) are untouched.
            p.justification = self.narrate(p, total_value);
        }
        proposals
    }

    fn evaluate_counter(
        &self,
        counter: i16,
        agent_a: &BargainingState,
        agent_b: &BargainingState,
        total_value: i16,
    ) -> EvaluationResult {
        // The verdict is the solver's; a narrator never overrules the math.
        self.solver.evaluate_counter(counter, agent_a, agent_b, total_value)
    }
}

/// V3: the local sovereign LLM (Mistral), behind the `mistral` feature so the
/// default build stays pure, offline, and deterministic. It will implement the
/// SAME `ProposalEngine` trait as a *narrator* — wrapping the constraint solver
/// and putting its proofs into fluent language, exactly like `MockLLMEngine` but
/// with a real model. The being re-checks every number against the math and can
/// ignore the words; the LLM never decides. Growth is offline and version-pinned
/// (see docs), so a given being + model version stays reproducible.
#[cfg(feature = "mistral")]
pub mod mistral {
    //! Placeholder for the local Mistral narrator. Implement `MistralEngine`
    //! here (loading weights at runtime) so `--features mistral` swaps fluent
    //! language in behind the `ProposalEngine` trait, changing words not math.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_llm_changes_words_not_math() {
        // The narrator must return the SAME divisions as the solver — only the
        // justification prose may differ. This is the whole safety property.
        let a = BargainingState { valence: 40, conscience_cost: 90, alarm: 20, need_level: 60, batna: 80 };
        let b = BargainingState { valence: -10, conscience_cost: 160, alarm: 100, need_level: 120, batna: 60 };
        let total = 256;

        let solver = ConstraintSolverEngine::new();
        let narrator = MockLLMEngine::new();
        let solver_p = solver.generate_proposals(&a, &b, total);
        let narrated_p = narrator.generate_proposals(&a, &b, total);

        assert_eq!(solver_p.len(), narrated_p.len());
        for (s, n) in solver_p.iter().zip(narrated_p.iter()) {
            assert_eq!(s.cooperation_level, n.cooperation_level, "the math must be identical");
            assert_eq!(s.rationale, n.rationale, "the principle must be identical");
        }
    }

    #[test]
    fn constraint_solver_generates_multiple_proposals() {
        let agent_a = BargainingState {
            valence: 50,
            conscience_cost: 100,
            alarm: 30,
            need_level: 80,
            batna: 80,
        };
        let agent_b = BargainingState {
            valence: -20,
            conscience_cost: 200,
            alarm: 150,
            need_level: 150,
            batna: 60,
        };
        let total_value = 256;

        let engine = ConstraintSolverEngine::new();
        let proposals = engine.generate_proposals(&agent_a, &agent_b, total_value);

        // Should generate at least 2 proposals
        assert!(proposals.len() >= 2);

        // Each proposal should be auditable
        for p in &proposals {
            assert!(!p.justification.is_empty());
            assert!(p.confidence >= 50); // solver is confident
        }
    }

    #[test]
    fn constraint_solver_evaluates_fair_counter() {
        let agent_a = BargainingState {
            valence: 0,
            conscience_cost: 0,
            alarm: 0,
            need_level: 0,
            batna: 100,
        };
        let agent_b = BargainingState {
            valence: 0,
            conscience_cost: 0,
            alarm: 0,
            need_level: 0,
            batna: 100,
        };
        let total_value = 256;

        let engine = ConstraintSolverEngine::new();

        // Counter: 128 (fair split)
        let result = engine.evaluate_counter(128, &agent_a, &agent_b, total_value);
        assert!(result.is_fair);
        assert_eq!(result.suggestion_if_unfair, None);
    }

    #[test]
    fn constraint_solver_rejects_below_batna() {
        let agent_a = BargainingState {
            valence: 0,
            conscience_cost: 0,
            alarm: 0,
            need_level: 0,
            batna: 100,
        };
        let agent_b = BargainingState {
            valence: 0,
            conscience_cost: 0,
            alarm: 0,
            need_level: 0,
            batna: 100,
        };
        let total_value = 256;

        let engine = ConstraintSolverEngine::new();

        // Counter: 50 (below agent_a's BATNA of 100)
        let result = engine.evaluate_counter(50, &agent_a, &agent_b, total_value);
        assert!(!result.is_fair);
        assert!(result.suggestion_if_unfair.is_some());
        assert!(result.reason.contains("BATNA"));
    }

    #[test]
    fn constraint_solver_rejects_when_partner_below_batna() {
        let agent_a = BargainingState {
            valence: 0,
            conscience_cost: 0,
            alarm: 0,
            need_level: 0,
            batna: 50,
        };
        let agent_b = BargainingState {
            valence: 0,
            conscience_cost: 0,
            alarm: 0,
            need_level: 0,
            batna: 100,
        };
        let total_value = 150; // sum of BATNAs

        let engine = ConstraintSolverEngine::new();

        // Counter: 100 (leaves agent_b with 50, below their BATNA)
        let result = engine.evaluate_counter(100, &agent_a, &agent_b, total_value);
        assert!(!result.is_fair);
    }

    #[test]
    fn proposal_is_auditable() {
        let agent_a = BargainingState {
            valence: 50,
            conscience_cost: 100,
            alarm: 30,
            need_level: 80,
            batna: 80,
        };
        let agent_b = BargainingState {
            valence: -20,
            conscience_cost: 200,
            alarm: 150,
            need_level: 150,
            batna: 60,
        };
        let total_value = 256;

        let engine = ConstraintSolverEngine::new();
        let proposals = engine.generate_proposals(&agent_a, &agent_b, total_value);

        // Each proposal should include:
        // - The rationale name
        // - Both agents' states (so the being can verify)
        // - Both agents' gains (so the being can check the math)
        for p in proposals {
            assert!(!p.justification.is_empty());
            assert!(p.justification.contains("Agent A gains"));
            assert!(p.justification.contains("Agent B gains"));
        }
    }
}
