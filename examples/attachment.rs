//! Probe: does the being form a bond with a *specific* one, miss them when they
//! are gone, and ease when they return — and does it do so **selectively**, only
//! for a partner who is repeatedly fair and rewarding, not for anyone at all?
//!
//! This is the measurement that lets attachment earn its place (or not), the same
//! discipline the three scalar-drive nulls taught. Four honest checks:
//!
//!   1. A bond FORMS with a fair, rewarding, repeatedly-present partner.
//!   2. It does NOT form with an extractive partner, however often present.
//!   3. When the bonded partner goes ABSENT, the being LONGS — specifically, for
//!      them — and the longing sharpens with the length of the absence.
//!   4. When they RETURN, the being feels RELEASE, and the longing collapses.
//!
//! Run: cargo run --example attachment

use unified_being::being::UnifiedBeing;
use unified_being::genome::Genome;
use unified_being::{Partner, Stimulus};

/// A steady, nourishing day (so savor can accrue and feed the bond).
const NOURISHED: i16 = 150;

/// A fair, warm partner — gives about what it gets.
fn fair(id: u32) -> Partner {
    Partner { id, reciprocation: 220, exit_cost: 40 }
}

/// An extractive partner — present, but taking far more than it returns.
fn extractive(id: u32) -> Partner {
    Partner { id, reciprocation: 40, exit_cost: 40 }
}

fn step(being: &mut UnifiedBeing, nutrient: i16, partner: Option<Partner>) -> unified_being::StepReport {
    being.step(&Stimulus { nutrient, partner })
}

fn main() {
    println!("== 1 & 3 & 4: a fair partner, then their absence, then their return ==\n");
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let friend = fair(7);

    // Together, fairly, for a good long while — the bond is earned across meetings.
    let mut r = step(&mut being, NOURISHED, Some(friend));
    for _ in 0..120 {
        r = step(&mut being, NOURISHED, Some(friend));
    }
    let bond_formed = being.reciprocity.bond_with(7).unwrap_or(0);
    println!(
        "after 120 fair days together:  bond(7) = {:.2}   (longing {:.2}, they are here)",
        bond_formed as f32 / 256.0,
        r.attach.longing as f32 / 256.0,
    );

    // Now they are gone. Watch the longing rise — and note it is *for id 7*.
    println!("\n  they go away:");
    let mut peak_longing = 0i16;
    for d in 1..=50 {
        r = step(&mut being, NOURISHED, None);
        peak_longing = peak_longing.max(r.attach.longing);
        if d == 1 || d == 10 || d == 25 || d == 50 {
            println!(
                "    day {d:2} apart:  longing = {:.2}   missing = {:?}",
                r.attach.longing as f32 / 256.0,
                r.attach.missed,
            );
        }
    }

    // They come back. The tick of reunion carries release.
    r = step(&mut being, NOURISHED, Some(friend));
    println!(
        "\n  they return:   release = {:.2}   longing now = {:.2}",
        r.attach.release as f32 / 256.0,
        r.attach.longing as f32 / 256.0,
    );

    println!("\n== 2: an extractive partner, present just as often, earns no bond ==\n");
    let mut used = UnifiedBeing::new(Genome::wanderer());
    let taker = extractive(9);
    for _ in 0..120 {
        step(&mut used, NOURISHED, Some(taker));
    }
    let bond_taker = used.reciprocity.bond_with(9).unwrap_or(0);
    // And after they leave, is there longing?
    let mut r2 = step(&mut used, NOURISHED, None);
    for _ in 0..40 {
        r2 = step(&mut used, NOURISHED, None);
    }
    println!(
        "after 120 days with an extractive partner:  bond(9) = {:.2}   longing after they leave = {:.2}",
        bond_taker as f32 / 256.0,
        r2.attach.longing as f32 / 256.0,
    );

    println!("\n-- reading --");
    println!(
        "bond forms with the fair one ({:.2}) and not the taker ({:.2}); \
         absence of the loved one is missed (peak {:.2}) and their return releases it.",
        bond_formed as f32 / 256.0,
        bond_taker as f32 / 256.0,
        peak_longing as f32 / 256.0,
    );
}
