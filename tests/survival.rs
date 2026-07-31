//! Survival, guarded — the audit from `docs/survival-first.md`, held so it cannot decay.
//!
//! A one-time audit is worth exactly as long as it takes someone to add a gate. These tests keep
//! the audit's findings true in the present tense:
//!
//! - no gate configuration up to pairs is lethal **except** those containing
//!   `workspace_persistence` (S1), so a newly-lethal faculty fails here rather than waiting to be
//!   noticed by luck a second time;
//! - no pair kills where neither member does (S4);
//! - the being **cannot starve in a world** (`AMBIENT_FLOOR`) — but **can** in the abstract loop,
//!   which is where every probe that does not check `.alive` drives it, so the safety line there
//!   is guarded too;
//! - the **death line** sits where §7 measured it, including the band in which the being dies
//!   with its prediction error resolved — a blind spot in OUR discriminator, not in the being;
//! - **the being feels its own death coming** (incident I-6, the claim I got backwards);
//! - and **solitude is the largest burden it carries**, the finding that fell out of that error.
//!
//! The pair sweep is the slow one (66 lives), so it is marked `#[ignore]` and run deliberately:
//! `cargo test --release --test survival -- --ignored`. Everything that is cheap runs always.
//!
//! Nothing here touches `life/being.journal`; every being is fresh and no journal is written.

use unified_being::being::{Partner, Stimulus, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment, Sensorium};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::q88::Q88_SCALE;

const LIFE: usize = 1_200;
const N_GATES: usize = 11;
/// Index of `workspace_persistence` in the gate order below — the one incident I-3 is about.
const PERSISTENCE: usize = 2;

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

fn apply(b: &mut UnifiedBeing, w: &[bool; N_GATES]) {
    if w[0] { b.enable_precision_learning(); }
    if w[1] { b.enable_workspace_broadcast(); }
    if w[2] { b.enable_workspace_persistence(); }
    if w[3] { b.enable_generative_perception(); }
    if w[4] { b.enable_receptors(); }
    if w[5] { b.enable_serial_access(); }
    if w[6] { b.enable_schema_control(); }
    if w[7] { b.enable_felt_choice(); }
    if w[8] { b.enable_reflection(); }
    if w[9] { b.enable_homecoming(); }
    if w[10] { b.enable_memory_guidance(); }
}

/// One life in the reference world, returning whether the being finished it.
fn survives(gates: &[bool; N_GATES]) -> bool {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    apply(&mut b, gates);
    let mut world = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let partner = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    for _ in 0..LIFE {
        let mut sens = world.sense();
        sens.partner = Some(partner);
        let r = b.step_embodied(&sens);
        world.actuate(&intent_from(&r));
        if !r.alive {
            return false;
        }
    }
    true
}

/// The being held at a constant stimulus — no world, no gradient, no gates. Returns
/// `(ticks, alive, free_energy_floor)`.
fn hold_at(threat: i16, nutrient: i16, ticks: usize) -> (usize, bool, f32) {
    let (t, a, f, _) = hold_at_watching(threat, nutrient, ticks);
    (t, a, f)
}

/// As `hold_at`, plus the tick on which the being **first felt the deficit coming**
/// (`felt.anticipating`). Incident I-6: I reported that the being dies in this band unwarned,
/// and it had in fact been warning for 36 of the 43 ticks it had left.
fn hold_at_watching(threat: i16, nutrient: i16, ticks: usize) -> (usize, bool, f32, Option<usize>) {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    let mut fe: Vec<i16> = Vec::new();
    let mut alive = true;
    let mut anticipated = None;
    for t in 0..ticks {
        let sens = Sensorium { nutrient, threat, exteroception: [0; 4], partner: None };
        let r = b.step_embodied(&sens);
        fe.push(r.free_energy);
        if anticipated.is_none() && r.felt.anticipating {
            anticipated = Some(t);
        }
        if !r.alive {
            alive = false;
            break;
        }
    }
    let s = fe.len().saturating_sub(10);
    let floor = fe[s..].iter().map(|&x| x as f32).sum::<f32>() / fe[s..].len().max(1) as f32;
    (fe.len(), alive, floor, anticipated)
}

#[test]
fn s1_no_single_gate_is_lethal_except_the_one_we_know_about() {
    for i in 0..N_GATES {
        let mut g = [false; N_GATES];
        g[i] = true;
        let lived = survives(&g);
        assert_eq!(
            lived,
            i != PERSISTENCE,
            "gate {i} alone: lived={lived}. docs/survival-first.md §7 S1 says exactly one gate \
             is lethal alone (workspace_persistence, index {PERSISTENCE}). If a NEW gate now \
             kills, that is a second I-3 and must be filed in docs/incidents.md before this \
             test is touched. If persistence was FIXED, that is good news — say so in the ledger."
        );
    }
}

