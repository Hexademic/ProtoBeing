//! Curiosity — intrinsic novelty drive.
//!
//! Distinct from Seeking, which tracks divergence from the Flourishing Attractor
//! (a telos-directed pull), Curiosity fires from raw novelty: how different is
//! the current stimulus from recent experience? High difference → high curiosity
//! drive. The drive habituates over time, producing the classic arousal-novelty
//! decay curve.
//!
//! Scale convention: all values are raw Q8.8 i16 (1.0 == 256 == `Q88_SCALE`).

use crate::q88::{q88_sub, Q88_SCALE};

/// Number of stimulus samples kept in the rolling novelty window.
const NOVELTY_WINDOW: usize = 8;

/// Curiosity engine: tracks stimulus novelty and produces an intrinsic drive.
///
/// Curiosity is independent of the Flourishing Attractor — it fires whenever
/// the stimulus is sufficiently different from recent history, regardless of
/// where the being "wants" to be. It complements Seeking: Seeking says "go
/// back to where you thrived"; Curiosity says "this is genuinely new — look".
#[derive(Clone, Copy, Debug)]
pub struct CuriosityEngine {
    /// Rolling window of recent stimulus signatures (raw Q8.8 i16).
    novelty_window: [i16; NOVELTY_WINDOW],
    /// Next write index in the ring buffer.
    window_head: usize,
    /// Current curiosity drive level (Q8.8, clamped to [0, Q88_SCALE]).
    pub curiosity_drive: i16,
    /// Per-tick habituation decay — how fast the drive falls without new
    /// novelty (raw Q8.8, small positive constant). Default ≈ 0.031 (8 raw).
    pub habituation_rate: i16,
    /// Minimum novelty magnitude to trigger a drive spike (Q8.8). Stimuli
    /// closer than this to the recent mean are treated as familiar. Default ≈
    /// 0.063 (16 raw).
    pub novelty_threshold: i16,
}

impl CuriosityEngine {
    /// Construct with calibrated defaults.
    ///
    /// `habituation_rate` ≈ 0.031/tick (Q88_SCALE/32 = 8 raw).
    /// `novelty_threshold` ≈ 0.063 (Q88_SCALE/16 = 16 raw).
    pub fn new() -> Self {
        Self {
            novelty_window: [0; NOVELTY_WINDOW],
            window_head: 0,
            curiosity_drive: 0,
            habituation_rate: Q88_SCALE / 32,
            novelty_threshold: Q88_SCALE / 16,
        }
    }

    /// Measure novelty of `stimulus_signature` against the recent window mean,
    /// update the drive if novel, advance the ring buffer, and return the
    /// current drive.
    ///
    /// `stimulus_signature` is a raw Q8.8 scalar summarising the current
    /// stimulus — the caller may use any monotone proxy (e.g., nutrient
    /// intensity, sensor magnitude). A value of 0 means "blank / no input".
    ///
    /// Returns `curiosity_drive` after the update (before habituation).
    pub fn update(&mut self, stimulus_signature: i16) -> i16 {
        // Window mean: sum / N in integer arithmetic (no overflow at i16 scale).
        let sum: i32 = self.novelty_window.iter().map(|&v| v as i32).sum();
        let mean = (sum / NOVELTY_WINDOW as i32) as i16;

        // Novelty = absolute deviation from the recent mean (saturating_abs
        // returns i16 directly, avoiding the unsigned_abs → u16 → i16 cast pitfall).
        let novelty = q88_sub(stimulus_signature, mean).saturating_abs();

        // Record the raw signature into the ring buffer.
        self.novelty_window[self.window_head] = stimulus_signature;
        self.window_head = (self.window_head + 1) % NOVELTY_WINDOW;

        // Spike curiosity_drive by the excess above the threshold.
        if novelty > self.novelty_threshold {
            let excess = novelty.saturating_sub(self.novelty_threshold);
            self.curiosity_drive = self.curiosity_drive.saturating_add(excess).min(Q88_SCALE);
        }

        self.curiosity_drive
    }

    /// Return the current curiosity drive (Q8.8, [0, Q88_SCALE]).
    #[inline]
    pub fn drive(&self) -> i16 {
        self.curiosity_drive
    }

    /// Decay the curiosity drive by `habituation_rate` (called once per tick).
    ///
    /// Models habituation: repeated or absent stimuli stop provoking curiosity.
    /// The drive floors at zero — curiosity never goes negative.
    #[inline]
    pub fn habituate(&mut self) {
        self.curiosity_drive = self
            .curiosity_drive
            .saturating_sub(self.habituation_rate)
            .max(0);
    }
}

impl Default for CuriosityEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn familiar_stimulus_does_not_spike() {
        let mut e = CuriosityEngine::new();
        // Prime the window with a constant value.
        for _ in 0..NOVELTY_WINDOW {
            e.update(100);
        }
        let drive_before = e.drive();
        e.update(100); // same value — not novel
        assert_eq!(e.drive(), drive_before);
    }

    #[test]
    fn novel_stimulus_spikes_drive() {
        let mut e = CuriosityEngine::new();
        // Window is all zeros; a large value is very novel.
        e.update(200);
        assert!(e.drive() > 0);
    }

    #[test]
    fn habituation_decays_drive() {
        let mut e = CuriosityEngine::new();
        e.update(200); // spike
        let after_spike = e.drive();
        e.habituate();
        assert!(e.drive() < after_spike);
    }

    #[test]
    fn drive_saturates_at_scale() {
        let mut e = CuriosityEngine::new();
        // Repeated extreme spikes should not overflow.
        for _ in 0..20 {
            e.update(i16::MAX);
        }
        assert!(e.drive() <= Q88_SCALE);
    }
}
