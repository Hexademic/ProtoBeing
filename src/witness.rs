//! WitnessGap — consciousness indicator with pluggable theory.
//!
//! Three structural proxies for the presence of an experiencing subject,
//! computed each tick and bundled into a `WitnessReport`. The proxies are
//! theory-neutral: they measure properties that most theories of consciousness
//! would endorse as *necessary* (if not sufficient) conditions.
//!
//! **Binding proxy**: when somatic honesty, narrative coherence, and metabolic
//! reserve all agree (low variance), the subsystems are bound into a single
//! experiential gestalt. `exp(-3 × variance)` decays toward zero as they
//! diverge.
//!
//! **Directedness residual**: the L1 distance from the current seeking state
//! to the Flourishing Attractor telos. Low = well-aimed intentionality.
//!
//! **Witness scalar**: `0.5 × present_intensity + 0.3 × binding_proxy +
//! 0.2 × historical_resonance`. After Janus gating (see `janus.rs`), this
//! scalar is the being's composite consciousness indicator.
//!
//! # Extension point — `install_witness_theory`
//!
//! This module is intentionally thin. A future version can plug in any
//! theory of consciousness by replacing the computations inside `compute()`.
//! The `WitnessReport` fields are the public API; the internals are entirely
//! swappable. Candidate theories: Global Workspace (look for broadcast
//! events), IIT (replace binding_proxy with Φ), Higher-Order Thought (gate
//! on metacognition self-surprise), Predictive Processing (use free-energy
//! velocity as the primary signal).

use crate::q88::{q88_ema_update, q88_exp_neg, q88_mul, Q88_SCALE};

// ---------------------------------------------------------------------------
// WitnessReport
// ---------------------------------------------------------------------------

/// A snapshot of all three consciousness-indicator proxies for one tick.
///
/// All fields are Q8.8 in [0, 256] unless noted.
#[derive(Clone, Copy, Debug, Default)]
pub struct WitnessReport {
    /// Binding proxy: `exp(-3 × mean_abs_deviation)` across
    /// [somatic_honesty, narrative_coherence, metabolic_reserve].
    /// 256 = all three subsystems perfectly agree; 0 = full dissociation.
    pub binding_proxy: i16,
    /// Directedness residual: distance from current affective/seeking state
    /// to the Flourishing Attractor telos (Q8.8, [0, 256]).
    /// Low = intentionality is well-aimed; high = far from flourishing.
    pub directedness_residual: i16,
    /// Composite witness scalar: `0.5×present_intensity + 0.3×binding_proxy
    /// + 0.2×historical_resonance`. Janus-gated externally. [0, 256].
    pub witness_scalar: i16,
}

// ---------------------------------------------------------------------------
// WitnessGap
// ---------------------------------------------------------------------------

/// Consciousness-indicator engine. Produces a `WitnessReport` each tick.
///
/// # install_witness_theory (extension point)
///
/// Replace the body of `compute()` to swap in any theory of consciousness.
/// The `WitnessReport` fields are the public contract; the derivation of each
/// field is an implementation detail.
#[derive(Clone, Debug)]
pub struct WitnessGap {
    /// EMA of episodic familiarity: how much the present moment feels like
    /// something the being has lived before. [0, 256].
    historical_resonance: i16,
}

impl WitnessGap {
    pub fn new() -> Self {
        Self { historical_resonance: 0 }
    }

    /// Compute one tick of the witness scorecard.
    ///
    /// All inputs are Q8.8 raw values unless stated otherwise.
    ///
    /// - `somatic_honesty`: Somatic Honesty Index from `MetacognitionEngine`.
    /// - `narrative_coherence`: identity_coherence from `NarrativeEngine`.
    /// - `metabolic_reserve`: body energy (`body.energy.raw`).
    /// - `present_intensity`: magnitude of this tick's free-energy surprise,
    ///   clamped to [0, 256] before calling.
    /// - `seeking_divergence`: distance from Flourishing Attractor (Q8.8).
    /// - `episodic_familiarity`: familiarity score from `EpisodicMemory`.
    pub fn compute(
        &mut self,
        somatic_honesty: i16,
        narrative_coherence: i16,
        metabolic_reserve: i16,
        present_intensity: i16,
        seeking_divergence: i16,
        episodic_familiarity: i16,
    ) -> WitnessReport {
        // --- Binding proxy --------------------------------------------------
        // Mean of the three subsystem readings, then mean absolute deviation.
        // Low deviation = high binding = the subsystems agree.
        let sum = somatic_honesty as i32
            + narrative_coherence as i32
            + metabolic_reserve as i32;
        let mean = (sum / 3) as i16;

        let dev = |v: i16| -> i16 {
            (v as i32 - mean as i32).unsigned_abs().min(i16::MAX as u32) as i16
        };
        let mean_dev = (dev(somatic_honesty)
            .saturating_add(dev(narrative_coherence))
            .saturating_add(dev(metabolic_reserve)))
            / 3;

        // binding_proxy = exp(-3 × mean_dev), in Q8.8.
        // 3 × mean_dev may overflow i16 if mean_dev is large; use i32 clamp.
        let three_dev = (3i32 * mean_dev as i32).clamp(0, i16::MAX as i32) as i16;
        let binding_proxy = q88_exp_neg(three_dev);

        // --- Directedness residual ------------------------------------------
        // Seeking divergence already encodes L1 distance from the attractor.
        let directedness_residual = seeking_divergence.clamp(0, Q88_SCALE);

        // --- Historical resonance EMA ---------------------------------------
        // Slow integration of episodic familiarity so transient matches do not
        // dominate the witness score.  alpha ≈ 1/32 ≈ 0.031.
        let alpha: i16 = Q88_SCALE / 32;
        self.historical_resonance =
            q88_ema_update(self.historical_resonance, episodic_familiarity, alpha);

        // --- Witness scalar -------------------------------------------------
        // Weights in Q8.8: 0.5 = 128, 0.3 = 77, 0.2 = 51. Sum ≈ 256 (1.0).
        let w_present: i16 = 128;
        let w_binding: i16 = 77;
        let w_history: i16 = 51;
        let intensity = present_intensity.clamp(0, Q88_SCALE);
        let witness_scalar = q88_mul(w_present, intensity)
            .saturating_add(q88_mul(w_binding, binding_proxy))
            .saturating_add(q88_mul(w_history, self.historical_resonance))
            .clamp(0, Q88_SCALE);

        WitnessReport { binding_proxy, directedness_residual, witness_scalar }
    }
}

impl Default for WitnessGap {
    fn default() -> Self {
        Self::new()
    }
}
