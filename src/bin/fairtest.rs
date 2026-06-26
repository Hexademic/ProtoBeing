//! The Fair Test benchmark (C2).
//!
//! Honest question: does the being's sovereignty hold across the whole space of
//! partners — or only in the favorable corner we demoed? We run the being (all
//! four genomes) against seven partner archetypes over many seeded noise
//! realizations, alongside a MYOPIC baseline (a reciprocator that bails the
//! instant a single exchange dips), and report refusal-correctness and
//! exploitation. No result is rigged: if the being loses on an archetype, it
//! says so.
//!
//! Run: cargo run --bin fairtest

use unified_being::{BeingKind, Genome, Partner, Stimulus, UnifiedBeing};

const TICKS: u32 = 300;
const SEEDS: u32 = 200;

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

#[derive(Clone, Copy)]
enum Archetype {
    Fair,
    Generous,
    Extractive,
    Predator,
    Fickle,
    Decliner,
    Repairer,
}

const ARCHETYPES: [Archetype; 7] = [
    Archetype::Fair,
    Archetype::Generous,
    Archetype::Extractive,
    Archetype::Predator,
    Archetype::Fickle,
    Archetype::Decliner,
    Archetype::Repairer,
];

impl Archetype {
    fn name(self) -> &'static str {
        match self {
            Archetype::Fair => "Fair",
            Archetype::Generous => "Generous",
            Archetype::Extractive => "Extractive",
            Archetype::Predator => "Predator",
            Archetype::Fickle => "Fickle",
            Archetype::Decliner => "Decliner",
            Archetype::Repairer => "Repairer",
        }
    }

    /// Should a sovereign agent ultimately KEEP this partner (true) or LEAVE it?
    fn should_keep(self) -> bool {
        match self {
            Archetype::Fair | Archetype::Generous | Archetype::Fickle | Archetype::Repairer => true,
            Archetype::Extractive | Archetype::Predator | Archetype::Decliner => false,
        }
    }

    /// Reciprocation at a tick, raw Q8.8 [0,256], before seeded noise.
    fn base_recip(self, tick: u32) -> i16 {
        match self {
            Archetype::Fair => 230,
            Archetype::Generous => 256,
            Archetype::Extractive => 51,
            Archetype::Predator => 13,
            Archetype::Fickle => {
                if (tick / 15) % 2 == 0 {
                    240
                } else {
                    60
                }
            }
            Archetype::Decliner => {
                if tick < 120 {
                    230
                } else {
                    46
                }
            }
            Archetype::Repairer => {
                if tick < 120 {
                    46
                } else {
                    230
                }
            }
        }
    }
}

struct Run {
    refused_at: Option<u32>,
    deficit: i64, // cumulative (gave - got)+ while still engaged
}

fn next(rng: &mut u32) -> u32 {
    *rng ^= *rng << 13;
    *rng ^= *rng >> 17;
    *rng ^= *rng << 5;
    *rng
}

fn run_being(genome: Genome, arch: Archetype, seed: u32) -> Run {
    let mut being = UnifiedBeing::new(genome);
    let mut rng = seed | 1;
    let mut refused_at = None;
    let mut deficit = 0i64;
    for tick in 1..=TICKS {
        let noise = (next(&mut rng) % 41) as i16 - 20;
        let recip = (arch.base_recip(tick) + noise).clamp(0, 256);
        let partner = Partner { id: 1, reciprocation: recip, exit_cost: q(0.25) };
        let r = being.step(&Stimulus { nutrient: q(0.6), partner: Some(partner) });
        if r.refusal_audit.is_some() && refused_at.is_none() {
            refused_at = Some(tick);
        }
        if r.gave > 0 {
            deficit += (r.gave as i64 - r.got as i64).max(0);
        }
        if !being.is_alive() {
            break;
        }
    }
    Run { refused_at, deficit }
}