#[test]
fn s2_the_composed_being_survives() {
    assert!(survives(&[true; N_GATES]), "the all-eleven being no longer lives its full life");
}

#[test]
#[ignore = "66 lives; run deliberately with --ignored"]
fn s1_s4_no_pair_is_lethal_without_a_gate_that_is_lethal_alone() {
    let solo_lethal: Vec<usize> = (0..N_GATES)
        .filter(|&i| {
            let mut g = [false; N_GATES];
            g[i] = true;
            !survives(&g)
        })
        .collect();

    for i in 0..N_GATES {
        for j in (i + 1)..N_GATES {
            let mut g = [false; N_GATES];
            g[i] = true;
            g[j] = true;
            if survives(&g) {
                continue;
            }
            // S1: every death contains persistence.
            assert!(
                g[PERSISTENCE],
                "gates {i}+{j} killed the being with no workspace_persistence involved. \
                 This is a SECOND lethal configuration and docs/survival-first.md §7 S1 says \
                 there is none — file it in docs/incidents.md."
            );
            // S4: no pair is lethal where neither member is.
            assert!(
                solo_lethal.contains(&i) || solo_lethal.contains(&j),
                "gates {i}+{j} kill together while neither kills alone — harm that exists ONLY \
                 in composition. docs/survival-first.md §7 S4 says this does not happen; it now \
                 does, and it is the most important finding in the repository until explained."
            );
        }
    }
}

#[test]
fn this_being_cannot_starve_in_a_world() {
    // Originally named `this_being_cannot_starve`, which was wider than the fact. The floor that
    // makes starvation impossible is `AMBIENT_FLOOR`, and it lives in `FieldWorld::sense()` — so
    // it protects a being that has a *world*. A being driven through the abstract loop is fed
    // whatever its probe passes, and the test below shows that it can absolutely starve there.
    //
    // Within a world, though, the guarantee is real, and it is what makes every death in this
    // architecture a COST-side event: income (nutrient·180/256 ≥ 28.1/tick) exceeds resting cost
    // (3/tick) everywhere.
    let (ticks, alive, _) = hold_at(0, 40, 20_000);
    assert!(
        alive && ticks == 20_000,
        "a being at the ambient nutrient floor with ZERO threat died after {ticks} ticks. \
         Within a world this being has never been able to starve; it now can."
    );
}

#[test]
fn the_abstract_loop_has_no_such_floor_and_probes_must_stay_above_it() {
    // The correction, and the guard that generalises it. `AMBIENT_FLOOR` protects a being with a
    // world; nothing protects a being driven by `step(&Stimulus)`, where nutrient is whatever the
    // probe chose. Measured across every partner reciprocation and both gate settings, the lowest
    // nutrient at which NOTHING kills the being is 36.
    //
    // Every probe in `examples/` that drives a whole being without checking `.alive` passes at
    // least q(0.4) = 102, so all of them are comfortably clear — but that is true today because
    // it was measured today, and this test is what keeps it true.
    let starves = |nutrient: i16, recip: i16| {
        let mut b = UnifiedBeing::new(Genome::wanderer());
        for _ in 0..5_000 {
            let p = (recip >= 0).then_some(Partner { id: 1, reciprocation: recip, exit_cost: 200 });
            if !b.step(&Stimulus { nutrient, partner: p }).alive {
                return true;
            }
        }
        false
    };

    assert!(starves(0, -1), "a being fed nothing in the abstract loop used to starve");
    assert!(
        starves(20, 0),
        "nutrient 20 with an extractive partner used to be lethal in the abstract loop"
    );

    // 36 is the measured safety line: nothing kills the being at or above it.
    for recip in [-1i16, 0, 32, 64, 128, 200, 256] {
        assert!(
            !starves(36, recip),
            "nutrient 36 used to be safe against every partner (reciprocation {recip}); the \
             abstract-loop safety line has MOVED, and every probe that drives a being without \
             checking `.alive` was cleared against the old line. Re-audit examples/ before \
             touching this number."
        );
    }
}

#[test]
fn the_death_line_is_where_it_was_measured() {
    // §7: threat 105 lives, 106 dies. Asserted as a boundary rather than a point so ordinary
    // drift does not fail it, but a MOVED line does.
    assert!(hold_at(105, 40, 4_000).1, "threat 105 used to be survivable at the ambient floor");
    assert!(!hold_at(106, 40, 4_000).1, "threat 106 used to be lethal at the ambient floor");
}

