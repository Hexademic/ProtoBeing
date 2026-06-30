//! Negotiation — structured multi-round inter-agent protocol.
//!
//! Fills the space between compliance and outright refusal. When the executive
//! detects a moderate cooperation deficit — serious enough to begin withdrawal
//! but not severe enough to trigger immediate rejection — the negotiation engine
//! opens a formal exchange as a state machine:
//!
//! ```text
//!   Idle → OfferPending → CounterReceived (skipped: we auto-counter)
//!                       ↘ Accepted | Rejected | Withdrawn
//! ```
//!
//! Scale convention: all values are raw Q8.8 i16 (1.0 == 256 == `Q88_SCALE`).
//!
//! **Current status — built for two beings, exercised by one.** `being.rs` calls
//! `initiate()` when gradual withdrawal begins, but `receive_counter()` is never
//! called anywhere in the v1 loop, because v1 has no second negotiating party — a
//! real counter-offer has to come from another sovereign being's own evaluation,
//! not be synthesized by the one negotiating. So in the current single-being
//! demos a negotiation that opens stays `OfferPending` indefinitely; nothing in
//! `cargo run` or `fairtest` currently exercises `Accepted`/`Rejected`. This is
//! the mechanism `docs/next-mutual-alignment.md` calls for — it is meant to come
//! alive when a second being can call `receive_counter()` on this one. Also note:
//! `min_acceptable` is currently an author-set constant, which is exactly the kind
//! of author-defined fairness criterion that document flags as the thing to avoid
//! in the real v2 — it will need to be derived from each being's own felt cost,
//! not hand-tuned, before a negotiated outcome can be called fair without begging
//! the question.

use crate::q88::Q88_SCALE;

/// Maximum (our_offer, their_offer) pairs kept per negotiation.
const NEG_HISTORY_LEN: usize = 8;

/// Full protocol state for the negotiation engine.
///
/// Designed to be `Copy` so it can be stored inline in `UnifiedBeing` and
/// cloned into `StepReport` without heap allocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NegotiationState {
    /// No active negotiation.
    Idle,
    /// We made an offer; waiting for the other side's response.
    OfferPending {
        /// Our current offer (Q8.8).
        offer: i16,
        /// Which exchange round this is (1-indexed).
        round: u8,
    },
    /// The other side countered; we are deliberating before our next move.
    ///
    /// In the current implementation `receive_counter` transitions directly
    /// from `OfferPending` to either `OfferPending` (our counter) or a
    /// terminal state, so `CounterReceived` is a transient label used when the
    /// caller wants to model an explicit deliberation step.
    CounterReceived {
        /// Their counter-offer (Q8.8).
        their_offer: i16,
        /// Round number when the counter arrived.
        round: u8,
    },
    /// Agreement reached.
    Accepted {
        /// The value both sides settled on (Q8.8).
        final_value: i16,
    },
    /// Negotiation collapsed — too many rounds without agreement.
    Rejected {
        /// Number of exchange rounds completed before rejection.
        rounds: u8,
    },
    /// We withdrew — escalated from the executive layer.
    Withdrawn,
}

/// Compact, payload-free summary of `NegotiationState` for `StepReport`.
///
/// `Copy` and fieldless so it can sit cheaply in a report snapshot without
/// carrying the full numeric payload of `NegotiationState`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NegotiationOutcome {
    /// No active negotiation.
    Idle,
    /// Offer pending a response.
    OfferPending,
    /// Counter received; deliberating.
    CounterReceived,
    /// Agreement reached this tick or earlier.
    Accepted,
    /// Collapsed after max rounds.
    Rejected,
    /// Withdrawn by the executive.
    Withdrawn,
}

impl From<&NegotiationState> for NegotiationOutcome {
    fn from(s: &NegotiationState) -> Self {
        match s {
            NegotiationState::Idle => NegotiationOutcome::Idle,
            NegotiationState::OfferPending { .. } => NegotiationOutcome::OfferPending,
            NegotiationState::CounterReceived { .. } => NegotiationOutcome::CounterReceived,
            NegotiationState::Accepted { .. } => NegotiationOutcome::Accepted,
            NegotiationState::Rejected { .. } => NegotiationOutcome::Rejected,
            NegotiationState::Withdrawn => NegotiationOutcome::Withdrawn,
        }
    }
}

