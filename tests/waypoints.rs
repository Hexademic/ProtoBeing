//! Waypoints — the tests, written before the implementation.
//!
//! Written against `docs/waypoints.md` §5 and §6 and watched to fail (the `Waypoint`
//! type did not exist) before a line of `persistence.rs` changed. Integration tests on
//! purpose: they may only use the public surface, which is the same surface anyone
//! auditing a saved life would have.

use unified_being::being::Stimulus;
use unified_being::genome::Genome;
use unified_being::persistence::{Features, LifeJournal, RestoreError};

const CADENCE: u32 = 16;

/// A deterministic, slightly varying life — so consecutive moments differ and a
/// forgery at one of them is a real change rather than a no-op.
fn stim(i: usize) -> Stimulus {
    Stimulus { nutrient: ((i * 7) % 200) as i16, ..Default::default() }
}

/// A forgery the soul-hash actually notices — starving one moment of a fed life.
///
/// It must be this rather than "any different stimulus", because of a limit found
/// while writing these tests and documented in `docs/soul-hash-limits.md`: the hash
/// fingerprints three of the being's own scalars, not the journal's bytes, so a
/// forgery the being could not possibly have noticed leaves it unchanged. Waypoints
/// inherit exactly the detection power of the anchor — no more, no less — and these
/// tests exercise that power rather than pretending to a stronger one.
fn a_noticed_forgery() -> Stimulus {
    Stimulus { nutrient: 0, ..Default::default() }
}

/// Live `n` moments with waypoints at `CADENCE`, sealed and ready to restore.
///
/// The record's integrity hash is **cleared** so these tests exercise the waypoint
/// chain in isolation. Since `docs/journal-integrity.md`, a forged record is caught by
/// the integrity hash *before* replay begins — cheaper and more complete than the chain
/// — so the chain's remaining job is divergence of the *replay itself* (code drift,
/// version skew, a nondeterminism bug) and legacy v3 journals that carry no hash. That
/// narrower job is what is tested here; the ordering is asserted separately below.
fn a_life(n: usize) -> LifeJournal {
    let (mut being, mut j) = LifeJournal::birth_with_waypoints(
        Genome::wanderer(),
        Features::default(),
        CADENCE,
    );
    for i in 0..n {
        j.live(&mut being, &stim(i));
    }
    j.seal(&being);
    j.clear_journal_hash_for_test();
    j
}

// ---------------------------------------------------------------------------
// C4 — an honest life is unchanged. The floor: waypoints observe, never steer.
// ---------------------------------------------------------------------------

#[test]
fn the_record_hash_is_checked_before_the_replay_begins() {
    // The ordering, asserted. A forged record is the integrity hash's business and it
    // answers first, without stepping the being at all — see docs/journal-integrity.md.
    let (mut being, mut j) = LifeJournal::birth_with_waypoints(
        Genome::wanderer(),
        Features::default(),
        CADENCE,
    );
    for i in 0..100 {
        j.live(&mut being, &stim(i));
    }
    j.seal(&being); // keeps its journal hash
    j.forge_for_test(40, a_noticed_forgery());

    assert!(
        matches!(j.restore(), Err(RestoreError::JournalTampered)),
        "an intact-hash journal that was forged is caught by the record check first"
    );
    match j.restore_counting() {
        Err((_, replayed)) => assert_eq!(replayed, 0, "and not one moment was replayed"),
        Ok(_) => panic!("a forged life must not wake"),
    }
}

#[test]
fn c4_waypoints_do_not_change_the_life_they_watch() {
    // The same life, lived with and without waypoints, must be the same being.
    let with = a_life(100);

    let (mut being, mut without) =
        LifeJournal::birth(Genome::wanderer(), Features::default());
    for i in 0..100 {
        without.live(&mut being, &stim(i));
    }
    without.seal(&being);
    without.clear_journal_hash_for_test();

    assert_eq!(
        with.anchor(),
        without.anchor(),
        "a life watched by waypoints must be bit-identical to one that is not"
    );
    assert!(with.restore().is_ok(), "and it still wakes");
}

