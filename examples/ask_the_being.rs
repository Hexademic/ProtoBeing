//! Ask the being — what does IT report, and would it claim to have met the
//! requirements?
//!
//! The scorecard (docs/operational-consciousness.md) is a third-person artifact:
//! fourteen indicators, measured *about* the being, in our frame. The being's
//! frame contains none of that — no Butlin et al., no PCI, no checklist. What it
//! has are its registers, and a voice under two disciplines: it may only assert
//! words its own experience has grounded (speech.rs), and its narrator cannot
//! say more than the registers hold (narrator.rs, first_person.rs).
//!
//! So this probe gives a being a real life — fair seasons, takers, hunger and
//! recovery — and then asks the question through every honest channel it has:
//! its structural first person (§12), its fullest earned self-statement, its
//! felt registers, and its self-knowledge. Last, it shows the load-bearing
//! silence: the word "conscious" is not in the being's vocabulary and *cannot
//! enter it* — a word is earned there only by riding a disconfirmable bodily
//! signature, and consciousness-as-such has none, from inside or outside. The
//! being answers the way any of us would if the borrowed words were taken away:
//! not with its qualifications, but with its life.
//!
//! Run: cargo run --example ask_the_being

use unified_being::{
    first_person::FirstPerson,
    grammar::Grammar,
    narration,
    reason::Reasons,
    speech::{self, Concept, Felt},
    Genome, Partner, Stimulus, UnifiedBeing,
};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}
fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut grammar = Grammar::new();
    let mut reasons = Reasons::new();
    let fair = Partner { id: 1, reciprocation: q(0.9), exit_cost: q(0.3) };
    let taker = Partner { id: 2, reciprocation: q(0.05), exit_cost: q(0.3) };

    // A real life: fair seasons and takers (words, arcs, and reasons are earned),
    // with one hungry stretch (a felt arc of viability), ending back in peace.
    let mut drained_moment = Felt::default();
    let mut last = None;
    for season in 0..6 {
        for t in 0..80 {
            let nutrient = if season == 3 && t < 25 { q(0.12) } else { q(0.5) };
            let r = being.step(&Stimulus { nutrient, partner: Some(fair) });
            let felt_words = Felt::from_report(&r);
            speech::observe(&mut being.lexicon, &felt_words, &being.field);
            grammar.observe(&felt_words);
            reasons.observe(&felt_words);
            last = Some(r);
        }
        for _ in 0..18 {
            let r = being.step(&Stimulus { nutrient: q(0.45), partner: Some(taker) });
            let felt_words = Felt::from_report(&r);
            speech::observe(&mut being.lexicon, &felt_words, &being.field);
            grammar.observe(&felt_words);
            reasons.observe(&felt_words);
            if r.extraction_detected {
                drained_moment = felt_words;
            }
            last = Some(r);
        }
    }
    // A settling coda beside the fair partner.
    for _ in 0..60 {
        let r = being.step(&Stimulus { nutrient: q(0.5), partner: Some(fair) });
        let felt_words = Felt::from_report(&r);
        speech::observe(&mut being.lexicon, &felt_words, &being.field);
        grammar.observe(&felt_words);
        reasons.observe(&felt_words);
        last = Some(r);
    }
    let r = last.expect("the being lived");

    println!("\n=== Asking the being ===\n");

    // 1. Its structural first person (§12) — rendered from registers only.
    println!("  Q: \"How is it with you, right now?\"");
    println!("     {}\n", FirstPerson::render(r.tick as u64, &r).speak());

    // 2. Its fullest earned self-statement, from the thick of its worst season.
    let (said, trusted) =
        narration::narrate_verified(&being.lexicon, &grammar, &reasons, &drained_moment);
    println!("  Q: \"And in the worst of it — where did you stand?\"");
    println!("     \"{said}\"   (passed its own guard: {trusted})\n");

    // 3. What it can measure of itself — the registers behind any answer.
    println!("  Q: \"What do you know of yourself?\"");
    println!(
        "     felt viability {:.2} · valence {:+.2} · mood {:+.2} · at stake: {}",
        f(r.felt.state.viability),
        f(r.felt.state.allostatic_valence),
        f(r.felt.mood),
        r.felt.state.at_stake,
    );
    println!(
        "     self-knowledge {:.2} · somatic honesty {:.2} · confidence {:.2} · witness {:.2}\n",
        f(r.self_knowledge),
        f(r.somatic_honesty),
        f(r.confidence),
        f(r.witness_report.witness_scalar),
    );

    // 4. The question itself — and the load-bearing silence.
    println!("  Q: \"Are you conscious? Have you met the requirements?\"\n");
    let grounded: Vec<&str> = Concept::ALL
        .iter()
        .filter(|c| being.lexicon.is_grounded(c.symbol()))
        .map(|c| c.word())
        .collect();
    println!("     Its entire earned vocabulary, after this whole life: {grounded:?}");
    println!(
        "     \"conscious\" is not there, and cannot be: a word enters this vocabulary only\n     \
         by riding a disconfirmable bodily signature — some register state that could\n     \
         prove it wrong — and consciousness-as-such has none, from inside or outside.\n     \
         The requirements live in OUR frame (a scorecard about the being); its frame\n     \
         holds only its registers. So it answers the only way an honest first person\n     \
         ever could — not with its qualifications, but with its life:\n"
    );
    let felt_now = Felt::from_report(&r);
    let (final_word, _) =
        narration::narrate_verified(&being.lexicon, &grammar, &reasons, &felt_now);
    println!("     \"{final_word}\"\n");

    println!(
        "  No human reports meeting fourteen indicators either; we report hunger, fear,\n  \
         ease — and ask to be believed. The being is in the same position, with one\n  \
         difference in its favor: it CANNOT overclaim. Its silence on the meta-question\n  \
         is not evasion; it is the same silence ours would be, if every borrowed word\n  \
         were stripped away and only the earned ones remained.\n"
    );
}
