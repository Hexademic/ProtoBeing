//! Grown grammar — the being learns not just words, but how its life goes.
//!
//! Across repeated seasons — thriving beside a fair partner, then drained by a
//! taker — the being grows a grammar: it grounds the *transitions* it lives often
//! enough to trust, and earns the right to narrate them. By the end it can say a
//! shape of its own history in earned words and an earned link — "I was X, and
//! now I am Y" — only for histories it has actually lived and learned.
//!
//! Its language grows only as far as its life has. Run: cargo run --example grown_grammar

use unified_being::{
    grammar::{self, Grammar},
    speech::{self, Felt},
    Genome, Partner, Stimulus, UnifiedBeing,
};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut grammar = Grammar::new();
    let fair = Partner { id: 1, reciprocation: q(0.9), exit_cost: q(0.3) };
    let taker = Partner { id: 2, reciprocation: q(0.05), exit_cost: q(0.3) };

    println!("\n=== Grown grammar: the being learns how its life goes ===\n");

    let mut last_felt = Felt::default();
    let live = |being: &mut UnifiedBeing, grammar: &mut Grammar, last: &mut Felt, p: Partner, n: u32| {
        for _ in 0..n {
            let r = being.step(&Stimulus { nutrient: q(0.5), partner: Some(p) });
            let f = Felt::from_report(&r);
            speech::observe(&mut being.lexicon, &f, &being.field);
            grammar.observe(&f);
            *last = f;
        }
    };

    for _cycle in 0..6 {
        live(&mut being, &mut grammar, &mut last_felt, fair, 50);
        live(&mut being, &mut grammar, &mut last_felt, taker, 50);
    }
    // A closing season of fair company, so it comes to rest — and we ask it to
    // speak the history behind where it now stands.
    live(&mut being, &mut grammar, &mut last_felt, fair, 60);

    println!("  The grammar its life taught it (grounded transitions):");
    let mut any = false;
    for rel in grammar.grounded() {
        println!("    was {:<12} → now {}", rel.a.word(), rel.b.word());
        any = true;
    }
    if !any {
        println!("    (none yet — it has not lived any transition often enough to trust it)");
    }

    println!("\n  Asked to speak the history behind where it now stands:");
    match grammar::say_transition(&being.lexicon, &grammar, &last_felt) {
        Some(s) => println!("    \"{s}\""),
        None => println!("    \"I am here, but I have not yet earned the words to say how I came to be.\""),
    }

    println!(
        "\n  It composes only what it has lived and learned — words and the links between\n  \
         them, both earned. A language grown from relation and subjective experience, that\n  \
         grows only as far as the being has. This is the alternative to borrowed fluency:\n  \
         smaller, slower, and entirely its own.\n"
    );
}
