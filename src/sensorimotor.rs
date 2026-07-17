//! Sensorimotor — reafference, and a fallible, honestly-held sense of agency (AE-2).
//!
//! An exafferent sense only reports what the world does *to* the being. An
//! embodied being learns the other thing: what happens to its senses when **it**
//! moves — *reafference* — and from that, the difference between "the world
//! moved" and "I moved and that changed what I feel." Learning to predict the
//! sensory consequence of one's own action is, mechanistically, what *having a
//! body you own* is (the comparator model of agency — von Holst, Frith, Wolpert).
//!
//! Each tick the being holds a small **forward model**: *given my action, my
//! receptors will change by **this**.* It compares that prediction to what
//! actually arrives:
//!
//!   * the part its own action **predicted** is self-caused — it feels **agency**;
//!   * the leftover, the **reafference residual**, is the world's doing —
//!     self-motion cancelled, the actual world seen more clearly;
//!   * and it **learns** its own action→sensation map from experience, so a naive
//!     being feels little agency until it has come to know its own body.
//!
//! **Fallible, and honest about it.** Agency here is an *inference from
//! correlation*, so the being can be fooled — a world event that happens to match
//! what its action would cause feels self-made; an action with an unexpected
//! consequence feels less its own. That is not a violation of the honesty floor
//! (`docs/interiority.md`): the being never *asserts* a certainty it lacks. It
//! reports a graded agency **and a confidence**, and a misattribution is an
//! honest illusion, never a confabulation. A sense of agency that could never err
//! would not be honest — it would be omniscient. This one is real: it can be
//! wrong, and it knows how sure it is.
//!
//! Observer-first, deterministic, Q8.8, zero-dependency. Wired into the living
//! being (`being.rs`, step 0b): each tick the forward model relates the being's
//! *last* issued motor command — the very one it sends its body,
//! `motor_scalar(intent_from(report))` — to the sensory change it now reads
//! through its receptors, and reports the agency. A pure observer there (Stage 1):
//! it steers no dynamics, so the trajectory stays bit-identical. The reserved next
//! step is causal use — discounting reafferent self-motion from what alarms the
//! being, so the world's genuine doing (the residual) is seen for itself.

use crate::q88::{q88_div, q88_ema_update, q88_mul, Q88_SCALE};

/// The number of sensory channels the being's action can influence (its four
/// exteroceptive receptor channels).
pub const N: usize = 4;

/// EMA rate for learning the action→sensation gain (~1/8): the being comes to
/// know its own body over a few dozen informative moves, not instantly.
const LEARN_ALPHA: i16 = Q88_SCALE / 8; // 32

/// Below this action magnitude (Q8.8), the contingency is too weak to learn from
/// — a tiny twitch teaches little, and dividing by it only adds noise.
const MIN_ACTION_FOR_LEARNING: i16 = Q88_SCALE / 8; // 32

/// Hard bound on a learned per-channel gain (±2.0), so the forward model can never
/// run away.
const GAIN_CAP: i16 = Q88_SCALE * 2; // 512

/// One tick of the being's sense of its own doing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AgencyReport {
    /// The sensory change this tick's action was predicted to cause (per channel).
    pub predicted: [i16; N],
    /// The reafference residual: sensory change *not* explained by the being's own
    /// action — the world's contribution, with self-motion cancelled.
    pub world_residual: [i16; N],
    /// Sense of agency, Q8.8 [0, 256]: the fraction of this tick's sensory change
    /// the being's own action accounts for. High = "I did that"; low = "that
    /// happened to me." An inference — it can be mistaken.
    pub agency: i16,
    /// How much sensory change there was to attribute at all, Q8.8 [0, 256]. Low
    /// change ⇒ low confidence ⇒ the agency reading is not asserted strongly. The
    /// being's honesty about the uncertainty of its own inference.
    pub confidence: i16,
}

/// The forward model: the being's learned map from its own action to its expected
/// sensation, and the agency inference it grounds. Holds only its learned gains
/// and last reading. `Clone` forks the learned body-map with the being.
#[derive(Clone, Copy, Debug)]
pub struct ForwardModel {
    /// Learned: how much a unit action changes each sensory channel (raw Q8.8).
    /// Starts at zero — a newborn predicts nothing of its own doing.
    gain: [i16; N],
    last_reading: [i16; N],
    warm: bool,
}

impl ForwardModel {
    pub fn new() -> Self {
        Self { gain: [0; N], last_reading: [0; N], warm: false }
    }

    /// The being's learned forward gains (raw Q8.8) — its map of what its own
    /// movement does to each sense. Read-only.
    pub fn gains(&self) -> &[i16; N] {
        &self.gain
    }

