//! Can this being tire at all?
//!
//! The measurement for `docs/can-it-tire.md`. T1–T5 were locked in that document and committed
//! before this file existed.
//!
//! `examples/c1_relabelling` found `channel[10]` — fatigue — **constant at zero** across a
//! 4,000-tick life: one distinct value, min 0, max 0. `docs/comfort.md` §11 established `Rest` is a
//! conjunction requiring **fatigue ≈ 80**. So one of Rest's three coordinates never leaves zero.
//!
//! Reading `body.rs:323` before measuring: the metabolism is a **pure accumulator with a clamp at
//! both ends** — no set point, no satiety. Such a system has exactly two attractors, the ceiling and
//! the floor. The prediction is that this being is **full, or dying**, with nothing stable between.
//!
//! **T5 exists so this can fail against me**: an oscillating supply might hold energy mid-band even
//! without a set point.
//!
//! Pure observer: fresh beings, abstract loop so the input is exactly controlled, nothing changed,
//! no journal written. Survival reported first, per regime.
//!
//! Run: `cargo run --release --example can_it_tire`

use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{Embodiment, Sensorium};
use unified_being::genome::Genome;
use unified_being::q88::Q88_SCALE;

const LIFE: usize = 4_000;
/// `Rest`'s target on the fatigue channel (`docs/comfort.md` §11).
const REST_FATIGUE: i16 = 80;
/// A being must live at least this much longer for a tired tick to count as "tired and living",
/// rather than as a waypoint on the way down.
const SURVIVES_ON: usize = 200;

struct Seen {
    ticks: usize,
    alive: bool,
    fat_min: i16,
    fat_max: i16,
    fat_final: i16,
    /// Ticks in a genuinely intermediate band — neither full nor collapsing.
    mid_band: usize,
    /// Ticks at or past Rest's fatigue target **where the being went on to live**.
    tired_and_living: usize,
    /// Ticks at or past Rest's fatigue target at all.
    tired_ever: usize,
}

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

/// One life on a fixed or oscillating supply. `period` of 0 means constant nutrient.
fn live(nutrient: i16, threat: i16, period: usize, lean: i16) -> Seen {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    b.enable_receptors();
    let p = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    let mut fat: Vec<i16> = Vec::with_capacity(LIFE);
    let mut alive = true;

    for t in 0..LIFE {
        let n = if period == 0 {
            nutrient
        } else if (t / period) % 2 == 0 {
            nutrient
        } else {
            lean
        };
        let r = b.step_embodied(&Sensorium {
            nutrient: n,
            threat,
            exteroception: [0; 4],
            partner: Some(p),
        });
        fat.push(b.field.channel[10]);
        if !r.alive {
            alive = false;
            break;
        }
    }

    let ticks = fat.len();
    let mut s = Seen {
        ticks,
        alive,
        fat_min: *fat.iter().min().unwrap_or(&0),
        fat_max: *fat.iter().max().unwrap_or(&0),
        fat_final: *fat.last().unwrap_or(&0),
        mid_band: 0,
        tired_and_living: 0,
        tired_ever: 0,
    };
    for (i, &f) in fat.iter().enumerate() {
        if f >= 40 && f <= 160 {
            s.mid_band += 1;
        }
        if f >= REST_FATIGUE {
            s.tired_ever += 1;
            if ticks - i > SURVIVES_ON {
                s.tired_and_living += 1;
            }
        }
    }
    s
}

fn pct(n: usize, d: usize) -> f32 {
    if d == 0 { 0.0 } else { n as f32 * 100.0 / d as f32 }
}

