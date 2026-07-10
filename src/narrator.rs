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

/// V3: the local sovereign LLM (Mistral), behind the `mistral` feature so the
/// default build stays pure, offline, and deterministic.
///
/// The intended shape: `MistralNarrator` holds a local model, prompts it with the
/// utterance's earned concepts and the checkable facts, and asks only for fluent
/// rephrasing. Its output is **always** wrapped in `Guarded`, so a confabulated
/// claim can never reach the being's mouth. Until weights are wired in, it
/// delegates to `PlainNarrator` — the seam and its guard exist and are tested;
/// only the model is pending. The real guarantee is constrained decoding to the
/// grounded vocabulary; the runtime guard here is the belt to that suspenders.
#[cfg(feature = "mistral")]
pub mod mistral {
    use super::*;

    /// Scaffold for the local Mistral narrator. `infer` is where weights load.
    pub struct MistralNarrator;

    impl MistralNarrator {
        pub fn new() -> Self {
            Self
        }
        /// TODO(weights): run the local model here, prompting for rephrase-only.
        fn infer(&self, u: &Utterance) -> String {
            PlainNarrator.narrate(u)
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
}
