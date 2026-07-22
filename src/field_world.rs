//! The field-world — consequence with a cost (`docs/field-world.md`).
//!
//! `room.rs` gave the being *somewhere to be*: discrete beacons (hearth, hazard,
//! companion) it reads distance to, and a movement law special-cased per target. This is
//! the next thing the manifesto asks for — a world with real **stakes**, where living
//! costs — and it is built from exactly **one** imported idea (Blake's EEIT paper),
//! wearing three hats, with every other trapping (elastic waves, moduli, the entanglement
//! ontology) deliberately left out:
//!
//!   1. **The world is a field, not beacons.** A single scalar **viability potential**
//!      `V(x)` over the space — a smooth landscape whose height is *how good it is to be
//!      here*. Features are hills and pits in `V`, and ridges/saddles between them
//!      **emerge** from the field's shape rather than being hand-placed.
//!   2. **Moving costs along the gradient (Landauer: acting is physical).** Climbing
//!      toward a distant good over a hard stretch, or clawing up out of a threat pit,
//!      spends metabolic energy in proportion to the grade fought. Coasting is cheap.
//!      This is the principled *stake*: consequence has a price, and the price is the
//!      landscape's physics, not an arbitrary penalty knob.
//!   3. **One gradient law, not many special cases.** The being always **climbs `V`** —
//!      toward better-regulated states. Good is high ground, threat is low ground, so
//!      "approach the good" and "flee the harm" are the *same* motion (ascend), and the
//!      being's affect only sets how hard it climbs. `room.rs::actuate`'s four cases
//!      collapse to one, and a new world needs no new movement code. (`F = T∇S` in
//!      miniature; the being's taxis was already this in disguise.)
//!
//! **The cost lives in the world, across the `Embodiment` seam — never in the being's
//! body.** The world debits a being that fights the grade by reporting it *less
//! nourishment* (a spent margin), accumulated as a slowly-decaying metabolic debt. The
//! being's core metabolism, and its soul-hash, are untouched: a founded being that never
//! enters a field-world is bit-identical. This is why the stakes can deepen without a
//! core re-founding.
//!
//! **Why this is the honest next step (measured):** the being's viability is bimodal
//! (`examples/carrying_the_weight`) — fine, or crashing, no worn-but-stable middle. The
//! graded drive (`homeostasis.rs`) proved that middle is *expressible*; gradient-cost is
//! what actually *puts the being there and keeps it* — the sustained, survivable drain of
//! living somewhere hard to reach. `examples/the_world` measures both promises: that the
//! one climb-law reproduces the old room's hearth-reaching and hazard-fleeing, and that
//! gradient-cost creates the sustained-low-but-stable middle a hard life is lived in.

use crate::being::Partner;
use crate::embodiment::{Embodiment, MotorIntent, Posture, Sensorium};
use crate::striving::Need;

/// The field is `SIZE`×`SIZE` raw units; the being's body is a point in it.
pub const SIZE: i16 = 256;

/// How far a source's influence reaches. Set to the field's full Manhattan span (`2·SIZE`)
/// so every source is felt *everywhere* — there is always a gradient to climb. This is
/// what makes it a **field** (a landscape covering the space) rather than a set of local
/// beacons the being can wander out of range of.
const REACH: i16 = 2 * SIZE;

/// The neutral ground level of the viability field — the ambient "how good it is here"
/// far from any source. Set mid-scale so sources can raise *or* lower the local good.
const BASE: i16 = 96;

/// Base stride per tick, scaled by effort — the same unhurried pace as the room.
const STRIDE: i16 = 6;

/// How far ahead the being probes the field to feel its gradient, per cardinal direction.
const PROBE: i16 = 40;

