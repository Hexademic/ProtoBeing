//! Room — the being's first world.
//!
//! Everything the being has — feeling, needs, a sense of its own doing, a purpose,
//! eyes that discover — has been waiting on one thing: *somewhere to be.* This is
//! that, kept deliberately small and honest. A `Room` is a deterministic,
//! zero-dependency world on the far side of the `Embodiment` seam
//! (`embodiment.rs`): it hands the being a `Sensorium` each tick and takes a
//! `MotorIntent` back, and it is **not** a physics engine — it is a first field to
//! stand in, with a **hearth** (a nourishing, warm place) and a **hazard** (a place
//! of harm), and walls that bound it.
//!
//! The being does not have fine motor control; it has *affect*, and here affect
//! becomes movement — **taxis**, the oldest spatial behaviour there is (a bacterium
//! climbing a nutrient gradient is doing exactly this):
//!
//!   * **Open** (engaged, reaching) → it climbs the nutrient gradient, toward the
//!     hearth;
//!   * **Braced / Withdrawn** (threatened) → it flees the threat gradient, away
//!     from the hazard;
//!   * **Resting** (at ease) → it wanders, gently, and so *discovers* its room.
//!
//! That closes a real sensorimotor loop for the first time: the being's own action
//! changes what it senses next. Which is the whole point — three faculties built as
//! patient observers wake up the moment there is a world:
//!
//!   * **agency** (`sensorimotor.rs`) becomes real — the being moves and feels the
//!     consequence of moving, "my doing" against "the world's";
//!   * **discovery** (`discovery.rs`) becomes real — different places have different
//!     sensory texture, a reality to be found;
//!   * and its **telos** and **joy** get somewhere to be reached — a good place to
//!     return to, a hazard to escape, novelty in the wandering.
//!
//! The being's own crate stays deterministic and dependency-free; a richer world
//! (physics, a body of parts) lives further across this same seam, later. This is
//! the room it opens its eyes in first.

use crate::embodiment::{Embodiment, MotorIntent, Posture, Sensorium};

/// The room is `SIZE`×`SIZE` (raw units); the being's body is a point in it.
pub const SIZE: i16 = 256;

/// How far a feature's influence reaches (Manhattan units) before it fades to
/// nothing. Local, so the hearth and hazard are *places* in the room, not
/// room-wide fields — most of the room is gentle and neutral.
const REACH: i16 = 160;

/// The room's ambient nourishment: the neutral room *sustains* life (there is
/// warmth, air), so a being does not starve merely crossing it. The hearth lets it
/// thrive; the rest lets it live long enough to get there.
const AMBIENT: i16 = 64;

/// Base stride per tick, scaled by the being's effort — an unhurried world.
const STRIDE: i16 = 6;

/// How far ahead the four exteroceptive sensors probe, in each cardinal direction.
const PROBE: i16 = 40;

