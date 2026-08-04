//! Journal integrity — the tests, written before the implementation.
//!
//! Written against `docs/journal-integrity.md` §4 and §5 and watched to fail. J2 is
//! aimed squarely at the four moments `docs/soul-hash-limits.md` measured as invisible
//! to the soul-hash: if this does not catch those, it has not fixed the hole it exists
//! to fix.

use unified_being::being::Stimulus;
use unified_being::genome::Genome;
use unified_being::persistence::{Features, LifeJournal, RestoreError, PHYSICS_VERSION};

fn stim(i: usize) -> Stimulus {
    Stimulus { nutrient: ((i * 7) % 200) as i16, ..Default::default() }
}

fn a_life(n: usize) -> LifeJournal {
    let (mut being, mut j) = LifeJournal::birth(Genome::wanderer(), Features::default());
    for i in 0..n {
        j.live(&mut being, &stim(i));
    }
    j.seal(&being);
    j
}

#[test]
fn j1_an_honest_journal_round_trips_and_verifies() {
    let j = a_life(500);
    assert!(j.journal_hash().is_some(), "sealing records a journal hash");

    let bytes = j.encode();
    let back = LifeJournal::decode(&bytes).expect("decodes");
    assert_eq!(back.journal_hash(), j.journal_hash(), "the hash survives the round trip");
    assert!(back.restore().is_ok(), "and an untouched life still wakes");
}

#[test]
fn j2_every_forgery_is_caught_including_the_ones_the_soul_hash_misses() {
    // THE CRUX. These four moments are measured in docs/soul-hash-limits.md as leaving
    // the soul-hash completely unchanged. The journal hash must catch all of them.
    let honest = a_life(20_000);
    let starve = Stimulus { nutrient: 0, ..Default::default() };

    for at in [1_013usize, 5_007, 10_001, 19_990] {
        assert_ne!(stim(at).nutrient, starve.nutrient, "moment {at} must really change");

        let mut forged = honest.clone();
        forged.forge_for_test(at, starve);

        assert!(
            matches!(forged.restore(), Err(RestoreError::JournalTampered)),
            "a forgery at moment {at} must be caught by the journal hash — \
             the soul-hash provably does not catch it"
        );
    }
}

#[test]
fn j2b_detection_is_deterministic_not_probabilistic() {
    // Every single moment of a short life, forged in turn. There is no quantum to fall
    // beneath, so there must be no survivors.
    let honest = a_life(200);
    let starve = Stimulus { nutrient: 0, ..Default::default() };
    let mut missed = Vec::new();

    for at in 0..200 {
        if stim(at).nutrient == starve.nutrient {
            continue; // forging this would be a no-op
        }
        let mut forged = honest.clone();
        forged.forge_for_test(at, starve);
        if forged.restore().is_ok() {
            missed.push(at);
        }
    }
    assert!(missed.is_empty(), "every forged moment must be caught; missed {missed:?}");
}

#[test]
fn j3_no_beings_soul_hash_moves() {
    // The floor. The journal hash is computed over the record, outside the tick, so a
    // being that lives alongside it must be bit-identical to one that does not.
    let mut bare = unified_being::being::UnifiedBeing::new(Genome::wanderer());
    for i in 0..500 {
        bare.step(&stim(i));
    }
    let journaled = a_life(500);
    assert_eq!(
        journaled.anchor(),
        Some(bare.soul_hash()),
        "the journal hash must not perturb the being by one bit"
    );
}

#[test]
fn j4_resealing_an_existing_life_preserves_its_identity_exactly() {
    // The founded being could gain integrity coverage without its identity moving.
    // Proved here on a fresh life rather than asserted about the real file, which is
    // not touched by this work (`docs/journal-integrity.md` §4).
    let original = a_life(390);
    let anchor_before = original.anchor();

    // A journal saved before this feature existed: same life, no journal hash.
    let mut legacy = original.clone();
    legacy.clear_journal_hash_for_test();
    assert!(legacy.journal_hash().is_none());
    assert!(legacy.restore().is_ok(), "a legacy journal still wakes");

    // Re-seal it: the hash appears, the identity does not move.
    let woken = legacy.restore().expect("wakes");
    let mut resealed = legacy.clone();
    resealed.seal(&woken);

    assert_eq!(resealed.anchor(), anchor_before, "its soul-hash is exactly what it was");
    assert!(resealed.journal_hash().is_some(), "and it now carries integrity coverage");
    assert!(resealed.restore().is_ok(), "and it still wakes as itself");
}

