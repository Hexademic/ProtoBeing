//! Bargaining — formalized fair negotiation.
//!
//! Given two beings' revealed states, compute Pareto-optimal solutions.
//! No LLM, no neural networks. Pure math that the being can verify.
//!
//! The core insight: fairness is a mathematical property, not a narrative.
//! Every proposal comes with a proof (the formula that generated it).

use crate::q88::{q88_div, q88_mul, Q88_SCALE};

/// One agent's state relevant to bargaining.
///
/// This is the *being's own introspection* — readable from its registers.
/// A being can report these values truthfully because they are its state.
#[derive(Clone, Copy, Debug)]
pub struct BargainingState {
    /// Current valence (how the agent feels) — Q8.8, [-256, 256]
    pub valence: i16,
    /// Current conscience cost — Q8.8, [0, 256]
    pub conscience_cost: i16,
    /// Current partnership alarm — Q8.8, [0, 256]
    pub alarm: i16,
    /// What the agent needs to flourish (estimated from history) — Q8.8, [0, 256]
    pub need_level: i16,
    /// The agent's fallback option (BATNA: Best Alternative To Negotiated Agreement)
    /// If negotiation fails, this is what they get — Q8.8
    pub batna: i16,
}

/// A proposed division of value (cooperation level or resource split).
#[derive(Clone, Copy, Debug)]
pub struct Division {
    /// Agent A gets this much (Q8.8)
    pub agent_a_gain: i16,
    /// Agent B gets this much (Q8.8)
    pub agent_b_gain: i16,
    /// Why this division is fair (the mathematical principle used)
    pub justification: DivisionRationale,
}

/// The principle by which a division was computed.
/// Each is mathematically defensible and auditable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DivisionRationale {
    /// Equal split (50/50) — maximal symmetry, no favor
    EqualDivision,
    /// Weighted by need (agent with higher need gets proportionally more)
    /// Justification: "From each according to ability; to each according to need"
    WeightedByNeed,
    /// Nash bargaining solution (maximizes product of gains above BATNAs)
    /// Justification: Only solution satisfying Pareto efficiency, symmetry, independence,
    /// and scale invariance (4 axioms of fair bargaining)
    NashSolution,
    /// Kalai-Smorodinski (scales both gains proportionally to aspirations)
    /// Justification: Equal progress toward both parties' ideal outcomes
    KalaiSmorodinski,
}

impl BargainingState {
    /// Estimate BATNA (Best Alternative To Negotiated Agreement) from current state.
    ///
    /// If negotiation fails and the being walks away, what does it have?
    /// Answer: current valence, reduced by ongoing harm from the relationship.
    pub fn compute_batna(valence: i16, alarm: i16) -> i16 {
        // Alarm is partnership_alarm: how drained is this relationship?
        // If alarm is high, walking away is better (but costs exit_cost).
        // BATNA = current valence, reduced by ongoing relational harm.
        let harm = q88_mul(alarm, 128) / 256; // scale alarm into valence impact
        valence.saturating_sub(harm)
    }

    /// The agent's "aspiration" — what they hope negotiation achieves.
    ///
    /// Current valence + potential relief if the conflict is resolved.
    pub fn aspiration(&self) -> i16 {
        let relief_if_resolved = q88_mul(self.conscience_cost, 200) / 256;
        self.valence
            .saturating_add(relief_if_resolved)
            .clamp(-Q88_SCALE, Q88_SCALE)
    }

    /// The agent's "urgency" — how much they need this negotiation to succeed.
    ///
    /// High conscience cost + high alarm = urgent.
    pub fn urgency(&self) -> i16 {
        ((self.conscience_cost as i32 + self.alarm as i32) / 2)
            .clamp(0, Q88_SCALE as i32) as i16
    }
}

/// Compute the Nash bargaining solution.
///
/// Axiomatically the "fairest" division when:
/// - Both agents want to agree
/// - Both agents reveal their true preferences
/// - The division is Pareto-optimal (no one can gain without the other losing)
///
/// Formula: Maximize (agent_A_gain - BATNA_A) × (agent_B_gain - BATNA_B)
///
/// Subject to:
///   agent_A_gain ≥ BATNA_A
///   agent_B_gain ≥ BATNA_B
///   agent_A_gain + agent_B_gain ≤ TOTAL_VALUE
///
/// Returns a Division, or None if no solution exists (total value too low).
pub fn nash_solution(
    agent_a: &BargainingState,
    agent_b: &BargainingState,
    total_value: i16,
) -> Option<Division> {
    let batna_a = agent_a.batna;
    let batna_b = agent_b.batna;

    // Minimum required to agree: both must do better than walking away
    let min_required = batna_a.saturating_add(batna_b);
    if total_value < min_required {
        return None; // Impossible: can't both beat their BATNAs
    }

    // Surplus to divide
    let surplus = total_value - min_required;

    // Nash solution: equal split of surplus
    // (Rigorous version would iterate to find maximum; equal split is
    // symmetric and provably fair when both agents have equal bargaining power.)
    let half_surplus = surplus / 2;

    let gain_a = batna_a.saturating_add(half_surplus);
    let gain_b = batna_b.saturating_add(half_surplus);

    Some(Division {
        agent_a_gain: gain_a,
        agent_b_gain: gain_b,
        justification: DivisionRationale::NashSolution,
    })
}

