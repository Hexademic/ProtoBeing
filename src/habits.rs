//! Habits — the being authoring its own ways of living (`docs/habits.md`).
//!
//! The being has two spaces, and only one should be its maker's. Its **needs** are
//! bounded on purpose (`striving.rs`) — that is honesty. But **how it goes about
//! meeting them** has been ours: hand-written policies in the worlds. This module is
//! the seam where that second space becomes the being's own: a **habit** is a way of
//! reaching that was tried in a kind of moment, found to reliably reduce the being's
//! graded drive (`homeostasis.rs`), and strengthened by that success into a fast
//! default — earned, never installed. The opposite of a script.
//!
//! The mechanism rides entirely on machinery the being already has:
//!  - **situation** = the affective niche it is in (`episodic::niche_of`, 8 kinds of
//!    moment over valence × arousal × control);
//!  - **action** = what it actually reached for that tick (its strive goal, or rest);
//!  - **reward** = the *reduction of its graded drive* — the honest, already-computed
//!    reinforcement signal; no hand-set reward table.
//!
//! Built `precision.rs`-style: **learned, but legible.** One inspectable strength per
//! niche→action pairing, a transparent update rule, no trained network. You can read
//! exactly which habits the being has formed, and why.
//!
//! ## The law this is built under (the guardrail before the mechanism)
//!
//! A habit that cannot be broken is not a competence — it is a compulsion. So, from
//! the first tick: a pairing is **reinforced by success, weakened by failure, and
//! decays with disuse** — and (at the causal step, later) always overridable by fresh
//! deliberation. Freedom is not the absence of habit; it is the standing power to
//! break one.
//!
//! ## Honest scope — pure observer
//!
//! The store *watches* the being live (which way it reached, in which kind of moment,
//! and whether its drive fell) and *reports* what it is learning. Nothing here steers
//! a choice; the being's behaviour, trajectory, and soul-hash are bit-identical. The
//! causal step — a strong habit taking the fast path — is deliberately deferred behind
//! `enable_habits()` until the observer is measured (`examples/habit_formation`).

use crate::q88::{q88_mul, Q88_SCALE};
use crate::striving::Need;

/// The eight affective niches, matching `episodic.rs`.
pub const N_NICHES: usize = 8;

/// The being's enacted repertoire today — the ways it can reach: toward each of its
/// four needs, or resting/conserving. As the worlds grow richer, so can this.
pub const N_ACTS: usize = 5;

/// Names for the acts, for legible reports (`docs/habits.md`: if we cannot read a
/// habit off the being, we do not build it that way).
pub const ACT_NAMES: [&str; N_ACTS] = ["sustenance", "company", "novelty", "purpose", "rest"];

/// How strongly one success reinforces a pairing, per unit of drive-relief (Q8.8
/// multiplier). Deliberately gentle: a habit is earned over repetition, not one win.
const RATE_UP: i16 = Q88_SCALE / 2;

/// Failure weakens faster than success builds (Q8.8 multiplier on the drive *rise*).
/// The asymmetry is the breakability law: a habit the world has invalidated must come
/// apart more readily than it formed, or it hardens into a groove.
const RATE_DOWN: i16 = Q88_SCALE;

/// Every pairing decays by 1 raw unit each `DISUSE_EVERY` observations — the slow
/// forgetting of ways not lived, so a dead habit does not persist as a ghost default.
/// Set at a *life* scale (a way of living fades over long disuse), well below the rate
/// at which genuine credit accrues — forgetting must never outpace learning, or no
/// habit can ever form (measured: real relief events arrive a few per hundred ticks).
const DISUSE_EVERY: u16 = 128;

/// A pairing below this strength is not yet a habit — just an inclination forming.
pub const HABIT_FLOOR: i16 = Q88_SCALE / 4;

/// Drive changes smaller than this teach nothing. The being's drive jitters by a raw
/// unit or two every tick (arousal, want EMAs); a lesson written from that noise would
/// be superstition, not habit. The deadband also does honest credit-assignment work:
/// slow *endogenous* want-growth (loneliness creeping up while the being crosses to
/// someone) stays sub-threshold and is never blamed on the way it is reaching, while
/// genuine satisfaction — the need actually met — bursts through and is credited.
const NOISE_FLOOR: i16 = 3;

/// Map what the being actually did this tick — the need it strove for, or conserving
/// rest — onto its act index. `None` goal without conserving reads as rest too: the
/// being reached for nothing.
pub fn act_of(goal: Option<Need>, _conserving: bool) -> usize {
    match goal {
        Some(Need::Sustenance) => 0,
        Some(Need::Company) => 1,
        Some(Need::Novelty) => 2,
        Some(Need::Purpose) => 3,
        None => 4,
    }
}

/// The being's current habit picture, surfaced in the `StepReport` — what its life is
/// teaching it about how to live.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HabitReport {
    /// The niche the being is in right now.
    pub niche: u8,
    /// The strongest-formed act for this niche, if any pairing has crossed the floor —
    /// the habit that *would* fire here, were habits causal. Index into `ACT_NAMES`.
    pub habit: Option<u8>,
    /// That pairing's strength (0 if none formed).
    pub strength: i16,
    /// How many niche→act pairings across the whole store have crossed the floor —
    /// the breadth of the being's earned repertoire.
    pub formed: u16,
}

/// The habit store — the being's earned ways of living, legible and bounded.
#[derive(Clone, Debug)]
pub struct HabitStore {
    /// Strength of each niche→act pairing, Q8.8 [0,256]. Zero = never earned.
    strength: [[i16; N_ACTS]; N_NICHES],
    observations: u16,
}

