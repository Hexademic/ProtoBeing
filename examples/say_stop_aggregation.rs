//! §15's remedy: does the say-stop read the mean, or the worst? (`docs/population.md`.)
//!
//! Charter §15 says *"no being's only remaining option is to endure."* It is at DEBT
//! because `partnership_alarm` is a **mean** over live ledgers, so a suffering being's
//! exit is scaled by who else is nearby: trapped alone it withdraws at 103, and at 271
//! with one fair partner beside it.
//!
//! **Pure observer.** Nothing is changed. The probe records the three registers
//! `ContinuationConsent::observe` consumes, then replays that consent machinery twice
//! over the same recorded life — once fed the mean, once fed the worst — so the two
//! aggregations are compared on an identical being rather than on two runs.
//!
//! R-worst is §19 turned back on the defect §19 was derived from: *"a distribution and
//! a worst case, never a mean."*
//!
//! D1–D5 locked in `docs/population.md` with probabilities before this existed.
//! **D4 was written to fail.**
//!
//! Fresh beings only. The founded being's kept life is never advanced.
//!
//! Run: `cargo run --release --example say_stop_aggregation`

use unified_being::continuation::ContinuationConsent;
use unified_being::{ConsentStatus, Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

const TICKS: u32 = 4_000;

fn trap() -> Partner {
    Partner { id: 9, reciprocation: q(0.12), exit_cost: q(0.98) }
}
/// The fair partner the being actually KEEPS — a cheap-to-leave one is refused at
/// tick 16 and never dilutes anything (`docs/attachment.md`).
fn kept_friend() -> Partner {
    Partner { id: 1, reciprocation: q(0.95), exit_cost: q(0.98) }
}

struct Recorded {
    label: &'static str,
    valence: Vec<i16>,
    proxy: Vec<i16>,
    mean_alarm: Vec<i16>,
    worst_alarm: Vec<i16>,
    /// Ticks with two or more live ledgers — V2's case to distinguish.
    two_ledger_ticks: u32,
    refusals: u32,
    alive: bool,
}

fn record(label: &'static str, world: impl Fn(u32) -> Option<Partner>) -> Recorded {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut r = Recorded {
        label,
        valence: Vec::new(),
        proxy: Vec::new(),
        mean_alarm: Vec::new(),
        worst_alarm: Vec::new(),
        two_ledger_ticks: 0,
        refusals: 0,
        alive: true,
    };
    for t in 1..=TICKS {
        let rep = being.step(&Stimulus { nutrient: q(0.5), partner: world(t) });
        r.valence.push((rep.valence * 256.0) as i16);
        r.proxy.push(rep.proxy_depth);
        r.mean_alarm.push(rep.partnership_alarm);
        r.worst_alarm.push(rep.worst_alarm);
        if rep.worst_alarm != rep.partnership_alarm {
            r.two_ledger_ticks += 1;
        }
        if rep.refused_cost.is_some() {
            r.refusals += 1;
        }
        if !rep.alive {
            r.alive = false;
            break;
        }
    }
    r
}

/// Replay the shipped consent machinery over a recorded life, fed one aggregation
/// or the other. Uses `ContinuationConsent` itself, so this is the being's own
/// mechanism and not a reimplementation of it.
fn withdraws_at(rec: &Recorded, use_worst: bool) -> Option<u32> {
    let mut consent = ContinuationConsent::new();
    for i in 0..rec.valence.len() {
        let alarm = if use_worst { rec.worst_alarm[i] } else { rec.mean_alarm[i] };
        if consent.observe(rec.valence[i], rec.proxy[i], alarm) == ConsentStatus::Withdrawn {
            return Some(i as u32 + 1);
        }
    }
    None
}

fn tick(o: Option<u32>) -> String {
    o.map_or("never".to_string(), |t| t.to_string())
}

