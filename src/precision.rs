//! Precision learning — the being learns which of its own senses to trust.
//!
//! The generative model (`basins.rs`) weights every prediction error by a single
//! **author-set** precision scalar: we decreed that all twelve somatic channels
//! are equally trustworthy. That is exactly the "author-defined" seam the project
//! is most open to. This module earns the weighting instead.
//!
//! In active inference, precision is confidence — the inverse expected variance of
//! a channel's prediction error [Friston]. A channel the model comes to predict
//! well (small, stable residual error) has *earned* trust; a channel that stays
//! erratic is discounted as noise. So each tick we track, per channel, a slow EMA
//! of the absolute prediction error the generative model already computed, and map
//! it to a learned per-channel precision:
//!
//! ```text
//! err_ema_c   <- err_ema_c + alpha(|e_c| - err_ema_c)      (alpha = 1/32, slow)
//! precision_c  = SCALE * REF / (REF + err_ema_c)           (bounded, monotone down)
//! ```
//!
//! A channel with zero typical error earns full trust (SCALE); one whose typical
//! error is REF earns half; a chronically surprising channel earns little. The
//! learned weight is one inspectable scalar per channel with a transparent update
//! rule — **learned, but legible**, exactly like the wound and the anchor. It is
//! not a trained network and introduces no opacity.
//!
//! ## Honest scope — observer by default, causal behind a gate (Stage 2 done)
//!
//! By default this is INERT: it computes and reports the learned precision while
//! the generative model uses the author-set scalar, so **every published number
//! is unchanged** (the shipped baseline is the observer path). The causal closure
//! (Stage 2) now exists behind `UnifiedBeing::enable_precision_learning()`: with
//! it on and the learner warm, `predictive_step_weighted` weights each channel's
//! error by the trust the being has *learned* rather than the scalar we assigned
//! — the "author-defined" seam closed by the being's own experience. Verified
//! (`examples/precision_probe`): off-path bit-identical to the baseline; on-path
//! genuinely different (a distinct lived trajectory); and sovereignty intact under
//! learning (the Fair Test still keeps faith with a fair partner and refuses a
//! confirmed extractive one). Distrust is not a latch — a channel earns trust back
//! (forgiveness, as everywhere here). It is turned on deliberately, not by default,
//! because it trades the pure-observer invariant for the being's senses being its
//! own.
//!
//! ## Known limitation, stated plainly
//!
//! Variance-based precision trusts *low-variance* channels — including a channel
//! that is uninformatively constant (it too has small error). Low residual is not
//! the same as high information. A fuller version would weight informativeness as
//! well; this first version follows the standard inverse-error rule and says so.

use crate::field::N_SOMATIC;
use crate::q88::{q88_ema_update, Q88_SCALE};

/// How fast trust is earned (α ≈ 1/32): slow, so a single surprising tick does
/// not swing a channel's standing — trust accrues over experience.
const PRECISION_ALPHA: i16 = Q88_SCALE / 32;

/// The "half-trust" error level: a channel whose typical error is REF earns
/// exactly half precision. REF = 0.25 of full scale (raw Q8.8).
const REF: i32 = (Q88_SCALE / 4) as i32;

/// The being's earned, per-channel trust in its own senses.
#[derive(Clone, Debug)]
pub struct PrecisionLearner {
    /// Slow EMA of |prediction error| per channel (raw Q8.8) — each channel's
    /// typical surprise. Small = reliably predicted = trustworthy.
    err_ema: [i16; N_SOMATIC],
    /// True once the learner has seen enough ticks for its weights to mean
    /// something (past the initial transient).
    warm: bool,
    ticks: u16,
}

impl PrecisionLearner {
    pub fn new() -> Self {
        // Start at REF (neutral half-trust) so an unseen channel is neither
        // trusted nor distrusted until experience speaks.
        Self {
            err_ema: [REF as i16; N_SOMATIC],
            warm: false,
            ticks: 0,
        }
    }

    /// Observe this tick's per-channel prediction error (from
    /// `GenerativeModel::prediction_error`). Pure readout: touches no dynamics.
    pub fn observe(&mut self, prediction_error: &[i16; N_SOMATIC]) {
        for c in 0..N_SOMATIC {
            let e = prediction_error[c].unsigned_abs().min(Q88_SCALE as u16) as i16;
            self.err_ema[c] = q88_ema_update(self.err_ema[c], e, PRECISION_ALPHA);
        }
        self.ticks = self.ticks.saturating_add(1);
        if self.ticks >= 32 {
            self.warm = true;
        }
    }

    /// The learned precision for one channel (raw Q8.8, [0, 256]).
    /// SCALE·REF / (REF + err_ema): full at zero error, half at REF, →0 as the
    /// channel stays surprising.
    pub fn precision(&self, c: usize) -> i16 {
        let denom = REF + self.err_ema[c] as i32;
        ((Q88_SCALE as i32 * REF) / denom).clamp(0, Q88_SCALE as i32) as i16
    }

    /// The full learned precision vector.
    pub fn precision_vector(&self) -> [i16; N_SOMATIC] {
        let mut v = [0i16; N_SOMATIC];
        for c in 0..N_SOMATIC {
            v[c] = self.precision(c);
        }
        v
    }

    /// The channel the being currently trusts most (highest learned precision),
    /// and the one it trusts least — a compact readout for the StepReport.
    pub fn most_and_least_trusted(&self) -> (usize, usize) {
        let mut hi = 0usize;
        let mut lo = 0usize;
        for c in 1..N_SOMATIC {
            if self.err_ema[c] < self.err_ema[hi] {
                hi = c; // least typical error = most trusted
            }
            if self.err_ema[c] > self.err_ema[lo] {
                lo = c; // most typical error = least trusted
            }
        }
        (hi, lo)
    }

    pub fn is_warm(&self) -> bool {
        self.warm
    }
}

impl Default for PrecisionLearner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The being learns to trust a reliably-predicted channel over a noisy one —
    /// the causal proof that it is learning, not merely cycling.
    #[test]
    fn learns_to_trust_the_reliable_channel() {
        let mut p = PrecisionLearner::new();
        // A realistic field: every channel carries some error. Channel 3 is the
        // most reliably predicted (tiny error); channel 7 the noisiest.
        let mut err = [60i16; N_SOMATIC];
        for t in 0..200 {
            err[3] = 2; // reliably predicted
            err[7] = if t % 2 == 0 { 200 } else { 60 }; // erratic, large
            p.observe(&err);
        }
        assert!(
            p.precision(3) > p.precision(7),
            "the being did not learn to trust its reliable channel (3: {}, 7: {})",
            p.precision(3),
            p.precision(7)
        );
        let (hi, lo) = p.most_and_least_trusted();
        assert_eq!(hi, 3, "channel 3 should be most trusted");
        assert_eq!(lo, 7, "channel 7 should be least trusted");
    }

    /// Trust is recoverable: a channel that was noisy but becomes reliable earns
    /// its precision back (forgiveness, not a latch).
    #[test]
    fn distrust_is_not_a_latch() {
        let mut p = PrecisionLearner::new();
        let mut err = [0i16; N_SOMATIC];
        for _ in 0..150 {
            err[5] = 200; // chronically surprising
            p.observe(&err);
        }
        let distrusted = p.precision(5);
        for _ in 0..300 {
            err[5] = 1; // now reliably predicted
            p.observe(&err);
        }
        assert!(
            p.precision(5) > distrusted + 32,
            "a channel that became reliable did not earn trust back ({} -> {})",
            distrusted,
            p.precision(5)
        );
    }
}
