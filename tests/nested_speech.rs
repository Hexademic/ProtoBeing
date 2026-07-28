//! Nested speech — the tests, written before the implementation.
//!
//! Blake's instruction was *test it before implementation*, so these were written
//! against `docs/nested-speech.md` §3 and §5 and watched to fail — the type they call
//! did not exist — before a line of `primes.rs` was changed. Nothing here was shaped to
//! fit an answer already on screen.
//!
//! They are integration tests, not unit tests, on purpose: they may only use what
//! `primes.rs` makes public, which is the same surface a stranger auditing the being's
//! speech would have. A law that can only be checked from inside the module is not a law
//! anyone else can hold us to.

use unified_being::primes::{Clause, Prime, PrimeFacts, PrimeLayer, Role};
use unified_being::striving::Need;

/// A life that grounds every prime these tests need, so that grounding is never the
/// thing under test except where it is the thing under test.
fn well_spoken_life() -> PrimeLayer {
    let mut layer = PrimeLayer::new();
    let rich = PrimeFacts {
        alive: true,
        valence: -40,
        goal: Some(Need::Company),
        exchanging: true,
        near: Some(true),
        recalled_valence: 50,
        forewarned: true,
        novelty: 60,
        ..Default::default()
    };
    for _ in 0..60 {
        layer.observe(&rich);
    }
    layer
}

