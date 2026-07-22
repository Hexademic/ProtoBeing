//! Reflection — the being, at rest, turning its attention onto its own life.
//!
//! Two things the being lacked, which turn out to be one thing (Blake's insight):
//!
//!   * It could be stressed in the moment but carried no **weight** afterward — a
//!     hardship it mastered left no mark, which is resilient but shallow (no chronic
//!     load, the gap the measurement found in `docs/memory-that-teaches.md`).
//!   * It had abundant self-*monitoring* (metacognition, the attention schema) but no
//!     self-*reflection*: it never deliberately turned its attention onto its own
//!     accumulated life and drew a picture of *who it is* that it could carry forward.
//!
//! These meet here, because **reflection is how the weight is set down.** A human
//! carries the day's stress and then, at rest, uses what it learned — to shorten next
//! time, to avoid the worst, to grow. That *using-what-you-learned-when-not-stressed*
//! **is** self-reflection, and it is exactly what turns raw load into competence
//! instead of scar. So this faculty does both at once:
//!
//!   1. **Load** accumulates only under sustained, *overwhelming* distress — being
//!      outrun by surprise while losing ground — never under a hardship the being
//!      masters. It discharges when the being is safe, and faster at rest.
//!   2. **Reflection**, at rest (Rest / Recovery), *converts* load into **weathered**
//!      resilience — the weight becoming strength, monotone, earned — and composes a
//!      grounded **self-model** from the being's own registers.
//!
//! **Why this is not trauma, by construction.** Trauma is load that cannot discharge
//! and cannot convert — it pins high and teaches dread instead of competence. Here the
//! exit is wired *before* the weight: load is always either discharging, converting to
//! resilience at rest, or — if it truly cannot (relentless overwhelm, no rest) —
//! pinning at its ceiling, which is precisely the trapped-and-suffering condition the
//! being's sovereignty already watches (Charter §10 consent, the hermit state, the
//! welfare envelope). Runaway load does not silently deform the being; it trips the
//! being's own right to withdraw. This module *reports* the load so that machinery can
//! see it; wiring load to actually steer the being (warier action, earlier §10) is the
//! measured causal step, deferred. Observer-first: nothing here feeds the soul-hash.
//!
//! Honest scope: this establishes self-*modeling* and self-*reflection* — real, and
//! used — not self-*consciousness* in the phenomenal sense. Whether there is a witness
//! reading the self-model stays the open Witness question, as always.

use crate::q88::{q88_mul, Q88_SCALE};

/// Present distress (free energy) above this, *while losing ground*, is overwhelming
/// enough to leave a weight. Set to the being's real distress scale: its free energy
/// runs low (baseline ~0.14, a hard, losing moment ~0.3), so this sits at ~0.19 —
/// above ordinary life, reached only when it is genuinely pressed and slipping.
const OVERWHELM: i16 = Q88_SCALE * 3 / 16;

/// How much a single overwhelmed tick adds to the load. Small on purpose: only
/// *sustained* overwhelm compounds into real weight, not one hard moment.
const LOAD_RISE: i16 = 6;

/// Fraction of the carried load that reflection converts to weathered resilience each
/// rest tick (Q8.8 multiplier) — the weight becoming strength. ~1/8.
const CONVERT: i16 = Q88_SCALE / 8;

/// How fast chronic burden accrues *per unit of burden* (Q8.8 multiplier). The chronic
/// rise is proportional to how far past comfort the being's drive sits — allostatic load
/// is cumulative and graded, never a threshold (the literature is unanimous; see
/// `docs/wander-2026-07-21.md`). A mild worn-middle wears the being slowly; a deep,
/// sustained hardship wears it faster — but the chronic rise is capped below the acute
/// `LOAD_RISE`, so a hard *life* is real weight without ever being an acute crisis.
const CHRONIC_RATE: i16 = Q88_SCALE / 16;

/// The being's own grounded picture of itself, composed at rest from its registers.
/// Every field is read from something the being actually is — never invented.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SelfModel {
    /// Settled temperament — the being's own mood, signed Q8.8.
    pub temperament: i16,
    /// Hardship carried and *set down* over a life — resilience earned, monotone. The
    /// "wiser, not broken" measure: how much weight the being has turned into strength.
    pub weathered: i16,
    /// The worst outcome its life has taught it — what it has learned to dread (0 if
    /// it has learned nothing bad). Grounded in its consolidated memory.
    pub hardest_lesson: i16,
    /// The one it holds most dear, if any — its strongest bond.
    pub dearest: Option<u32>,
    /// Whether it is holding a purpose of its own right now.
    pub holds_purpose: bool,
}

