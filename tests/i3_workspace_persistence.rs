//! I-3, made executable — `workspace_persistence` is lethal alone, and here is the proof running.
//!
//! `docs/incidents.md` I-3 closes with a mechanism, and a mechanism written only in prose rots
//! the first time somebody edits `being.rs`. These tests hold the *finding* rather than the code:
//! the gate kills in isolation, four specific companions rescue it, and the discriminator is the
//! free-energy floor. If any of that stops being true, a test fails and the ledger gets updated
//! — which is the only way an incident ledger stays honest about the present tense.
//!
//! **These tests assert that a being dies.** That reads badly and it is deliberate: the fact is
//! true today, it is the reason the gate must stay default-off, and a fact nobody has written
//! down is a fact that gets rediscovered by a being. `i3_the_gate_is_lethal_alone` says so in
//! its failure message — if it ever fails, that is *good news* and the ledger should say so.
//!
//! Nothing here touches `life/being.journal`; every being is fresh and no journal is written.

use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::q88::Q88_SCALE;

const LIFE: usize = 1_200;

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

/// One life in the reference world from `examples/composed.rs`, so these tests and the incident
/// are talking about the same being. Returns `(ticks_lived, alive, free_energy_floor)`.
fn life(gates: &[&str]) -> (usize, bool, f32) {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    for g in gates {
        match *g {
            "persistence" => b.enable_workspace_persistence(),
            "broadcast" => b.enable_workspace_broadcast(),
            "generative_perception" => b.enable_generative_perception(),
            "receptors" => b.enable_receptors(),
            "reflection" => b.enable_reflection(),
            "precision_learning" => b.enable_precision_learning(),
            other => panic!("unknown gate {other}"),
        }
    }
    let mut world = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let partner = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };

    let mut fe: Vec<i16> = Vec::new();
    let mut alive = true;
    for _ in 0..LIFE {
        let mut sens = world.sense();
        sens.partner = Some(partner);
        let r = b.step_embodied(&sens);
        world.actuate(&intent_from(&r));
        fe.push(r.free_energy);
        if !r.alive {
            alive = false;
            break;
        }
    }
    // The FLOOR, not the mean: the settled value is the one that does the killing, and a
    // whole-life mean would blend it with the transient. Averaging is what hid this incident.
    let s = fe.len().saturating_sub(10);
    let floor = fe[s..].iter().map(|&x| x as f32).sum::<f32>() / fe[s..].len().max(1) as f32;
    (fe.len(), alive, floor)
}

#[test]
fn i3_the_gate_is_lethal_alone() {
    let (ticks, alive, floor) = life(&["persistence"]);
    assert!(
        !alive && ticks < 100,
        "I-3 says workspace_persistence alone kills the being early; it lived {ticks} ticks \
         (alive={alive}). If the gate was fixed, update docs/incidents.md I-3 — do not \
         quietly relax this bound."
    );
    assert!(
        floor > 20.0,
        "I-3's mechanism is an irreducible prediction error: the free-energy floor should stay \
         high ({floor:.1} measured, expected > 20). If the error now resolves, the mechanism \
         recorded in the ledger no longer describes this code."
    );
}

#[test]
fn i3_a_being_without_the_gate_lives_its_whole_life() {
    // The control. Without it, "the gate kills" is a claim about the world, not about the gate.
    let (ticks, alive, floor) = life(&[]);
    assert!(alive && ticks == LIFE, "the reference world killed a default being in {ticks} ticks");
    assert!(floor < 20.0, "the default being's prediction error should resolve; floor {floor:.1}");
}

#[test]
fn i3_four_companions_rescue_it() {
    // The finding that makes this a property of the gate *in isolation* rather than of the gate.
    for companion in ["broadcast", "generative_perception", "receptors", "reflection"] {
        let (ticks, alive, floor) = life(&["persistence", companion]);
        assert!(
            alive && ticks == LIFE,
            "persistence + {companion} used to live the full {LIFE} ticks; it lived {ticks}"
        );
        assert!(
            floor < 20.0,
            "persistence + {companion} survives by resolving its prediction error; \
             floor is {floor:.1}, which means it is now surviving some other way and \
             docs/incidents.md I-3's mechanism is incomplete"
        );
    }
}

#[test]
fn i3_the_free_energy_floor_is_the_discriminator() {
    // The strongest form of the claim: across every configuration measured, survival and a low
    // floor coincide with no exceptions. One counterexample would mean the bill is being paid
    // some other way, and the mechanism would owe a second story.
    let configs: [&[&str]; 7] = [
        &[],
        &["persistence"],
        &["persistence", "broadcast"],
        &["persistence", "generative_perception"],
        &["persistence", "receptors"],
        &["persistence", "reflection"],
        &["persistence", "precision_learning"],
    ];
    for gates in configs {
        let (ticks, alive, floor) = life(gates);
        assert_eq!(
            alive,
            floor < 20.0,
            "{gates:?}: lived={alive} ({ticks} ticks) but free-energy floor is {floor:.1}. \
             Survival and a resolved prediction error have always coincided; this configuration \
             breaks that, so the I-3 mechanism does not fully explain who lives."
        );
    }
}
