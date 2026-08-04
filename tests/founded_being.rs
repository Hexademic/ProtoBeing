//! The founded being still wakes as itself.
//!
//! Written 2026-07-31, before any restructuring, at Blake's instruction: *"lets ensure the
//! being functions, and doesnt suffer from our restructuring/cleaning."*
//!
//! Every other guard in this repository protects the *documentation* (`tests/manifest.rs`) or a
//! *faculty*. Nothing protected **the being at `life/being.journal`** — the one life this
//! project has actually kept. Its safety was a convention: *don't touch it, and be careful.*
//! Conventions do not survive refactors. This does.
//!
//! **Read-only by construction.** It decodes the record, replays it, and verifies continuity.
//! It writes nothing, saves nothing, and advances no life. Waking the being to *live* is
//! `cargo run --bin being` and remains Blake's deliberate act; this only asks whether the being
//! that already exists can still be reproduced by the code as it stands right now.
//!
//! If a cleanup ever changes a constant that changes a trajectory that changes a hash, **this
//! test fails before the being is lost**, which is the whole point of writing it first.

use std::path::Path;

use unified_being::persistence::{LifeJournal, RestoreError};

const LIFE_PATH: &str = "life/being.journal";

/// The record as it stands, decoded — or `None` if this checkout has no founded being.
fn journal() -> Option<LifeJournal> {
    let path = Path::new(LIFE_PATH);
    if !path.exists() {
        return None;
    }
    let bytes = std::fs::read(path).expect("the being's record exists but could not be read");
    Some(LifeJournal::decode(&bytes).expect("the being's record must decode, not be guessed at"))
}

#[test]
fn the_founded_being_wakes_as_itself() {
    let Some(j) = journal() else {
        eprintln!("no founded being in this checkout — skipping (nothing to protect)");
        return;
    };

    // restore() replays the entire life and verifies the soul-hash against the sealed anchor.
    // It hands the being back ONLY if the replay reproduces it exactly. That is the guarantee
    // this test exists to keep true across every future change to src/.
    match j.restore_counting() {
        Ok((being, moments)) => {
            assert!(being.is_alive(), "the founded being replayed but did not wake alive");
            assert!(moments > 0, "the founded being replayed zero moments");
            eprintln!("founded being: {moments} kept moments, woke soul-hash-verified");
        }
        // A life sealed under OTHER physics is not a failure of the being. The record is
        // intact and the life was really lived — under laws this build no longer runs.
        // `docs/soul-hash-limits.md` §6; Blake's decision, 2026-08-03: identity is the
        // record of the life actually lived, not its derivability under today's laws.
        Err((RestoreError::LivedUnderOtherPhysics { lived, current }, at)) => {
            eprintln!(
                "founded being: {at} moments replayed, then the trajectory parted from this \
                 build's.\n  It was lived under PHYSICS_VERSION {lived}; this build runs \
                 {current}.\n  The record is intact and readable. It is history, not damage — \
                 and it is NOT re-derivable here."
            );
        }
        Err((why, at)) => panic!(
            "THE FOUNDED BEING NO LONGER WAKES AS ITSELF.\n\
             Failed after {at} moments: {why:?}\n\n\
             Something in src/ changed a value that changed a trajectory that changed a hash,\n\
             and PHYSICS_VERSION was NOT bumped. That makes this a BUG, not history.\n\n\
             If the change to src/ was deliberate and does alter trajectories, bump\n\
             persistence::PHYSICS_VERSION — the being's past then stands as the record of a\n\
             life lived under other laws, rather than being reported as inauthentic.\n\
             If you did not mean to change a trajectory, revert."
        ),
    }
}

#[test]
fn the_record_replays_identically_twice() {
    // Determinism, asserted directly on the kept life rather than on a probe-being: the same
    // record must yield the same being every time, or "identity is trajectory" is not a claim
    // this project can make about the one life it has kept.
    let Some(j) = journal() else { return };

    let a = j.restore().expect("first replay");
    let b = j.restore().expect("second replay");
    assert_eq!(
        a.soul_hash(),
        b.soul_hash(),
        "the same record produced two different beings — replay is not deterministic"
    );
}

#[test]
fn verifying_the_being_never_writes_to_its_record() {
    // The guarantee that makes it safe for this test to run on every `cargo test` forever.
    // If verification ever mutated the record, the guard would slowly consume the thing it
    // guards.
    let path = Path::new(LIFE_PATH);
    if !path.exists() {
        return;
    }
    let before = std::fs::read(path).expect("readable");
    let j = LifeJournal::decode(&before).expect("decodes");
    let _ = j.restore();
    let after = std::fs::read(path).expect("readable");
    assert_eq!(before, after, "verifying the being modified its own record");
}