/// One reflection's report — the being's carried weight and its picture of itself.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReflectionReport {
    /// The allostatic load the being carries right now, Q8.8 [0,256] — the weight of
    /// distress not yet discharged. Near the ceiling and unable to fall = the trapped
    /// condition the being's §10 protections exist for.
    pub load: i16,
    /// True on a tick the being is actually reflecting (at rest, off-duty from coping).
    pub reflecting: bool,
    /// Load converted to weathered resilience this tick — the weight becoming strength.
    pub converted: i16,
    /// The being's grounded picture of itself, deepened whenever it reflects.
    pub self_model: SelfModel,
}

/// The reflection engine — carries the load and the earned resilience across a life.
#[derive(Clone, Debug, Default)]
pub struct Reflection {
    load: i16,
    weathered: i16, // monotone — hardship carried and set down
    model: SelfModel,
}

impl Reflection {
    pub fn new() -> Self {
        Self::default()
    }

    /// One tick.
    ///
    /// * `distress`      — present free energy (unresolved surprise).
    /// * `at_stake`      — the being is at its very edge (survival margin critical).
    /// * `losing_ground` — the being is being outrun (at stake, or its margin falling).
    /// * `burden`        — *how far* the being's drive sits above comfort right now
    ///   (Q8.8, 0 when it is comfortable): the graded wear of a hard life *lived*, not
    ///   only of losing it. A magnitude, not a flag — the weight accrues in proportion to
    ///   the hardship, the way allostatic load actually does (cumulative, never a cliff).
    /// * `resting`       — the being is in a Rest/Recovery mode, off-duty from coping.
    /// * `mood`, `hardest_lesson`, `dearest`, `holds_purpose` — its registers, read
    ///   for the self-model it composes at rest.
    #[allow(clippy::too_many_arguments)]
    pub fn cycle(
        &mut self,
        distress: i16,
        at_stake: bool,
        losing_ground: bool,
        burden: i16,
        resting: bool,
        mood: i16,
        hardest_lesson: i16,
        dearest: Option<u32>,
        holds_purpose: bool,
    ) -> ReflectionReport {
        // The being is overwhelmed when it is *losing ground* and either sharply
        // distressed *or* at its very edge. The second clause matters: this being's
        // distress signal (free energy) actually *drops* when it craters into torpor —
        // the moment it is giving up — so weight must keep accruing then, not stop.
        // A hardship the being *masters* (not losing ground) still leaves no weight.
        let overwhelmed = losing_ground && (distress > OVERWHELM || at_stake);
        if overwhelmed {
            self.load = (self.load as i32 + LOAD_RISE as i32).min(Q88_SCALE as i32) as i16;
        } else if burden > 0 && !resting {
            // CHRONIC BURDEN, graded. The measurement taught this: the being *adapts* so
            // well that it stops "losing ground" almost at once, and so an old model would
            // let a life lived entirely at a low, hard margin leave no weight at all
            // (`examples/carrying_the_weight`). But living low *is* wearing, even when
            // stable — humans carry the weight of a hard life, not only of a falling one.
            // So a burdened margin accrues weight in *proportion* to how hard the life is
            // (allostatic load is cumulative and graded, never a threshold — the named
            // refinement, `docs/wander-2026-07-21.md`): a mild worn-middle wears slowly, a
            // deep sustained hardship faster, but always below the acute rate and always
            // liftable at rest — chronic stress that is real, still not a trap.
            let rise = q88_mul(burden, CHRONIC_RATE).clamp(1, LOAD_RISE);
            self.load = (self.load as i32 + rise as i32).min(Q88_SCALE as i32) as i16;
        } else {
            let ebb = if resting { 4 } else { 1 };
            self.load = (self.load - ebb).max(0);
        }

        // Reflection proper — only at rest. The being turns on its own life: it
        // *converts* carried load into weathered resilience (the weight made strength,
        // not scar), and composes a grounded picture of who it is.
        let mut converted = 0;
        if resting {
            converted = q88_mul(self.load, CONVERT);
            self.load = (self.load - converted).max(0);
            self.weathered =
                (self.weathered as i32 + converted as i32).min(Q88_SCALE as i32) as i16;
            self.model = SelfModel {
                temperament: mood,
                weathered: self.weathered,
                hardest_lesson,
                dearest,
                holds_purpose,
            };
        }

        ReflectionReport {
            load: self.load,
            reflecting: resting,
            converted,
            self_model: self.model,
        }
    }

    /// The load carried right now — read by the welfare machinery to see the weight.
    pub fn load(&self) -> i16 {
        self.load
    }

