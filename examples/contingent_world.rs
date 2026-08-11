//! **Contingency** — a world that remembers the being.
//!
//! **Predictions SUB-1..SUB-5 are locked in `docs/richness.md` §7 and were committed before this
//! file existed.**
//!
//! `richness.md` §4 tested **variety**: several independent movers on unrelated schedules. It
//! supplied novelty and the being was a spectator to it — every source moved regardless of what
//! the being did. This tests the other thing: **a world whose answer depends on what the being
//! did.** Three contingencies, all at the `Embodiment` seam:
//!
//! 1. **Depletion and regrowth** — feeding at the hearth draws down a local store; away from it,
//!    the store regrows. What the being ate yesterday is not there today.
//! 2. **Sensitisation** — repeated entry into the hazard raises the threat it reports; avoidance
//!    lets it decay. The world learns to punish.
//! 3. **A partner with history** — reciprocation tracks what the being actually gave, instead of
//!    sitting at a constant.
//!
//! **Consolidated 2026-08-09:** the contingency now lives in `src/room.rs` behind
//! `Room::with_contingency()` — gated, default-off, every existing world bit-identical, and
//! **the founded being at `life/being.journal` is never woken.** It was written at the
//! `Embodiment` seam here first, then moved so there is exactly one copy.
//!
//! Run: `cargo run --release --example contingent_world`

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::q88::Q88_SCALE;
use unified_being::room::Room;

const LIFE: usize = 4_000;

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

/// **The contingency now lives in `src/room.rs`** behind `Room::with_contingency()`, gated and
/// default-off, with `AMBIENT` in scope so the floor cannot be scaled away again. It was written
/// here first and then consolidated: two hand-copied versions of a world that must agree is the
/// exact drift these probes exist to catch.
fn room(contingent: bool) -> Room {
    let r = Room::peopled((32, 200), (224, 56), (128, 220), (40, 40)).with_friend((210, 128));
    if contingent { r.with_contingency() } else { r }
}

struct Lived {
    name: String,
    alive: bool,
    ticks: usize,
    /// Distinct quality points, finest grain — the register the census already reports.
    quality: Vec<i32>,
    /// Distinct habits actually in use.
    habit: Vec<i32>,
    /// Distinct attention foci, for a second read on the same question.
    focus: Vec<i32>,
    /// `habits.formed` read directly, because "2 distinct values of `Option<u8>`" is
    /// `{None, Some(x)}` — **one** habit, not two, and the count must not be inferred.
    formed: u16,
}

/// **The suspicion check.** A 25× jump in distinct quality points is the direction I have argued
/// all week, which is exactly when to distrust it. A slowly drifting input — the hearth store
/// ramping 256→0→256 — would make almost every tick's point unique *without the being doing
/// anything new*. Genuine exploration settles: novelty falls as the being finds its orbit. A drift
/// artifact does not: novelty stays flat to the last tick.
fn novelty_by_quarter(v: &[i32]) -> [f64; 4] {
    let mut seen: Vec<i32> = Vec::new();
    let mut out = [0.0; 4];
    let q = (v.len() / 4).max(1);
    for (qi, chunk) in v.chunks(q).take(4).enumerate() {
        let mut fresh = 0usize;
        for &p in chunk {
            if seen.binary_search(&p).is_err() {
                let at = seen.partition_point(|&x| x < p);
                seen.insert(at, p);
                fresh += 1;
            }
        }
        out[qi] = fresh as f64 / chunk.len() as f64;
    }
    out
}

fn distinct(v: &[i32]) -> usize {
    let mut s = v.to_vec();
    s.sort_unstable();
    s.dedup();
    s.len()
}

fn live(name: &str, contingent: bool, gates: fn(&mut UnifiedBeing)) -> Lived {
    let mut b = UnifiedBeing::new(unified_being::genome::Genome::wanderer());
    gates(&mut b);

    let mut world = room(contingent);

    let mut l = Lived {
        name: name.to_string(),
        alive: true,
        ticks: 0,
        quality: Vec::with_capacity(LIFE),
        habit: Vec::with_capacity(LIFE),
        focus: Vec::with_capacity(LIFE),
        formed: 0,
    };

    for _ in 0..LIFE {
        let sens = world.sense();
        let r = b.step_embodied(&sens);
        world.actuate(&intent_from(&r));
        world.remember(r.gave, r.got);

        let a = r.quality.point.axis;
        l.quality.push((a[0] as i32) * 1_000_000 + (a[1] as i32) * 10_000 + (a[2] as i32) * 100 + a[3] as i32);
        l.habit.push(r.habits.habit.map_or(-1, |h| h as i32));
        l.focus.push(r.attention.attended.map_or(-1, |c| c as i32));
        l.formed = r.habits.formed;

        l.ticks += 1;
        if !r.alive {
            l.alive = false;
            break;
        }
    }
    l
}

