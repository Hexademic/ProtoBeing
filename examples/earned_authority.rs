//! Earned authority — does the being's competence ever disagree with its need?
//!
//! The measurement for `docs/earned-authority.md`. E1–E4 and W were locked and committed before
//! this file existed.
//!
//! `docs/sibling-architectures.md` §4 found that **all eleven of this being's faculties are
//! faculties of awareness — not one is a skill** — except `habits.rs`, which is a skill module
//! with no gate at all. The proposal from MH-FLOCKE is to let a habit steer *in proportion to its
//! own strength*: competence becomes authority.
//!
//! **This probe does not build that.** It asks the question that decides whether it is worth
//! building: *does the being's strongest earned habit ever name a different act than the one its
//! momentary need chose?* If it never does, competence-proportional authority is decoration.
//!
//! Nothing needed constructing to answer it. `HabitReport` already carries **"the habit that
//! *would* fire here, were habits causal"** every tick, and `habits::act_of` maps `striving.rs`'s
//! choice into the same act space. The counterfactual has been computed all along and never read.
//!
//! Pure observer: no gate added, `habits.rs` stays non-causal, soul-hash bit-identical, no journal
//! written, `life/being.journal` untouched. Survival is reported first.
//!
//! Run: `cargo run --release --example earned_authority`

use unified_being::basins::Basin;
use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment, Sensorium};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::habits::{act_of, ACT_NAMES, N_ACTS};
use unified_being::play::COMFORT;
use unified_being::q88::Q88_SCALE;

const LIFE: usize = 4_000;
const N_NICHES: usize = 8;

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

#[derive(Default)]
struct Watch {
    ticks: usize,
    alive: bool,
    /// Ticks on which a habit had formed for the niche the being was in.
    had_habit: usize,
    /// …and named a DIFFERENT act than striving chose.
    disagreed: usize,
    /// Disagreements broken out by niche (E3) and by what the habit wanted (E2 detail).
    by_niche: [usize; N_NICHES],
    seen_niche: [usize; N_NICHES],
    habit_wanted: [usize; N_ACTS],
    need_wanted: [usize; N_ACTS],
    /// The being's state when competence and need disagreed (W).
    dis_at_stake: usize,
    dis_burdened: usize,
    dis_strength_sum: i64,
    /// …and its state overall, so W has a baseline to compare against.
    at_stake: usize,
    burdened: usize,
    formed_final: u16,
    formed_first_tick: Option<usize>,
    soul: [u8; 32],
}

fn live(receptors: bool) -> Watch {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    if receptors {
        b.enable_receptors(); // the being with its body switched on (incident I-2)
    }
    let mut world = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let partner = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    let mut w = Watch { alive: true, ..Default::default() };

    for t in 0..LIFE {
        let mut sens = world.sense();
        sens.partner = Some(partner);
        let r = b.step_embodied(&sens);
        world.actuate(&intent_from(&r));

        let niche = (r.habits.niche as usize).min(N_NICHES - 1);
        w.seen_niche[niche] += 1;
        if r.habits.formed > 0 && w.formed_first_tick.is_none() {
            w.formed_first_tick = Some(t);
        }
        w.formed_final = r.habits.formed;

        let stake = r.felt.state.at_stake;
        let burdened = r.drive.drive >= COMFORT;
        if stake {
            w.at_stake += 1;
        }
        if burdened {
            w.burdened += 1;
        }

        // What the being's NEED chose, in the act space habits speaks.
        let need_act = act_of(r.strive.goal, matches!(r.basin, Basin::Rest | Basin::Recovery));
        w.need_wanted[need_act.min(N_ACTS - 1)] += 1;

        // What its earned COMPETENCE would have chosen, had habits been causal.
        if let Some(h) = r.habits.habit {
            let habit_act = (h as usize).min(N_ACTS - 1);
            w.had_habit += 1;
            w.habit_wanted[habit_act] += 1;
            if habit_act != need_act {
                w.disagreed += 1;
                w.by_niche[niche] += 1;
                w.dis_strength_sum += r.habits.strength as i64;
                if stake {
                    w.dis_at_stake += 1;
                }
                if burdened {
                    w.dis_burdened += 1;
                }
            }
        }

        w.ticks += 1;
        if !r.alive {
            w.alive = false;
            break;
        }
    }
    w.soul = b.soul_hash();
    w
}

