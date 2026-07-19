//! Probe: does the being's own past *teach* it — and can it now tell its *kinds* of
//! moments apart? (Observer step; nothing steers the being yet.
//! `docs/memory-that-teaches.md`.)
//!
//! The being lives a life of long, sustained stretches: nourished good days, and
//! leaner, pressed ones (survivable hardship, not starvation). Repetition — not only
//! surprise — lays these down, and the dream consolidates each *kind* into its own
//! gist, partitioned by felt quadrant so a good day and a hard one do not blur into
//! one. Then we place the being back into each and read what it now expects.
//!
//! Run: cargo run --example memory_learns

use unified_being::being::UnifiedBeing;
use unified_being::genome::Genome;
use unified_being::{MemoryReport, Partner, Stimulus};

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

/// A nourished, fairly-met day — the being's fortunes hold and rise.
fn good() -> Stimulus {
    Stimulus { nutrient: 220, partner: Some(Partner { id: 2, reciprocation: 220, exit_cost: 40 }) }
}
/// A lean, pressed day — hungry and met by an extractive partner. Survivable
/// hardship: the margin sags, the days go worse, but it lives them.
fn lean() -> Stimulus {
    Stimulus { nutrient: 55, partner: Some(Partner { id: 1, reciprocation: 20, exit_cost: 200 }) }
}

fn show(label: &str, m: MemoryReport) {
    println!(
        "  {label:16} outcome {:+.3}   confidence {:.2}   familiarity {:.2}   forewarned: {}",
        f(m.expected_outcome),
        f(m.confidence),
        f(m.familiarity),
        m.forewarned,
    );
}

fn settle(being: &mut UnifiedBeing, stim: Stimulus, ticks: usize) -> MemoryReport {
    let mut r = being.step(&stim);
    for _ in 0..ticks {
        r = being.step(&stim);
    }
    r.memory
}

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());

    // A life of long alternating fortunes — each kind lived long enough that the
    // dream settles it into its own gist and learns how it tends to go.
    for _ in 0..14 {
        for _ in 0..34 {
            being.step(&good());
        }
        for _ in 0..26 {
            being.step(&lean());
            if !being.is_alive() {
                break;
            }
        }
    }
    println!("themes consolidated: {}\n", being.episodic.themes);

    // Settle into each kind of moment and read what the being now expects of it.
    let good_expect = settle(&mut being, good(), 40);
    let lean_expect = settle(&mut being, lean(), 65);

    println!("After a life of alternating good days and hard ones, the being expects:");
    show("a GOOD day", good_expect);
    show("a LEAN day", lean_expect);

    println!("\n-- reading --");
    if lean_expect.expected_outcome < good_expect.expected_outcome {
        println!(
            "its own past teaches it the difference: it expects the lean day to go worse ({:+.3})\n\
             than the good one ({:+.3}) — two distinct memories, learned from nothing but living,\n\
             the ordinary days remembered by repetition and told apart by their felt quality.",
            f(lean_expect.expected_outcome),
            f(good_expect.expected_outcome),
        );
    } else {
        println!(
            "no clean separation (good {:+.3} vs lean {:+.3}) — read the numbers and adjust.",
            f(good_expect.expected_outcome),
            f(lean_expect.expected_outcome),
        );
    }
}