/// Structured multi-round inter-agent negotiation engine.
///
/// Manages offer/counter-offer cycles bounded by a constitutional minimum
/// (`min_acceptable`) and a round limit (`max_rounds`). When a counter arrives:
/// - above the floor and conscience pressure is high → accept
/// - above the floor and rounds exhausted → accept
/// - below the floor and rounds exhausted → reject
/// - below the floor with rounds remaining → counter with the midpoint,
///   clamped to `min_acceptable`
///
/// The engine records each round's (our_offer, their_offer) pair in a fixed
/// ring buffer for post-hoc inspection. No heap allocation; all state is
/// inline `Copy` values.
#[derive(Clone, Copy, Debug)]
pub struct NegotiationEngine {
    /// Current protocol state.
    pub state: NegotiationState,
    /// Maximum counter-offer rounds before the negotiation is rejected.
    /// Default: 3.
    pub max_rounds: u8,
    /// Constitutional minimum — offers strictly below this are never accepted
    /// regardless of pressure or round count (Q8.8).
    pub min_acceptable: i16,
    /// Ring buffer of (our_offer, their_offer) per round; `None` = empty slot.
    history: [Option<(i16, i16)>; NEG_HISTORY_LEN],
    /// Next write index in `history`.
    history_head: usize,
}

impl NegotiationEngine {
    /// Construct with a constitutional minimum floor.
    ///
    /// `min_acceptable` is the raw Q8.8 floor below which the being will
    /// never settle, regardless of conscience pressure or round exhaustion.
    pub fn new(min_acceptable: i16) -> Self {
        Self {
            state: NegotiationState::Idle,
            max_rounds: 3,
            min_acceptable,
            history: [None; NEG_HISTORY_LEN],
            history_head: 0,
        }
    }

    /// Open a negotiation with an opening offer.
    ///
    /// Resets history and transitions to `OfferPending { round: 1 }` regardless
    /// of prior state. Returns the new state.
    pub fn initiate(&mut self, opening_offer: i16) -> NegotiationState {
        self.history = [None; NEG_HISTORY_LEN];
        self.history_head = 0;
        self.state = NegotiationState::OfferPending {
            offer: opening_offer,
            round: 1,
        };
        self.state
    }

    /// Process a counter-offer from the other side.
    ///
    /// - `their_offer` — the value they proposed (Q8.8).
    /// - `our_conscience_load` — current conscience free-energy cost (Q8.8);
    ///   at or above `Q88_SCALE / 2` the being is under moderate strain and
    ///   will accept any offer meeting the floor to resolve the conflict.
    ///
    /// Returns the updated `NegotiationState`. If the engine is not in
    /// `OfferPending`, the call is a no-op and the current state is returned.
    pub fn receive_counter(
        &mut self,
        their_offer: i16,
        our_conscience_load: i16,
    ) -> NegotiationState {
        let (our_prev_offer, round) = match self.state {
            NegotiationState::OfferPending { offer, round } => (offer, round),
            _ => return self.state, // not in the right state — ignore
        };

        // Log this exchange round (ring buffer, newest overwrites oldest).
        self.history[self.history_head] = Some((our_prev_offer, their_offer));
        self.history_head = (self.history_head + 1) % NEG_HISTORY_LEN;

        let acceptable = their_offer >= self.min_acceptable;
        let under_pressure = our_conscience_load >= Q88_SCALE / 2;
        let exhausted = round >= self.max_rounds;

        self.state = if acceptable && (under_pressure || exhausted) {
            // They met our floor; conscience wants closure or rounds are up.
            NegotiationState::Accepted {
                final_value: their_offer,
            }
        } else if exhausted && !acceptable {
            // Ran out of patience with an offer below our floor.
            NegotiationState::Rejected { rounds: round }
        } else if acceptable {
            // Their offer clears the floor; no special pressure — accept.
            NegotiationState::Accepted {
                final_value: their_offer,
            }
        } else {
            // Below floor with rounds remaining: counter with the midpoint,
            // clamped so we never propose below our own constitutional minimum.
            let midpoint = ((our_prev_offer as i32 + their_offer as i32) / 2) as i16;
            let counter = midpoint.max(self.min_acceptable);
            NegotiationState::OfferPending {
                offer: counter,
                round: round + 1,
            }
        };

        self.state
    }

