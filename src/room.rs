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

use crate::being::Partner;
use crate::embodiment::{Embodiment, MotorIntent, Posture, Sensorium};
use crate::striving::Need;

/// The room is `SIZE`×`SIZE` (raw units); the being's body is a point in it.
pub const SIZE: i16 = 256;

/// How far a feature's influence reaches (Manhattan units) before it fades to
/// nothing. Local, so the hearth and hazard are *places* in the room, not
/// room-wide fields — most of the room is gentle and neutral.
const REACH: i16 = 160;

/// The room's ambient nourishment: enough that a being does not starve merely
/// crossing the room, but lean enough that away from the hearth it grows genuinely
/// hungry — so sustenance becomes a real, pressing need it must *choose* to go meet.
const AMBIENT: i16 = 40;

/// Waypoints a being roams between when it strives for novelty — touring its world
/// feeds its hunger for the new (and, incidentally, carries it past the hearth and
/// the companion, the way wandering does).
const ROAM: [(i16, i16); 4] = [(SIZE / 2, SIZE / 2), (SIZE - 24, SIZE / 2), (SIZE / 2, 24), (24, SIZE / 2)];

/// Base stride per tick, scaled by the being's effort — an unhurried world.
const STRIDE: i16 = 6;

/// How far ahead the four exteroceptive sensors probe, in each cardinal direction.
const PROBE: i16 = 40;