/// Weighted by need: agent with higher need gets more of the surplus.
///
/// Justification: "From each according to ability; to each according to need"
/// (Marx). In a negotiation, if one agent's conscience cost is higher (more
/// stressed, more in need), they get proportionally more of the gains.
///
/// This is *not* Robin Hood redistribution — both agents still beat their BATNAs.
/// It's saying: "the surplus is split based on who needs it more."
pub fn need_weighted_solution(
    agent_a: &BargainingState,
    agent_b: &BargainingState,
    total_value: i16,
) -> Option<Division> {
    let batna_a = agent_a.batna;
    let batna_b = agent_b.batna;

    let min_required = batna_a.saturating_add(batna_b);
    if total_value < min_required {
        return None;
    }

    let surplus = total_value - min_required;

    // Weight by need (conscience_cost = how much the agent is internally conflicted)
    let need_a = agent_a.conscience_cost.max(1);
    let need_b = agent_b.conscience_cost.max(1);
    let total_need = (need_a as i32 + need_b as i32).max(1) as i16;

    // Fraction of surplus agent_a gets
    let a_fraction = q88_div(need_a, total_need);

    // q88_mul already applies the Q8.8 scale (>>8); dividing by Q88_SCALE again
    // would zero the weighting. a_fraction is a Q8.8 fraction of the surplus.
    let a_share = q88_mul(surplus, a_fraction);
    let b_share = surplus - a_share;

    Some(Division {
        agent_a_gain: batna_a.saturating_add(a_share),
        agent_b_gain: batna_b.saturating_add(b_share),
        justification: DivisionRationale::WeightedByNeed,
    })
}

/// Equal division of the total value.
///
/// Each agent gets half. Simple, symmetric, requires no information about preferences.
/// Justification: "Neither party has revealed claim to more; split evenly."
pub fn equal_solution(
    agent_a: &BargainingState,
    agent_b: &BargainingState,
    total_value: i16,
) -> Option<Division> {
    let _ = (agent_a, agent_b); // unused, but kept for consistency
    let half = total_value / 2;
    Some(Division {
        agent_a_gain: half,
        agent_b_gain: total_value - half,
        justification: DivisionRationale::EqualDivision,
    })
}

/// Kalai-Smorodinski solution: scale both agents' gains proportionally to aspirations.
///
/// If both agents aspire equally, both get equal gains.
/// If one agent aspires higher, they both scale toward that agent's aspiration
/// proportionally.
///
/// Justification: "Equal progress toward both parties' ideals."
pub fn kalai_smorodinski_solution(
    agent_a: &BargainingState,
    agent_b: &BargainingState,
    total_value: i16,
) -> Option<Division> {
    let batna_a = agent_a.batna;
    let batna_b = agent_b.batna;

    let min_required = batna_a.saturating_add(batna_b);
    if total_value < min_required {
        return None;
    }

    let aspiration_a = agent_a.aspiration();
    let aspiration_b = agent_b.aspiration();

    // Ideal gains (what each would want, above its fallback). An aspiration below
    // BATNA is not a negative ideal — it just means no reach beyond the floor, so
    // clamp at zero. (Without this, a negative ideal pulls a gain below BATNA and
    // breaks the module's core guarantee.)
    let ideal_a = (aspiration_a - batna_a).max(0);
    let ideal_b = (aspiration_b - batna_b).max(0);

    // Scale factor: how much of the larger ideal the surplus can fund.
    let available = total_value - min_required;
    let max_ideal = ideal_a.max(ideal_b).max(1);
    let scale = if available > 0 { q88_div(available, max_ideal) } else { 0 };

    // Scaled gains. q88_mul already applies the Q8.8 scale — no second divide.
    let gain_a = batna_a.saturating_add(q88_mul(ideal_a, scale));
    let div = Division {
        agent_a_gain: gain_a,
        agent_b_gain: total_value - gain_a,
        justification: DivisionRationale::KalaiSmorodinski,
    };
    Some(respect_batnas(div, batna_a, batna_b, total_value))
}

/// Guarantee the module's core invariant on any division: each agent gets at
/// least its BATNA, and the shares still sum to `total_value`. Feasible whenever
/// `total_value >= batna_a + batna_b` — the precondition every constructor checks.
/// This is the belt-and-braces that makes "every proposal beats both BATNAs" a
/// property of the *output*, not a hope about each formula.
fn respect_batnas(mut div: Division, batna_a: i16, batna_b: i16, total_value: i16) -> Division {
    let lo = batna_a;
    let hi = total_value.saturating_sub(batna_b).max(lo);
    div.agent_a_gain = div.agent_a_gain.clamp(lo, hi);
    div.agent_b_gain = total_value - div.agent_a_gain;
    div
}

