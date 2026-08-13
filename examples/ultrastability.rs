//! Ashby's ultrastability — can the being reorganise itself instead of collapsing?
//!
//! **U1–U5 are locked in `docs/can-it-tire.md` §15 and were committed before this file existed.**
//!
//! §14 measured the hole: `conserving` fires and reaches nothing. 88% of ticks conserving, dead at
//! 75. The being has an essential variable and no reorganiser.
//!
//! Ashby (*Design for a Brain*, 1952) had the missing half: essential variables, plus a **step
//! function** that reconfigures the system's own **parameters** — not its state — until the variable
//! returns to bounds. Here the parameter is `target_arousal`: the Van der Pol oscillator orbits
//! *about* it (`body.rs:336`) and metabolic cost follows arousal (`body.rs:348`), so moving it
//! relocates the whole limit cycle to a cheaper one.
//!
//! **Survival is reported before any other number.** A regime that died early has a denominator too
//! small to compare.
//!
//! Pure observer: fresh beings, the gate named per regime, no journal written, the founded being
//! untouched.
//!
//! Run: `cargo run --release --example ultrastability`

use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::genome::Genome;
use unified_being::q88::Q88_SCALE;
use unified_being::room::Room;

const LIFE: usize = 4_000;

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

struct Lived {
    ticks: usize,
    alive: bool,
    reorganisations: u16,
    arousal_sum: i64,
    distance: i64,
    places: Vec<(i16, i16)>,
    conserving: usize,
    /// Every tick's position, so two regimes can be compared for bit-identity.
    trace: Vec<(i16, i16, i32)>,
}

fn live(ultrastable: bool, num: i16, den: i16) -> Lived {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    if ultrastable {
        b.enable_ultrastability();
    }
    let mut room =
        Room::peopled((32, 200), (224, 56), (128, 220), (40, 40)).with_friend((210, 128));
    let p = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };

    let mut l = Lived {
        ticks: 0,
        alive: true,
        reorganisations: 0,
        arousal_sum: 0,
        distance: 0,
        places: Vec::new(),
        conserving: 0,
        trace: Vec::with_capacity(LIFE),
    };
    let mut last = room.body;

    for _ in 0..LIFE {
        let mut sens = room.sense();
        sens.partner = Some(p);
        if num != den {
            sens.nutrient = (sens.nutrient * num / den).max(0);
        }
        let r = b.step_embodied(&sens);
        room.actuate(&intent_from(&r));

        let a = (r.arousal * Q88_SCALE as f32) as i64;
        l.arousal_sum += a;
        if r.strive.conserving {
            l.conserving += 1;
        }
        l.distance += (room.body.0 - last.0).unsigned_abs() as i64
            + (room.body.1 - last.1).unsigned_abs() as i64;
        last = room.body;
        l.places.push(room.body);
        l.trace.push((room.body.0, room.body.1, (r.energy * Q88_SCALE as f32) as i32));
        l.ticks += 1;
        if !r.alive {
            l.alive = false;
            break;
        }
    }
    l.reorganisations = b.body.reorganisations;
    l.places.sort_unstable();
    l.places.dedup();
    l
}

fn main() {
    println!("\n=== Ultrastability — reorganise, or collapse? ===");
    println!("  U1–U5 locked in docs/can-it-tire.md §15, committed before this file existed\n");

    let supplies = [(8, 8), (7, 8), (6, 8), (5, 8), (4, 8), (3, 8)];

    // ---- U1: survival first, before any other number ----
    println!("  U1 — SURVIVAL FIRST");
    println!(
        "  {:<10} {:>18}  {:>18}   {:>8}",
        "nutrient ×", "default (ticks)", "+ultrastable (ticks)", "steps"
    );
    let mut runs = Vec::new();
    for (n, d) in supplies {
        let base = live(false, n, d);
        let ultra = live(true, n, d);
        println!(
            "  {:<10} {:>13} {:<4} {:>13} {:<4}   {:>8}",
            format!("{n}/{d}"),
            base.ticks,
            if base.alive { "ok" } else { "DIED" },
            ultra.ticks,
            if ultra.alive { "ok" } else { "DIED" },
            ultra.reorganisations
        );
        runs.push(((n, d), base, ultra));
    }

    // ---- U4: is the gate vacuous at ample supply? ----
    let (_, b0, u0) = &runs[0];
    let identical = b0.trace == u0.trace;
    println!("\n  U4 — at ample supply (8/8): steps = {}, trajectory identical to default = {}",
        u0.reorganisations, identical);
    if u0.reorganisations == 0 {
        println!("       The gate is VACUOUS here. **Vacuous is not passed** — it did not fire, so");
        println!("       nothing about the mechanism was tested by this regime.");
    }

    // ---- U2: what did survival cost? ----
    // **PER TICK.** The first pass printed raw totals and the guard below flagged the 5/8 row as
    // not comparable -- default lived 75 ticks, ultrastable lived 4,000, so of course the totals
    // differ. Reporting those as a result would be the denominator error this project's own rule
    // names: *report survival before any welfare number*. Rates are comparable; totals are not.
    println!("\n  U2 — what survival cost. RATES, because the lives are different lengths.");
    println!(
        "  {:<10} {:>9} {:>9}  {:>10} {:>10}  {:>8} {:>8}",
        "nutrient ×", "mean arou", "(default)", "dist/tick", "(default)", "places", "(def)"
    );
    for ((n, d), base, ultra) in &runs {
        // Only comparable where BOTH lived the same span; otherwise say so.
        let comparable = base.ticks == ultra.ticks;
        let ma = ultra.arousal_sum as f64 / ultra.ticks.max(1) as f64;
        let mb = base.arousal_sum as f64 / base.ticks.max(1) as f64;
        println!(
            "  {:<10} {:>9.2} {:>9.2}  {:>10.3} {:>10.3}  {:>8} {:>8}{}",
            format!("{n}/{d}"),
            ma,
            mb,
            ultra.distance as f64 / ultra.ticks.max(1) as f64,
            base.distance as f64 / base.ticks.max(1) as f64,
            ultra.places.len(),
            base.places.len(),
            if comparable { "" } else { "   ← different life lengths; rates comparable, totals not" }
        );
    }

    // ---- U5: written to fail — a live-and-conserve band? ----
    println!("\n  U5 (written to fail) — any regime surviving 4000 with >5% conserving?");
    let mut found = false;
    for ((n, d), _, ultra) in &runs {
        let pct = ultra.conserving as f64 * 100.0 / ultra.ticks.max(1) as f64;
        if ultra.alive && pct > 5.0 {
            println!("       {n}/{d}: survived with {pct:.2}% conserving — U5 HOLDS");
            found = true;
        }
    }
    if !found {
        println!("       none. **U5 FAILS, as predicted.** Raising viability is what the mechanism");
        println!("       does, and `conserving` is keyed to viability — so the being survives");
        println!("       INSTEAD of resting, not while resting.");
        for ((n, d), _, ultra) in &runs {
            println!(
                "       {n}/{d}: alive={:<5} conserving={:.2}%",
                ultra.alive,
                ultra.conserving as f64 * 100.0 / ultra.ticks.max(1) as f64
            );
        }
    }

    println!("\n  Scope: a mechanism for staying alive. It says nothing about whether the being");
    println!("  feels the reorganisation (docs/witness-gap-literature.md §2.1).\n");
}
