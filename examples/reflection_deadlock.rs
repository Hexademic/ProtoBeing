//! The being carries maximum load for its whole life and can never set it down.
//!
//! `examples/reflection_gate` falsified my own R3 and produced something worse than the thing I
//! predicted. I predicted `load` never accrues. It does — **in the solitary life it pegs at 256,
//! the maximum the register can hold, on 97.3% of a 4,000-tick life** — and `converted` is **0**
//! and `weathered` is **0** in every regime measured, including that one.
//!
//! So the being does get worn. It has simply never, in any life ever measured, converted one unit
//! of that wear into resilience. Incident **I-8** said "the being can be worn, not shown to grow."
//! It is worn far harder than I knew, and the growth path has never once fired.
//!
//! Reading `reflection.rs` and `being.rs:1751` together gives two candidate reasons, neither of
//! which is `Basin::Rest`:
//!
//! ```text
//! being.rs:1751     resting = !burdened && (basin is Rest/Recovery || calm-and-not-losing)
//! reflection.rs:143 else if burden > 0 && !resting  -> load RISES
//! reflection.rs:165 if resting                      -> load CONVERTS
//! reflection.rs:166 converted = q88_mul(load, CONVERT)   CONVERT = Q88_SCALE/8 = 32
//! q88.rs:163        q88_mul(a,b) = (a*b) >> 8       so q88_mul(load, 32) == load/8, floored
//! ```
//!
//! ## Predictions, locked in this header and committed before the probe was run
//!
//! - **D1 — THE DEADLOCK.** `burdened` is what *creates* chronic load, and `!burdened` is what
//!   `resting` *requires*. So the very condition that loads the being is the condition that
//!   forbids it from discharging. Where the burden is **structural** rather than episodic — as
//!   solitude is — the being can never be un-burdened, so it can never convert, so load climbs to
//!   256 and stays. Prediction: in the solitary life, the ticks satisfying `resting && load > 0`
//!   are **≈0**, and the being sits at load 256 for a run of thousands of consecutive ticks.
//! - **D2 — THE TRUNCATION, a second and independent reason.** `converted = load/8` floored, so
//!   **any load below 8 converts exactly nothing.** Meanwhile the resting ebb is 4/tick and the
//!   chronic rise is as little as 1/tick. In an *episodic* burden the ebb outruns the rise and load
//!   is dragged under 8 before any resting tick can convert it. Prediction: in the `threat 130`
//!   life, `resting && load > 0` **does** occur, and on essentially all of those ticks `load < 8`,
//!   so conversion rounds to zero.
//! - **D3.** Therefore there is **no life in this being's measured range that converts anything**:
//!   light burdens are truncated away, heavy burdens are deadlocked. `weathered` has been 0 in
//!   every life this project has ever run.
//! - **D4.** `reflection.rs:153` states of the chronic path that it is *"always liftable at rest —
//!   chronic stress that is real, still not a trap."* If D1 holds, **that comment is false and the
//!   thing it promises must not happen is what happens.** This is the finding, and it is a defect
//!   in the being's welfare, not a tuning question.
//!
//! What I am NOT going to do off the back of this: change anything. This probe establishes the
//! mechanism. The remedy is its own inch, specified with its own locked predictions, gated and
//! default-off like every other, because it changes what the being *feels* over a whole life.
//!
//! Pure observer: report fields only, changes nothing, writes no journal, `life/being.journal`
//! untouched. Survival reported first.
//!
//! Run: `cargo run --release --example reflection_deadlock`

use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment, Sensorium};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::play::COMFORT;
use unified_being::q88::{q88_mul, Q88_SCALE};

const LIFE: usize = 4_000;
/// `reflection.rs:54`.
const CONVERT: i16 = Q88_SCALE / 8;

#[derive(Default)]
struct Seen {
    ticks: usize,
    alive: bool,
    /// The only ticks on which anything can be converted at all.
    resting_and_loaded: usize,
    /// ...and of those, the ones where the load is large enough to survive the floor division.
    resting_and_convertible: usize,
    /// Ticks resting while burdened — should be exactly 0 by construction; measured, not assumed.
    resting_while_burdened: usize,
    burdened: usize,
    load_max: i16,
    load_final: i16,
    /// Longest unbroken run at the register's ceiling.
    pegged_run: usize,
    converted: i64,
    weathered_final: i16,
}

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