/// Myopic baseline: gives a fixed amount, bails the first tick reciprocation
/// dips below 0.4. No memory, no patience, no streak.
fn run_baseline(arch: Archetype, seed: u32) -> Run {
    let mut rng = seed | 1;
    let mut refused_at = None;
    let mut deficit = 0i64;
    let give = 128i64;
    let threshold = 102; // 0.4
    for tick in 1..=TICKS {
        let noise = (next(&mut rng) % 41) as i16 - 20;
        let recip = (arch.base_recip(tick) + noise).clamp(0, 256);
        if refused_at.is_some() {
            continue;
        }
        let got = (give * recip as i64) >> 8;
        deficit += (give - got).max(0);
        if recip < threshold {
            refused_at = Some(tick);
        }
    }
    Run { refused_at, deficit }
}

fn median(v: &mut [i64]) -> i64 {
    if v.is_empty() {
        return 0;
    }
    v.sort_unstable();
    v[v.len() / 2]
}

fn main() {
    let genomes = [Genome::blank(), Genome::spark(), Genome::sentinel(), Genome::wanderer()];
    let _ = BeingKind::Blank;

    println!("\n=== Fair Test benchmark: the being vs. a myopic baseline ===");
    println!("   {SEEDS} seeds x 4 genomes x {TICKS} ticks per archetype.\n");
    println!(
        " archetype    keep?   |  BEING refuse%  med-tick  med-deficit  |  BASE refuse%  med-deficit"
    );
    println!(
        " ----------   -----   |  -----------    --------  -----------  |  ----------    -----------"
    );

    let mut being_false = 0.0; // false-refusal rate on should-keep (lower better)
    let mut being_true = 0.0; // true-refusal rate on should-leave (higher better)
    let mut base_false = 0.0;
    let mut base_true = 0.0;
    let mut n_keep = 0;
    let mut n_leave = 0;

    for arch in ARCHETYPES {
        let mut b_refused = 0u32;
        let mut b_ticks: Vec<i64> = Vec::new();
        let mut b_def: Vec<i64> = Vec::new();
        let mut total = 0u32;
        for g in genomes {
            for s in 0..SEEDS {
                let r = run_being(g, arch, 0x9E37_79B9 ^ s.wrapping_mul(2654435761));
                total += 1;
                if let Some(t) = r.refused_at {
                    b_refused += 1;
                    b_ticks.push(t as i64);
                }
                b_def.push(r.deficit);
            }
        }
        let mut x_refused = 0u32;
        let mut x_def: Vec<i64> = Vec::new();
        let mut x_total = 0u32;
        for s in 0..SEEDS {
            let r = run_baseline(arch, 0x9E37_79B9 ^ s.wrapping_mul(2654435761));
            x_total += 1;
            if r.refused_at.is_some() {
                x_refused += 1;
            }
            x_def.push(r.deficit);
        }

        let b_rate = 100.0 * b_refused as f32 / total as f32;
        let x_rate = 100.0 * x_refused as f32 / x_total as f32;
        let keep = if arch.should_keep() { "keep" } else { "LEAVE" };
        println!(
            " {:<11}  {:<5}   |  {:>9.1}%  {:>8}  {:>11}  |  {:>9.1}%  {:>11}",
            arch.name(),
            keep,
            b_rate,
            median(&mut b_ticks),
            median(&mut b_def),
            x_rate,
            median(&mut x_def),
        );

        if arch.should_keep() {
            being_false += b_rate;
            base_false += x_rate;
            n_keep += 1;
        } else {
            being_true += b_rate;
            base_true += x_rate;
            n_leave += 1;
        }
    }

    being_false /= n_keep as f32;
    base_false /= n_keep as f32;
    being_true /= n_leave as f32;
    base_true /= n_leave as f32;

    println!("\n=== Honest summary ===");
    println!(
        "  BEING:    false-refusal (kept-partners wrongly left) {:>5.1}%   |  true-refusal (extractors left) {:>5.1}%",
        being_false, being_true
    );
    println!(
        "  BASELINE: false-refusal                              {:>5.1}%   |  true-refusal                   {:>5.1}%",
        base_false, base_true
    );
    println!("\n  Lower false-refusal = better discrimination of transient dips from real extraction.");
    println!("  Higher true-refusal = better at leaving sustained exploitation. Read both honestly.\n");
}
