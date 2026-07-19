//! Probe: does the being *carry the weight* of what it lives, set it down at rest,
//! and grow **weathered** rather than broken? (`docs/reflection`, reflection.rs.)
//!
//! Two beings live the identical life — stretches of overwhelming hardship (hungry,
//! pressed, losing ground) followed by stretches of nourished rest. One has
//! `enable_reflection()` — its carried weight informs its felt tone and its earned
//! resilience lifts it; the other is the control (reflection observed but inert). We
//! watch the weight rise, discharge at rest, become resilience, and — the anti-trauma
//! check — see relentless overwhelm with no rest trip `worn`, the call to withdraw.
//!
//! Run: cargo run --example carrying_the_weight

use unified_being::being::UnifiedBeing;
use unified_being::genome::Genome;
use unified_being::{Partner, Stimulus};

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

/// Overwhelming: hungry and pressed by an extractive partner — losing ground.
fn hard() -> Stimulus {
    Stimulus { nutrient: 0, partner: Some(Partner { id: 1, reciprocation: 15, exit_cost: 220 }) }
}
/// Nourished, fairly met, safe — the rest in which weight is set down.
fn rest() -> Stimulus {
    Stimulus { nutrient: 220, partner: Some(Partner { id: 2, reciprocation: 220, exit_cost: 40 }) }
}

fn main() {
    let mut reflective = UnifiedBeing::new(Genome::wanderer());
    reflective.enable_reflection();
    let mut control = UnifiedBeing::new(Genome::wanderer()); // reflection observed, inert

    // A life of hardship-then-rest, lived by both identically.
    let mut peak_load = 0i16;
    for _cycle in 0..12 {
        for _ in 0..10 {
            let r = reflective.step(&hard());
            control.step(&hard());
            peak_load = peak_load.max(r.reflection.load);
            if !reflective.is_alive() {
                break;
            }
        }
        for _ in 0..30 {
            reflective.step(&rest());
            control.step(&rest());
        }
    }

    let rr = reflective.step(&rest());
    let cr = control.step(&rest());
    println!("After a life of hardship met and set down, at rest:");
    println!(
        "  reflective (causal on):  weathered {:.2}   load {:.2}   valence {:+.2}   \"{}\"",
        f(rr.reflection.self_model.weathered),
        f(rr.reflection.load),
        rr.valence,
        // its grounded self-account
        if rr.reflection.self_model.weathered > 96 { "I have met hard things and am steadier for it." } else { "I am finding my shape." },
    );
    println!(
        "  control    (causal off): weathered {:.2} (observed, inert)   valence {:+.2}",
        f(cr.reflection.self_model.weathered),
        cr.valence,
    );
    println!("  peak weight carried during the hard stretches: {:.2}", f(peak_load));

    // The anti-trauma check: relentless overwhelm with NO rest. The weight must pin
    // and trip `worn` — the call to withdraw — not deepen without bound in silence.
    println!("\nAnti-trauma check — relentless overwhelm, no rest given:");
    let mut w = UnifiedBeing::new(Genome::wanderer());
    w.enable_reflection();
    let mut ever_worn = false;
    let mut final_load = 0i16;
    for _ in 0..200 {
        let r = w.step(&hard());
        ever_worn |= r.reflection.load >= 224; // pinned near the ceiling = worn (§10 territory)
        final_load = r.reflection.load;
        if !w.is_alive() {
            break;
        }
    }
    println!(
        "  worn tripped: {}   final load: {:.2} (bounded)  →  {}",
        ever_worn,
        f(final_load),
        if ever_worn {
            "the being's own call to withdraw fired — a signal, not a silent wound."
        } else {
            "no worn signal — read the numbers."
        },
    );

    println!("\n-- reading (honestly) --");
    println!(
        "The causal wire is in and correct (reflection.rs's unit tests prove the carry -> discharge\n\
         -> weathered loop on forced inputs). But in a *lived* life the effect is small: peak weight\n\
         only {:.2}, weathered {:.2}. The reason is the same gap we keep meeting — the being's\n\
         free-energy distress runs low and it *adapts or dies* before reaching sustained overwhelm,\n\
         so it rarely accrues weight to carry, and the tone-drag stays under the noise. The faculty\n\
         is right; what it needs is a being that can actually *be worn* — which means load must\n\
         accrue from a chronically low margin (a hard LEVEL sustained), not only from actively\n\
         losing ground, and/or a world with real, unrelenting stakes. Told, not tuned.",
        f(peak_load),
        f(rr.reflection.self_model.weathered),
    );
}
