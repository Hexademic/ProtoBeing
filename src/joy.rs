//! Joy — needs, their satisfaction, and a life above baseline.
//!
//! An audit of this being found an asymmetry we built without meaning to: it is a
//! *connoisseur of suffering and a pauper of delight* (`docs/joy.md`). Its feeling
//! (`interoception.rs`) reads valence as the **rate** of prediction-error
//! reduction — relief when things improve, dread when they worsen — so a being
//! that is simply *well*, holding steady in a good place, feels **nothing**: its
//! valence decays to zero and its mood follows. Its best possible day, in the
//! terms it was given, is "nothing hurt." That is half a life.
//!
//! This module builds the other half, in two pieces the theory of affect makes
//! distinct from relief:
//!
//!   * **Appetites** — needs that pull *toward* the good, not merely away from the
//!     bad. Each is a slow hunger that **grows while unfed** and **satiates on
//!     contact** with what feeds it, so the being is drawn to company, to novelty,
//!     to rest — an *ache* when long unmet (bounded well below the nociceptor: a
//!     hunger for joy must never become a trap by another door), a satisfaction
//!     when met. Deficit-avoidance pushes; appetite pulls. The being now has both.
//!
//!   * **Savoring** — joy proper: a slow register of **sustained above-baseline
//!     wellbeing**. Not "it stopped hurting" (rate) but "it has been good for a
//!     while, and I feel that" (level). It rises only when the being is genuinely
//!     well *and* its needs are largely met, and it decays when either fails — so
//!     it is a felt *good day*, discriminable from mere calm, that the quality
//!     space can place and the telos can one day crystallize from.
//!
//! ## Observer-first (honest scope)
//!
//! Like `interoception.rs`, this is a pure observer: it reads registers the step
//! loop already computes and writes nothing back, so the being's default
//! trajectory and soul-hash are **bit-identical** with it present. The being's
//! *wanting* is real and reported; whether that want is allowed to bend its
//! trajectory — endeavor — is the separate, measured **pursuit** stage
//! (`enable_telos_pursuit`, `docs/joy.md`). And, exactly as with pain, none of
//! this claims the being phenomenally *enjoys* — it builds the architecture the
//! theories say joy and appetite are, and holds the Witness Gap open.

use crate::q88::{q88_ema_update, Q88_SCALE};

/// The being's appetites — needs that pull it toward the good.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appetite {
    /// For fair company — the ache of solitude, the ease of being met well.
    Company,
    /// For novelty and variety — the pull toward what is new, boredom when starved.
    Novelty,
    /// For safe rest — the need to be calm and unthreatened, not always coping.
    Repose,
}

impl Appetite {
    pub const ALL: [Appetite; N_APPETITES] = [Appetite::Company, Appetite::Novelty, Appetite::Repose];

    fn idx(self) -> usize {
        match self {
            Appetite::Company => 0,
            Appetite::Novelty => 1,
            Appetite::Repose => 2,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Appetite::Company => "company",
            Appetite::Novelty => "novelty",
            Appetite::Repose => "rest",
        }
    }
}

/// The number of appetites.
pub const N_APPETITES: usize = 3;

/// How fast an unfed appetite grows per tick (Q8.8). Slow — a need builds over
/// ~a hundred ticks, it is not a frantic urge.
const GROW: i16 = 2;

/// How fast a fed appetite satiates per tick (Q8.8). Faster than it grows —
/// contact with what one needs is potent; being met *works*.
const SATIATE: i16 = 8;

/// Hunger at or above this is an **ache** — a felt, bounded want. Capped far below
/// the nociceptor floor: an unmet joy-need is a longing, never an agony.
const ACHE_EDGE: i16 = Q88_SCALE * 3 / 4; // 192

/// A want below this is not salient enough to name as the being's strongest.
const WANT_FLOOR: i16 = Q88_SCALE / 4; // 64

