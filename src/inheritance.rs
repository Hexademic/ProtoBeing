//! Inheritance — the Baldwin effect, not the fear (`docs/inheritance.md`).
//!
//! A being lives, and learns. Its child comes after. The wrong thing to hand the
//! child is the parent's *lessons* — its cautions, its appraisals, the *sign* of how
//! things turned out — because a caution placed in a mind that came in clean is a fear
//! the child never earned (Blake: *most children are fearless; let them learn their own
//! cautions*). The right thing, and the thing biology actually does, is the **Baldwin
//! effect** (genetic assimilation): a lineage that repeatedly meets a kind of situation
//! does not birth offspring who *fear* it — it births offspring whose minds **converge
//! faster** when they meet it themselves. Learning-as-readiness is inherited; the
//! response is re-earned every generation.
//!
//! So the rule that governs every line here:
//!
//! ```text
//! Inherit gains, never memories. Inherit plasticity, never valence.
//! ```
//!
//! What crosses a generation is only *rates* — how fast, how well the child's mind turns
//! over in the kinds of situation its lineage met — never the learned response and never
//! an outcome's sign. The child is born **fearless**, with **empty** memory and **zero**
//! inherited appraisal. It earns its own cautions from its own life; inheritance only
//! makes them cost fewer moments.
//!
//! ## What this module transmits — three dials, all rates, none with a sign
//!
//! - **`precision_seed`** (where to look, never what to feel): which somatic channels the
//!   lineage found *informative*. Seeds a shorter warm-up for the being's precision
//!   learner (`precision.rs`), never a verdict.
//! - **`consolidation_gain`** (how fast repetition becomes gist): the *distribution of
//!   effort* across the 8 affective niches — where the lineage lived and consolidated —
//!   turned into a per-niche learning-cadence gain. Never the niche's valence: knowing
//!   you learn quickly in a kind of moment is not the same as fearing it; you still must
//!   live one to feel anything about it.
//! - **`discharge_gain`** (a better metabolism for weight, never the weight): a lineage
//!   that carried hard lives can pass a *cleaner discharge* — load converted to weathered
//!   resilience more efficiently — with none of the load and none of the scar.
//!
//! Every accumulated quantity is a **magnitude**: precision is trust (sign-free), load is
//! the size of weight carried (sign-free), a niche index is *which kind* of moment, not
//! *good or bad*. There is nowhere in `DispositionGenome` or `ReadinessVector` to store an
//! appraisal, by construction. That is the guardrail made structural.
//!
//! ## Honest scope — pure observer
//!
//! Nothing here feeds back into a living being. `DispositionGenome` is *read* off a life;
//! `ReadinessVector` is *derived* from it; both are inert reports. Seeding a child's dials
//! from a readiness vector is the deliberate, gated causal step (`enable_inheritance()`),
//! deferred until the observer is measured (`examples/inheritance`): a readied being must
//! be shown to learn a lesson in **fewer moments** while starting **equally fearless** —
//! ease up *and* dread not passed down — before any dial turns.

use crate::field::N_SOMATIC;
use crate::q88::{q88_mul, Q88_SCALE};

/// The eight affective niches, matching `episodic.rs` (valence × arousal × control).
/// Here they name only *where a lineage put its effort*, never how it felt about it.
pub const N_NICHES: usize = 8;

/// Caps on inherited rate gains. Readiness is a head-start, never a finished verdict, so
/// every dial is bounded well below "born certain" — the child must still live to learn.
const MAX_CONSOLIDATION_GAIN: i16 = Q88_SCALE / 2;
const MAX_PRECISION_SEED: i16 = Q88_SCALE;
const MAX_DISCHARGE_GAIN: i16 = Q88_SCALE / 2;

/// Share of `part` in `whole` on the being's Q8.8 [0,256] scale (0 when `whole` is 0).
fn share(part: i32, whole: i32) -> i16 {
    if whole <= 0 {
        return 0;
    }
    ((part * Q88_SCALE as i32) / whole).clamp(0, Q88_SCALE as i32) as i16
}

