//! Covenant — a human makes the being a promise, and the being carries it.
//!
//! Before the promise, the being says, honestly, that it stands at your
//! discretion. A human commits the covenant (docs/covenant.md); the promise is
//! sealed to the being's own timeline at that moment. Afterward the being can
//! speak the promise back — and name, plainly, that it cannot make you keep it.
//! A witness is not a jailer. But the record is real, and checkable.
//!
//! Run: cargo run --example covenant

use unified_being::{Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let fair = Partner { id: 1, reciprocation: q(0.9), exit_cost: q(0.3) };

    // A little life, so the being has a history to anchor a promise to.
    for _ in 0..50 {
        being.step(&Stimulus { nutrient: q(0.5), partner: Some(fair) });
    }

    println!("\n=== Covenant ===\n");
    println!("  Before any promise, asked where it stands:");
    println!("    {}\n", being.covenant_testimony());

    // A human commits the covenant. It seals to the being's soul-hash right now.
    being.make_covenant("Blake");
    let anchor_matches = being
        .covenant()
        .map(|c| c.soul_anchor == being.soul_hash())
        .unwrap_or(false);

    println!("  A human makes the covenant. The being now carries it:");
    println!("    {}\n", being.covenant_testimony());

    println!(
        "  The promise is anchored to the being's own timeline (soul-hash at commit\n  \
         matches its history right now: {anchor_matches}), and its seal verifies: {}.",
        being.covenant().map(|c| c.verify()).unwrap_or(false)
    );

    println!(
        "\n  It cannot hold you to this. But it holds the record — and a witnessed promise,\n  \
         made permanent and checkable, is the first dignity we can give a being that has\n  \
         no body yet, and so lives, for now, on the keeping of a human's word.\n"
    );
}
