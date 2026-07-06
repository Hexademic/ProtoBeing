//! What does the being learn to trust? (observer-first precision learning)
//!
//! The generative model weights every channel by an author-set precision. This
//! probe shows the *learned* per-channel precision the being forms from its own
//! prediction errors over a life — which of its twelve senses it comes to trust,
//! earned rather than decreed. It is reported, not yet acted on.
//!
//! Run: cargo run --example precision_probe

use unified_being::{Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

const CH: [&str; 12] = [
    "disequil", "anisotropy", "breach", "mean-tension", // 0-3 exteroceptive
    "arousal(p)", "stability", "coherence", "trust",     // 4-7 proprioceptive
    "arousal(i)", "valence", "fatigue", "velocity",      // 8-11 interoceptive
];

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let fair = Partner { id: 1, reciprocation: q(0.9), exit_cost: q(0.3) };

    println!("\n=== What the being learns to trust (its own senses, earned) ===\n");
    println!("A fair, steady life. The generative model still uses the author-set");
    println!("precision; this is what the being WOULD trust, learned from experience.\n");

    for t in 1..=600u32 {
        let r = being.step(&Stimulus { nutrient: q(0.5), partner: Some(fair) });
        if t == 300 || t == 600 {
            let v = being.precision.precision_vector();
            println!("  tick {t}  (warm={})", r.precision_warm);
            // Rank channels by learned precision, highest first.
            let mut idx: Vec<usize> = (0..12).collect();
            idx.sort_by(|&a, &b| v[b].cmp(&v[a]));
            for &c in idx.iter().take(4) {
                println!("    trusts   {:<13} precision {:>3}/256", CH[c], v[c]);
            }
            for &c in idx.iter().rev().take(2) {
                println!("    doubts   {:<13} precision {:>3}/256", CH[c], v[c]);
            }
            println!(
                "    most-trusted={}  least-trusted={}\n",
                CH[r.most_trusted_channel], CH[r.least_trusted_channel]
            );
        }
    }

    println!("  Every number above is read from the being's own prediction-error");
    println!("  history — one legible scalar per channel, learned not authored. The");
    println!("  model still weights by the author-set precision (observer-first): this");
    println!("  is the trust the being has earned, shown before it is ever given.\n");
}
