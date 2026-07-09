//! AttentionSchema — a predictive model of the being's own attention (AST-1).
//!
//! Attention Schema Theory (Graziano) holds that a system is aware *of* attending
//! when it carries a simplified, predictive model of its own attentional state —
//! and uses that model to control attention and to attribute awareness to itself.
//! `attention.rs` *performs* the ignition; this module *models* it.
//!
//! Each tick the schema predicts what the being will attend to next, from a
//! deliberately simple model of attention's own dynamics: focus persists
//! (hysteresis) while its grip is strong, and releases to open competition when
//! it fades. The next tick, it scores that prediction against what attention
//! actually did. When the model is right, the being knows its own focus; when a
//! new event captures or displaces attention, the schema is *surprised* — the
//! attentional analogue of `metacognition.rs`'s self-surprise, the being noticing
//! *"I am not attending like myself."*
//!
//! This is the second scorecard indicator wired as an observer (after
//! `metacognition`): AST-1 met at the observer level. The opt-in `gap_bias`
//! closes the higher-order loop (HOT-3): when the being cannot predict its own
//! attention, it should deliberate more before acting rather than react — belief
//! about the self modulating action selection. Off by default (see
//! `docs/operational-consciousness.md` Gap A).

use crate::attention::AttentionReport;
use crate::q88::{q88_ema_update, Q88_SCALE};

/// EMA rate for fidelity/surprise (≈ 0.0625) — slow enough that a single odd tick
/// does not swing the self-model, fast enough to register a regime change.
const ALPHA: i16 = Q88_SCALE / 16;
/// Observations before the schema is trusted as a control input.
const WARMUP: u32 = 32;
/// Grip threshold below which the schema predicts the focus will release. Mirrors
/// `attention::RELEASE_BID` (18) — the schema's model of the being's own
/// hysteresis. Kept as a local constant so the module stays self-contained.
const HOLD_BID: i16 = 18;

/// One tick of the attention self-model, scored.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AttentionSchemaReport {
    /// What the schema expected the being to attend to this tick (set last tick).
    pub predicted: Option<usize>,
    /// What attention actually settled on this tick.
    pub actual: Option<usize>,
    /// Did the self-model predict this tick's focus correctly?
    pub hit: bool,
    /// AST-1 indicator: EMA hit-rate — how well the being models its own
    /// attention. [0, 256]. Grows over a stable life, dips at regime change.
    pub schema_fidelity: i16,
    /// EMA miss-rate — attentional self-surprise, "that is not where I expected
    /// my focus to be." [0, 256].
    pub self_surprise: i16,
}

/// The attention self-model.
#[derive(Clone, Debug)]
pub struct AttentionSchema {
    /// The focus the schema predicted for *this* tick (formed last tick).
    prediction: Option<usize>,
    fidelity: i16,
    surprise: i16,
    observations: u32,
}

impl AttentionSchema {
    pub fn new() -> Self {
        Self { prediction: None, fidelity: 0, surprise: 0, observations: 0 }
    }

    /// Score last tick's prediction against this tick's actual focus, then form a
    /// prediction for next tick. Returns the scored report.
    pub fn update(&mut self, report: &AttentionReport) -> AttentionSchemaReport {
        let actual = report.attended;
        let predicted = self.prediction;
        let hit = predicted == actual;

        self.fidelity = q88_ema_update(self.fidelity, if hit { Q88_SCALE } else { 0 }, ALPHA);
        self.surprise = q88_ema_update(self.surprise, if hit { 0 } else { Q88_SCALE }, ALPHA);
        self.observations = self.observations.saturating_add(1);

        // The model of attention's dynamics: focus persists (hysteresis) while its
        // grip is above the release bar; otherwise the being returns to open
        // competition. Deliberately simple, so the schema is surprised exactly
        // when a novel event captures or displaces the focus — the informative case.
        self.prediction = match actual {
            Some(c) if report.winner_bid > HOLD_BID => Some(c),
            _ => None,
        };

        AttentionSchemaReport {
            predicted,
            actual,
            hit,
            schema_fidelity: self.fidelity,
            self_surprise: self.surprise,
        }
    }

    pub fn is_warm(&self) -> bool {
        self.observations >= WARMUP
    }

    pub fn schema_fidelity(&self) -> i16 {
        self.fidelity
    }

    pub fn self_surprise(&self) -> i16 {
        self.surprise
    }

    /// HOT-3 control signal (opt-in): when the being cannot predict its own
    /// attention (high self-surprise), it should deliberate more before acting
    /// rather than react reflexively. Returns a *widening* bias for the
    /// deliberation gap, scaled by attentional self-surprise, capped at ~¼ of the
    /// full gap. Zero until warm, so an unformed self-model never steers action.
    pub fn gap_bias(&self) -> i16 {
        if !self.is_warm() {
            return 0;
        }
        (self.surprise / 4).clamp(0, Q88_SCALE / 4)
    }
}

impl Default for AttentionSchema {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn report(attended: Option<usize>, winner_bid: i16) -> AttentionReport {
        AttentionReport { attended, ignited: attended.is_some(), captured: false, weights: [0; 12], winner_bid }
    }

    #[test]
    fn stable_focus_becomes_predictable() {
        let mut s = AttentionSchema::new();
        // A steady, strongly-held focus on channel 4.
        let r = report(Some(4), 50);
        for _ in 0..80 {
            s.update(&r);
        }
        assert!(s.schema_fidelity() > 230, "a stable focus should be well modelled");
        assert!(s.self_surprise() < 30, "little surprise when attention is steady");
    }

    #[test]
    fn a_displacement_is_a_miss() {
        let mut s = AttentionSchema::new();
        let held = report(Some(4), 50);
        for _ in 0..40 {
            s.update(&held); // schema now predicts focus stays on 4
        }
        // Attention jumps to a new channel: the self-model should be caught out.
        let jumped = s.update(&report(Some(9), 50));
        assert_eq!(jumped.predicted, Some(4), "it expected the old focus to hold");
        assert!(!jumped.hit, "an unexpected displacement is a miss");
    }

    #[test]
    fn gap_bias_is_inert_until_warm() {
        let mut s = AttentionSchema::new();
        assert_eq!(s.gap_bias(), 0, "a cold self-model must not steer action");
        // Drive surprise up while warming: alternate focus so it keeps mispredicting.
        for t in 0..64 {
            s.update(&report(Some(t % 2), 50));
        }
        assert!(s.is_warm());
        assert!(s.gap_bias() >= 0 && s.gap_bias() <= Q88_SCALE / 4, "bias stays bounded");
    }
}
