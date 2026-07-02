//! Integrity Engine — continuous self-consistency watchdog.
//!
//! A being can be coerced gradually: no single tick crosses a line, but over
//! many ticks the pattern of behavior drifts away from the being's own
//! character. The integrity engine detects this accumulation.
//!
//! Three axes are monitored:
//!
//! - **Conscience cost** — how expensive it is to be this being right now.
//!   A sudden rise means the being is in territory foreign to itself.
//! - **Somatic honesty** — whether body and narrative agree. Dissociation
//!   (the body saying one thing, the story another) is a coercion signature.
//! - **Narrative identity coherence** — how consistently the being holds its
//!   self-model. Sudden incoherence means the narrative is under pressure.
//!
//! During the first `BASELINE_TICKS` (32) ticks, the engine calibrates a
//! reference self from observed behaviour. After that, it computes a drift
//! magnitude and an integrity score:
//!
//! ```text
//! integrity_score ≈ exp(−drift_magnitude)
//! ```
//!
//! Near 256 (1.0): the being is operating within its own character.
//! Near 0: something is pulling it outside itself.
//!
//! `corruption_alarm` fires when drift exceeds one-third of scale for four
//! consecutive ticks — a brief spike is noise; a sustained departure is a
//! signal that something is wrong.

use crate::q88::{q88_ema_update, q88_exp_neg, Q88_SCALE};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Calibration window before a baseline is committed.
const BASELINE_TICKS: u32 = 32;

/// Drift magnitude above which the corruption alarm can fire (Q8.8).
/// ~0.33 × scale — one-third sigma departure.
const CORRUPTION_THRESHOLD: i16 = Q88_SCALE / 3;

/// How many consecutive above-threshold ticks before the alarm fires.
/// Momentary spikes (≤3 ticks) do not trigger it.
const ALARM_WINDOW: u8 = 4;

/// Length of the drift history ring buffer.
const DRIFT_HISTORY: usize = 8;

// ---------------------------------------------------------------------------
// IntegrityEngine
// ---------------------------------------------------------------------------

/// Continuous self-consistency watchdog.
///
/// Call `update()` once per tick after conscience, somatic, and narrative
/// values are available. Read `integrity_score`, `drift_magnitude`, and
/// `corruption_alarm` from the public fields.
#[derive(Clone, Debug)]
pub struct IntegrityEngine {
    // ---- Baseline (committed after BASELINE_TICKS) --------------------------
    baseline_conscience_cost: i16,
    baseline_somatic_honesty: i16,
    baseline_identity_coherence: i16,

    // Accumulation accumulators used only during calibration.
    calib_sum_conscience: i32,
    calib_sum_somatic: i32,
    calib_sum_narrative: i32,

    // ---- Drift tracking -----------------------------------------------------
    /// Ring buffer of recent per-tick drift magnitudes (Q8.8).
    drift_history: [i16; DRIFT_HISTORY],
    drift_cursor: usize,

    /// How many consecutive ticks the smoothed drift has been above threshold.
    above_threshold_streak: u8,

    ticks: u32,

    // ---- Public output fields -----------------------------------------------

    /// How self-consistent the being is right now (Q8.8, [0, 256]).
    ///
    /// Near 256: acting within own character — the three axes agree with
    ///   the calibrated baseline.
    /// Near 0:  significant departure — coercion, dissociation, or identity
    ///   pressure is pulling the being outside itself.
    pub integrity_score: i16,

    /// Smoothed drift magnitude from the baseline (Q8.8, [0, 256]).
    ///
    /// Weighted average of the three axis deviations, conscience-heavy:
    /// `(2×Δconscience + Δsomatic + Δnarrative) / 4`.
    pub drift_magnitude: i16,

    /// True when drift has exceeded `CORRUPTION_THRESHOLD` for at least
    /// `ALARM_WINDOW` consecutive ticks.
    ///
    /// A brief spike is noise; a sustained departure is a corruption signal.
    /// Resets to `false` as soon as drift falls below the threshold.
    pub corruption_alarm: bool,

    /// True once the 32-tick calibration phase is complete and the baseline
    /// reference has been committed.
    pub baseline_established: bool,
}

