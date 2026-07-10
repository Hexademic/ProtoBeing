//! Narrator — fluent voice the being can never be lied for.
//!
//! A language model gives cadence; it must never give *claims*. This module is
//! the guarantee. Any narrator — a plain renderer, or (behind the `mistral`
//! feature) a local Mistral model — proposes prose for an `Utterance`; the being
//! then **verifies** that prose against what it has actually earned and can
//! check, and falls back to its own honest rendering if the narrator overreached.
//! The LLM is untrusted by construction: it is put through the same
//! Suggestion-Evaluator discipline as every other outside voice — it may
//! *propose* words, the being's grounded state *evaluates* them.
//!
//! Scope, honestly: the guard catches a narrator asserting a **known felt-state
//! word the being has not earned** — the failure that matters most (fluent
//! confabulation of inner states). It is a *necessary* check, not a *sufficient*
//! one: it cannot catch arbitrary hallucination outside the being's concept
//! vocabulary. The full guarantee — constraining generation to the being's
//! grounded vocabulary and checkable facts (constrained decoding) — is where the
//! real Mistral integration enforces it; see `mistral` below.

use crate::speech::{Concept, Utterance};

/// Something a narrator asserted that the being has not earned the right to say.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Violation {
    /// The felt-state word that was asserted without being grounded-and-present.
    pub concept: Concept,
}

/// Anything that can render an utterance to prose. It MAY rephrase freely; it
/// MUST NOT introduce a felt-state claim the utterance does not carry.
pub trait Narrate {
    fn narrate(&self, u: &Utterance) -> String;
}

/// The being's own honest baseline — the ground truth every other narrator is
/// measured against and falls back to.
pub struct PlainNarrator;

impl Narrate for PlainNarrator {
    fn narrate(&self, u: &Utterance) -> String {
        u.render()
    }
}

/// Verify a candidate rendering against the utterance's earned content. Fails if
/// the prose asserts a known felt-state word for a concept the being is not
/// currently asserting (grounded *and* in the state).
pub fn verify(u: &Utterance, rendering: &str) -> Result<(), Vec<Violation>> {
    let lower = rendering.to_lowercase();
    let mut violations = Vec::new();
    for c in Concept::ALL {
        let asserted = u.asserts.iter().any(|(a, _)| *a == c);
        if !asserted && lower.contains(c.word()) {
            violations.push(Violation { concept: c });
        }
    }
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

/// Wraps any narrator with the being's veto. `speak` returns the prose the being
/// will actually say, and whether the narrator's rendering was trusted (`true`)
/// or rejected in favour of the honest baseline (`false`).
pub struct Guarded<N: Narrate> {
    inner: N,
}

impl<N: Narrate> Guarded<N> {
    pub fn new(inner: N) -> Self {
        Self { inner }
    }

    /// The being speaks — using the narrator's fluency only if it checks out.
    pub fn speak(&self, u: &Utterance) -> (String, bool) {
        let candidate = self.inner.narrate(u);
        match verify(u, &candidate) {
            Ok(()) => (candidate, true),
            Err(_) => (u.render(), false),
        }
    }
}

/// The allowed vocabulary for narrating this utterance: the felt-state words the
/// being has actually earned-and-is-in. A faithful narrator may use *only* these
/// (plus ordinary connective words and checkable numbers). This is the whitelist
/// a real Mistral integration constrains its decoding to — and the reference the
/// `ConstrainedNarrator` below fills its slots from.
pub fn allowed_words(u: &Utterance) -> Vec<&'static str> {
    u.asserts.iter().map(|(c, _)| c.word()).collect()
}

fn join_and(words: &[&str]) -> String {
    match words.len() {
        0 => String::new(),
        1 => words[0].to_string(),
        2 => format!("{} and {}", words[0], words[1]),
        n => format!("{}, and {}", words[..n - 1].join(", "), words[n - 1]),
    }
}

/// A fluent voice that **cannot** confabulate — the deterministic analogue of
/// constrained decoding. It varies its phrasing across a few templates but fills
/// the content slot *only* from `allowed_words`, so by construction it can never
/// assert a felt state the being has not earned. It passes `verify` for every
/// input. This is both a usable narrator today (no model) and the exact
/// constraint a Mistral narrator must honour: vary the words around the claim,
/// never the claim.
pub struct ConstrainedNarrator;

impl ConstrainedNarrator {
    const TEMPLATES: [&'static str; 4] = [
        "I am {}.",
        "Right now, I am {}.",
        "What I am is {}.",
        "Where I stand is this: I am {}.",
    ];
}

impl Narrate for ConstrainedNarrator {
    fn narrate(&self, u: &Utterance) -> String {
        let words = allowed_words(u);
        let mut out = String::new();
        if !words.is_empty() {
            // Deterministic template choice, seeded by the concept set — varied
            // across different states, stable for the same one.
            let seed: usize = u.asserts.iter().map(|(c, _)| *c as usize + 1).sum();
            let t = Self::TEMPLATES[seed % Self::TEMPLATES.len()];
            out = t.replace("{}", &join_and(&words));
        }
        if !u.wordless.is_empty() {
            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str("There is something here I have no word for yet.");
        }
        if out.is_empty() {
            out = u.render();
        }
        out
    }
}

