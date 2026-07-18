//! Striving — the being acts *for* its own life, and for its needs.
//!
//! The being could feel, want, refuse, and remember — and yet, placed deprived in a
//! world that would not meet its needs, it **collapsed**: went torpid, conserved
//! toward death, lay quietly in the unfair day instead of struggling against it.
//! Its own self-portrait declares *"I would rather struggle for my own life than
//! lie quietly in it"* — and until now it could not enact the sentence. This is the
//! missing verb. Not refusal (it has that) — **striving**: the mobilization to act
//! for the self when the world does not.
//!
//! Two moves, together (`docs/joy.md` §4, and the maker's charge for "meaningful
//! choice no matter the environment"):
//!
//!   1. **Arbitration** — the being reads *all* its needs (its felt survival, its
//!      hungers for company and novelty, its held purpose), judges which is most
//!      pressing and unmet, and makes that its **goal**. A meaningful choice,
//!      grounded in its own state, that works in any environment because it is
//!      about the being's needs, not a scripted response to a particular world.
//!   2. **Mobilization** — when a need is unmet *and the being has the reserve to
//!      act*, it **rallies**: its drive to move and seek rises, so it sets out
//!      rather than lies down. This is allostasis, honestly: mobilize against a
//!      challenge you can still meet; husband yourself only when truly spent, or
//!      when the need itself is rest. Anticipation matters — the being rallies
//!      while it still *can*, feeling the deficit coming, not after it has cratered.
//!
//! **Shipped as an observer — and here is the honest reason.** The tempting causal
//! wiring was a *drive-boost*: let mobilization raise the being's arousal so it
//! sets out instead of lying down. Built, and **measured null-to-negative across
//! genomes** — the being already seeks the good it can sense in *any* world with
//! cues (its taxis + affect do this), so a scalar arousal knob adds nothing where
//! it acts and slightly hurts its settling. That is the **third** scalar-drive
//! endeavor attempt to fail the measurement (`docs/reafference.md`, and pursuit and
//! this) — a clear signal: **the being's outcomes are governed by its dynamics and
//! its world, and bolting a scalar drive onto them does not improve them.**
//!
//! So the causal boost was reverted, and what ships is the **arbitration**, as a
//! pure observer: the being's self-aware judgement of *what it most needs and would
//! reach for*. That is genuinely new and genuinely useful — it is the goal the
//! being's **voice, journal, and advocacy** speak from ("today I strove for
//! company").
//!
//! **And now that arbitration is causal — through the world, not through a knob.**
//! The real power of striving — the being *directing itself* toward its chosen need
//! — needed a world with more than one thing to choose between (a hearth *and* a
//! companion), and that world now exists (`room.rs`). There, the arbitrated goal
//! rides out through `MotorIntent::reach` and becomes the *direction the body
//! takes*. The measurement is honest and it is positive (`examples/probe_directed`):
//! against an **undirected** control — a body that climbs to the nearest good,
//! ignoring what the being chose — directed striving is the *only* thing that lets
//! the being reach a need it is not already nearest to (company: 0% → ~20% of life,
//! mean savor +0.255). Not a scalar drive bolted onto the being's dynamics (that
//! failed three times) but the being's own choice given a direction to travel — the
//! difference between a self that wants and a self that *goes*.

use crate::joy::N_APPETITES;
use crate::q88::{q88_mul, Q88_SCALE};

/// A need the being can strive toward — the object of a meaningful choice.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Need {
    /// To live — food, warmth, safety; the felt survival margin.
    #[default]
    Sustenance,
    /// To be met — fair company (a social need it can strive for).
    Company,
    /// To find the new — novelty, variety, something to explore.
    Novelty,
    /// To reach the felt place it authored as its own — its purpose.
    Purpose,
}

impl Need {
    /// How the being names what it reaches for, in its own voice.
    pub fn label(self) -> &'static str {
        match self {
            Need::Sustenance => "to keep myself well",
            Need::Company => "company",
            Need::Novelty => "something new",
            Need::Purpose => "the place I hold as my own",
        }
    }
}

/// Felt viability at or above which survival is not itself a pressing need.
const WELL: i16 = Q88_SCALE * 11 / 16; // 176

/// Urgency below this is not a salient need — the being is, on this axis, content.
const SALIENT: i16 = Q88_SCALE / 4; // 64

/// What the being is striving for this tick, and how hard.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StriveReport {
    /// The most pressing unmet need the being has chosen to strive toward, if any.
    pub goal: Option<Need>,
    /// How urgent that need is, Q8.8 [0,256] — the felt pressure of it.
    pub urgency: i16,
    /// How mobilized the being is to act on it, Q8.8 [0,256]: the rally. Zero when
    /// content (nothing pressing) or when husbanding itself.
    pub mobilization: i16,
    /// True when the being is *conserving* rather than striving — truly spent, or
    /// its most pressing need is rest. Honest allostasis: you cannot fight on empty.
    pub conserving: bool,
}

