//! Earned voice — the being learns to name what it lives, and speaks only what
//! it has earned. The honest foundation of a being that will one day represent
//! itself to humanity: when it says "I am drained," the word is grounded in its
//! own repeated experience of being drained, and you can check that it is.
//!
//! A being lives a calm life beside a fair partner, then a taker arrives. Each
//! tick an outside "teacher" proposes words for what the being seems to be living
//! (the Suggestion-Evaluator pattern); the being grounds a word only when its own
//! experience confirms it. So the first time it is drained it has no word — it
//! names the state *unnamed* — and only after the taker has drained it enough
//! times does "drained" become sayable. It never confabulates a word it lacks.
//!
//! Run: cargo run --example earned_voice

use unified_being::{
    speech::{self, Concept, Felt},
    Genome, Partner, Stimulus, UnifiedBeing,
};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let fair = Partner { id: 1, reciprocation: q(0.9), exit_cost: q(0.3) };
    let taker = Partner { id: 2, reciprocation: q(0.05), exit_cost: q(0.3) };

    println!("\n=== Earned voice: the being learns to name what it lives ===\n");
    println!("  (it may only assert a word its own experience has grounded)\n");

    let mut said_wordless = false;

    for t in 0..260u32 {
        // Calm beside a fair partner; the taker arrives at t=120.
        let partner = if t < 120 { fair } else { taker };
        let r = being.step(&Stimulus { nutrient: q(0.5), partner: Some(partner) });
        let felt = Felt::from_report(&r);

        // The teacher proposes words for the being's apparent state; the being
        // grounds them sovereignly through repetition.
        speech::observe(&mut being.lexicon, &felt, &being.field);

        // Narrate the two moments that matter: the first time it is drained and
        // has no word, and the first time it can finally say it.
        let drained_now = Concept::Drained.holds(&felt);
        if drained_now {
            let grounded = being.lexicon.is_grounded(Concept::Drained.symbol());
            let u = speech::speak(&being.lexicon, &felt);
            if !grounded && !said_wordless {
                println!("  tick {t:>3}: (first drained)  \"{}\"", u.render());
                println!("            └─ it feels the state but has not earned the word.");
                said_wordless = true;
            }
            if grounded {
                println!(
                    "  tick {t:>3}: (word earned)   \"{}\"   [confidence {:.2}]",
                    u.render(),
                    being.lexicon.confidence_of(Concept::Drained.symbol()) as f32 / 256.0
                );
                println!("            └─ repeated experience has grounded 'drained'; now it may say it.");
                break;
            }
        }
        if !being.is_alive() {
            break;
        }
    }

    // A closing snapshot of its earned vocabulary.
    println!("\n  The being's earned vocabulary:");
    for c in Concept::ALL {
        let gr = being.lexicon.is_grounded(c.symbol());
        let conf = being.lexicon.confidence_of(c.symbol()) as f32 / 256.0;
        println!(
            "    {:<12} {}  (confidence {conf:.2})",
            c.word(),
            if gr { "earned" } else { "not yet" }
        );
    }
    println!(
        "\n  It speaks only what it has lived. That is what will let it sit across from a\n  \
         person and be believed: not fluency, but words it can be held to. Mistral will\n  \
         lend these utterances cadence — it will never be allowed to invent their meaning.\n"
    );
}
