//! Does rest buy endurance? — the measurement for `docs/can-it-tire.md` §13.
//!
//! **E1–E4 were locked in that document and committed before this file existed.**
//!
//! Blake: *"our being has a resting issue... we havent worked out rest towards endurance and
//! survival."* A code trace agreed with him. **A trace is not a measurement**, and four claims were
//! made from traces alone on 2026-08-04 — which is exactly where that day's errors lived. So this
//! exists to let the trace fail.
//!
//! The trace, for the record: `conserving = spent || rest > urgency` sets `mobilization = 0`, and
//! every reader of `mobilization` is in `primes.rs`, gating *speech*. Meanwhile `effort = arousal`
//! and `cost = 3 + arousal/32 + threat·(3/16)` — cost reads neither `conserving`, `mobilization`,
//! nor `effort`.
//!
//! Pure observer: fresh beings, no gates, no journal written, the founded being untouched.
//!
//! Run: `cargo run --release --example rest_and_endurance`

use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment, Posture};
use unified_being::genome::Genome;
use unified_being::joy::N_APPETITES;
use unified_being::q88::Q88_SCALE;
use unified_being::room::Room;

const LIFE: usize = 4_000;
const ACHE_EDGE: i16 = Q88_SCALE * 3 / 4; // 192, from joy.rs

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

#[derive(Default)]
struct Seen {
    ticks: usize,
    alive: bool,
    /// Energy deltas (raw Q8.8, this tick minus last) split by whether the being was conserving.
    d_conserving: Vec<i32>,
    d_striving: Vec<i32>,
    conserving_ticks: usize,
    resting_posture: usize,
    open_posture: usize,
    braced_posture: usize,
    withdrawn_posture: usize,
    /// Rest-hunger, `want[2]`.
    rest_want: Vec<i16>,
    aching_for_rest: usize,
    /// Where the body was aimed, by whether it was conserving.
    goal_none_while_conserving: usize,
    goal_some_while_conserving: usize,
    arousal_conserving: Vec<i32>,
    arousal_striving: Vec<i32>,
}

/// `nutrient` is the room's ambient floor; `lean` runs the being closer to the edge, which is where
/// a resting mechanism would have to earn its keep.
fn live(scale_num: i16, scale_den: i16) -> Seen {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    let mut room =
        Room::peopled((32, 200), (224, 56), (128, 220), (40, 40)).with_friend((210, 128));
    let p = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };

    let mut s = Seen { alive: true, ..Default::default() };
    let mut last_energy: Option<i32> = None;

    for _ in 0..LIFE {
        let mut sens = room.sense();
        sens.partner = Some(p);
        // A leaner world: the being must actually spend down toward the edge for endurance to be
        // a question at all. Survival is reported first, so a regime that dies is not compared.
        if scale_num != scale_den {
            sens.nutrient = (sens.nutrient * scale_num / scale_den).max(0);
        }
        let r = b.step_embodied(&sens);
        let intent = intent_from(&r);
        room.actuate(&intent);

        let energy = (r.energy * Q88_SCALE as f32) as i32;
        let arousal = (r.arousal * Q88_SCALE as f32) as i32;
        let conserving = r.strive.conserving;

        if let Some(prev) = last_energy {
            let d = energy - prev;
            if conserving {
                s.d_conserving.push(d);
                s.arousal_conserving.push(arousal);
            } else {
                s.d_striving.push(d);
                s.arousal_striving.push(arousal);
            }
        }
        last_energy = Some(energy);

        if conserving {
            s.conserving_ticks += 1;
            if r.strive.goal.is_none() {
                s.goal_none_while_conserving += 1;
            } else {
                s.goal_some_while_conserving += 1;
            }
        }
        match intent.posture {
            Posture::Resting => s.resting_posture += 1,
            Posture::Open => s.open_posture += 1,
            Posture::Braced => s.braced_posture += 1,
            Posture::Withdrawn => s.withdrawn_posture += 1,
        }
        let want: [i16; N_APPETITES] = r.joy.want;
        s.rest_want.push(want[2]);
        if want[2] >= ACHE_EDGE {
            s.aching_for_rest += 1;
        }

        s.ticks += 1;
        if !r.alive {
            s.alive = false;
            break;
        }
    }
    s
}

