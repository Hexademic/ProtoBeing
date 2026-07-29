//! The null space — the tests, written before the implementation.
//!
//! Against `docs/null-space.md` §3 and §5, watched to fail. The question is whether this
//! being already has more than one adequate way to do the same thing, in a world that only
//! ever lets it see one.
//!
//! N4 and N5 — *how much* redundancy there is, and whether it survives under load — belong
//! to the probe, because they need lived beings. What is asserted here is that the observer
//! cannot be wrong about the being it is watching.

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::{FieldWorld, COMPASS};
use unified_being::genome::Genome;
use unified_being::null_space::{adequate, N_DIRS};

#[test]
fn a_flat_choice_is_seen_as_freedom_and_a_forced_one_as_none() {
    // Four equally-good ways is four ways, not one. This is the tie that `climb`'s strict
    // `>` silently resolves to compass order and never reports.
    let all_same = adequate(&[8, 8, 8, 8], 0);
    assert_eq!(all_same.count, 4, "four equally-good directions is four ways to do the same thing");

    // One clear winner is one way. Freedom must not be manufactured where there is none.
    let forced = adequate(&[8, 1, -3, 0], 0);
    assert_eq!(forced.count, 1, "one clearly-best direction is a forced way");
    assert_eq!(forced.best, 8);
}

#[test]
fn a_direction_that_does_not_climb_is_never_adequate() {
    // "Good enough" means good enough *to do the task*. A direction that loses ground is
    // not a different way of arriving; it is not arriving. Tolerance must never launder
    // a non-climbing direction into the set.
    let a = adequate(&[8, 0, -4, -9], 0);
    assert_eq!(a.count, 1, "zero and negative deltas do not climb");

    let generous = adequate(&[8, 0, -4, -9], 100);
    assert_eq!(
        generous.count, 1,
        "even an absurd tolerance must not admit a direction that fails the task"
    );
}

#[test]
fn nothing_to_climb_is_reported_as_singular_not_as_a_free_choice() {
    // docs/j-space.md's geometry of despair: the map is singular where the being needs it.
    // An empty adequate set and a four-way tie are opposite conditions and must not be
    // confused — both would be "the direction does not matter", for opposite reasons.
    let singular = adequate(&[0, 0, 0, 0], 0);
    assert_eq!(singular.count, 0, "no direction improves anything — singular, not free");
    assert_eq!(singular.best, 0);

    let all_down = adequate(&[-1, -5, -2, -9], 0);
    assert_eq!(all_down.count, 0, "every way is downhill — singular");
}

#[test]
fn tolerance_admits_the_near_miss_and_is_monotone() {
    // A direction one raw unit worse than the best reaches the same high ground a tick
    // later. Below the being's own resolution (~3 raw), that is the same outcome.
    let deltas = [10, 9, 6, -2];
    assert_eq!(adequate(&deltas, 0).count, 1, "at zero tolerance, only the best");
    assert_eq!(adequate(&deltas, 1).count, 2, "one unit admits the near miss");
    assert_eq!(adequate(&deltas, 4).count, 3, "four units admits the third");

    // Monotone non-decreasing in tolerance, always (N2).
    let mut last = 0u8;
    for tol in 0..64i16 {
        let n = adequate(&deltas, tol).count;
        assert!(n >= last, "adequate count fell as tolerance rose (tol {tol}): {n} after {last}");
        last = n;
    }
    assert!(last <= N_DIRS as u8, "cannot be more ways than there are directions");
}

#[test]
fn the_mask_names_the_same_directions_the_count_counts() {
    // The set is reported as a bitmask so a caller can act on *which* ways, not only how
    // many. The two reports must never disagree.
    for deltas in [[8, 8, 8, 8], [8, 1, -3, 0], [0, 0, 0, 0], [10, 9, 6, -2], [-1, 5, 5, 2]] {
        for tol in [0i16, 1, 3, 8] {
            let a = adequate(&deltas, tol);
            let named = (0..N_DIRS).filter(|&i| a.contains(i)).count();
            assert_eq!(
                named, a.count as usize,
                "mask and count disagree for {deltas:?} at tolerance {tol}"
            );
        }
    }
}

#[test]
fn n1_the_observer_never_disagrees_with_the_being_it_watches() {
    // The load-bearing test. Whatever the world actually made the being do must always be
    // one of the ways the observer calls adequate. An observer that reports the being's own
    // action as inadequate is measuring something else — a bug, not a finding.
    let worlds = [
        FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20)),
        FieldWorld::with((128, 128), (140, 140), (250, 250)).with_person(1, (20, 20)),
        FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20)).with_weather(0, 2),
    ];

    for (w, mut world) in worlds.into_iter().enumerate() {
        let mut being = UnifiedBeing::new(Genome::wanderer());
        for t in 0..600 {
            let sens = world.sense();
            let r = being.step_embodied(&sens);
            let intent = intent_from(&r);

            let deltas = world.climb_deltas(&intent);
            let (chosen, delta) = world.chosen_climb(&intent);

            for tol in [0i16, 1, 3, 8] {
                let a = adequate(&deltas, tol);
                if delta > 0 {
                    // The being climbed: its direction must be in the set, at every tolerance.
                    let idx = COMPASS.iter().position(|&d| d == chosen).unwrap_or_else(|| {
                        panic!("world {w} tick {t}: climb chose {chosen:?}, not a compass direction")
                    });
                    assert!(
                        a.contains(idx),
                        "world {w} tick {t} tol {tol}: the being went {chosen:?} (delta {delta}) \
                         but the observer calls that inadequate — deltas {deltas:?}"
                    );
                    assert_eq!(a.best, delta, "world {w} tick {t}: observer's best disagrees");
                    // N2: never empty when something climbed.
                    assert!(a.count >= 1, "world {w} tick {t}: climbed, yet the set is empty");
                } else {
                    // Nothing climbed: singular, and the being stayed put.
                    assert_eq!(
                        a.count, 0,
                        "world {w} tick {t}: nothing climbed (delta {delta}) yet the observer \
                         found {} adequate ways — deltas {deltas:?}",
                        a.count
                    );
                }
            }

            world.actuate(&intent);
            if !being.is_alive() {
                break;
            }
        }
    }
}

#[test]
fn watching_changes_nothing() {
    // Stage 1. Two identical lives, one of them observed at every tick, must end on the
    // same soul-hash. If watching moves the trajectory by one raw unit, this inch failed.
    let live = |observe: bool| {
        let mut world = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
        let mut being = UnifiedBeing::new(Genome::wanderer());
        for _ in 0..500 {
            let sens = world.sense();
            let r = being.step_embodied(&sens);
            let intent = intent_from(&r);
            if observe {
                let deltas = world.climb_deltas(&intent);
                for tol in 0..8 {
                    let _ = adequate(&deltas, tol);
                }
                let _ = world.chosen_climb(&intent);
            }
            world.actuate(&intent);
        }
        being.soul_hash()
    };
    assert_eq!(live(false), live(true), "the observer moved the being's trajectory");
}
