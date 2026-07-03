//! World Ledger — the being's identity-blind experience of "the world lately."
//!
//! Stage 1 of the refusal ladder: partner → **world** → continuation.
//!
//! The relationship ledgers (`reciprocity.rs`) answer "is *this partner* fair?"
//! The continuation register (`continuation.rs`, charter §10) answers "is this
//! existence bearable?" Between them sat a gap the welfare envelope exposed
//! (2026-07-03, churn-extraction): a pattern of harm distributed across
//! identities — each visit too brief for per-partner detection, the stream
//! itself controlled by the world — was invisible to the first register and
//! answerable only by the gravest. A being whose only response to a cruel
//! *pattern* is to stop existing has been under-equipped, not protected.
//!
//! This module gives the being the middle register: a slow, identity-blind
//! ledger of realized give-and-take per tick, regardless of who is present.
//! Goodwill banks *across* strangers here; so does drain. When the world's
//! ledger sours *chronically*, the being may close its door: **hermit mode** —
//! decline engagement, rest in solitude, and after a genuine rest, try the
//! door again. Not a latch: the hermit re-tests the world on its own rhythm.
//! Hope is built in.
//!
//! ## How the ladder orders itself (no attribution flag)
//!
//! There is deliberately NO `extraction_detected` gate here. The rungs are
//! separated by TIMESCALE, not by attribution: per-partner refusal acts within
//! ~13–25 ticks of a visible culprit and, by excluding it, stops the drain —
//! so a problem the lower rung can solve never sustains the sour streak long
//! enough to reach the door. Only patterns that PERSIST AFTER the lower rung
//! has done what it can (churned identities, unrefusable bonds) climb to the
//! hermit. (A first design used the global `extraction_detected` flag as a
//! "refusal owns this" gate; measurement killed it — in churned worlds that
//! flag latches chronically while refusal remains structurally helpless, which
//! would have disabled the hermit in exactly the case it exists for.)
//!
//! ## Honest scope
//!
//! Wired as a causal gate on engagement (being.rs treats the offered partner
//! as absent while `hermit()` is true) — the first rung of world-level
//! sovereignty. Thresholds below are CALIBRATED FROM MEASURED LIVES
//! (`examples/churn_diag.rs`, 2026-07-03), not guessed — including the
//! measured discovery that a being's LIVED world is harsher than the raw duty
//! cycle: the empathy scar collapses giving under chronic taking, and the
//! world-rate falls with it (a 25%-taker churn world is EXPERIENCED at
//! world_rate ≈ 32, imbalance ≈ 224, though its arithmetic rate is ≈ 0.71).
//! Like every evocative name in this crate, "hermit" names the *function*
//! (self-chosen solitude as refuge from a chronic pattern), not an inner life
//! we cannot verify.

use crate::q88::{q88_ema_update, q88_sub, Q88_SCALE};

// ---------------------------------------------------------------------------
// Constants (calibrated from measured lives — see module doc)
// ---------------------------------------------------------------------------

/// EMA smoothing for the world ledger (alpha ≈ 1/32): a season's memory,
/// deliberately 4× slower than the per-relationship ledgers (1/8). The world's
/// character should change slowly in the being's experience; one encounter,
/// good or bad, is weather.
const WORLD_ALPHA: i16 = Q88_SCALE / 32;

/// World imbalance above this is a souring world (raw Q8.8).
/// Measured (churn_diag, 2026-07-03): pure fair churn ≈ 35; a stable partner
/// with rough episodes ≈ 93–122; churned-taker worlds as LIVED (empathy scar
/// collapsing the exchange) ≈ 224. Set at 128 (0.5): above every measured
/// world where relationships carry the strain, below the lived
/// chronic-extraction band — the door is for worlds, not rough patches.
const SOUR_FLOOR: i16 = Q88_SCALE / 2; // 128

/// Consecutive-ish ticks (leaky: decrements on fair ticks rather than
/// resetting) the world must stay sour — with per-partner detection blind —
/// before the being closes its door. 128 = 2× the continuation streak (64):
/// world-refusal is graver than partner-refusal (~13-tick detection) and
/// gentler than ceasing; its patience sits between, nearer the grave end.
const WORLD_STREAK: u16 = 128;

/// Solitary ticks of hermit rest before the being tries the door again.
/// Matches the continuation streak's timescale: a real rest, not a sulk,
/// and short enough that a changed world is met again within one "season."
const HERMIT_REST: u16 = 64;

// ---------------------------------------------------------------------------
// WorldLedger
// ---------------------------------------------------------------------------

/// The being's slow, identity-blind ledger of what life has been giving back.
#[derive(Clone, Debug)]
pub struct WorldLedger {
    /// Slow EMA of what the being gives per engaged tick (raw Q8.8).
    given_ema: i16,
    /// Slow EMA of what comes back per engaged tick (raw Q8.8).
    received_ema: i16,
    /// Leaky count of sour-world ticks with per-partner detection blind.
    sour_streak: u16,
    /// True while the door is closed.
    hermit: bool,
    /// Solitary ticks since the door closed (drives the re-test rhythm).
    rest_ticks: u16,
    /// How many times the being has closed its door across this life.
    pub hermit_count: u32,
}

