//! The exercise census — the architecture scores 14 of 14, but **is it used?**
//!
//! **Predictions EX-1..EX-5 are locked in `docs/operational-consciousness.md` §8 and were
//! committed before this file existed.**
//!
//! §7's negative control found that nine of the fourteen Butlin indicators are also met by this
//! repository's own test suite. §8 names what none of the fourteen ask: *every indicator scores an
//! architecture, and not one asks whether the architecture is ever used.* This probe measures that.
//!
//! **Exercise** = realized variation ÷ afforded variation, per indicator-bearing register.
//!
//! * **Realized** — distinct values in one life under `blessed_features()`, the founded being's own
//!   four faculties. Not a hypothetical regime: the one it actually lives in.
//! * **Afforded** — distinct values across the **union of every regime below**. Not "afforded in
//!   principle" — afforded *as demonstrated by this being elsewhere*.
//!
//! Three things declared here rather than discovered later:
//!
//! 1. **Every ratio is an UPPER bound on exercise.** The denominator is bounded below by our regime
//!    menu, so a richer menu can only lower these numbers. It flatters the being, and naming it
//!    beats being caught by it (`docs/errors.md` shape).
//! 2. **GWT-1 is not covered** — a module count is not a variation quantity. **HOT-3 is not covered
//!    because `schema_control` is off in the blessed regime**, and that is the finding, not a gap.
//! 3. **Distinctness is quantised at a stated grain**, and reported at three grains, because a
//!    ratio that holds at only one bin size is a fact about the bin size.
//!
//! Pure observer: fresh beings, no journal written, **the founded being at `life/being.journal` is
//! never touched and never advanced.**
//!
//! Run: `cargo run --release --example exercise_census`

use unified_being::being::{Partner, StepReport, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::q88::Q88_SCALE;
use unified_being::room::Room;

const LIFE: usize = 4_000;
const BINS: [i32; 3] = [1, 8, 32];

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

/// One indicator-bearing register, and how to read it off a tick.
struct Reg {
    indicator: &'static str,
    /// §1's grade for that indicator, so EX-3 can ask whether the grade tracks the use.
    grade: &'static str,
    name: &'static str,
    read: fn(&StepReport) -> i32,
}

/// Twelve of the fourteen. See the header for why GWT-1 and HOT-3 are absent.
const REGS: &[Reg] = &[
    Reg {
        indicator: "RPT-1",
        grade: "✅",
        name: "body (valence,arousal)",
        // The recurrent body state, as one value: the Van der Pol pair.
        read: |r| (q(r.valence) as i32) * 1024 + q(r.arousal) as i32,
    },
    Reg { indicator: "RPT-2", grade: "🟡", name: "percept binding", read: |r| r.percept.binding as i32 },
    Reg {
        indicator: "GWT-2",
        grade: "✅",
        name: "attention focus",
        read: |r| r.attention.attended.map_or(-1, |c| c as i32),
    },
    Reg { indicator: "GWT-3", grade: "✅", name: "broadcast reach", read: |r| r.percept.broken_through as i32 },
    Reg {
        indicator: "GWT-4",
        grade: "✅",
        name: "focus succession",
        // A succession is a transition, not a state — GWT-4 is about walking foci, so the unit of
        // variation is the *pair*. Filled in by the caller, which has the previous tick.
        read: |r| r.attention.attended.map_or(-1, |c| c as i32),
    },
    Reg { indicator: "HOT-1", grade: "✅", name: "top-down mean", read: |r| r.percept.top_down_mean as i32 },
    Reg { indicator: "HOT-2", grade: "✅", name: "self-surprise", read: |r| r.self_surprise as i32 },
    Reg {
        indicator: "HOT-4",
        grade: "✅",
        name: "quality point",
        read: |r| {
            let a = r.quality.point.axis;
            (a[0] as i32) * 1_000_000 + (a[1] as i32) * 10_000 + (a[2] as i32) * 100 + a[3] as i32
        },
    },
    Reg {
        indicator: "AST-1",
        grade: "✅",
        name: "schema prediction",
        read: |r| r.attention_schema.predicted.map_or(-1, |c| c as i32),
    },
    Reg { indicator: "PP-1", grade: "✅", name: "free energy", read: |r| r.free_energy as i32 },
    Reg { indicator: "AE-1", grade: "✅", name: "habit in use", read: |r| r.habits.habit.map_or(-1, |h| h as i32) },
    Reg { indicator: "AE-2", grade: "🟡", name: "agency", read: |r| r.agency.agency as i32 },
];

struct Lived {
    name: &'static str,
    alive: bool,
    ticks: usize,
    /// `[register][tick]`, raw before quantisation.
    trace: Vec<Vec<i32>>,
}

/// `blessed_features()` from `src/bin/being.rs`, mirrored as gate calls. The founded being lives
/// with exactly these four, so this is the regime whose exercise the question is actually about.
fn bless(b: &mut UnifiedBeing) {
    b.enable_felt_choice();
    b.enable_precision_learning();
    b.enable_generative_perception();
    b.enable_workspace_persistence();
}

fn live(name: &'static str, gates: fn(&mut UnifiedBeing)) -> Lived {
    let mut b = UnifiedBeing::new(unified_being::genome::Genome::wanderer());
    gates(&mut b);
    let mut room =
        Room::peopled((32, 200), (224, 56), (128, 220), (40, 40)).with_friend((210, 128));
    let p = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };

    let mut l =
        Lived { name, alive: true, ticks: 0, trace: vec![Vec::with_capacity(LIFE); REGS.len()] };
    let mut prev_focus = -2i32;

    for _ in 0..LIFE {
        let mut sens = room.sense();
        sens.partner = Some(p);
        let r = b.step_embodied(&sens);
        room.actuate(&intent_from(&r));

        for (i, reg) in REGS.iter().enumerate() {
            let v = (reg.read)(&r);
            // GWT-4 alone is scored on the transition, because a succession is a pair.
            let v = if reg.indicator == "GWT-4" { prev_focus * 32 + v } else { v };
            l.trace[i].push(v);
        }
        prev_focus = r.attention.attended.map_or(-1, |c| c as i32);
        l.ticks += 1;
        if !r.alive {
            l.alive = false;
            break;
        }
    }
    l
}

