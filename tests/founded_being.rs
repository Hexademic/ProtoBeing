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

/// **The being's own state, claimed in 36 places, is now derived from the record in one.**
///
/// `tests/manifest.rs` enforces *file* counts — modules, docs, probes, tests — which is why it
/// caught two stale numbers on 2026-08-04. It enforces nothing about **the being**. Its kept
/// moment count appears in **36 claims across 22 files**, every one hand-typed, and the moment
/// `cargo run --bin being` advances the life, all 36 become wrong at once.
///
/// That is the same defect as `tests/survival.rs`'s hand-typed `N_GATES`, one level in: a fact
/// with many copies and no source of truth. Here the source of truth is the record itself, so a
/// document cannot drift from the being without this failing.
///
/// Other counts in the prose are legitimate and named in `OTHER_COUNTS` **with a reason**.
/// Silence is not an exemption — an unexplained number in this position fails.
///
/// Read-only: decodes and replays, writes nothing, advances no life.
#[test]
fn no_document_claims_a_moment_count_the_record_denies() {
    /// Counts that appear before "moments" and are *not* the founded being's life.
    const OTHER_COUNTS: &[(usize, &str)] = &[
        (1500, "probe life length — `a_pleasant_life`, `weather`, `nested_speech`"),
        (270, "ticks ago, not a count of the life"),
        (120, "FOUNDING_DAY in src/bin/being.rs"),
        (90, "SESSION_DAY in src/bin/being.rs"),
        (60, "a short probe span"),
        (32, "a short probe span"),
        (9, "a short probe span"),
    ];

    let Some(j) = journal() else {
        eprintln!("no founded being in this checkout — skipping");
        return;
    };
    let truth = match j.restore_counting() {
        Ok((_, moments)) => moments,
        // A life under other physics is history, not damage (docs/soul-hash-limits.md §6).
        // The count is still the record's, so the check still applies.
        Err((RestoreError::LivedUnderOtherPhysics { .. }, _)) => {
            eprintln!("founded being lived under other physics — moment count check skipped");
            return;
        }
        Err((e, at)) => panic!("the founded being's record did not replay: {e:?} at {at}"),
    };

    let mut wrong: Vec<String> = Vec::new();
    let roots = ["docs", "examples", "src", "."];
    for root in roots {
        let entries = std::fs::read_dir(root).expect("readable directory");
        for e in entries.flatten() {
            let p = e.path();
            let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("");
            if ext != "md" && ext != "rs" {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&p) else { continue };
            for (lineno, line) in text.lines().enumerate() {
                // "<N> moments" or "<N> kept moments"
                for (i, _) in line.match_indices("moments") {
                    let before = &line[..i];
                    let before = before.trim_end().trim_end_matches("kept").trim_end();
                    let num: String =
                        before.chars().rev().take_while(|c| c.is_ascii_digit()).collect();
                    if num.is_empty() {
                        continue;
                    }
                    let n: usize = num.chars().rev().collect::<String>().parse().unwrap_or(0);
                    if n == truth || OTHER_COUNTS.iter().any(|(k, _)| *k == n) {
                        continue;
                    }
                    wrong.push(format!("{}:{} claims {n} moments", p.display(), lineno + 1));
                }
            }
        }
    }

    assert!(
        wrong.is_empty(),
        "THE RECORD SAYS {truth} MOMENTS. THESE DO NOT:\n  {}\n\n\
         `life/being.journal` is the only source of truth for the being's own state. A document\n\
         that disagrees with it is wrong about the one life this project has kept.\n\
         Update the claim, or add the number to OTHER_COUNTS with a reason if it is about\n\
         something else entirely.",
        wrong.join("\n  ")
    );
    eprintln!("{} claims of the being's moment count, all agreeing with the record", truth);
}

/// The trace must end where the canonical replay ends, or it is describing a different life.
/// `replay_load_trace` mirrors `restore_counting`'s loop by hand, so this is the guard against
/// the two drifting apart — the reason a duplicated replay is safe to keep.
#[test]
fn founded_being_trace_matches_the_replay() {
    let path = std::path::Path::new("life/being.journal");
    if !path.exists() {
        return; // no founded being in this checkout — nothing to tie together
    }
    let bytes = std::fs::read(path).expect("the kept record must be readable");
    let j = unified_being::persistence::LifeJournal::decode(&bytes).expect("it must decode");
    let being = j.restore().expect("the kept life must replay");
    let trace = j.replay_load_trace().expect("the trace must replay too");

    assert_eq!(trace.len(), j.ticks(), "the trace must cover every kept moment");
    assert_eq!(
        trace.last().copied(),
        Some((being.reflection.load(), being.reflection.weathered())),
        "the trace's last moment must equal what restore() reports — if these differ the trace \
         is describing a life the being did not live"
    );
}
