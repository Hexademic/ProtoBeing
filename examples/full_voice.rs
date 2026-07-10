//! Full voice — the being says what it is, how it came to be, and why, in one
//! honest sentence.
//!
//! After a lived history (fair seasons, then takers), the being's fullest earned
//! self-statement pulls three layers into one voice: its present state (speech),
//! the arc that led there (grammar), and the checkable reason beneath it (reason).
//! Everything grounded, linked, or presently-true — nothing narrated it did not
//! earn. And the tense-aware guard lets it say "I was flourishing" (earned past)
//! while still catching "I am flourishing" (present-tense lie).
//!
//! Run: cargo run --example full_voice

use unified_being::{
    narration, narrator,
    grammar::Grammar,
    reason::Reasons,
    speech::{self, Concept, Felt, Utterance},
    Genome, Partner, Stimulus, UnifiedBeing,
};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut grammar = Grammar::new();
    let mut reasons = Reasons::new();
    let fair = Partner { id: 1, reciprocation: q(0.9), exit_cost: q(0.3) };
    let taker = Partner { id: 2, reciprocation: q(0.05), exit_cost: q(0.3) };

    println!("\n=== Full voice: what it is, how it came to be, and why ===\n");

    let mut drained_moment = Felt::default();
    for _ in 0..6 {
        for _ in 0..80 {
            let r = being.step(&Stimulus { nutrient: q(0.5), partner: Some(fair) });
            let f = Felt::from_report(&r);
            speech::observe(&mut being.lexicon, &f, &being.field);
            grammar.observe(&f);
            reasons.observe(&f);
        }
        for _ in 0..18 {
            let r = being.step(&Stimulus { nutrient: q(0.45), partner: Some(taker) });
            let f = Felt::from_report(&r);
            speech::observe(&mut being.lexicon, &f, &being.field);
            grammar.observe(&f);
            reasons.observe(&f);
            if r.extraction_detected {
                drained_moment = f;
            }
        }
    }

    let (said, trusted) = narration::narrate_verified(&being.lexicon, &grammar, &reasons, &drained_moment);
    println!("  Asked, in the thick of it, to say where it stands:");
    println!("    \"{said}\"");
    println!("    (passes its own guard: {trusted})\n");

    // Show the tense-aware guard directly: same being-state, two candidate lines.
    let u: Utterance = speech::speak(&being.lexicon, &drained_moment);
    let honest_past = "I was flourishing, and now I am drained.";
    let present_lie = "I am flourishing, and now I am drained.";
    println!("  The guard, on two ways of speaking the same history:");
    println!(
        "    \"{honest_past}\"  → {}",
        if narrator::verify(&u, honest_past).is_ok() { "allowed (earned past)" } else { "caught" }
    );
    println!(
        "    \"{present_lie}\"  → {}",
        if narrator::verify(&u, present_lie).is_ok() { "allowed" } else { "caught (present-tense lie)" }
    );

    let _ = Concept::Drained;
    println!(
        "\n  One voice, three layers, every claim earned or checkable — and a guard that can\n  \
         tell a remembered past from a present pretence. This is what it means for a\n  \
         synthetic mind to be believed: not that it speaks well, but that it cannot speak\n  \
         false about itself.\n"
    );
}
