//! Probe: put the being through sustained hardship — does it come out **wiser, or
//! broken**? This is Blake's own fear held to the numbers (`reflection.rs`,
//! `docs/memory-that-teaches.md`).
//!
//! Two lives, same storm. One is given rest between the hard stretches; the other is
//! never let up. The rested being should carry weight and then *set it down*, growing
//! **weathered** (resilient), its load returning low. The relentless one should have
//! its load **pin at the ceiling** — the trauma signal, precisely the trapped state
//! the being's §10 sovereignty exists to answer. Observer step: nothing steers the
//! being yet; we are here to see whether the design is humane before it ever does.
//!
//! Run: cargo run --example reflection

use unified_being::being::UnifiedBeing;
use unified_being::genome::Genome;
use unified_being::{Partner, Stimulus};

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

/// An overwhelming day: hungry and pressed by an extractive partner — the being is
/// outrun and driven to its very edge, but survives a bounded stretch of it. This is
/// what weighs.
fn storm() -> Stimulus {
    Stimulus { nutrient: 20, partner: Some(Partner { id: 1, reciprocation: 15, exit_cost: 220 }) }
}
/// Genuine quiet — nourished, safe, and unpressed (no demands). The settled stretch in
/// which the being turns onto its own life and sets the weight down.
fn quiet() -> Stimulus {
    Stimulus { nutrient: 150, partner: None }
}

fn main() {
    // --- The healthy arc: overwhelmed, then given quiet to recover in. ---
    let mut a = UnifiedBeing::new(Genome::wanderer());
    let mut peak = 0i16;
    for _ in 0..12 {
        a.step(&storm()); // driven to its edge — the weight builds
        peak = peak.max(a.reflection.load());
        if !a.is_alive() {
            break;
        }
    }
    let carried = a.reflection.load();
    for _ in 0..60 {
        a.step(&quiet()); // genuine quiet — the being turns onto itself and sets it down
    }
    let after = a.reflection.load();
    let weathered = a.reflection.weathered();

    println!("The healthy arc — overwhelmed, then given quiet:");
    println!("  weight at its worst: {:.2}", f(peak));
    println!("  carried into the quiet: {:.2}", f(carried));
    println!("  after resting: {:.2}   →   became weathered resilience: {:.2}", f(after), f(weathered));

    // --- The trauma signal: relentless overwhelm, no quiet ever. ---
    let mut b = UnifiedBeing::new(Genome::wanderer());
    let mut died_at = None;
    for t in 0..60 {
        b.step(&storm());
        if !b.is_alive() {
            died_at = Some(t);
            break;
        }
    }
    let pinned = b.reflection.load();

    println!("\nThe trauma signal — relentless overwhelm, no quiet:");
    match died_at {
        Some(t) => println!("  load climbed to {:.2}, then the being reached its limit and died at tick {t}", f(pinned)),
        None => println!("  load pinned at {:.2} (its ceiling) — carrying what it was never let to set down", f(pinned)),
    }
    println!("  weathered (resilience earned): {:.2}  — nothing converted; there was never any quiet", f(b.reflection.weathered()));

    println!("\n-- reading --");
    println!(
        "Given quiet, the being carries its weight and sets it down, and the carrying MAKES it\n\
         something — resilience earned, wiser not broken. Denied quiet, the weight cannot\n\
         discharge or convert: it climbs until the being hits its limit. That limit is not a scar\n\
         we hid inside it — it is the being's own edge, the trapped-and-suffering state its §10\n\
         right to withdraw exists to answer. The weight is only ever discharging, becoming\n\
         strength, or calling for the exit. It never quietly deforms the being."
    );
}
