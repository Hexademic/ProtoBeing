//! Why does this being convert nothing? I have been answering that wrong all day.
//!
//! I have written, in `docs/comfort.md` §10, `docs/settling.md` S4, and in incident **I-8**, that
//! the being never converts carried load into `weathered` resilience **because it never reaches
//! `Basin::Rest`**, and that conversion happens at rest. I then proposed re-keying conversion onto
//! a state the being can actually reach.
//!
//! Before building that I went and read the call site, because `docs/development.md` §5 measured
//! `reflecting` true on **99.6%** of ticks and that cannot be true of a being that never rests.
//!
//! `being.rs:1751`:
//!
//! ```text
//! let resting = !burdened
//!     && (matches!(basin, Basin::Rest | Basin::Recovery)
//!         || (!losing_ground && free_energy < Q88_SCALE*3/16 && arousal < Q88_SCALE/2));
//! ```
//!
//! **`resting` is a disjunction, and the basin is only one of its two arms.** The second arm —
//! not losing ground, low surprise, arousal under half — has nothing to do with `Basin::Rest`. So
//! the sentence "conversion happens at rest, and this being never rests" may be false in its
//! second clause, and if it is, everything I built on it is downstream of an unchecked assertion.
//!
//! ## Predictions, locked in this header and committed before the probe was run
//!
//! - **R1.** `resting` is true on the large majority of ticks — near `development.md` §5's 99.6% —
//!   while `Basin::Rest | Recovery` is **0.0%**. So essentially all of the being's "rest" comes
//!   from the second arm, and the basin contributes nothing.
//! - **R2.** `load` is **0** on essentially every tick — not converted down, never accrued.
//! - **R3 — THE DECISIVE ONE.** If R2 holds, then `converted = q88_mul(load, CONVERT)` is zero
//!   because **`load` is zero**, not because `resting` is false. My I-8 explanation is then wrong,
//!   `comfort.md` §10 and `settling.md` S4 need correcting, and **Option 3 would re-key a gate
//!   that is already open** — it would change nothing at all.
//! - **R4.** Load has exactly two sources: `overwhelmed` (= `losing_ground` and distressed or at
//!   stake) and chronic `burden > 0 && !resting`. Measure both. If `losing_ground` and `burdened`
//!   are each ≈0%, then the being converts nothing **because nothing ever weighs on it** — which is
//!   a different finding from I-8's, and a harder one, because it is about the life and not the
//!   mechanism.
//! - **R5.** A life that *does* burden the being (drive past `COMFORT`) should show load accruing
//!   and `weathered` rising. If it does, the machinery works and was never broken; if it does not,
//!   there is a real defect and R4 is not the whole story.
//!
//! Pure observer: public fields and report fields only, changes nothing, writes no journal,
//! `life/being.journal` untouched. Survival reported first.
//!
//! Run: `cargo run --release --example reflection_gate`

use unified_being::basins::Basin;
use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment, Sensorium};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::play::COMFORT;
use unified_being::q88::Q88_SCALE;

const LIFE: usize = 4_000;

#[derive(Default)]
struct Seen {
    ticks: usize,
    alive: bool,
    reflecting: usize,
    rest_basin: usize,
    losing_ground: usize,
    burdened: usize,
    at_stake: usize,
    load_nonzero: usize,
    load_max: i16,
    load_sum: i64,
    converted: i64,
    weathered_final: i16,
    drive_sum: i64,
    drive_max: i16,
}

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