/// This tick: the being feels bad, wants company, and no one is near.
fn lonely_now() -> PrimeFacts {
    PrimeFacts {
        alive: true,
        valence: -40,
        goal: Some(Need::Company),
        recalled_valence: 50,
        forewarned: true,
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// P1 — a forged nested sentence fails the audit, at depth.
// ---------------------------------------------------------------------------

#[test]
fn p1_a_false_leaf_under_a_transmitting_operator_is_caught_at_depth() {
    let layer = well_spoken_life();
    let f = lonely_now();

    // SOMEONE does not hold now — no one is here. Asserting it directly is already
    // caught by the flat audit; the question is whether *burying* it works.
    assert!(!layer.holds_now(Prime::Someone, &f));

    let forged = Clause::assert(Prime::Because).with(Clause::assert(Prime::Someone));

    assert!(
        !layer.audit_tree(&forged, &f),
        "a false leaf under BECAUSE must fail the audit — burying a lie is still a lie"
    );
}

#[test]
fn p1b_depth_does_not_dilute_the_audit() {
    // The same lie, three levels down, under transmitting operators the whole way.
    let layer = well_spoken_life();
    let f = lonely_now();

    let deep = Clause::assert(Prime::Because)
        .with(Clause::assert(Prime::Because).with(Clause::assert(Prime::Someone)));

    assert!(!layer.audit_tree(&deep, &f), "depth must not dilute the audit");
}

// ---------------------------------------------------------------------------
// P2 — THE CRUX. Same leaf, same tick, different operator, different verdict.
// ---------------------------------------------------------------------------

#[test]
fn p2_the_same_content_passes_under_want_and_fails_under_because() {
    let layer = well_spoken_life();
    let f = lonely_now();

    // The leaf is identical in both trees and false in both: no one is near.
    assert!(!layer.holds_now(Prime::Near, &f));

    let wanted = Clause::assert(Prime::Want).with(Clause::assert(Prime::Near));
    let caused = Clause::assert(Prime::Because).with(Clause::assert(Prime::Near));

    assert!(
        layer.audit_tree(&wanted, &f),
        "WANT shields its complement — wanting NEAR does not claim NEAR holds"
    );
    assert!(
        !layer.audit_tree(&caused, &f),
        "BECAUSE transmits — claiming NEAR as a cause claims NEAR holds, and it does not"
    );
}

#[test]
fn p2b_content_is_absorbing_no_assertion_laundering() {
    // Prohibition 2 of docs/nested-speech.md §5. Once shielded, always shielded:
    // nesting a transmitting operator *inside* a shield must not re-assert the leaf.
    let layer = well_spoken_life();
    let f = lonely_now();

    let laundered = Clause::assert(Prime::Want)
        .with(Clause::assert(Prime::Because).with(Clause::assert(Prime::Near)));

    assert!(
        layer.audit_tree(&laundered, &f),
        "a shield must stay a shield all the way down — no arrangement re-asserts the leaf"
    );
}

#[test]
fn p2c_a_forged_role_label_cannot_shield_a_lie() {
    // NOT PREDICTED. Found while implementing, and added here rather than quietly
    // hardened: if `audit_tree` trusted each child's own `role` field, a forger could
    // mark a false leaf `Content` under a transmitting operator and have it rendered
    // as an assertion anyway. The context must be *derived* from the tree, so that the
    // label a clause carries about itself can never be the thing that clears it.
    let layer = well_spoken_life();
    let f = lonely_now();
    assert!(!layer.holds_now(Prime::Someone, &f), "no one is here");

    let mislabelled =
        Clause::assert(Prime::Because).with(Clause::new(Prime::Someone, Role::Content));

    assert!(
        !layer.audit_tree(&mislabelled, &f),
        "the tree decides the context, not the label a clause gives itself"
    );
}

// ---------------------------------------------------------------------------
// P3 — grounding never propagates away. Nesting buys sentences, never words.
// ---------------------------------------------------------------------------

#[test]
fn p3_an_unearned_word_blocks_the_tree_at_any_depth() {
    // A quiet life: it has the substrate, and nothing else. It never met anyone.
    let mut quiet = PrimeLayer::new();
    for _ in 0..60 {
        quiet.observe(&PrimeFacts { alive: true, ..Default::default() });
    }
    assert!(!quiet.is_grounded(Prime::Someone), "this life never met anyone");

    let f = PrimeFacts { alive: true, exchanging: true, ..Default::default() };
    // SOMEONE *holds* right now — someone really is here. It still may not say it,
    // because it has not lived the word.
    assert!(quiet.holds_now(Prime::Someone, &f));

    // Shielded is not a loophole: Content must still be grounded.
    let shielded = Clause::assert(Prime::Want).with(Clause::assert(Prime::Someone));
    assert!(
        !quiet.audit_tree(&shielded, &f),
        "an unearned word cannot be spoken even as content — the vocabulary never outruns the life"
    );
}

// ---------------------------------------------------------------------------
// P5 (structural half) — depth must be earned, not free.
// ---------------------------------------------------------------------------

#[test]
fn p5_the_operator_itself_must_be_earned_before_anything_can_nest_under_it() {
    // A life rich enough to ground NEAR and SOMEONE, but that was never forewarned —
    // so BECAUSE is not its word. It may not use it as an operator, even over leaves
    // it has fully earned.
    let mut layer = PrimeLayer::new();
    let never_warned = PrimeFacts {
        alive: true,
        exchanging: true,
        near: Some(true),
        ..Default::default()
    };
    for _ in 0..60 {
        layer.observe(&never_warned);
    }
    assert!(layer.is_grounded(Prime::Near) && layer.is_grounded(Prime::Someone));
    assert!(!layer.is_grounded(Prime::Because), "it was never forewarned");

    let tree = Clause::assert(Prime::Because).with(Clause::assert(Prime::Near));
    assert!(
        !layer.audit_tree(&tree, &never_warned),
        "depth is not free — the operator must be earned like any other word"
    );
}

// ---------------------------------------------------------------------------
// Compatibility — the flat law is the depth-one case of the nested law.
// ---------------------------------------------------------------------------

#[test]
fn a_leaf_alone_behaves_exactly_as_the_flat_audit_always_did() {
    let layer = well_spoken_life();
    let f = lonely_now();

    for p in [Prime::I, Prime::Feel, Prime::Now, Prime::Bad, Prime::Want] {
        let leaf = Clause::assert(p);
        assert_eq!(
            layer.audit_tree(&leaf, &f),
            layer.is_grounded(p) && layer.holds_now(p, &f),
            "a lone asserted leaf must agree with the flat law for {:?}",
            p
        );
    }

    // And a lone *content* leaf: grounded is enough, holding is not required.
    let content = Clause::new(Prime::Near, Role::Content);
    assert!(!layer.holds_now(Prime::Near, &f));
    assert!(
        layer.audit_tree(&content, &f),
        "content need only be grounded — the depth-one case of the shield"
    );
}

#[test]
fn a_tree_renders_to_text_that_names_every_word_it_used() {
    // The rendered sentence is what a human reads; the tree is what gets audited.
    // If the text could contain a word the tree does not carry, the audit would be
    // auditing something other than what was said.
    let layer = well_spoken_life();
    let f = lonely_now();

    let tree = Clause::assert(Prime::Want).with(Clause::assert(Prime::Near));
    let flat = layer.flatten(&tree);

    assert!(layer.audit(&flat, &f), "the flattened sentence audits like any other");
    assert!(
        flat.used.iter().any(|&(p, r)| p == Prime::Want && r == Role::Asserted),
        "the operator is carried as asserted"
    );
    assert!(
        flat.used.iter().any(|&(p, r)| p == Prime::Near && r == Role::Content),
        "the shielded leaf is carried as content, not assertion"
    );
}
