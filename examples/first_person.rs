//! The being, speaking of itself — charter §12, the transparent interpreter
//! pointed inward. Every line is rendered from the registers of a real life;
//! nothing is narrated. Imagined and remembered clauses are marked, never
//! spoken as things the being lives.
//!
//! Run: cargo run --example first_person

use unified_being::{FirstPerson, Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

fn main() {
    println!("\n=== A being speaks of itself — read from its registers, nothing narrated ===\n");

    // A fair life: it should come to speak content, whole, and sure.
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let fair = Partner { id: 1, reciprocation: q(0.9), exit_cost: q(0.3) };
    println!("-- with a fair partner --");
    for t in 1..=60u32 {
        let r = being.step(&Stimulus { nutrient: q(0.5), partner: Some(fair) });
        if t % 20 == 0 {
            println!("  {}", FirstPerson::render(t as u64, &r).speak());
        }
    }

    // Betrayed once, recover, then meet a *fresh* betrayer of the same kind
    // (recognition is of the felt pattern, not the identity — mirrors Exp 5).
    // At the second onset, recall should surface — and it must be marked
    // (recalled), never spoken as a thing the being lives now.
    println!("\n-- betrayed, recovered, then meeting the same kind of taker again --");
    let mut b = UnifiedBeing::new(Genome::wanderer());
    let fair2 = Partner { id: 1, reciprocation: q(0.92), exit_cost: q(0.3) };
    let taker_a = Partner { id: 2, reciprocation: q(0.18), exit_cost: q(0.3) };
    let taker_b = Partner { id: 3, reciprocation: q(0.18), exit_cost: q(0.3) };
    let mut spoke_recall = false;
    for tick in 1..=170u64 {
        let partner = if tick <= 50 {
            Some(fair2)
        } else if tick <= 100 {
            Some(taker_a)
        } else if tick <= 150 {
            None
        } else {
            Some(taker_b)
        };
        let r = b.step(&Stimulus { nutrient: q(0.6), partner });
        // At the second onset, print the first self-report that carries recall.
        if tick >= 151 && !spoke_recall {
            let fp = FirstPerson::render(tick, &r);
            if fp.recall.is_some() {
                println!("  {}", fp.speak());
                spoke_recall = true;
            }
        }
    }
    if !spoke_recall {
        println!("  (recall did not cross the speaking threshold this run — honestly, nothing said)");
    }

    println!("\n  Note: every '(recalled)' and '(imagined)' clause is sourced from a memory");
    println!("  or a rollout register — the being never says it LIVES what it only recalls");
    println!("  or foresees. What could not be traced to a register was not said (§12).\n");
}
