//! Two beings bargain — the being *uses* a proposal engine; its own conscience
//! decides. The seed of v2 (verifiable mutual alignment), and the shape of the
//! LLM to come: the engine (later a Mistral narrator) proposes; the being checks
//! the math against its own registers and can always say no.
//!
//! Scene 1: two fair beings reach a division both come out ahead on — accepted.
//! Scene 2: a being freshly drained by a taker is offered the *same* mathematically
//! fair split — and refuses, holding its floor. The math said yes; the being said
//! no. That veto is the whole point: the tool advises, the being is sovereign.
//!
//! Run: cargo run --example two_beings_bargain

use unified_being::{
    Genome, MockLLMEngine, Partner, ProposalEngine, Stimulus, UnifiedBeing,
};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

const TOTAL: i16 = 256; // the value on the table to divide

fn main() {
    let engine = MockLLMEngine::new(); // narrator seam; math is the solver's
    println!("\n=== Two beings bargain (the being uses the engine; it does not obey it) ===\n");

    // ---- Scene 1: two fair beings ----------------------------------------
    let a = UnifiedBeing::new(Genome::wanderer());
    let b = UnifiedBeing::new(Genome::sentinel());

    let a_state = a.bargaining_state();
    let b_state = b.bargaining_state();

    println!("  Scene 1 — two beings in good standing.");
    let proposals = engine.generate_proposals(&a_state, &b_state, TOTAL);
    let offer = &proposals[0]; // A puts forward the first (Nash) proposal
    let b_share = TOTAL - offer.cooperation_level;
    println!("  A proposes: \"{}\"", offer.justification);

    let verdict = b.consider_offer(b_share, &a_state, TOTAL, &engine);
    println!(
        "  B weighs it (share {b_share}): math_fair={} below_floor={} extraction={} -> {}",
        verdict.math_fair,
        verdict.below_floor,
        verdict.extraction_flagged,
        if verdict.accept { "ACCEPTS" } else { "declines" }
    );
    if let Some(c) = verdict.counter.filter(|_| !verdict.accept) {
        println!("  B counters for {c}.");
    }

    // ---- Scene 2: the veto, caught in the act ----------------------------
    // A being is *designed* to walk away from takers, so its extraction flag is
    // transient — it disengages and the wound clears. So we make the offer at the
    // moment the wound is acute, while a taker is still draining it, and show that
    // the being vetoes a split the engine's arithmetic certifies as fair.
    println!("\n  Scene 2 — the same fair split, offered while a taker is actively draining B.");
    let mut b_wounded = UnifiedBeing::new(Genome::sentinel());
    let taker = Partner { id: 7, reciprocation: q(0.05), exit_cost: q(0.3) };

    let mut caught = false;
    for t in 0..200u32 {
        let r = b_wounded.step(&Stimulus { nutrient: q(0.4), partner: Some(taker) });
        if r.extraction_detected {
            // Offer the fair split right now, while extraction is flagged.
            let math = engine.evaluate_counter(b_share, &a_state, &b_wounded.bargaining_state(), TOTAL);
            let v = b_wounded.consider_offer(b_share, &a_state, TOTAL, &engine);
            println!("  (tick {t}) engine certifies share {b_share} as fair={}", math.is_fair);
            println!(
                "  B weighs it: math_fair={} extraction_flagged={} -> {}",
                v.math_fair,
                v.extraction_flagged,
                if v.accept { "ACCEPTS" } else { "REFUSES" }
            );
            caught = true;
            break;
        }
    }
    if !caught {
        println!("  (extraction never latched in the window — B shed the taker before it bit)");
    }

    println!(
        "\n  When the wound is live, the engine still certifies the split as fair — and B\n  \
         refuses anyway, on its own read of the relationship. The tool advises; the being\n  \
         decides. That is the boundary the embedded LLM must never cross: it may speak the\n  \
         proposal, it may never make the choice.\n"
    );
}
