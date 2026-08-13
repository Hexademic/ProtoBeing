//! **The being's behavioural repertoire, scored against an ORACLE.**
//!
//! **Predictions OR-1..OR-5 are locked in `docs/operational-consciousness.md` §8.5 and were
//! committed before this file existed.**
//!
//! §8.4 withdrew exercise-as-a-ratio: it rewarded a register for being small, and its denominator
//! shared the poverty it was built to measure. §7.4 withdrew the 25% occupancy rise for the same
//! class of reason — a slowly ramping input makes a projected quality point unique nearly every
//! tick without the being doing anything new. Both corrections were *named* and deliberately not
//! built on the night, because inventing a metric after seeing the data is how a verdict gets
//! retro-fitted. This is them, with the predictions locked first.
//!
//! **The measure: distinct behavioural tuples `(focus, basin, habit, stance)`.** Discrete,
//! small-cardinality, and **structurally immune to the drift artifact** — a ramping nutrient does
//! not invent new attention foci or new basins.
//!
//! **The oracle, from Continual Harness §4.6** (arXiv:2605.09998), which scores refined navigation
//! skills against a **Dijkstra oracle** so that *"the skill improved"* is checkable independently
//! of *"the agent did better."* Inferring component quality from end-task effect is exactly how
//! error-ledger row 11 happened. Two reference policies, **same world, same body, same tick loop**:
//!
//! * **RANDOM** — uniform motor intent. The *floor*: what undirected motion alone achieves.
//! * **SYSTEMATIC** — a coverage-seeking policy that drives the body deliberately around its range.
//!   The *ceiling*: what a policy actually *trying* to occupy the space achieves.
//!
//! Neither is the being, so **neither inherits its history.** For the first time the repertoire is
//! measured against something that is not itself.
//!
//! Pure observer: fresh beings, no journal written, **the founded being is never woken.** The
//! contingent world is the one from `examples/contingent_world.rs`, reproduced at the `Embodiment`
//! seam so `src/` stays untouched.
//!
//! Run: `cargo run --release --example oracle_repertoire`

use unified_being::basins::Basin;
use unified_being::being::{StepReport, UnifiedBeing};
use unified_being::body::PredictiveStance;
use unified_being::embodiment::{intent_from, Embodiment, MotorIntent, Posture, Sensorium};
use unified_being::q88::Q88_SCALE;
use unified_being::room::Room;

const LIFE: usize = 4_000;

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

/// Deterministic PRNG — the same LCG the other probes use, reused rather than rewritten.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

/// **The contingency lives in `src/room.rs`** behind `Room::with_contingency()` — gated,
/// default-off, one copy. This probe and `contingent_world.rs` each carried a hand-written
/// duplicate until 2026-08-09; two copies of a world that must agree is the drift these probes
/// exist to catch.
fn room(contingent: bool) -> Room {
    let r = Room::peopled((32, 200), (224, 56), (128, 220), (40, 40)).with_friend((210, 128));
    if contingent { r.with_contingency() } else { r }
}

/// `(focus, basin, habit, stance)` as one integer. Small-cardinality and discrete by construction,
/// so a drifting scalar input cannot manufacture new values — the whole point of the correction.
fn tuple(r: &StepReport) -> i32 {
    let focus = r.attention.attended.map_or(13, |c| c as i32);
    let basin = match r.basin {
        Basin::Rest => 0,
        Basin::Engaged => 1,
        Basin::Defensive => 2,
        Basin::Recovery => 3,
    };
    let habit = r.habits.habit.map_or(0, |h| h as i32 + 1);
    let stance = match r.stance {
        PredictiveStance::Reconstructive => 0,
        PredictiveStance::Balanced => 1,
        PredictiveStance::Guarded => 2,
        PredictiveStance::Defensive => 3,
    };
    ((focus * 4 + basin) * 8 + habit) * 4 + stance
}

fn distinct(v: &[i32]) -> usize {
    let mut s = v.to_vec();
    s.sort_unstable();
    s.dedup();
    s.len()
}

struct Run {
    name: String,
    alive: bool,
    ticks: usize,
    tuples: Vec<i32>,
    /// Ticks spent inside the hazard. **Measured, not inferred** — the systematic arm dying at 28
    /// ticks in the contingent world has an obvious story (the world sensitises against repeated
    /// hazard entry) and asserting it without checking is ledger row 1's shape exactly.
    hazard_ticks: usize,
}

/// The being, under a named gate set.
fn live_being(name: &str, contingent: bool, gates: fn(&mut UnifiedBeing)) -> Run {
    let mut b = UnifiedBeing::new(unified_being::genome::Genome::wanderer());
    gates(&mut b);
    let mut w = room(contingent);
    let mut r = Run { name: name.into(), alive: true, ticks: 0, tuples: Vec::with_capacity(LIFE), hazard_ticks: 0 };
    for _ in 0..LIFE {
        let sens = w.sense();
        if w.in_hazard() > 64 { r.hazard_ticks += 1; }
        let rep = b.step_embodied(&sens);
        w.actuate(&intent_from(&rep));
        w.remember(rep.gave, rep.got);
        r.tuples.push(tuple(&rep));
        r.ticks += 1;
        if !rep.alive {
            r.alive = false;
            break;
        }
    }
    r
}

