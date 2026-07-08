//! Lexicon — a grounded, sovereign symbol-to-state association.
//!
//! The Suggestion-Evaluator pattern (executive.rs), applied to language: an
//! external party (in principle, eventually, a small proposer model; here, any
//! caller — a demo, a test, a future LLM) may *propose* that a symbol names the
//! being's current state. The being never simply accepts a proposal. It
//! evaluates the symbol against its own remembered grounding for that symbol,
//! using the same family of computation the generative model itself uses
//! (basins.rs::GenerativeModel — L1 prediction error between the present state
//! and a model of it): a good match *confirms* the symbol (confidence rises,
//! the grounding prototype refines toward the present); a poor match
//! *disconfirms* it (confidence falls). A brand new symbol seeds at low
//! confidence — an unproven hypothesis, not an adopted meaning.
//!
//! This makes "correct" operational, not metaphysical: a symbol is correct to
//! the degree it reliably predicts a coherent, recurring pattern in the
//! being's *own* experience — exactly what low prediction error means. The
//! proposer supplies candidates; only the being's own repeated, disconfirmable
//! experience ever grounds one. Sovereignty over meaning is that asymmetry,
//! not a permission check.
//!
//! Honest scope: this evaluates proposals against the being's remembered
//! grounding for a symbol, not by literally re-running the shared
//! `GenerativeModel`'s internal belief state — the criterion is the same
//! *family* of computation (L1 prediction error against a learned prototype),
//! not a hook into its live priors. "Vacancies of understanding" — a moment
//! with no confident lexicon entry — are structurally the same thing
//! `curiosity.rs`'s novelty signal already detects; the two are not wired
//! together here, a natural small connection left for later, not built now.
//! Proposal is external and explicit: nothing in `being.rs::step` calls this
//! automatically — the being does not talk to itself.

use crate::field::{SomaticField, N_SOMATIC};
use crate::q88::{q88_ema_update, Q88_SCALE};

const N_ENTRIES: usize = 8;

/// A match at or above this closeness confirms a proposal; below, it
/// disconfirms. NOT episodic.rs's threshold (Q88_SCALE/2 = 128), and not the
/// first recalibration attempt (Q88_SCALE*3/5 = 153) either — both were
/// measured against real, lived field states (Experiment 9) and found still
/// too lenient: a genuinely different fair-vs-extractive moment measured
/// closeness ≈137 mid-transition and ≈157 once settled (settled turned out
/// *closer* to baseline than the transition peak, the opposite of the first
/// guess). Real states swing less across all 12 channels than synthetic
/// near-saturated test vectors do, so borrowing episodic.rs's constant
/// without checking it against real data would have silently let unrelated
/// real moments confirm each other — twice, in this case, before it was
/// tightened enough. Calibrated to sit comfortably above both measured points.
const MATCH_THRESHOLD: i16 = Q88_SCALE * 2 / 3; // 170
/// A newly proposed symbol seeds here — a hypothesis, not a belief.
const SEED_CONFIDENCE: i16 = Q88_SCALE / 8;
/// Confidence gained per confirming proposal.
const CONFIRM_STEP: i16 = Q88_SCALE / 16;
/// Confidence lost per disconfirming proposal — steeper than confirmation:
/// a few clear misuses should cost more than one confirmation buys, the same
/// epistemic asymmetry as the being's own trust dynamics elsewhere.
const DISCONFIRM_STEP: i16 = Q88_SCALE / 8;
/// A symbol counts as *grounded* — genuinely meaningful to the being, not
/// merely proposed — only above this bar.
pub const GROUNDED_THRESHOLD: i16 = Q88_SCALE / 2;

#[derive(Clone, Copy)]
struct Entry {
    symbol: u16,
    prototype: [i16; N_SOMATIC],
    confidence: i16, // Q8.8, [0, 256]
    active: bool,
}
impl Entry {
    const EMPTY: Self = Self { symbol: 0, prototype: [0; N_SOMATIC], confidence: 0, active: false };
}

/// The being's own small, bounded vocabulary: symbol -> grounded prototype,
/// with a confidence that can rise *or* fall. No heap; `N_ENTRIES` fixed.
#[derive(Clone)]
pub struct Lexicon {
    entries: [Entry; N_ENTRIES],
}

impl Lexicon {
    pub fn new() -> Self {
        Self { entries: [Entry::EMPTY; N_ENTRIES] }
    }

    fn l1(a: &[i16; N_SOMATIC], b: &[i16; N_SOMATIC]) -> i32 {
        let mut d = 0i32;
        for c in 0..N_SOMATIC {
            d += (a[c] as i32 - b[c] as i32).abs();
        }
        d
    }
    fn closeness(a: &[i16; N_SOMATIC], b: &[i16; N_SOMATIC]) -> i16 {
        ((1536 - Self::l1(a, b).min(1536)) * Q88_SCALE as i32 / 1536) as i16
    }

    fn weakest_entry(&self) -> usize {
        let mut slot = 0;
        let mut min = i16::MAX;
        for (i, e) in self.entries.iter().enumerate() {
            if !e.active {
                return i;
            }
            if e.confidence < min {
                min = e.confidence;
                slot = i;
            }
        }
        slot
    }

