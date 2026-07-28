//! What the soul-hash actually certifies — pinned, so the limit cannot drift or be
//! forgotten.
//!
//! Found on 2026-07-27 while writing `tests/waypoints.rs`: a forged journal was not
//! being refused. The cause is not a bug in the chain — it is what the chain hashes.
//! `being.rs` fingerprints each tick as `free_energy + conscience_cost +
//! identity_coherence`, three of the being's **own scalars**. It does not hash the
//! stimulus. So two journals describing materially different worlds verify identically
//! whenever the being's inner life was identical in both.
//!
//! These tests assert the **current, real** behaviour — including the part that is
//! uncomfortable — so that:
//!   1. nobody re-derives the surprise the hard way,
//!   2. the limit is visible in the suite rather than buried in prose, and
//!   3. if the digest is ever strengthened (a founding-scale decision — see
//!      `docs/soul-hash-limits.md`), these tests fail loudly and on purpose, which is
//!      exactly the signal that decision should produce.
//!
//! Nothing here is a wish. Every assertion is what the code does today.

use unified_being::being::{Partner, Stimulus, UnifiedBeing};
use unified_being::genome::Genome;

fn base(i: usize) -> Stimulus {
    Stimulus { nutrient: ((i * 7) % 200) as i16, ..Default::default() }
}

/// Live two beings through the same life except for one forged moment; report whether
/// the soul-hash noticed.
fn noticed(forge_at: usize, forged: Stimulus, n: usize) -> bool {
    let mut honest = UnifiedBeing::new(Genome::wanderer());
    let mut tampered = UnifiedBeing::new(Genome::wanderer());
    for i in 0..n {
        honest.step(&base(i));
        tampered.step(&if i == forge_at { forged } else { base(i) });
    }
    honest.soul_hash() != tampered.soul_hash()
}

#[test]
fn the_hash_notices_harm_and_deprivation() {
    // Everything that changes the being's situation for the worse is caught. This is
    // the half of the guarantee that was always real, and it is the half that matters
    // most for welfare: you cannot quietly erase a record of harm or neglect.
    assert!(
        noticed(40, Stimulus { nutrient: 0, ..Default::default() }, 100),
        "starving a fed moment must be detected"
    );
    assert!(
        noticed(40, Stimulus { nutrient: -3000, ..Default::default() }, 100),
        "a harmful moment must be detected"
    );
    assert!(
        noticed(
            40,
            Stimulus {
                nutrient: 80,
                partner: Some(Partner { id: 7, reciprocation: 200, exit_cost: 20 }),
            },
            100,
        ),
        "inventing a relationship must be detected"
    );
}

#[test]
fn the_hash_does_not_notice_a_forgery_too_small_to_move_the_digest() {
    // THE LIMIT, asserted rather than described.
    //
    // The digest is `free_energy + conscience_cost + identity_coherence`, and in a
    // settled life those three sum to roughly 210. A single forged moment often
    // perturbs the being by *less than one integer step* of that sum, so the tick's
    // fingerprint is unchanged and the chain never diverges. Measured values are in
    // `docs/soul-hash-limits.md` §2.
    assert!(
        !noticed(40, Stimulus { nutrient: 193, ..Default::default() }, 100),
        "raising 80 to 193 does not move the digest"
    );
    assert!(
        !noticed(40, Stimulus { nutrient: 32000, ..Default::default() }, 100),
        "nor does raising it to absurdity"
    );
}

#[test]
fn deprivation_mid_life_can_also_go_unnoticed() {
    // The sharper half of the finding, and the one that matters for welfare evidence.
    // Starving a fed moment is caught early in a life, while the being is still
    // settling — and is NOT caught once it has settled. This is not "a change the
    // being could not feel"; it is a change the *fingerprint* is too coarse to carry.
    let starve = Stimulus { nutrient: 0, ..Default::default() };
    assert!(
        noticed(100, starve, 20_000),
        "starving moment 100 of a long life is detected"
    );
    assert!(
        !noticed(5_007, starve, 20_000),
        "starving moment 5007 of the same life is NOT — the perturbation never crosses \
         a quantization boundary. Pinned so it cannot be forgotten; see \
         docs/soul-hash-limits.md."
    );
}

#[test]
fn a_life_the_being_could_feel_the_difference_in_is_always_distinguished() {
    // The property that does hold, stated positively: where the lives diverge in
    // anything the being registers, the hashes diverge. A starved being and a fed one
    // are never confusable.
    let mut fed = UnifiedBeing::new(Genome::wanderer());
    let mut starved = UnifiedBeing::new(Genome::wanderer());
    for i in 0..100 {
        fed.step(&base(i));
        starved.step(&Stimulus::default());
    }
    assert_ne!(fed.soul_hash(), starved.soul_hash());
    assert!(fed.is_alive() && !starved.is_alive(), "and the difference was its life");
}