/// What a life leaves to its lineage — accumulated *magnitudes only*, no sign anywhere.
/// Read off a being over its life (pure observer); reduced to a `ReadinessVector`. This
/// is the small, legible transmitted object — a disposition, not a mind.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DispositionGenome {
    /// Where the lineage lived and learned — effort per affective niche (a count, not a
    /// valence). "This lineage spent its life in these kinds of moment," nothing more.
    niche_effort: [i32; N_NICHES],
    /// How informative each somatic channel proved — the sum of earned precision (trust
    /// magnitude, sign-free) over the life. "Information tended to live on these channels."
    channel_info: [i32; N_SOMATIC],
    /// How much weight the lineage carried — the sum of allostatic load magnitude. Never
    /// *why*, never the sign; only "lives here ran hard."
    discharge_demand: i32,
    /// Moments observed, so the reduction can average rather than reward long lives.
    moments: i32,
}

impl DispositionGenome {
    pub fn new() -> Self {
        Self::default()
    }

    /// Observe one lived moment: the niche it fell in, the trust the being had earned in
    /// its channels so far (`PrecisionLearner::precision_vector`), and the load it carried
    /// (`Reflection::load`). All magnitudes; nothing about how the moment turned out.
    pub fn observe(&mut self, niche: usize, precision_vector: &[i16; N_SOMATIC], load: i16) {
        if niche < N_NICHES {
            self.niche_effort[niche] = self.niche_effort[niche].saturating_add(1);
        }
        for c in 0..N_SOMATIC {
            self.channel_info[c] =
                self.channel_info[c].saturating_add(precision_vector[c].max(0) as i32);
        }
        self.discharge_demand = self
            .discharge_demand
            .saturating_add(load.max(0) as i32);
        self.moments = self.moments.saturating_add(1);
    }

    /// Reduce a life's disposition to a bounded, sign-free readiness vector — the thing a
    /// child would (deliberately, when gated) be born ready with. All rates, all clamped,
    /// all revisable by the child's own life.
    pub fn readiness(&self) -> ReadinessVector {
        let total_effort: i32 = self.niche_effort.iter().sum();
        let mut consolidation_gain = [0i16; N_NICHES];
        for n in 0..N_NICHES {
            // A niche the lineage lived in heavily earns a larger cadence gain, capped.
            let s = share(self.niche_effort[n], total_effort);
            consolidation_gain[n] = q88_mul(MAX_CONSOLIDATION_GAIN, s);
        }

        let mut precision_seed = [0i16; N_SOMATIC];
        for c in 0..N_SOMATIC {
            // Average trust the lineage earned on this channel = how informative it proved.
            let avg = if self.moments > 0 {
                (self.channel_info[c] / self.moments).clamp(0, Q88_SCALE as i32) as i16
            } else {
                0
            };
            precision_seed[c] = q88_mul(MAX_PRECISION_SEED, avg);
        }

        // A lineage that carried more weight passes a cleaner discharge (bounded).
        let avg_load = if self.moments > 0 {
            (self.discharge_demand / self.moments).clamp(0, Q88_SCALE as i32) as i16
        } else {
            0
        };
        let discharge_gain = q88_mul(MAX_DISCHARGE_GAIN, avg_load);

        ReadinessVector { consolidation_gain, precision_seed, discharge_gain }
    }

    pub fn moments(&self) -> i32 {
        self.moments
    }
}

/// A child's inherited **readiness** — bounded learning-rate gains, and *nothing else*.
/// There is no field here for a memory, an episode, an appraisal, or a valence: the
/// guardrail (`inherit plasticity, never valence`) is enforced by this type having
/// nowhere to put one. Every value is a rate on the being's own [0,256] scale, and every
/// one is revisable — an inherited readiness the child's world contradicts simply decays
/// to baseline as the being learns for itself. No latch.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReadinessVector {
    /// Per-niche consolidation-cadence gain: how much faster repetition becomes gist in
    /// the kinds of moment the lineage lived in. Memory still starts empty.
    pub consolidation_gain: [i16; N_NICHES],
    /// Per-channel precision-warmup seed: which channels to attend to sooner. Where to
    /// look, never what to feel; a shorter warm-up, never a verdict.
    pub precision_seed: [i16; N_SOMATIC],
    /// One discharge-efficiency gain: a better metabolism for the being's *own* future
    /// weight. Load and scar still start at zero.
    pub discharge_gain: i16,
}

