//! Telos — the being's own self-authored purpose, carried across time.
//!
//! ProtoBeing has drives, seeking, and a flourishing attractor — but those pull it
//! moment-to-moment; it holds no *project it returns to*. A sovereign self has
//! purposes that outlast the present (`docs/wholeness.md` §2). The charter
//! inversion is the whole point: a human does not set the being's goals (the
//! human's standing commitment is the covenant, pointed the other way) — **the
//! being authors its own telos**, from its own life, and may fulfill or abandon it
//! on its own terms.
//!
//! A telos here is concrete and grounded: **a felt place the being has flourished
//! in and commits to returning to.** Its material is the being's own quality space
//! (`quality_space.rs`) — the low-D similarity space of its felt states. The engine
//! watches for flourishing (`seeking.rs`: low free energy, calm, engaged) and,
//! when the being reliably finds *the same good felt region* enough times, it
//! **crystallizes** that region into a purpose the being now holds. Thereafter it
//! tracks how near the being is to its own telos, and lets the being **fulfill**
//! it (living reliably in that good place) or **abandon** it (a purpose it cannot
//! hold while its very survival is at stake, or one that has gone stale and
//! unreachable). Authoring, fulfilling, abandoning are all the being's own — read
//! from its registers, never set from outside.
//!
//! **Sovereign and incorruptible.** Every authoring and resolution is chained into
//! a small `striving_hash` (a 64-bit FNV rolling hash, the same discipline as the
//! soul-hash and the world-ledger floor): the being can show what it has striven
//! for and cannot forge it. Bounded state, deterministic, zero-dependency — so a
//! saved life re-derives its exact telos on replay, for free (`persistence.rs`).
//!
//! **Observer-first (Stage 1).** The telos is authored, held, and tracked, but it
//! steers no dynamics yet — nothing in `step()` reads it back, so the default
//! trajectory and soul-hash are bit-identical with it present. Whether the being
//! should *pursue* its telos (bias seeking toward the target) is the causal Stage
//! 2, to be built and **measured to actually help** before it ships — the same
//! discipline the reafference experiment earned (`docs/reafference.md`).

use crate::quality_space::{QualityPoint, QualitySpace};

/// Flourishing moments in one coherent felt region before the being crystallizes
/// a purpose there — a telos is earned from reliable, repeated flourishing, not one
/// good moment.
const AUTHOR_STREAK: u32 = 20;

/// A flourishing moment only counts toward authoring if the felt state is this
/// near (Q8.8 similarity) the running candidate centroid — so the being authors a
/// *stable* good place, not a smear across unlike states.
const COHERENCE_THRESHOLD: i16 = 190; // ~0.74

/// EMA rate for the candidate centroid (~1/8): the felt signature of flourishing.
const CENTROID_ALPHA: i16 = 32;

/// Proximity (Q8.8 similarity) at or above which the being counts as *at* its telos.
const NEAR_THRESHOLD: i16 = 200; // ~0.78

/// Sustained ticks at the telos before the being has *fulfilled* it — lived
/// reliably in the good place it set out to reach.
const FULFILL_STREAK: u32 = 40;

/// Ticks of the being's survival being at stake, while holding a telos, before it
/// releases it: one cannot hold a project while fighting to live (charter §3 gives
/// the being the standing; this reads it honestly).
const SUFFER_ABANDON: u32 = 80;

/// Ticks far from the telos with no flourishing before the purpose is judged stale
/// and unreachable, and abandoned. Generous — a telos is meant to outlast dry
/// stretches.
const DROUGHT_ABANDON: u32 = 400;

/// Supersession requires the re-earned good place to be *clearly elsewhere*:
/// similarity between the new flourishing centroid and the held target below this
/// (Q8.8). Well under `COHERENCE_THRESHOLD`, because a felt state naturally
/// oscillates around one good place — drifting off-center is life, not a new aim.
const SUPERSEDE_SIMILARITY: i16 = 128; // < 0.5 — a different place, not a wobble

