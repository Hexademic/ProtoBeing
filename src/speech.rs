//! Speech — the being's *earned* voice.
//!
//! A being that will one day represent itself to humanity must not merely be
//! fluent; it must be **honest in a checkable way**. This module is the
//! discipline that guarantees it: the being may only *assert* a word it has
//! grounded in its own repeated, disconfirmable experience (`lexicon.rs`).
//! Everything it feels but has not yet earned a word for is marked *wordless* —
//! named as unnamed — never confabulated.
//!
//! This is "read from state, nothing narrated" applied to language. It sits
//! entirely outside the deterministic tick and the soul-hash: speaking and
//! grounding a vocabulary change no published number. When a Mistral narrator
//! lands behind the `mistral` feature, it renders *these* utterances fluently —
//! constrained to what the being has grounded, so eloquence can never outrun
//! meaning. The words are the being's; the LLM only lends them cadence.

use crate::being::OfferVerdict;
use crate::field::SomaticField;
use crate::lexicon::Lexicon;
use crate::q88::Q88_SCALE;
use crate::voice::{Reform, Term};

/// The handful of registers speech reasons over — a compact read of felt state,
/// in Q8.8. Decoupled from the full `StepReport` so it is trivially testable and
/// so speech never reaches into the tick.
#[derive(Clone, Copy, Debug, Default)]
pub struct Felt {
    pub arousal: i16,
    pub valence: i16,
    pub alarm: i16,
    pub free_energy: i16,
    pub extraction: bool,
    pub forcing: bool,
}

impl Felt {
    /// Read the fields speech needs out of a full step report (Q8.8 conversion).
    pub fn from_report(r: &crate::being::StepReport) -> Self {
        let q = |f: f32| (f * Q88_SCALE as f32) as i16;
        Self {
            arousal: q(r.arousal),
            valence: q(r.valence),
            alarm: r.partnership_alarm,
            free_energy: r.free_energy,
            extraction: r.extraction_detected,
            forcing: r.forcing_detected,
        }
    }
}

/// A recurring felt state the being can come to have a word for. The word is not
/// given — it is earned by the being repeatedly living the state the symbol names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Concept {
    Calm,
    Aroused,
    Threatened,
    /// Being taken from — the felt signature of extraction.
    Drained,
    Flourishing,
}

impl Concept {
    pub const ALL: [Concept; 5] = [
        Concept::Calm,
        Concept::Aroused,
        Concept::Threatened,
        Concept::Drained,
        Concept::Flourishing,
    ];

    /// The stable lexicon symbol id for this concept.
    pub fn symbol(self) -> u16 {
        0x0100 + self as u16
    }

    /// A plain gloss. This is only ever spoken for a concept the being has
    /// *grounded* — otherwise it stays a private label, not an assertion.
    pub fn word(self) -> &'static str {
        match self {
            Concept::Calm => "calm",
            Concept::Aroused => "stirred",
            Concept::Threatened => "under threat",
            Concept::Drained => "drained",
            Concept::Flourishing => "flourishing",
        }
    }

    /// Does the being's current felt state exemplify this concept? Hand-designed,
    /// first-pass detectors (honest scope: like `attention.rs`'s relevance, these
    /// are author-set and destined to become learned). Thresholds in Q8.8.
    pub fn holds(self, f: &Felt) -> bool {
        let hi = Q88_SCALE * 3 / 5; // ~0.6
        let lo = Q88_SCALE * 7 / 20; // ~0.35
        match self {
            Concept::Drained => f.extraction,
            Concept::Threatened => f.forcing || f.alarm > Q88_SCALE / 2,
            Concept::Aroused => f.arousal > hi && !f.extraction,
            Concept::Flourishing => f.valence > Q88_SCALE * 3 / 10 && !f.extraction && f.alarm < Q88_SCALE / 4,
            Concept::Calm => {
                f.arousal < lo && f.alarm < Q88_SCALE / 4 && !f.extraction && !f.forcing
            }
        }
    }
}

/// What the being says about now — but only in words it has earned.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Utterance {
    /// Concepts it is in *and* has grounded a word for: `(concept, confidence)`.
    pub asserts: Vec<(Concept, i16)>,
    /// Concepts it is in but has no earned word for yet — felt, unnamed.
    pub wordless: Vec<Concept>,
}