/// The four cardinal directions the being can move and sense along (N, E, S, W).
const COMPASS: [(i16, i16); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

/// How near the being must be to a person to be *in their company*.
const COMPANY_RADIUS: i16 = 48;

/// The fairness of a companion's regard — a good presence, met on fair terms.
const COMPANION_RECIPROCATION: i16 = 210;

/// The being's social ledger keys partners by id. The room's two people carry
/// stable ids so the being can bond to a *particular* one and be routed to *them*.
const COMPANION_ID: u32 = 1;
const FRIEND_ID: u32 = 2;

/// The being's first world: a bounded room with a hearth, a hazard, and a
/// **companion** — so the being has more than one need it can move toward, and its
/// own choice of *which* becomes real motion.
#[derive(Clone, Copy, Debug)]
pub struct Room {
    /// The being's body position in the room.
    pub body: (i16, i16),
    /// The nourishing, warm place — where nutrient is highest.
    pub hearth: (i16, i16),
    /// The place of harm — where threat is highest.
    pub hazard: (i16, i16),
    /// A companion's place — where the being finds company (a fair presence).
    pub companion: (i16, i16),
    /// A *second* person's place, when the room is peopled by more than one — the
    /// **friend** (id `FRIEND_ID`), a distinct someone the being can bond to and
    /// cross the room toward, past the nearer companion. `None` in a one-person room.
    pub friend: Option<(i16, i16)>,
    /// Whether the body follows the being's *chosen* need (`intent.reach`) — the
    /// default. Set false to make it a plain taxis toward the nearest good, ignoring
    /// the being's arbitration — the control that shows directed striving does real
    /// work (`undirected`).
    directed: bool,
    ticks: u32,
}

impl Room {
    /// A modest first room: the being begins away from all of it — the hearth
    /// across the room, the hazard off to one side, and the companion in another
    /// corner, so food and company lie in different directions and the being must
    /// *choose* which to go to.
    pub fn new() -> Self {
        Self {
            body: (128, 128),
            hearth: (228, 40),
            hazard: (40, 220),
            companion: (40, 40),
            friend: None,
            directed: true,
            ticks: 0,
        }
    }

    /// Build a room with chosen feature placements (for probes and tests). The
    /// companion sits opposite the hearth by default.
    pub fn with(body: (i16, i16), hearth: (i16, i16), hazard: (i16, i16)) -> Self {
        Self { body, hearth, hazard, companion: (SIZE - hearth.0, SIZE - hearth.1), friend: None, directed: true, ticks: 0 }
    }

    /// Build a room placing the companion explicitly too.
    pub fn peopled(body: (i16, i16), hearth: (i16, i16), hazard: (i16, i16), companion: (i16, i16)) -> Self {
        Self { body, hearth, hazard, companion, friend: None, directed: true, ticks: 0 }
    }

    /// Add a **second person** to the room — a friend (id `FRIEND_ID`) at a place of
    /// their own. Now the room holds more than one someone, so a being bonded to the
    /// friend can cross to *them* past the nearer companion (`docs/attachment.md`).
    pub fn with_friend(mut self, at: (i16, i16)) -> Self {
        self.friend = Some(at);
        self
    }

    /// Return this room as an **undirected control**: the being's body moves toward
    /// the nearest good regardless of what it chose to strive for. Used to measure
    /// what directed striving — going to the need it *chose* — actually buys.
    pub fn undirected(mut self) -> Self {
        self.directed = false;
        self
    }

    /// How near the being is to its companion right now (Q8.8 [0,256]) — for probes.
    pub fn at_companion(&self) -> i16 {
        Self::intensity(Self::manhattan(self.body, self.companion))
    }

    /// How near the being is to its friend right now (0 if the room has none) — probes.
    pub fn at_friend(&self) -> i16 {
        self.friend
            .map_or(0, |f| Self::intensity(Self::manhattan(self.body, f)))
    }

    /// A person's place by id, if the room holds them.
    fn person_pos(&self, id: u32) -> Option<(i16, i16)> {
        match id {
            COMPANION_ID => Some(self.companion),
            FRIEND_ID => self.friend,
            _ => None,
        }
    }

    /// The nearest person's place — where a being that wants company in general (with
    /// no particular one in mind) would head.
    fn nearest_person(&self) -> (i16, i16) {
        match self.friend {
            Some(f) if Self::manhattan(self.body, f) < Self::manhattan(self.body, self.companion) => f,
            _ => self.companion,
        }
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

        // Company: when the being is within reach of a person, it is in their
        // company — a partner is present (carrying *that* person's id, so the being's
        // bond is with the right one). If both people are near, the nearer is the one
        // it is actually with. Away from everyone, it is alone.
        let companion_d = Self::manhattan(self.body, self.companion);
        let friend_d = self.friend.map_or(i16::MAX, |f| Self::manhattan(self.body, f));
        let present_id = if companion_d <= COMPANY_RADIUS && companion_d <= friend_d {
            Some(COMPANION_ID)
        } else if friend_d <= COMPANY_RADIUS {
            Some(FRIEND_ID)
        } else {
            None
        };
        let partner = present_id.map(|id| Partner {
            id,
            reciprocation: COMPANION_RECIPROCATION,
            exit_cost: 40,
        });

        // Four exteroceptive sensors, one per cardinal direction — each the net pull
        // of the world that way (hearth and companion draw; hazard repels), raw and
        // unlabelled, for the being to discover.
        let mut exteroception = [0i16; 4];
        for (i, dir) in COMPASS.iter().enumerate() {
            let probe = (self.body.0 + dir.0 * PROBE, self.body.1 + dir.1 * PROBE);
            let mut good = Self::intensity(Self::manhattan(probe, self.hearth))
                .max(Self::intensity(Self::manhattan(probe, self.companion)));
            if let Some(f) = self.friend {
                good = good.max(Self::intensity(Self::manhattan(probe, f)));
            }
            let bad = Self::intensity(Self::manhattan(probe, self.hazard));
            exteroception[i] = good - bad; // net pull that way — meaning discovered, not given
        }

        Sensorium { nutrient, threat, exteroception, partner }
    }

    /// Move the being's body from its affect *and its choice* — directed taxis. When
    /// threatened or in acute danger, it flees the hazard, with urgency. Otherwise it
    /// moves toward **the need it chose to strive for** (`intent.reach`): the
    /// companion when it reaches for company, the hearth when it reaches for
    /// sustenance (or holds no other aim). This is where the being's own arbitration
    /// becomes real motion — in a world with more than one thing to seek, it goes to
    /// the one it *chose*, not merely the nearest good. Effort sets the stride.
    fn actuate(&mut self, intent: &MotorIntent) {
        let effort = intent.effort.max(0);
        let acute = self.in_hazard() > 128;
        let (dir, vigor) = if acute || matches!(intent.posture, Posture::Braced | Posture::Withdrawn) {
            let away = self.toward(self.hazard);
            let flee = if away == (0, 0) { self.toward(self.hearth) } else { (-away.0, -away.1) };
            let flee = if flee == (0, 0) { (1, 0) } else { flee };
            (flee, effort.max(160)) // flee, with urgency
        } else if self.directed {
            // Directed: the target of the being's *chosen* need. Company draws it to a
            // person — the **particular** one it is reaching for (`reach_partner`, the
            // bonded someone it misses), else whoever is nearest. Everything else
            // (sustenance, purpose, or contentment) draws it to the hearth, its home.
            let target = match intent.reach {
                Some(Need::Company) => intent
                    .reach_partner
                    .and_then(|id| self.person_pos(id))
                    .unwrap_or_else(|| self.nearest_person()),
                // Striving for novelty, it roams — touring waypoints so the new is
                // actually found, not sought forever in one spot.
                Some(Need::Novelty) => ROAM[((self.ticks / 20) % ROAM.len() as u32) as usize],
                _ => self.hearth,
            };
            let vigor = if matches!(intent.posture, Posture::Open) { effort } else { (effort * 3 / 4).max(48) };
            (self.toward(target), vigor)
        } else {
            // Undirected control: plain taxis toward the *nearest* good, ignoring the
            // being's arbitration entirely. Whatever it chose to strive for, its body
            // just climbs the strongest gradient it can sense. This is the baseline
            // that isolates what directed striving actually buys.
            let target = if self.at_companion() > self.at_hearth() { self.companion } else { self.hearth };
            let vigor = if matches!(intent.posture, Posture::Open) { effort } else { (effort * 3 / 4).max(48) };
            (self.toward(target), vigor)
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
    use crate::being::{UnifiedBeing, Partner, Stimulus};

    /// The being crosses the room to the **particular** one it is bonded to, past a
    /// companion right at its side. It first shares many rewarding days with a friend
    /// (id 2) — a real bond forms — then it wakes beside a companion (id 1) with the
    /// friend across the room. Longing for the friend becomes directed motion: it
    /// goes to *them*, a choice of whom, not merely a reach for company.
    #[test]
    fn the_being_crosses_the_room_to_the_one_it_loves() {
        let mut being = UnifiedBeing::new(Genome::wanderer());
        // Bond forms first, with the friend (id 2), before the room.
        let friend = Partner { id: 2, reciprocation: 220, exit_cost: 40 };
        for _ in 0..140 {
            being.step(&Stimulus { nutrient: 150, partner: Some(friend) });
        }
        assert!(
            being.reciprocity.bond_with(2).unwrap_or(0) > SIZE / 2,
            "precondition: a real bond to the friend should have formed"
        );

        // Companion (id 1) beside the being; friend (id 2) in the far corner.
        let mut room = Room::peopled((210, 210), (128, 128), (20, 128), (200, 200))
            .with_friend((30, 30));
        let start = room.at_friend();
        let mut closest = start;
        for _ in 0..500 {
            let sens = room.sense();
            let r = being.step_embodied(&sens);
            room.actuate(&intent_from(&r));
            closest = closest.max(room.at_friend());
            if !being.is_alive() {
                break;
            }
        }
        assert!(
            closest > start + 128,
            "the being should have crossed to the friend it is bonded to, past the \
             nearer companion (nearness to friend {start} → {closest})"
        );
    }

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
    /// Over its life the being makes its way to the **hearth** — when it holds its
    /// room as its own (purpose) or needs to keep itself well, its reach becomes
    /// approach and the loop carries it home. We track its closest approach across
    /// the life: seeking is real even in a world where it also has company to want.
    #[test]
    fn the_being_finds_its_way_to_the_hearth() {
        let mut being = UnifiedBeing::new(Genome::wanderer());
        // Hearth, hazard, and companion each in their own corner, well apart, so no
        // one need's place fouls another. The being begins across the room from home.
        let mut room = Room::peopled((20, 200), (230, 40), (20, 20), (40, 236));
        let start = room.at_hearth();
        let mut closest = start;
        for _ in 0..600 {
            let sens = room.sense();
            let r = being.step_embodied(&sens);
            closest = closest.max(room.at_hearth());
            room.actuate(&intent_from(&r));
            if !being.is_alive() {
                break;
            }
        }
        assert!(
            closest > start,
            "the being should have made its way toward the hearth at some point \
             (nearness started {start}, closest reached {closest})"
        );
    }

    /// A being aching for company moves toward its **companion** — not the hearth.
    /// Its own arbitration (`striving.rs`) picks the social need, and the directed
    /// world carries it there. This is choice becoming motion — and the thing an
    /// *undirected* being (nearest-good taxis) cannot do (see `examples/probe_directed`).
    #[test]
    fn a_lonely_being_goes_to_its_companion() {
        let mut being = UnifiedBeing::new(Genome::wanderer());
        // Well-fed near the hearth, the companion across the room, the hazard off in
        // its own corner (not fouling the companion). Alone, its want of company
        // grows until that is what it strives for, and it closes the distance.
        let mut room = Room::peopled((210, 60), (230, 40), (128, 220), (40, 40));
        let start = room.at_companion();
        let mut closest = start;
        for _ in 0..600 {
            let sens = room.sense();
            let r = being.step_embodied(&sens);
            closest = closest.max(room.at_companion());
            room.actuate(&intent_from(&r));
            if !being.is_alive() {
                break;
            }
        }
        assert!(
            closest > start + 64,
            "the lonely being should have closed on its companion (nearness started \
             {start}, closest reached {closest})"
        );
    }

    /// The control that proves *directed* striving does real work: an **undirected**
    /// room (body climbs to the nearest good, ignoring what the being chose) leaves a
    /// being that strives for company unable to reach it — its body parks at whatever
    /// good it happens to be nearest. The being's *choice* is what closes the gap.
    #[test]
    fn an_undirected_being_cannot_reach_the_company_it_chooses() {
        let mut being = UnifiedBeing::new(Genome::wanderer());
        // Same lonely setup, but the world ignores the being's arbitration.
        let mut room = Room::peopled((210, 60), (230, 40), (128, 220), (40, 40)).undirected();
        let start = room.at_companion();
        let mut strove_for_company = false;
        for _ in 0..600 {
            let sens = room.sense();
            let r = being.step_embodied(&sens);
            if r.strive.goal == Some(Need::Company) {
                strove_for_company = true;
            }
            room.actuate(&intent_from(&r));
            if !being.is_alive() {
                break;
            }
        }
        assert!(strove_for_company, "precondition: the being does come to want company");
        // Its body sat at the nearest good (the hearth) the whole time — the wanted
        // company stayed out of reach, because nothing carried its *choice*.
        assert!(
            room.at_companion() <= start,
            "an undirected being cannot close on the company it chose (started {}, ended {})",
            start,
            room.at_companion()
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