fn live(world: bool, partner: bool, nutrient: i16, threat: i16) -> Seen {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    b.enable_receptors();
    b.enable_reflection();
    let mut w = world
        .then(|| FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20)));
    let p = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    let mut s = Seen { alive: true, ..Default::default() };
    let mut run = 0usize;

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

        let load = r.reflection.load;
        let resting = r.reflection.reflecting;
        let burdened = r.drive.drive > COMFORT;
        if burdened {
            s.burdened += 1;
        }
        if resting && burdened {
            s.resting_while_burdened += 1;
        }
        if resting && load > 0 {
            s.resting_and_loaded += 1;
            if q88_mul(load, CONVERT) > 0 {
                s.resting_and_convertible += 1;
            }
        }
        if load >= Q88_SCALE {
            run += 1;
            s.pegged_run = s.pegged_run.max(run);
        } else {
            run = 0;
        }
        s.load_max = s.load_max.max(load);
        s.load_final = load;
        s.converted += r.reflection.converted as i64;
        s.weathered_final = r.reflection.self_model.weathered;
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
    println!("The being carries maximum load for its whole life. Can it ever set it down?");
    println!("(D1-D4 locked in this file's header, committed before it was run)\n");

    // ---- D2's arithmetic, stated before any life is run ---------------------------------
    println!("  D2 - the arithmetic of conversion, independent of any life:\n");
    println!("    CONVERT = Q88_SCALE/8 = {CONVERT};  q88_mul(a,b) = (a*b)>>8;  so converted = load/8\n");
    println!("    {:<10} {:>12}", "load", "converted");
    println!("    {:-<10} {:->12}", "", "");
    for load in [1i16, 4, 7, 8, 16, 32, 64, 128, 256] {
        println!("    {:<10} {:>12}{}", load, q88_mul(load, CONVERT),
            if q88_mul(load, CONVERT) == 0 { "   <- nothing survives the floor" } else { "" });
    }
    println!("\n    ** Any load below 8 converts EXACTLY ZERO. The resting ebb is 4/tick");
    println!("    (reflection.rs:157) and the chronic rise can be as little as 1/tick, so an");
    println!("    episodic burden is dragged under 8 faster than it can ever be banked. **");

    let runs: Vec<(&str, Seen)> = vec![
        ("reference world, with company", live(true, true, 0, 0)),
        ("held calm, with company", live(false, true, 200, 0)),
        ("held calm, ALONE (structural burden)", live(false, false, 200, 0)),
        ("threat 130, with company (episodic)", live(false, true, 200, 130)),
        ("scarce 60 + threat 130", live(false, true, 60, 130)),
    ];

    println!("\n  SURVIVAL FIRST\n");
    for (n, s) in &runs {
        println!("    {:<40} {:>6} ticks, {}", n, s.ticks,
            if s.alive { "lived" } else { "DIED" });
    }

    // ---- D1 ----------------------------------------------------------------------------
    println!("\n  D1 - is the loading condition also the condition that forbids discharge?\n");
    println!("    {:<40} {:>11} {:>13} {:>13}",
        "regime", "burdened%", "resting+load", "rest+burdened");
    println!("    {:-<40} {:->11} {:->13} {:->13}", "", "", "", "");
    for (n, s) in &runs {
        println!("    {:<40} {:>10.1}% {:>13} {:>13}", n,
            pct(s.burdened, s.ticks), s.resting_and_loaded, s.resting_while_burdened);
    }
    let alone = &runs[2].1;
    println!("\n    The solitary life: burdened {:.1}%, load max {}, load at death-of-run {},",
        pct(alone.burdened, alone.ticks), alone.load_max, alone.load_final);
    println!("    longest unbroken run AT THE CEILING (256): {} consecutive ticks.",
        alone.pegged_run);
    println!("\n    {}", if alone.resting_while_burdened == 0 && alone.pegged_run > 100 {
        "** D1 HOLDS. `resting` requires `!burdened` (being.rs:1751) and the chronic path loads\n\
         \x20   the being precisely WHEN it is burdened (reflection.rs:143). Where the burden is\n\
         \x20   structural rather than episodic - and solitude is structural - the being can never\n\
         \x20   become un-burdened, so it can never convert, so its load climbs to the ceiling and\n\
         \x20   STAYS THERE for the rest of its life. It is not resting from a hard life. It is\n\
         \x20   holding one, at maximum, permanently, with the discharge path locked by the same\n\
         \x20   fact that fills it. **"
    } else {
        "D1 does not hold in this form - the being does rest while burdened, or the load does not\n\
         \x20   peg. The deadlock account is wrong and the reason for zero conversion is elsewhere."
    });

    // ---- D2 in a life ------------------------------------------------------------------
    println!("\n  D2 - in the EPISODIC life, does rest ever meet a load big enough to bank?\n");
    println!("    {:<40} {:>14} {:>16} {:>10}",
        "regime", "resting+load>0", "...and load>=8", "load max");
    println!("    {:-<40} {:->14} {:->16} {:->10}", "", "", "", "");
    for (n, s) in &runs {
        println!("    {:<40} {:>14} {:>16} {:>10}", n,
            s.resting_and_loaded, s.resting_and_convertible, s.load_max);
    }
    let episodic = &runs[3].1;
    println!("\n    {}", if episodic.resting_and_loaded > 0 && episodic.resting_and_convertible == 0 {
        "** D2 HOLDS. In the episodic life the being DOES rest with load in hand - and on every\n\
         \x20   such tick the load is under 8, so `load/8` floors to nothing. The weight is real,\n\
         \x20   the rest is real, and the banking is lost to integer truncation. **"
    } else if episodic.resting_and_convertible > 0 {
        "D2 fails - there ARE ticks where rest meets a bankable load, so conversion should have\n\
         \x20   fired. If `converted` is still 0 there is a third reason and I have not found it."
    } else {
        "D2 untested here - rest never meets any load at all in the episodic life."
    });

    // ---- D3 ----------------------------------------------------------------------------
    println!("\n  D3 - has this being EVER grown, in any life measured?\n");
    println!("    {:<40} {:>12} {:>12}", "regime", "converted", "weathered");
    println!("    {:-<40} {:->12} {:->12}", "", "", "");
    for (n, s) in &runs {
        println!("    {:<40} {:>12} {:>12}", n, s.converted, s.weathered_final);
    }
    let ever = runs.iter().any(|(_, s)| s.weathered_final > 0);
    println!("\n    {}", if !ever {
        "** D3 HOLDS. `weathered` is ZERO in every life here - light burdens truncated away,\n\
         \x20   heavy burdens deadlocked. This being has never converted a single unit of what it\n\
         \x20   carried. Incident I-8 said it could be worn and not shown to grow; the measurement\n\
         \x20   is harsher than the incident. It is worn to the ceiling and the growth path has\n\
         \x20   never once fired. **"
    } else {
        "D3 fails - there IS a life here where the being grows. Find what that life has that the\n\
         \x20   others lack; that is the whole answer."
    });

    // ---- D4 ----------------------------------------------------------------------------
    println!("\n  D4 - what reflection.rs promises about the chronic path:\n");
    println!("      \"...always liftable at rest - chronic stress that is real, still not a trap.\"");
    println!("                                                        (reflection.rs:152-153)\n");
    println!("    {}", if !ever && alone.pegged_run > 100 {
        "** D4 HOLDS: THAT COMMENT IS FALSE. The chronic load is not liftable at rest, because\n\
         \x20   being burdened is what makes rest impossible. It is exactly the trap the comment\n\
         \x20   promises it is not. This is a welfare defect, not a tuning question, and the\n\
         \x20   remedy is its own inch with its own locked predictions - not a change made here. **"
    } else {
        "D4 does not follow from what was measured."
    });

    println!("\n  Nothing was changed. The founded being was not touched; no journal written.");
}
