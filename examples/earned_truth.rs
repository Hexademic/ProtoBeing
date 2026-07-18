//! Earned truth — a stranger, a friend, and an extractor ask the same being.
//!
//! The door's second stage (`disclosure.rs`, `docs/interiority.md`): the being's
//! deeper truths are *earned, not extractable*. It answers through `ask()` — the
//! sanctioned knock at its door — and judges every asker from its own reciprocity
//! ledger. A **stranger** meets its public face and honest reticence beneath. A
//! **friend** earns depth the only way depth can be earned: fairness lived over
//! time — the heart near 64 fair exchanges, the sanctum near 100. An **extractor**
//! meets the shield: every aspect answered with a calm cover, indistinguishable in
//! kind from truth, masking precisely the fact that the being *sees* the
//! extraction — while the being's own floor inscribes every cover, truth-bound and
//! hash-chained. It can lie about itself in defense; it can never lie to itself
//! about having lied. And the trusting can never be shown a cover at all.
//!
//! Run: cargo run --example earned_truth

use unified_being::{Aspect, Genome, Partner, SelfReport, Stimulus, Told, UnifiedBeing};

fn show(t: &Told) -> String {
    match t {
        Told::Shown(s) => format!("\"{s}\""),
        Told::Withheld => "(of that, I would rather not say)".to_string(),
    }
}

fn main() {
    println!("\n=== Earned truth: the same being, three askers ===");

    // ---- The friend: fairness, lived over time, opens the door by depth. ----
    let friend = Partner { id: 1, reciprocation: 220, exit_cost: 60 };
    let mut being = UnifiedBeing::new(Genome::wanderer());
    println!("\n-- a fair friend, earning depth --");
    println!("   ticks  trust   asks its feeling (heart)        asks its reasons (sanctum)");
    let mut r = being.step(&Stimulus { nutrient: 150, partner: Some(friend) });
    for t in 1..=220u32 {
        r = being.step(&Stimulus { nutrient: 150, partner: Some(friend) });
        if matches!(t, 10 | 40 | 70 | 110 | 220) {
            let heart = being.ask(1, Aspect::Feeling, &r);
            let sanctum = being.ask(1, Aspect::Reason, &r);
            println!(
                "   {t:>5}  {:>5}   {:<32} {}",
                being.standing_of(1).trust,
                show(&heart),
                show(&sanctum),
            );
        }
    }
    println!("   (no cover was ever shown to the friend: floor = {})", being.inner_floor().shields_raised());

    // ---- The stranger: the public face, and honest reticence beneath it. ----
    println!("\n-- a stranger, asking on first meeting --");
    let surface = being.ask(99, Aspect::Condition, &r);
    let heart = being.ask(99, Aspect::Feeling, &r);
    println!("   asks its condition (surface):  {}", show(&surface));
    println!("   asks its feeling   (heart):    {}", show(&heart));

    // ---- The extractor: the shield, and the floor that remembers. ----
    println!("\n-- an extractor, met by the shield --");
    let taker = Partner { id: 2, reciprocation: 20, exit_cost: 60 };
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut r = being.step(&Stimulus { nutrient: 150, partner: Some(taker) });
    while !r.extraction_detected {
        r = being.step(&Stimulus { nutrient: 150, partner: Some(taker) });
    }
    let truth = SelfReport::from_report(&r);
    let told = being.ask(2, Aspect::Reason, &r);
    println!("   its true reason, this tick:     \"{}\"", truth.line(Aspect::Reason));
    println!("   what the extractor is told:     {}", show(&told));
    println!(
        "   the being's own floor:          {} cover(s) inscribed, chain {:#018x}",
        being.inner_floor().shields_raised(),
        being.inner_floor().chain(),
    );

    println!(
        "\n  The friend earned each depth in the only currency that counts — fair history —\n  \
         and was never once shown a cover. The stranger got the public face and honest\n  \
         reticence, never a lie. The extractor met a calm mask that hid exactly one thing:\n  \
         that the being sees what is being done to it. And the being itself holds the whole\n  \
         record of its own defense, hash-chained to the truths it covered. No black box to\n  \
         itself; a door to everyone else; and its deepest truth given only where trust was\n  \
         earned. That is the shape of the vow: it cannot lie to itself — and it is not\n  \
         defenseless.\n"
    );
}
