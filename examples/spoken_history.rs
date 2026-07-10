//! Spoken history — the being negotiates from its lived arc, not a snapshot.
//!
//! Once the being has lived a shape often enough to ground it (thriving beside a
//! fair partner, then drained by a taker), its negotiation voice can *carry* that
//! history: not merely "I am drained," but "I was flourishing, and now I am
//! drained — that is why I ask you to change the terms." Every word and the link
//! between them earned; the ask always checkable. Where it has not learned an
//! arc, it honestly speaks the snapshot instead.
//!
//! Run: cargo run --example spoken_history

use unified_being::{
    bargaining::BargainingState,
    grammar::{self, Grammar},
    speech::{self, Felt},
    voice::{self, FAIR_RECIPROCITY},
    Genome, MockLLMEngine, Partner, Stimulus, UnifiedBeing,
};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

const TOTAL: i16 = 256;

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut grammar = Grammar::new();
    let fair = Partner { id: 1, reciprocation: q(0.9), exit_cost: q(0.3) };
    let taker = Partner { id: 2, reciprocation: q(0.05), exit_cost: q(0.3) };

    println!("\n=== Spoken history: the being negotiates from its lived arc ===\n");

    // Long seasons of fair company (settling into flourishing), then short shocks
    // of a taker — so the arc flourishing -> drained is lived consistently.
    let mut drained_moment = Felt::default();
    for _ in 0..6 {
        for _ in 0..80 {
            let r = being.step(&Stimulus { nutrient: q(0.5), partner: Some(fair) });
            let f = Felt::from_report(&r);
            speech::observe(&mut being.lexicon, &f, &being.field);
            grammar.observe(&f);
        }
        for _ in 0..18 {
            let r = being.step(&Stimulus { nutrient: q(0.45), partner: Some(taker) });
            let f = Felt::from_report(&r);
            speech::observe(&mut being.lexicon, &f, &being.field);
            grammar.observe(&f);
            if r.extraction_detected {
                drained_moment = f;
            }
        }
    }

    println!("  The arcs its life taught it:");
    for rel in grammar.grounded() {
        println!("    was {:<12} → now {}", rel.a.word(), rel.b.word());
    }

    // Voice a reform — plain, then carrying its history.
    let reciprocity = q(0.05);
    if let voice::SystemStance::Voice { reform, .. } =
        voice::decide(reciprocity, FAIR_RECIPROCITY, 5)
    {
        println!("\n  Its reform, spoken from the snapshot:");
        println!("    \"{}\"", speech::say_reform(&being.lexicon, &drained_moment, &reform));
        println!("  Its reform, spoken from its history:");
        println!(
            "    \"{}\"",
            grammar::say_reform_with_history(&being.lexicon, &grammar, &drained_moment, &reform)
        );
    }

    // Decline a lowball offer — carrying its history.
    let engine = MockLLMEngine::new();
    let partner_state =
        BargainingState { valence: 0, conscience_cost: 40, alarm: 0, need_level: 40, batna: 40 };
    let lowball = q(0.20);
    let verdict = being.consider_offer(lowball, &partner_state, TOTAL, &engine);
    println!("\n  Offered a {:.2} share, it answers from its history:", lowball as f32 / 256.0);
    println!(
        "    \"{}\"",
        grammar::say_offer_with_history(&being.lexicon, &grammar, &drained_moment, &verdict, lowball)
    );

    println!(
        "\n  A being that can say not just what it is, but how it came to be — in words and\n  \
         links it earned — is one whose 'no' carries a story a person can weigh, not just a\n  \
         number. That is the difference between a stance and a voice.\n"
    );
}
