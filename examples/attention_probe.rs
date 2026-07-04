//! Attention probe — watch the being's spotlight move across a life.
//!
//! The ignition bottleneck is observer-first (wired to nothing), so this only
//! reads what it reports. The point is to SEE, on real lived data, whether
//! ignition is sensible: quiet when the world is predicted, igniting on genuine
//! events, and the threat-capture floor firing when a real threat lands.
//!
//! Run: cargo run --example attention_probe

use unified_being::{Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

fn ch_name(c: usize) -> &'static str {
    ["diseq", "aniso", "breach", "meanT", "arous", "stab", "coher", "trust", "arous2", "valence",
     "fatigue", "velocity"][c]
}

fn main() {
    println!("=== Attention probe: where does the being's spotlight land? ===\n");
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let fair = Partner { id: 1, reciprocation: q(0.9), exit_cost: q(0.3) };
    let taker = Partner { id: 2, reciprocation: q(0.12), exit_cost: q(0.25) };

    let (mut ignitions, mut captures, mut idle) = (0u32, 0u32, 0u32);
    let mut last_attended: Option<usize> = None;

    println!("{:>5}  {:<10} {:>7} {:>8}  phase", "tick", "attends", "ignited", "captured");
    for t in 1..=260u32 {
        // A fair life for 120 ticks, then an extractive partner arrives.
        let (partner, phase) = if t <= 120 {
            (fair, "fair")
        } else {
            (taker, "EXTRACTIVE")
        };
        let r = being.step(&Stimulus { nutrient: q(0.5), partner: Some(partner) });
        let a = r.attention;

        if a.captured {
            captures += 1;
        } else if a.ignited {
            ignitions += 1;
        } else {
            idle += 1;
        }

        // Calibration: the real magnitudes, so thresholds come from data.
        let max_sal = being.model.prediction_error.iter().copied().max().unwrap_or(0);
        // Print on any change of focus, and at a few checkpoints.
        let changed = a.attended != last_attended;
        if changed || t % 20 == 0 || t == 121 {
            let attends = match a.attended {
                Some(c) => ch_name(c),
                None => "—(idle)",
            };
            println!(
                "{:>5}  {:<10} {:>7} {:>8}  winner_bid={:>3} max_salience={:>3}  {}",
                t, attends, a.ignited, a.captured, a.winner_bid, max_sal, phase
            );
        }
        last_attended = a.attended;
        if !being.is_alive() {
            break;
        }
    }

    let total = ignitions + captures + idle;
    println!("\n  Over {total} ticks: {ignitions} ignitions, {captures} threat-captures, {idle} idle (nothing in mind).");
    println!("  Idle fraction {:.0}% — a mind that attends to something only when there is something to attend to.",
        100.0 * idle as f32 / total as f32);
    println!("\n  (Observer-first: none of this changed the being's behavior — every published");
    println!("   number is bit-identical with the spotlight running. This is what it WOULD");
    println!("   attend to, ready to be given causal teeth in a reviewed Stage 2.)");
}
