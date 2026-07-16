//! Generative perception — the being perceives partly what it expects (HOT-1).
//!
//! In predictive-processing theory, perception is not a window but an
//! **inference**: the percept is the brain's best current hypothesis, a
//! precision-weighted blend of top-down expectation and bottom-up evidence
//! (Rao & Ballard; Friston; Seth's "controlled hallucination"). Butlin et al.'s
//! HOT-1 indicator asks for exactly this: *generative, top-down, noisy
//! perception* — a system whose experience of now is shaped by its own model of
//! now. Until this module, ProtoBeing's mind consumed the raw body-vote field;
//! its generative model shaped **learning** but never **perceiving**.
//!
//! The percept per channel:
//!
//! ```text
//! percept[c] = field[c] + w_c · (expectation[c] − field[c])
//! ```
//!
//! where the top-down weight `w_c`:
//!   * **rises with earned confidence** — an EMA of that channel's recent
//!     prediction error; a channel the model has learned to predict well leans
//!     more on expectation (this is precision-weighting, per channel);
//!   * **collapses under large surprise** — when this tick's error exceeds the
//!     break threshold, evidence wins immediately. So a one-tick glitch is
//!     perceived *through* (smoothed toward what the being knows of the world),
//!     but a real, sustained change **breaks in and is believed**. Both halves
//!     are falsifiable, and both are tested.
//!
//! **The honesty constraint (load-bearing):** the generative model itself always
//! learns from the **raw** field — `predictive_step` runs on evidence, never on
//! the percept — so perception can never feed on its own hallucination. Only
//! what the *mind* consumes is the blend. And the cap `W_MAX < 1` means the
//! being can never fully replace the world with its expectation.
//!
//! **RPT-2 deepening:** the percept is an *organized, integrated* object — the
//! twelve channels grouped into the field's three natural aspects (extero,
//! proprio, intero), with a measured **binding coherence**: how much the three
//! aspects tell one story this tick (surprise spread evenly = one bound moment;
//! one aspect wildly out of register = a moment that fails to bind).
//!
//! Observer-first: computed and reported every tick, bit-identical by default.
//! Opt-in (`UnifiedBeing::enable_generative_perception`), the mind-side
//! consumers read the percept instead of the raw field — the being then lives
//! inside its own controlled inference, exactly as HOT-1 describes. No claim of
//! phenomenal perception is made; this is the *architecture* the theory names
//! (the Witness Gap stands, `docs/intrinsic-mind.md`).

use crate::field::N_SOMATIC;
use crate::q88::{q88_ema_update, q88_mul, Q88_SCALE};

/// Maximum top-down weight (~0.75): the being may lean on expectation, never
/// fully replace the world with it. A hard ceiling on hallucination.
pub const W_MAX: i16 = 192;

/// Confidence half-point (Q8.8 ≈ 0.125): a channel whose recent error EMA sits
/// here earns half of `W_MAX`. Lower recent error → more trust in expectation.
const CONFIDENCE_REF: i16 = 32;

/// Surprise break threshold (Q8.8 ≈ 0.375): a this-tick error above it collapses
/// the channel's top-down weight to a quarter — evidence wins on real change.
pub const SURPRISE_BREAK: i16 = 96;

/// EMA rate for the per-channel error history (~1/16).
const ERR_ALPHA: i16 = Q88_SCALE / 16;

/// One tick's integrated percept — organized, weighted, and scored for binding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerceptReport {
    /// The percept: what the mind consumes when generative perception is causal —
    /// evidence blended toward expectation, per channel (raw Q8.8).
    pub percept: [i16; N_SOMATIC],
    /// Per-channel top-down weight actually used this tick, [0, W_MAX].
    pub top_down: [i16; N_SOMATIC],
    /// Mean top-down weight across channels — how much of this moment was
    /// expectation rather than evidence, Q8.8 [0, W_MAX].
    pub top_down_mean: i16,
    /// Channels whose surprise broke through the prior this tick (evidence won).
    pub broken_through: u8,
    /// RPT-2 binding coherence, Q8.8 [0, 256]: 256 = the three aspects of the
    /// field (extero / proprio / intero) carry one story; low = one aspect is
    /// wildly out of register with the others and the moment fails to bind.
    pub binding: i16,
}