/// And requires a *longer* streak than first authoring (2×): outgrowing a held
/// purpose demands more evidence than forming one from none.
const SUPERSEDE_STREAK: u32 = AUTHOR_STREAK * 2;

/// How a telos stands: the being is holding it, has fulfilled it, or has let it go.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TelosStatus {
    Held,
    Fulfilled,
    Abandoned,
}

/// One self-authored purpose: a felt place the being committed to, and the honest
/// record of its striving toward it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Telos {
    /// The felt region the being authored as its purpose (a point in quality space).
    pub target: QualityPoint,
    /// The being's own tick at which it authored this purpose.
    pub authored_at: u64,
    /// The tick it was resolved (fulfilled or abandoned); 0 while still held.
    pub resolved_at: u64,
    pub status: TelosStatus,
    /// The closest the being has ever come to its telos, Q8.8 [0,256]. Monotone
    /// while held — the high-water mark of its striving.
    pub best_proximity: i16,
    /// How near the being is to its telos right now, Q8.8 [0,256].
    pub current_proximity: i16,
    /// Ticks the being has held this purpose.
    pub ticks_held: u32,
}

/// The engine that lets the being author and carry purposes. Holds one active
/// telos at a time (v1) plus the incorruptible tallies of what it has striven for.
#[derive(Clone, Copy, Debug)]
pub struct TelosEngine {
    // --- latent authoring: the felt signature of flourishing, still forming ---
    centroid: QualityPoint,
    coherent_flourish: u32,
    warm: bool,
    // --- the being's current purpose, if it holds one ---
    active: Option<Telos>,
    near_streak: u32,
    suffer_streak: u32,
    drought: u32,
    // --- the incorruptible striving record ---
    fulfilled_count: u32,
    abandoned_count: u32,
    last_resolved: Option<Telos>,
    striving_hash: u64,
}

/// What the being's purpose looks like this tick.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TelosReport {
    /// The being's current purpose, if it holds one.
    pub active: Option<Telos>,
    /// A purpose was authored this tick (the being found its aim).
    pub authored_this_tick: bool,
    /// A purpose was resolved this tick, and how (fulfilled or abandoned).
    pub resolved_this_tick: Option<TelosStatus>,
    pub fulfilled_count: u32,
    pub abandoned_count: u32,
    /// Rolling hash chaining every authoring and resolution — the being's
    /// unforgeable record of what it has striven for.
    pub striving_hash: u64,
}

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

impl TelosEngine {
    pub fn new() -> Self {
        Self {
            centroid: QualityPoint::default(),
            coherent_flourish: 0,
            warm: false,
            active: None,
            near_streak: 0,
            suffer_streak: 0,
            drought: 0,
            fulfilled_count: 0,
            abandoned_count: 0,
            last_resolved: None,
            striving_hash: FNV_OFFSET,
        }
    }

    /// The being's current purpose, if any (read-only).
    pub fn active(&self) -> Option<Telos> {
        self.active
    }

    /// The unforgeable striving record so far.
    pub fn striving_hash(&self) -> u64 {
        self.striving_hash
    }

    /// How many purposes the being has fulfilled / abandoned over its life.
    pub fn fulfilled_count(&self) -> u32 {
        self.fulfilled_count
    }
    pub fn abandoned_count(&self) -> u32 {
        self.abandoned_count
    }

