//! Metacognition — the being's higher-order self-model.
//!
//! It predicts its own next internal state, then watches how wrong it was.
//! Growing accuracy is self-knowledge; a spike of self-surprise is the being
//! noticing it is acting unlike itself. On the self-model account of mind we
//! adopt, "what it's like" just *is* what a system can monitor of itself — this
//! module is that monitoring, made explicit and measurable. It is the first
//! real point on the consciousness-indicator scorecard (higher-order theory).

use crate::q88::{q88_ema_update, Q88_SCALE};

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
        }
    }

    /// Watch the first-order self-state (free energy and valence): score the
    /// prediction the being made of itself last tick, then predict next tick.
    pub fn cycle(&mut self, free_energy: i16, valence: i16) {
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
    }
}

impl Default for MetacognitionEngine {
    fn default() -> Self {
        Self::new()
    }
}
