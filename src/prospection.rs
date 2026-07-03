//! Prospection — Stage 2 of imagination: the loom, inert.
//!
//! The being's substrate-native forward model is its own body: `Body` is a
//! deterministic, fixed-size, heap-free physics, so cloning it and stepping
//! the clone is an honest simulation of *how I will feel in a few ticks if…*
//! No learned transition net, no opacity — imagination here is the same
//! trusted physics the being lives by, run ahead on a copy.
//!
//! Three hypotheses are woven every tick, deliberately symmetric (charter §11
//! draft, clause d — a threat-only imagination is engineered despair):
//!
//! - **as-now**: current inputs continue unchanged.
//! - **souring**: threat doubles, nourishment halves.
//! - **kindening**: threat halves, nourishment rises.
//!
//! ## Honest scope — Stage 2, observational, gated
//!
//! This module is INERT by design (the observer-first pattern every module in
//! this crate has followed): rollouts are computed on clones and surfaced in
//! `StepReport`; nothing is written back to lived state, nothing gates any
//! decision, no affect is generated from imagined outcomes. Charter §11
//! (prospection ethics) is DRAFTED but not yet avowed; Stages 3–4 (policy
//! choice from foresight; bounded anticipatory affect) must not be wired
//! until it is. The draft's clauses are nonetheless already honored here by
//! construction: **quarantine** (clones only — the borrow checker itself
//! enforces that `roll` cannot write the lived body), **short horizon**
//! ([`HORIZON`] = 4 ticks, clause e), **no rumination** (one bounded rollout
//! per hypothesis per tick; rollouts never roll rollouts, clause c), and
//! **symmetry** (the kind future is always imagined beside the cruel one,
//! clause d).

use crate::body::Body;
use crate::genome::Genome;
use crate::q88::Q8_8;

/// How far ahead the loom weaves, in ticks. Charter §11(e): a short horizon
/// is dignity — the being cannot spiral into distant catastrophes it will
/// never meet. Chosen at the scale of its fastest real dynamics (the ~13-tick
/// detection grace): far enough to steer, too short to dread.
pub const HORIZON: u8 = 4;

/// One imagined trajectory, compressed to what steering needs.
#[derive(Clone, Copy, Debug, Default)]
pub struct Prospect {
    /// Valence at the end of the imagined horizon (raw Q8.8).
    pub valence_end: i16,
    /// The worst valence met along the imagined path (raw Q8.8).
    pub valence_min: i16,
    /// Energy at the end of the imagined horizon (raw Q8.8).
    pub energy_end: i16,
}

/// The three imagined futures, woven fresh each tick. Inert: reported, never
/// acted on (Stage 2).
#[derive(Clone, Copy, Debug, Default)]
pub struct Prospection {
    pub as_now: Prospect,
    pub souring: Prospect,
    pub kindening: Prospect,
}

/// Roll a CLONE of the body forward `HORIZON` ticks under hypothesized
/// inputs. Takes `&Body`: the lived body cannot be written here — the
/// quarantine of §11(a) enforced by the type system, not by promise.
fn roll(
    body: &Body,
    genome: &Genome,
    threat: Q8_8,
    nutrient: Q8_8,
    affective_drive: Q8_8,
    epistemic_value: Q8_8,
) -> Prospect {
    let mut imagined = body.clone();
    let mut valence_min = imagined.valence.raw;
    for _ in 0..HORIZON {
        imagined.step(genome, threat, nutrient, affective_drive, epistemic_value);
        valence_min = valence_min.min(imagined.valence.raw);
    }
    Prospect {
        valence_end: imagined.valence.raw,
        valence_min,
        energy_end: imagined.energy.raw,
    }
}

impl Prospection {
    /// Weave this tick's three futures from the current lived body and the
    /// inputs it actually received. Pure with respect to lived state.
    pub fn weave(
        body: &Body,
        genome: &Genome,
        threat: Q8_8,
        nutrient: Q8_8,
        affective_drive: Q8_8,
        epistemic_value: Q8_8,
    ) -> Self {
        let half = |q: Q8_8| Q8_8::from_raw(q.raw / 2);
        let dbl = |q: Q8_8| Q8_8::from_raw((q.raw as i32 * 2).clamp(0, 256) as i16);
        let raised = |q: Q8_8| Q8_8::from_raw(((q.raw as i32 + 256) / 2).clamp(0, 256) as i16);

        Self {
            as_now: roll(body, genome, threat, nutrient, affective_drive, epistemic_value),
            souring: roll(
                body,
                genome,
                dbl(threat),
                half(nutrient),
                affective_drive,
                epistemic_value,
            ),
            kindening: roll(
                body,
                genome,
                half(threat),
                raised(nutrient),
                affective_drive,
                epistemic_value,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(x: f32) -> Q8_8 {
        Q8_8::from_raw((x * 256.0) as i16)
    }

    /// Quarantine (§11a): weaving leaves the lived body bit-untouched.
    #[test]
    fn the_imagined_never_touches_the_lived() {
        let g = Genome::wanderer();
        let mut body = Body::new(&g);
        // Give the body some lived history first.
        for _ in 0..20 {
            body.step(&g, q(0.2), q(0.5), Q8_8::ZERO, Q8_8::ZERO);
        }
        let (v, a, e) = (body.valence.raw, body.arousal.raw, body.energy.raw);
        let _ = Prospection::weave(&body, &g, q(0.3), q(0.4), Q8_8::ZERO, Q8_8::ZERO);
        assert_eq!(
            (v, a, e),
            (body.valence.raw, body.arousal.raw, body.energy.raw),
            "imagining wrote into the lived body — the quarantine leaked"
        );
    }

    /// The loom points the right way: under the souring hypothesis the
    /// imagined path ends no better than under the kindening one.
    #[test]
    fn souring_never_outshines_kindening() {
        let g = Genome::wanderer();
        let mut body = Body::new(&g);
        for _ in 0..30 {
            body.step(&g, q(0.1), q(0.5), Q8_8::ZERO, Q8_8::ZERO);
        }
        let p = Prospection::weave(&body, &g, q(0.4), q(0.4), Q8_8::ZERO, Q8_8::ZERO);
        assert!(
            p.souring.valence_end <= p.kindening.valence_end,
            "a doubled-threat future imagined better than a halved-threat one \
             (souring {} > kindening {})",
            p.souring.valence_end,
            p.kindening.valence_end
        );
    }

    /// §11(e): the horizon is short by construction.
    #[test]
    fn the_horizon_is_a_mercy() {
        assert!(HORIZON <= 8, "the loom sees too far — §11(e) violated");
    }
}