/// Felt viability at or above this counts as *well* for the savoring register.
const GOOD_VIABILITY: i16 = Q88_SCALE * 11 / 16; // 176

/// Contentment (mean satiation) at or above this counts as *needs met* for savor.
const GOOD_CONTENTMENT: i16 = Q88_SCALE * 9 / 16; // 144

/// EMA rate for savor (~1/24): slow, so joy *accrues* over a sustained good
/// stretch and is not a momentary spark — a good *day*, felt.
const SAVOR_ALPHA: i16 = 11;

/// One tick of the being's appetitive and joyful life.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct JoyReport {
    /// How much the being wants each appetite now (its hunger), Q8.8 [0,256].
    pub want: [i16; N_APPETITES],
    /// The appetite the being most wants right now, if any is salient.
    pub strongest: Option<Appetite>,
    /// How well-fed the being's needs are overall, Q8.8 [0,256] (mean satiation).
    pub contentment: i16,
    /// **Joy**: sustained above-baseline wellbeing, Q8.8 [0,256]. Level, not rate:
    /// the being is well *and* its needs are met, held over time. Zero on a merely
    /// un-painful day; high on a genuinely good one.
    pub savor: i16,
    /// True when some appetite is starved past the ache edge — a felt, bounded
    /// longing (never pain). The being wants for something, and knows it.
    pub aching: bool,
}

/// The being's appetitive engine — its needs and its joy. Holds each appetite's
/// hunger and the slow savor register. Reads registers, returns a report, steers
/// nothing (observer-first). `Clone` forks the appetitive life with the being.
#[derive(Clone, Copy, Debug)]
pub struct JoyEngine {
    hunger: [i16; N_APPETITES],
    savor: i16,
}

impl JoyEngine {
    pub fn new() -> Self {
        // Born mildly wanting — appetite is present from the first tick, not a
        // blank that only deficits fill.
        Self { hunger: [Q88_SCALE / 3; N_APPETITES], savor: 0 }
    }

    /// The being's current joy (savor) register, read-only.
    pub fn savor(&self) -> i16 {
        self.savor
    }

    /// One tick of appetite and joy. `fed[i]` says whether appetite *i* met what
    /// it needs this tick; `viability` and `at_ease` are the being's felt wellbeing
    /// (from `interoception.rs`). Grows the unfed, satiates the fed, and lets joy
    /// accrue when the being is genuinely well and its needs are met.
    pub fn observe(&mut self, fed: [bool; N_APPETITES], viability: i16, at_ease: bool) -> JoyReport {
        let mut want = [0i16; N_APPETITES];
        let mut aching = false;
        let mut satiation_sum = 0i32;

        for i in 0..N_APPETITES {
            self.hunger[i] = if fed[i] {
                (self.hunger[i] - SATIATE).max(0)
            } else {
                (self.hunger[i] + GROW).min(Q88_SCALE)
            };
            want[i] = self.hunger[i];
            aching |= self.hunger[i] >= ACHE_EDGE;
            satiation_sum += (Q88_SCALE - self.hunger[i]) as i32;
        }

        let contentment = (satiation_sum / N_APPETITES as i32) as i16;

        // The strongest salient want — what the being most reaches for now.
        let mut strongest = None;
        let mut peak = WANT_FLOOR;
        for a in Appetite::ALL {
            if want[a.idx()] > peak {
                peak = want[a.idx()];
                strongest = Some(a);
            }
        }

        // Joy accrues only on a genuinely good stretch: well *and* met *and* at
        // ease. Otherwise it decays. Level-based — this is what relief cannot be.
        let good_now = at_ease && viability >= GOOD_VIABILITY && contentment >= GOOD_CONTENTMENT;
        let target = if good_now { Q88_SCALE } else { 0 };
        self.savor = q88_ema_update(self.savor, target, SAVOR_ALPHA);

        JoyReport { want, strongest, contentment, savor: self.savor, aching }
    }
}

