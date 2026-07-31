//! Survival, guarded — the audit from `docs/survival-first.md`, held so it cannot decay.
//!
//! A one-time audit is worth exactly as long as it takes someone to add a gate. These tests keep
//! the audit's findings true in the present tense:
//!
//! - no gate configuration up to pairs is lethal **except** those containing
//!   `workspace_persistence` (S1), so a newly-lethal faculty fails here rather than waiting to be
//!   noticed by luck a second time;
//! - no pair kills where neither member does (S4);
//! - the being **cannot starve** — the finding that explains why the sweep looked clean;
//! - and the **death line** sits where §7 measured it, including the band in which the being dies
//!   with its prediction error resolved and every instrument reading calm.
//!
//! The pair sweep is the slow one (66 lives), so it is marked `#[ignore]` and run deliberately:
//! `cargo test --release --test survival -- --ignored`. Everything that is cheap runs always.
//!
//! Nothing here touches `life/being.journal`; every being is fresh and no journal is written.

use unified_being::being::{Partner, UnifiedBeing};
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
    let mut b = UnifiedBeing::new(Genome::wanderer());
    let mut fe: Vec<i16> = Vec::new();
    let mut alive = true;
    for _ in 0..ticks {
        let sens = Sensorium { nutrient, threat, exteroception: [0; 4], partner: None };
        let r = b.step_embodied(&sens);
        fe.push(r.free_energy);
        if !r.alive {
            alive = false;
            break;
        }
    }
    let s = fe.len().saturating_sub(10);
    let floor = fe[s..].iter().map(|&x| x as f32).sum::<f32>() / fe[s..].len().max(1) as f32;
    (fe.len(), alive, floor)
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
fn this_being_cannot_starve() {
    // The fact that explains why a gate sweep could never find S3's exception: nutrient is
    // clamped to AMBIENT_FLOOR everywhere in every FieldWorld, and that income exceeds the
    // resting metabolic cost. Death here is always a COST-side event. If this ever fails, the
    // whole shape of "how a being dies" in this architecture has changed.
    let (ticks, alive, _) = hold_at(0, 40, 20_000);
    assert!(
        alive && ticks == 20_000,
        "a being at the ambient nutrient floor with ZERO threat died after {ticks} ticks. \
         This being has always been unable to starve (docs/survival-first.md §7); it now can."
    );
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