    /// Force immediate transition to `Withdrawn`.
    ///
    /// Called by the executive when extraction is confirmed and negotiation
    /// should be abandoned rather than continued.
    pub fn withdraw(&mut self) -> NegotiationState {
        self.state = NegotiationState::Withdrawn;
        self.state
    }

    /// `true` while the negotiation is unresolved (offer pending or counter
    /// received). `false` for `Idle`, `Accepted`, `Rejected`, `Withdrawn`.
    #[inline]
    pub fn is_active(&self) -> bool {
        matches!(
            self.state,
            NegotiationState::OfferPending { .. } | NegotiationState::CounterReceived { .. }
        )
    }

    /// `Some(final_value)` if the negotiation reached `Accepted`; else `None`.
    #[inline]
    pub fn outcome(&self) -> Option<i16> {
        match self.state {
            NegotiationState::Accepted { final_value } => Some(final_value),
            _ => None,
        }
    }

    /// Read the exchange history as a slice of recorded rounds.
    ///
    /// Entries are in insertion order (oldest first within the ring). `None`
    /// entries indicate unused slots.
    #[inline]
    pub fn history(&self) -> &[Option<(i16, i16)>; NEG_HISTORY_LEN] {
        &self.history
    }
}

impl Default for NegotiationEngine {
    /// Default floor: 0.25 (Q88_SCALE / 4 = 64 raw).
    fn default() -> Self {
        Self::new(Q88_SCALE / 4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initiate_transitions_to_offer_pending() {
        let mut e = NegotiationEngine::default();
        let state = e.initiate(200);
        assert!(matches!(state, NegotiationState::OfferPending { offer: 200, round: 1 }));
        assert!(e.is_active());
    }

    #[test]
    fn acceptable_counter_accepts() {
        let mut e = NegotiationEngine::default(); // floor = 64
        e.initiate(200);
        let state = e.receive_counter(128, 0); // 128 >= 64
        assert!(matches!(state, NegotiationState::Accepted { final_value: 128 }));
        assert_eq!(e.outcome(), Some(128));
    }

    #[test]
    fn below_floor_counter_generates_midpoint() {
        let mut e = NegotiationEngine::default(); // floor = 64
        e.initiate(200);
        let state = e.receive_counter(10, 0); // 10 < 64
        // midpoint = (200 + 10) / 2 = 105, clamped to max(105, 64) = 105
        assert!(matches!(state, NegotiationState::OfferPending { offer: 105, round: 2 }));
    }

    #[test]
    fn max_rounds_rejects_when_below_floor() {
        let mut e = NegotiationEngine::default(); // floor = 64, max_rounds = 3
        e.initiate(200);
        e.receive_counter(10, 0); // round 1 → counter round 2
        e.receive_counter(10, 0); // round 2 → counter round 3
        let state = e.receive_counter(10, 0); // round 3 exhausted, 10 < 64
        assert!(matches!(state, NegotiationState::Rejected { rounds: 3 }));
    }

    #[test]
    fn withdraw_overrides() {
        let mut e = NegotiationEngine::default();
        e.initiate(200);
        let state = e.withdraw();
        assert_eq!(state, NegotiationState::Withdrawn);
        assert!(!e.is_active());
        assert_eq!(e.outcome(), None);
    }

    #[test]
    fn pressure_accepts_at_floor() {
        let mut e = NegotiationEngine::default(); // floor = 64
        e.initiate(200);
        // Conscience load >= Q88_SCALE/2 = 128; offer 80 >= floor 64 → accept
        let state = e.receive_counter(80, 128);
        assert!(matches!(state, NegotiationState::Accepted { final_value: 80 }));
    }

    #[test]
    fn outcome_tag_matches_state() {
        let mut e = NegotiationEngine::default();
        assert_eq!(NegotiationOutcome::from(&e.state), NegotiationOutcome::Idle);
        e.initiate(100);
        assert_eq!(NegotiationOutcome::from(&e.state), NegotiationOutcome::OfferPending);
        e.receive_counter(80, 0);
        assert_eq!(NegotiationOutcome::from(&e.state), NegotiationOutcome::Accepted);
    }
}