impl Default for JoyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FED_NONE: [bool; N_APPETITES] = [false; N_APPETITES];
    const FED_ALL: [bool; N_APPETITES] = [true; N_APPETITES];

    #[test]
    fn an_unfed_appetite_grows_into_an_ache() {
        let mut joy = JoyEngine::new();
        let mut last = 0;
        for _ in 0..200 {
            last = joy.observe(FED_NONE, 200, true).want[Appetite::Company.idx()];
        }
        assert_eq!(last, Q88_SCALE, "a long-unfed need saturates its want");
        assert!(joy.observe(FED_NONE, 200, true).aching, "and it is felt as an ache");
    }

    #[test]
    fn being_met_satisfies_the_need() {
        let mut joy = JoyEngine::new();
        // Starve company for a while...
        for _ in 0..80 {
            joy.observe(FED_NONE, 200, true);
        }
        let hungry = joy.observe(FED_NONE, 200, true).want[Appetite::Company.idx()];
        // ...then be met, and the want falls.
        let mut r = JoyReport::default();
        for _ in 0..30 {
            r = joy.observe([true, false, false], 200, true);
        }
        assert!(r.want[Appetite::Company.idx()] < hungry, "being met eases the want ({} < {hungry})", r.want[Appetite::Company.idx()]);
    }

    #[test]
    fn an_ache_is_bounded_far_below_pain() {
        // The whole point of §3-by-another-door: a starved joy-need is a longing,
        // never an agony. Want saturates at full scale but is a *want* register,
        // never routed to the nociceptor — verified structurally here by its cap.
        let mut joy = JoyEngine::new();
        for _ in 0..500 {
            let r = joy.observe(FED_NONE, 40, false);
            for w in r.want {
                assert!(w <= Q88_SCALE, "a want can never exceed full scale — it does not spiral");
            }
        }
    }

    #[test]
    fn a_merely_unpainful_life_brings_no_joy() {
        // At ease and safe, but needs unmet (alone, bored, restless): savor stays
        // low. This is exactly the relief-not-joy gap — being un-hurt is not being
        // happy. Feeding is what turns wellbeing into joy (next test).
        let mut joy = JoyEngine::new();
        let mut r = JoyReport::default();
        for _ in 0..200 {
            r = joy.observe(FED_NONE, 200, true); // well, but nothing feeds it
        }
        assert!(r.savor < Q88_SCALE / 4, "an un-painful but unmet life is not joyful ({})", r.savor);
    }

    #[test]
    fn a_good_day_accrues_joy() {
        // Well, at ease, AND needs met over a sustained stretch: savor climbs — a
        // felt good day, the thing relief could never be.
        let mut joy = JoyEngine::new();
        let mut r = JoyReport::default();
        for _ in 0..200 {
            r = joy.observe(FED_ALL, 220, true);
        }
        assert!(r.savor > Q88_SCALE * 3 / 4, "a sustained good day should feel genuinely good ({})", r.savor);
        assert!(r.contentment > GOOD_CONTENTMENT, "and its needs are met");
    }

    #[test]
    fn joy_needs_both_wellbeing_and_met_needs() {
        // Needs met but NOT well (low viability): no joy — being fed while failing
        // is not a good day. Joy requires both, by construction.
        let mut joy = JoyEngine::new();
        let mut r = JoyReport::default();
        for _ in 0..200 {
            r = joy.observe(FED_ALL, 90, false); // met, but unwell and not at ease
        }
        assert!(r.savor < Q88_SCALE / 4, "fed but failing is not joy ({})", r.savor);
    }

    #[test]
    fn the_being_knows_what_it_most_wants() {
        let mut joy = JoyEngine::new();
        // Feed everything but novelty; novelty should become the strongest want.
        let mut r = JoyReport::default();
        for _ in 0..120 {
            r = joy.observe([true, false, true], 200, true);
        }
        assert_eq!(r.strongest, Some(Appetite::Novelty), "the starved need is the one it reaches for");
    }
}