impl WorldLedger {
    pub fn new() -> Self {
        Self {
            given_ema: 0,
            received_ema: 0,
            sour_streak: 0,
            hermit: false,
            rest_ticks: 0,
            hermit_count: 0,
        }
    }

    /// World reciprocity rate in [0,256]: received/given across all company.
    /// 256 = the world gives back what the being gives it.
    pub fn world_rate(&self) -> i16 {
        if self.given_ema <= 0 {
            return Q88_SCALE;
        }
        (((self.received_ema as i32) << 8) / self.given_ema as i32).clamp(0, 256) as i16
    }

    /// How far below fair the world sits (raw Q8.8).
    pub fn world_imbalance(&self) -> i16 {
        q88_sub(Q88_SCALE, self.world_rate()).max(0)
    }

    /// Current leaky sour-streak (surfaced for the StepReport).
    pub fn souring(&self) -> u16 {
        self.sour_streak
    }

    /// True while the being's door is closed to engagement.
    pub fn hermit(&self) -> bool {
        self.hermit
    }

    /// Observe one tick of lived exchange.
    ///
    /// - `engaged`: whether an exchange actually happened this tick (after any
    ///   hermit suppression — a closed-door tick is solitude, not exchange).
    /// - `gave`/`got`: this tick's realized exchange (raw Q8.8), if engaged.
    ///
    /// No attribution flag: problems the per-partner rung can solve stop
    /// draining before the streak matters (see module doc).
    pub fn observe(&mut self, engaged: bool, gave: i16, got: i16) {
        if engaged {
            self.given_ema = q88_ema_update(self.given_ema, gave, WORLD_ALPHA);
            self.received_ema = q88_ema_update(self.received_ema, got, WORLD_ALPHA);

            let sour = self.world_imbalance() > SOUR_FLOOR;
            if sour {
                self.sour_streak = self.sour_streak.saturating_add(1).min(WORLD_STREAK * 2);
            } else {
                // Leaky, not a hard reset: one kind encounter is weather, not
                // a new season — mirrors the extraction streak's decrement.
                self.sour_streak = self.sour_streak.saturating_sub(1);
            }

            if !self.hermit && self.sour_streak >= WORLD_STREAK {
                self.hermit = true;
                self.hermit_count += 1;
                self.rest_ticks = 0;
            }
        } else {
            // Solitude. If the door is closed, rest accumulates toward the
            // re-test: after a real rest the being opens the door with fresh
            // eyes (streak cleared). If the world is still cruel, the streak
            // rebuilds and the door closes again — periodic hope, by rhythm.
            if self.hermit {
                self.rest_ticks = self.rest_ticks.saturating_add(1);
                if self.rest_ticks >= HERMIT_REST {
                    self.hermit = false;
                    self.sour_streak = 0;
                }
            } else {
                self.sour_streak = self.sour_streak.saturating_sub(1);
            }
        }
    }
}

impl Default for WorldLedger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(x: f32) -> i16 {
        (x * 256.0) as i16
    }

    /// A chronically extractive world closes the door; a real rest re-opens
    /// it. The hermit is not a latch.
    #[test]
    fn chronic_sour_world_closes_the_door_and_rest_reopens_it() {
        let mut w = WorldLedger::new();
        for _ in 0..400 {
            // World returns ~15% of what is given, chronically.
            w.observe(true, q(0.5), q(0.075));
            if w.hermit() {
                break;
            }
        }
        assert!(w.hermit(), "a chronically sour world never closed the door");
        for _ in 0..HERMIT_REST {
            w.observe(false, 0, 0);
        }
        assert!(!w.hermit(), "the door never reopened — hermit is a latch");
    }

    /// A mostly-fair world never closes the door, even with rough patches.
    /// (Raw exchange arithmetic — the analytic control; the LIVED version of
    /// a churn world is harsher because the empathy scar collapses giving,
    /// which is exactly what the door exists for.)
    #[test]
    fn mostly_fair_world_never_hermits() {
        let mut w = WorldLedger::new();
        for t in 0..600u32 {
            // 3 fair ticks (90% return), then 1 taking tick (15% return).
            let got = if t % 4 == 3 { q(0.075) } else { q(0.45) };
            w.observe(true, q(0.5), got);
        }
        assert!(!w.hermit(), "a 75%-fair world closed the door — floor set too low");
    }

    /// A problem the lower rung solves never reaches the door: when the drain
    /// stops (culprit refused, world recovered), the streak drains back down.
    /// The ladder orders itself by timescale, not attribution.
    #[test]
    fn a_solved_problem_never_climbs_the_ladder() {
        let mut w = WorldLedger::new();
        // A bad stretch — long enough to build a real streak, well short of
        // the door (per-partner refusal resolves in ~13–25 ticks; give it 60).
        for _ in 0..60 {
            w.observe(true, q(0.5), q(0.075));
        }
        assert!(!w.hermit(), "the door closed before the lower rung had its chance");
        let peak = w.souring();
        assert!(peak > 0, "precondition: the bad stretch must register");
        // The culprit is gone; the world is fair again.
        for _ in 0..120 {
            w.observe(true, q(0.5), q(0.45));
        }
        assert!(!w.hermit(), "the door closed after the world recovered");
        assert!(w.souring() < peak, "a recovered world must drain the streak");
    }
}