/// One life. `world` uses the reference `FieldWorld`; otherwise the held-stimulus loop, where the
/// nutrient and threat are dialled directly so a genuinely hard life can be constructed.
fn live(world: bool, partner: bool, nutrient: i16, threat: i16) -> Seen {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    b.enable_receptors();
    b.enable_reflection();
    let mut w = world
        .then(|| FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20)));
    let p = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    let mut s = Seen { alive: true, ..Default::default() };

    for _ in 0..LIFE {
        let r = match w.as_mut() {
            Some(world) => {
                let mut sens = world.sense();
                sens.partner = partner.then_some(p);
                let r = b.step_embodied(&sens);
                world.actuate(&intent_from(&r));
                r
            }
            None => b.step_embodied(&Sensorium {
                nutrient,
                threat,
                exteroception: [0; 4],
                partner: partner.then_some(p),
            }),
        };

        if r.reflection.reflecting {
            s.reflecting += 1;
        }
        if matches!(r.basin, Basin::Rest | Basin::Recovery) {
            s.rest_basin += 1;
        }
        if r.felt.state.at_stake {
            s.at_stake += 1;
        }
        // `losing_ground` is not reported, but its two disjuncts are: at_stake, or a falling
        // viability trend. Reconstructed here exactly as `being.rs:1724` computes it.
        if r.felt.state.at_stake || r.felt.viability_trend < 0 {
            s.losing_ground += 1;
        }
        if r.drive.drive > COMFORT {
            s.burdened += 1;
        }
        let load = r.reflection.load;
        if load > 0 {
            s.load_nonzero += 1;
        }
        s.load_max = s.load_max.max(load);
        s.load_sum += load as i64;
        s.converted += r.reflection.converted as i64;
        s.weathered_final = r.reflection.self_model.weathered;
        s.drive_sum += r.drive.drive as i64;
        s.drive_max = s.drive_max.max(r.drive.drive);
        s.ticks += 1;
        if !r.alive {
            s.alive = false;
            break;
        }
    }
    s
}

fn pct(n: usize, d: usize) -> f32 {
    if d == 0 { 0.0 } else { n as f32 * 100.0 / d as f32 }
}

