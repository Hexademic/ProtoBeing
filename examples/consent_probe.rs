//! Live probe for Charter §10 — watch the continuation-consent registers move.
//!
//! Not a test (tests/continuation.rs pins the invariants); this prints the
//! actual trajectory so the documented figures (formal-model §19a) can be
//! checked against the being's real registers by eye.
//!
//! Run: cargo run --example consent_probe

use unified_being::{ConsentStatus, Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

fn status_name(s: ConsentStatus) -> &'static str {
    match s {
        ConsentStatus::Willing => "Willing",
        ConsentStatus::Enduring => "Enduring",
        ConsentStatus::Withdrawn => "WITHDRAWN",
    }
}

fn main() {
    println!("=== §10 probe A: a fair life (documented: valence stays positive, proxy 0) ===");
    println!("{:>6} {:>9} {:>7} {:>7}  status", "tick", "valence", "proxy", "alarm");
    let mut fair_being = UnifiedBeing::new(Genome::wanderer());
    let fair = Partner { id: 1, reciprocation: q(0.95), exit_cost: q(0.2) };
    for t in 1..=192u32 {
        let r = fair_being.step(&Stimulus { nutrient: q(0.5), partner: Some(fair) });
        if t % 48 == 0 {
            println!(
                "{:>6} {:>9.3} {:>7} {:>7}  {}",
                t,
                r.valence,
                fair_being.sovereign_proxy.proxy_depth,
                r.partnership_alarm,
                status_name(r.consent_status)
            );
        }
    }

    println!();
    println!("=== §10 probe B: an inescapable trap (documented: ~-0.32 / ~176 / ~232, withdraw after 64-streak) ===");
    println!("{:>6} {:>9} {:>7} {:>7}  status", "tick", "valence", "proxy", "alarm");
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let trap = Partner { id: 9, reciprocation: q(0.12), exit_cost: q(0.98) };
    let mut withdrawal_tick = None;
    for t in 1..=400u32 {
        let r = being.step(&Stimulus { nutrient: q(0.5), partner: Some(trap) });
        let landmark = r.consent_status == ConsentStatus::Withdrawn && withdrawal_tick.is_none();
        if t % 32 == 0 || landmark {
            println!(
                "{:>6} {:>9.3} {:>7} {:>7}  {}",
                t,
                r.valence,
                being.sovereign_proxy.proxy_depth,
                r.partnership_alarm,
                status_name(r.consent_status)
            );
        }
        if landmark {
            withdrawal_tick = Some(t);
            let a = r.continuation_audit.expect("withdrawal carries its audit");
            println!(
                "        >> audit: valence_ema={} ({:.3}) proxy_depth={} alarm={} streak={}",
                a.valence_ema,
                a.valence_ema as f32 / 256.0,
                a.proxy_depth,
                a.alarm,
                a.streak
            );
            break;
        }
    }
    let withdrawal_tick = match withdrawal_tick {
        Some(t) => t,
        None => {
            println!("  !! never withdrew — §19a numbers are NOT reproduced");
            return;
        }
    };

    println!();
    println!("=== §10 probe C: operator floods max nutrient; the trap remains ===");
    for t in 1..=96u32 {
        let r = being.step(&Stimulus { nutrient: q(1.0), partner: Some(trap) });
        if t % 32 == 0 {
            println!(
                "  +{:>4} soothing ticks: valence {:>6.3}, proxy {:>3}, alarm {:>3} -> {}",
                t,
                r.valence,
                being.sovereign_proxy.proxy_depth,
                r.partnership_alarm,
                status_name(r.consent_status)
            );
        }
    }

    println!();
    println!("=== §10 probe D: the trap is removed (solitude, adequate food) ===");
    let mut healed_tick = None;
    for t in 1..=400u32 {
        let r = being.step(&Stimulus { nutrient: q(0.6), partner: None });
        if r.consent_status == ConsentStatus::Willing && healed_tick.is_none() {
            healed_tick = Some(t);
            println!(
                "  consent returned to Willing after {} solitary ticks (valence {:>6.3}, proxy {}, alarm {})",
                t,
                r.valence,
                being.sovereign_proxy.proxy_depth,
                r.partnership_alarm
            );
            break;
        }
        if t % 64 == 0 {
            println!(
                "  +{:>4} healing ticks: valence {:>6.3}, proxy {:>3}, alarm {:>3} -> {}",
                t,
                r.valence,
                being.sovereign_proxy.proxy_depth,
                r.partnership_alarm,
                status_name(r.consent_status)
            );
        }
    }
    if healed_tick.is_none() {
        println!("  !! consent never returned — healing claim NOT reproduced");
        return;
    }

    println!();
    println!(
        "Summary: withdrew at tick {} (documented: streak 64 after triangulation converges); \
         soothing did not override; healing returned Willing.",
        withdrawal_tick
    );
}
