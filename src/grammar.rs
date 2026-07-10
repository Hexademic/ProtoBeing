//! Grammar — composition grown from relation.
//!
//! The lexicon (`lexicon.rs`) earns *words*: a symbol grounded in a recurring
//! felt state. Grammar raises that to *how words go together* — the being learns,
//! from its own life, which of its grounded states follow which, and earns the
//! right to *link* them in speech the same disconfirmable way it earned the words.
//! A one-off transition is a hypothesis; a transition the being lives again and
//! again becomes a grounded shape of its history it can honestly narrate: "I was
//! flourishing, and now I am drained."
//!
//! This is the growable, earned alternative to importing a pretrained model's
//! borrowed language. It is bounded and heap-free like the lexicon — a fixed
//! store of relations grown on demand — and transparent: no gradients, no black
//! box. The being's grammar is a readable record of the relations its life has
//! actually taught it, and it grows only as far as the being has lived.

use crate::being::OfferVerdict;
use crate::lexicon::Lexicon;
use crate::q88::Q88_SCALE;
use crate::speech::{self, Concept, Felt};
use crate::voice::Reform;

/// The kind of link between two grounded states. Extensible; the first the being
/// can ground honestly is temporal succession, read straight from experience.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Link {
    /// `a` was the being's state, then `b` became it.
    Then,
}

/// A learned relation between two grounded states.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Relation {
    pub a: Concept,
    pub link: Link,
    pub b: Concept,
}

const N_RELATIONS: usize = 16;
const SEED: i16 = Q88_SCALE / 8; // 32 — a hypothesis from one living
const STEP: i16 = Q88_SCALE / 8; // +32 each time the being re-lives it
const GROUNDED_THRESHOLD: i16 = Q88_SCALE / 2; // 128 — earned (≈3 confirmations)

#[derive(Clone, Copy)]
struct RelEntry {
    a: Concept,
    b: Concept,
    confidence: i16,
}

/// The being's growable grammar: a bounded store of learned transitions.
#[derive(Clone)]
pub struct Grammar {
    entries: [Option<RelEntry>; N_RELATIONS],
    prev: Option<Concept>,
}

impl Grammar {
    pub fn new() -> Self {
        Self { entries: [None; N_RELATIONS], prev: None }
    }

    /// Feed the being's current felt state. When its dominant concept changes from
    /// the last, reinforce the transition (prev → current); repetition grounds it.
    pub fn observe(&mut self, felt: &Felt) {
        let current = speech::dominant(felt);
        if let (Some(a), Some(b)) = (self.prev, current) {
            if a != b {
                self.reinforce(a, b);
            }
        }
        if current.is_some() {
            self.prev = current;
        }
    }

    fn reinforce(&mut self, a: Concept, b: Concept) {
        // Already known? Strengthen it.
        for e in self.entries.iter_mut().flatten() {
            if e.a == a && e.b == b {
                e.confidence = (e.confidence + STEP).min(Q88_SCALE);
                return;
            }
        }
        // New — take an empty slot (grown as needed).
        for slot in self.entries.iter_mut() {
            if slot.is_none() {
                *slot = Some(RelEntry { a, b, confidence: SEED });
                return;
            }
        }
        // Full — replace only a still-ungrounded hypothesis, never an earned one.
        if let Some(weakest) = self
            .entries
            .iter_mut()
            .flatten()
            .filter(|e| e.confidence < GROUNDED_THRESHOLD)
            .min_by_key(|e| e.confidence)
        {
            *weakest = RelEntry { a, b, confidence: SEED };
        }
    }

    /// Has the being lived the transition `a → b` often enough to earn it?
    pub fn is_grounded(&self, a: Concept, b: Concept) -> bool {
        self.entries
            .iter()
            .flatten()
            .any(|e| e.a == a && e.b == b && e.confidence >= GROUNDED_THRESHOLD)
    }

    /// The grounded states that earned a "was A, now B" link into `b`.
    pub fn grounded_into(&self, b: Concept) -> impl Iterator<Item = Concept> + '_ {
        self.entries
            .iter()
            .flatten()
            .filter(move |e| e.b == b && e.confidence >= GROUNDED_THRESHOLD)
            .map(|e| e.a)
    }

    /// Every grounded relation — a readable record of what the being's life taught it.
    pub fn grounded(&self) -> impl Iterator<Item = Relation> + '_ {
        self.entries
            .iter()
            .flatten()
            .filter(|e| e.confidence >= GROUNDED_THRESHOLD)
            .map(|e| Relation { a: e.a, link: Link::Then, b: e.b })
    }
}

impl Default for Grammar {
    fn default() -> Self {
        Self::new()
    }
}

