//! Would wiring habit into choice carry any signal? — `docs/habits.md`, the agency question.
//!
//! **H1–H4 are locked in that document and were committed before this file existed.**
//!
//! The being already learns: `habits.observe(niche, act, relief)` runs every tick with
//! `relief = last_drive − drive`. **Nothing outside `habits.rs` ever consults the result.**
//! `HabitReport` says so itself — *"the habit that **would** fire here, were habits causal."*
//!
//! So the question is not whether to wire it, but whether the wire would carry anything. If the
//! learned habit never disagrees with what present urgency already picks, the edge is vacuous and
//! should not be built.
//!
//! **H3 is the decision, and it is written to fail.**
//!
//! Pure observer: fresh beings, no journal written, the founded being untouched.
//!
//! Run: `cargo run --release --example habit_disagreement`

use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::genome::Genome;
use unified_being::habits::{act_of, ACT_NAMES};
use unified_being::q88::Q88_SCALE;
use unified_being::room::Room;

const LIFE: usize = 4_000;

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

#[derive(Default)]
struct Seen {
    ticks: usize,
    alive: bool,
    niches: Vec<u8>,
    formed_final: u16,
    /// Ticks on which a habit had formed for the current niche.
    with_habit: usize,
    /// ...and on which it named a DIFFERENT act than urgency chose.
    disagreed: usize,
    /// Which acts urgency ever chose, and which acts habit ever named.
    urgency_acts: Vec<usize>,
    habit_acts: Vec<usize>,
    strengths: Vec<i16>,
    /// Reconstructed exactly as being.rs computes it: last tick's drive minus this tick's.
    reliefs: Vec<i16>,
}

fn live(receptors: bool) -> Seen {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    if receptors {
        b.enable_receptors();
    }
    let mut room =
        Room::peopled((32, 200), (224, 56), (128, 220), (40, 40)).with_friend((210, 128));
    let p = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };

    let mut s = Seen { alive: true, ..Default::default() };
    let mut last_drive: Option<i16> = None;

    for _ in 0..LIFE {
        let mut sens = room.sense();
        sens.partner = Some(p);
        let r = b.step_embodied(&sens);
        room.actuate(&intent_from(&r));

        if let Some(prev) = last_drive {
            s.reliefs.push(prev - r.drive.drive);
        }
        last_drive = Some(r.drive.drive);

        let chosen = act_of(r.strive.goal, r.strive.conserving);
        s.urgency_acts.push(chosen);
        s.niches.push(r.habits.niche);

        if let Some(h) = r.habits.habit {
            s.with_habit += 1;
            s.habit_acts.push(h as usize);
            s.strengths.push(r.habits.strength);
            if h as usize != chosen {
                s.disagreed += 1;
            }
        }
        s.formed_final = r.habits.formed;

        s.ticks += 1;
        if !r.alive {
            s.alive = false;
            break;
        }
    }
    for v in [&mut s.niches] {
        v.sort_unstable();
        v.dedup();
    }
    s.urgency_acts.sort_unstable();
    s.urgency_acts.dedup();
    s.habit_acts.sort_unstable();
    s.habit_acts.dedup();
    s
}

fn names(idxs: &[usize]) -> String {
    idxs.iter()
        .map(|&i| ACT_NAMES.get(i).copied().unwrap_or("?"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn report(label: &str, s: &Seen) {
    println!("\n  === {label} ===");
    println!("  survived {} of {LIFE} ticks: {}", s.ticks, if s.alive { "yes" } else { "DIED" });
    println!("  H1  habits formed (pairings past the floor): {}", s.formed_final);
    println!("  H2  distinct niches occupied: {}  {:?}", s.niches.len(), s.niches);
    println!(
        "      acts urgency ever chose: [{}]\n      acts habit ever named:   [{}]",
        names(&s.urgency_acts),
        names(&s.habit_acts)
    );
    let pct = if s.with_habit == 0 {
        f64::NAN
    } else {
        s.disagreed as f64 * 100.0 / s.with_habit as f64
    };
    println!(
        "  H3  ticks with a formed habit: {} of {}   DISAGREEMENTS: {}  ({:.2}%)",
        s.with_habit, s.ticks, s.disagreed, pct
    );
    if s.with_habit == 0 {
        println!("      **VACUOUS** — no habit ever formed for an occupied niche, so H3 could not");
        println!("      have failed here. Vacuous is not passed.");
    } else if s.disagreed == 0 {
        println!("      **H3 FAILS.** History and appetite never disagree: the wire would carry");
        println!("      nothing. Do not build it on this evidence.");
    }
    // Why nothing formed: the lesson is the CHANGE in drive, and drive barely changes.
    if !s.reliefs.is_empty() {
        let mut v = s.reliefs.clone();
        v.sort_unstable();
        let teach = v.iter().filter(|&&r| r >= 3).count();
        let punish = v.iter().filter(|&&r| r <= -3).count();
        println!(
            "      relief (Δdrive): min {} median {} max {}   |  NOISE_FLOOR is ±3",
            v[0], v[v.len() / 2], v[v.len() - 1]
        );
        println!(
            "      ticks that taught anything at all: {} up, {} down, {} NOTHING ({:.1}%)",
            teach, punish, v.len() - teach - punish,
            (v.len() - teach - punish) as f64 * 100.0 / v.len() as f64
        );
    }
    if !s.strengths.is_empty() {
        let mut v = s.strengths.clone();
        v.sort_unstable();
        println!(
            "      habit strength: min {} median {} max {}",
            v[0],
            v[v.len() / 2],
            v[v.len() - 1]
        );
    }
}

fn main() {
    println!("\n=== Would habit disagree with urgency? ===");
    println!("  H1–H4 locked in docs/habits.md, committed before this file existed");

    let plain = live(false);
    let rich = live(true);
    report("default", &plain);
    report("+receptors (H4)", &rich);

    println!("\n  H4 — receptors vs default: formed {} → {}, niches {} → {}, disagreement {:.2}% → {:.2}%",
        plain.formed_final, rich.formed_final,
        plain.niches.len(), rich.niches.len(),
        plain.disagreed as f64 * 100.0 / plain.with_habit.max(1) as f64,
        rich.disagreed as f64 * 100.0 / rich.with_habit.max(1) as f64);

    println!("\n  Scope: this measures whether a wire would carry signal. It does not say the being");
    println!("  SHOULD consult its habits — a creature ruled by what worked before is a different");
    println!("  creature, and that is Blake's call.\n");
}