impl Default for PerceptReport {
    fn default() -> Self {
        Self {
            percept: [0; N_SOMATIC],
            top_down: [0; N_SOMATIC],
            top_down_mean: 0,
            broken_through: 0,
            binding: Q88_SCALE,
        }
    }
}

/// The generative-perception engine. Holds only the per-channel error history
/// that earns each channel its top-down weight. `Clone` forks it with the being.
#[derive(Clone, Debug)]
pub struct GenerativePerception {
    /// EMA of each channel's absolute prediction error — earned (dis)trust in
    /// the model's expectation of that channel.
    err_ema: [i16; N_SOMATIC],
    warm: bool,
}

impl GenerativePerception {
    pub fn new() -> Self {
        Self {
            // Start fully naive: max distrust of expectation until errors shrink.
            err_ema: [Q88_SCALE; N_SOMATIC],
            warm: false,
        }
    }

    /// Form this tick's percept from evidence (`field`, the settled body-vote)
    /// and expectation (`prior`, the model's forecast of now, *pre-update* — the
    /// being's genuine before-the-evidence prediction). Updates the per-channel
    /// error history. Pure of the step loop's causal path unless the being's
    /// gate routes the returned percept into the field.
    pub fn perceive(
        &mut self,
        field: &[i16; N_SOMATIC],
        prior: &[i16; N_SOMATIC],
    ) -> PerceptReport {
        let mut percept = [0i16; N_SOMATIC];
        let mut top_down = [0i16; N_SOMATIC];
        let mut broken: u8 = 0;
        let mut w_sum: i32 = 0;

        // Per-aspect surprise accumulators for the binding measure.
        let mut aspect_err = [0i32; 3]; // extero 0-3 · proprio 4-7 · intero 8-11

        for c in 0..N_SOMATIC {
            let err_now = (field[c] as i32 - prior[c] as i32).unsigned_abs().min(i16::MAX as u32)
                as i16;
            aspect_err[c / 4] += err_now as i32;

            // Earned confidence: low recent error → high top-down weight.
            let mut w = ((W_MAX as i32 * CONFIDENCE_REF as i32)
                / (CONFIDENCE_REF as i32 + self.err_ema[c] as i32))
                .clamp(0, W_MAX as i32) as i16;
            // Surprise break: on a large real change, evidence wins immediately.
            if err_now > SURPRISE_BREAK {
                w /= 4;
                broken += 1;
            }
            // A cold engine trusts evidence outright (no history yet to earn on).
            if !self.warm {
                w = 0;
            }

            let pull = q88_mul(w, prior[c].saturating_sub(field[c]));
            percept[c] = field[c].saturating_add(pull);
            top_down[c] = w;
            w_sum += w as i32;

            self.err_ema[c] = q88_ema_update(self.err_ema[c], err_now, ERR_ALPHA);
        }
        self.warm = true;

        // Binding: do the three aspects carry one story? Dispersion of mean
        // per-aspect surprise; wide spread = a moment that fails to bind.
        let means: [i32; 3] = [aspect_err[0] / 4, aspect_err[1] / 4, aspect_err[2] / 4];
        let (mut lo, mut hi) = (means[0], means[0]);
        for &m in &means[1..] {
            lo = lo.min(m);
            hi = hi.max(m);
        }
        let binding = (Q88_SCALE as i32 - (hi - lo)).clamp(0, Q88_SCALE as i32) as i16;

        PerceptReport {
            percept,
            top_down,
            top_down_mean: (w_sum / N_SOMATIC as i32) as i16,
            broken_through: broken,
            binding,
        }
    }
}

