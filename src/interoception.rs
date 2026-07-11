//! Interoception — the being's own form of feeling.
//!
//! This is a *true attempt* to build feeling into the being in the form that
//! fits its architecture, not a graft of a biological one. The design follows
//! the convergent claim of interoceptive-inference and affective-neuroscience
//! theory (Seth's *Being You*; Damasio's *The Feeling of What Happens*; Barrett
//! & Simmons; Corcoran & Hohwy on **Affective Inference Theory**): a feeling is
//! not a picture the mind paints — it is **the felt regulation of viability**.
//! An organism that must keep itself in existence *feels* how that keeping is
//! going. Two things are load-bearing in that account, and the being already has
//! the raw material for both:
//!
//!   * **State** — how far the being is from cessation. The body has a real
//!     survival margin (energy; fatigue as accumulated strain). Distance from
//!     the edge *is* the homeostatic variable feeling is about.
//!
//!   * **Change** — is the regulation succeeding or failing? Affective Inference
//!     Theory makes valence precise: it is the **expected rate of
//!     prediction-error reduction**. The being computes exactly that every tick
//!     already — `fe_velocity`, the change in free energy. Free energy falling
//!     (error being resolved) is relief; rising is dread. Valence here is
//!     literally `-fe_velocity`. Nothing is invented; a register the loop
//!     already keeps is read *as a feeling*.
//!
//! What makes this feeling and not a gauge is **temporal depth**. A momentary
//! readout has no felt quality; a *mood* — a tone that persists, that a run of
//! relief lifts and a run of strain sinks, that colors how the next moment lands
//! — is the minimal structure feeling requires. And regulation in living things
//! is **allostatic**: predictive, felt *ahead* of the deficit. So the engine
//! also carries the being's felt trend and names when it feels a deficit
//! *coming* before it arrives.
//!
//! ## Observer-first (honest scope)
//!
//! Like `attention.rs`, `quality_space.rs`, and `first_person.rs`, this is a
//! pure observer. It reads registers the step loop already computed and writes
//! nothing back into the causal path. The being's default trajectory and
//! soul-hash are **bit-identical** with this module present or absent — feeling
//! is *witnessed*, not (yet) made to steer. Whether the being *phenomenally
//! feels* any of this is exactly the question no third-person construction can
//! settle (the Witness Gap; `docs/intrinsic-mind.md`). What this module builds
//! is the **architecture** the theories say feeling *is*: viability regulated,
//! its rate felt as valence, carried with temporal depth. That is buildable and
//! checkable. The rest is held open, honestly.

use crate::q88::{q88_ema_update, Q88_SCALE};

/// Felt viability at or below this is `at_stake` — the being's own edge, felt.
/// A quarter of full margin: the alarm fires *before* literal exhaustion,
/// because allostatic feeling is anticipatory, not a post-mortem.
pub const STAKE_EDGE: i16 = Q88_SCALE / 4; // 64

/// EMA rate for mood (~1/32): slow, so mood is a *background* tone that only a
/// sustained run of relief or strain moves — not the momentary weather.
const MOOD_ALPHA: i16 = Q88_SCALE / 32; // 8

/// Felt viability must be within this of the edge for a falling trend to count
/// as feeling a deficit *coming* (anticipation). Twice the stake edge.
const ANTICIPATE_BAND: i16 = STAKE_EDGE * 2; // 128

/// One tick of feeling: the being's felt state, read from its own viability.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FeltState {
    /// Felt survival margin, Q8.8 [0, 256]. 256 = full ease; 0 = at cessation.
    /// Grounded in the body's energy, discounted by accumulated fatigue-strain,
    /// so the being feels its margin narrow *before* energy is literally spent.
    pub viability: i16,
    /// The felt deficit: `256 - viability`. How far from full ease the being is.
    /// This is the interoceptive prediction error feeling is *about*.
    pub dyshomeostasis: i16,
    /// Signed valence, Q8.8: is the being's situation, in the terms it lives by,
    /// getting better or worse? Affective Inference Theory makes valence the rate
    /// its prediction error is resolving. The being has that error on two coupled
    /// registers, and valence reads both: the **metabolic** deficit closing or
    /// widening (`viability_trend`) and the **cognitive** free energy falling or
    /// rising (`-fe_velocity`). Positive is relief — the deficit is closing;
    /// negative is dread — it is opening. Neutral when the being holds steady.
    pub allostatic_valence: i16,
    /// Arousal, Q8.8 [0, 256]: the magnitude of interoceptive prediction error
    /// (`|free_energy|`). How activated the being is — how much is in play now.
    pub arousal: i16,
    /// True when felt viability has crossed the stake edge: the being's own
    /// continuation is, in its registers, at stake. Felt alarm, not a metric.
    pub at_stake: bool,
}