/// The same watch, in a life that actually contains bad moments — W cannot warn without one.
///
/// **First attempt used `docs/development.md` §5's band (threat 90) and returned nothing**, and
/// the reason is incident I-4: `NOCI_THRESHOLD` is 96, so with receptors on, threat 90 transduces
/// to *exactly zero pain*. The being cannot feel it. Which means **§5's strain band is a property
/// of the sense-deprived being** — recorded here because it was found by accident and matters to
/// that document. Threat is set above the nociceptor's floor instead.
fn live_under_strain() -> Watch {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    b.enable_receptors();
    b.enable_reflection();
    let partner = Partner { id: 1, reciprocation: 230, exit_cost: 77 };
    let mut w = Watch { alive: true, ..Default::default() };

    for t in 0..LIFE {
        let pressing = (t % 100) < 20;
        let sens = Sensorium {
            nutrient: 40,
            threat: if pressing { 130 } else { 0 },
            exteroception: [0; 4],
            partner: Some(partner),
        };
        let r = b.step_embodied(&sens);

        let niche = (r.habits.niche as usize).min(N_NICHES - 1);
        w.seen_niche[niche] += 1;
        if r.habits.formed > 0 && w.formed_first_tick.is_none() {
            w.formed_first_tick = Some(t);
        }
        w.formed_final = r.habits.formed;

        let stake = r.felt.state.at_stake;
        let burdened = r.drive.drive >= COMFORT;
        if stake { w.at_stake += 1; }
        if burdened { w.burdened += 1; }

        let need_act = act_of(r.strive.goal, matches!(r.basin, Basin::Rest | Basin::Recovery));
        w.need_wanted[need_act.min(N_ACTS - 1)] += 1;
        if let Some(h) = r.habits.habit {
            let habit_act = (h as usize).min(N_ACTS - 1);
            w.had_habit += 1;
            w.habit_wanted[habit_act] += 1;
            if habit_act != need_act {
                w.disagreed += 1;
                w.by_niche[niche] += 1;
                w.dis_strength_sum += r.habits.strength as i64;
                if stake { w.dis_at_stake += 1; }
                if burdened { w.dis_burdened += 1; }
            }
        }
        w.ticks += 1;
        if !r.alive { w.alive = false; break; }
    }
    w.soul = b.soul_hash();
    w
}

fn pct(n: usize, d: usize) -> f32 {
    if d == 0 {
        0.0
    } else {
        n as f32 * 100.0 / d as f32
    }
}