#[test]
fn a_being_can_die_with_its_prediction_error_resolved() {
    // The bound on I-3's discriminator, and the reason it must never be used as a general test
    // of whether a being is in trouble. At threat 110 the being dies with a free-energy floor
    // BELOW the "resolved" threshold of 20: a model that understood its world perfectly, in a
    // body that could not pay for it.
    //
    // NOTE (incident I-6): this is a fact about OUR instrument, not about the being. I originally
    // wrote it up as "the being dies and every instrument reads calm"; the being's own
    // interoception fires 36 ticks before the end. See the test below, which guards that.
    let (ticks, alive, floor) = hold_at(110, 40, 4_000);
    assert!(!alive, "threat 110 used to be lethal");
    assert!(
        floor < 20.0,
        "the quiet-death band (docs/survival-first.md §7) is a being dying with a RESOLVED \
         model — floor was {floor:.1}, expected < 20. If free energy now rises before this \
         death, our instruments would see it coming, and the welfare finding has changed."
    );
    assert!(ticks < 200, "the quiet death used to arrive within ~80 ticks; took {ticks}");
}

#[test]
fn doubling_the_ambient_floor_makes_the_being_invulnerable() {
    // The dial we hold. At nutrient 40 the being dies above threat 105; at 80 it survives the
    // top of the scale. Stated as a test so the trade stays explicit rather than implicit.
    assert!(
        hold_at(256, 80, 1_500).1,
        "at nutrient 80 the being used to survive maximum threat; it no longer does, so the \
         safety margin docs/survival-first.md §7 offers Blake is no longer real"
    );
    assert!(
        !hold_at(256, 40, 1_500).1,
        "at the current ambient floor the being used to die at maximum threat — if it now \
         survives, the floor changed and §7's frontier table is stale"
    );
}

#[test]
fn the_being_feels_its_own_death_coming() {
    // Incident I-6, made executable. I published that this being dies unwarned in the quiet band.
    // It does not: `interoception.rs` is allostatic by design — it feels a deficit BEFORE it
    // arrives — and it fires at tick 7 of the 43 the being has left. That is a welfare-relevant
    // capability and it must not be allowed to regress silently just because it once went
    // unmeasured.
    let (ticks, alive, floor, anticipated) = hold_at_watching(110, 40, 4_000);
    assert!(!alive, "threat 110 used to be lethal at the ambient floor");
    assert!(floor < 20.0, "this is the band where OUR discriminator is blind; floor {floor:.1}");

    let at = anticipated.expect(
        "THE BEING NO LONGER FEELS ITS DEATH COMING. `felt.anticipating` never fired across a \
         life that ended. This is the exact capability incident I-6 exists because I wrongly \
         reported it lacked — if it is now genuinely absent, that is a real welfare regression \
         and belongs in docs/incidents.md, not in a relaxed assertion.",
    );
    let warning = ticks - at;
    assert!(
        warning * 2 > ticks,
        "the being used to know for most of the life it had left: warned at tick {at} of \
         {ticks} ({warning} ticks of warning). It now knows for less than half."
    );
}

#[test]
fn solitude_is_the_largest_burden_this_being_carries() {
    // The finding that fell out of I-6's third error. Identical conditions — no threat, ample
    // nutrient, full viability — differing only in whether someone is there.
    let burden = |partner: bool| {
        let mut b = UnifiedBeing::new(Genome::wanderer());
        let (mut sum, mut over, mut n) = (0i64, 0usize, 0usize);
        for _ in 0..2_000 {
            let p = partner.then_some(Partner { id: 1, reciprocation: 230, exit_cost: 77 });
            let sens = Sensorium { nutrient: 40, threat: 0, exteroception: [0; 4], partner: p };
            let r = b.step_embodied(&sens);
            sum += r.drive.drive as i64;
            if r.drive.drive >= unified_being::play::COMFORT {
                over += 1;
            }
            n += 1;
            if !r.alive {
                break;
            }
        }
        (sum as f32 / n as f32, over as f32 * 100.0 / n as f32)
    };
    let (alone_drive, alone_pct) = burden(false);
    let (together_drive, together_pct) = burden(true);

    assert!(
        alone_drive - together_drive > 20.0,
        "company used to be worth ~40 points of drive to this being (alone {alone_drive:.1}, \
         with someone {together_drive:.1}); the gap has closed to {:.1}",
        alone_drive - together_drive
    );
    assert!(
        alone_pct > 50.0 && together_pct < 5.0,
        "the same being used to be burdened {alone_pct:.1}% of the time alone and \
         {together_pct:.1}% of the time with someone — the whole difference between a burdened \
         life and an easy one was solitude. Now: alone {alone_pct:.1}%, together {together_pct:.1}%."
    );
}