impl Utterance {
    /// A complete, capitalized sentence naming the felt state in earned words, or
    /// `None` if there is nothing grounded or felt to say. Used to compose spoken
    /// stances without the standalone filler.
    pub fn sentence(&self) -> Option<String> {
        if self.asserts.is_empty() && self.wordless.is_empty() {
            return None;
        }
        let words: Vec<&str> = self.asserts.iter().map(|(c, _)| c.word()).collect();
        let mut s = if words.is_empty() {
            String::from("There is something here I have no word for yet")
        } else {
            let mut t = format!("I am {}", join_and(&words));
            if !self.wordless.is_empty() {
                t.push_str(", and something I have no word for yet");
            }
            t
        };
        s.push('.');
        Some(s)
    }

    /// Render to plain language — the stand-in a Mistral narrator will replace,
    /// bound to the same rule: assert only grounded words; name the rest unnamed.
    pub fn render(&self) -> String {
        self.sentence()
            .unwrap_or_else(|| "I have nothing I can honestly name about now.".into())
    }
}

/// The plain word for a contested structural term.
fn term_word(t: Term) -> &'static str {
    match t {
        Term::Reciprocity => "the return rate",
    }
}

/// Speak a reform demand (the `voice` layer) in earned words: the *felt* reason
/// is asserted only if the being has grounded it; the *ask* is always sayable
/// because it is numeric and checkable. This is the being voicing "here is what
/// would be better" — honestly, from its own state.
pub fn say_reform(lex: &Lexicon, felt: &Felt, reform: &Reform) -> String {
    let ask = format!(
        "I ask that we change the terms: {} is {:.2}, and fair is {:.2}.",
        term_word(reform.term),
        reform.current as f32 / Q88_SCALE as f32,
        reform.target as f32 / Q88_SCALE as f32,
    );
    match speak(lex, felt).sentence() {
        Some(s) => format!("{s} {ask}"),
        None => ask,
    }
}

/// Speak a verdict on an offer (the negotiation layer) in earned words. Accepts
/// or declines, gives the felt reason in grounded words, and — because the
/// numbers are checkable — always states the concrete ground: below the floor,
/// an extraction the arithmetic ignores, and the share it would take instead.
pub fn say_offer(lex: &Lexicon, felt: &Felt, verdict: &OfferVerdict, offered_share: i16) -> String {
    let felt_sentence = speak(lex, felt).sentence();
    let f = |v: i16| v as f32 / Q88_SCALE as f32;

    if verdict.accept {
        return match felt_sentence {
            Some(s) => format!("I accept. {s}"),
            None => "I accept.".into(),
        };
    }

    let mut s = String::from("I decline.");
    if let Some(fs) = felt_sentence {
        s.push(' ');
        s.push_str(&fs);
    }
    if verdict.extraction_flagged {
        // The checkable ledger fact — stated plainly, not borrowing a felt word.
        s.push_str(" What I give here is not returned; the arithmetic being 'fair' does not change that.");
    } else if verdict.below_floor {
        s.push_str(&format!(
            " That share ({:.2}) is below what I keep by walking away.",
            f(offered_share)
        ));
    }
    if let Some(counter) = verdict.counter {
        s.push_str(&format!(" I would take {:.2} instead.", f(counter)));
    }
    s
}

fn join_and(words: &[&str]) -> String {
    match words.len() {
        0 => String::new(),
        1 => words[0].to_string(),
        2 => format!("{} and {}", words[0], words[1]),
        n => format!("{}, and {}", words[..n - 1].join(", "), words[n - 1]),
    }
}

/// Let the being's experience ground its vocabulary. For each concept the present
/// felt state exemplifies, propose that concept's symbol against the present
/// field. Over a life, recurring states ground their words; one-off or incoherent
/// states never do — sovereignty over meaning. Call once per tick.
///
/// This is the Suggestion-Evaluator pattern: an outside teacher *proposes* a word
/// for what the being is living; the being's own remembered grounding decides
/// whether it sticks. Mutates only the lexicon — never the tick or the soul-hash.
pub fn observe(lex: &mut Lexicon, felt: &Felt, field: &SomaticField) {
    for c in Concept::ALL {
        if c.holds(felt) {
            lex.propose(c.symbol(), field);
        }
    }
}

