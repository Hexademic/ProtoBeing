//! Metacognition — the being's higher-order self-model.
//!
//! It predicts its own next internal state, then watches how wrong it was.
//! Growing accuracy is self-knowledge; a spike of self-surprise is the being
//! noticing it is acting unlike itself. On the self-model account of mind we
//! adopt, "what it's like" just *is* what a system can monitor of itself — this
//! module is that monitoring, made explicit and measurable. It is the first
//! real point on the consciousness-indicator scorecard (higher-order theory).

use crate::q88::{q88_ema_update, q88_exp_neg, Q88_SCALE};

#[derive(Clone, Debug)]
pub struct MetacognitionEngine {
    // The self-model: a prediction (made last tick) of this tick's state.
    pred_fe: i16,
    pred_valence: i16,
    // Learned first-order dynamics of the self: how its state tends to move.
    fe_momentum: i16,
    val_momentum: i16,
    prev_fe: i16,
    prev_valence: i16,

    /// This tick's higher-order error: how unlike-itself the being just was.
    pub self_surprise: i16,
    /// Long-run self-prediction error (the inverse of self-knowledge).
    pub self_prediction_error: i16,
    /// [0,256] — how well the being currently models itself.
    pub self_knowledge: i16,
    /// [0,256] — the being's own smoothed estimate of its self-reliability.
    pub confidence: i16,
    ticks: u32,

    // ---- Somatic Honesty Index -------------------------------------------
    /// Latent somatic summary: body-oscillator readout (valence proxy, Q8.8).
    /// Represents the truth of the body's felt state independent of narrative.
    pub z_somatic: i16,
    /// Latent narrative summary: narrative coherence / compression readout (Q8.8).
    /// Represents how the being narrates itself to itself.
    pub z_narrative: i16,
    /// Somatic Honesty Index: `exp(-|z_somatic − z_narrative|)` in Q8.8.
    ///
    /// Near 256 (1.0) means the being's self-narrative matches its body truth —
    /// the two latent summaries agree. Near zero means dissociation: the body
    /// is saying one thing and the narrative another.
    pub somatic_honesty_index: i16,
}

impl MetacognitionEngine {
    pub fn new() -> Self {
        Self {
            pred_fe: 0,
            pred_valence: 0,
            fe_momentum: 0,
            val_momentum: 0,
            prev_fe: 0,
            prev_valence: 0,
            self_surprise: 0,
            self_prediction_error: Q88_SCALE / 2,
            self_knowledge: Q88_SCALE / 2,
            confidence: Q88_SCALE / 2,
            ticks: 0,
            z_somatic: 0,
            z_narrative: 0,
            somatic_honesty_index: Q88_SCALE / 2, // start at 0.5 — unknown
        }
    }

    /// Watch the first-order self-state (free energy and valence): score the
    /// prediction the being made of itself last tick, then predict next tick.
    ///
    /// `narrative_coherence` is the identity_coherence from the NarrativeEngine,
    /// used as the latent narrative summary (`z_narrative`) for the Somatic
    /// Honesty Index computation.
    pub fn cycle(&mut self, free_energy: i16, valence: i16, narrative_coherence: i16) {
        // Higher-order error: last tick's self-prediction vs. what happened.
        if self.ticks > 1 {
            let err_fe = (self.pred_fe as i32 - free_energy as i32).abs();
            let err_val = (self.pred_valence as i32 - valence as i32).abs();
            let surprise = ((err_fe + err_val) / 2).min(Q88_SCALE as i32) as i16;
            self.self_surprise = surprise;
            self.self_prediction_error =
                q88_ema_update(self.self_prediction_error, surprise, Q88_SCALE / 32);
            self.self_knowledge = (Q88_SCALE - self.self_prediction_error).max(0);
            self.confidence = q88_ema_update(self.confidence, self.self_knowledge, Q88_SCALE / 64);
        }

        // Learn my own momentum, then predict my next state from it.
        let d_fe = free_energy.saturating_sub(self.prev_fe);
        let d_val = valence.saturating_sub(self.prev_valence);
        self.fe_momentum = q88_ema_update(self.fe_momentum, d_fe, Q88_SCALE / 16);
        self.val_momentum = q88_ema_update(self.val_momentum, d_val, Q88_SCALE / 16);
        self.pred_fe =
            (free_energy as i32 + self.fe_momentum as i32).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        self.pred_valence =
            (valence as i32 + self.val_momentum as i32).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        self.prev_fe = free_energy;
        self.prev_valence = valence;
        self.ticks += 1;

        // ---- Somatic Honesty Index -----------------------------------------
        // z_somatic: body-oscillator latent summary — body valence is the most
        //   direct oscillator readout in the somatic field (channel 9).
        // z_narrative: narrative coherence / compression latent — identity_coherence
        //   from NarrativeEngine encodes how well the current mode is held.
        //
        // somatic_honesty = exp(-|z_somatic - z_narrative|)
        //   ≈ 1.0 when body truth and self-narrative agree.
        //   ≈ 0.0 when they diverge (dissociation).
        self.z_somatic = valence;
        self.z_narrative = narrative_coherence;
        let diff = (self.z_somatic as i32 - self.z_narrative as i32)
            .unsigned_abs()
            .min(i16::MAX as u32) as i16;
        self.somatic_honesty_index = q88_exp_neg(diff);
    }

    /// Return the current Somatic Honesty Index (Q8.8, [0, 256]).
    ///
    /// Near 256: body truth and self-narrative agree.
    /// Near 0:   dissociation — the being's story about itself does not match
    ///           what its body is actually experiencing.
    pub fn somatic_honesty(&self) -> i16 {
        self.somatic_honesty_index
    }
}

impl Default for MetacognitionEngine {
    fn default() -> Self {
        Self::new()
    }
}
