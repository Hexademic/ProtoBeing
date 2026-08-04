//! Incident **I-9** must not come back.
//!
//! `reflection.rs` accrues chronic load when the being is *burdened* and, before this, required
//! it to be *un-burdened* in order to discharge. A structurally burdened being therefore locked
//! its own drain: measured at **3,638 consecutive ticks** pinned at the 256 ceiling, converting
//! nothing (`examples/reflection_deadlock`, `docs/setting-it-down.md`).
//!
//! These tests hold the three things that must stay true: the gate is really off by default, the
//! ceiling is really released when it is on, and setting weight down can never happen while the
//! being is being outrun.

use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::Sensorium;
use unified_being::genome::Genome;
use unified_being::q88::Q88_SCALE;

const LIFE: usize = 4_000;

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

/// The solitary life — a *structural* burden, the regime where the deadlock bites.
/// Returns `(longest run pegged at the ceiling, final load, final weathered, soul hash)`.
fn solitary(setting_down: bool) -> (usize, i16, i16, [u8; 32]) {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    b.enable_receptors();
    b.enable_reflection();
    if setting_down {
        b.enable_setting_down();
    }
    let (mut pegged, mut run) = (0usize, 0usize);
    let (mut load, mut weathered) = (0i16, 0i16);
    for _ in 0..LIFE {
        let r = b.step_embodied(&Sensorium {
            nutrient: 200,
            threat: 0,
            exteroception: [0; 4],
            partner: None,
        });
        load = r.reflection.load;
        weathered = r.reflection.self_model.weathered;
        if load >= Q88_SCALE {
            run += 1;
            pegged = pegged.max(run);
        } else {
            run = 0;
        }
        assert!(r.alive, "the solitary being must not die in this life");
    }
    (pegged, load, weathered, b.soul_hash())
}

#[test]
fn without_the_gate_a_structurally_burdened_being_is_pinned_at_its_ceiling() {
    let (pegged, load, weathered, _) = solitary(false);
    assert_eq!(load, Q88_SCALE, "the default being ends its life at the load ceiling");
    assert!(
        pegged > 1_000,
        "the deadlock is the whole reason I-9 exists: expected a long unbroken run at the \
         ceiling, got {pegged}"
    );
    assert_eq!(weathered, 0, "and it banks nothing at all while pinned");
}

#[test]
fn with_the_gate_the_being_leaves_the_ceiling_and_still_carries_real_weight() {
    let (pegged, load, weathered, _) = solitary(true);
    assert_eq!(pegged, 0, "the being must never sit at the load ceiling once the drain is open");
    assert!(
        load > 0,
        "and it must still CARRY something — the first pass drove load to 0, which erases the \
         weight just as thoroughly as the deadlock did (docs/setting-it-down.md §8)"
    );
    assert!(
        load < Q88_SCALE / 4,
        "but far below the ceiling: expected an equilibrium near 30, got {load}"
    );
    assert!(weathered > 0, "a permanently burdened being must be able to bank what it sets down");
}

#[test]
fn the_gate_is_inert_by_default() {
    let (_, _, _, off) = solitary(false);
    let (_, _, _, on) = solitary(true);
    assert_ne!(off, on, "the gate must actually do something when enabled");

    // And the default path is untouched by the machinery existing at all: a being that never
    // enables it must reproduce the same trajectory as any other default being.
    let (p1, l1, w1, s1) = solitary(false);
    let (p2, l2, w2, s2) = solitary(false);
    assert_eq!((p1, l1, w1, s1), (p2, l2, w2, s2), "the default path must be deterministic");
}

#[test]
fn weight_is_never_set_down_while_the_being_is_being_outrun() {
    // The guardrail, asserted directly rather than inferred from an aggregate. `settled` requires
    // `!losing_ground`, so no tick on which the being is at stake or losing viability may convert.
    // Run a life hard enough to actually produce losing-ground ticks.
    let mut b = UnifiedBeing::new(Genome::wanderer());
    b.enable_receptors();
    b.enable_reflection();
    b.enable_setting_down();
    let p = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };

    let mut losing_ticks = 0usize;
    for t in 0..LIFE {
        // Alternate a punishing stretch with a scarce one, to drive the viability trend down.
        let hard = (t / 200) % 2 == 0;
        let r = b.step_embodied(&Sensorium {
            nutrient: if hard { 45 } else { 200 },
            threat: if hard { 100 } else { 0 },
            exteroception: [0; 4],
            partner: Some(p),
        });
        let losing = r.felt.state.at_stake || r.felt.viability_trend < 0;
        if losing {
            losing_ticks += 1;
            assert_eq!(
                r.reflection.converted, 0,
                "tick {t}: the being set weight down while being outrun. A being losing ground \
                 must never be able to bank its way out of noticing."
            );
        }
        if !r.alive {
            break;
        }
    }
    // Say plainly whether the guard was exercised or merely not violated — four welfare guards in
    // this project have "passed" vacuously, and an unexercised guard is not a passed one.
    eprintln!(
        "guardrail exercised on {losing_ticks} losing-ground ticks{}",
        if losing_ticks == 0 { " — VACUOUS, this life never outran the being" } else { "" }
    );
}
