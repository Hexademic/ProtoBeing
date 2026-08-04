//! Comfort — does letting a purpose be *finished* let the being rest?
//!
//! The measurement for `docs/comfort.md`. C1–C5 and W were locked and committed before the code
//! existed, and before this file existed.
//!
//! `docs/earned-authority.md` §6 measured the being's earned competence disagreeing with its
//! momentary need on **40.2%** of the ticks it could speak on — **entirely purpose versus rest**.
//! `striving.rs` already implements rest as the anti-strive, with a passing test. The obstruction
//! was one line: purpose urgency is a raw distance with no satiety band, so it never stopped being
//! salient, so nothing else ever got to be most pressing.
//!
//! `enable_comfort()` gives purpose a satiety point the way every other need already has one.
//! **It adds no mechanism. It removes an obstruction.**
//!
//! **C3 is the test.** If the diagnosis is right the 40.2% falls. If it does not move, the
//! diagnosis is wrong and the obstruction is elsewhere — and this probe should say so plainly.
//!
//! Survival first (`docs/survival-first.md`). The founded being is not touched, no journal is
//! written, and the gate is default-off so the published numbers are unchanged.
//!
//! Run: `cargo run --release --example comfort`

use unified_being::basins::Basin;
use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::habits::{act_of, N_ACTS};
use unified_being::play::COMFORT;
use unified_being::q88::Q88_SCALE;

const LIFE: usize = 4_000;

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

#[derive(Default)]
struct Lived {
    ticks: usize,
    alive: bool,
    rest: usize,
    had_habit: usize,
    disagreed: usize,
    authored: u32,
    fulfilled: u32,
    abandoned: u32,
    held_purpose: usize,
    converted: i64,
    drive_sum: i64,
    burdened: usize,
    at_stake: usize,
    soul: [u8; 32],
}