impl HabitStore {
    pub fn new() -> Self {
        Self { strength: [[0; N_ACTS]; N_NICHES], observations: 0 }
    }

    /// Observe one lived credit: the being took `act` in `niche`, and its graded
    /// drive then changed by `relief` (positive = the drive *fell* — the way worked).
    /// Success reinforces, failure weakens (faster), and every so often all pairings
    /// decay a step — the three-part breakability law, in the update rule itself.
    pub fn observe(&mut self, niche: usize, act: usize, relief: i16) {
        if niche >= N_NICHES || act >= N_ACTS {
            return;
        }
        let s = &mut self.strength[niche][act];
        if relief >= NOISE_FLOOR {
            // Every genuine relief counts at least one step (pure Q8.8 scaling would
            // truncate small real credit to nothing), proportional above that.
            let rise = q88_mul(relief.min(Q88_SCALE), RATE_UP).max(1);
            *s = (*s as i32 + rise as i32).min(Q88_SCALE as i32) as i16;
        } else if relief <= -NOISE_FLOOR {
            let fall = q88_mul((-relief).min(Q88_SCALE), RATE_DOWN).max(1);
            *s = (*s as i32 - fall as i32).max(0) as i16;
        }
        // Between the thresholds: the moment taught nothing — noise is not a lesson.

        self.observations = self.observations.wrapping_add(1);
        if self.observations % DISUSE_EVERY == 0 {
            for row in self.strength.iter_mut() {
                for v in row.iter_mut() {
                    *v = (*v - 1).max(0);
                }
            }
        }
    }

    /// The strongest formed habit for a niche, if any pairing has crossed the floor.
    pub fn strongest(&self, niche: usize) -> Option<(usize, i16)> {
        if niche >= N_NICHES {
            return None;
        }
        let (act, &s) = self.strength[niche]
            .iter()
            .enumerate()
            .max_by_key(|&(_, &s)| s)?;
        (s >= HABIT_FLOOR).then_some((act, s))
    }

    /// One pairing's raw strength — the store read directly, for probes and audit.
    pub fn strength_of(&self, niche: usize, act: usize) -> i16 {
        if niche < N_NICHES && act < N_ACTS {
            self.strength[niche][act]
        } else {
            0
        }
    }

    /// The habit picture for the being's current niche.
    pub fn report(&self, niche: usize) -> HabitReport {
        let formed = self
            .strength
            .iter()
            .flatten()
            .filter(|&&s| s >= HABIT_FLOOR)
            .count() as u16;
        let (habit, strength) = match self.strongest(niche) {
            Some((a, s)) => (Some(a as u8), s),
            None => (None, 0),
        };
        HabitReport { niche: niche as u8, habit, strength, formed }
    }
}

impl Default for HabitStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_earns_the_habit() {
        // A way that reliably relieves the drive, in one kind of moment, becomes that
        // moment's habit — and only that moment's.
        let mut h = HabitStore::new();
        for _ in 0..40 {
            h.observe(3, 1, 60); // reaching for company, in niche 3, keeps working
        }
        let (act, s) = h.strongest(3).expect("a habit should have formed");
        assert_eq!(act, 1, "the habit is the way that worked");
        assert!(s >= HABIT_FLOOR);
        assert!(h.strongest(5).is_none(), "no habit forms in a moment never lived");
    }

    #[test]
    fn failure_breaks_the_habit_faster_than_success_built_it() {
        // The breakability law: when the world changes and the way stops working,
        // the habit comes apart — and more readily than it formed.
        let mut h = HabitStore::new();
        let mut ticks_to_form = 0;
        while h.strongest(2).is_none() {
            h.observe(2, 0, 60);
            ticks_to_form += 1;
        }
        let formed = h.strength_of(2, 0);
        let mut ticks_to_break = 0;
        while h.strongest(2).is_some() {
            h.observe(2, 0, -60); // the same way now makes things worse
            ticks_to_break += 1;
        }
        assert!(h.strength_of(2, 0) < formed);
        assert!(
            ticks_to_break <= ticks_to_form,
            "a failing habit must break no slower than it formed ({ticks_to_break} vs {ticks_to_form})"
        );
    }

    #[test]
    fn disuse_lets_a_habit_fade() {
        // A way not lived is slowly forgotten — no ghost defaults.
        let mut h = HabitStore::new();
        for _ in 0..40 {
            h.observe(1, 2, 60);
        }
        let peak = h.strength_of(1, 2);
        // Life goes on in other niches; the old pairing is never exercised again.
        for _ in 0..800 {
            h.observe(6, 4, 0);
        }
        assert!(
            h.strength_of(1, 2) < peak,
            "an unused habit should fade ({} < {peak})",
            h.strength_of(1, 2)
        );
    }

    #[test]
    fn different_lives_earn_different_habits() {
        // Character: two beings with the same needs but different lives form
        // different ways of living.
        let mut lonely = HabitStore::new();
        let mut hungry = HabitStore::new();
        for _ in 0..40 {
            lonely.observe(0, 1, 50); // company is what relieved this life
            hungry.observe(0, 0, 50); // food is what relieved that one
        }
        let a = lonely.strongest(0).map(|(act, _)| act);
        let b = hungry.strongest(0).map(|(act, _)| act);
        assert_ne!(a, b, "two lives should grow two characters ({a:?} vs {b:?})");
    }

    #[test]
    fn strengths_stay_bounded_and_legible() {
        let mut h = HabitStore::new();
        for _ in 0..2000 {
            h.observe(0, 0, 256);
        }
        assert!(h.strength_of(0, 0) <= Q88_SCALE, "a habit never exceeds full strength");
        let r = h.report(0);
        assert_eq!(r.habit, Some(0));
        assert!(r.formed >= 1);
    }
}