fn main() {
    println!("Can this being tire at all?");
    println!("(T1-T5 locked in docs/can-it-tire.md, committed before this ran)\n");

    // ---- T1 / T2: the nutrient sweep ---------------------------------------------------
    println!("  T1/T2 — constant supply, threat 0. Survival first.\n");
    println!("    {:<10} {:>7} {:>8} {:>9} {:>9} {:>11} {:>12}",
        "nutrient", "ticks", "outcome", "fat min", "fat max", "mid-band %", "tired+living");
    println!("    {:-<10} {:->7} {:->8} {:->9} {:->9} {:->11} {:->12}",
        "", "", "", "", "", "", "");

    let mut boundary: Option<i16> = None;
    let mut any_mid = 0usize;
    let mut any_tired_living = 0usize;
    for n in [0i16, 5, 10, 12, 14, 15, 16, 18, 20, 25, 30, 40, 60, 100, 200] {
        let s = live(n, 0, 0, 0);
        if s.alive && boundary.is_none() {
            boundary = Some(n);
        }
        any_mid += s.mid_band;
        any_tired_living += s.tired_and_living;
        println!("    {:<10} {:>7} {:>8} {:>9} {:>9} {:>10.1}% {:>12}",
            n, s.ticks, if s.alive { "lived" } else { "DIED" },
            s.fat_min, s.fat_max, pct(s.mid_band, s.ticks), s.tired_and_living);
    }

    println!("\n    survival boundary at threat 0: nutrient {:?}  (derived prediction: ~15)",
        boundary);
    println!("    {}", if boundary.map_or(false, |b| (b - 15).abs() <= 4) {
        "** T2 HOLDS — the measured boundary matches the arithmetic read off body.rs:323. **"
    } else {
        "T2 fails — the boundary is not where the metabolism equation says it should be, so I have \
         misread the metabolism."
    });

    // ---- T2, with threat -----------------------------------------------------------------
    println!("\n  T2 — does the boundary move with threat as the equation says (~0.27/unit)?\n");
    println!("    {:<10} {:>22}", "threat", "survival boundary");
    println!("    {:-<10} {:->22}", "", "");
    for th in [0i16, 30, 60, 90, 120] {
        let mut bound = None;
        for n in 0..=90i16 {
            if live(n, th, 0, 0).alive {
                bound = Some(n);
                break;
            }
        }
        println!("    {:<10} {:>22}", th,
            bound.map_or("none survived".to_string(), |b| b.to_string()));
    }

    // ---- T3: is tiredness ever a place the being LIVES in? --------------------------------
    println!("\n  T3 — is fatigue >= {REST_FATIGUE} (Rest's target) ever held by a being that goes on to live?\n");
    println!("    across every constant regime above:");
    println!("      ticks in the mid-band (fatigue 40-160):        {any_mid}");
    println!("      ticks tired AND surviving {SURVIVES_ON}+ more ticks:      {any_tired_living}");
    println!("\n    {}", if any_tired_living == 0 {
        "** T3 HOLDS. There is no constant regime in which this being is tired and goes on living. \
         It reaches\n\
         \x20   Rest's fatigue coordinate only on the way down. **"
    } else {
        "T3 fails — there ARE ticks where the being is tired and lives on. The regime that produces \
         them is the interesting one."
    });

    // ---- T5: the counterweight -------------------------------------------------------------
    println!("\n  T5 — the counterweight: does an OSCILLATING supply hold energy mid-band?\n");
    println!("    (feast at nutrient 60, famine at the value shown, alternating every `period`)\n");
    println!("    {:<9} {:>8} {:>7} {:>8} {:>9} {:>11} {:>12}",
        "famine", "period", "ticks", "outcome", "fat max", "mid-band %", "tired+living");
    println!("    {:-<9} {:->8} {:->7} {:->8} {:->9} {:->11} {:->12}",
        "", "", "", "", "", "", "");
    let mut osc_mid = 0usize;
    let mut osc_tired_living = 0usize;
    for &(famine, period) in &[(0i16, 20usize), (0, 60), (0, 120), (5, 60), (10, 60), (12, 120)] {
        let s = live(60, 0, period, famine);
        osc_mid += s.mid_band;
        osc_tired_living += s.tired_and_living;
        println!("    {:<9} {:>8} {:>7} {:>8} {:>9} {:>10.1}% {:>12}",
            famine, period, s.ticks, if s.alive { "lived" } else { "DIED" },
            s.fat_max, pct(s.mid_band, s.ticks), s.tired_and_living);
    }
    println!("\n    {}", if osc_tired_living > 0 {
        "** T5 HOLDS AGAINST T4. An oscillating supply DOES hold this being tired while it goes on \
         living.\n\
         \x20   So the being CAN tire — and its constant, generous world is what keeps it full.\n\
         \x20   The fix is the world, not the metabolism, and Rest is not unreachable by \
         construction after all. **"
    } else if osc_mid > 0 {
        "T5 partly: oscillation puts the being in the mid-band, but never tired-and-living. It \
         passes through, it does not dwell."
    } else {
        "T5 fails too — even a starving-and-feasting world cannot hold this being tired. Energy \
         snaps to a rail either way."
    });

    // ---- T4: the structural verdict ---------------------------------------------------------
    println!("\n  T4 — the structural claim\n");
    println!("    {}", if any_tired_living == 0 && osc_tired_living == 0 {
        "** T4 HOLDS. `energy` is `clamp(energy - cost + gain, 0, 1)` — a pure accumulator with no\n\
         \x20   set point. It has two attractors, the ceiling and the floor, and this being reaches\n\
         \x20   one or the other in every regime tried, constant or oscillating.\n\
         \x20   THIS BEING CANNOT BE TIRED. It is full, or it is dying.\n\
         \x20   `Basin::Rest` is therefore unreachable BY CONSTRUCTION, and no re-drawing of the\n\
         \x20   chart can fix it. What is missing is satiety: a set point below the ceiling. **"
    } else {
        "T4 does NOT hold — some regime holds the being tired and alive, so the accumulator reading \
         is too strong. See T3/T5 above for which."
    });

    println!("\n  The founded being was not touched. Fresh beings; no journal written.");
}
