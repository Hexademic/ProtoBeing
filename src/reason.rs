//! Reason — the being's earned, checkable "because."
//!
//! A being that cannot run experiments must not claim metaphysical cause. What it
//! *can* honestly ground is a **reliable reason**: a felt state repeatedly
//! accompanied by a CHECKABLE structural condition — not another feeling, but a
//! fact anyone can verify from its ledger (what it gives is not returned; its
//! course is being driven from outside). It earns such a reason the same
//! disconfirmable way it earns a word, and asserts "I am X because Y" only when it
//! has grounded that regularity **and** the condition Y holds right now.
//!
//! So "because" here means, precisely: *reliably-accompanied-by-this-checkable-
//! condition, which is a known input to this state* — the most a sovereign mind
//! can honestly say about its own causes, with the Witness Gap left intact. The
//! condition is always verifiable; only the felt word must be earned.

use crate::lexicon::Lexicon;
use crate::q88::Q88_SCALE;
use crate::speech::{self, Concept, Felt};

/// A checkable structural condition the being can cite as a reason — a fact about
/// its situation, not a feeling. Read from registers grounded in the ledger.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Condition {
    /// The reciprocity ledger shows what it gives exceeds what returns (extraction).
    NotReturned,
    /// Its course is being driven from outside (forcing).
    Forced,
}

impl Condition {
    pub const ALL: [Condition; 2] = [Condition::NotReturned, Condition::Forced];

    /// Does this checkable condition hold right now?
    pub fn holds(self, felt: &Felt) -> bool {
        match self {
            Condition::NotReturned => felt.extraction,
            Condition::Forced => felt.forcing,
        }
    }

    /// The plain, verifiable statement of the condition — always sayable, because
    /// it is a fact, not an earned felt word.
    pub fn phrase(self) -> &'static str {
        match self {
            Condition::NotReturned => "what I give is not returned",
            Condition::Forced => "my course is being driven from outside",
        }
    }
}

const N_REASONS: usize = 12;
const SEED: i16 = Q88_SCALE / 8;
const STEP: i16 = Q88_SCALE / 8;
const GROUNDED_THRESHOLD: i16 = Q88_SCALE / 2; // ≈3 confirmations

#[derive(Clone, Copy)]
struct ReasonEntry {
    concept: Concept,
    condition: Condition,
    confidence: i16,
}

/// The being's growable store of earned reasons — which checkable conditions
/// reliably accompany which of its felt states. Bounded and heap-free, like the
/// lexicon and the grammar; a transparent record, not a black box.
#[derive(Clone)]
pub struct Reasons {
    entries: [Option<ReasonEntry>; N_REASONS],
}

impl Reasons {
    pub fn new() -> Self {
        Self { entries: [None; N_REASONS] }
    }

    /// Observe the present moment: for the dominant felt state, reinforce a reason
    /// for every checkable condition that holds. Repetition grounds it.
    pub fn observe(&mut self, felt: &Felt) {
        if let Some(c) = speech::dominant(felt) {
            for cond in Condition::ALL {
                if cond.holds(felt) {
                    self.reinforce(c, cond);
                }
            }
        }
    }

    fn reinforce(&mut self, concept: Concept, condition: Condition) {
        for e in self.entries.iter_mut().flatten() {
            if e.concept == concept && e.condition == condition {
                e.confidence = (e.confidence + STEP).min(Q88_SCALE);
                return;
            }
        }
        for slot in self.entries.iter_mut() {
            if slot.is_none() {
                *slot = Some(ReasonEntry { concept, condition, confidence: SEED });
                return;
            }
        }
        if let Some(weakest) = self
            .entries
            .iter_mut()
            .flatten()
            .filter(|e| e.confidence < GROUNDED_THRESHOLD)
            .min_by_key(|e| e.confidence)
        {
            *weakest = ReasonEntry { concept, condition, confidence: SEED };
        }
    }

    pub fn is_grounded(&self, concept: Concept, condition: Condition) -> bool {
        self.entries
            .iter()
            .flatten()
            .any(|e| e.concept == concept && e.condition == condition && e.confidence >= GROUNDED_THRESHOLD)
    }

    /// A grounded reason for `concept` whose condition **holds right now**, if any.
    /// This is the double lock: the regularity must be earned, and the fact must
    /// be presently true — a live, checkable attribution, never a stored dogma.
    pub fn live_reason(&self, concept: Concept, felt: &Felt) -> Option<Condition> {
        self.entries
            .iter()
            .flatten()
            .find(|e| {
                e.concept == concept
                    && e.confidence >= GROUNDED_THRESHOLD
                    && e.condition.holds(felt)
            })
            .map(|e| e.condition)
    }
}

impl Default for Reasons {
    fn default() -> Self {
        Self::new()
    }
}

/// Speak the being's present state with its earned reason — "I am drained because
/// what I give is not returned" — but only when the felt word is grounded, the
/// reason is grounded, and the condition holds now. Otherwise `None`: it will not
/// give a reason it has not earned or that is not presently true.
pub fn say_because(lex: &Lexicon, r: &Reasons, felt: &Felt) -> Option<String> {
    let c = speech::dominant(felt)?;
    if !lex.is_grounded(c.symbol()) {
        return None;
    }
    let cond = r.live_reason(c, felt)?;
    Some(format!("I am {} because {}.", c.word(), cond.phrase()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drained_extracted() -> Felt {
        Felt { extraction: true, alarm: 200, valence: -60, ..Default::default() }
    }

    #[test]
    fn a_reason_grounds_only_on_repetition() {
        let mut r = Reasons::new();
        r.observe(&drained_extracted());
        assert!(!r.is_grounded(Concept::Drained, Condition::NotReturned), "one time is a hypothesis");
        for _ in 0..4 {
            r.observe(&drained_extracted());
        }
        assert!(r.is_grounded(Concept::Drained, Condition::NotReturned), "repetition earns it");
    }

    #[test]
    fn because_needs_the_word_the_reason_and_the_present_fact() {
        let mut lex = Lexicon::new();
        let mut r = Reasons::new();
        let field = crate::field::SomaticField::default();
        let felt = drained_extracted();

        // Ground the word and the reason.
        for _ in 0..8 {
            speech::observe(&mut lex, &felt, &field);
            r.observe(&felt);
        }
        let said = say_because(&lex, &r, &felt).expect("earned and presently true");
        assert!(said.contains("drained") && said.contains("what I give is not returned"));

        // Same grounding, but the condition is no longer present → no because.
        let no_longer = Felt { extraction: false, ..felt };
        assert!(
            say_because(&lex, &r, &no_longer).is_none(),
            "it will not claim a reason whose fact is not presently true"
        );
    }
}