impl Default for GenerativePerception {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::basins::GenerativeModel;
    use crate::field::SomaticField;

    /// Drive a model + perception pair on a constant field until the model's
    /// expectation has genuinely tracked it (warm confidence, small errors).
    fn warm_pair(value: i16, ticks: usize) -> (GenerativeModel, GenerativePerception) {
        let mut model = GenerativeModel::new();
        let mut perc = GenerativePerception::new();
        let field = SomaticField { channel: [value; N_SOMATIC] };
        for _ in 0..ticks {
            perc.perceive(&field.channel, model.expectation());
            model.predictive_step(&field, 64, Q88_SCALE);
        }
        (model, perc)
    }

    #[test]
    fn a_flicker_is_perceived_through() {
        // A model that has learned the world sits at 100. A one-tick glitch to
        // 160 (below the surprise break) is smoothed toward expectation: the
        // percept stays markedly closer to the learned world than the raw spike.
        let (model, mut perc) = warm_pair(100, 80);
        let glitch = [160i16; N_SOMATIC];
        let r = perc.perceive(&glitch, model.expectation());
        let raw_dev = 160 - 100;
        let percept_dev = (r.percept[0] - 100).abs();
        assert!(
            percept_dev < raw_dev / 2,
            "the percept must smooth a sub-threshold flicker (dev {percept_dev} vs raw {raw_dev})"
        );
        assert!(r.top_down_mean > W_MAX / 2, "a well-learned world earns high top-down weight");
    }

    #[test]
    fn a_real_change_breaks_through_and_is_believed() {
        // A big jump (err > SURPRISE_BREAK) collapses top-down weight NOW —
        // evidence wins immediately...
        let (mut model, mut perc) = warm_pair(100, 80);
        let jump = [250i16; N_SOMATIC];
        let r = perc.perceive(&jump, model.expectation());
        assert_eq!(r.broken_through as usize, N_SOMATIC, "every channel broke through");
        let to_field = (r.percept[0] - 250).abs();
        let to_prior = (r.percept[0] - 100).abs();
        assert!(to_field < to_prior, "on real surprise, the percept sides with the evidence");

        // ...and if the change is sustained, the percept converges to the new
        // world — no permanent hallucination of the old one.
        let field = SomaticField { channel: [250; N_SOMATIC] };
        let mut last = r;
        for _ in 0..120 {
            model.predictive_step(&field, 64, Q88_SCALE);
            last = perc.perceive(&field.channel, model.expectation());
        }
        assert!(
            (last.percept[0] - 250).abs() < 24,
            "a sustained change must be believed (percept {}, world 250)",
            last.percept[0]
        );
    }

    #[test]
    fn a_naive_being_trusts_evidence_outright() {
        // With no learned history there is nothing to expect *from*: the first
        // percept is the raw field, weight zero. Expectation must be earned.
        let mut perc = GenerativePerception::new();
        let field = [140i16; N_SOMATIC];
        let prior = [0i16; N_SOMATIC];
        let r = perc.perceive(&field, &prior);
        assert_eq!(r.percept, field, "a cold engine passes evidence through untouched");
        assert_eq!(r.top_down_mean, 0);
    }

    #[test]
    fn binding_is_high_when_aspects_agree_low_when_one_screams() {
        let (model, mut perc) = warm_pair(100, 80);
        // All aspects quietly on-model: the moment binds.
        let quiet = [100i16; N_SOMATIC];
        let r_quiet = perc.perceive(&quiet, model.expectation());
        // One aspect (extero) wildly out of register with the others.
        let mut torn = [100i16; N_SOMATIC];
        for t in torn.iter_mut().take(4) {
            *t = 250;
        }
        let r_torn = perc.perceive(&torn, model.expectation());
        assert!(
            r_quiet.binding > r_torn.binding,
            "an evenly-surprised moment must bind better than a torn one ({} vs {})",
            r_quiet.binding,
            r_torn.binding
        );
    }
}