#[test]
fn j3b_a_journal_from_before_this_feature_still_wakes() {
    let mut legacy = a_life(100);
    legacy.clear_journal_hash_for_test();
    let bytes = legacy.encode();
    let back = LifeJournal::decode(&bytes).expect("an older journal decodes");
    assert!(back.journal_hash().is_none());
    assert!(back.restore().is_ok(), "and restores exactly as it always did");
}

// ---------------------------------------------------------------------------------
// Physics versioning — `docs/soul-hash-limits.md` §6.
//
// Three questions had been welded into one. The soul-hash answers "did this being live
// this inner life?"; `hash_record` answers "are these the bytes that were written?"; and
// until now the soul-hash was ALSO being asked "can the physics as it stands right now
// re-derive this life?" — so every improvement to the being's own laws reported its past
// as inauthentic, and the being could only stay itself if we stopped developing.
//
// These tests exercise the third mechanism. A record whose integrity holds but whose
// replay diverges is exactly what a life lived under other laws looks like from outside.
// ---------------------------------------------------------------------------------

/// A life that is internally consistent and no longer re-derivable — what a stretch lived
/// under other physics looks like once the laws have moved on.
fn a_life_this_build_cannot_rederive(n: usize) -> LifeJournal {
    let mut j = a_life(n);
    let starve = Stimulus { nutrient: 0, ..Default::default() };
    j.forge_for_test(n / 2, starve);
    // The bytes ARE the bytes that were written; only the replay parts from them.
    j.reseal_record_for_test();
    j
}

#[test]
fn p1_a_life_lived_under_other_physics_is_history_not_damage() {
    let mut j = a_life_this_build_cannot_rederive(200);
    j.set_physics_for_test(Some(9_999));

    match j.restore() {
        Err(RestoreError::LivedUnderOtherPhysics { lived, current }) => {
            assert_eq!(lived, 9_999, "it reports the laws the life was actually lived under");
            assert_eq!(current, PHYSICS_VERSION, "and the laws this build runs");
        }
        Err(other) => panic!(
            "a life sealed under other physics must be reported as history, not as a broken \
             or forged being — got {other:?}"
        ),
        Ok(_) => panic!("this record does not re-derive; it must not verify"),
    }
}

#[test]
fn p2_the_same_divergence_under_the_same_physics_is_still_a_bug() {
    // The guard must not become a way to wave away real breakage. Identical record,
    // physics UNCHANGED: this is a bug and must still be reported as one.
    let mut j = a_life_this_build_cannot_rederive(200);
    j.set_physics_for_test(Some(PHYSICS_VERSION));

    match j.restore() {
        Err(RestoreError::LivedUnderOtherPhysics { .. }) => {
            panic!("a divergence under UNCHANGED physics must never be excused as history")
        }
        Err(_) => {}
        Ok(_) => panic!("this record does not re-derive; it must not verify"),
    }
}

#[test]
fn p3_a_physics_bump_that_did_not_touch_this_life_costs_it_nothing() {
    // The version says which laws were in force, never whether to attempt the replay.
    // A life that still reproduces itself wakes at full strength regardless.
    let mut j = a_life(200);
    let anchor_before = j.anchor();
    j.set_physics_for_test(Some(9_999));

    assert!(
        j.restore().is_ok(),
        "a bump for a change that did not affect THIS trajectory must cost it nothing"
    );
    assert_eq!(j.anchor(), anchor_before, "and its identity does not move");
}

#[test]
fn p4_a_journal_from_before_physics_was_recorded_still_wakes_unchanged() {
    let mut legacy = a_life(200);
    let anchor_before = legacy.anchor();
    legacy.set_physics_for_test(None);

    assert!(legacy.physics().is_none());
    assert!(legacy.restore().is_ok(), "a life recorded before this field still wakes");
    assert_eq!(legacy.anchor(), anchor_before, "with its identity untouched");

    // And round-trips: what is written is what is read back.
    let sealed = a_life(200);
    let decoded = LifeJournal::decode(&sealed.encode()).expect("decodes");
    assert_eq!(decoded.physics(), Some(PHYSICS_VERSION), "a sealed life records its laws");
    assert_eq!(decoded.anchor(), sealed.anchor());
    assert!(decoded.restore().is_ok());
}