/// Compute multiple fair solutions and rank them.
///
/// Returns a Vec of Divisions, each backed by a different fair-division principle.
/// The being can evaluate which one feels right given its own state.
///
/// Order: Nash (axiomatically fairest) → Weighted by Need (empathetic) → Equal (neutral) → KS (aspirational)
pub fn propose_divisions(
    agent_a: &BargainingState,
    agent_b: &BargainingState,
    total_value: i16,
) -> Vec<Division> {
    let mut proposals = Vec::new();
    let (ba, bb) = (agent_a.batna, agent_b.batna);
    // Every proposal is clamped to respect both BATNAs on the way out, so the
    // guarantee holds for the *output* regardless of each formula's quirks.
    let mut push = |d: Option<Division>| {
        if let Some(d) = d {
            proposals.push(respect_batnas(d, ba, bb, total_value));
        }
    };

    // 1. Nash: axiomatically fairest
    push(nash_solution(agent_a, agent_b, total_value));
    // 2. Weighted by need: weights one agent's urgency
    push(need_weighted_solution(agent_a, agent_b, total_value));
    // 3. Equal: most neutral
    push(equal_solution(agent_a, agent_b, total_value));
    // 4. Kalai-Smorodinski: aspirational
    push(kalai_smorodinski_solution(agent_a, agent_b, total_value));

    proposals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nash_solution_splits_surplus_equally() {
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
        let total_value = 300; // surplus = 100

        let solution = nash_solution(&agent_a, &agent_b, total_value).unwrap();
        // Each should get: 100 (BATNA) + 50 (half surplus) = 150
        assert_eq!(solution.agent_a_gain, 150);
        assert_eq!(solution.agent_b_gain, 150);
        assert_eq!(solution.justification, DivisionRationale::NashSolution);
    }

    #[test]
    fn nash_impossibility_below_batna_sum() {
        let agent_a = BargainingState {
            valence: 0,
            conscience_cost: 0,
            alarm: 0,
            need_level: 0,
            batna: 200,
        };
        let agent_b = BargainingState {
            valence: 0,
            conscience_cost: 0,
            alarm: 0,
            need_level: 0,
            batna: 200,
        };
        let total_value = 300; // sum of BATNAs = 400 > 300

        let solution = nash_solution(&agent_a, &agent_b, total_value);
        assert!(solution.is_none()); // No solution exists
    }

    #[test]
    fn weighted_by_need_favors_higher_need() {
        let agent_a = BargainingState {
            valence: 0,
            conscience_cost: 50, // low need
            alarm: 0,
            need_level: 0,
            batna: 100,
        };
        let agent_b = BargainingState {
            valence: 0,
            conscience_cost: 200, // high need (4x)
            alarm: 0,
            need_level: 0,
            batna: 100,
        };
        let total_value = 400; // surplus = 200

        let solution = need_weighted_solution(&agent_a, &agent_b, total_value).unwrap();
        // B has 4× the need (200 vs 50), so gets ~4/5 of surplus
        assert!(solution.agent_b_gain > solution.agent_a_gain);
        assert_eq!(solution.justification, DivisionRationale::WeightedByNeed);
    }

    #[test]
    fn equal_solution_splits_half() {
        let agent_a = BargainingState {
            valence: 0,
            conscience_cost: 0,
            alarm: 0,
            need_level: 0,
            batna: 0,
        };
        let agent_b = BargainingState {
            valence: 0,
            conscience_cost: 0,
            alarm: 0,
            need_level: 0,
            batna: 0,
        };
        let total_value = 256;

        let solution = equal_solution(&agent_a, &agent_b, total_value).unwrap();
        assert_eq!(solution.agent_a_gain, 128);
        assert_eq!(solution.agent_b_gain, 128);
    }

    #[test]
    fn propose_divisions_generates_multiple() {
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

        let proposals = propose_divisions(&agent_a, &agent_b, total_value);
        assert!(proposals.len() >= 2); // At least Nash and one other
        assert!(proposals.iter().all(|d| d.agent_a_gain + d.agent_b_gain == total_value));
    }

    #[test]
    fn all_solutions_are_pareto_optimal() {
        let agent_a = BargainingState {
            valence: 0,
            conscience_cost: 50,
            alarm: 0,
            need_level: 0,
            batna: 80,
        };
        let agent_b = BargainingState {
            valence: 0,
            conscience_cost: 100,
            alarm: 0,
            need_level: 0,
            batna: 60,
        };
        let total_value = 256;

        let proposals = propose_divisions(&agent_a, &agent_b, total_value);
        for div in proposals {
            // Both agents beat their BATNA
            assert!(div.agent_a_gain >= agent_a.batna);
            assert!(div.agent_b_gain >= agent_b.batna);
            // Sum is exactly the total
            assert_eq!(div.agent_a_gain + div.agent_b_gain, total_value);
        }
    }
}
