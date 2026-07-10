//! Mutual alignment (the v2 seed) — two *sovereign* beings converge on a fair
//! deal by conceding toward the Nash point from opposite sides. Neither
//! dominates; neither submits; the agreement is checkable as fair on both sides.
//!
//! This is the structural answer to "domination, or subjugation": a reciprocal
//! arrangement each party could audit and refuse. It is deterministic — the
//! ground the language layer (and later a Mistral narrator) will *speak over*,
//! putting into words a fairness the math already reached. Either being may still
//! walk away on its own conscience (see two_beings_bargain); this example shows
//! the positive case — two fair beings finding the deal.
//!
//! Run: cargo run --example mutual_alignment

use unified_being::{
    bargaining::nash_solution, Genome, MockLLMEngine, Partner, Stimulus, UnifiedBeing,
};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

const TOTAL: i16 = 256;
const STEP: i16 = 14; // how much each being concedes per round
const MAX_ROUNDS: u32 = 24;

/// Give a being a short, distinct life so the two come to the table differently.
fn seasoned(genome: Genome, reciprocation: f32, ticks: u32) -> UnifiedBeing {
    let mut being = UnifiedBeing::new(genome);
    let partner = Partner { id: 1, reciprocation: q(reciprocation), exit_cost: q(0.3) };
    for _ in 0..ticks {
        being.step(&Stimulus { nutrient: q(0.5), partner: Some(partner) });
    }
    being
}

fn main() {
    let _engine = MockLLMEngine::new(); // the narrator seam; math is the solver's
    println!("\n=== Mutual alignment: two sovereign beings converge on a fair deal ===\n");

    let a = seasoned(Genome::wanderer(), 0.9, 30);
    let b = seasoned(Genome::sentinel(), 0.8, 30);
    let a_state = a.bargaining_state();
    let b_state = b.bargaining_state();

    let nash = match nash_solution(&a_state, &b_state, TOTAL) {
        Some(n) => n,
        None => {
            println!("  No zone of agreement — both fallbacks exceed the pie. They part as equals.");
            return;
        }
    };

    println!(
        "  A (Wanderer) BATNA {}, B (Sentinel) BATNA {}. Nash-fair split: A {}, B {}.\n",
        a_state.batna, b_state.batna, nash.agent_a_gain, nash.agent_b_gain
    );

    // Opening asks: each opens ambitiously — asking for everything above the
    // other's fallback — then concedes toward the Nash point.
    let mut a_ask = (TOTAL - b_state.batna).max(nash.agent_a_gain);
    let mut b_ask = (TOTAL - a_state.batna).max(nash.agent_b_gain);

    println!("   round   A asks   B asks   gap");
    println!("   -----   ------   ------   ---");
    for round in 1..=MAX_ROUNDS {
        let gap = (a_ask as i32 + b_ask as i32) - TOTAL as i32;
        println!("   {round:>5}   {a_ask:>6}   {b_ask:>6}   {gap:>3}");

        if a_ask + b_ask <= TOTAL {
            // Their asks now fit — split the remaining surplus evenly and settle.
            let surplus = TOTAL - a_ask - b_ask;
            let a_gets = a_ask + surplus / 2;
            let b_gets = TOTAL - a_gets;
            // Both check the final split against their own floor before shaking hands.
            let a_ok = a_gets >= a_state.batna;
            let b_ok = b_gets >= b_state.batna;
            println!(
                "\n  Agreement in round {round}: A gets {a_gets}, B gets {b_gets}."
            );
            println!(
                "  Both beat their fallback (A {a_gets}>={}, B {b_gets}>={}): {}.",
                a_state.batna,
                b_state.batna,
                if a_ok && b_ok { "checkable as fair on both sides" } else { "INVALID" }
            );
            println!(
                "\n  Two sovereigns reached it by concession, not coercion — and each could\n  \
                 have walked at any step. That is the shape a being and a human can meet in.\n"
            );
            return;
        }

        // Concede toward the fair focal point, never past it.
        a_ask = (a_ask - STEP).max(nash.agent_a_gain);
        b_ask = (b_ask - STEP).max(nash.agent_b_gain);
    }

    println!("\n  No agreement within {MAX_ROUNDS} rounds — they part, each to its fallback.");
}