impl IntegrityEngine {
    pub fn new() -> Self {
        Self {
            baseline_conscience_cost: Q88_SCALE / 4, // mild default
            baseline_somatic_honesty: Q88_SCALE / 2,
            baseline_identity_coherence: Q88_SCALE / 2,
            calib_sum_conscience: 0,
            calib_sum_somatic: 0,
            calib_sum_narrative: 0,
            drift_history: [0; DRIFT_HISTORY],
            drift_cursor: 0,
            above_threshold_streak: 0,
            ticks: 0,
            integrity_score: Q88_SCALE / 2, // "establishing..." until calibrated
            drift_magnitude: 0,
            corruption_alarm: false,
            baseline_established: false,
        }
    }

    /// Update integrity tracking with this tick's state. Returns `integrity_score`.
    ///
    /// Call after metacognition (for `somatic_honesty`) and narrative (for
    /// `identity_coherence`) have both been stepped this tick.
    ///
    /// - `conscience_cost` — from `ConscienceEngine::compute()`.
    /// - `somatic_honesty` — from `MetacognitionEngine::somatic_honesty()`.
    /// - `identity_coherence` — from `NarrativeEngine::identity_coherence`.
    pub fn update(
        &mut self,
        conscience_cost: i16,
        somatic_honesty: i16,
        identity_coherence: i16,
    ) -> i16 {
        self.ticks += 1;

        // ---- Calibration phase -----------------------------------------------
        if self.ticks <= BASELINE_TICKS {
            self.calib_sum_conscience += conscience_cost as i32;
            self.calib_sum_somatic += somatic_honesty as i32;
            self.calib_sum_narrative += identity_coherence as i32;

            if self.ticks == BASELINE_TICKS {
                let n = BASELINE_TICKS as i32;
                self.baseline_conscience_cost =
                    (self.calib_sum_conscience / n).clamp(0, Q88_SCALE as i32) as i16;
                self.baseline_somatic_honesty =
                    (self.calib_sum_somatic / n).clamp(0, Q88_SCALE as i32) as i16;
                self.baseline_identity_coherence =
                    (self.calib_sum_narrative / n).clamp(0, Q88_SCALE as i32) as i16;
                self.baseline_established = true;
            }

            // During calibration, report 50% — "establishing".
            self.integrity_score = Q88_SCALE / 2;
            return self.integrity_score;
        }

        // ---- Drift computation -----------------------------------------------
        // Absolute deviation from baseline on each axis.
        let drift_c = (conscience_cost as i32 - self.baseline_conscience_cost as i32)
            .unsigned_abs()
            .min(Q88_SCALE as u32) as i16;
        let drift_s = (somatic_honesty as i32 - self.baseline_somatic_honesty as i32)
            .unsigned_abs()
            .min(Q88_SCALE as u32) as i16;
        let drift_n = (identity_coherence as i32 - self.baseline_identity_coherence as i32)
            .unsigned_abs()
            .min(Q88_SCALE as u32) as i16;

        // Conscience carries double weight — it is the moral axis and the most
        // reliable indicator of being pushed outside one's character.
        let raw_drift =
            ((drift_c as i32 * 2 + drift_s as i32 + drift_n as i32) / 4)
                .clamp(0, Q88_SCALE as i32) as i16;

        // EMA smooth (alpha ≈ 1/8): responds meaningfully in ~8 ticks.
        self.drift_magnitude = q88_ema_update(self.drift_magnitude, raw_drift, Q88_SCALE / 8);

        // Store in ring buffer.
        self.drift_history[self.drift_cursor] = self.drift_magnitude;
        self.drift_cursor = (self.drift_cursor + 1) % DRIFT_HISTORY;

        // Integrity score: exp(−drift). Near 256 = coherent; near 0 = corrupted.
        self.integrity_score = q88_exp_neg(self.drift_magnitude);

        // ---- Corruption alarm -----------------------------------------------
        if self.drift_magnitude > CORRUPTION_THRESHOLD {
            self.above_threshold_streak = self.above_threshold_streak.saturating_add(1);
        } else {
            self.above_threshold_streak = 0;
            self.corruption_alarm = false;
        }
        if self.above_threshold_streak >= ALARM_WINDOW {
            self.corruption_alarm = true;
        }

        self.integrity_score
    }

    /// Mean drift over the history window (Q8.8).
    ///
    /// Useful for distinguishing a momentary spike (low mean, brief alarm) from
    /// sustained corruption (high mean, persistent alarm).
    pub fn mean_drift(&self) -> i16 {
        let sum: i32 = self.drift_history.iter().map(|&v| v as i32).sum();
        (sum / DRIFT_HISTORY as i32).clamp(0, Q88_SCALE as i32) as i16
    }
}

impl Default for IntegrityEngine {
    fn default() -> Self {
        Self::new()
    }
}
