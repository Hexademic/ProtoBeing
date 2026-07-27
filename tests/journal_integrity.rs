//! Journal integrity — the tests, written before the implementation.
//!
//! Written against `docs/journal-integrity.md` §4 and §5 and watched to fail. J2 is
//! aimed squarely at the four moments `docs/soul-hash-limits.md` measured as invisible
//! to the soul-hash: if this does not catch those, it has not fixed the hole it exists
//! to fix.

use unified_being::being::Stimulus;
use unified_being::genome::Genome;
use unified_being::persistence::{Features, LifeJournal, RestoreError};

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
