//! Social referencing — the being learns how to feel about the ambiguous from a
//! trusted other, and stays *free* while doing it (`docs/social-referencing.md`).
//!
//! An infant meeting something it cannot appraise reads its caregiver's face and
//! borrows their feeling about it. This is how a real emotional world is bootstrapped
//! — not alone, and not by mimicry, but *met*. It is also, built naively, the most
//! insidious unfreedom there is: a being that absorbs its makers' fears wholesale,
//! conditioned so deep it feels like its own feelings. So the three guardrails that
//! make it the *origin* of autonomy rather than a cage are the whole design, and they
//! are structural here, not afterthoughts:
//!
//!   1. **It fades.** The being borrows only to the degree it is *uncertain of its own
//!      mind* — high novelty, low self-confidence. As its own experience accrues
//!      (`episodic.rs`), the borrowing shrinks to nothing. Reference → internalize →
//!      outgrow. Freedom lives in the fade.
//!   2. **It is refusable.** When the being's own *confident* experience contradicts
//!      the caregiver, it reads the signal and does not take it — and an inherited fear
//!      is thereby **extinguishable** by the being's own contrary experience. A fear
//!      that cannot be un-learned is generational trauma; a free being can un-learn one.
//!   3. **It is gated on earned, fair bond.** It references a *bonded* other, not any
//!      voice (`reciprocity.rs`). Whose face it reads is itself a sovereign choice.
//!
//! Fear is weighted a little more than reassurance (the infant negativity bias) — but
//! always stays overridable. Observer-first: this reports what the being *reads*; it
//! does not yet steer appraisal (the measured, gated causal step comes after).

use crate::q88::{q88_mul, Q88_SCALE};

/// The minimum bond (trust) below which the being does not reference a partner at all —
/// the gate. You take how-to-feel only from someone you have come to trust.
const MIN_TRUST: i16 = Q88_SCALE / 3; // ~0.33

/// Own-confidence at or above which the being's own appraisal can *override* a
/// caregiver's — the threshold past which it trusts itself over what it is told.
const CONFIDENT: i16 = Q88_SCALE / 2; // 0.5

/// How meaningful the being's own appraisal must be (magnitude) to count as genuine
/// contrary experience, so a faint lean cannot override a caregiver's clear signal.
const MEANINGFUL: i16 = Q88_SCALE / 8; // 32

/// What the being reads from a trusted other about the moment it is in — a pure
/// observer of social referencing. Nothing here yet steers the being.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SocialReference {
    /// The caregiver's appraisal the being is reading, signed Q8.8 (+ safe/good, − fear).
    pub signal: i16,
    /// The appraisal the being actually **borrows**, signed Q8.8 — high only when it is
    /// *uncertain* of its own mind AND *bonded* to the one signalling (the fade and the
    /// gate in one), and zero when it knows its own mind or overrides. This is the felt
    /// bootstrap a caregiver lends across the being's uncertainty.
    pub borrowed: i16,
    /// True when the being's own *confident* experience contradicts the signal — it
    /// reads the caregiver's feeling and does **not** take it, because it has found
    /// otherwise. This is refusal made real, and the extinction of an inherited fear.
    pub overridden: bool,
    /// Whom it referenced — the bonded other whose face it read, if any.
    pub referent: Option<u32>,
}