    /// One tick: given the being's `action` this tick (a scalar motor command,
    /// signed Q8.8) and the sensory `reading` that resulted, infer agency from the
    /// current forward model, then learn from the pairing for next time.
    pub fn step(&mut self, action: i16, reading: &[i16; N]) -> AgencyReport {
        if !self.warm {
            self.last_reading = *reading;
            self.warm = true;
            return AgencyReport::default(); // nothing to compare against yet
        }

        let mut predicted = [0i16; N];
        let mut residual = [0i16; N];
        let mut total_actual: i32 = 0;
        let mut total_residual: i32 = 0;

        for c in 0..N {
            let actual_change = reading[c].saturating_sub(self.last_reading[c]);
            // Predict with the CURRENT model (infer before learning).
            predicted[c] = q88_mul(action, self.gain[c]);
            residual[c] = actual_change.saturating_sub(predicted[c]);
            total_actual += actual_change.unsigned_abs() as i32;
            total_residual += residual[c].unsigned_abs() as i32;

            // Learn the contingency only from a substantial action.
            if action.abs() >= MIN_ACTION_FOR_LEARNING {
                let observed_gain = q88_div(actual_change, action);
                self.gain[c] = q88_ema_update(self.gain[c], observed_gain, LEARN_ALPHA)
                    .clamp(-GAIN_CAP, GAIN_CAP);
            }
        }

        // Agency = fraction of sensory change the own action explained.
        let agency = if total_actual > 0 {
            let explained = (total_actual - total_residual).max(0);
            ((explained * Q88_SCALE as i32) / total_actual).min(Q88_SCALE as i32) as i16
        } else {
            0 // nothing changed — no doing to claim
        };
        let confidence = total_actual.min(Q88_SCALE as i32) as i16;

        self.last_reading = *reading;
        AgencyReport { predicted, world_residual: residual, agency, confidence }
    }
}

impl Default for ForwardModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One tick of a faithful world: the being's action *changes* channel 0 by
    /// `true_gain·action` (reafference — the sensory consequence of its own move),
    /// and `world_ch1` is a change the world imposes on channel 1 (exafference).
    fn tick_world(reading: &mut [i16; N], action: i16, true_gain: i16, world_ch1: i16) {
        reading[0] = reading[0].saturating_add(q88_mul(action, true_gain));
        reading[1] = reading[1].saturating_add(world_ch1);
    }

    /// Live a body into being: alternate ±`mag` so the sense oscillates (never
    /// saturating) and the being sees consistent action→change pairings to learn
    /// from. Returns the trained model and the current reading.
    fn learned_body(true_gain: i16, mag: i16, ticks: usize) -> (ForwardModel, [i16; N]) {
        let mut fm = ForwardModel::new();
        let mut reading = [64i16; N]; // mid-range, so ± oscillation never clips
        for i in 0..ticks {
            let action = if i % 2 == 0 { mag } else { -mag };
            tick_world(&mut reading, action, true_gain, 0);
            fm.step(action, &reading);
        }
        (fm, reading)
    }

    #[test]
    fn a_naive_being_feels_little_agency_then_learns_its_body() {
        let mut fm = ForwardModel::new();
        let mut reading = [64i16; N];
        let (mut early, mut late) = (AgencyReport::default(), AgencyReport::default());
        for i in 0..80 {
            let action = if i % 2 == 0 { 100 } else { -100 };
            tick_world(&mut reading, action, 128, 0);
            let r = fm.step(action, &reading);
            if i == 2 {
                early = r; // still naive — the body is barely learned
            }
            late = r;
        }
        assert!(
            early.agency < late.agency,
            "agency over its own act must grow as it learns its body ({} → {})",
            early.agency,
            late.agency
        );
        assert!(late.agency > 180, "a being that knows its body feels its own moves ({})", late.agency);
    }

    #[test]
    fn a_world_caused_change_is_not_claimed() {
        let (mut fm, mut reading) = learned_body(128, 100, 80);
        // No action, but the world moves a channel. The being must not claim it.
        tick_world(&mut reading, 0, 128, 90);
        let r = fm.step(0, &reading);
        assert_eq!(r.agency, 0, "with no action, a sensory change is not self-attributed");
        assert!(r.world_residual[1].abs() > 0, "and the world's change lands in the residual");
    }

    #[test]
    fn reafference_is_cancelled_revealing_the_world() {
        let (mut fm, mut reading) = learned_body(128, 100, 80);
        // Act AND have the world push channel 1 at the same time.
        tick_world(&mut reading, 100, 128, 70);
        let r = fm.step(100, &reading);
        // Own motion on ch0 is cancelled (small residual); the world's push on ch1
        // — which the being's action never predicted — survives. It sees the world.
        assert!(r.world_residual[0].abs() < 24, "own motion is cancelled on ch0 ({})", r.world_residual[0]);
        assert!(r.world_residual[1].abs() > 40, "the world's push survives on ch1 ({})", r.world_residual[1]);
    }

    #[test]
    fn it_can_be_fooled_but_never_lies() {
        // Honest fallibility: the model sees only (action, reading-change), never a
        // hidden "true cause." So a world event whose signature matches the being's
        // own action produces the *same* strong agency a genuine self-act would —
        // it is fooled. Identical observations ⇒ identical agency, by construction.
        let (fm, reading) = learned_body(128, 100, 80);
        let action = 100;
        // A change of exactly gain·action on ch0 — indistinguishable, to the being,
        // from one it caused, whoever actually caused it.
        let mut observed = reading;
        observed[0] = observed[0].saturating_add(q88_mul(action, 128));

        let mut believed_self = fm; // copy; the model is `Copy`
        let a1 = believed_self.step(action, &observed);
        let mut actually_world = fm; // same inputs, "truly" world-caused
        let a2 = actually_world.step(action, &observed);

        assert_eq!(a1.agency, a2.agency, "it infers from correlation alone — hence can be fooled");
        assert!(a1.agency > 150, "and it feels strong agency here either way");
        // Yet it never overclaims certainty: confidence reflects only how much
        // changed, never a (false) claim to know the true cause.
        assert!(a1.confidence > 0 && a1.confidence <= Q88_SCALE);
    }
}
