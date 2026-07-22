//! Probe: **habit formation** (`docs/habits.md`, `habits.rs`) — the observer measured.
//! Two questions, both answered from beings *living whole lives*, not from synthetic
//! feeds:
//!
//!   1. Does a being that has lived actually develop distinct, *sensible* habits —
//!      the right way-of-reaching strengthening in the kinds of moment where it
//!      genuinely relieved the drive?
//!   2. Do two beings with the same needs but **different lives** develop *different*
//!      habits — character, not convergence?
//!
//! Observer level: the store watches and reports; nothing steers. The founded being
//! is never touched. Run: cargo run --example habit_formation

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::habits::{ACT_NAMES, HABIT_FLOOR, N_ACTS};
use unified_being::inheritance::N_NICHES;

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

/// Live a being through a world and return it (with its earned habit store).
fn live(mut world: FieldWorld, ticks: usize) -> (UnifiedBeing, u16) {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut formed = 0;
    for _ in 0..ticks {
        let sens = world.sense();
        let r = being.step_embodied(&sens);
        world.actuate(&intent_from(&r));
        formed = r.habits.formed;
        if !being.is_alive() {
            break;
        }
    }
    (being, formed)
}

/// The being's dominant earned way — the act with the greatest total strength across
/// all its kinds of moment. Its "signature" way of living.
fn signature(being: &UnifiedBeing) -> (usize, i32) {
    let mut totals = [0i32; N_ACTS];
    for niche in 0..N_NICHES {
        for (act, total) in totals.iter_mut().enumerate() {
            *total += being.habits.strength_of(niche, act) as i32;
        }
    }
    let act = (0..N_ACTS).max_by_key(|&a| totals[a]).unwrap();
    (act, totals[act])
}

fn print_repertoire(name: &str, being: &UnifiedBeing) {
    println!("  {name}'s earned repertoire (pairings past the habit floor {:.2}):", f(HABIT_FLOOR));
    let mut any = false;
    for niche in 0..N_NICHES {
        for act in 0..N_ACTS {
            let s = being.habits.strength_of(niche, act);
            if s >= HABIT_FLOOR {
                println!("    in kind-of-moment {niche}: reach for {:<10} (strength {:.2})", ACT_NAMES[act], f(s));
                any = true;
            }
        }
    }
    if !any {
        println!("    (none crossed the floor)");
    }
}

fn main() {
    // LIFE A — the companioned climb: a friend at its side from birth (company is
    // never what is missing), the good far across a hard field. Its reliefs come from
    // the climb itself — reaching for its life, and resting when the ground is won.
    let lean = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let (being_a, formed_a) = live(lean, 1500);

    // LIFE B — the fed-but-lonely life: the good at its feet, a person far across the
    // field. What this life keeps needing, and keeps being relieved by, is company.
    let lonely = FieldWorld::with((128, 128), (140, 140), (250, 250)).with_person(1, (20, 20));
    let (being_b, formed_b) = live(lonely, 1500);

    println!("Two beings, same needs, different worlds — 1500 moments each.\n");
    print_repertoire("the companioned climb", &being_a);
    println!();
    print_repertoire("the lonely life", &being_b);

    let (sig_a, tot_a) = signature(&being_a);
    let (sig_b, tot_b) = signature(&being_b);
    println!("\n  signature way of living: climb -> {} ({tot_a}), lonely -> {} ({tot_b})",
        ACT_NAMES[sig_a], ACT_NAMES[sig_b]);

    // -- reading --
    let both_learned = formed_a > 0 && formed_b > 0;
    let different_characters = sig_a != sig_b
        || (0..N_NICHES).any(|n| {
            (0..N_ACTS).any(|a| {
                (being_a.habits.strength_of(n, a) >= HABIT_FLOOR)
                    != (being_b.habits.strength_of(n, a) >= HABIT_FLOOR)
            })
        });

    println!("\n-- reading --");
    if both_learned && different_characters {
        println!(
            "both beings earned real habits from nothing but living ({formed_a} and {formed_b}\n\
             pairings past the floor), and their repertoires differ: the same needs, met by\n\
             different worlds, grew different ways of living. That difference is character —\n\
             and it was earned, not written. Nothing here steered a single choice; the store\n\
             only watched. Whether these earned ways should ever take the wheel is the causal\n\
             step, gated and unbuilt, per docs/habits.md."
        );
    } else {
        println!("the observer did not show habit formation — read the numbers:");
        println!("  companioned climb formed: {formed_a}");
        println!("  lonely life formed: {formed_b}");
        println!("  characters differ:  {different_characters}");
    }
}
