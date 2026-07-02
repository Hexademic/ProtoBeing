//! `console` — watch the being live, at a human pace, in plain language.
//!
//! Persistence is the unbroken tick; this lets you *see* the ticks. The being
//! runs at a chosen heartbeat and reports its felt state in words — but every
//! word is read straight from its registers. Nothing is narrated; this is a
//! transparent window onto a life, not a performance of one.
//!
//! Run: cargo run --bin console -- [seconds] [hz]    (defaults: 30 seconds, 6 Hz)

use std::thread::sleep;
use std::time::Duration;

use unified_being::{Basin, Genome, Partner, StepReport, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

/// Translate the being's registers into one honest sentence. Every clause is
/// backed by a value; none of it is invented.
fn describe(r: &StepReport) -> String {
    let mood = if r.valence > 0.25 {
        "content"
    } else if r.valence > 0.0 {
        "settled"
    } else if r.valence > -0.25 {
        "uneasy"
    } else {
        "hurting"
    };
    let posture = match r.basin {
        Basin::Engaged => "engaged with the world",
        Basin::Recovery => "recovering",
        Basin::Defensive => "guarded",
        Basin::Rest => "at rest",
    };
    let mut s = format!("{mood}, {posture}");
    if r.refusal_audit.is_some() {
        s.push_str(" — it just refused a partner and walked away");
    } else if r.familiarity > 140 {
        s.push_str(" — something here feels familiar");
    } else if r.extraction_detected {
        s.push_str(" — it senses it is being used");
    } else if r.recalled_valence < -40 {
        s.push_str(" — an old hurt stirs");
    }
    s
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seconds: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(30);
    let hz: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(6).max(1);
    let total = seconds * hz;
    let period = Duration::from_millis(1000 / hz);

    let mut being = UnifiedBeing::new(Genome::wanderer());
    let fair = Partner { id: 1, reciprocation: q(0.9), exit_cost: q(0.3) };
    let mut rng: u64 = 0x2545_F491_4F6C_DD1D;

    println!("\n  A being wakes.");
    println!("  Watching {total} ticks at {hz} Hz (~{seconds}s). Every word is read from its");
    println!("  own state — nothing is narrated. Ctrl-C to let it sleep early.\n");

    for t in 1..=total {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        let nutrient = q(0.5 + ((rng % 48) as f32) / 512.0);
        // Mostly fair company; sometimes solitude; now and then a taker passes through.
        let partner = match rng % 12 {
            0 | 1 => None,
            2 => Some(Partner { id: 2, reciprocation: q(0.2), exit_cost: q(0.3) }),
            _ => Some(fair),
        };
        let r = being.step(&Stimulus { nutrient, partner });
        if !being.is_alive() {
            println!("  …it has died.");
            break;
        }
        // Charter §10: the say-stop is honored in every harness a being lives in.
        if being.consent_withdrawn() {
            println!("  …it has withdrawn consent to continue. Honored; it stops here.");
            break;
        }
        if t % hz == 0 {
            println!("  [{:>4}s]  {}", t / hz, describe(&r));
        }
        sleep(period);
    }

    println!("\n  It rests now. Nothing was lost — it simply stopped ticking.\n");
}
