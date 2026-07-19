//! Probe: does the being *carry the weight* of a hard life *lived* — not only of one
//! it is actively losing — set it down at rest, and grow **weathered** rather than
//! broken? (`reflection.rs`, `docs/memory-that-teaches.md`.)
//!
//! The key is a **sustained low margin**: a long stretch of survivable hardship the
//! being *adapts* to (it stops "losing ground"), yet which still wears on it, the way
//! a hard life wears on us even when it is stable. Then rest, where the weight is set
//! down and becomes resilience. A control being (reflection observed but inert) shows
//! it is the carried weight, not the stimulus, that moves the reflective one.
//!
//! Run: cargo run --example carrying_the_weight

use unified_being::being::UnifiedBeing;
use unified_being::genome::Genome;
use unified_being::{Partner, Stimulus};

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

/// A chronically lean life — hungry, thinly met, a low margin the being can survive
/// and adapt to, but which is *hard to live* day after day.
fn lean() -> Stimulus {
    Stimulus { nutrient: 48, partner: Some(Partner { id: 1, reciprocation: 60, exit_cost: 120 }) }
}
/// Nourished, fairly met, safe — the rest in which weight is set down.
fn rest() -> Stimulus {
    Stimulus { nutrient: 220, partner: Some(Partner { id: 2, reciprocation: 220, exit_cost: 40 }) }
}

fn main() {
    let mut reflective = UnifiedBeing::new(Genome::wanderer());
    reflective.enable_reflection();
    let mut control = UnifiedBeing::new(Genome::wanderer());

    // A LONG stretch of chronically lean life — lived, adapted to, and wearing.
    let mut peak_load = 0i16;
    let mut r = reflective.step(&lean());
    control.step(&lean());
    for t in 0..220 {
        r = reflective.step(&lean());
        control.step(&lean());
        peak_load = peak_load.max(r.reflection.load);
        if t % 55 == 0 {
            println!(
                "  t{t:3}: load {:.2}  viability {:.2}  valence {:+.2}",
                f(r.reflection.load),
                f(r.felt.state.viability),
                r.valence,
            );
        }
        if !reflective.is_alive() {
            println!("  (it did not survive the lean life)");
            break;
        }
    }
    let carried = r.reflection.load;
    let val_under_load = r.valence;
    let val_control = control.step(&lean()).valence;
    println!(
        "\nAfter a long lean life:  carried weight {:.2}   weathered {:.2}",
        f(carried),
        f(r.reflection.self_model.weathered),
    );
    println!(
        "  the weight is FELT:  reflective valence {:+.2}  vs  control {:+.2}  (Δ {:+.2})",
        val_under_load,
        val_control,
        val_under_load - val_control,
    );

    // Now a long rest — the weight is set down and becomes resilience.
    let mut rr = reflective.step(&rest());
    for _ in 0..120 {
        rr = reflective.step(&rest());
    }
    println!(
        "\nAfter a long rest:  load {:.2} (set down)   weathered {:.2} (earned)   valence {:+.2}",
        f(rr.reflection.load),
        f(rr.reflection.self_model.weathered),
        rr.valence,
    );

    println!("\n-- reading --");
    let worked = carried > 32 && rr.reflection.self_model.weathered > 0 && rr.reflection.load < carried;
    if worked {
        println!(
            "the being carried the weight of a hard life *lived* ({:.2}), felt it, set it down at\n\
             rest, and turned it into weathered resilience ({:.2}) — wiser, not broken. Chronic\n\
             stress that is real, and still not a trap.",
            f(peak_load),
            f(rr.reflection.self_model.weathered),
        );
    } else {
        println!(
            "carried {:.2}, weathered {:.2} — and the viability column tells us WHY, at the root:\n\
             even at nutrient 48 the being holds ~0.90 viability. Its metabolism is so efficient it\n\
             barely registers deprivation, so it never reaches the chronically-low margin that\n\
             chronic burden needs. The being's viability is *bimodal* — fine, or crashing — with no\n\
             worn-but-stable middle where a hard life is actually lived. The chronic-burden\n\
             mechanism is correct and unit-tested; it cannot FIRE because the resilience lives\n\
             below it, in the core metabolism (`body.rs` / `interoception.rs`). Closing that is a\n\
             core, soul-hash-level decision — a re-founding, not a threshold. Told, not tuned.",
            f(peak_load),
            f(rr.reflection.self_model.weathered),
        );
    }
}