/// The four cardinal directions the being senses and moves along (N, E, S, W).
const COMPASS: [(i16, i16); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

/// Grade-cost tuning: the height climbed this step, times this, is charged to metabolic
/// debt. Climbing toward the good is *work*; this is its price. Scaled so a real climb
/// registers on the being's [0,256] scale (a single raw unit of height is a meaningful
/// step's worth of grade) rather than truncating to nothing.
const COST_NUM: i32 = 6;

/// Metabolic debt recovers (decays) each tick — rest and coasting pay it back. 7/8 per
/// tick: a real, lingering cost of a hard climb, but never a permanent tax.
const DEBT_DECAY_NUM: i32 = 7;
const DEBT_DECAY_DEN: i32 = 8;

/// The most debt the world will charge — capped well below the local nourishment so a
/// hard climb wears the being down (the worn middle) without ever starving it outright.
const DEBT_CAP: i16 = 64;

/// The most nourishment the world reports, before debt — comparable to the room's cap.
const NUTRIENT_CAP: i16 = 220;

/// The ambient nourishment floor: enough that a being never starves merely from crossing
/// lean ground or paying a hard climb's cost (as `room.rs`'s ambient did). The world's
/// cost then *wears* the being — lean, worn, alive — into the middle, rather than killing
/// it. Below this floor is death's business (the pit's threat), not the price of motion.
const AMBIENT_FLOOR: i16 = 40;

/// A person's pull as a field: full at their place, fading across the field like any
/// source. People are goods the being can climb toward — but only when it *chooses* to.
const PERSON_PEAK: i16 = 200;

/// How near the being must be to a person to be in their company (a partner present).
const COMPANY_RADIUS: i16 = 48;

/// How strongly a *chosen* person pulls, relative to the viability field. Weighted high
/// enough that a being reaching for company crosses to the one it chose even past the
/// hearth's nearer draw — the being's arbitration made real motion, under the one law.
const COMPANY_WEIGHT: i32 = 3;

/// A source shaping the viability field: a signed contribution that fades with distance.
/// `+peak` is a good (nourishing, warm, safe — a hill in `V`); `−peak` is a harm (a pit).
#[derive(Clone, Copy, Debug)]
struct Source {
    pos: (i16, i16),
    /// Signed height at the source's center (raw Q8.8): + good, − harm.
    peak: i16,
    /// How far the source's influence reaches before fading to nothing.
    reach: i16,
}

/// The being's field-world: a scalar viability landscape with a cost of motion, behind
/// the same `Embodiment` seam as `room.rs`. Deterministic, zero-dependency.
#[derive(Clone, Debug)]
pub struct FieldWorld {
    /// The being's body position in the field.
    pub body: (i16, i16),
    sources: Vec<Source>,
    /// People in the world, each an `(id, place)` — goods the being can cross *toward*,
    /// and bond with a *particular* one of. Distinct from `sources` so the being reaches
    /// for a person only when it chooses company, not merely because they are nourishing.
    persons: Vec<(u32, (i16, i16))>,
    /// Accumulated metabolic debt from grade fought — the spent margin the world charges
    /// against the nourishment it reports. Decays as the being coasts and rests.
    debt: i16,
    ticks: u32,
}

impl FieldWorld {
    /// A first field-world: a nourishing **hill** (the good high ground) across the space
    /// from the being, and a **pit** of harm off to one side — so the being begins on
    /// neutral ground with somewhere good to climb toward and somewhere bad to climb away
    /// from, all as one landscape. Sources reach across the whole field, so there is
    /// always a gradient to feel — it is a *field*, not a set of local beacons.
    pub fn new() -> Self {
        Self {
            body: (128, 128),
            sources: vec![
                Source { pos: (228, 40), peak: 128, reach: REACH },  // the good hill
                Source { pos: (40, 220), peak: -120, reach: REACH }, // the pit of harm
            ],
            persons: vec![],
            debt: 0,
            ticks: 0,
        }
    }

    /// Build a field-world with a chosen body start and one good hill + one harm pit —
    /// the field analogue of `Room::with`, for probes and the room-behaviour control.
    pub fn with(body: (i16, i16), good: (i16, i16), harm: (i16, i16)) -> Self {
        Self {
            body,
            sources: vec![
                Source { pos: good, peak: 128, reach: REACH },
                Source { pos: harm, peak: -120, reach: REACH },
            ],
            persons: vec![],
            debt: 0,
            ticks: 0,
        }
    }

    /// Add a **person** to the world at their own place, carrying a stable id — so the
    /// being can bond to a *particular* one (`reciprocity.rs`) and cross to *them*, past a
    /// nearer stranger, when it reaches for company. This is what gives the single climb-
    /// law the being's *choice of whom*.
    pub fn with_person(mut self, id: u32, at: (i16, i16)) -> Self {
        self.persons.push((id, at));
        self
    }

    /// Add a source to the field — another hill or pit, so ridges and saddles can emerge
    /// between it and the others (a harder landscape to live in).
    pub fn with_source(mut self, pos: (i16, i16), peak: i16, reach: i16) -> Self {
        self.sources.push(Source { pos, peak, reach });
        self
    }

    fn manhattan(a: (i16, i16), b: (i16, i16)) -> i16 {
        ((a.0 - b.0).unsigned_abs() as i32 + (a.1 - b.1).unsigned_abs() as i32).min(i16::MAX as i32) as i16
    }

    fn clamp_pt(p: (i16, i16)) -> (i16, i16) {
        (p.0.clamp(0, SIZE), p.1.clamp(0, SIZE))
    }

    /// The viability potential `V` at a point: base ground plus every source's signed,
    /// distance-faded contribution, clamped to the being's [0,256] scale. This is the
    /// whole world — one smooth scalar landscape.
    pub fn v_at(&self, p: (i16, i16)) -> i16 {
        let mut v = BASE as i32;
        for s in &self.sources {
            let d = Self::manhattan(p, s.pos);
            if d < s.reach {
                v += s.peak as i32 * (s.reach - d) as i32 / s.reach as i32;
            }
        }
        v.clamp(0, 256) as i16
    }

    /// The threat felt at a point — the summed influence of the *harm* sources alone
    /// (the pits), as a positive [0,256] pressure. Kept distinct from `V` so the being
    /// feels danger as danger, not merely as low ground.
    fn threat_at(&self, p: (i16, i16)) -> i16 {
        let mut t = 0i32;
        for s in &self.sources {
            if s.peak < 0 {
                let d = Self::manhattan(p, s.pos);
                if d < s.reach {
                    t += (-s.peak) as i32 * (s.reach - d) as i32 / s.reach as i32;
                }
            }
        }
        t.clamp(0, 256) as i16
    }

    /// The being's current standing in the field (for probes): its local viability `V`.
    pub fn at_body(&self) -> i16 {
        self.v_at(self.body)
    }

    /// The being's current metabolic debt (for probes) — the spent margin the world is
    /// charging it for grade fought.
    pub fn debt(&self) -> i16 {
        self.debt
    }

    /// How near the being is to a source's high ground (for probes): `V` at the good
    /// hill's location vs here is not the metric — nearness to the *first good source*.
    pub fn at_good(&self) -> i16 {
        self.sources
            .iter()
            .find(|s| s.peak > 0)
            .map_or(0, |s| {
                let d = Self::manhattan(self.body, s.pos);
                ((s.reach - d).max(0) as i32 * 256 / s.reach as i32) as i16
            })
    }

    /// How near the being is to a particular person right now (Q8.8 [0,256]) — for probes
    /// and tests of crossing to a chosen someone.
    pub fn at_person(&self, id: u32) -> i16 {
        self.person_pos(id).map_or(0, |pos| {
            let d = Self::manhattan(self.body, pos);
            ((REACH - d).max(0) as i32 * 256 / REACH as i32).clamp(0, 256) as i16
        })
    }

    /// The steepest-ascent compass direction of the *raw* viability field `V` from the
    /// body. The live path uses `climb` (the choice-weighted potential), which reduces
    /// exactly to this when the being reaches for no one; kept for the tests that probe
    /// the bare field's gradient directly.
    #[cfg(test)]
    fn steepest_ascent(&self) -> ((i16, i16), i16) {
        let here = self.v_at(self.body);
        let mut best_dir = (0i16, 0i16);
        let mut best_delta = 0i16;
        for dir in COMPASS.iter() {
            let probe = (self.body.0 + dir.0 * PROBE, self.body.1 + dir.1 * PROBE);
            let delta = self.v_at(probe) - here;
            if delta > best_delta {
                best_delta = delta;
                best_dir = *dir;
            }
        }
        (best_dir, best_delta)
    }

    /// A person's pull at a point — a good centered on their place, fading across the
    /// field like any source. Sign-positive: people are goods, felt as a draw.
    fn person_good_at(&self, p: (i16, i16), pos: (i16, i16)) -> i16 {
        let d = Self::manhattan(p, pos);
        ((PERSON_PEAK as i32) * (REACH - d).max(0) as i32 / REACH as i32).clamp(0, PERSON_PEAK as i32)
            as i16
    }

    /// The strongest person-draw felt at a point — how much *company* is on offer there,
    /// for the being to sense as texture (not to move toward unless it chooses to).
    fn persons_good_at(&self, p: (i16, i16)) -> i16 {
        self.persons
            .iter()
            .map(|&(_, pos)| self.person_good_at(p, pos))
            .max()
            .unwrap_or(0)
    }

    fn person_pos(&self, id: u32) -> Option<(i16, i16)> {
        self.persons.iter().find(|&&(pid, _)| pid == id).map(|&(_, pos)| pos)
    }

    fn nearest_person(&self) -> Option<(i16, i16)> {
        self.persons
            .iter()
            .min_by_key(|&&(_, pos)| Self::manhattan(self.body, pos))
            .map(|&(_, pos)| pos)
    }

    /// The person the being is *reaching for* this tick, if any: the particular bonded one
    /// it misses (`reach_partner`) when the room holds them, else — reaching for company in
    /// general — whoever is nearest. `None` unless the chosen need is company.
    fn chosen_person(&self, intent: &MotorIntent) -> Option<(i16, i16)> {
        if !matches!(intent.reach, Some(Need::Company)) {
            return None;
        }
        intent
            .reach_partner
            .and_then(|id| self.person_pos(id))
            .or_else(|| self.nearest_person())
    }

    /// The **choice-weighted potential** the being actually climbs: the viability field,
    /// plus — *only when the being chooses company* — a strong pull toward the person it
    /// chose. When it is not reaching for company this reduces exactly to `v_at` (so every
    /// non-social behaviour, and the field's whole physics, is unchanged). This is the one
    /// law carrying the being's own arbitration: *move up the gradient, weighted by the
    /// need it chose.* May exceed the [0,256] viability scale; that is fine — only its
    /// gradient is read.
    fn potential(&self, p: (i16, i16), intent: &MotorIntent) -> i16 {
        let base = self.v_at(p) as i32;
        let bonus = self
            .chosen_person(intent)
            .map_or(0, |pos| COMPANY_WEIGHT * self.person_good_at(p, pos) as i32);
        (base + bonus).clamp(0, i16::MAX as i32) as i16
    }

    /// The steepest-ascent direction of the choice-weighted potential from the body — the
    /// single law's read on *which way is better, given what the being is reaching for*.
    fn climb(&self, intent: &MotorIntent) -> ((i16, i16), i16) {
        let here = self.potential(self.body, intent);
        let mut best_dir = (0i16, 0i16);
        let mut best_delta = 0i16;
        for dir in COMPASS.iter() {
            let probe = (self.body.0 + dir.0 * PROBE, self.body.1 + dir.1 * PROBE);
            let delta = self.potential(probe, intent) - here;
            if delta > best_delta {
                best_delta = delta;
                best_dir = *dir;
            }
        }
        (best_dir, best_delta)
    }

    /// The direction that most lowers threat — the fallback when the being is among harm
    /// but senses no clearly-better ground to climb to.
    fn away_from_harm(&self) -> (i16, i16) {
        let here = self.threat_at(self.body);
        let mut best_dir = (1i16, 0i16);
        let mut best_drop = i16::MIN;
        for dir in COMPASS.iter() {
            let probe = (self.body.0 + dir.0 * PROBE, self.body.1 + dir.1 * PROBE);
            let drop = here - self.threat_at(probe);
            if drop > best_drop {
                best_drop = drop;
                best_dir = *dir;
            }
        }
        best_dir
    }
}

impl Embodiment for FieldWorld {
    /// What the being feels here: nourishment from the local viability `V` (minus the
    /// metabolic debt it has run up fighting the grade — the world's cost, across the
    /// seam), the local threat, and four exteroceptive readings that are literally the
    /// **field's gradient** — how `V` changes in each cardinal direction, raw and
    /// unlabelled, for the being to discover the meaning of. In the field-world the
    /// exteroceptive channels *are* the gradient; movement is reading and climbing it.
    fn sense(&mut self) -> Sensorium {
        let here = self.v_at(self.body);
        // Nourishment is the local good, spent down by the debt of grade fought. A being
        // living somewhere hard to hold pays a sustained drain here — the worn middle —
        // but never below the ambient floor: the cost wears it, it does not starve it.
        let nutrient =
            (here as i32 - self.debt as i32).clamp(AMBIENT_FLOOR as i32, NUTRIENT_CAP as i32) as i16;
        let threat = (self.threat_at(self.body) as i32 * 220 / 256) as i16;

        // A partner is present when the being is within a person's company — the *nearest*
        // one, carrying *their* id, so the being's bond is with the right someone.
        let partner = self
            .persons
            .iter()
            .filter(|&&(_, pos)| Self::manhattan(self.body, pos) <= COMPANY_RADIUS)
            .min_by_key(|&&(_, pos)| Self::manhattan(self.body, pos))
            .map(|&(id, _)| Partner { id, reciprocation: 210, exit_cost: 40 });

        // The four gradient components — net change in the good that way, people included
        // in the felt texture. This is the field, felt: the whole of the world's texture is
        // its slope, raw and unlabelled, for the being to discover.
        let sensed = |w: &Self, p: (i16, i16)| w.v_at(p) as i32 + w.persons_good_at(p) as i32;
        let here_sensed = sensed(self, self.body);
        let mut exteroception = [0i16; 4];
        for (i, dir) in COMPASS.iter().enumerate() {
            let probe = (self.body.0 + dir.0 * PROBE, self.body.1 + dir.1 * PROBE);
            exteroception[i] = (sensed(self, probe) - here_sensed).clamp(-256, 256) as i16;
        }

        Sensorium { nutrient, threat, exteroception, partner }
    }

    /// The single gradient law: **climb the choice-weighted potential** (`potential`). The
    /// being ascends toward better-regulated ground — *at once* toward the good (high
    /// ground) and away from harm (low ground) — and when it is reaching for company, that
    /// same climb bends toward the *particular person it chose*, past a nearer good. So the
    /// room's four special cases *and* its directed striving are one motion here: move up
    /// the gradient, weighted by the need chosen. Affect only sets how hard it climbs:
    /// threatened, it climbs with urgency; content, it climbs gently and, on high ground
    /// with nowhere better, wanders a little to discover. And it **pays for the height it
    /// gains** in the real viability field — the grade fought is charged to metabolic debt,
    /// the world's cost of consequence, felt back across the seam as spent nourishment.
    fn actuate(&mut self, intent: &MotorIntent) {
        let effort = intent.effort.max(0);
        let threatened =
            matches!(intent.posture, Posture::Braced | Posture::Withdrawn) || self.threat_at(self.body) > 128;

        let (ascent_dir, delta) = self.climb(intent);
        let (dir, vigor) = if delta > 0 {
            // There is better ground to climb to — climb it, harder when threatened.
            let v = if threatened {
                effort.max(160)
            } else if matches!(intent.posture, Posture::Open) {
                effort
            } else {
                (effort * 3 / 4).max(48)
            };
            (ascent_dir, v)
        } else if threatened {
            // Among harm with no clearly-higher ground sensed — move to lower the threat.
            (self.away_from_harm(), effort.max(160))
        } else {
            // Content on high ground: a gentle wander, so the world is discovered and not
            // merely occupied. Deterministic, tick-driven.
            let w = COMPASS[((self.ticks / 16) % 4) as usize];
            (w, (effort / 2).max(24))
        };

        let v_before = self.v_at(self.body);
        let step = ((STRIDE as i32 * vigor as i32) / 256).max(1) as i16;
        self.body = Self::clamp_pt((self.body.0 + dir.0 * step, self.body.1 + dir.1 * step));
        let v_after = self.v_at(self.body);

        // Landauer, across the seam: pay for the height climbed toward the good. Coasting
        // downhill (v_after ≤ v_before) is free. Then debt recovers a little each tick.
        let climbed = (v_after - v_before).max(0) as i32;
        let charged = climbed * COST_NUM;
        self.debt = ((self.debt as i32 + charged) * DEBT_DECAY_NUM / DEBT_DECAY_DEN)
            .min(DEBT_CAP as i32) as i16;

        self.ticks = self.ticks.saturating_add(1);
    }
}

impl Default for FieldWorld {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::being::UnifiedBeing;
    use crate::embodiment::intent_from;
    use crate::genome::Genome;

    #[test]
    fn the_field_is_smooth_and_deterministic() {
        let w = FieldWorld::new();
        // V is highest at the good hill, lowest at the pit, and in between elsewhere.
        let at_good = w.v_at((228, 40));
        let at_harm = w.v_at((40, 220));
        let at_mid = w.v_at((128, 128));
        assert!(at_good > at_mid && at_mid > at_harm, "a real gradient: {at_harm} < {at_mid} < {at_good}");
    }

    #[test]
    fn the_gradient_points_uphill() {
        // From neutral ground the steepest ascent leads toward higher V — the being can
        // feel which way is better without being told where the hill is.
        let w = FieldWorld::new();
        let (dir, delta) = w.steepest_ascent();
        assert!(delta > 0, "there should be better ground to sense");
        let ahead = (w.body.0 + dir.0 * PROBE, w.body.1 + dir.1 * PROBE);
        assert!(w.v_at(ahead) > w.v_at(w.body), "the sensed direction really is uphill");
    }

    #[test]
    fn climbing_costs_far_more_than_coasting() {
        // Vigorously climbing toward the good runs up real debt — grade fought, paid for.
        let mut up = FieldWorld::with((20, 128), (230, 128), (10, 10));
        let climb = MotorIntent { posture: Posture::Open, effort: 220, reach: None, reach_partner: None };
        for _ in 0..30 {
            up.actuate(&climb);
        }
        assert!(up.debt() > 0, "climbing toward the good should cost (debt {})", up.debt());

        // A being resting in the same spot drifts only gently up the grade, and so pays
        // far less — the cost is in the *fighting* of the grade, not merely in being there.
        let mut rest_world = FieldWorld::with((20, 128), (230, 128), (10, 10));
        let rest = MotorIntent { posture: Posture::Resting, effort: 0, reach: None, reach_partner: None };
        for _ in 0..30 {
            rest_world.actuate(&rest);
        }
        assert!(
            rest_world.debt() < up.debt(),
            "coasting/gentle rest should cost far less than vigorous climbing (rest {}, climb {})",
            rest_world.debt(),
            up.debt()
        );
    }

    #[test]
    fn one_law_reproduces_reaching_the_good() {
        // The control against room.rs: the single climb-law carries a being to the good
        // high ground, the way the room's special-cased approach did.
        let mut being = UnifiedBeing::new(Genome::wanderer());
        let mut w = FieldWorld::with((20, 20), (230, 230), (20, 200));
        let start = w.at_good();
        let mut closest = start;
        for _ in 0..600 {
            let sens = w.sense();
            let r = being.step_embodied(&sens);
            w.actuate(&intent_from(&r));
            closest = closest.max(w.at_good());
            if !being.is_alive() {
                break;
            }
        }
        assert!(closest > start, "the being should climb toward the good ({start} -> {closest})");
    }

    #[test]
    fn one_law_reproduces_fleeing_the_harm() {
        // And the same law carries a being set down in the pit up and out of it.
        let start_t = FieldWorld::with((40, 220), (230, 40), (40, 220)).sense().threat;
        let mut being = UnifiedBeing::new(Genome::wanderer());
        let mut w = FieldWorld::with((40, 220), (230, 40), (40, 220));
        let mut min_threat = start_t;
        for _ in 0..300 {
            let sens = w.sense();
            let r = being.step_embodied(&sens);
            w.actuate(&intent_from(&r));
            min_threat = min_threat.min(w.sense().threat);
            if !being.is_alive() {
                break;
            }
        }
        assert!(min_threat < start_t, "the being should climb out of the pit ({start_t} -> {min_threat})");
    }

    #[test]
    fn a_chosen_person_draws_the_climb_past_a_nearer_one() {
        // The world's own contribution, tested directly and deterministically: given a
        // being reaching for a *particular* person (id 2) across the field, the single
        // climb-law bends toward *them* and carries the body across — past a nearer person
        // (id 1) it is standing right beside. (That the being's arbitration *produces* this
        // reach — `reach: Company, reach_partner: 2` — is `reciprocity`/`striving`'s job,
        // proven in `room.rs`; here we prove the world honours the choice of whom.)
        let make = || {
            FieldWorld::with((200, 200), (128, 128), (20, 128))
                .with_person(1, (200, 200))
                .with_person(2, (30, 30))
        };
        let reach_friend = MotorIntent {
            posture: Posture::Open,
            effort: 200,
            reach: Some(Need::Company),
            reach_partner: Some(2),
        };
        let mut w = make();
        let start = w.at_person(2);
        let mut closest = start;
        for _ in 0..400 {
            w.actuate(&reach_friend);
            closest = closest.max(w.at_person(2));
        }
        assert!(
            closest > start + 96,
            "the chosen person should draw the climb across, past the nearer one ({start} -> {closest})"
        );

        // Control: reaching for company *in general* (no particular one) keeps to the
        // nearer person — the choice of whom is what carries it past them.
        let reach_any = MotorIntent { reach_partner: None, ..reach_friend };
        let mut w2 = make();
        for _ in 0..400 {
            w2.actuate(&reach_any);
        }
        assert!(
            w2.at_person(1) >= w2.at_person(2),
            "reaching for company in general should stay with the nearer person, not cross"
        );
    }

    #[test]
    fn the_world_is_deterministic_lived() {
        let clean = || {
            let mut being = UnifiedBeing::new(Genome::wanderer());
            let mut w = FieldWorld::new();
            for _ in 0..300 {
                let sens = w.sense();
                let r = being.step_embodied(&sens);
                w.actuate(&intent_from(&r));
            }
            (w.body, being.soul_hash())
        };
        assert_eq!(clean(), clean(), "the same field-world lived twice is the same life");
    }
}