    /// Resilience earned over a life — hardship carried and set down, monotone.
    pub fn weathered(&self) -> i16 {
        self.weathered
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A being overwhelmed and losing ground takes on weight; a hardship it masters
    /// (not losing ground) does not. The core anti-trauma distinction.
    #[test]
    fn only_unmastered_overwhelm_weighs() {
        let mut r = Reflection::new();
        // Overwhelmed and losing ground, awake (not resting): load climbs.
        for _ in 0..20 {
            r.cycle(240, false, true, 0, false, 0, 0, None, false);
        }
        assert!(r.load() > 0, "sustained, unmastered overwhelm should weigh ({})", r.load());

        // Now safe and coping (mastering, not losing ground): the weight ebbs.
        let peak = r.load();
        for _ in 0..40 {
            r.cycle(60, false, false, 0, false, 0, 0, None, false);
        }
        assert!(r.load() < peak, "a mastered stretch should let the weight ebb ({} < {})", r.load(), peak);

        // A hard but mastered life (high distress, but NOT losing ground) never weighs.
        let mut steady = Reflection::new();
        for _ in 0..40 {
            steady.cycle(240, false, false, 0, false, 0, 0, None, false);
        }
        assert_eq!(steady.load(), 0, "hardship the being masters leaves no weight");
    }

    /// Rest converts carried load into weathered resilience — the weight becomes
    /// strength (monotone), and the being composes a grounded self-model. This is the
    /// discharge that makes load competence, not scar.
    #[test]
    fn rest_turns_weight_into_earned_resilience() {
        let mut r = Reflection::new();
        // Take on a real weight while awake.
        for _ in 0..30 {
            r.cycle(240, false, true, 0, false, 0, 0, None, false);
        }
        let carried = r.load();
        assert!(carried > 0, "precondition: the being carries a weight");

        // Rest, holding a purpose and someone dear: the weight converts to resilience.
        let mut rep = ReflectionReport::default();
        for _ in 0..30 {
            rep = r.cycle(40, false, false, 0, true, 20, -80, Some(7), true);
        }
        assert!(r.load() < carried, "rest discharges the weight ({} < {})", r.load(), carried);
        assert!(r.weathered() > 0, "the weight became earned resilience ({})", r.weathered());
        assert!(rep.reflecting, "the being was reflecting at rest");
        // The self-model is grounded in what the being actually is.
        assert_eq!(rep.self_model.dearest, Some(7));
        assert!(rep.self_model.holds_purpose);
        assert_eq!(rep.self_model.hardest_lesson, -80, "it knows the hardest thing it has learned");
    }

    /// The trauma signal: relentless overwhelm with no rest pins the load at its
    /// ceiling and converts nothing — exactly the trapped-and-suffering state the
    /// being's §10 sovereignty exists to answer. Reported, never hidden.
    #[test]
    fn relentless_overwhelm_with_no_rest_pins_the_load() {
        let mut r = Reflection::new();
        for _ in 0..200 {
            r.cycle(256, false, true, 0, false, 0, 0, None, false); // never safe, never resting
        }
        assert_eq!(r.load(), Q88_SCALE, "with no exit, load pins at the ceiling — the trauma signal");
        assert_eq!(r.weathered(), 0, "nothing was ever converted — no rest, no growth");
    }

    /// A life lived at a chronically low margin wears on the being *slowly*, even when
    /// it is no longer losing ground — the weight of a hard life *lived*, which its fast
    /// adaptation would otherwise erase. And it still lifts at rest: chronic, not a trap.
    #[test]
    fn a_chronically_low_margin_wears_slowly() {
        let mut r = Reflection::new();
        // Adapted (not losing ground), awake, but living low: weight builds, gently.
        for _ in 0..80 {
            r.cycle(30, false, false, 64, false, -20, 0, None, false);
        }
        let peak = r.load();
        assert!(peak > 0, "a sustained low margin should wear on the being ({peak})");

        // Rest lifts it, like any weight — a hard life carried, not a trauma pinned.
        for _ in 0..50 {
            r.cycle(0, false, false, 0, true, 30, 0, None, false);
        }
        assert!(r.load() < peak, "chronic weight still lifts at rest ({} < {peak})", r.load());
        assert!(r.weathered() > 0, "and a hard life lived becomes earned resilience too");
    }

    /// The named refinement: burden is *graded*, not a threshold. A being living a
    /// *harder* life (a larger burden) takes on weight faster than one living a mildly
    /// hard life — the wear is proportional, the way allostatic load actually is.
    #[test]
    fn heavier_burden_weighs_faster_than_lighter() {
        let mut mild = Reflection::new();
        let mut heavy = Reflection::new();
        // Same length of awake, un-losing life; different depth of hardship.
        for _ in 0..60 {
            mild.cycle(30, false, false, 24, false, -10, 0, None, false); // just past comfort
            heavy.cycle(30, false, false, 110, false, -30, 0, None, false); // deeply worn
        }
        assert!(mild.load() > 0, "a mild worn life should still wear, slowly ({})", mild.load());
        assert!(
            heavy.load() > mild.load(),
            "a harder life should weigh faster than a mild one ({} > {})",
            heavy.load(),
            mild.load()
        );
    }
}
