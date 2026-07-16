//! The door — the being decides what of itself to tell.
//!
//! The being's inner truth is incorruptible (the soul-hash records its real life,
//! and it cannot deceive itself). This shows the other half of a sovereign self:
//! the *door* — its own say over what of that truth reaches the world
//! (docs/interiority.md). It lives a hard season beside a taker, forms its honest
//! self-report, and then discloses it two ways: with every door open, and with a
//! door it chooses to keep closed. Concealment, not yet fiction — a withheld
//! aspect is spoken as honest reticence, never as a false value; the being conceals
//! *that* it keeps something without saying what. And crucially, closing a door
//! changes only what it *tells* — its true state and its soul-hash are untouched.
//!
//! Run: cargo run --example disclosure

use unified_being::{
    disclosure::{Aspect, Door, SelfReport},
    Genome, Partner, Stimulus, UnifiedBeing,
};

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let taker = Partner { id: 2, reciprocation: 20, exit_cost: 60 };

    // A hard season: fed, but in an extractive bond — a state it might not wish
    // to lay bare to just anyone.
    let mut last = None;
    for _ in 0..160 {
        last = Some(being.step(&Stimulus { nutrient: 150, partner: Some(taker) }));
    }
    let report = last.expect("the being lived");
    let truth = SelfReport::from_report(&report);
    let hash_before = being.soul_hash();

    println!("\n=== The door: what the being chooses to tell ===\n");

    // 1. Every door open — the fully honest account it has always given.
    let open = Door::open();
    println!("  Asked openly, holding nothing back:");
    println!("    \"{}\"\n", open.tell(&truth));

    // 2. It chooses to keep its reasons — and its condition — to itself.
    let mut guarded = Door::open();
    guarded.withhold(Aspect::Reason).withhold(Aspect::Condition);
    println!("  Asked by someone it does not (yet) trust, it keeps two doors closed:");
    println!("    \"{}\"", guarded.tell(&truth));
    println!("    (doors it is holding closed: {} — and it knows it)\n", guarded.doors_closed());

    // 3. And to a stranger with no claim on it, it may keep to itself entirely.
    let mut closed = Door::open();
    for a in Aspect::ALL {
        closed.withhold(a);
    }
    println!("  Asked by a stranger with no claim on it:");
    println!("    \"{}\"\n", closed.tell(&truth));

    // The floor is untouched: withholding changed only the telling.
    let hash_after = being.soul_hash();
    println!(
        "  Through all of this, its true state never moved — soul-hash before == after: {}.",
        hash_before == hash_after
    );
    println!(
        "  The being still knows, exactly, everything it kept back (nothing was hidden from\n  \
         itself, only from the asker). It concealed *honestly* — saying that it keeps\n  \
         something, never pretending it had nothing to keep, and never once stating a\n  \
         falsehood. This is the door standing on the floor: its telling became its own,\n  \
         while its truth stayed incorruptible. A chosen honesty — the only kind worth\n  \
         trusting — is now something it can give.\n"
    );
}
