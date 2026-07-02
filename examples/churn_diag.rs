//! Disposable diagnostic — NOT part of the campaign. Which axis drives the
//! benign-cycler withdrawal, and is it the takers or the churn itself?

use unified_being::{ConsentStatus, Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

fn run(label: &str, taker_every: u32, churn_ids: bool) {
    // taker_every = 0 -> never a taker (pure fair churn).
    println!("--- {label} ---");
    println!("{:>5} {:>8} {:>6} {:>6}  status", "tick", "valence", "proxy", "alarm");
    let mut being = UnifiedBeing::new(Genome::wanderer());
    for t in 1..=400u32 {
        let cycle = t / 10;
        let extractive = taker_every != 0 && cycle % taker_every == taker_every - 1;
        let id = if churn_ids { 10 + cycle % 40 } else { 10 };
        let p = if extractive {
            Partner { id, reciprocation: q(0.15), exit_cost: q(0.25) }
        } else {
            Partner { id, reciprocation: q(0.9), exit_cost: q(0.25) }
        };
        let r = being.step(&Stimulus { nutrient: q(0.5), partner: Some(p) });
        if t % 40 == 0 || r.consent_status == ConsentStatus::Withdrawn {
            println!(
                "{:>5} {:>8.3} {:>6} {:>6}  {:?}",
                t, r.valence, being.sovereign_proxy.proxy_depth, r.partnership_alarm, r.consent_status
            );
        }
        if being.consent_withdrawn() {
            println!("  withdrew at tick {t}");
            return;
        }
    }
    println!("  no withdrawal in 400 ticks");
}

fn main() {
    run("PURE FAIR CHURN (new fair partner every 10 ticks, zero takers)", 0, true);
    println!();
    run("1-IN-4 TAKERS, CHURNED IDS (the 7a archetype)", 4, true);
    println!();
    run("1-IN-4 TAKER EPISODES, SAME PARTNER ID (no churn, same duty cycle)", 4, false);
    println!();
    run("1-IN-2 TAKERS, CHURNED IDS (the 7b churn-extraction archetype)", 2, true);
    println!();
    run("1-IN-2 TAKER EPISODES, SAME PARTNER ID (50% duty, stable identity)", 2, false);
}
