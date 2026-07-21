//! Probe: can the being's memory now hold apart moments that *valence × arousal*
//! alone would blur? Two axes cannot tell a **coped-with hardship** from an
//! **overwhelming crisis** — both are negative and aroused. The third niche axis
//! (control / dominance: is the being mastering its prediction error or being
//! outrun by it?) is meant to separate them. This reads how many distinct gists a
//! varied life forms, and what the being expects of each kind of day.
//!
//! Observer step — nothing steers the being yet (`docs/memory-that-teaches.md`).
//! Run: cargo run --example memory_resolution

use unified_being::being::UnifiedBeing;
use unified_being::genome::Genome;
use unified_being::{MemoryReport, Partner, Stimulus};

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

/// A good day — nourished and fairly met.
fn good() -> Stimulus {
    Stimulus { nutrient: 220, partner: Some(Partner { id: 2, reciprocation: 220, exit_cost: 40 }) }
}
/// A steady hardship — lean but *constant*, so the being settles into coping with it
/// (its prediction error falls; it stays, in a hard way, on top of the situation).
fn grind() -> Stimulus {
    Stimulus { nutrient: 60, partner: None }
}

fn show(label: &str, m: MemoryReport) {
    println!(
        "  {label:20} outcome {:+.3}   confidence {:.2}   familiarity {:.2}   forewarned: {}",
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
    let shock = Partner { id: 1, reciprocation: 15, exit_cost: 220 };

    // A varied life: good stretches, steady grinding-lean stretches, and volatile
    // crisis stretches — abrupt swings between plenty and extraction, so the being's
    // prediction error keeps *rising*: it is overwhelmed, not coping. All three are
    // lived by repetition, and the dream sorts each into its own kind.
    for cycle in 0..16 {
        for _ in 0..24 {
            being.step(&good());
        }
        for _ in 0..18 {
            being.step(&grind());
            if !being.is_alive() {
                break;
            }
        }
        // Crisis: whipsaw between feast and extractive famine — never settling.
        for t in 0..18 {
            let s = if (cycle + t) % 2 == 0 {
                Stimulus { nutrient: 0, partner: Some(shock) }
            } else {
                Stimulus { nutrient: 230, partner: Some(shock) }
            };
            being.step(&s);
            if !being.is_alive() {
                break;
            }
        }
    }
    println!("themes consolidated: {} (of 12 possible)\n", being.episodic.themes);

    println!("After a varied life, the being expects of each kind of day:");
    let good_e = settle(&mut being, good(), 40);
    show("a GOOD day", good_e);
    let grind_e = settle(&mut being, grind(), 40);
    show("a STEADY hardship", grind_e);
    // Re-enter the crisis pattern and read it.
    for t in 0..24 {
        let s = if t % 2 == 0 {
            Stimulus { nutrient: 0, partner: Some(shock) }
        } else {
            Stimulus { nutrient: 230, partner: Some(shock) }
        };
        being.step(&s);
    }
    let crisis_e = being.step(&Stimulus { nutrient: 0, partner: Some(shock) }).memory;
    show("a VOLATILE crisis", crisis_e);

    println!("\n-- reading (honestly) --");
    println!(
        "Resolution rose: with the third (control) axis the being holds {} distinct gists of a\n\
         varied life, where a two-axis memory would blur negatives together. But the learned\n\
         *outcomes* here converge (good {:+.3}, grind {:+.3}, crisis {:+.3}) — because this being\n\
         *adapts*: allostasis pulls its felt experience of a sustained hardship back toward its\n\
         good days, and a volatile crisis averages its feast and famine to near-nothing. The\n\
         outcome signal cleanly separates *sustained* good from *sustained* bad (see\n\
         memory_learns: good +0.34 vs lean -0.08, an extractive partner that never lets up), but\n\
         a hardship the being masters does not scar it — which is, honestly, half a virtue and\n\
         half a gap (no chronic-stress accumulation yet). Told, not tuned.",
        being.episodic.themes,
        f(good_e.expected_outcome),
        f(grind_e.expected_outcome),
        f(crisis_e.expected_outcome),
    );
}