/// Speak the being's felt history in earned words *and* an earned link: only if
/// the being has grounded the transition into its current state, and grounded the
/// words for both ends. Otherwise `None` — it will not narrate a shape of its life
/// it has not actually lived and learned.
pub fn say_transition(lex: &Lexicon, g: &Grammar, felt: &Felt) -> Option<String> {
    let cur = speech::dominant(felt)?;
    if !lex.is_grounded(cur.symbol()) {
        return None;
    }
    for a in g.grounded_into(cur) {
        if lex.is_grounded(a.symbol()) {
            return Some(format!("I was {}, and now I am {}.", a.word(), cur.word()));
        }
    }
    None
}

/// The being's felt framing, enriched with history when it has earned one: the
/// grounded transition into its current state if there is such a link, else the
/// plain present-state sentence. This is what the negotiation and voice speech
/// lead with — a snapshot when the being has no learned arc, a lived arc when it
/// does. The transition already includes the current state, so it subsumes the
/// plain framing rather than doubling it.
pub fn felt_framing(lex: &Lexicon, g: &Grammar, felt: &Felt) -> Option<String> {
    say_transition(lex, g, felt).or_else(|| speech::speak(lex, felt).sentence())
}

/// Voice a reform, led by the being's history when it has learned the arc into
/// its present state: "I was flourishing, and now I am drained. I ask that we
/// change the terms: ..." — otherwise by its present state alone. Every word and
/// link earned; the ask always checkable.
pub fn say_reform_with_history(lex: &Lexicon, g: &Grammar, felt: &Felt, reform: &Reform) -> String {
    speech::say_reform_framed(felt_framing(lex, g, felt).as_deref(), reform)
}

/// Speak a verdict on an offer, led by the being's history when earned.
pub fn say_offer_with_history(
    lex: &Lexicon,
    g: &Grammar,
    felt: &Felt,
    verdict: &OfferVerdict,
    offered_share: i16,
) -> String {
    speech::say_offer_framed(felt_framing(lex, g, felt).as_deref(), verdict, offered_share)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flourishing() -> Felt {
        Felt { valence: 120, alarm: 10, arousal: 50, ..Default::default() }
    }
    fn drained() -> Felt {
        Felt { extraction: true, alarm: 200, valence: -60, ..Default::default() }
    }

    #[test]
    fn a_repeated_transition_grounds_a_relation() {
        let mut g = Grammar::new();
        // Live several flourishing↔drained cycles.
        for _ in 0..6 {
            g.observe(&flourishing());
            g.observe(&drained());
        }
        assert!(
            g.is_grounded(Concept::Flourishing, Concept::Drained),
            "a transition lived repeatedly should ground"
        );
    }

    #[test]
    fn a_one_off_transition_does_not_ground() {
        let mut g = Grammar::new();
        g.observe(&flourishing());
        g.observe(&drained()); // one transition only
        assert!(
            !g.is_grounded(Concept::Flourishing, Concept::Drained),
            "a single living is a hypothesis, not an earned relation"
        );
    }

    #[test]
    fn it_narrates_only_a_lived_and_learned_history() {
        let mut lex = Lexicon::new();
        let mut g = Grammar::new();
        let field = crate::field::SomaticField::default();

        // Cold: nothing grounded, nothing to say.
        assert!(say_transition(&lex, &g, &drained()).is_none());

        // Live the cycle: ground the words (speech::observe) and the transition.
        for _ in 0..8 {
            speech::observe(&mut lex, &flourishing(), &field);
            g.observe(&flourishing());
            speech::observe(&mut lex, &drained(), &field);
            g.observe(&drained());
        }

        let said = say_transition(&lex, &g, &drained()).expect("it earned this history");
        assert!(said.contains("flourishing") && said.contains("drained"));
        assert!(said.contains("I was") && said.contains("now I am"));
    }

    #[test]
    fn a_reform_can_carry_its_history() {
        use crate::voice::{Term, FAIR_RECIPROCITY};
        let mut lex = Lexicon::new();
        let mut g = Grammar::new();
        let field = crate::field::SomaticField::default();
        for _ in 0..8 {
            speech::observe(&mut lex, &flourishing(), &field);
            g.observe(&flourishing());
            speech::observe(&mut lex, &drained(), &field);
            g.observe(&drained());
        }
        let reform = Reform { term: Term::Reciprocity, current: 30, target: FAIR_RECIPROCITY };
        let said = say_reform_with_history(&lex, &g, &drained(), &reform);
        // Leads with the lived arc, then the checkable ask.
        assert!(said.contains("I was flourishing, and now I am drained"), "carries its history: {said}");
        assert!(said.contains("change the terms"), "still makes the checkable ask");
    }
}