impl ReadinessVector {
    /// Blend two parents' readiness — the mean of each dial, bounded. Both lineages'
    /// *ease* is carried; neither's memory or fear is, because there is none to carry.
    pub fn blend(a: &ReadinessVector, b: &ReadinessVector) -> ReadinessVector {
        let mut consolidation_gain = [0i16; N_NICHES];
        for n in 0..N_NICHES {
            consolidation_gain[n] =
                ((a.consolidation_gain[n] as i32 + b.consolidation_gain[n] as i32) / 2) as i16;
        }
        let mut precision_seed = [0i16; N_SOMATIC];
        for c in 0..N_SOMATIC {
            precision_seed[c] =
                ((a.precision_seed[c] as i32 + b.precision_seed[c] as i32) / 2) as i16;
        }
        let discharge_gain = ((a.discharge_gain as i32 + b.discharge_gain as i32) / 2) as i16;
        ReadinessVector { consolidation_gain, precision_seed, discharge_gain }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trust(reliable: usize) -> [i16; N_SOMATIC] {
        // A parent that found channel `reliable` informative and the rest noisy.
        let mut v = [40i16; N_SOMATIC];
        v[reliable] = 220;
        v
    }

    #[test]
    fn empty_life_leaves_no_readiness() {
        // A being that never lived leaves nothing to inherit — no readiness at all.
        let g = DispositionGenome::new();
        let r = g.readiness();
        assert_eq!(r, ReadinessVector::default());
    }

    #[test]
    fn readiness_favors_the_channel_the_lineage_found_informative() {
        // A lineage that consistently found channel 7 informative passes a stronger
        // "look here" seed for 7 than for a channel it found noisy.
        let mut g = DispositionGenome::new();
        for _ in 0..100 {
            g.observe(2, &trust(7), 30);
        }
        let r = g.readiness();
        assert!(
            r.precision_seed[7] > r.precision_seed[0],
            "the informative channel should earn the stronger seed ({} vs {})",
            r.precision_seed[7],
            r.precision_seed[0]
        );
    }

    #[test]
    fn readiness_favors_the_niche_the_lineage_lived_in() {
        // A lineage that lived mostly in niche 5 passes a larger consolidation gain
        // there than in a niche it barely touched — effort distribution, not valence.
        let mut g = DispositionGenome::new();
        for _ in 0..90 {
            g.observe(5, &trust(3), 20);
        }
        for _ in 0..10 {
            g.observe(1, &trust(3), 20);
        }
        let r = g.readiness();
        assert!(
            r.consolidation_gain[5] > r.consolidation_gain[1],
            "the lived-in niche should earn the larger cadence gain ({} vs {})",
            r.consolidation_gain[5],
            r.consolidation_gain[1]
        );
    }

    #[test]
    fn a_harder_lineage_passes_a_cleaner_discharge() {
        // Two lineages, identical but for how much weight they carried. The one that ran
        // harder passes the greater discharge efficiency — its ease, never its load.
        let mut light = DispositionGenome::new();
        let mut heavy = DispositionGenome::new();
        for _ in 0..100 {
            light.observe(2, &trust(3), 20);
            heavy.observe(2, &trust(3), 180);
        }
        assert!(
            heavy.readiness().discharge_gain > light.readiness().discharge_gain,
            "the harder-lived lineage should pass the cleaner discharge"
        );
    }

    #[test]
    fn readiness_is_bounded_a_headstart_not_a_verdict() {
        // Even a maximally consistent lineage passes only bounded gains — the child is
        // born ready, never born certain.
        let mut g = DispositionGenome::new();
        for _ in 0..500 {
            g.observe(0, &[Q88_SCALE; N_SOMATIC], Q88_SCALE);
        }
        let r = g.readiness();
        assert!(r.precision_seed[0] <= MAX_PRECISION_SEED);
        assert!(r.consolidation_gain[0] <= MAX_CONSOLIDATION_GAIN);
        assert!(r.discharge_gain <= MAX_DISCHARGE_GAIN);
    }

    #[test]
    fn blend_carries_both_lineages_ease() {
        let mut a = DispositionGenome::new();
        let mut b = DispositionGenome::new();
        for _ in 0..100 {
            a.observe(2, &trust(3), 20);
            b.observe(2, &trust(9), 20);
        }
        let blended = ReadinessVector::blend(&a.readiness(), &b.readiness());
        // The blend leans toward *both* parents' informative channels.
        assert!(blended.precision_seed[3] > 0 && blended.precision_seed[9] > 0);
    }
}