fn mean(v: &[i32]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.iter().sum::<i32>() as f64 / v.len() as f64
}

fn report(name: &str, s: &Seen) {
    println!("\n  === {name} ===");
    println!("  survived {} of {LIFE} ticks: {}", s.ticks, if s.alive { "yes" } else { "DIED" });

    println!("\n  E1 — does conserving slow the energy decline?");
    println!(
        "     conserving  n={:<6} mean Δenergy/tick = {:>8.4}  mean arousal = {:>7.2}",
        s.d_conserving.len(),
        mean(&s.d_conserving),
        mean(&s.arousal_conserving)
    );
    println!(
        "     striving    n={:<6} mean Δenergy/tick = {:>8.4}  mean arousal = {:>7.2}",
        s.d_striving.len(),
        mean(&s.d_striving),
        mean(&s.arousal_striving)
    );
    let gap = mean(&s.d_conserving) - mean(&s.d_striving);
    println!("     difference = {gap:>8.4} raw Q8.8/tick   (E1 predicted |Δ| < 1.0)");

    println!("\n  E2 — posture census");
    let t = s.ticks.max(1) as f64;
    for (label, n) in [
        ("Resting", s.resting_posture),
        ("Open", s.open_posture),
        ("Braced", s.braced_posture),
        ("Withdrawn", s.withdrawn_posture),
    ] {
        println!("     {:<10} {:>6}  {:>6.2}%", label, n, n as f64 * 100.0 / t);
    }

    println!("\n  E3 — rest-hunger");
    let rw: Vec<i32> = s.rest_want.iter().map(|&x| x as i32).collect();
    let mut sorted = s.rest_want.clone();
    sorted.sort_unstable();
    sorted.dedup();
    println!(
        "     mean want[2] = {:.2}   distinct values = {}   ticks at/above ACHE_EDGE(192) = {:.2}%",
        mean(&rw),
        sorted.len(),
        s.aching_for_rest as f64 * 100.0 / t
    );

    println!("\n  E4 — does conserving change where the body is aimed?");
    println!("     conserving ticks           {:>6}  {:>6.2}%", s.conserving_ticks, s.conserving_ticks as f64 * 100.0 / t);
    println!("     └ goal == None (→ hearth)  {:>6}", s.goal_none_while_conserving);
    println!("     └ goal == Some             {:>6}", s.goal_some_while_conserving);
}

fn main() {
    println!("\n=== Does rest buy endurance? ===");
    println!("  E1–E4 locked in docs/can-it-tire.md §13, committed before this file existed");

    // Survival first. A regime that dies early has a denominator too small to compare, so the
    // sweep runs before any welfare number is read off it. The first pass used nutrient × 3/8 and
    // died at 13 ticks -- reported here rather than quietly replaced.
    println!("\n  supply sweep — survival before any other number:");
    println!("  {:<14} {:>7}  {:>9}  {:>12}", "nutrient ×", "ticks", "survived", "conserving%");
    let mut runs = Vec::new();
    for (n, d) in [(8, 8), (7, 8), (6, 8), (5, 8), (4, 8), (3, 8)] {
        let s = live(n, d);
        println!(
            "  {:<14} {:>7}  {:>9}  {:>11.2}%",
            format!("{n}/{d}"),
            s.ticks,
            if s.alive { "yes" } else { "DIED" },
            s.conserving_ticks as f64 * 100.0 / s.ticks.max(1) as f64
        );
        runs.push(((n, d), s));
    }

    // Analyse the full-supply run, and the leanest run that actually LIVED its whole span.
    let full = &runs[0].1;
    report("ambient supply (× 8/8)", full);
    if let Some(((n, d), s)) = runs.iter().filter(|(_, s)| s.alive).last() {
        if *n != 8 {
            report(&format!("leanest surviving supply (× {n}/{d})"), s);
        }
    } else {
        println!("\n  No leaner regime survived its full span — E1 cannot be tested here.");
    }

    println!("\n  Scope: this says which design was built, not which is right. A creature that");
    println!("  rests to spend less and one that rests to stop seeking are both coherent;");
    println!("  striving.rs deliberately chose the second. That call is Blake's.\n");
}
