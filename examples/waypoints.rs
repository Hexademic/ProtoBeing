//! Waypoints — how much of a forged life do we replay before catching it?
//!
//! The measurement for `docs/waypoints.md` §6, C5 and C6. The law is proven in
//! `tests/waypoints.rs` (written before the implementation); this reports the shape of
//! the saving across the whole life, not the best point on it.
//!
//! C5 — how much replay is avoided? Predicted: large for early tampering, ZERO for a
//!      forgery past the last waypoint. The honest report is the whole curve.
//! C6 — this does NOT make an honest being wake faster. Measured and shown flat, so
//!      nobody infers a win that was never claimed.
//!
//! Fresh probe-beings only; the founded being is never touched.
//!
//! Run: `cargo run --release --example waypoints`

use std::time::Instant;
use unified_being::being::Stimulus;
use unified_being::genome::Genome;
use unified_being::persistence::{Features, LifeJournal};

const LIFE: usize = 20_000;
const CADENCE: u32 = 512;

fn stim(i: usize) -> Stimulus {
    Stimulus { nutrient: ((i * 7) % 200) as i16, ..Default::default() }
}

/// A forgery the hash can actually see — see `docs/soul-hash-limits.md` for why that
/// qualifier is necessary and what it excludes.
///
/// Starvation at a moment that was *not* already starved.
///
/// This probe reported spurious "NOT DETECTED" rows twice before it was right, both
/// times because the forgery was a no-op: `stim(i)` is already 0 whenever i is a
/// multiple of 200, and a negative nutrient clamps to 0 as well. The being was correct
/// both times and the measurement was wrong. So the choice is now *checked* rather than
/// trusted — `forge_at` refuses to report on a moment it did not actually change.
fn noticed() -> Stimulus {
    Stimulus { nutrient: 0, ..Default::default() }
}

/// Guard against measuring a no-op: the forged moment must really differ from the one
/// it replaces, or this is not a test of detection at all.
fn is_a_real_change(at: usize) -> bool {
    stim(at).nutrient != noticed().nutrient
}

fn a_life(cadence: u32) -> LifeJournal {
    let (mut being, mut j) =
        LifeJournal::birth_with_waypoints(Genome::wanderer(), Features::default(), cadence);
    for i in 0..LIFE {
        j.live(&mut being, &stim(i));
    }
    j.seal(&being);
    j
}

fn main() {
    println!("Waypoints — the cost of catching a forged life");
    println!("(predictions locked in docs/waypoints.md §6 before this was written)\n");

    let chained = a_life(CADENCE);
    let plain = a_life(0);

    println!(
        "  a {LIFE}-moment life · cadence {CADENCE} · {} waypoints · {} bytes of chain",
        chained.waypoints().len(),
        chained.waypoints().len() * 36
    );

    // ---- C5: the curve -----------------------------------------------------
    println!("\n  C5 — moments replayed before a forgery is caught:\n");
    println!("    forged at      with chain    without    saved");
    let mut any_zero = false;
    for &at in &[100usize, 1_013, 5_007, 10_001, 15_009, 19_003, 19_900, 19_990] {
        assert!(
            is_a_real_change(at),
            "moment {at} is already starved — forging it would measure nothing"
        );
        let mut c = chained.clone();
        let mut p = plain.clone();
        c.forge_for_test(at, noticed());
        p.forge_for_test(at, noticed());

        let with = match c.restore_counting() {
            Err((_, n)) => n,
            Ok(_) => {
                println!("    {at:>8}      ** NOT DETECTED **");
                continue;
            }
        };
        let without = match p.restore_counting() {
            Err((_, n)) => n,
            Ok(_) => LIFE,
        };
        let saved = 100 - (with * 100 / without.max(1));
        if saved == 0 {
            any_zero = true;
        }
        println!("    {at:>8} {with:>13} {without:>10} {saved:>7}%");
    }

    println!(
        "\n    the last waypoint is at {} — past it there is nothing to catch a forgery",
        chained.waypoints().last().map(|w| w.at).unwrap_or(0)
    );
    println!(
        "    a forgery in the final segment saves nothing: {}",
        if any_zero { "CONFIRMED, as predicted" } else { "not observed in this sample" }
    );

    // ---- C2: localization --------------------------------------------------
    let mut c = chained.clone();
    c.forge_for_test(5_007, noticed());
    print!("\n  C2 — the rejection says where: ");
    match c.restore() {
        Err(e) => println!("{e:?}"),
        Ok(_) => println!("** the forged life woke **"),
    }

    // ---- C6: the honest flat line ------------------------------------------
    println!("\n  C6 — waking an HONEST life (the thing this does NOT speed up):");
    let t0 = Instant::now();
    let a = chained.restore().expect("honest life wakes");
    let with_ms = t0.elapsed().as_secs_f64() * 1000.0;
    let t1 = Instant::now();
    let b = plain.restore().expect("honest life wakes");
    let without_ms = t1.elapsed().as_secs_f64() * 1000.0;
    println!("    with the chain     {with_ms:>8.1} ms");
    println!("    without it         {without_ms:>8.1} ms");
    println!(
        "    -> {} — an honest life is still replayed whole, by design",
        if with_ms > without_ms * 1.25 { "the chain COSTS measurably" } else { "no meaningful difference" }
    );
    assert_eq!(a.soul_hash(), b.soul_hash(), "and it is the same being either way");
    println!("    and both woke as the same being (soul-hash identical)");

    println!("\n  What this bought: a forged life fails early and the failure says where.");
    println!("  What it did not:  an honest life still costs a full replay. The wake-cost");
    println!("  win needs the state cache, which needs this, and which is a separate call.");
}