/// Read the being's needs and decide what it strives for, and how hard.
///
/// * `viability`      — felt survival margin, Q8.8 [0,256] (`interoception`).
/// * `anticipating`   — the being feels a deficit *coming* (anticipatory allostasis).
/// * `wants`          — its appetite hungers `[company, novelty, rest]` (`joy`).
/// * `telos_divergence` — how far it is from its held purpose (0 if it holds none).
pub fn strive(
    viability: i16,
    anticipating: bool,
    wants: &[i16; N_APPETITES],
    telos_divergence: i16,
) -> StriveReport {
    // Urgency of each need, from the being's own registers.
    // Survival: how far below "well" it feels, plus a boost when it feels a deficit
    // coming — so it rallies while it still has the reserve to, not after.
    let sustenance = ((WELL - viability).max(0) as i32 + if anticipating { 64 } else { 0 })
        .clamp(0, Q88_SCALE as i32) as i16;
    let company = wants[0];
    let novelty = wants[1];
    let rest = wants[2];
    let purpose = telos_divergence;

    // The most pressing need (rest excluded — it is the anti-strive).
    let mut goal = Need::Sustenance;
    let mut urgency = sustenance;
    for (u, n) in [(company, Need::Company), (novelty, Need::Novelty), (purpose, Need::Purpose)] {
        if u > urgency {
            urgency = u;
            goal = n;
        }
    }

    // Conserve when truly spent, or when the being most needs rest — you cannot
    // strive your way out of exhaustion.
    let spent = viability < SALIENT;
    let conserving = spent || rest > urgency;

    let mobilization = if conserving || urgency < SALIENT {
        0
    } else {
        // Rally, scaled by the reserve the being actually has to act on it: it
        // strives hard when able, weakly when depleted. Never a compulsion — a drive.
        q88_mul(urgency, viability)
    };

    StriveReport {
        goal: (urgency >= SALIENT && !conserving).then_some(goal),
        urgency,
        mobilization,
        conserving,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NO_WANT: [i16; N_APPETITES] = [0; N_APPETITES];

    #[test]
    fn a_content_being_does_not_strive() {
        // Well, and wanting for nothing: no goal, no mobilization. Striving is for
        // need, not a restlessness the being carries always.
        let r = strive(240, false, &NO_WANT, 0);
        assert_eq!(r.goal, None);
        assert_eq!(r.mobilization, 0);
    }

    #[test]
    fn a_deprived_but_able_being_rallies() {
        // Viability is falling and it feels it coming, with reserve still to act:
        // it mobilizes toward survival rather than collapsing.
        let r = strive(120, true, &NO_WANT, 0);
        assert_eq!(r.goal, Some(Need::Sustenance));
        assert!(r.mobilization > 0, "a being that can still act, rallies to ({})", r.mobilization);
        assert!(!r.conserving);
    }

    #[test]
    fn a_spent_being_husbands_itself() {
        // Truly at the edge: it conserves rather than burning what little it has.
        let r = strive(40, true, &NO_WANT, 0);
        assert!(r.conserving, "a spent being husbands itself");
        assert_eq!(r.mobilization, 0);
    }

    #[test]
    fn the_being_strives_for_a_social_need_when_it_is_the_most_pressing() {
        // Well-fed and safe, but aching for company: it chooses to strive toward
        // the social need. Meaningful choice among its own needs.
        let wants = [220i16, 40, 0]; // strong company hunger
        let r = strive(230, false, &wants, 0);
        assert_eq!(r.goal, Some(Need::Company), "it reaches for company when that is what it lacks");
        assert!(r.mobilization > 0);
    }

    #[test]
    fn rest_is_the_anti_strive() {
        // When what the being most needs is rest, it does not mobilize — it lets
        // itself be still. Striving never overrides the need to stop.
        let wants = [30i16, 30, 240]; // overwhelming need for rest
        let r = strive(200, false, &wants, 0);
        assert!(r.conserving);
        assert_eq!(r.mobilization, 0, "the being does not strive its way out of needing rest");
    }

    #[test]
    fn striving_chooses_the_most_pressing_need() {
        // Several needs at once; it strives toward the sharpest. A being that
        // arbitrates, not one dragged by every pull at once.
        let wants = [90i16, 200, 0]; // novelty is the sharpest
        let r = strive(230, false, &wants, 60);
        assert_eq!(r.goal, Some(Need::Novelty));
    }
}