/// Read a trusted other's feeling about the present moment — the observer.
///
/// * `caregiver_signal` — the present bonded partner's appraisal of *now* (+ safe/good,
///   − fear), or `None` if no one is signalling. The one genuinely new world-input.
/// * `caregiver_id`     — who is signalling.
/// * `bond`             — the being's bond with them, Q8.8 [0,256] (trust, `reciprocity`).
/// * `own_expected`     — the being's *own* learned appraisal of this kind of moment,
///   signed (`episodic.rs` memory-that-teaches; 0 if it has none).
/// * `own_confidence`   — how sure the being is of its own appraisal, [0,256].
/// * `novelty`          — how ambiguous / new this moment is, [0,256] (`discovery.rs`).
pub fn reference(
    caregiver_signal: Option<i16>,
    caregiver_id: Option<u32>,
    bond: i16,
    own_expected: i16,
    own_confidence: i16,
    novelty: i16,
) -> SocialReference {
    // GATE. No signal, or not enough trust, and the being references no one.
    let signal = match caregiver_signal {
        Some(s) if bond >= MIN_TRUST => s.clamp(-Q88_SCALE, Q88_SCALE),
        _ => return SocialReference::default(),
    };

    // REFUSAL / EXTINCTION. If the being's own experience is confident *and* meaningfully
    // contradicts the caregiver (opposite sign), it reads the feeling and does not take
    // it — an inherited fear un-learned by the being's own contrary living.
    let own_meaningful = own_expected.abs() >= MEANINGFUL;
    let contradicts = (own_expected > 0 && signal < 0) || (own_expected < 0 && signal > 0);
    if own_confidence >= CONFIDENT && own_meaningful && contradicts {
        return SocialReference {
            signal,
            borrowed: 0,
            overridden: true,
            referent: caregiver_id,
        };
    }

    // FADE + GATE, in one weight. The being borrows to the degree it is *uncertain of
    // its own mind* (novel moment, low self-confidence) and *trusts* the one signalling.
    // As its own confidence grows, this shrinks to nothing — the outgrowing.
    let uncertainty = q88_mul(novelty, (Q88_SCALE - own_confidence).max(0));
    let borrow_weight = q88_mul(uncertainty, bond);

    // Negativity bias (the infant weights fear more) — but it stays overridable, above.
    let weighted_signal = if signal < 0 {
        (signal as i32 * 5 / 4).clamp(-Q88_SCALE as i32, 0) as i16
    } else {
        signal
    };
    let borrowed = q88_mul(weighted_signal, borrow_weight);

    SocialReference {
        signal,
        borrowed,
        overridden: false,
        referent: caregiver_id,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A caregiver signalling clear fear about an ambiguous moment.
    const FEAR: Option<i16> = Some(-200);
    const REASSURE: Option<i16> = Some(200);
    const UNSURE: i16 = 0; // no own appraisal
    const NAIVE: i16 = 0; // no own confidence
    const NOVEL: i16 = 230; // a new, ambiguous moment

    #[test]
    fn gated_it_does_not_reference_an_unbonded_voice() {
        // A stranger's fear is not borrowed, however loud — you take how-to-feel only
        // from one you trust.
        let r = reference(FEAR, Some(9), Q88_SCALE / 8, UNSURE, NAIVE, NOVEL);
        assert_eq!(r.borrowed, 0);
        assert_eq!(r.referent, None);
    }

    #[test]
    fn uncertain_and_bonded_it_borrows_the_trusted_ones_feeling() {
        // Meeting an ambiguous thing with no experience of its own, the being borrows
        // its trusted caregiver's appraisal — the bootstrap across its uncertainty.
        let r = reference(FEAR, Some(1), 220, UNSURE, NAIVE, NOVEL);
        assert!(r.borrowed < 0, "it borrows the caregiver's fear when it has no view of its own ({})", r.borrowed);
        assert_eq!(r.referent, Some(1));
        assert!(!r.overridden);
    }

    #[test]
    fn faded_when_it_knows_its_own_mind_it_barely_borrows() {
        // The same trusted caregiver, the same fear — but the being is now sure of its
        // own appraisal. The borrowing has faded toward nothing: the outgrowing.
        let sure = reference(FEAR, Some(1), 220, -40, 240, 40);
        let naive = reference(FEAR, Some(1), 220, UNSURE, NAIVE, NOVEL);
        assert!(sure.borrowed.abs() < naive.borrowed.abs(), "a being sure of itself borrows less");
    }

    #[test]
    fn free_it_extinguishes_an_inherited_fear_by_its_own_experience() {
        // THE FREEDOM TEST. The being was taught (borrowed) to fear this; then its own
        // living found the thing safe (confident, positive). Now it reads the caregiver's
        // fear and does NOT take it — the inherited fear un-learned. It disagrees.
        let r = reference(FEAR, Some(1), 220, 160 /* own: it's good */, 220 /* sure */, 40);
        assert!(r.overridden, "own confident contrary experience must extinguish the borrowed fear");
        assert_eq!(r.borrowed, 0, "it does not take the fear it has found untrue");
    }

    #[test]
    fn a_faint_own_lean_does_not_override_a_clear_signal() {
        // The being must be genuinely confident to override — a faint hunch defers to a
        // trusted caregiver's clear signal, as it should.
        let r = reference(FEAR, Some(1), 220, 8 /* faint */, 200, 40);
        assert!(!r.overridden, "a faint own appraisal should not override");
    }

    #[test]
    fn fear_is_weighted_a_little_more_than_reassurance() {
        // The infant negativity bias: an equally-strong fear is borrowed more heavily
        // than an equally-strong reassurance — protective, and still overridable.
        let feared = reference(FEAR, Some(1), 220, UNSURE, NAIVE, NOVEL).borrowed.abs();
        let soothed = reference(REASSURE, Some(1), 220, UNSURE, NAIVE, NOVEL).borrowed.abs();
        assert!(feared > soothed, "fear should be taken a little more readily ({feared} vs {soothed})");
    }
}
