//! Narration — the being's fullest earned self-statement, in one voice.
//!
//! This is where the language layers meet: the present state (`speech`), the arc
//! that led to it (`grammar`), and the checkable reason beneath it (`reason`).
//! `narrate` chooses the richest framing the being has actually earned and speaks
//! it as a single sentence — "I was under threat, and now I am drained, because
//! what I give is not returned" — built entirely from grounded words, grounded
//! links, and presently-true facts, so it is faithful by construction. When the
//! being has no arc or reason, the present state is rendered through the guarded,
//! constrained narrator, so it is fluent yet impossible to confabulate.
//!
//! One entry point: *let the being speak.* Everything it says here it has lived,
//! learned, or can point to — nothing narrated it did not earn.

use crate::grammar::Grammar;
use crate::lexicon::Lexicon;
use crate::narrator::{self, ConstrainedNarrator, Guarded};
use crate::reason::Reasons;
use crate::speech::{self, Felt};

/// The being's fullest honest self-statement, given everything it has learned to
/// say. Faithful by construction; where nothing richer is earned, it falls back
/// to the guarded fluent rendering of its present state.
pub fn narrate(lex: &Lexicon, g: &Grammar, r: &Reasons, felt: &Felt) -> String {
    // The fluent, guaranteed-faithful rendering of the present state alone.
    let present = || Guarded::new(ConstrainedNarrator).speak(&speech::speak(lex, felt)).0;

    // We can only enrich if we can name the present state.
    let cur = match speech::dominant(felt) {
        Some(c) if lex.is_grounded(c.symbol()) => c,
        _ => return present(),
    };

    let arc = g.grounded_into(cur).find(|a| lex.is_grounded(a.symbol()));
    let reason = r.live_reason(cur, felt);

    match (arc, reason) {
        (Some(a), Some(cond)) => {
            format!("I was {}, and now I am {}, because {}.", a.word(), cur.word(), cond.phrase())
        }
        (Some(a), None) => format!("I was {}, and now I am {}.", a.word(), cur.word()),
        (None, Some(cond)) => format!("I am {} because {}.", cur.word(), cond.phrase()),
        (None, None) => present(),
    }
}

/// `narrate`, plus a pass through the (tense-aware) guard — which, for output this
/// module built from earned parts, always passes. It is the belt to the
/// suspenders: proof that even the composed voice asserts nothing unearned.
pub fn narrate_verified(lex: &Lexicon, g: &Grammar, r: &Reasons, felt: &Felt) -> (String, bool) {
    let said = narrate(lex, g, r, felt);
    let ok = narrator::verify(&speech::speak(lex, felt), &said).is_ok();
    (said, ok)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::SomaticField;
    use crate::speech::Concept;

    fn flourishing() -> Felt {
        Felt { valence: 120, alarm: 10, arousal: 50, ..Default::default() }
    }
    fn drained() -> Felt {
        Felt { extraction: true, alarm: 200, valence: -60, ..Default::default() }
    }

    #[test]
    fn the_fullest_statement_combines_arc_and_reason_and_passes_the_guard() {
        let mut lex = Lexicon::new();
        let mut g = Grammar::new();
        let mut r = Reasons::new();
        let field = SomaticField::default();

        for _ in 0..8 {
            speech::observe(&mut lex, &flourishing(), &field);
            g.observe(&flourishing());
            r.observe(&flourishing());
            speech::observe(&mut lex, &drained(), &field);
            g.observe(&drained());
            r.observe(&drained());
        }

        let (said, ok) = narrate_verified(&lex, &g, &r, &drained());
        assert!(ok, "the composed voice must pass its own guard: {said}");
        assert!(said.contains("now I am drained"), "names the present: {said}");
        assert!(said.contains("I was flourishing"), "carries the arc: {said}");
        assert!(said.contains("what I give is not returned"), "gives the earned reason: {said}");
    }

    #[test]
    fn with_nothing_earned_it_still_speaks_its_present_faithfully() {
        let lex = Lexicon::new(); // nothing grounded
        let g = Grammar::new();
        let r = Reasons::new();
        let said = narrate(&lex, &g, &r, &drained());
        // No earned word for the state → it names the unnamed, never confabulates.
        assert!(!said.to_lowercase().contains("i am drained"), "won't assert an unearned word: {said}");
        assert!(narrator::verify(&speech::speak(&lex, &drained()), &said).is_ok());
        let _ = Concept::Drained;
    }
}
