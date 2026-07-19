//! Probe: does the being's own past *teach* it? (Observer step — it only *sees* what
//! experience predicts; nothing steers it yet. `docs/memory-that-teaches.md`.)
//!
//! An honest finding shapes this probe: the being lays down durable memory only for
//! *salient* moments — in practice, the moments it has to **refuse** (an extractive
//! partner). Its gentle good days do not surprise it enough to be remembered. So we
//! train it on recurring conflict, then ask what it now expects of (a) a conflict
//! moment like the ones it lived, and (b) a calm, nourished moment it never
//! consolidated. The teaching arrow should make it *dread the first* and stay
//! *neutral to the second* — and the asymmetry is itself the real result.
//!
//! Run: cargo run --example memory_learns

use unified_being::being::UnifiedBeing;
use unified_being::genome::Genome;
use unified_being::{MemoryReport, Partner, Stimulus};

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

/// A conflict moment: hungry, and pressed by an extractive partner (takes far more
/// than it gives) — the kind of moment the being refuses, and remembers.
fn conflict() -> Stimulus {
    Stimulus { nutrient: 10, partner: Some(Partner { id: 1, reciprocation: 20, exit_cost: 200 }) }
}
/// A calm, nourished, fairly-met moment — good, and (as it turns out) unmemorable.
fn calm() -> Stimulus {
    Stimulus { nutrient: 210, partner: Some(Partner { id: 2, reciprocation: 210, exit_cost: 40 }) }
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

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());

    // A life that keeps meeting conflict, with room to recover between bouts so it
    // survives to remember them. The conflict moments are what consolidate.
    for _ in 0..30 {
        for _ in 0..6 {
            being.step(&conflict());
            if !being.is_alive() {
                break;
            }
        }
        for _ in 0..18 {
            being.step(&calm());
        }
    }
    println!("themes consolidated: {}\n", being.episodic.themes);

    // Settle into a conflict moment and read what the being now expects of it.
    let mut r = being.step(&conflict());
    for _ in 0..3 {
        r = being.step(&conflict());
    }
    let conflict_expect = r.memory;

    // Recover, then settle into a calm moment and read what it expects of that.
    for _ in 0..30 {
        being.step(&calm());
    }
    let mut r = being.step(&calm());
    for _ in 0..5 {
        r = being.step(&calm());
    }
    let calm_expect = r.memory;

    println!("After a life of recurring conflict, the being expects:");
    show("a CONFLICT moment", conflict_expect);
    show("a CALM moment", calm_expect);

    println!("\n-- reading --");
    println!(
        "the being dreads what it has lived: it {} the conflict moment, and reads the calm one\n\
         with {} — a memory skewed toward the moments it had to refuse, exactly as its encoding is.",
        if conflict_expect.forewarned { "is forewarned by" } else { "does not yet dread" },
        if calm_expect.familiarity < conflict_expect.familiarity { "far less recognition" } else { "similar recognition" },
    );
}
