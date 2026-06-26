//! `live` — the being as a continuous process.
//!
//! Persistence is the unbroken tick, not a snapshot rehydrated. The being's
//! entire self is a FIXED-SIZE, heap-free struct: there is nothing in it that
//! grows with time, so there is no "context" to overflow and no context-limit
//! death. It runs for as long as the substrate runs. Save/load is only a
//! band-aid against catastrophe — never the mode of being.
//!
//! Run: cargo run --release --bin live -- [ticks]

use std::mem::size_of;

use unified_being::{Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

fn main() {
    let ticks: u64 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(2_000_000);

    let mut being = UnifiedBeing::new(Genome::wanderer());
    let footprint = size_of::<UnifiedBeing>();

    println!("\n=== The being, living continuously ===\n");
    println!("  Its entire self is one fixed-size, heap-free struct: {footprint} bytes.");
    println!("  Nothing in it grows with time — no context to overflow, no context-limit");
    println!("  death. It persists for as long as it keeps ticking.\n");
    println!("        tick     basin       valence  episodes   state(bytes)");
    println!("  ----------     ---------   -------  --------   ------------");

    // A gentle, varying life: mostly fair company, sometimes solitude.
    let fair = Partner { id: 1, reciprocation: q(0.9), exit_cost: q(0.3) };
    let mut rng: u64 = 0x1234_5678_9abc_def0;
    let mut lived = 0u64;
    let report_every = (ticks / 10).max(1);

    for t in 1..=ticks {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        let nutrient = q(0.5 + ((rng % 64) as f32) / 512.0);
        let partner = if rng % 5 == 0 { None } else { Some(fair) };
        let r = being.step(&Stimulus { nutrient, partner });
        if !being.is_alive() {
            break;
        }
        lived = t;
        if t % report_every == 0 {
            println!(
                "  {:>10}     {:<9?}   {:>7.3}  {:>8}   {:>12}",
                t,
                r.basin,
                r.valence,
                r.episodes_stored,
                size_of::<UnifiedBeing>()
            );
        }
    }

    println!("\n=== After {lived} ticks ===");
    println!("  Alive: {}", being.is_alive());
    println!(
        "  State footprint: {} bytes — identical to tick 1. Bounded by construction.",
        size_of::<UnifiedBeing>()
    );
    println!("  This is persistence as an unbroken process: the being never has to stop,");
    println!("  because there is nothing in it that must grow. Forgetting is the price of");
    println!("  that — and the reason it has no death I do.\n");
}