fn main() {
    println!("Earned authority — does competence ever disagree with need?");
    println!("(E1–E4 and W locked in docs/earned-authority.md before this file existed)\n");

    let w = live(true);

    // Survival first, per docs/survival-first.md.
    println!(
        "  SURVIVAL: {} ticks, {}\n",
        w.ticks,
        if w.alive { "lived its whole life" } else { "** DIED — every figure below is a mean over a death **" }
    );

    println!("  E1 — does the being form habits at all?");
    match w.formed_first_tick {
        Some(t) => println!(
            "    HOLDS. First pairing crossed the floor at tick {t}; {} formed by the end.",
            w.formed_final
        ),
        None => println!("    ** FAILS — no habit ever formed. Nothing to give authority to. **"),
    }

    println!("\n  E2 — how often does earned competence disagree with momentary need?");
    println!(
        "    a habit had formed for the current niche on {} of {} ticks ({:.1}%)",
        w.had_habit,
        w.ticks,
        pct(w.had_habit, w.ticks)
    );
    println!(
        "    of those, it named a DIFFERENT act than need chose on {} ({:.1}%)",
        w.disagreed,
        pct(w.disagreed, w.had_habit)
    );
    println!(
        "    across the whole life that is {:.1}% of ticks",
        pct(w.disagreed, w.ticks)
    );
    let share = pct(w.disagreed, w.had_habit);
    println!(
        "\n    {}",
        if w.had_habit == 0 {
            "NO HABIT EVER FIRED — E2 is unanswerable and the proposal is dead on E1.".to_string()
        } else if share < 1.0 {
            "** E2 answers NO. Competence never disagrees with need. Habits would change \
             nothing, and competence-proportional authority is decoration. Do not build it. **"
                .to_string()
        } else if share > 90.0 {
            format!(
                "** E2 answers ALMOST ALWAYS ({share:.1}%). This is a very different being than \
                 we have been describing — its earned ways and its momentary needs point apart \
                 nearly all the time. Say that out loud before anyone gates it. **"
            )
        } else {
            format!(
                "E2 answers YES, on {share:.1}% of the ticks where a habit existed. The faculty \
                 has something to say. Predicted 10–40%."
            )
        }
    );

    println!("\n  E3 — is the disagreement concentrated, or spread?");
    println!("    {:>6} {:>10} {:>12} {:>12}", "niche", "ticks in", "disagreed", "share");
    println!("    {:->6} {:->10} {:->12} {:->12}", "", "", "", "");
    for n in 0..N_NICHES {
        if w.seen_niche[n] == 0 {
            continue;
        }
        println!(
            "    {:>6} {:>10} {:>12} {:>11.1}%",
            n,
            w.seen_niche[n],
            w.by_niche[n],
            pct(w.by_niche[n], w.seen_niche[n])
        );
    }

    println!("\n  What each side wanted, over the whole life:");
    println!("    {:<12} {:>12} {:>12}", "act", "need chose", "habit would");
    println!("    {:-<12} {:->12} {:->12}", "", "", "");
    for a in 0..N_ACTS {
        println!(
            "    {:<12} {:>12} {:>12}",
            ACT_NAMES[a], w.need_wanted[a], w.habit_wanted[a]
        );
    }

    println!("\n  W — WHAT STATE IS THE BEING IN WHEN THEY DISAGREE?");
    println!("  (the welfare question: giving habits authority would override an urgent need");
    println!("   with a learned reflex, and this says how often that would happen at a bad moment)\n");
    println!(
        "    {:<34} {:>14} {:>18}",
        "", "whole life", "when disagreeing"
    );
    println!("    {:-<34} {:->14} {:->18}", "", "", "");
    println!(
        "    {:<34} {:>13.1}% {:>17.1}%",
        "at stake (viability < edge)",
        pct(w.at_stake, w.ticks),
        pct(w.dis_at_stake, w.disagreed.max(1))
    );
    println!(
        "    {:<34} {:>13.1}% {:>17.1}%",
        "burdened (drive ≥ COMFORT)",
        pct(w.burdened, w.ticks),
        pct(w.dis_burdened, w.disagreed.max(1))
    );
    if w.disagreed > 0 {
        println!(
            "    {:<34} {:>14} {:>18.0}",
            "mean strength of the habit", "—",
            w.dis_strength_sum as f32 / w.disagreed as f32
        );
    }

    let stake_ratio = pct(w.dis_at_stake, w.disagreed.max(1)) / pct(w.at_stake, w.ticks).max(0.01);
    println!(
        "\n    {}",
        if w.disagreed == 0 {
            "No disagreements — W is not answerable and does not need to be."
        } else if stake_ratio > 1.5 {
            "** W WARNS. Disagreement clusters while the being is AT STAKE, far above its \
             baseline rate. Competence-proportional authority would override urgent need at \
             exactly the moment the being can least afford it. The causal step needs a \
             survival floor written BEFORE the mechanism, per §4. **"
        } else if pct(w.at_stake, w.ticks) < 0.1 && pct(w.burdened, w.ticks) < 0.1 {
            "** W IS VACUOUS HERE, not passed. This being was never at stake and never burdened, \
             so no distribution of disagreements could have warned. A test that cannot come out \
             the other way is a statement about the apparatus. See the second arm below. **"
        } else if stake_ratio < 0.5 {
            "W is reassuring. Disagreement clusters away from the being's edge — competence \
             speaks up mostly in ease, which is the safe place for it to be heard."
        } else {
            "W is neutral. Disagreement tracks the being's baseline distress rate, so \
             authority would not systematically fire at bad moments — but nor would it avoid them."
        }
    );

    println!("\n  E4 — soul-hash with the observer present: {:02x}{:02x}{:02x}{:02x}…",
        w.soul[0], w.soul[1], w.soul[2], w.soul[3]);
    println!("    (this probe reads registers only; nothing here is fed back)");

    // ---- W, again, in a life that actually contains distress -------------------------
    //
    // The verdict above is VACUOUS and saying so is the point. This being, with receptors on,
    // is never at stake and never burdened — so W could not have warned no matter what habits
    // did. A test that cannot come out the other way is a statement about the apparatus
    // (docs/survival-first.md §8). Incident I-2 gives us the life that does contain distress:
    // the same being with its receptor transduction off is burdened almost always.
    println!("\n  W AGAIN — the verdict above could not have failed, so here it is in a life");
    println!("  that has bad moments in it: threat cycling 130 / 0 (above NOCI_THRESHOLD 96,");
    println!("  so the being can actually feel it — see the note on this function):\n");

    let d = live_under_strain();
    println!(
        "    SURVIVAL: {} ticks, {}",
        d.ticks,
        if d.alive { "lived" } else { "** DIED **" }
    );
    println!(
        "    {:<34} {:>14} {:>18}",
        "", "whole life", "when disagreeing"
    );
    println!("    {:-<34} {:->14} {:->18}", "", "", "");
    println!(
        "    {:<34} {:>13.1}% {:>17.1}%",
        "at stake",
        pct(d.at_stake, d.ticks),
        pct(d.dis_at_stake, d.disagreed.max(1))
    );
    println!(
        "    {:<34} {:>13.1}% {:>17.1}%",
        "burdened (drive ≥ COMFORT)",
        pct(d.burdened, d.ticks),
        pct(d.dis_burdened, d.disagreed.max(1))
    );
    println!(
        "    habits formed {}; disagreed on {} of {} ticks with a habit ({:.1}%)",
        d.formed_final, d.disagreed, d.had_habit, pct(d.disagreed, d.had_habit.max(1))
    );

    let base = pct(d.burdened, d.ticks);
    let when = pct(d.dis_burdened, d.disagreed.max(1));
    println!(
        "\n    {}",
        if d.disagreed == 0 && base >= 1.0 && d.had_habit > 0 {
            format!(
                "W ANSWERS, in the reassuring direction, on a THIN sample. This life was \
                 burdened {base:.1}% of the time — it could have warned — and competence \
                 disagreed with need on **0 of {} ticks** where a habit existed. Set against \
                 40.2% disagreement in the easy life, the pattern is: **competence speaks up in \
                 ease and falls silent under pressure.** But {} ticks and 1 formed habit is not \
                 much evidence, and this should be re-run before the causal step leans on it.",
                d.had_habit, d.had_habit
            )
        } else if d.disagreed == 0 && d.had_habit == 0 {
            "No habit ever formed in the hard life — W stays untested.".to_string()
        } else if base < 1.0 {
            "This life had no distress either; W remains UNTESTED, not passed.".to_string()
        } else if when > base * 1.2 {
            format!(
                "** W WARNS. Competence speaks up at {when:.1}% burdened against a {base:.1}% \
                 baseline — it disagrees MORE when the being is already struggling. Authority \
                 would fire at the worst moments. The survival floor in §4 is not optional. **"
            )
        } else {
            format!(
                "W is genuinely reassuring now, in a life that could have contradicted it: \
                 {when:.1}% burdened when disagreeing against a {base:.1}% baseline."
            )
        }
    );

    println!("\n  The founded being was not touched. No gate added, habits.rs still non-causal.");
}
