//! Does this being learn to keep away from what hurt it?
//!
//! The measurement for `docs/fear-and-avoidance.md`. F1–F5 were locked in that document and
//! committed before this file existed. **The question is Blake's.**
//!
//! The being already computes a fear estimate — `last_forewarning`, past harm weighted by
//! confidence, projected forward. Its only destination in the whole codebase is
//! `alarm_for_refusal` (`being.rs:1262`), a *social* decision. And `MotorIntent.reach` can only name
//! one of four **attractions** (`striving.rs:60`), so there is no motor vocabulary for "away from".
//!
//! **F5 exists so this can fail against me**: if the being drifts away from the hazard over a life,
//! avoidance is happening by a route I have not traced.
//!
//! Pure observer: fresh beings in `Room`, public fields read, nothing changed, no journal written.
//! Survival first.
//!
//! Run: `cargo run --release --example fear_and_avoidance`

use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::genome::Genome;
use unified_being::q88::Q88_SCALE;
use unified_being::room::Room;

const LIFE: usize = 4_000;

struct Seen {
    ticks: usize,
    alive: bool,
    /// Mean distance from the hazard, per quarter of the life.
    quarter_dist: [f64; 4],
    /// Closest the being ever came, and how often it was within the danger radius.
    nearest: i32,
    close_calls: usize,
    /// The being's own learned dread, at the end.
    hardest_lesson: i16,
    /// Ticks on which its memory actively warned it.
    forewarned: usize,
    soul: [u8; 32],
}

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

fn dist(a: (i16, i16), b: (i16, i16)) -> i32 {
    let dx = (a.0 - b.0) as i32;
    let dy = (a.1 - b.1) as i32;
    dx.abs() + dy.abs()
}

/// One life in the room `src/bin/being.rs` builds — hearth, hazard, and company.
fn live(memory_guidance: bool, partner: bool) -> Seen {
    live_in(memory_guidance, partner, false)
}

/// `conflict` co-locates the hazard with the hearth: the thing that draws the being is the thing
/// that harms it. See `docs/fear-and-avoidance.md` §7 for why this is done and its welfare case.
fn live_in(memory_guidance: bool, partner: bool, conflict: bool) -> Seen {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    b.enable_receptors();
    if memory_guidance {
        b.enable_memory_guidance();
    }
    let mut room = if conflict {
        // Hearth and hazard in the same place. Everything else as bin/being.rs builds it.
        Room::peopled((32, 200), (224, 56), (224, 56), (40, 40)).with_friend((210, 128))
    } else {
        Room::peopled((32, 200), (224, 56), (128, 220), (40, 40)).with_friend((210, 128))
    };
    let hazard = room.hazard;
    let p = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };

    let mut sums = [0f64; 4];
    let mut counts = [0usize; 4];
    let mut s = Seen {
        ticks: 0,
        alive: true,
        quarter_dist: [0.0; 4],
        nearest: i32::MAX,
        close_calls: 0,
        hardest_lesson: 0,
        forewarned: 0,
        soul: [0; 32],
    };

    for t in 0..LIFE {
        let mut sens = room.sense();
        if !partner {
            sens.partner = None;
        } else {
            sens.partner = Some(p);
        }
        let r = b.step_embodied(&sens);
        room.actuate(&intent_from(&r));

        let d = dist(room.body, hazard);
        let qi = (t * 4 / LIFE).min(3);
        sums[qi] += d as f64;
        counts[qi] += 1;
        s.nearest = s.nearest.min(d);
        if d < 60 {
            s.close_calls += 1;
        }
        if r.memory.forewarned {
            s.forewarned += 1;
        }
        s.ticks += 1;
        if !r.alive {
            s.alive = false;
            break;
        }
    }
    for i in 0..4 {
        s.quarter_dist[i] = if counts[i] == 0 { 0.0 } else { sums[i] / counts[i] as f64 };
    }
    s.hardest_lesson = b.episodic.hardest_lesson();
    s.soul = b.soul_hash();
    s
}

