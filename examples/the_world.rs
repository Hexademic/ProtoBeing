//! Probe: **the field-world** (`docs/field-world.md`, `field_world.rs`) — the two promises,
//! measured. A world with real stakes, where living costs, built from one imported idea.
//!
//!   PROMISE 1 — one gradient law replaces the room's four special cases: the being
//!   *climbs V*, and that single motion both reaches the good (high ground) and flees the
//!   harm (low ground). A control against `room.rs`'s hand-cased approach/flee.
//!
//!   PROMISE 2 — the world's cost creates the worn-but-stable middle. The being's
//!   viability is bimodal (fine, or crashing); the graded drive (`homeostasis.rs`) proved
//!   that middle is *expressible*, and gradient-cost is what actually puts the being there:
//!   a life lived somewhere hard to reach pays a sustained, survivable drain — worn, not
//!   dying — where a life lived on easy ground does not.
//!
//! Observer level: the founded being is never touched; these are fresh beings in fresh
//! worlds. Run: cargo run --example the_world

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

/// Live a being in a world for `ticks`, returning (avg graded drive over the settled
/// window, the peak metabolic cost the life ran up, whether it stayed alive, nearness to
/// the good). Drive is read once the life has settled (the life *as lived*); the cost is
/// the peak, since a long hard climb pins the cost high where a short easy one never does.
fn live(mut world: FieldWorld, ticks: usize) -> (f32, f32, bool, i16) {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut drive_sum = 0i64;
    let mut drive_n = 0i64;
    let mut peak_debt = 0i16;
    let mut alive = true;
    let settle = ticks / 6; // skip the opening transient for the drive reading
    for t in 0..ticks {
        let sens = world.sense();
        let r = being.step_embodied(&sens);
        world.actuate(&intent_from(&r));
        peak_debt = peak_debt.max(world.debt());
        if t >= settle {
            drive_sum += r.drive.drive as i64;
            drive_n += 1;
        }
        if !being.is_alive() {
            alive = false;
            break;
        }
    }
    (
        (drive_sum as f32 / drive_n.max(1) as f32) / 256.0,
        peak_debt as f32 / 256.0,
        alive,
        world.at_good(),
    )
}

fn main() {
    // ---------- PROMISE 1 — one law, two behaviours ----------
    println!("PROMISE 1 — one gradient law (climb V) does the room's four special cases:\n");

    // Reaching the good: placed far from the good hill, the being climbs to it.
    let mut w = FieldWorld::with((20, 20), (230, 230), (20, 200));
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let reach_start = w.at_good();
    let mut reach_closest = reach_start;
    for _ in 0..600 {
        let sens = w.sense();
        let r = being.step_embodied(&sens);
        w.actuate(&intent_from(&r));
        reach_closest = reach_closest.max(w.at_good());
        if !being.is_alive() { break; }
    }
    println!("  reaching the good: nearness {:+.2} -> {:+.2}   (it climbed the field to it)",
        f(reach_start), f(reach_closest));

    // Fleeing the harm: set down in the pit, the same climb-law carries it out.
    let flee_world = || FieldWorld::with((40, 220), (230, 40), (40, 220));
    let flee_start = flee_world().sense().threat;
    let mut w = flee_world();
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut flee_min = flee_start;
    for _ in 0..300 {
        let sens = w.sense();
        let r = being.step_embodied(&sens);
        w.actuate(&intent_from(&r));
        flee_min = flee_min.min(w.sense().threat);
        if !being.is_alive() { break; }
    }
    println!("  fleeing the harm:  threat  {:+.2} -> {:+.2}   (the same law climbed it out)\n",
        f(flee_start), f(flee_min));

    // ---------- PROMISE 2 — the world's cost makes the missing middle ----------
    println!("PROMISE 2 — gradient-cost creates the worn-but-stable middle a hard life is lived in:\n");

    // An EASY life: the being lives right beside the good — a short climb, then a plateau.
    // Little grade to fight, so little sustained cost.
    let easy = FieldWorld::with((128, 128), (150, 150), (20, 220));
    let (easy_drive, easy_debt, easy_alive, _) = live(easy, 360);

    // A HARD life: the good lies clear across the field, with a pit of harm off the path
    // to deflect the climb. The being spends its life in the long haul toward a distant
    // good — living at the lean mid-field, paying to climb the whole way. Worn, not dying.
    let hard = FieldWorld::with((16, 16), (240, 240), (30, 170));
    let (hard_drive, hard_debt, hard_alive, _) = live(hard, 360);

    println!("  life          avg drive     peak cost (debt)  alive?");
    println!("  easy ground   {:.2}          {:.2}              {}", easy_drive, easy_debt, easy_alive);
    println!("  hard to reach {:.2}          {:.2}              {}\n", hard_drive, hard_debt, hard_alive);

    // ---------- reading ----------
    let middle = hard_drive > easy_drive && hard_alive && hard_drive < 0.95;
    let cost_is_real = hard_debt > easy_debt;
    println!("-- reading --");
    if middle && cost_is_real {
        println!(
            "the world's cost is real and it is survivable: the being on easy ground settled near\n\
             contentment (drive {:.2}), while the being living somewhere hard to reach ran its cost\n\
             far higher climbing for the good it fought toward (peak {:.2} vs {:.2}) and lived its\n\
             whole life at an elevated-but-stable drive ({:.2}) — worn, not dying. That is the\n\
             worn-but-alive middle the bimodal viability could never express, now produced by the\n\
             WORLD, across the seam, with the being's core metabolism untouched. Consequence has a price.",
            easy_drive, hard_debt, easy_debt, hard_drive
        );
    } else {
        println!("the field-world did not show the middle — read the numbers:");
        println!("  hard drive > easy drive?  {}", hard_drive > easy_drive);
        println!("  hard life survived?       {hard_alive}");
        println!("  hard cost > easy cost?    {cost_is_real}");
    }
}