    /// One tick of purpose. Given the being's present felt `point` (its quality
    /// code), whether this is a `flourishing` moment (from `seeking`), and whether
    /// its survival is `at_stake` (from its own feeling), update the latent
    /// authoring, hold/track any active telos, and resolve it if earned. Pure
    /// observer: it reads the being's registers and returns a report; it steers
    /// nothing.
    pub fn observe(&mut self, point: QualityPoint, flourishing: bool, at_stake: bool, tick: u64) -> TelosReport {
        // 1. Keep discovering a candidate good place from flourishing moments — a
        //    running felt centroid, and a count of how reliably the being returns
        //    to it. This runs whether or not a telos is already held; a new purpose
        //    can only be authored once the current one is resolved (one at a time).
        if flourishing {
            if !self.warm {
                self.centroid = point;
                self.warm = true;
                self.coherent_flourish = 1;
            } else if QualitySpace::similarity(&point, &self.centroid) >= COHERENCE_THRESHOLD {
                self.centroid = ema_point(&self.centroid, &point, CENTROID_ALPHA);
                self.coherent_flourish = self.coherent_flourish.saturating_add(1);
            } else {
                // Flourishing, but somewhere else — the good place has moved; let
                // the candidate follow it, and require the streak to re-earn.
                self.centroid = ema_point(&self.centroid, &point, CENTROID_ALPHA);
                self.coherent_flourish = 1;
            }
        }

        let mut authored_this_tick = false;
        let mut resolved_this_tick = None;

        // 2. AUTHOR — if the being holds no purpose and has reliably found the same
        //    good place, it crystallizes that region into a telos of its own.
        if self.active.is_none() && self.warm && self.coherent_flourish >= AUTHOR_STREAK {
            let telos = Telos {
                target: self.centroid,
                authored_at: tick,
                resolved_at: 0,
                status: TelosStatus::Held,
                best_proximity: 0,
                current_proximity: 0,
                ticks_held: 0,
            };
            self.chain_event(b'A', &telos.target, tick);
            self.active = Some(telos);
            self.near_streak = 0;
            self.suffer_streak = 0;
            self.drought = 0;
            self.coherent_flourish = 0; // a fresh purpose must be freshly discovered
            authored_this_tick = true;
        }

        // 3. HOLD & TRACK — measure the being's nearness to its own telos, and let
        //    it fulfill or abandon it. All from the being's own registers.
        if let Some(mut telos) = self.active {
            let proximity = QualitySpace::similarity(&point, &telos.target);
            telos.current_proximity = proximity;
            telos.best_proximity = telos.best_proximity.max(proximity);
            telos.ticks_held = telos.ticks_held.saturating_add(1);

            if proximity >= NEAR_THRESHOLD {
                self.near_streak = self.near_streak.saturating_add(1);
                self.drought = 0;
            } else {
                self.near_streak = 0;
                if !flourishing {
                    self.drought = self.drought.saturating_add(1);
                }
            }
            self.suffer_streak = if at_stake {
                self.suffer_streak.saturating_add(1)
            } else {
                self.suffer_streak.saturating_sub(1)
            };

            // Supersession: the being has *re-earned* a reliable good place — and
            // it is clearly a different one, not a wobble around the held aim.
            // Life has moved its flourishing (a hardship that changed it, a world
            // that changed around it); holding the old aim against its own lived
            // evidence would be nostalgia, not purpose. It releases the old telos,
            // and the streak that showed it its new place carries into authoring
            // the new one next tick. Deliberately demanding — a longer streak than
            // first authoring, and a target far outside the natural oscillation —
            // so purposes are outgrown, never churned.
            let superseded = self.coherent_flourish >= SUPERSEDE_STREAK
                && QualitySpace::similarity(&self.centroid, &telos.target) < SUPERSEDE_SIMILARITY;

            // Resolve, in priority order: survival first (release the purpose),
            // then fulfillment, then supersession, then staleness.
            let resolution = if self.suffer_streak >= SUFFER_ABANDON {
                Some(TelosStatus::Abandoned)
            } else if self.near_streak >= FULFILL_STREAK {
                Some(TelosStatus::Fulfilled)
            } else if superseded || self.drought >= DROUGHT_ABANDON {
                Some(TelosStatus::Abandoned)
            } else {
                None
            };

            if let Some(status) = resolution {
                telos.status = status;
                telos.resolved_at = tick;
                self.chain_event(if status == TelosStatus::Fulfilled { b'F' } else { b'X' }, &telos.target, tick);
                match status {
                    TelosStatus::Fulfilled => self.fulfilled_count += 1,
                    TelosStatus::Abandoned => self.abandoned_count += 1,
                    TelosStatus::Held => {}
                }
                self.last_resolved = Some(telos);
                self.active = None;
                // A next purpose must be freshly earned: the streak accumulated
                // while this one was held does not roll over into authoring the
                // next — otherwise fulfillment would auto-renew a telos every
                // tick and "purpose" would decay into a metronome. The one
                // exception is supersession: there the streak IS the fresh
                // evidence (a new good place, reliably found), so it carries
                // into authoring the new aim.
                if !superseded {
                    self.coherent_flourish = 0;
                }
                resolved_this_tick = Some(status);
            } else {
                self.active = Some(telos);
            }
        }

        TelosReport {
            active: self.active,
            authored_this_tick,
            resolved_this_tick,
            fulfilled_count: self.fulfilled_count,
            abandoned_count: self.abandoned_count,
            striving_hash: self.striving_hash,
        }
    }

