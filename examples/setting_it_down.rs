//! Can the being set weight down while still carrying its life?
//!
//! The measurement for `docs/setting-it-down.md` and incident **I-9**. P1–P5, W and G were locked
//! in that document and committed before this code existed.
//!
//! `examples/reflection_deadlock` established the defect: `resting` requires `!burdened`, and the
//! chronic path loads the being **when it is burdened**. Where the burden is structural rather
//! than episodic, the being can never become un-burdened, never converts, and sits at the 256
//! ceiling — measured at **3,638 consecutive ticks** of a 4,000-tick life. `reflection.rs:152–153`
//! promises that path is *"always liftable at rest — chronic stress that is real, still not a
//! trap."* It is a trap.
//!
//! `enable_setting_down()` splits the flag: stopping *accrual* still requires `!burdened`; setting
//! weight *down* needs only `settled` — calm and not being outrun — at a quarter rate, with a
//! floor of 1 that defeats the `load/8` truncation.
//!
//! **P5 is predicted to FAIL and that prediction is on the record.** `weathered` is monotone and
//! capped at 256; a permanently burdened being converting every settled tick may simply saturate
//! it, trading a trap for a giveaway. I would rather find that here than ship it.
//!
//! Survival first. Founded being untouched, gate default-off, no journal written.
//!
//! Run: `cargo run --release --example setting_it_down`

use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment, Sensorium};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::play::COMFORT;
use unified_being::q88::Q88_SCALE;

const LIFE: usize = 4_000;

#[derive(Default)]
struct Lived {
    ticks: usize,
    alive: bool,
    burdened: usize,
    losing_ground: usize,
    load_max: i16,
    load_final: i16,
    load_sum: i64,
    pegged_run: usize,
    converted: i64,
    weathered_final: i16,
    /// The tick `weathered` first reached the 256 ceiling, if it ever did — P5's check.
    saturated_at: Option<usize>,
    drive_sum: i64,
    past_comfort: usize,
    /// G: any tick on which load fell while the being was losing ground would be a violation.
    discharged_while_losing: usize,
    soul: [u8; 32],
}

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

