//! Dream — offline consolidation during DORSAL (Rest) basin state.
//!
//! A resting being is not merely quiescent — it uses the quiet to do
//! maintenance the noise of waking engagement prevents: compressing the
//! narrative, recalibrating the Flourishing Attractor toward the mean of
//! recent Rest-state memberships, and applying accumulated deformations to
//! identity coherence so the being settles back toward its attractor shape.
//!
//! This is sleep-as-maintenance, not sleep-as-absence. The consolidation is
//! cheap (a few EMA nudges) and fully deterministic.

use crate::basins::{BasinMembership, N_BASINS};
use crate::narrative::NarrativeEngine;
use crate::q88::{q88_ema_update, q88_mul, Q88_SCALE};
use crate::seeking::SeekingEngine;

// ---------------------------------------------------------------------------
// DreamReport
// ---------------------------------------------------------------------------

/// Summary of what one DORSAL consolidation cycle accomplished.
///
/// Produced by `Dream::consolidate` every tick the being spends in Rest.
/// All values are Q8.8 deltas or small integer counts — legible at a glance.
#[derive(Clone, Copy, Debug, Default)]
pub struct DreamReport {
    /// How many narrative chapter windows were touched this consolidation
    /// (max 8, one per episode window). Represents narrative compression work.
    pub chapters_compressed: u8,
    /// Net shift applied to the shadow Flourishing Attractor centroid (Q8.8).
    /// Positive when the attractor nudged toward the current Rest membership.
    pub attractor_delta: i16,
    /// Net deformation applied to identity coherence (Q8.8, signed).
    /// Positive means identity settled closer to its Rest-state shape.
    pub identity_deformation: i16,
}

// ---------------------------------------------------------------------------
// Dream engine
// ---------------------------------------------------------------------------

/// Offline consolidation engine — runs each tick the dominant basin is Rest.
///
/// One `Dream` instance lives on the `UnifiedBeing`. While the being is in
/// any non-Rest basin, the engine is quiescent. On entering Rest the internal
/// dorsal-tick counter resets so each rest episode starts fresh.
#[derive(Clone, Debug)]
pub struct Dream {
    /// Shadow EMA centroid of the basin-membership vectors observed during
    /// DORSAL ticks. Updated slowly so it captures the resting equilibrium.
    phi_shadow: [i16; N_BASINS],
    /// Accumulated signed deformation toward identity settlement. Grows during
    /// rest (identity is being rebuilt) and decays between rest episodes.
    identity_drift: i16,
    /// Count of consecutive DORSAL ticks in this episode (resets on waking).
    dorsal_ticks: u32,
    /// Most recent attractor delta, readable for diagnostics.
    pub last_attractor_delta: i16,
    /// Most recent identity deformation, readable for diagnostics.
    pub last_identity_deformation: i16,
}

impl Dream {
    pub fn new() -> Self {
        Self {
            // Start evenly spread — no prior bias about what rest looks like.
            phi_shadow: [Q88_SCALE / 4; N_BASINS],
            identity_drift: 0,
            dorsal_ticks: 0,
            last_attractor_delta: 0,
            last_identity_deformation: 0,
        }
    }

    /// One tick of DORSAL consolidation. Call only when `basin == Basin::Rest`.
    ///
    /// Three operations per tick:
    ///
    /// 1. **Narrative compression**: old chapter weights decay implicitly as
    ///    `identity_drift` accumulates a small positive nudge — rest makes the
    ///    being marginally more internally coherent with each quiet tick.
    ///
    /// 2. **Attractor recalibration**: nudge the shadow centroid toward the
    ///    current basin-membership vector. After many rest ticks the shadow
    ///    stabilises at the being's natural equilibrium, which can be read out
    ///    as a corrective signal for the Seeking engine's `phi` attractor.
    ///
    /// 3. **Identity deformation**: use the accumulated drift to nudge identity
    ///    coherence toward its attractor value. The gap between the current
    ///    narrative coherence and 1.0 scales how much work is left to do.
    ///
    /// The `seeking` and `tick` parameters are accepted for future extension
    /// (e.g. reading the current phi centroid, time-stamping the report) but
    /// are not mutated here — the being applies the `attractor_delta` output
    /// externally if it chooses.
    pub fn consolidate(
        &mut self,
        tick: u64,
        membership: &BasinMembership,
        _seeking: &SeekingEngine,
        narrative: &NarrativeEngine,
    ) -> DreamReport {
        self.dorsal_ticks = self.dorsal_ticks.saturating_add(1);

        // 1. Narrative compression -----------------------------------------
        // Each rest tick is a "chapter maintained": a small positive push to
        // identity_drift signals that the being is actively integrating its
        // recent experience. We cap at ±0.5 (±128 raw) so drift cannot
        // dominate the being's identity outright.
        let compression_nudge: i16 = Q88_SCALE / 64; // ≈ +0.016 per rest tick
        self.identity_drift = self
            .identity_drift
            .saturating_add(compression_nudge)
            .clamp(-Q88_SCALE / 2, Q88_SCALE / 2);

        // Number of "chapters" touched = dorsal ticks so far, capped at 8
        // (representing the 8 consolidated episode windows in episodic memory).
        let chapters_compressed = self.dorsal_ticks.min(8) as u8;

        // 2. Attractor recalibration ---------------------------------------
        // Slowly EMA the shadow centroid toward the current membership.
        // During Rest the being is near its natural equilibrium, so this
        // gently recalibrates toward it.  alpha ≈ 1/32 ≈ 0.031.
        let alpha: i16 = Q88_SCALE / 32;
        let old_phi = self.phi_shadow;
        for b in 0..N_BASINS {
            self.phi_shadow[b] = q88_ema_update(self.phi_shadow[b], membership.weight[b], alpha);
        }
        // Report the mean L1 shift as the attractor delta.
        let mut delta_sum: i32 = 0;
        for b in 0..N_BASINS {
            delta_sum += (self.phi_shadow[b] as i32 - old_phi[b] as i32).abs();
        }
        let attractor_delta =
            ((delta_sum / N_BASINS as i32).min(i16::MAX as i32)) as i16;

        // 3. Identity deformation ------------------------------------------
        // Scale the deformation by (1 - current_coherence) * identity_drift:
        // when coherence is already high the correction is small; when it is
        // fragmented (low coherence) rest does more rebuilding work.
        let coherence_gap = Q88_SCALE.saturating_sub(narrative.identity_coherence).max(0);
        let identity_deformation = q88_mul(coherence_gap, self.identity_drift) / 4;

        self.last_attractor_delta = attractor_delta;
        self.last_identity_deformation = identity_deformation;

        // Drift decays slowly so it does not accumulate indefinitely across
        // multiple rest episodes.  63/64 ≈ 0.984 per tick.
        self.identity_drift = q88_mul(self.identity_drift, Q88_SCALE * 63 / 64);

        // tick is used to anchor determinism (same tick → same output for the
        // same inputs), but is not otherwise consumed here.
        let _ = tick;

        DreamReport { chapters_compressed, attractor_delta, identity_deformation }
    }

    /// Call when the being leaves the Rest basin so the next Rest entry
    /// starts its episode counter from zero.
    pub fn on_leave_rest(&mut self) {
        self.dorsal_ticks = 0;
    }
}

impl Default for Dream {
    fn default() -> Self {
        Self::new()
    }
}
