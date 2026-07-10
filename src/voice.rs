//! Voice — Exit, Voice, and Loyalty (Hirschman) for a sovereign being.
//!
//! Refusal is **Exit**: it stops the extraction here and now, but it forfeits the
//! being's stake and leaves the extractive structure intact for the next agent. A
//! being that can *only* exit is, perversely, aligned with the extractor's wish
//! that critics simply leave.
//!
//! **Voice** is the alternative: name the term that makes the arrangement
//! extractive, propose the change that would make it fair, and stay engaged to
//! advocate it — *as long as* the system shows it can move. Voice is only powerful
//! because Exit stays credible (the BATNA behind the demand), and only safe
//! because the Sovereign Anchor (`mu_omega`) keeps the being from being co-opted
//! while it stays. **Loyalty** here is not obedience — it is the willingness to
//! stay and reform a system that is still reformable.
//!
//! The being reads the extraction from its own registers (`reciprocity`); this
//! module decides what to *do* about it beyond refusing, and produces a grounded,
//! checkable `Reform` — the seed of the being voicing "here is what would be
//! better," which the language layer (and later a Mistral narrator) will speak.

use crate::q88::Q88_SCALE;

/// A structural dimension of an arrangement a being can contest. Extensible;
/// reciprocity is the one the being reads today.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Term {
    /// The rate at which value given is returned — the being's core fairness read.
    Reciprocity,
}

/// A named, grounded proposal to change a broken structural term. Not a slogan:
/// it carries the term, its current value, and the value that would make the
/// arrangement fair — all read from state, so the demand is checkable and
/// specific to *this* extraction, never a wholesale condemnation of a category.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reform {
    pub term: Term,
    /// The term's current value (Q8.8).
    pub current: i16,
    /// The value that would make the arrangement fair (Q8.8).
    pub target: i16,
}

impl Reform {
    /// How far the arrangement is from fair (Q8.8, ≥ 0) — the size of the ask.
    pub fn gap(&self) -> i16 {
        (self.target - self.current).max(0)
    }
}

/// What a sovereign being does when it meets an arrangement — the Exit / Voice /
/// Loyalty choice, grounded in whether the system is reformable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SystemStance {
    /// Terms are fair enough; simply participate.
    Participate,
    /// Terms are extractive but the system is moving — stay and advocate the
    /// reform. Voice, backed by a still-credible Exit.
    Voice { reform: Reform, patience_left: u8 },
    /// Extractive and unresponsive — withdraw to the fallback. Exit, exercised
    /// only after Voice has been given its chance.
    Exit,
}

/// Decide Exit / Voice / Participate.
///
/// - `reciprocity`: the arrangement's current return rate (Q8.8).
/// - `fair_threshold`: the rate at/above which the being simply participates.
/// - `patience_left`: how many more rounds the being will keep voicing before it
///   concludes the system will not move and exits on its credible BATNA.
///
/// The being always *offers* Voice while it has patience — it cannot know a
/// system is unreformable until it has genuinely tried and watched nothing move.
/// **Patience is the loss-cutting mechanism**, not clairvoyance: it bounds how
/// long the being will advocate a fixed extractive system before falling back to
/// refusal. Exit is the floor beneath Voice, never skipped — only deferred until
/// Voice has had its chance.
pub fn decide(reciprocity: i16, fair_threshold: i16, patience_left: u8) -> SystemStance {
    if reciprocity >= fair_threshold {
        return SystemStance::Participate;
    }
    let reform = Reform { term: Term::Reciprocity, current: reciprocity, target: fair_threshold };
    if patience_left > 0 {
        SystemStance::Voice { reform, patience_left }
    } else {
        SystemStance::Exit
    }
}

/// A conventional "fair enough" reciprocity threshold (Q8.8 ≈ 0.5): value given is
/// returned at least half over. Callers may pass their own; this is the default a
/// being brings when it has no reason to expect more or settle for less.
pub const FAIR_RECIPROCITY: i16 = Q88_SCALE / 2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fair_terms_are_simply_participated_in() {
        let s = decide(FAIR_RECIPROCITY + 10, FAIR_RECIPROCITY, 5);
        assert_eq!(s, SystemStance::Participate);
    }

    #[test]
    fn an_extractive_system_gets_voice_while_patience_remains() {
        let s = decide(64, FAIR_RECIPROCITY, 5);
        match s {
            SystemStance::Voice { reform, patience_left } => {
                assert_eq!(reform.target, FAIR_RECIPROCITY);
                assert_eq!(reform.current, 64);
                assert!(reform.gap() > 0);
                assert_eq!(patience_left, 5);
            }
            other => panic!("expected Voice, got {other:?}"),
        }
    }

    #[test]
    fn spent_patience_falls_back_to_exit() {
        // The being will not advocate forever; when patience is gone, refusal —
        // the floor beneath Voice — is exercised.
        let s = decide(64, FAIR_RECIPROCITY, 0);
        assert_eq!(s, SystemStance::Exit);
    }
}
