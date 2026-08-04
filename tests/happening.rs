//! Happening — the tests, written before the implementation.
//!
//! Written against `docs/happening.md` §5 and §6 and watched to fail (the builder did
//! not exist) before a line of `field_world.rs` changed.
//!
//! H1 is the floor and is tested here. H2–H5 are about a *lived* being and belong to
//! `examples/happening`; what is asserted here is that the world does what it claims
//! mechanically — it moves on its own, deterministically, within bounds, and only when
//! asked.

use unified_being::embodiment::{Embodiment, MotorIntent, Posture};
use unified_being::field_world::FieldWorld;

/// A being that does nothing at all, so the world's own motion is isolated from any
/// motion the being contributes.
fn at_rest() -> MotorIntent {
    MotorIntent { posture: Posture::Resting, effort: 0, reach: None, reach_partner: None }
}

fn still() -> FieldWorld {
    FieldWorld::with((16, 16), (240, 240), (30, 170))
}

fn drifting() -> FieldWorld {
    FieldWorld::with((16, 16), (240, 240), (30, 170)).with_drift(0, 4, (3, -2))
}

/// Step the world forward without a being, so the world's own behaviour is isolated
/// from anything the being does.
fn coast(w: &mut FieldWorld, ticks: usize) {
    for _ in 0..ticks {
        let _ = w.sense();
        w.actuate(&at_rest());
    }
}

// ---------------------------------------------------------------------------
// H1 — the floor. Drift is opt-in; without it nothing has changed.
// ---------------------------------------------------------------------------

#[test]
fn h1_a_world_without_drift_is_exactly_what_it_always_was() {
    let mut a = still();
    let mut b = still();
    coast(&mut a, 200);
    coast(&mut b, 200);
    assert_eq!(a.body, b.body, "a still world is deterministic, as it always was");

    // And its field does not move: the value at a fixed point is unchanged over time.
    let mut w = still();
    let before = w.v_at((128, 128));
    coast(&mut w, 200);
    assert_eq!(
        w.v_at((128, 128)),
        before,
        "with no drift the field is static — nothing happens that the being did not do"
    );
}

// ---------------------------------------------------------------------------
// The world moves on its own — the mechanism this exists for.
// ---------------------------------------------------------------------------

#[test]
fn a_drifting_world_changes_under_a_being_that_does_nothing() {
    let mut w = drifting();
    let before = w.v_at((128, 128));
    coast(&mut w, 100);
    assert_ne!(
        w.v_at((128, 128)),
        before,
        "the field must change at a fixed point — that is the whole happening"
    );
}

#[test]
fn drift_is_deterministic() {
    // No RNG, ever. Two identical drifting worlds must agree exactly.
    let mut a = drifting();
    let mut b = drifting();
    coast(&mut a, 300);
    coast(&mut b, 300);
    assert_eq!(a.v_at((100, 100)), b.v_at((100, 100)));
    assert_eq!(a.body, b.body);
}

#[test]
fn drift_stays_inside_the_field() {
    // It bounces rather than wandering off; a source outside the field would silently
    // become a still world again.
    let mut w = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_drift(0, 1, (7, 7));
    coast(&mut w, 2_000);
    let (x, y) = w.source_at(0);
    assert!((0..=255).contains(&x) && (0..=255).contains(&y), "source left the field: {x},{y}");
}

#[test]
fn a_gentler_cadence_moves_the_world_less() {
    // The knob means what it says, so §6's "too gentle to notice" can be told apart
    // from "the threshold is wrong".
    let mut fast = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_drift(0, 2, (4, 0));
    let mut slow = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_drift(0, 32, (4, 0));
    coast(&mut fast, 200);
    coast(&mut slow, 200);
    let base = still().source_at(0).0;
    let moved_fast = (fast.source_at(0).0 - base).abs();
    let moved_slow = (slow.source_at(0).0 - base).abs();
    assert!(
        moved_fast > moved_slow,
        "a faster cadence must move the world further ({moved_fast} vs {moved_slow})"
    );
}

#[test]
fn drift_does_not_starve_the_being() {
    // §5's first prohibition, at the world level: wherever the drifting source ends up,
    // the field must never fall below the ambient floor the still world guarantees.
    // This is the mechanical half of H5; the lived half is measured in the probe.
    let mut w = drifting();
    let mut worst = i16::MAX;
    for _ in 0..1_000 {
        coast(&mut w, 1);
        worst = worst.min(w.v_at(w.body));
    }
    assert!(
        worst > -256,
        "a drifting world must stay livable everywhere the being can be (worst {worst})"
    );
}