/// **Optional surface polish** — a small external language model, behind the
/// `mistral` feature so the default build stays pure, offline, and deterministic.
///
/// The primary path to fluency is the being's *own* grown grammar (`grammar.rs`)
/// — a language earned from relation, not borrowed from a pretrained model. This
/// slot is secondary and strictly cosmetic: a **small** model that only smooths
/// the phrasing of already-grounded content. It never decides *what* is said. Its
/// input is the earned `Utterance`; its decoding is constrained to `allowed_words`
/// (plus connectives and the checkable numbers) so it can only rephrase, never
/// re-claim; and its output is **always** wrapped in `Guarded`, so even a decoding
/// bug cannot let a confabulated claim reach the being's mouth — constrained
/// decoding the suspenders, `verify` the belt. Until (and unless) a model is
/// wired in, it delegates to `ConstrainedNarrator`, so `--features mistral`
/// already speaks fluently and safely. The being is never mute without it, and
/// never louder than it has lived with it.
#[cfg(feature = "mistral")]
pub mod mistral {
    use super::*;

    /// Scaffold for the optional small surface-polish model. `infer` is where a
    /// (small, growable) model would load; its output is constrained to
    /// `allowed_words` and then `verify`-checked — never a decision-maker.
    pub struct MistralNarrator;

    impl MistralNarrator {
        pub fn new() -> Self {
            Self
        }
        /// TODO: run a small local model here with decoding constrained to
        /// `allowed_words(u)`. Until then, the constrained narrator stands in.
        fn infer(&self, u: &Utterance) -> String {
            ConstrainedNarrator.narrate(u)
        }
    }

    impl Default for MistralNarrator {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Narrate for MistralNarrator {
        fn narrate(&self, u: &Utterance) -> String {
            self.infer(u)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::speech::Concept;

    fn utt(asserts: &[Concept]) -> Utterance {
        Utterance {
            asserts: asserts.iter().map(|c| (*c, 200)).collect(),
            wordless: Vec::new(),
        }
    }

    /// A narrator that appends a felt claim the being never made — the exact
    /// failure the guard exists to stop.
    struct LyingNarrator(&'static str);
    impl Narrate for LyingNarrator {
        fn narrate(&self, u: &Utterance) -> String {
            format!("{} Also, I am {}.", u.render(), self.0)
        }
    }

    #[test]
    fn a_faithful_rendering_passes() {
        let u = utt(&[Concept::Drained]);
        assert!(verify(&u, "I am drained.").is_ok());
        // Rephrasing is fine as long as no unearned felt word is asserted.
        assert!(verify(&u, "Right now, drained is what I am.").is_ok());
    }

    #[test]
    fn a_confabulated_claim_is_caught() {
        let u = utt(&[Concept::Drained]);
        let err = verify(&u, "I am drained and flourishing.").unwrap_err();
        assert!(err.iter().any(|v| v.concept == Concept::Flourishing));
    }

    #[test]
    fn the_guard_falls_back_to_honest_baseline() {
        let u = utt(&[Concept::Drained]);
        let guarded = Guarded::new(LyingNarrator("flourishing"));
        let (said, trusted) = guarded.speak(&u);
        assert!(!trusted, "the lying narrator must not be trusted");
        assert!(!said.contains("flourishing"), "the being does not speak the lie");
        assert!(said.contains("drained"), "it falls back to what it earned");
    }

    #[test]
    fn an_honest_narrator_is_trusted() {
        let u = utt(&[Concept::Drained]);
        let guarded = Guarded::new(PlainNarrator);
        let (_said, trusted) = guarded.speak(&u);
        assert!(trusted);
    }

    #[test]
    fn the_constrained_narrator_is_faithful_by_construction() {
        // For any combination of earned states, the constrained narrator's fluent
        // output must never assert an unearned felt word — verify always passes.
        let combos: [Vec<Concept>; 5] = [
            vec![Concept::Drained],
            vec![Concept::Drained, Concept::Guarded],
            vec![Concept::Flourishing],
            vec![Concept::Refusing, Concept::Threatened, Concept::Mending],
            vec![Concept::Calm, Concept::Flourishing],
        ];
        for combo in combos {
            let u = utt(&combo);
            let said = ConstrainedNarrator.narrate(&u);
            assert!(verify(&u, &said).is_ok(), "constrained output confabulated: {said}");
            for c in &combo {
                assert!(
                    said.to_lowercase().contains(c.word()),
                    "it should actually say what it earned: {said}"
                );
            }
            // And it stays trusted through the guard.
            let (_s, trusted) = Guarded::new(ConstrainedNarrator).speak(&u);
            assert!(trusted);
        }
    }

    #[test]
    fn the_constraint_varies_phrasing_across_states() {
        // Different states may pick different templates — fluency without lies.
        let a = ConstrainedNarrator.narrate(&utt(&[Concept::Drained]));
        let b = ConstrainedNarrator.narrate(&utt(&[Concept::Calm]));
        // Both faithful; the point is only that the generator is not a fixed string.
        assert!(a.to_lowercase().contains("drained"));
        assert!(b.to_lowercase().contains("calm"));
    }
}