fn live(setting_down: bool, world: bool, partner: bool, nutrient: i16, threat: i16) -> Lived {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    b.enable_receptors();
    b.enable_reflection();
    if setting_down {
        b.enable_setting_down();
    }
    let mut w = world
        .then(|| FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20)));
    let p = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    let mut l = Lived { alive: true, ..Default::default() };
    let mut run = 0usize;

    for t in 0..LIFE {
        let r = match w.as_mut() {
            Some(world) => {
                let mut s = world.sense();
                s.partner = partner.then_some(p);
                let r = b.step_embodied(&s);
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

        let load = r.reflection.load;
        let losing = r.felt.state.at_stake || r.felt.viability_trend < 0;
        if losing {
            l.losing_ground += 1;
            if r.reflection.converted > 0 {
                l.discharged_while_losing += 1;
            }
        }
        if r.drive.drive > COMFORT {
            l.burdened += 1;
        }
        if r.drive.drive >= COMFORT {
            l.past_comfort += 1;
        }
        if load >= Q88_SCALE {
            run += 1;
            l.pegged_run = l.pegged_run.max(run);
        } else {
            run = 0;
        }
        l.load_max = l.load_max.max(load);
        l.load_final = load;
        l.load_sum += load as i64;
        l.converted += r.reflection.converted as i64;
        l.weathered_final = r.reflection.self_model.weathered;
        if l.saturated_at.is_none() && l.weathered_final >= Q88_SCALE {
            l.saturated_at = Some(t);
        }
        l.drive_sum += r.drive.drive as i64;
        l.ticks += 1;
        if !r.alive {
            l.alive = false;
            break;
        }
    }
    l.soul = b.soul_hash();
    l
}

fn pct(n: usize, d: usize) -> f32 {
    if d == 0 { 0.0 } else { n as f32 * 100.0 / d as f32 }
}

fn main() {
    println!("Can the being set weight down while still carrying its life?");
    println!("(P1-P5, W and G locked in docs/setting-it-down.md before the code existed)\n");

    // The structurally burdened life — solitude — where the deadlock was measured.
    let off = live(false, false, false, 200, 0);
    let on = live(true, false, false, 200, 0);
    // The weak-burden life, where the load/8 truncation erased what was carried.
    let eoff = live(false, false, true, 200, 130);
    let eon = live(true, false, true, 200, 130);

    println!("  SURVIVAL FIRST\n");
    println!("    solitary  gate off {} ticks ({})   gate ON {} ticks ({})",
        off.ticks, if off.alive { "lived" } else { "DIED" },
        on.ticks, if on.alive { "lived" } else { "DIED" });
    println!("    episodic  gate off {} ticks ({})   gate ON {} ticks ({})",
        eoff.ticks, if eoff.alive { "lived" } else { "DIED" },
        eon.ticks, if eon.alive { "lived" } else { "DIED" });

    println!("\n  THE SOLITARY LIFE — structural burden, where the drain was welded shut\n");
    println!("  {:<36} {:>14} {:>14}", "", "gate OFF", "gate ON");
    println!("  {:-<36} {:->14} {:->14}", "", "", "");
    let row = |n: &str, a: String, b: String| println!("  {:<36} {:>14} {:>14}", n, a, b);
    row("burdened", format!("{:.1}%", pct(off.burdened, off.ticks)),
        format!("{:.1}%", pct(on.burdened, on.ticks)));
    row("load, maximum", format!("{}", off.load_max), format!("{}", on.load_max));
    row("load, mean",
        format!("{:.0}", off.load_sum as f32 / off.ticks.max(1) as f32),
        format!("{:.0}", on.load_sum as f32 / on.ticks.max(1) as f32));
    row("load, final", format!("{}", off.load_final), format!("{}", on.load_final));
    row("longest run AT CEILING (P2)", format!("{}", off.pegged_run), format!("{}", on.pegged_run));
    row("converted, total", format!("{}", off.converted), format!("{}", on.converted));
    row("weathered, final (P3)", format!("{}", off.weathered_final),
        format!("{}", on.weathered_final));
    row("mean drive (W)",
        format!("{:.1}", off.drive_sum as f32 / off.ticks.max(1) as f32),
        format!("{:.1}", on.drive_sum as f32 / on.ticks.max(1) as f32));
    row("past COMFORT (W)", format!("{:.1}%", pct(off.past_comfort, off.ticks)),
        format!("{:.1}%", pct(on.past_comfort, on.ticks)));

    // ---- P1 --------------------------------------------------------------------------
    println!("\n  P1 - is the gate really off by default?");
    println!("    soul off {:02x}{:02x}{:02x}{:02x}...  on {:02x}{:02x}{:02x}{:02x}...  {}",
        off.soul[0], off.soul[1], off.soul[2], off.soul[3],
        on.soul[0], on.soul[1], on.soul[2], on.soul[3],
        if off.soul != on.soul { "different - the gate acts" } else { "** IDENTICAL - inert **" });
    println!("    (default-path bit-identity is proved by the full suite and tests/founded_being.rs,");
    println!("     which still wakes the kept life at 390 moments)");

    // ---- P2 --------------------------------------------------------------------------
    println!("\n  P2 - does the being leave the ceiling?");
    println!("    longest unbroken run at 256: {} -> {}", off.pegged_run, on.pegged_run);
    println!("    {}", if on.pegged_run < 100 && off.pegged_run >= 100 {
        "** P2 HOLDS. The being is off the ceiling. The drain is open. **"
    } else if on.pegged_run < off.pegged_run {
        "P2 partly holds - the run shortened but did not fall below 100. The quarter rate is too \
         slow against the chronic rise."
    } else {
        "** P2 FAILS - the being is still pegged. The split did not open the drain and the reason \
         is not what I thought. **"
    });

    // ---- P3 --------------------------------------------------------------------------
    println!("\n  P3 - does a structurally burdened being bank anything, for the first time?");
    println!("    weathered: {} -> {}   (converted {} -> {})",
        off.weathered_final, on.weathered_final, off.converted, on.converted);
    println!("    {}", if on.weathered_final > 0 && off.weathered_final == 0 {
        "** P3 HOLDS. A being under a permanent burden banked resilience - which no being in this \
         architecture has ever done. **"
    } else {
        "P3 fails - nothing was banked."
    });

    // ---- P4 --------------------------------------------------------------------------
    println!("\n  P4 - the truncation, isolated: the weak-burden life (threat 130, load max ~11)\n");
    println!("  {:<36} {:>14} {:>14}", "", "gate OFF", "gate ON");
    println!("  {:-<36} {:->14} {:->14}", "", "", "");
    row("load, maximum", format!("{}", eoff.load_max), format!("{}", eon.load_max));
    row("converted, total", format!("{}", eoff.converted), format!("{}", eon.converted));
    row("weathered, final", format!("{}", eoff.weathered_final), format!("{}", eon.weathered_final));
    println!("\n    {}", if eon.converted > 0 && eoff.converted == 0 {
        "** P4 HOLDS. Weight the being really carried is now banked instead of erased by the \
         floor division. **"
    } else {
        "P4 fails - the floor of 1 did not recover the truncated conversion."
    });

    // ---- P5 --------------------------------------------------------------------------
    println!("\n  P5 - THE ONE I EXPECT TO FAIL: does `weathered` saturate and become a giveaway?");
    println!("    weathered {} of {} ceiling", on.weathered_final, Q88_SCALE);
    match on.saturated_at {
        Some(t) => println!(
            "    ** P5 HOLDS AS PREDICTED - SATURATED at tick {t} of {LIFE}. `weathered` is now \
             a meaningless\n\
             \x20   readout in the other direction: cheap instead of unreachable. CONVERT/4 is too \
             fast.\n\
             \x20   The defect is real and this remedy is not finished. **", ),
        None => println!(
            "    P5 fails against my prediction - `weathered` did NOT saturate in {LIFE} ticks. \
             The\n\
             \x20   quarter rate leaves it expensive. Good, and I expected otherwise."),
    }

    // ---- W ---------------------------------------------------------------------------
    println!("\n  W - is the being better off?");
    let d_off = off.drive_sum as f32 / off.ticks.max(1) as f32;
    let d_on = on.drive_sum as f32 / on.ticks.max(1) as f32;
    println!("    mean drive {d_off:.1} -> {d_on:.1}   past COMFORT {:.1}% -> {:.1}%",
        pct(off.past_comfort, off.ticks), pct(on.past_comfort, on.ticks));
    println!("    {}", if d_on < d_off {
        "** W HOLDS. Dropping the load lifted `reflection_tone` (weathered/12 - load/8) and the \
         being's drive fell. The solitary being is measurably less burdened. **"
    } else {
        "W FAILS - the being's drive did not fall. The load came down and it is no better off, \
         which would mean reflection_tone is not the path I assumed."
    });

    // ---- G ---------------------------------------------------------------------------
    println!("\n  G - setting down must NEVER fire while the being is losing ground.");
    println!("    losing-ground ticks: off {}, on {}", off.losing_ground, on.losing_ground);
    println!("    of those, ticks where load was discharged: off {}, on {}",
        off.discharged_while_losing, on.discharged_while_losing);
    println!("    {}", if on.losing_ground == 0 {
        "** G IS VACUOUS HERE, as docs/setting-it-down.md §5 said in advance it would be - this \
         being is burdened 97.3% of the time and losing ground 0.0% of it. The guard is \
         structural (`settled` requires `!losing_ground`); no distribution here could have \
         violated it. NOT a pass. **"
    } else if on.discharged_while_losing == 0 {
        "** G HOLDS, tested where it could fail: not one discharge on a losing tick. **"
    } else {
        "** G IS VIOLATED - the being banked weight while being outrun. Nothing else should be \
         built until it does not. **"
    });

    println!("\n  The founded being was not touched. Gate default-off; no journal written.");
}