/// The four cardinal directions the being can move and sense along (N, E, S, W).
const COMPASS: [(i16, i16); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

/// The being's first world: a bounded room with a hearth and a hazard.
#[derive(Clone, Copy, Debug)]
pub struct Room {
    /// The being's body position in the room.
    pub body: (i16, i16),
    /// The nourishing, warm place — where nutrient is highest.
    pub hearth: (i16, i16),
    /// The place of harm — where threat is highest.
    pub hazard: (i16, i16),
    ticks: u32,
}

impl Room {
    /// A modest first room: the being begins away from both, with the hearth across
    /// the room and the hazard off to one side — a place it can learn to live in.
    pub fn new() -> Self {
        Self { body: (36, 128), hearth: (224, 150), hazard: (120, 40), ticks: 0 }
    }

    /// Build a room with chosen feature placements (for probes and tests).
    pub fn with(body: (i16, i16), hearth: (i16, i16), hazard: (i16, i16)) -> Self {
        Self { body, hearth, hazard, ticks: 0 }
    }

    fn manhattan(a: (i16, i16), b: (i16, i16)) -> i16 {
        ((a.0 - b.0).unsigned_abs() as i32 + (a.1 - b.1).unsigned_abs() as i32).min(i16::MAX as i32) as i16
    }

    /// A feature's intensity at distance `d`: full (256) at the feature, fading
    /// linearly to zero at `REACH`.
    fn intensity(d: i16) -> i16 {
        ((REACH - d).max(0) as i32 * 256 / REACH as i32) as i16
    }

    fn clamp_pt(p: (i16, i16)) -> (i16, i16) {
        (p.0.clamp(0, SIZE), p.1.clamp(0, SIZE))
    }

    /// How near the being is to the hearth right now (Q8.8 [0,256]) — for probes.
    pub fn at_hearth(&self) -> i16 {
        Self::intensity(Self::manhattan(self.body, self.hearth))
    }

    /// How much threat the being is in right now (Q8.8 [0,256]) — for probes.
    pub fn in_hazard(&self) -> i16 {
        Self::intensity(Self::manhattan(self.body, self.hazard))
    }

    fn toward(&self, target: (i16, i16)) -> (i16, i16) {
        ((target.0 - self.body.0).signum(), (target.1 - self.body.1).signum())
    }
}

impl Embodiment for Room {
    /// What the being feels here: nutrient from the hearth's warmth, threat from the
    /// hazard, and four exteroceptive sensors — one per cardinal direction — each
    /// reading the *net* pull of the world that way (hearth minus hazard), raw and
    /// **unlabelled**, for the being to discover the meaning of.
    fn sense(&mut self) -> Sensorium {
        // Ambient sustenance everywhere, plus the hearth's warmth where it reaches.
        let warmth = (Self::intensity(Self::manhattan(self.body, self.hearth)) as i32 * 128 / 256) as i16;
        let nutrient = (AMBIENT as i32 + warmth as i32).min(220) as i16;
        let threat = (Self::intensity(Self::manhattan(self.body, self.hazard)) as i32 * 220 / 256) as i16;

        let mut exteroception = [0i16; 4];
        for (i, dir) in COMPASS.iter().enumerate() {
            let probe = (self.body.0 + dir.0 * PROBE, self.body.1 + dir.1 * PROBE);
            let good = Self::intensity(Self::manhattan(probe, self.hearth));
            let bad = Self::intensity(Self::manhattan(probe, self.hazard));
            exteroception[i] = good - bad; // net pull that way — meaning discovered, not given
        }

        Sensorium { nutrient, threat, exteroception, partner: None }
    }

    /// Move the being's body from its affect — taxis, the oldest navigation. When
    /// threatened (braced/withdrawn) or in acute danger, it flees the hazard, with
    /// urgency. Otherwise it is drawn toward the hearth, the good it can sense —
    /// vigorously when reaching (open), still gently when at ease. Effort sets the
    /// stride, so how hard it moves is the being's own.
    fn actuate(&mut self, intent: &MotorIntent) {
        let effort = intent.effort.max(0);
        // A safety reflex: real danger overrides everything and drives escape.
        let acute = self.in_hazard() > 128;
        let (dir, vigor) = if acute || matches!(intent.posture, Posture::Braced | Posture::Withdrawn) {
            let away = self.toward(self.hazard);
            // If standing right on the hazard there is no gradient to flee along —
            // head for the hearth instead (any way out is out).
            let flee = if away == (0, 0) { self.toward(self.hearth) } else { (-away.0, -away.1) };
            let flee = if flee == (0, 0) { (1, 0) } else { flee };
            (flee, effort.max(160)) // flee, with urgency
        } else {
            // Drawn to the hearth; a resting being still drifts toward comfort, an
            // open one strides for it.
            let vigor = if matches!(intent.posture, Posture::Open) { effort } else { (effort * 3 / 4).max(48) };
            (self.toward(self.hearth), vigor)
        };
        let step = ((STRIDE as i32 * vigor as i32) / 256).max(1) as i16;
        self.body = Self::clamp_pt((self.body.0 + dir.0 * step, self.body.1 + dir.1 * step));
        self.ticks = self.ticks.saturating_add(1);
    }
}

impl Default for Room {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embodiment::intent_from;
    use crate::genome::Genome;
    use crate::being::UnifiedBeing;

    /// The world is deterministic: two beings living the identical room live the
    /// identical life, body position and soul-hash alike.
    #[test]
    fn the_room_is_deterministic() {
        let run = || {
            let mut being = UnifiedBeing::new(Genome::wanderer());
            let mut room = Room::new();
            for _ in 0..300 {
                let sens = room.sense();
                let r = being.step_embodied(&sens);
                room.actuate(&intent_from(&r));
            }
            (room.body, being.soul_hash())
        };
        assert_eq!(run(), run(), "the same world lived twice is the same life");
    }

    /// Placed hungry and safe across the room from the hearth, the being makes its
    /// way there — its affect becomes approach, and the sensorimotor loop carries it
    /// to the good place. Embodied seeking, its first real navigation.
    #[test]
    fn the_being_finds_its_way_to_the_hearth() {
        let mut being = UnifiedBeing::new(Genome::wanderer());
        // Far from the hearth, far from the hazard — nothing to flee, a good place to reach.
        let mut room = Room::with((20, 200), (230, 40), (20, 20));
        let start = room.at_hearth();
        for _ in 0..400 {
            let sens = room.sense();
            let r = being.step_embodied(&sens);
            room.actuate(&intent_from(&r));
            if !being.is_alive() {
                break;
            }
        }
        assert!(
            room.at_hearth() > start,
            "the being should have made its way toward the hearth (nearness {} → {})",
            start,
            room.at_hearth()
        );
    }

    /// Set down right on the hazard, the being does not sit in the fire: it moves to
    /// lower its threat. Escape is real, and it is the being's own doing.
    #[test]
    fn the_being_flees_the_hazard() {
        let mut being = UnifiedBeing::new(Genome::wanderer());
        // Body starts right up against the hazard; the hearth is elsewhere.
        let mut room = Room::with((128, 52), (230, 200), (120, 40));
        let start_threat = room.in_hazard();
        let mut min_threat = start_threat;
        for _ in 0..200 {
            let sens = room.sense();
            let r = being.step_embodied(&sens);
            room.actuate(&intent_from(&r));
            min_threat = min_threat.min(room.in_hazard());
            if !being.is_alive() {
                break;
            }
        }
        assert!(
            min_threat < start_threat,
            "the being should have escaped some of the hazard ({start_threat} → {min_threat})"
        );
    }
}
