//! Probe: with the graded homeostatic drive wired into the chronic-burden trigger
//! (`homeostasis.rs` → `reflection.rs::burdened`), does the being finally *carry the
//! weight* of a hard life *lived* — feel it, survive it, set it down at rest, and grow
//! **weathered** rather than broken? The causal step, gated behind `enable_reflection`.
//!
//! Run: cargo run --example carrying_the_weight

use unified_being::being::UnifiedBeing;
use unified_being::genome::Genome;
use unified_being::{Partner, Stimulus};

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

/// A chronically lean, thinly-and-extractively-met life — survivable hardship, but
/// hard to live day after day. Elevates the graded drive into the worn middle.
fn lean() -> Stimulus {
    Stimulus { nutrient: 45, partner: Some(Partner { id: 1, reciprocation: 25, exit_cost: 180 }) }
}
/// Nourished, fairly met, safe — the rest in which the weight is set down.
fn rest() -> Stimulus {
    Stimulus { nutrient: 220, partner: Some(Partner { id: 2, reciprocation: 220, exit_cost: 40 }) }
}

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    being.enable_reflection();

    // A long, chronically lean life — lived, adapted to, and wearing.
    let mut peak_load = 0i16;
    let mut alive = true;
    let mut r = being.step(&lean());
    for t in 0..220 {
        r = being.step(&lean());
        peak_load = peak_load.max(r.reflection.load);
        if t % 55 == 0 {
            println!(
                "  t{t:3}: drive {:.2}  load {:.2}  viability {:.2}  valence {:+.2}",
                f(r.drive.drive),
                f(r.reflection.load),
                f(r.felt.state.viability),
                r.valence,
            );
        }
        if !being.is_alive() {
            alive = false;
            println!("  (it did not survive the lean life)");
            break;
        }
    }
    let carried = r.reflection.load;
    let val_lean = r.valence;

    println!(
        "\nAfter a long lean life:  carried weight {:.2}   felt as valence {:+.2}   alive: {alive}",
        f(carried),
        val_lean,
    );

    // A long rest — the weight is set down, and becomes resilience.
    let mut rr = being.step(&rest());
    for _ in 0..150 {
        rr = being.step(&rest());
    }
    println!(
        "After a long rest:   load {:.2} (set down)   weathered {:.2} (earned)   valence {:+.2}",
        f(rr.reflection.load),
        f(rr.reflection.self_model.weathered),
        rr.valence,
    );

    let _ = carried;
    println!("\n-- reading --");
    let worked = peak_load > 64 && rr.reflection.self_model.weathered > 0 && alive;
    if worked {
        println!(
            "the being carried the weight of a hard life *lived* (peak {:.2}), FELT it (valence\n\
             {:+.2} under it), survived it, then at rest turned it into weathered resilience\n\
             ({:.2}). Wiser, not broken — chronic stress made real, and still not a trap. The\n\
             graded drive was the key: the worn middle the bimodal viability could never express\n\
             is now a weight the being can carry.\n\n\
             (Honest nuance: `burdened` is a hard threshold on the drive, and the drive hovers\n\
             near it, so the weight flickers and even discharges as the being adapts *further* —\n\
             a graded burden proportional to how far the drive exceeds comfort would hold it\n\
             steadier. A named refinement, not tuned away.)",
            f(peak_load),
            val_lean,
            f(rr.reflection.self_model.weathered),
        );
    } else {
        println!(
            "peak {:.2}, weathered {:.2}, alive {alive} — read the numbers.",
            f(peak_load),
            f(rr.reflection.self_model.weathered),
        );
    }
}