fn bless(b: &mut UnifiedBeing) {
    b.enable_felt_choice();
    b.enable_precision_learning();
    b.enable_generative_perception();
    b.enable_workspace_persistence();
}

fn all_loops(b: &mut UnifiedBeing) {
    bless(b);
    b.enable_schema_control();
    b.enable_serial_access();
    b.enable_workspace_broadcast();
    b.enable_reflection();
    b.enable_memory_guidance();
}

fn main() {
    println!("\n=== Contingency — a world that remembers the being ===");
    println!("  predictions SUB-1..SUB-5 locked in docs/richness.md §7, committed first");
    println!("  §4 tested VARIETY. This tests whether the world's answer depends on what the being did.\n");

    let runs = [
        live("blessed  / static", false, bless),
        live("blessed  / contingent", true, bless),
        live("bare     / static", false, |_| {}),
        live("bare     / contingent", true, |_| {}),
        live("all-loops/ static", false, all_loops),
        live("all-loops/ contingent", true, all_loops),
    ];

    // ---- survival FIRST. A regime that died early has a small denominator. ----
    println!("  {:<24} {:>7}  {:>9}", "regime", "ticks", "survived");
    for r in &runs {
        println!("  {:<24} {:>7}  {:>9}", r.name, r.ticks, if r.alive { "yes" } else { "DIED" });
    }
    let deaths = runs.iter().filter(|r| !r.alive).count();

    println!(
        "\n  {:<24} {:>9} {:>8} {:>7} {:>6}",
        "regime", "quality", "formed", "in-use", "foci"
    );
    for r in &runs {
        println!(
            "  {:<24} {:>9} {:>8} {:>7} {:>6}",
            r.name,
            distinct(&r.quality),
            r.formed,
            distinct(&r.habit).saturating_sub(1), // minus the None value
            distinct(&r.focus)
        );
    }

    // ---- the suspicion check: exploration settles, drift does not ----
    println!("\n  novelty per tick, by quarter of life — a DRIFT artifact stays flat:");
    println!("  {:<24} {:>8} {:>8} {:>8} {:>8}", "regime", "Q1", "Q2", "Q3", "Q4");
    for r in &runs {
        let n = novelty_by_quarter(&r.quality);
        println!(
            "  {:<24} {:>7.2} {:>8.2} {:>8.2} {:>8.2}",
            r.name, n[0], n[1], n[2], n[3]
        );
    }

    let bs = distinct(&runs[0].quality);
    let bc = distinct(&runs[1].quality);
    let bare_c = distinct(&runs[3].quality);
    let loops_s = distinct(&runs[4].quality);
    let habit_c = distinct(&runs[1].habit);

    println!("\n  --- the locked predictions ---");
    println!(
        "  SUB-1  habits ≥2 in contingent? ............ {}   ({habit_c}, baseline 1)",
        verdict(habit_c >= 2)
    );
    println!(
        "  SUB-2  quality ≥306 in contingent? ......... {}   ({bc}, baseline {bs})",
        verdict(bc >= 306)
    );
    println!(
        "  SUB-3  bare/contingent > all-loops/static? . {}   ({bare_c} vs {loops_s})",
        verdict(bare_c > loops_s)
    );
    println!(
        "  SUB-4  at least one death? ................. {}   ({deaths} of {})",
        verdict(deaths > 0),
        runs.len()
    );
    println!("  SUB-5  slow body — NOT RUN, no such gate exists. Carried forward.");

    if deaths == 0 {
        println!(
            "\n  ** SUB-4 failed: nothing died. The world still exerts no selection pressure,\n     \
             and SUB-1..3 must be read against that.**"
        );
    }
    println!();
}

fn verdict(held: bool) -> &'static str {
    if held {
        "HOLDS"
    } else {
        "FAILED"
    }
}