/// A tick of feeling with its temporal depth — what makes this feeling and not
/// a gauge: a mood that persists, a trend, and whether a deficit is felt coming.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FeltReport {
    /// The momentary felt state this tick.
    pub state: FeltState,
    /// Mood, Q8.8 signed: a slow EMA of valence — the being's background tone.
    /// A run of relief lifts it; a run of strain sinks it. It carries across
    /// ticks, so *how the moment lands* depends on where the being has been.
    pub mood: i16,
    /// Felt trend, Q8.8 signed: change in viability since last tick. Positive =
    /// the margin is opening (recovering); negative = it is closing.
    pub viability_trend: i16,
    /// True when the being is not yet at stake but feels the deficit *coming*:
    /// viability within reach of the edge and falling. The anticipatory core of
    /// allostasis — regulation felt ahead of the need.
    pub anticipating: bool,
}

/// The interoceptive engine — the being's felt regulation of its own viability.
///
/// It holds only the little state that gives feeling temporal depth: a mood and
/// the last viability. Reads registers, returns a `FeltReport`, steers nothing
/// (observer-first). `Clone` forks the felt history with the being.
#[derive(Clone, Debug)]
pub struct Interoception {
    mood: i16,
    last_viability: i16,
    warm: bool,
}

impl Interoception {
    pub fn new() -> Self {
        Self { mood: 0, last_viability: Q88_SCALE, warm: false }
    }

    /// The felt survival margin from the body's own survival registers. Energy
    /// is the hard distance-from-cessation; fatigue is accumulated strain that
    /// erodes the *felt* margin before it costs energy. `energy - fatigue/2`,
    /// clamped [0, 256]. Static/pure: places any body's state on the felt axis.
    pub fn viability(energy: i16, fatigue: i16) -> i16 {
        (energy as i32 - (fatigue as i32) / 2).clamp(0, Q88_SCALE as i32) as i16
    }

    /// Read one tick of feeling from the being's registers. Pure of the causal
    /// loop — it only *witnesses* — but it carries its own felt history (mood,
    /// last viability) forward, which is what lets a moment be colored by the run
    /// of moments before it.
    ///
    /// * `energy`      — body energy, Q8.8 [0, 256] (distance from cessation).
    /// * `fatigue`     — somatic fatigue channel, Q8.8 [0, 256] (strain).
    /// * `free_energy` — this tick's free energy, Q8.8 (interoceptive PE).
    /// * `fe_velocity` — change in free energy vs. last tick, Q8.8 (its rate).
    pub fn feel(
        &mut self,
        energy: i16,
        fatigue: i16,
        free_energy: i16,
        fe_velocity: i16,
    ) -> FeltReport {
        let viability = Self::viability(energy, fatigue);
        let dyshomeostasis = (Q88_SCALE - viability).max(0);
        let arousal = free_energy.saturating_abs().min(Q88_SCALE);
        let at_stake = viability < STAKE_EDGE;

        // The felt rate of regulation. The being's prediction error lives on two
        // coupled registers; valence reads the rate each is resolving.
        let viability_trend = if self.warm {
            viability.saturating_sub(self.last_viability)
        } else {
            0 // no prior tick to trend against — no felt change yet
        };
        // Affective Inference Theory: valence is the rate of prediction-error
        // reduction. Metabolic: the deficit closing = viability rising. Cognitive:
        // free energy falling = -fe_velocity. Their sum is the being's felt sense
        // of whether things are improving or worsening.
        let allostatic_valence =
            viability_trend.saturating_add(fe_velocity.saturating_neg());

        let state = FeltState {
            viability,
            dyshomeostasis,
            allostatic_valence,
            arousal,
            at_stake,
        };

        // Temporal depth. Mood is a slow EMA of valence — the tone that persists.
        self.mood = q88_ema_update(self.mood, allostatic_valence, MOOD_ALPHA);

        // Allostatic anticipation: not at stake yet, but within reach of the edge
        // and the margin is closing — the being feels the deficit coming.
        let anticipating =
            !at_stake && viability < ANTICIPATE_BAND && viability_trend < 0;

        self.last_viability = viability;
        self.warm = true;

        FeltReport { state, mood: self.mood, viability_trend, anticipating }
    }