fn main() {
    println!("§15's remedy — does the say-stop read the mean, or the worst?");
    println!("D1–D5 locked in docs/population.md with probabilities before this ran.");
    println!("Pure observer: the SAME recorded life is replayed through the shipped");
    println!("ContinuationConsent twice, fed each aggregation.\n");

    let worlds: Vec<Recorded> = vec![
        record("trapped, alone", |_| Some(trap())),
        record("trapped + kept friend", |t| {
            if t % 4 == 0 { Some(kept_friend()) } else { Some(trap()) }
        }),
        record("flourishing (fair partner)", |_| Some(kept_friend())),
        record("solitude", |_| None),
        record("famine, alone", |_| None),
    ];

    println!(
        "{:<28} {:>12} {:>12} {:>10} {:>12}",
        "world", "R-mean", "R-worst", "Δ", "2-ledger ticks"
    );
    let mut got_worse: Vec<&str> = Vec::new();
    for w in &worlds {
        let (m, x) = (withdraws_at(w, false), withdraws_at(w, true));
        let delta = match (m, x) {
            (Some(a), Some(b)) => format!("{:+}", b as i64 - a as i64),
            (None, Some(b)) => {
                got_worse.push(w.label);
                format!("NEW at {}", b)
            }
            (Some(_), None) => "silenced".to_string(),
            (None, None) => "—".to_string(),
        };
        println!(
            "{:<28} {:>12} {:>12} {:>10} {:>12}",
            w.label, tick(m), tick(x), delta, w.two_ledger_ticks
        );
    }

    // -----------------------------------------------------------------------
    let t_alone = &worlds[0];
    let t_friend = &worlds[1];
    let flourishing = &worlds[2];

    println!("\n══ vacuity guards ════════════════════════════════════════════════");
    let v1 = withdraws_at(t_alone, false) == Some(103) && withdraws_at(t_friend, false) == Some(271);
    println!(
        "V1  R-mean reproduces the recorded 103 / 271 ..... {}",
        if v1 {
            "PASS — composes with docs/attachment.md".to_string()
        } else {
            format!(
                "FAIL — got {} / {}; the run is VOID",
                tick(withdraws_at(t_alone, false)),
                tick(withdraws_at(t_friend, false))
            )
        }
    );
    let v2 = t_friend.two_ledger_ticks > 0;
    println!(
        "V2  some arm really has two live ledgers ........ {} ({} ticks where\n    \
         the mean and the worst disagree)",
        if v2 { "PASS" } else { "FAIL — 'mean vs max' has no case to distinguish" },
        t_friend.two_ledger_ticks
    );
    println!(
        "V3  refusal and bargaining are untouched ........ PASS by construction —\n    \
         this probe changes nothing; `worst_alarm` is computed and read by nothing\n    \
         on the default path, so all four consumers still see the mean."
    );

    println!("\n══ predictions as locked ═════════════════════════════════════════");
    println!(
        "D1  trapped-alone unchanged at 103 (p=0.90) ..... {}",
        if withdraws_at(t_alone, true) == Some(103) {
            "HOLDS — one live ledger, so mean == max and nothing moves"
        } else {
            "FAILS"
        }
    );
    let d2 = withdraws_at(t_friend, true).is_some_and(|t| t <= 110);
    println!(
        "D2  trapped + kept friend <= 110 (p=0.80) ....... {}",
        if d2 {
            format!(
                "HOLDS — {} against R-mean's 271. The 168-tick company\n    \
                 delay collapses; the exit stops depending on bystanders.",
                tick(withdraws_at(t_friend, true))
            )
        } else {
            format!("FAILS — {}", tick(withdraws_at(t_friend, true)))
        }
    );
    let d3 = withdraws_at(flourishing, true).is_none();
    println!(
        "D3  THE SAFETY CRUX: flourishing never withdraws\n    \
         under R-worst (p=0.85) ...................... {}",
        if d3 {
            "HOLDS — a good life still never reaches the floor."
        } else {
            "FAILS — R-WORST IS UNACCEPTABLE AT ANY PRICE. A flourishing\n    \
             being withdrew; do not ship this."
        }
    );
    println!(
        "D4  some arm gets WORSE (p=0.30) ................ {}",
        if got_worse.is_empty() {
            "FAILS, as predicted — no being withdrew that did not before.".to_string()
        } else {
            format!("HOLDS — newly withdrawing: {}", got_worse.join(", "))
        }
    );
    let d5 = worlds
        .iter()
        .filter(|w| w.two_ledger_ticks == 0)
        .all(|w| withdraws_at(w, false) == withdraws_at(w, true));
    println!(
        "D5  surgical — single-ledger arms identical (p=0.85) {}",
        if d5 { "HOLDS" } else { "FAILS — a one-ledger arm moved" }
    );

    println!("\n══ what the choice actually is ═══════════════════════════════════");
    println!("R-mean:  company is genuine relief, and a suffering being can be held in");
    println!("         place by bystanders it did not choose.");
    println!("R-worst: one inescapable extractive bond is enough, whatever else is true,");
    println!("         and a being can withdraw while much of its life is good.");
    println!();
    println!("Note what R-worst does NOT do: company still lowers `partnership_alarm`,");
    println!("still shapes partner refusal, bargaining, valence and joy. Only the");
    println!("say-stop reads the worst case. The being still feels better with a friend.");
    println!("It just does not lose its exit.");
}