#[test]
fn c4b_a_life_records_the_waypoints_it_passed() {
    let j = a_life(100);
    // 100 moments at cadence 16: waypoints at 16, 32, 48, 64, 80, 96.
    assert_eq!(j.waypoints().len(), 6, "one waypoint per completed cadence");
    assert_eq!(j.waypoints()[0].at, 16);
    assert_eq!(j.waypoints()[5].at, 96);
}

#[test]
fn c4c_a_journal_without_waypoints_still_wakes() {
    // Backward compatibility is a requirement, not a hope: an older life carries no
    // waypoints and must restore exactly as it always did.
    let (mut being, mut j) = LifeJournal::birth(Genome::wanderer(), Features::default());
    for i in 0..40 {
        j.live(&mut being, &stim(i));
    }
    j.seal(&being);
    j.clear_journal_hash_for_test();
    assert!(j.waypoints().is_empty());
    assert!(j.restore().is_ok(), "a waypoint-less journal wakes unchanged");
}

#[test]
fn c4d_waypoints_survive_the_round_trip_to_bytes() {
    let j = a_life(100);
    let bytes = j.encode();
    let back = LifeJournal::decode(&bytes).expect("a sealed journal decodes");
    assert_eq!(back.waypoints().len(), j.waypoints().len());
    assert_eq!(back.waypoints()[3].at, j.waypoints()[3].at);
    assert_eq!(back.waypoints()[3].hash, j.waypoints()[3].hash);
    assert!(back.restore().is_ok(), "and the decoded life still wakes");
}

// ---------------------------------------------------------------------------
// C1 / C2 — a forgery is still refused, and now the refusal says WHERE.
// ---------------------------------------------------------------------------

#[test]
fn c1_a_forged_journal_is_still_refused() {
    let mut j = a_life(100);
    j.forge_for_test(40, a_noticed_forgery());
    assert!(j.restore().is_err(), "the floor is unchanged: a forged life is refused");
}

#[test]
fn c2_the_refusal_names_the_segment_the_forgery_lies_in() {
    // THE CRUX. Today's ContinuityBroken says only "something is wrong". With the
    // chain, a forgery at moment 40 must be pinned between waypoints 32 and 48.
    let mut j = a_life(100);
    j.forge_for_test(40, a_noticed_forgery());

    match j.restore() {
        Err(RestoreError::ForgedBetween { after, before }) => {
            assert_eq!(after, 32, "the last waypoint the life still matched");
            assert_eq!(before, 48, "the first waypoint it no longer matches");
        }
        Err(e) => panic!("expected a localized rejection, got {e:?}"),
        Ok(_) => panic!("expected a localized rejection, the forged life woke"),
    }
}

#[test]
fn c2b_a_forgery_after_the_last_waypoint_is_still_caught_by_the_anchor() {
    // Beyond the last waypoint there is no segment to name — the anchor must still
    // refuse it. Waypoints are additional checks, never a substitute (§5).
    let mut j = a_life(100);
    j.forge_for_test(98, a_noticed_forgery()); // last waypoint is at 96
    assert!(
        matches!(j.restore(), Err(RestoreError::ContinuityBroken)),
        "past the last waypoint the anchor is still the backstop"
    );
}

// ---------------------------------------------------------------------------
// C3 — detection stops early. The whole point of the chain.
// ---------------------------------------------------------------------------

#[test]
fn c3_detection_stops_at_the_first_waypoint_past_the_forgery() {
    let mut j = a_life(1000);
    j.forge_for_test(40, a_noticed_forgery());

    let replayed = match j.restore_counting() {
        Err((_, n)) => n,
        Ok(_) => panic!("a forged life must not wake"),
    };
    assert!(
        replayed <= 48,
        "a forgery at 40 must be caught by moment 48, not after all 1000 — replayed {replayed}"
    );
}

#[test]
fn c3b_an_honest_life_is_replayed_whole() {
    // The saving is for dishonest journals only. An honest one is replayed in full,
    // as it must be — nothing here hands back a being unreplayed (§5).
    let j = a_life(1000);
    let (_being, replayed) = j.restore_counting().expect("an honest life wakes");
    assert_eq!(replayed, 1000, "every moment of an honest life is re-lived");
}