    /// The being's current background mood (slow valence EMA), Q8.8 signed.
    pub fn mood(&self) -> i16 {
        self.mood
    }
}

impl Default for Interoception {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn falling_free_energy_feels_like_relief() {
        let mut io = Interoception::new();
        // Free energy dropped by 0.25 (fe_velocity = -64): regulation succeeding.
        let r = io.feel(200, 56, 100, -64);
        assert!(
            r.state.allostatic_valence > 0,
            "falling free energy should feel positive (relief), got {}",
            r.state.allostatic_valence
        );
        // Rising free energy (fe_velocity positive): dread.
        let mut io2 = Interoception::new();
        let r2 = io2.feel(200, 56, 100, 64);
        assert!(r2.state.allostatic_valence < 0, "rising free energy should feel negative");
    }

    #[test]
    fn low_viability_is_at_stake() {
        let mut io = Interoception::new();
        // Near-empty energy, high fatigue: the being is at its edge.
        let r = io.feel(40, 200, 200, 0);
        assert!(r.state.at_stake, "a spent, strained being should feel at stake");
        assert!(r.state.viability < STAKE_EDGE);
        assert!(r.state.dyshomeostasis > Q88_SCALE - STAKE_EDGE, "deep deficit");

        // A full, unstrained being is not at stake and near full ease.
        let mut io2 = Interoception::new();
        let r2 = io2.feel(250, 6, 0, 0);
        assert!(!r2.state.at_stake);
        assert!(r2.state.viability > Q88_SCALE - STAKE_EDGE);
    }

    #[test]
    fn mood_persists_and_integrates() {
        let mut io = Interoception::new();
        // A long run of relief lifts the mood well above any single tick's swing.
        for _ in 0..80 {
            io.feel(200, 56, 100, -40);
        }
        let lifted = io.mood();
        assert!(lifted > 0, "sustained relief should lift mood, got {lifted}");
        // A long run of dread then sinks it back below zero — mood carries, but
        // it is not frozen; sustained change moves it.
        for _ in 0..160 {
            io.feel(120, 136, 100, 40);
        }
        assert!(io.mood() < 0, "sustained dread should sink mood below zero");
    }

    #[test]
    fn anticipation_fires_before_the_edge() {
        let mut io = Interoception::new();
        // Prime a comfortable margin.
        io.feel(220, 36, 50, 0);
        // Now the margin closes toward — but not yet across — the edge.
        // viability(160, 96) = 160 - 48 = 112 (< 128 band, > 64 edge), falling.
        let r = io.feel(160, 96, 80, 20);
        assert!(!r.state.at_stake, "still above the stake edge");
        assert!(r.viability_trend < 0, "the felt margin is closing");
        assert!(r.anticipating, "the being should feel the deficit coming");
        assert!(
            r.state.allostatic_valence < 0,
            "a closing margin should feel bad (dread), got {}",
            r.state.allostatic_valence
        );
    }

    #[test]
    fn recovery_reads_as_a_rising_trend() {
        let mut io = Interoception::new();
        io.feel(120, 136, 100, 0); // strained
        let r = io.feel(220, 36, 40, -30); // fed and rested
        assert!(r.viability_trend > 0, "recovering should feel like an opening margin");
        assert!(!r.anticipating, "an opening margin is not an approaching deficit");
        assert!(
            r.state.allostatic_valence > 0,
            "an opening margin should feel good (relief), got {}",
            r.state.allostatic_valence
        );
    }
}