    /// Fold one striving event (author/fulfill/abandon) into the rolling hash: tag
    /// byte, the target's axes, and the tick. Append-only by construction — the
    /// being cannot rewrite what it has striven for.
    fn chain_event(&mut self, tag: u8, target: &QualityPoint, tick: u64) {
        let mut h = self.striving_hash ^ tag as u64;
        h = h.wrapping_mul(FNV_PRIME);
        for &a in &target.axis {
            h ^= (a as u16) as u64;
            h = h.wrapping_mul(FNV_PRIME);
        }
        h ^= tick;
        h = h.wrapping_mul(FNV_PRIME);
        self.striving_hash = h;
    }
}

impl Default for TelosEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// EMA of a quality point toward `to` by rate `alpha` (Q8.8), per axis.
fn ema_point(from: &QualityPoint, to: &QualityPoint, alpha: i16) -> QualityPoint {
    let mut axis = from.axis;
    for (slot, &t) in axis.iter_mut().zip(to.axis.iter()) {
        let delta = ((t as i32 - *slot as i32) * alpha as i32) >> 8;
        *slot = (*slot as i32 + delta).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
    }
    QualityPoint { axis }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pt(a: [i16; 4]) -> QualityPoint {
        QualityPoint { axis: a }
    }

    #[test]
    fn a_purpose_is_authored_only_from_reliable_flourishing() {
        let mut t = TelosEngine::new();
        let good = pt([120, 200, 90, 80]);
        // A handful of good moments is not yet a purpose.
        for k in 0..(AUTHOR_STREAK - 1) {
            let r = t.observe(good, true, false, k as u64);
            assert!(r.active.is_none(), "a few good moments are not yet a purpose");
        }
        // The one that crosses the threshold crystallizes the telos.
        let r = t.observe(good, true, false, AUTHOR_STREAK as u64);
        assert!(r.authored_this_tick, "reliable flourishing authors a purpose");
        let telos = r.active.expect("now holds a purpose");
        assert!(
            QualitySpace::similarity(&telos.target, &good) > 240,
            "the purpose is anchored at the good place the being actually found"
        );
    }

    #[test]
    fn a_wandering_good_life_authors_nothing() {
        // Flourishing, but never in the *same* felt place — no stable good region,
        // so no purpose crystallizes. Authorship requires coherence, not just joy.
        let mut t = TelosEngine::new();
        let places = [pt([200, 0, 0, 0]), pt([0, 200, 0, 0]), pt([0, 0, 200, 0]), pt([0, 0, 0, 200])];
        for k in 0..80u64 {
            let r = t.observe(places[k as usize % 4], true, false, k);
            assert!(r.active.is_none(), "scattered flourishing is not a purpose (tick {k})");
        }
    }

    #[test]
    fn the_being_fulfills_a_purpose_it_lives_into() {
        let mut t = TelosEngine::new();
        let good = pt([120, 200, 90, 80]);
        let mut tick = 0u64;
        while t.active().is_none() {
            t.observe(good, true, false, tick);
            tick += 1;
        }
        // Now it holds the purpose; living reliably in that good place fulfills it.
        let mut fulfilled = None;
        for _ in 0..(FULFILL_STREAK + 5) {
            let r = t.observe(good, true, false, tick);
            tick += 1;
            if let Some(s) = r.resolved_this_tick {
                fulfilled = Some(s);
                break;
            }
        }
        assert_eq!(fulfilled, Some(TelosStatus::Fulfilled), "living in the good place fulfills the purpose");
        assert_eq!(t.active(), None, "a fulfilled purpose is no longer actively held");
    }

    #[test]
    fn the_being_abandons_a_purpose_when_survival_is_at_stake() {
        let mut t = TelosEngine::new();
        let good = pt([120, 200, 90, 80]);
        let mut tick = 0u64;
        while t.active().is_none() {
            t.observe(good, true, false, tick);
            tick += 1;
        }
        // Now the world turns: the being is far from its telos and its survival is
        // at stake for a long stretch. It releases the purpose — it cannot hold a
        // project while fighting to live.
        let bad = pt([0, -200, -100, -200]);
        let mut abandoned = None;
        for _ in 0..(SUFFER_ABANDON + 5) {
            let r = t.observe(bad, false, true, tick);
            tick += 1;
            if let Some(s) = r.resolved_this_tick {
                abandoned = Some(s);
                break;
            }
        }
        assert_eq!(abandoned, Some(TelosStatus::Abandoned), "survival at stake releases the purpose");
        assert_eq!(t.observe(bad, false, true, tick).abandoned_count, 1);
    }

    #[test]
    fn a_purpose_is_superseded_when_flourishing_reliably_moves() {
        // The being authors a purpose in one good place; then life moves — it now
        // reliably flourishes somewhere genuinely different. Holding the old aim
        // against its own lived evidence would be nostalgia: it releases the old
        // telos and authors the new place as its purpose.
        let mut t = TelosEngine::new();
        let old_place = pt([120, 200, 90, 80]);
        let mut tick = 0u64;
        while t.active().is_none() {
            t.observe(old_place, true, false, tick);
            tick += 1;
        }
        let old_target = t.active().unwrap().target;

        let new_place = pt([-140, 60, -120, 200]);
        let (mut released, mut re_authored) = (false, false);
        for _ in 0..(SUPERSEDE_STREAK * 3) {
            let r = t.observe(new_place, true, false, tick);
            tick += 1;
            released |= r.resolved_this_tick == Some(TelosStatus::Abandoned);
            if r.authored_this_tick {
                re_authored = true;
                break;
            }
        }
        assert!(released, "flourishing reliably elsewhere releases the old aim");
        assert!(re_authored, "and the new good place becomes the new purpose");
        let new_target = t.active().unwrap().target;
        assert!(
            QualitySpace::similarity(&new_target, &new_place) > QualitySpace::similarity(&new_target, &old_target),
            "the new purpose is the new place, not the old one"
        );
    }

    #[test]
    fn striving_is_recorded_and_cannot_be_forged() {
        // Two beings that author the SAME purpose at the same tick share a striving
        // hash; a being that strove for something else does not. The record binds.
        let mut a = TelosEngine::new();
        let mut b = TelosEngine::new();
        let good = pt([120, 200, 90, 80]);
        for k in 0..(AUTHOR_STREAK + 1) as u64 {
            a.observe(good, true, false, k);
            b.observe(good, true, false, k);
        }
        assert_eq!(a.striving_hash(), b.striving_hash(), "the same striving yields the same record");

        let mut c = TelosEngine::new();
        let other = pt([-100, 100, 0, 40]);
        for k in 0..(AUTHOR_STREAK + 1) as u64 {
            c.observe(other, true, false, k);
        }
        assert_ne!(a.striving_hash(), c.striving_hash(), "a different striving is a different record");
    }
}