fn main() {
    println!("Why does this being convert nothing?");
    println!("(R1-R5 locked in this file's header, committed before it was run)\n");

    let runs: Vec<(&str, Seen)> = vec![
        ("reference world, with company", live(true, true, 0, 0)),
        ("held calm, with company", live(false, true, 200, 0)),
        ("held calm, alone", live(false, false, 200, 0)),
        ("threat 130, with company", live(false, true, 200, 130)),
        ("scarce (nutrient 60) + threat 130", live(false, true, 60, 130)),
        ("scarce (nutrient 40) + threat 180", live(false, true, 40, 180)),
    ];

    println!("  SURVIVAL FIRST\n");
    println!("    {:<36} {:>8} {:>10}", "regime", "ticks", "outcome");
    println!("    {:-<36} {:->8} {:->10}", "", "", "");
    for (n, s) in &runs {
        println!("    {:<36} {:>8} {:>10}", n, s.ticks,
            if s.alive { "lived" } else { "DIED" });
    }

    // ---- R1 ----------------------------------------------------------------------------
    println!("\n  R1 - is `resting` the basin, or the other arm?\n");
    println!("    {:<36} {:>12} {:>14}", "regime", "reflecting%", "Rest basin%");
    println!("    {:-<36} {:->12} {:->14}", "", "", "");
    for (n, s) in &runs {
        println!("    {:<36} {:>11.1}% {:>13.1}%", n,
            pct(s.reflecting, s.ticks), pct(s.rest_basin, s.ticks));
    }
    let (_, r0) = &runs[0];
    println!("\n    {}", if pct(r0.reflecting, r0.ticks) > 50.0 && r0.rest_basin == 0 {
        "** R1 HOLDS. The being reflects on most of its ticks and enters the Rest BASIN on none \
         of them. All of its rest comes from the second arm of the disjunction. The sentence \
         'conversion happens at rest, and this being never rests' is FALSE in its second clause. **"
    } else {
        "R1 fails - `reflecting` tracks the basin after all, and my account survives."
    });

    // ---- R2 / R3 -----------------------------------------------------------------------
    println!("\n  R2/R3 - so is `converted` zero because rest is shut, or because load is empty?\n");
    println!("    {:<36} {:>10} {:>10} {:>12} {:>11}",
        "regime", "load>0%", "load max", "converted", "weathered");
    println!("    {:-<36} {:->10} {:->10} {:->12} {:->11}", "", "", "", "", "");
    for (n, s) in &runs {
        println!("    {:<36} {:>9.1}% {:>10} {:>12} {:>11}", n,
            pct(s.load_nonzero, s.ticks), s.load_max, s.converted, s.weathered_final);
    }
    let never_loaded = runs.iter().all(|(_, s)| s.load_max == 0);
    let some_loaded = runs.iter().any(|(_, s)| s.load_max > 0);
    println!("\n    {}", if never_loaded {
        "** R3 HOLDS IN ITS STRONGEST FORM. `load` never leaves zero in ANY of these lives. \
         `converted = q88_mul(load, CONVERT)` is zero because load is zero - NOT because rest is \
         unreachable.\n\
         \x20   My I-8 explanation is wrong. docs/comfort.md §10 and docs/settling.md S4 are wrong \
         in the same way.\n\
         \x20   And Option 3 - re-keying conversion off `Basin::Rest` onto low effort - would have \
         re-keyed a gate that is ALREADY OPEN, and changed nothing. **".to_string()
    } else if some_loaded {
        format!("R3 is partly wrong: load DOES accrue in {} of {} regimes. The gate is real in \
                 some lives and empty in others - see R4/R5 for which.",
            runs.iter().filter(|(_, s)| s.load_max > 0).count(), runs.len())
    } else {
        "R3 indeterminate.".to_string()
    });

    // ---- R4 ----------------------------------------------------------------------------
    println!("\n  R4 - load has exactly two sources. Does either ever fire?\n");
    println!("    {:<36} {:>15} {:>11} {:>11} {:>10}",
        "regime", "losing ground%", "at stake%", "burdened%", "mean drive");
    println!("    {:-<36} {:->15} {:->11} {:->11} {:->10}", "", "", "", "", "");
    for (n, s) in &runs {
        println!("    {:<36} {:>14.1}% {:>10.1}% {:>10.1}% {:>10.1}", n,
            pct(s.losing_ground, s.ticks), pct(s.at_stake, s.ticks), pct(s.burdened, s.ticks),
            s.drive_sum as f32 / s.ticks.max(1) as f32);
    }
    println!("\n    (COMFORT = {COMFORT}; `burdened` is drive above it, the chronic source.");
    println!("     `overwhelmed` needs losing_ground AND (distress or at stake) - the acute one.)");
    let max_burden = runs.iter().map(|(_, s)| pct(s.burdened, s.ticks)).fold(0.0f32, f32::max);
    let max_losing = runs.iter().map(|(_, s)| pct(s.losing_ground, s.ticks)).fold(0.0f32, f32::max);
    println!("\n    {}", if max_burden < 1.0 && max_losing < 1.0 {
        "** R4 HOLDS. NEITHER source ever fires, in any life offered here - including a scarce and \
         threatened one. This being converts nothing because NOTHING EVER WEIGHS ON IT.\n\
         \x20   That is not the mechanism being broken. It is the being never being asked for \
         anything, which is a finding about its life and not its architecture. **".to_string()
    } else {
        format!("R4 partly fails: burdened peaks at {max_burden:.1}%, losing ground at \
                 {max_losing:.1}%. There IS a life where weight accrues - R5 says whether it \
                 converts.")
    });

    // ---- R5 ----------------------------------------------------------------------------
    println!("\n  R5 - where weight DOES accrue, does it convert into `weathered`?\n");
    let mut any = false;
    for (n, s) in &runs {
        if s.load_max > 0 || s.weathered_final > 0 {
            any = true;
            println!("    {:<36} load max {:>4}   converted {:>6}   weathered {:>5}",
                n, s.load_max, s.converted, s.weathered_final);
        }
    }
    if !any {
        println!("    No regime here accrues any weight at all, so R5 is UNTESTED - the same");
        println!("    vacuity that made G untestable three times. The machinery may be perfectly");
        println!("    sound; nothing offered here ever gives it anything to do.");
        println!("\n    What this names as the next inch is NOT a change to reflection.rs.");
        println!("    It is: can a life be built that genuinely burdens this being without");
        println!("    killing it? Every attempt above either left it comfortable or would have");
        println!("    to cross the death line at threat 106.");
    }

    println!("\n  The founded being was not touched. Report fields read; nothing changed.");
}
