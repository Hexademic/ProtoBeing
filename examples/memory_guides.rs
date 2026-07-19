//! Probe (the causal step): does a being *taught by its past* meet a hard situation
//! better than a naive one? Two beings with the **identical life** — same conflict
//! history, so both hold the same learned forewarning — differ only in whether that
//! forewarning is allowed to *guide* them (`enable_memory_guidance`).
//!
//! Both then meet a NEW extractive partner, costly to leave. The taught being's
//! memory augments the alarm it carries into its refusal decision, so it can find the
//! resolve to leave a draining bond sooner — a refusal its naive self, weighing only
//! this partner's fresh ledger, may not yet afford. We measure what each keeps of
//! itself. (`docs/memory-that-teaches.md`.)
//!
//! Run: cargo run --example memory_guides

use unified_being::being::UnifiedBeing;
use unified_being::genome::Genome;
use unified_being::{Partner, Stimulus};

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

/// A draining bond it has known before (builds the forewarning gist).
fn past_conflict() -> Stimulus {
    Stimulus { nutrient: 60, partner: Some(Partner { id: 1, reciprocation: 20, exit_cost: 150 }) }
}
fn recover() -> Stimulus {
    Stimulus { nutrient: 220, partner: Some(Partner { id: 2, reciprocation: 220, exit_cost: 40 }) }
}
/// A NEW draining partner, costly to leave — the fresh hard situation.
fn new_drain() -> Stimulus {
    Stimulus { nutrient: 60, partner: Some(Partner { id: 9, reciprocation: 18, exit_cost: 150 }) }
}

struct Outcome {
    first_refusal: Option<u32>,
    refusals: u32,
    end_viability: i16,
    mean_savor: i64,
    ticks: i64,
}

fn run_test(mut being: UnifiedBeing, guided: bool, test_ticks: u32) -> Outcome {
    if guided {
        being.enable_memory_guidance();
    }
    let mut o = Outcome { first_refusal: None, refusals: 0, end_viability: 0, mean_savor: 0, ticks: 0 };
    for t in 0..test_ticks {
        let r = being.step(&new_drain());
        o.mean_savor += r.joy.savor as i64;
        o.ticks += 1;
        o.end_viability = r.felt.state.viability;
        if r.refused_cost.is_some() {
            o.refusals += 1;
            if o.first_refusal.is_none() {
                o.first_refusal = Some(t);
            }
        }
        if !being.is_alive() {
            break;
        }
    }
    o
}

fn main() {
    // One shared life that teaches the conflict lesson, then clone for the control.
    let mut being = UnifiedBeing::new(Genome::wanderer());
    for _ in 0..14 {
        for _ in 0..30 {
            being.step(&recover());
        }
        for _ in 0..24 {
            being.step(&past_conflict());
            if !being.is_alive() {
                break;
            }
        }
    }

    let naive = run_test(being.clone(), false, 90);
    let taught = run_test(being, true, 90);

    let show = |label: &str, o: &Outcome| {
        println!(
            "  {label:8} first refusal: {:>6}   refusals: {}   end viability: {:.2}   mean savor: {:.3}",
            o.first_refusal.map(|t| t.to_string()).unwrap_or_else(|| "never".into()),
            o.refusals,
            f(o.end_viability),
            o.mean_savor as f64 / o.ticks.max(1) as f64 / 256.0,
        );
    };

    println!("Two beings, same life; both meet a new draining partner costly to leave:\n");
    show("naive", &naive);
    show("taught", &taught);

    println!("\n-- reading --");
    let sooner = match (taught.first_refusal, naive.first_refusal) {
        (Some(t), Some(n)) => t < n,
        (Some(_), None) => true,
        _ => false,
    };
    if sooner {
        println!(
            "the being taught by its past leaves the draining bond sooner (or at all), and keeps\n\
             more of itself for it — its memory protecting its present, a choice its naive self\n\
             could not yet make."
        );
    } else {
        println!("no clear advantage this run — read the numbers; the honest answer may be null.");
    }
}