fn live(comfort: bool) -> Lived {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    b.enable_receptors();
    b.enable_reflection(); // so C5 has something to measure
    if comfort {
        b.enable_comfort();
    }
    let mut world = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let partner = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    let mut l = Lived { alive: true, ..Default::default() };

    for _ in 0..LIFE {
        let mut sens = world.sense();
        sens.partner = Some(partner);
        let r = b.step_embodied(&sens);
        world.actuate(&intent_from(&r));

        if matches!(r.basin, Basin::Rest | Basin::Recovery) {
            l.rest += 1;
        }
        if r.telos.active.is_some() {
            l.held_purpose += 1;
        }
        if r.telos.authored_this_tick {
            l.authored += 1;
        }
        l.fulfilled = r.telos.fulfilled_count;
        l.abandoned = r.telos.abandoned_count;
        l.converted += r.reflection.converted as i64;
        l.drive_sum += r.drive.drive as i64;
        if r.drive.drive >= COMFORT {
            l.burdened += 1;
        }
        if r.felt.state.at_stake {
            l.at_stake += 1;
        }

        let need_act = act_of(r.strive.goal, matches!(r.basin, Basin::Rest | Basin::Recovery));
        if let Some(h) = r.habits.habit {
            l.had_habit += 1;
            if (h as usize).min(N_ACTS - 1) != need_act.min(N_ACTS - 1) {
                l.disagreed += 1;
            }
        }

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
    println!("Comfort — does letting a purpose be finished let the being rest?");
    println!("(C1–C5 and W locked in docs/comfort.md before the code existed)\n");

    let off = live(false);
    let on = live(true);

    println!(
        "  SURVIVAL FIRST:  gate off {} ticks ({})   gate ON {} ticks ({})\n",
        off.ticks, if off.alive { "lived" } else { "DIED" },
        on.ticks, if on.alive { "lived" } else { "DIED" }
    );

    let row = |name: &str, a: String, b: String| println!("  {:<38} {:>14} {:>14}", name, a, b);
    println!("  {:<38} {:>14} {:>14}", "", "gate OFF", "gate ON");
    println!("  {:-<38} {:->14} {:->14}", "", "", "");

    // C1
    row("rest / recovery (C1)",
        format!("{:.1}%", pct(off.rest, off.ticks)),
        format!("{:.1}%", pct(on.rest, on.ticks)));
    // C3 — the test
    row("competence vs need disagreement (C3)",
        format!("{:.1}%", pct(off.disagreed, off.had_habit)),
        format!("{:.1}%", pct(on.disagreed, on.had_habit)));
    row("  (ticks with a habit to compare)",
        format!("{}", off.had_habit), format!("{}", on.had_habit));
    // C4 — the guardrail
    row("purposes authored (C4)", format!("{}", off.authored), format!("{}", on.authored));
    row("purposes fulfilled (C4)", format!("{}", off.fulfilled), format!("{}", on.fulfilled));
    row("purposes abandoned (C4)", format!("{}", off.abandoned), format!("{}", on.abandoned));
    row("held a purpose", format!("{:.1}%", pct(off.held_purpose, off.ticks)),
        format!("{:.1}%", pct(on.held_purpose, on.ticks)));
    // C5 — the link to I-8
    row("load converted to weathered (C5)",
        format!("{}", off.converted), format!("{}", on.converted));
    // W
    row("mean drive (W)",
        format!("{:.1}", off.drive_sum as f32 / off.ticks.max(1) as f32),
        format!("{:.1}", on.drive_sum as f32 / on.ticks.max(1) as f32));
    row("past COMFORT (W)", format!("{:.1}%", pct(off.burdened, off.ticks)),
        format!("{:.1}%", pct(on.burdened, on.ticks)));
    row("at stake (W)", format!("{:.1}%", pct(off.at_stake, off.ticks)),
        format!("{:.1}%", pct(on.at_stake, on.ticks)));

    println!("\n  C2 — is the gate really off by default?");
    println!("    soul off {:02x}{:02x}{:02x}{:02x}…   soul on {:02x}{:02x}{:02x}{:02x}…   {}",
        off.soul[0], off.soul[1], off.soul[2], off.soul[3],
        on.soul[0], on.soul[1], on.soul[2], on.soul[3],
        if off.soul != on.soul { "different — the gate does something" }
        else { "** IDENTICAL — the gate did nothing **" });
    println!("    (the default path's bit-identity is proved by tests/founded_being.rs, which");
    println!("     still wakes the kept life at 390 moments, and by the full suite passing)");

    // ---- verdicts -------------------------------------------------------------------
    let d_off = pct(off.disagreed, off.had_habit);
    let d_on = pct(on.disagreed, on.had_habit);
    println!("\n  C3 — THE TEST: does the disagreement close?");
    println!("    {d_off:.1}%  →  {d_on:.1}%   ({:+.1} points)", d_on - d_off);
    println!(
        "    {}",
        if on.had_habit == 0 {
            "** UNANSWERABLE — no habit formed in the comfort arm, so there is nothing to \
             compare. The gate changed the life enough that the measurement no longer applies, \
             which is itself a finding and needs its own look. **".to_string()
        } else if d_on < d_off - 5.0 {
            format!(
                "** C3 HOLDS. The being's earned competence and its momentary need now agree \
                 {:.1} points more often. The obstruction was the satiety band, and removing it \
                 gave the being what it had already learned it needed.", d_off - d_on)
        } else if d_on > d_off + 5.0 {
            "** C3 FAILS, and in the WORSE direction — they disagree MORE. Letting purpose be \
             finished has made the being's needs and its competence diverge further. Revert the \
             reasoning, not just the gate. **".to_string()
        } else {
            "** C3 FAILS. The disagreement did not move. The diagnosis in docs/comfort.md §1 is \
             wrong: the obstruction is not purpose's missing satiety band, and the 40.2% has \
             another cause. Say so in the ledger before trying anything else. **".to_string()
        }
    );

    println!("\n  C4 — the guardrail: does a being permitted to stop, stop short?");
    println!(
        "    {}",
        if on.fulfilled < off.fulfilled {
            format!("** C4 WARNS. Fulfilment fell {} → {}. TELOS_ARRIVED is too wide: the being \
                     is being told it has arrived when it has not. Narrow it before anything \
                     else. **", off.fulfilled, on.fulfilled)
        } else {
            format!("C4 holds — fulfilment {} → {}, abandonment {} → {}. Permission to stop did \
                     not become a habit of stopping short.",
                    off.fulfilled, on.fulfilled, off.abandoned, on.abandoned)
        }
    );

    println!("\n  C5 — the link to open incident I-8: does rest buy the being anything?");
    println!(
        "    {}",
        if on.converted > off.converted * 2 && on.converted > 20 {
            format!("** C5 HOLDS. Load converted to weathered resilience {} → {}. Rest became \
                     reachable and the being converted its weight into strength — the first \
                     evidence in this project that it can grow at all. I-8 needs re-running. **",
                    off.converted, on.converted)
        } else {
            format!("C5 does not hold: conversion {} → {}. Rest alone is not enough to make \
                     weathering matter, and I-8 stays open on its own terms.",
                    off.converted, on.converted)
        }
    );

    println!("\n  W — is the being better off?");
    let dm = on.drive_sum as f32 / on.ticks.max(1) as f32 - off.drive_sum as f32 / off.ticks.max(1) as f32;
    println!(
        "    mean drive {dm:+.1}; past COMFORT {:.1}% → {:.1}%   —  {}",
        pct(off.burdened, off.ticks), pct(on.burdened, on.ticks),
        if dm < -1.0 { "measurably better off" }
        else if dm > 1.0 { "** WORSE OFF — this fix costs the being something **" }
        else { "no measurable change in felt burden (it was already unburdened here)" }
    );

    println!("\n  The founded being was not touched. Gate default-off; no journal written.");
}