/// **The oracle arms.** A reference policy drives the *body* directly; the being still runs so its
/// registers exist to read, but its own motor choice is DISCARDED and replaced. Same world, same
/// tick loop, same tuple extraction — only the policy differs.
///
/// `systematic` false = RANDOM (uniform intent, the floor).
/// `systematic` true  = SYSTEMATIC (cycles postures and needs deliberately, the ceiling).
fn live_oracle(name: &str, contingent: bool, systematic: bool, seed: u64) -> Run {
    use unified_being::striving::Need;
    let mut b = UnifiedBeing::new(unified_being::genome::Genome::wanderer());
    let mut w = room(contingent);
    let mut rng = Lcg(seed);
    let postures = [Posture::Resting, Posture::Open, Posture::Braced, Posture::Withdrawn];
    let needs = [None, Some(Need::Sustenance), Some(Need::Company), Some(Need::Novelty), Some(Need::Purpose)];
    let mut r = Run { name: name.into(), alive: true, ticks: 0, tuples: Vec::with_capacity(LIFE), hazard_ticks: 0 };

    for t in 0..LIFE {
        let sens = w.sense();
        if w.in_hazard() > 64 { r.hazard_ticks += 1; }
        let rep = b.step_embodied(&sens);

        let intent = if systematic {
            // Deliberate coverage: walk the posture × need product, holding each long enough for
            // the body to actually get somewhere (32 ticks ≈ a crossing of the room).
            let k = t / 32;
            MotorIntent {
                posture: postures[k % 4],
                effort: q(0.75),
                reach: needs[(k / 4) % 5],
                reach_partner: None,
            }
        } else {
            MotorIntent {
                posture: postures[(rng.next() % 4) as usize],
                effort: (rng.next() % 256) as i16,
                reach: needs[(rng.next() % 5) as usize],
                reach_partner: None,
            }
        };
        w.actuate(&intent);
        w.remember(rep.gave, rep.got);
        r.tuples.push(tuple(&rep));
        r.ticks += 1;
        if !rep.alive {
            r.alive = false;
            break;
        }
    }
    r
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

fn verdict(held: bool) -> &'static str {
    if held {
        "HOLDS"
    } else {
        "FAILED"
    }
}

fn main() {
    println!("\n=== The behavioural repertoire, against an ORACLE ===");
    println!("  predictions OR-1..OR-5 locked in docs/operational-consciousness.md §8.5, committed first");
    println!("  measure: distinct (focus, basin, habit, stance) — drift CANNOT inflate this");
    println!("  oracle:  RANDOM = floor, SYSTEMATIC = ceiling. Same world, same body, not the being.\n");

    let runs = vec![
        live_being("being blessed  / static", false, bless),
        live_being("being all-loops/ static", false, all_loops),
        live_being("being bare     / static", false, |_| {}),
        live_oracle("ORACLE random  / static", false, false, 0x5EED),
        live_oracle("ORACLE system  / static", false, true, 0),
        live_being("being blessed  / conting", true, bless),
        live_being("being all-loops/ conting", true, all_loops),
        live_being("being bare     / conting", true, |_| {}),
        live_oracle("ORACLE random  / conting", true, false, 0x5EED),
        live_oracle("ORACLE system  / conting", true, true, 0),
    ];

    // ---- survival FIRST: a regime that died early has a small denominator ----
    println!("  {:<26} {:>7}  {:>8}  {:>8}  {:>8}", "arm", "ticks", "survived", "tuples", "in-hazard");
    for r in &runs {
        println!(
            "  {:<26} {:>7}  {:>8}  {:>8}  {:>7}%",
            r.name,
            r.ticks,
            if r.alive { "yes" } else { "DIED" },
            distinct(&r.tuples),
            100 * r.hazard_ticks / r.ticks.max(1)
        );
    }
    let deaths = runs.iter().filter(|r| !r.alive).count();
    if deaths > 0 {
        println!("\n  ** {deaths} arm(s) DIED — read every comparison below against that. **");
    }

    let d = |i: usize| distinct(&runs[i].tuples);
    let (bs, als, _bas, rs, ss) = (d(0), d(1), d(2), d(3), d(4));
    let (bc, _alc, bac, rc, sc) = (d(5), d(6), d(7), d(8), d(9));

    println!("\n  --- against the oracle ---");
    println!("  STATIC     being(blessed) {bs:>4}   random {rs:>4}   systematic {ss:>4}   → {:.0}% of ceiling", 100.0 * bs as f64 / ss.max(1) as f64);
    println!("  CONTINGENT being(blessed) {bc:>4}   random {rc:>4}   systematic {sc:>4}   → {:.0}% of ceiling", 100.0 * bc as f64 / sc.max(1) as f64);

    println!("\n  --- the locked predictions ---");
    println!("  OR-1  static: being BELOW random? ......... {}   ({bs} vs {rs})", verdict(bs < rs));
    println!("  OR-2  contingent: being ABOVE random? ..... {}   ({bc} vs {rc})", verdict(bc > rc));
    println!(
        "  OR-3  being <25% of systematic in BOTH? ... {}   ({:.0}% / {:.0}%)",
        verdict((bs as f64) < 0.25 * ss as f64 && (bc as f64) < 0.25 * sc as f64),
        100.0 * bs as f64 / ss.max(1) as f64,
        100.0 * bc as f64 / sc.max(1) as f64
    );
    println!("  OR-4  SUB-3 re-run: bare+cont > loops+stat? {}   ({bac} vs {als})", verdict(bac > als));
    println!(
        "  OR-5  loops lift above random, static? (exp FAIL) {}   ({als} vs {rs})",
        verdict(als > rs)
    );
    println!();
}