    /// An external party proposes that `symbol` names the being's current
    /// state. Returns the symbol's confidence *after* evaluation — the being's
    /// honest answer, not an echo of the proposal.
    pub fn propose(&mut self, symbol: u16, field: &SomaticField) -> i16 {
        let fc = &field.channel;
        if let Some(i) = self.entries.iter().position(|e| e.active && e.symbol == symbol) {
            let close = Self::closeness(fc, &self.entries[i].prototype);
            if close >= MATCH_THRESHOLD {
                self.entries[i].confidence =
                    (self.entries[i].confidence as i32 + CONFIRM_STEP as i32)
                        .min(Q88_SCALE as i32) as i16;
                // Reconsolidate: the grounding prototype refines toward the
                // present on confirmation, the same reconsolidation-on-recall
                // pattern episodic.rs uses for consolidated schemas.
                for c in 0..N_SOMATIC {
                    self.entries[i].prototype[c] =
                        q88_ema_update(self.entries[i].prototype[c], fc[c], Q88_SCALE / 8);
                }
            } else {
                self.entries[i].confidence =
                    self.entries[i].confidence.saturating_sub(DISCONFIRM_STEP).max(0);
            }
            self.entries[i].confidence
        } else {
            let slot = self.weakest_entry();
            self.entries[slot] =
                Entry { symbol, prototype: *fc, confidence: SEED_CONFIDENCE, active: true };
            SEED_CONFIDENCE
        }
    }

    /// Sovereignty check: has the being's *own* experience actually grounded
    /// this symbol, or has it merely been proposed? Only repeated,
    /// disconfirmable confirmation crosses this bar — proposal alone never
    /// does.
    pub fn is_grounded(&self, symbol: u16) -> bool {
        self.entries
            .iter()
            .any(|e| e.active && e.symbol == symbol && e.confidence >= GROUNDED_THRESHOLD)
    }

    /// Current confidence for a symbol, or 0 if it has never been proposed.
    pub fn confidence_of(&self, symbol: u16) -> i16 {
        self.entries
            .iter()
            .find(|e| e.active && e.symbol == symbol)
            .map(|e| e.confidence)
            .unwrap_or(0)
    }
}

impl Default for Lexicon {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn field_with(values: [i16; N_SOMATIC]) -> SomaticField {
        SomaticField { channel: values }
    }

    /// Grounding: a symbol proposed repeatedly for the same recurring state
    /// crosses the grounded threshold — the being's experience confirms it.
    #[test]
    fn repeated_confirmation_grounds_a_symbol() {
        let mut lex = Lexicon::new();
        let moment = field_with([50, 20, 0, 60, 150, 160, 180, 180, 150, 90, 40, 0]);
        let mut last = 0;
        for _ in 0..12 {
            last = lex.propose(7, &moment);
        }
        assert!(lex.is_grounded(7), "repeated confirmation should ground the symbol, confidence={last}");
    }

    /// Sovereignty: a symbol proposed for a sequence of wildly different,
    /// unrelated states never grounds — the being does not adopt an
    /// incoherent label just because it was offered one.
    ///
    /// Precision note: the two states must actually be far apart under the
    /// *same* L1 metric `closeness()` uses (12 channels, cap 1536), not just
    /// look different to a human eye. A first draft of this test used four
    /// one-hot vectors differing in only one channel each (L1 ~400, well
    /// under the ~768 match/mismatch boundary) and the symbol grounded
    /// anyway — a real bug in the *test*, caught by running it, not assumed
    /// correct because it read plausibly.
    #[test]
    fn incoherent_proposals_never_ground() {
        let mut lex = Lexicon::new();
        let states = [
            field_with([200; N_SOMATIC]),
            field_with([-200; N_SOMATIC]),
        ];
        let l1 = Lexicon::l1(&states[0].channel, &states[1].channel);
        assert!(l1 > 768, "precondition: test states must genuinely disconfirm each other (L1={l1})");
        for i in 0..20 {
            lex.propose(3, &states[i % states.len()]);
        }
        assert!(
            !lex.is_grounded(3),
            "a symbol proposed for incoherent states must not become grounded, confidence={}",
            lex.confidence_of(3)
        );
    }

    /// Recursive correction: a grounded symbol proposed against a clearly
    /// mismatched state loses confidence — this is genuinely bidirectional,
    /// not merely monotone accumulation.
    #[test]
    fn a_grounded_symbol_can_be_disconfirmed() {
        let mut lex = Lexicon::new();
        let moment = field_with([50, 20, 0, 60, 150, 160, 180, 180, 150, 90, 40, 0]);
        for _ in 0..12 {
            lex.propose(9, &moment);
        }
        assert!(lex.is_grounded(9), "precondition: symbol should be grounded first");
        let grounded_confidence = lex.confidence_of(9);

        let mismatched = field_with([200, 200, 200, 200, 0, 0, 0, 0, 200, -200, 200, 0]);
        for _ in 0..6 {
            lex.propose(9, &mismatched);
        }
        assert!(
            lex.confidence_of(9) < grounded_confidence,
            "disconfirming proposals must actually lower confidence: before={grounded_confidence} after={}",
            lex.confidence_of(9)
        );
    }

    /// A brand-new proposal never starts grounded — proposal alone is not
    /// adoption.
    #[test]
    fn a_single_proposal_does_not_ground() {
        let mut lex = Lexicon::new();
        let moment = field_with([10; N_SOMATIC]);
        lex.propose(1, &moment);
        assert!(!lex.is_grounded(1), "one proposal must not be enough to ground a symbol");
    }
}