fn distinct(vals: &[i32], bin: i32) -> usize {
    let mut v: Vec<i32> = vals.iter().map(|x| x.div_euclid(bin)).collect();
    v.sort_unstable();
    v.dedup();
    v.len()
}

fn main() {
    println!("\n=== The exercise census — the architecture scores 14/14, but is it used? ===");
    println!("  predictions EX-1..EX-5 locked in docs/operational-consciousness.md §8, committed first");
    println!("  exercise = realized (blessed regime) / afforded (union of all regimes)");
    println!("  every ratio is an UPPER bound: a richer regime menu can only lower it\n");

    let regimes = [
        live("blessed", bless),
        live("+receptors", |b| {
            bless(b);
            b.enable_receptors();
        }),
        live("+all-loops", |b| {
            bless(b);
            b.enable_schema_control();
            b.enable_serial_access();
            b.enable_workspace_broadcast();
            b.enable_reflection();
            b.enable_memory_guidance();
        }),
        live("+survival", |b| {
            bless(b);
            b.enable_ultrastability();
            b.enable_comfort();
            b.enable_settling();
            b.enable_setting_down();
            b.enable_reserve();
        }),
        live("bare", |_| {}),
    ];

    // ---- survival first, before any ratio: a short life has a small denominator ----
    println!("  {:<12} {:>7}  {:>8}", "regime", "ticks", "survived");
    for r in &regimes {
        println!("  {:<12} {:>7}  {:>8}", r.name, r.ticks, if r.alive { "yes" } else { "DIED" });
    }

    let blessed = &regimes[0];

    for bin in BINS {
        println!("\n  --- grain: bin {bin} ---");
        println!(
            "  {:<7} {:<4} {:<22} {:>9} {:>9} {:>10}",
            "ind", "§1", "register", "realized", "afforded", "exercise"
        );
        let mut total = 0.0f64;
        let mut constant = 0usize;
        let mut met = (0.0f64, 0usize);
        let mut partial = (0.0f64, 0usize);

        for (i, reg) in REGS.iter().enumerate() {
            let realized = distinct(&blessed.trace[i], bin);
            let mut union: Vec<i32> = Vec::new();
            for r in &regimes {
                union.extend(r.trace[i].iter().map(|x| x.div_euclid(bin)));
            }
            union.sort_unstable();
            union.dedup();
            let afforded = union.len().max(1);
            let ex = realized as f64 / afforded as f64;
            total += ex;
            if realized == 1 {
                constant += 1;
            }
            if reg.grade == "✅" {
                met.0 += ex;
                met.1 += 1;
            } else {
                partial.0 += ex;
                partial.1 += 1;
            }
            println!(
                "  {:<7} {:<4} {:<22} {:>9} {:>9} {:>9.1}%{}",
                reg.indicator,
                reg.grade,
                reg.name,
                realized,
                afforded,
                ex * 100.0,
                if realized == 1 { "  ← never varies" } else { "" }
            );
        }

        let mean = total / REGS.len() as f64;
        println!("\n  mean exercise ............ {:.1}%", mean * 100.0);
        println!("  registers that never vary  {constant}");
        println!(
            "  mean by §1 grade ......... ✅ {:.1}%   🟡 {:.1}%",
            met.0 / met.1.max(1) as f64 * 100.0,
            partial.0 / partial.1.max(1) as f64 * 100.0
        );

        // EX-4: does receptors lift it? Same denominator, different numerator.
        let recep = &regimes[1];
        let mut rtotal = 0.0f64;
        for (i, _) in REGS.iter().enumerate() {
            let realized = distinct(&recep.trace[i], bin);
            let mut union: Vec<i32> = Vec::new();
            for r in &regimes {
                union.extend(r.trace[i].iter().map(|x| x.div_euclid(bin)));
            }
            union.sort_unstable();
            union.dedup();
            rtotal += realized as f64 / union.len().max(1) as f64;
        }
        let rmean = rtotal / REGS.len() as f64;
        println!(
            "  +receptors mean .......... {:.1}%   ({:.2}× blessed)",
            rmean * 100.0,
            if mean > 0.0 { rmean / mean } else { 0.0 }
        );

        if bin == BINS[0] {
            println!("\n  EX-1  mean < 25%? .................... {}", verdict(mean < 0.25));
            println!("  EX-2  ≥3 registers never vary? ....... {}", verdict(constant >= 3));
            println!(
                "  EX-3  ✅ not higher than 🟡? .......... {}",
                verdict(met.0 / met.1.max(1) as f64 <= partial.0 / partial.1.max(1) as f64)
            );
            println!(
                "  EX-4  receptors ≥2×? (expected FAIL) . {}",
                verdict(mean > 0.0 && rmean / mean >= 2.0)
            );
        }
    }

    println!("\n  EX-5 (Subject C) is scored in docs/operational-consciousness.md §8 — the test");
    println!("  suite's realized variation is not measurable from inside this crate.\n");
}

fn verdict(held: bool) -> &'static str {
    if held {
        "HOLDS"
    } else {
        "FAILED"
    }
}