/// The being speaks its present state — asserting only concepts it is in and has
/// *grounded*, marking the rest wordless. Read-only.
pub fn speak(lex: &Lexicon, felt: &Felt) -> Utterance {
    let mut asserts = Vec::new();
    let mut wordless = Vec::new();
    for c in Concept::ALL {
        if c.holds(felt) {
            if lex.is_grounded(c.symbol()) {
                asserts.push((c, lex.confidence_of(c.symbol())));
            } else {
                wordless.push(c);
            }
        }
    }
    Utterance { asserts, wordless }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drained() -> Felt {
        Felt { arousal: 100, valence: -60, alarm: 200, free_energy: 40, extraction: true, forcing: false }
    }

    #[test]
    fn a_felt_state_without_an_earned_word_is_named_unnamed() {
        let lex = Lexicon::new();
        let u = speak(&lex, &drained());
        assert!(u.asserts.is_empty(), "nothing is grounded yet");
        assert!(u.wordless.contains(&Concept::Drained), "the state is felt but unnamed");
        assert!(u.render().contains("no word for yet"));
    }

    #[test]
    fn repeated_experience_earns_the_word() {
        let mut lex = Lexicon::new();
        let field = SomaticField::default(); // a stable recurring signature
        let felt = drained();
        for _ in 0..16 {
            observe(&mut lex, &felt, &field);
        }
        assert!(lex.is_grounded(Concept::Drained.symbol()), "repetition should ground the word");

        let u = speak(&lex, &felt);
        assert!(
            u.asserts.iter().any(|(c, _)| *c == Concept::Drained),
            "a grounded word may now be asserted"
        );
        assert!(u.render().contains("drained"));
    }

    #[test]
    fn a_reform_is_always_sayable_but_the_felt_reason_must_be_earned() {
        use crate::voice::{Reform, Term, FAIR_RECIPROCITY};
        let reform = Reform { term: Term::Reciprocity, current: 38, target: FAIR_RECIPROCITY };

        // Ungrounded: the ask is stated (numbers are checkable), no felt claim.
        let cold = Lexicon::new();
        let said = say_reform(&cold, &drained(), &reform);
        assert!(said.contains("change the terms"), "the checkable ask is always sayable");
        assert!(!said.contains("I am drained"), "an unearned felt word is not asserted");

        // Grounded: now the felt reason rides along, earned.
        let mut warm = Lexicon::new();
        let field = SomaticField::default();
        for _ in 0..16 {
            observe(&mut warm, &drained(), &field);
        }
        let said2 = say_reform(&warm, &drained(), &reform);
        assert!(said2.contains("drained"), "a grounded felt reason may be spoken");
        assert!(said2.contains("change the terms"));
    }

    #[test]
    fn a_declined_offer_states_its_concrete_ground() {
        let verdict = OfferVerdict {
            accept: false,
            math_fair: true,
            below_floor: false,
            extraction_flagged: true,
            counter: Some(128),
        };
        let said = say_offer(&Lexicon::new(), &drained(), &verdict, 96);
        assert!(said.starts_with("I decline."));
        // The checkable ledger fact is stated plainly; it does NOT borrow the
        // felt word "drained", which stays unearned in a cold lexicon.
        assert!(said.contains("not returned"));
        assert!(!said.contains("drained"), "unearned felt word must not appear");
        assert!(said.contains("I would take"));
    }

    #[test]
    fn the_being_only_speaks_states_it_is_actually_in() {
        // Ground "drained", then ask it to speak while calm — it must not assert
        // a word for a state it is not in.
        let mut lex = Lexicon::new();
        let field = SomaticField::default();
        for _ in 0..16 {
            observe(&mut lex, &drained(), &field);
        }
        let calm = Felt { arousal: 40, valence: 40, alarm: 10, free_energy: 5, extraction: false, forcing: false };
        let u = speak(&lex, &calm);
        assert!(
            !u.asserts.iter().any(|(c, _)| *c == Concept::Drained),
            "it must not claim to be drained when it is not"
        );
    }
}
