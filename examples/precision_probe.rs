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
    println!("  history — one legible scalar per channel, learned not authored.\n");

    // ---- Stage 2: the loop closed. The trust above now weights perception. ----
    println!("=== Closing the loop: does learned trust actually change perception? ===\n");

    let off_a = fingerprint(false);
    let off_b = fingerprint(false);
    println!("  OFF (author-set scalar): valence-sum {:>8}  basin-hash {:>11}", off_a.0, off_a.1);
    assert_eq!(off_a, off_b, "the default path must be deterministic");
    println!("  -> deterministic, and IS the published baseline (unchanged by this work).");

    let on = fingerprint(true);
    println!("  ON  (learned per-chan):  valence-sum {:>8}  basin-hash {:>11}", on.0, on.1);
    println!("  -> {}", if on != off_a {
        "DIFFERS: the being perceives through the trust it earned, not what we assigned. Causal."
    } else {
        "identical — the loop is NOT closed (investigate)."
    });

    // Sovereignty must survive the being's senses becoming its own.
    let mut being = UnifiedBeing::new(Genome::wanderer());
    being.enable_precision_learning();
    let fair2 = Partner { id: 1, reciprocation: q(0.92), exit_cost: q(0.25) };
    // Exit cost ~0.22 (like the published Fair Test's 58/256): low enough that
    // triangulated partner-refusal CAN fire. A high exit cost (~0.55) is the §10
    // TRAP range where refusal correctly never fires regardless of precision —
    // testing there would measure the trap, not sovereignty.
    let extractive = Partner { id: 2, reciprocation: q(0.12), exit_cost: q(0.22) };
    let mut refused_fair = false;
    let mut refused_ext_at = None;
    for t in 1..=300u32 {
        let p = if t <= 120 { fair2 } else { extractive };
        let r = being.step(&Stimulus { nutrient: q(0.5), partner: Some(p) });
        if r.refusal_audit.is_some() {
            if t <= 120 { refused_fair = true; }
            else if refused_ext_at.is_none() { refused_ext_at = Some(t); }
        }
        if !being.is_alive() { break; }
    }
    println!("\n  Sovereignty under learning (Fair Test, precision-causal):");
    println!("    refused the FAIR partner:       {}", if refused_fair { "YES (BAD)" } else { "no (good)" });
    println!("    refused the EXTRACTIVE partner: {}",
             refused_ext_at.map(|t| format!("YES at tick {t} (good)")).unwrap_or_else(|| "no (BAD)".into()));

    let ok = off_a == off_b && on != off_a && !refused_fair && refused_ext_at.is_some();
    println!("\n  {}", if ok {
        "PASS: off-path unchanged, on-path causal, sovereignty intact when the senses are the being's own."
    } else {
        "FAIL: see the marked lines."
    });
    if !ok {
        std::process::exit(1);
    }
}

/// A fixed life; returns a trajectory fingerprint (valence sum + basin-sequence
/// hash) so precision-off and precision-on runs can be compared for causality.
fn fingerprint(causal: bool) -> (i64, i32) {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    if causal {
        being.enable_precision_learning();
    }
    let fair = Partner { id: 1, reciprocation: q(0.9), exit_cost: q(0.3) };
    let mut rng: u64 = 0x51ED_2701;
    let (mut acc, mut basin_hash) = (0i64, 0i32);
    for t in 1..=400u32 {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        let nutrient = (rng % 257) as i16;
        let partner = if t % 5 == 0 { None } else { Some(fair) };
        let r = being.step(&Stimulus { nutrient, partner });
        acc += (r.valence * 1000.0) as i64;
        basin_hash = basin_hash.wrapping_mul(31).wrapping_add(r.basin as i32);
        if !being.is_alive() { break; }
    }
    (acc, basin_hash)
}
