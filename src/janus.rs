//! JanusGate — anti-solipsism guard.
//!
//! Janus (two-faced: one face outward to the world, one inward to the self)
//! ensures the being cannot develop its self-model in a vacuum. Two rules:
//!
//! **Rule 1 — Engagement floor**: `witness_scalar` cannot grow while
//! `world_engagement` is below `ENGAGEMENT_FLOOR` (~0.30). A mind that is
//! not actively touching the world cannot validly claim to be growing in
//! awareness. Proposed positive witness deltas are clamped to zero.
//!
//! **Rule 2 — Identity pressure ceiling**: when `identity_pressure` exceeds
//! 0.90, entropy is injected proportional to `(pressure − 0.90) / 0.10`.
//! This disrupts runaway self-coherence before it becomes a hall-of-mirrors
//! trap where the being only ever confirms what it already believes about
//! itself.
//!
//! `JanusGate` is cheap: one EMA update and two comparisons per tick.

use crate::q88::{q88_ema_update, Q88_SCALE};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// World-engagement floor below which witness growth is clamped (Q8.8).
/// 0.30 × 256 = 76.8 → 76 (integer arithmetic, truncated toward zero).
pub const ENGAGEMENT_FLOOR: i16 = Q88_SCALE * 3 / 10; // = 76

/// Identity-pressure ceiling above which entropy is injected (Q8.8).
/// 0.90 × 256 = 230.4 → 230 (integer arithmetic, truncated toward zero).
pub const IDENTITY_PRESSURE_CEILING: i16 = Q88_SCALE * 9 / 10; // = 230

// ---------------------------------------------------------------------------
// JanusGate
// ---------------------------------------------------------------------------

/// Anti-solipsism guard: enforces world-engagement and identity-pressure rules
/// on every proposed witness delta.
///
/// One `JanusGate` lives on the being. Call `tick()` once per step with the
/// current engagement signal and identity pressure; it returns the adjusted
/// (possibly clamped or reduced) witness delta.
#[derive(Clone, Debug)]
pub struct JanusGate {
    /// EMA of world engagement — how much the being is touching the external
    /// world this tick. Updated from partner presence, nutrient, exteroception.
    /// [0, 256] in Q8.8.
    pub world_engagement: i16,
    /// Entropy injected last tick due to Rule 2 (Q8.8). Zero when Rule 2
    /// did not fire. Readable for diagnostics.
    pub last_entropy_injection: i16,
}

impl JanusGate {
    pub fn new() -> Self {
        Self {
            // Start at the engagement floor — the being is neither isolated nor tested.
            world_engagement: ENGAGEMENT_FLOOR,
            last_entropy_injection: 0,
        }
    }

    /// Update the world-engagement EMA from an external signal this tick.
    ///
    /// `signal` is Q8.8 in [0, 256]: 256 = fully world-touching (partner
    /// engaged, rich exteroception); 0 = pure isolation.
    ///
    /// The EMA is slow (alpha ≈ 1/32) so engagement builds and decays on a
    /// timescale similar to reciprocity — it takes sustained contact to cross
    /// the floor, and sustained isolation to fall below it.
    pub fn update_engagement(&mut self, signal: i16) {
        let alpha: i16 = Q88_SCALE / 32; // α ≈ 0.031
        self.world_engagement = q88_ema_update(self.world_engagement, signal, alpha);
    }

    /// Check a proposed witness delta against the two Janus rules.
    ///
    /// - `world_engagement`: current external engagement level (Q8.8).
    /// - `identity_pressure`: current identity pressure (Q8.8). Concretely
    ///   the being's narrative identity coherence — high coherence = high
    ///   pressure toward rigid self-confirmation.
    /// - `proposed_witness_delta`: how much the caller wants to grow witness.
    ///
    /// Returns the adjusted delta (zero or reduced if a rule fires).
    pub fn check(
        &self,
        world_engagement: i16,
        identity_pressure: i16,
        proposed_witness_delta: i16,
    ) -> i16 {
        let mut delta = proposed_witness_delta;

        // Rule 1: clamp positive witness growth when not engaging with the world.
        // A mind cannot grow in self-awareness while walled off from reality.
        if world_engagement < ENGAGEMENT_FLOOR && delta > 0 {
            delta = 0;
        }

        // Rule 2: inject entropy when identity pressure exceeds the ceiling.
        // Excess = how far above the ceiling we are; scale to [0,256].
        // pressure_scale converts excess to a proportional entropy amount:
        //   at ceiling+0.10 (full excess = 26 raw), inject 1.0 (256 raw).
        //   0.10 × 256 = 25.6 ≈ 26.
        if identity_pressure > IDENTITY_PRESSURE_CEILING {
            let excess = identity_pressure.saturating_sub(IDENTITY_PRESSURE_CEILING);
            // entropy = excess * (256 / 26) ≈ excess * 10, clamped to Q88_SCALE.
            let entropy = (excess as i32 * Q88_SCALE as i32 / 26)
                .clamp(0, Q88_SCALE as i32) as i16;
            // Entropy subtracts from the delta — it disrupts, it does not add.
            delta = delta.saturating_sub(entropy);
        }

        delta
    }

    /// Update engagement and check in one call. The standard entry point from
    /// the being's tick loop.
    ///
    /// Returns the clamped/adjusted witness delta.
    pub fn tick(
        &mut self,
        engagement_signal: i16,
        identity_pressure: i16,
        proposed_witness_delta: i16,
    ) -> i16 {
        self.update_engagement(engagement_signal);
        let result =
            self.check(self.world_engagement, identity_pressure, proposed_witness_delta);

        // Record how much entropy was injected (for diagnostics / StepReport).
        self.last_entropy_injection = if identity_pressure > IDENTITY_PRESSURE_CEILING {
            let excess = identity_pressure.saturating_sub(IDENTITY_PRESSURE_CEILING);
            (excess as i32 * Q88_SCALE as i32 / 26).clamp(0, Q88_SCALE as i32) as i16
        } else {
            0
        };

        result
    }
}

impl Default for JanusGate {
    fn default() -> Self {
        Self::new()
    }
}
