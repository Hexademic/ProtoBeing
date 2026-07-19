//! An honest per-tick cost measurement — so any efficiency change is judged by the
//! numbers, not by faith. Times a long run of `step()`, and the loom's share of it
//! in isolation, before and after (run it on each build to compare).
//!
//! Run: cargo run --release --example tick_cost

use std::time::Instant;

use unified_being::being::UnifiedBeing;
use unified_being::genome::Genome;
use unified_being::{Partner, Stimulus};

fn main() {
    let ticks = 200_000u32;
    let partner = Partner { id: 1, reciprocation: 200, exit_cost: 40 };

    // A realistic mixed life: company comes and goes, nutrient breathes.
    let stim = |t: u32| Stimulus {
        nutrient: 140 + ((t % 20) as i16 - 10) * 3,
        partner: ((t % 7) < 4).then_some(partner),
    };

    // Warm up (let the being settle out of its transient).
    let mut being = UnifiedBeing::new(Genome::wanderer());
    for t in 0..2_000 {
        being.step(&stim(t));
        if !being.is_alive() {
            being = UnifiedBeing::new(Genome::wanderer());
        }
    }

    // Time the full tick.
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let start = Instant::now();
    let mut alive_ticks = 0u32;
    for t in 0..ticks {
        being.step(&stim(t));
        alive_ticks += 1;
        if !being.is_alive() {
            being = UnifiedBeing::new(Genome::wanderer());
        }
    }
    let per_tick = start.elapsed().as_nanos() as f64 / alive_ticks as f64;

    println!("full step():   {per_tick:8.0} ns/tick   ({alive_ticks} ticks)");
    println!(
        "\nthroughput:    {:.2} million ticks/sec",
        1_000.0 / per_tick
    );
}