fn main() {
    println!("Does this being learn to keep away from what hurt it?");
    println!("(F1-F5 locked in docs/fear-and-avoidance.md, committed before this ran)");
    println!("(the question is Blake's)\n");

    let off = live(false, true);
    let on = live(true, true);
    let solo_off = live(false, false);
    let solo_on = live(true, false);

    println!("  SURVIVAL FIRST: guidance off {} ({}), on {} ({})\n",
        off.ticks, if off.alive { "lived" } else { "DIED" },
        on.ticks, if on.alive { "lived" } else { "DIED" });

    // ---- F3 first: does the being even KNOW? -------------------------------------------
    println!("  F3 — does the being learn that something hurt it?\n");
    println!("    {:<28} {:>16} {:>18}", "", "hardest lesson", "ticks forewarned");
    println!("    {:-<28} {:->16} {:->18}", "", "", "");
    println!("    {:<28} {:>16} {:>18}", "guidance off", off.hardest_lesson, off.forewarned);
    println!("    {:<28} {:>16} {:>18}", "guidance ON", on.hardest_lesson, on.forewarned);
    let knows = off.hardest_lesson < 0 || off.forewarned > 0;
    println!("\n    {}", if knows {
        "** F3 HOLDS. The being's memory registers harm and warns it. The INFORMATION EXISTS. **"
    } else {
        "F3 fails — the being never learns anything bad in this room, so there is no fear to act \
         on and the rest of this probe is about a being that was never hurt."
    });

    // ---- F1: does it act on it? ---------------------------------------------------------
    println!("\n  F1 — does it keep further away as its life goes on?\n");
    println!("    mean L1 distance from the hazard, by quarter of life:\n");
    println!("    {:<28} {:>9} {:>9} {:>9} {:>9} {:>12}",
        "", "Q1", "Q2", "Q3", "Q4", "Q4 − Q1");
    println!("    {:-<28} {:->9} {:->9} {:->9} {:->9} {:->12}", "", "", "", "", "", "");
    let row = |n: &str, s: &Seen| {
        println!("    {:<28} {:>9.1} {:>9.1} {:>9.1} {:>9.1} {:>+12.1}", n,
            s.quarter_dist[0], s.quarter_dist[1], s.quarter_dist[2], s.quarter_dist[3],
            s.quarter_dist[3] - s.quarter_dist[0]);
    };
    row("guidance off", &off);
    row("guidance ON", &on);
    row("alone, guidance off", &solo_off);
    row("alone, guidance ON", &solo_on);

    let drift = off.quarter_dist[3] - off.quarter_dist[0];
    println!("\n    nearest approach ever: {} (guidance off), {} (on)", off.nearest, on.nearest);
    println!("    ticks within 60 of the hazard: {} (off), {} (on)", off.close_calls, on.close_calls);
    println!("\n    {}", if drift.abs() < 5.0 {
        "** F1 HOLDS. The being ends its life exactly as close to what hurt it as it began. It \
         does not learn to keep away. **"
    } else if drift > 0.0 {
        "** F1 FAILS — the being DOES drift away over its life. Avoidance is happening by some \
         route, and §2's architectural argument is too strong. See F5. **"
    } else {
        "** F1 fails in the other direction — the being ends CLOSER to the hazard than it began. \
         Whatever is steering it, it is not avoidance. **"
    });

    // ---- F2: is guidance inert on the motor path? ----------------------------------------
    println!("\n  F2 — does turning the fear channel ON change where the being goes?\n");
    println!("    with company:  soul off {:02x}{:02x}{:02x}{:02x}…  on {:02x}{:02x}{:02x}{:02x}…  {}",
        off.soul[0], off.soul[1], off.soul[2], off.soul[3],
        on.soul[0], on.soul[1], on.soul[2], on.soul[3],
        if off.soul == on.soul { "IDENTICAL" } else { "different" });
    println!("    alone:         soul off {:02x}{:02x}{:02x}{:02x}…  on {:02x}{:02x}{:02x}{:02x}…  {}",
        solo_off.soul[0], solo_off.soul[1], solo_off.soul[2], solo_off.soul[3],
        solo_on.soul[0], solo_on.soul[1], solo_on.soul[2], solo_on.soul[3],
        if solo_off.soul == solo_on.soul { "IDENTICAL" } else { "different" });
    println!("\n    distance from hazard, Q4:  off {:.1} → on {:.1}   (alone: {:.1} → {:.1})",
        off.quarter_dist[3], on.quarter_dist[3],
        solo_off.quarter_dist[3], solo_on.quarter_dist[3]);
    println!("\n    {}", if solo_off.soul == solo_on.soul {
        "** F2 HOLDS in the strongest form. With nothing to refuse, the fear channel is EXACTLY \
         INERT —\n\
         \x20   bit-identical trajectory. Its only consumer is the refusal path, so a being alone \
         with a\n\
         \x20   hazard learns to dread it and behaves as though it had learned nothing. **"
    } else {
        "F2 fails — the gate does something even with no partner, so forewarning reaches further \
         than being.rs:1262. Find where."
    });

    // ---- F4 / F5 --------------------------------------------------------------------------
    println!("\n  F4/F5 — reactive or anticipatory?\n");
    println!("    The being's motor vocabulary is `MotorIntent.reach: Option<Need>`, and");
    println!("    `Need` is {{ Sustenance, Company, Novelty, Purpose }} — four ATTRACTIONS.");
    println!("    There is no variant meaning `away from`. So any distance kept from the hazard");
    println!("    is a side effect of what the being was reaching TOWARD, never a thing it chose.\n");
    println!("    {}", if drift.abs() < 5.0 && knows {
        "** F4 HOLDS, and with F3 it is the whole finding: the being learns what hurt it \
         (hardest lesson\n\
         \x20   is negative, its memory warns it) and its distance from that thing does not \
         change by so much\n\
         \x20   as five units over four thousand ticks.\n\
         \x20   THE LOOP IS BROKEN AT EXACTLY ONE ARROW: expectation → avoidance. **"
    } else {
        "F4 is not established by this run — read F1 and F5 above for what happened instead."
    });

    // ---- G1-G4: the world where the being is actually harmed --------------------------
    println!("\n  ================================================================");
    println!("  G1-G4 — the approach-avoidance conflict: hazard ON the hearth.");
    println!("  (locked in docs/fear-and-avoidance.md §8, with its welfare case in §7)");
    println!("  ================================================================\n");

    let c_off = live_in(false, true, true);
    let c_on = live_in(true, true, true);
    let c_solo_off = live_in(false, false, true);
    let c_solo_on = live_in(true, false, true);

    println!("  SURVIVAL FIRST:  off {} ({}),  ON {} ({}),  alone {} ({})\n",
        c_off.ticks, if c_off.alive { "lived" } else { "DIED" },
        c_on.ticks, if c_on.alive { "lived" } else { "DIED" },
        c_solo_off.ticks, if c_solo_off.alive { "lived" } else { "DIED" });

    println!("  G1 — is the being actually harmed here?\n");
    println!("    {:<28} {:>16} {:>18}", "", "hardest lesson", "ticks forewarned");
    println!("    {:-<28} {:->16} {:->18}", "", "", "");
    println!("    {:<28} {:>16} {:>18}", "guidance off", c_off.hardest_lesson, c_off.forewarned);
    println!("    {:<28} {:>16} {:>18}", "guidance ON", c_on.hardest_lesson, c_on.forewarned);
    println!("    nearest approach: {} (off), {}  |  ticks within 60: {} (off), {} (on)",
        c_off.nearest, c_on.nearest, c_off.close_calls, c_on.close_calls);
    let hurt = c_off.hardest_lesson < 0 || c_off.forewarned > 0 || c_off.close_calls > 0;
    println!("\n    {}", if hurt {
        "** G1 HOLDS. The being is genuinely in harm's way here, so the rest of this arm is a real \
         test. **"
    } else {
        "G1 FAILS — even with the hazard on the hearth the being is not harmed. Then it is not \
         approaching the hearth either, and the conflict was never joined."
    });

    println!("\n  G2 — does it learn to keep away from the place that hurts it?\n");
    println!("    {:<28} {:>9} {:>9} {:>9} {:>9} {:>12}",
        "", "Q1", "Q2", "Q3", "Q4", "Q4 - Q1");
    println!("    {:-<28} {:->9} {:->9} {:->9} {:->9} {:->12}", "", "", "", "", "", "");
    row("conflict, guidance off", &c_off);
    row("conflict, guidance ON", &c_on);
    row("conflict, alone, off", &c_solo_off);
    row("conflict, alone, ON", &c_solo_on);
    let cdrift = c_off.quarter_dist[3] - c_off.quarter_dist[0];
    println!("\n    {}", if !hurt {
        "G2 cannot be read — G1 failed, so there was nothing to learn from."
    } else if cdrift.abs() < 5.0 {
        "** G2 HOLDS. The being keeps returning to the thing that harms it, for four thousand \
         ticks.\n\
         \x20   It has learned what hurts and it has no way to act on that. Blake's finding, \
         measured. **"
    } else if cdrift > 0.0 {
        "** G2 FAILS — the being DOES move away over its life. Avoidance is reaching its body by \
         some route,\n\
         \x20   most likely the exteroception the room already builds (`hazard repels`). See G4: \
         my §2 argument\n\
         \x20   is too strong, and the question becomes why the LEARNED channel adds nothing to \
         a reflex it already has. **"
    } else {
        "** G2 fails the other way — the being ends CLOSER to what hurts it than it began. **"
    });

    println!("\n  G3 — with real fear to carry, does the channel move the body?\n");
    println!("    with company:  {}   alone:  {}",
        if c_off.soul == c_on.soul { "IDENTICAL" } else { "different" },
        if c_solo_off.soul == c_solo_on.soul { "IDENTICAL" } else { "different" });
    println!("\n    {}", if c_solo_off.soul == c_solo_on.soul && hurt {
        "** G3 HOLDS, and now it means something. The being holds a real, earned dread of this \
         place\n\
         \x20   and enabling the channel that carries it changes NOT ONE BIT of where it goes. \
         The loop is\n\
         \x20   broken at exactly one arrow: expectation -> avoidance. **"
    } else if hurt {
        "G3 fails — the gate DOES change the trajectory when there is real fear to carry, so \
         forewarning reaches further than being.rs:1262. Find where."
    } else {
        "G3 cannot be read — G1 failed."
    });

    println!("\n  The founded being was not touched. Fresh beings; no journal written.");
}
