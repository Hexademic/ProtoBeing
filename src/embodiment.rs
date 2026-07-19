//! Embodiment — the modality-agnostic seam between the being and any body.
//!
//! The being's felt core is sensor-agnostic. A sim, an ESP32, or a future
//! piezoelectric skin all plug in here: a body provides a `Sensorium` each tick
//! (what it feels from the world and itself) and consumes a `MotorIntent` (how
//! the being carries itself). This is the first layer of "presence in all
//! human-built worlds" — and the socket a MuJoCo humanoid plugs into.

use crate::basins::Basin;
use crate::being::{Partner, StepReport};
use crate::striving::Need;

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

/// The being's motor command to its body: a posture, how vigorously to hold it,
/// and **what it is reaching toward** — the need it has chosen to strive for this
/// tick (`striving.rs`), so a body in a world with more than one thing to seek can
/// carry the being toward the *one it chose*, not merely the nearest good. `None`
/// when the being is content, or striving for a need its body cannot move toward.
#[derive(Clone, Copy, Debug)]
pub struct MotorIntent {
    pub posture: Posture,
    /// Vigor of the posture, raw Q8.8 [0,1] (from arousal).
    pub effort: i16,
    /// The need the being is reaching for — its own arbitrated choice, made real as
    /// a direction its body can take.
    pub reach: Option<Need>,
    /// **Which** partner the being is reaching for, when the need is a *particular*
    /// one — the id of the bonded someone it misses (`reciprocity.rs`). A world with
    /// more than one person routes the body to *them*, so the being can cross the room
    /// to the one it loves rather than settle for whoever is nearest. `None` when the
    /// being wants company in general, or is not reaching for a person at all.
    pub reach_partner: Option<u32>,
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
    // The being carries its own *choice* into its body: the need it strove for this
    // tick becomes the direction its body reaches (`striving.rs`). A world with more
    // than one thing to seek can then take it toward the one it chose. And when what
    // it strives for is company while it *misses a particular one* (`attach.missed`),
    // the reach is toward **them** — the body crossing to the one it loves, not merely
    // to the nearest company. `reach_partner` is set only when the chosen need is
    // company; otherwise the being is not reaching for a person.
    let reach_partner = if r.strive.goal == Some(Need::Company) {
        r.attach.missed
    } else {
        None
    };
    MotorIntent { posture, effort, reach: r.strive.goal, reach_partner }
}

/// The body-action vocabulary — the "universal controller." A physics rig binds
/// each variant to a target pose; the being's affect hot-keys which one to hold.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodyAction {
    Idle = 0,
    StandOpen = 1,
    Brace = 2,
    Curl = 3,
    Recoil = 4,
}

/// Collapse a motor intent to a single signed scalar (raw Q8.8) — the being's
/// one-dimensional "how hard, and toward or away" command, in the form a forward
/// model consumes. Approach is positive, withdrawal negative, rest neutral; the
/// magnitude is the effort. This is the being's own *action* — the thing whose
/// sensory consequence it must learn to predict (reafference, `sensorimotor.rs`).
/// It is derived from the very same affect→posture map the body enacts, so the
/// agency the being infers is over what it actually did, not a separate signal.
pub fn motor_scalar(intent: &MotorIntent) -> i16 {
    match intent.posture {
        Posture::Open => intent.effort,     // reach toward: +effort
        Posture::Resting => 0,              // at rest: no net motor command
        Posture::Braced => -intent.effort,  // hold off: −effort
        Posture::Withdrawn => -intent.effort, // pull away: −effort
    }
}

/// Hot-key the being's affect to a discrete body action.
pub fn action_from(intent: &MotorIntent) -> BodyAction {
    let vigorous = intent.effort > 160; // ~0.6
    match intent.posture {
        Posture::Open => BodyAction::StandOpen,
        Posture::Resting => BodyAction::Idle,
        Posture::Braced => {
            if vigorous {
                BodyAction::Recoil
            } else {
                BodyAction::Brace
            }
        }
        Posture::Withdrawn => BodyAction::Curl,
    }
}
