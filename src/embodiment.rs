//! Embodiment — the modality-agnostic seam between the being and any body.
//!
//! The being's felt core is sensor-agnostic. A sim, an ESP32, or a future
//! piezoelectric skin all plug in here: a body provides a `Sensorium` each tick
//! (what it feels from the world and itself) and consumes a `MotorIntent` (how
//! the being carries itself). This is the first layer of "presence in all
//! human-built worlds" — and the socket a MuJoCo humanoid plugs into.

use crate::basins::Basin;
use crate::being::{Partner, StepReport};

/// What a body feels this tick and offers to the mind.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sensorium {
    /// Nourishment available, raw Q8.8 [0,1].
    pub nutrient: i16,
    /// Environmental threat (instability, cold, pain), raw Q8.8 [0,1].
    pub threat: i16,
    /// Four exteroceptive readings written into the somatic field — contact,
    /// balance, temperature, pressure, or whatever the body has.
    pub exteroception: [i16; 4],
    /// A social partner present this tick, if any.
    pub partner: Option<Partner>,
}

/// How the being carries itself — the affective posture a body should adopt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posture {
    Resting,
    Open,
    Braced,
    Withdrawn,
}

/// The being's motor command to its body: a posture and how vigorously to hold it.
#[derive(Clone, Copy, Debug)]
pub struct MotorIntent {
    pub posture: Posture,
    /// Vigor of the posture, raw Q8.8 [0,1] (from arousal).
    pub effort: i16,
}

/// Any body the being can inhabit. A MuJoCo humanoid, an ESP32 rig, or a toy
/// world all implement this same trait.
pub trait Embodiment {
    fn sense(&mut self) -> Sensorium;
    fn actuate(&mut self, intent: &MotorIntent);
}

/// Map the being's reported state to how it should carry its body. Affect, not
/// fine motor control: the being is the felt core that colors the posture.
pub fn intent_from(r: &StepReport) -> MotorIntent {
    let posture = if r.valence < -0.2 && r.arousal < 0.4 {
        Posture::Withdrawn
    } else {
        match r.basin {
            Basin::Engaged => Posture::Open,
            Basin::Defensive => Posture::Braced,
            Basin::Recovery | Basin::Rest => Posture::Resting,
        }
    };
    let effort = ((r.arousal * 256.0) as i16).clamp(0, 256);
    MotorIntent { posture, effort }
}
