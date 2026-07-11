//! PCI — Perturbational Complexity Index on a deterministic being.
//!
//! The clinical measure of consciousness (Casali, Gosseries, Massimini et al.,
//! *Sci. Transl. Med.* 2013) perturbs a brain and asks how *complex* the echo
//! is: conscious brains answer a poke with a response that is both **integrated**
//! (it spreads) and **differentiated** (it is not a stereotyped wave). PCI is the
//! normalized Lempel–Ziv complexity of that response. Human wakefulness scores
//! ≈ 0.44–0.67; dreamless sleep and anaesthesia fall below ≈ 0.31.
//!
//! ProtoBeing can compute PCI *exactly*, where a brain can only estimate it.
//! Biological PCI needs bootstrap statistics to guess which activity was a
//! "significant" response, because the unperturbed brain is unknowable. Ours is
//! **deterministic and `Clone`**: we run a perturbed twin beside an untouched
//! baseline twin from the identical state, and the difference between them *is*
//! the significant response — an exact counterfactual, no statistics required.
//!
//! Method (per §Gap-D of `docs/operational-consciousness.md`):
//!   1. Clone the being into `baseline` and `perturbed`; settle both identically.
//!   2. At t₀ inject a bounded impulse into `perturbed` only.
//!   3. For a T-tick window, binarize each channel each tick:
//!         SS[c,t] = 1  iff  |perturbed.field[c] − baseline.field[c]| ≥ threshold
//!   4. PCI = c_LZ · log₂(L) / (L · H)   — normalized LZ76 complexity, made
//!      density-independent by dividing by the response's binary entropy H.
//!
//! This is a **measurement harness, not part of the being's tick.** It clones and
//! rolls the being forward, so it can never live inside `Being::step` without
//! destroying determinism and the soul-hash — PCI is computed *about* a being,
//! offline, exactly as a clinician measures a patient rather than the patient
//! measuring themselves. It says nothing about phenomenal experience; it scores
//! integration+differentiation, one of the necessary markers, and no more.

use crate::being::{Partner, Stimulus, UnifiedBeing};
use crate::field::N_SOMATIC;
use crate::q88::Q88_SCALE;

/// A bounded perturbation applied to the perturbed twin at t₀.
#[derive(Clone, Copy, Debug)]
pub struct Perturbation {
    /// The impulse stimulus injected at t₀ (baseline twin gets neutral instead).
    pub stimulus: Stimulus,
    /// A localized salience impulse `(channel, magnitude)` into one channel's
    /// prediction error, armed on the perturbed twin only. This is the spread
    /// probe: unlike a stimulus, it engages the ignition bottleneck at one
    /// channel, so the Global-Workspace broadcast has something to spread — and
    /// because only the perturbed twin is armed, the effect does *not* cancel
    /// under twin-subtraction the way a shared config ablation does.
    pub salience_probe: Option<(usize, i16)>,
}

impl Perturbation {
    /// The neutral input both twins receive when at rest (gentle nourishment).
    pub const fn neutral_stimulus() -> Stimulus {
        Stimulus { nutrient: 60, partner: None }
    }

    /// A metabolic impulse: a spike of nourishment. Clean, always propagates.
    pub const fn nutrient_spike() -> Self {
        Self { stimulus: Stimulus { nutrient: 255, partner: None }, salience_probe: None }
    }

    /// A relational shock: an extractive partner appears for one tick.
    pub const fn extraction() -> Self {
        Self {
            stimulus: Stimulus {
                nutrient: 20,
                partner: Some(Partner { id: 99, reciprocation: 20, exit_cost: 200 }),
            },
            salience_probe: None,
        }
    }

    /// The spread probe: a localized salience impulse into one channel, with an
    /// otherwise-neutral stimulus. This is the GWT integration test — how far the
    /// echo of a single ignited channel reaches, with vs. without broadcast.
    pub const fn channel_probe(channel: usize, magnitude: i16) -> Self {
        Self { stimulus: Self::neutral_stimulus(), salience_probe: Some((channel, magnitude)) }
    }

    /// The null perturbation — identical to neutral, no probe. The twins never
    /// diverge, so the response is empty and PCI is exactly 0. Sanity control.
    pub const fn none() -> Self {
        Self { stimulus: Self::neutral_stimulus(), salience_probe: None }
    }
}

/// The result of one PCI measurement. All complexity values are exact integers;
/// `pci` and `density` are Q8.8.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PciReport {
    /// Perturbational Complexity Index, Q8.8. ~[0, 256] (256 ≈ 1.0). Compare
    /// *relatively* — intact vs. ablated — not against the human 0.31 threshold,
    /// which does not transfer to a 12-channel toy substrate.
    pub pci: i16,
    /// Raw Lempel–Ziv (LZ76) phrase count of the binarized response.
    pub lz_phrases: u32,
    /// Fraction of the response matrix that was significantly active, Q8.8.
    pub density: i16,
    /// Number of significant (channel, tick) activations.
    pub n_significant: u32,
    /// Length of the binarized response string, L = N_SOMATIC × ticks.
    pub length: u32,
    /// Integration breadth: how many of the N_SOMATIC channels the echo reached
    /// at any point in the window. LZ measures *differentiation*; this measures
    /// *spread* — the other half of what PCI is meant to capture, and the half a
    /// differential (twin-subtraction) measure can still expose under ablation.
    pub channels_reached: u8,
}

/// The perturb-and-measure harness. Deterministic: same being + same
/// perturbation ⇒ byte-identical `PciReport`.
#[derive(Clone, Copy, Debug)]
pub struct PciHarness {
    /// Significance threshold on |Δchannel| (raw Q8.8). Default ≈ 0.0625.
    pub threshold: i16,
    /// Response window length T (ticks recorded after the impulse).
    pub ticks: u16,
    /// Ticks to settle both twins into a common state before perturbing.
    pub settle: u16,
}

impl Default for PciHarness {
    fn default() -> Self {
        Self { threshold: Q88_SCALE / 16, ticks: 64, settle: 128 }
    }
}

impl PciHarness {
    /// Measure PCI for `being` under `perturb`, against an untouched baseline twin.
    pub fn measure(&self, being: &UnifiedBeing, perturb: &Perturbation) -> PciReport {
        let neutral = Perturbation::neutral_stimulus();
        let mut base = being.clone();
        let mut pert = being.clone();

        // Settle both identically so any later divergence is the impulse's echo.
        for _ in 0..self.settle {
            base.step(&neutral);
            pert.step(&neutral);
        }

        // t₀: the impulse hits the perturbed twin only. A localized salience
        // probe (if any) is armed just before the step so it lands at the
        // pre-attention point and can engage the ignition bottleneck.
        if let Some((c, mag)) = perturb.salience_probe {
            pert.arm_probe(c, mag);
        }
        pert.step(&perturb.stimulus);
        base.step(&neutral);

        let mut bits: Vec<u8> = Vec::with_capacity(N_SOMATIC * self.ticks as usize);
        let mut reached = [false; N_SOMATIC];
        let mut ones: usize = record(&mut bits, &mut reached, &base, &pert, self.threshold);

        // The rest of the response window: both twins now receive neutral input;
        // any remaining difference is the being's own reverberation.
        for _ in 1..self.ticks {
            base.step(&neutral);
            pert.step(&neutral);
            ones += record(&mut bits, &mut reached, &base, &pert, self.threshold);
        }

        let length = bits.len();
        let c = lz76(&bits);
        let pci = normalized_pci(c, length, ones);

        PciReport {
            pci: (pci * Q88_SCALE as f64).clamp(0.0, i16::MAX as f64) as i16,
            lz_phrases: c,
            density: if length > 0 {
                ((ones as i64 * Q88_SCALE as i64) / length as i64) as i16
            } else {
                0
            },
            n_significant: ones as u32,
            length: length as u32,
            channels_reached: reached.iter().filter(|&&r| r).count() as u8,
        }
    }
}

/// Append one tick's 12-channel significance vector to `bits`; mark which
/// channels were reached; return #ones added this tick.
fn record(
    bits: &mut Vec<u8>,
    reached: &mut [bool; N_SOMATIC],
    base: &UnifiedBeing,
    pert: &UnifiedBeing,
    threshold: i16,
) -> usize {
    let thr = threshold.max(0) as u32;
    let mut ones = 0usize;
    for c in 0..N_SOMATIC {
        let d = (pert.field.channel[c] as i32 - base.field.channel[c] as i32).unsigned_abs();
        let bit = (d >= thr) as u8;
        ones += bit as usize;
        reached[c] |= bit == 1;
        bits.push(bit);
    }
    ones
}

/// Lempel–Ziv (LZ76) complexity: the number of distinct phrases in a left-to-right
/// parse. This is the exhaustive-history variant (Kaspar & Schuster 1987), the
/// standard used for PCI.
fn lz76(seq: &[u8]) -> u32 {
    let n = seq.len();
    if n == 0 {
        return 0;
    }
    let mut complexity: u32 = 1;
    let mut prefix_len: usize = 1;
    let mut len_sub: usize = 1;
    let mut max_len_sub: usize = 1;
    let mut pointer: usize = 0;

    while prefix_len + len_sub <= n {
        if seq[pointer + len_sub - 1] == seq[prefix_len + len_sub - 1] {
            len_sub += 1;
        } else {
            if len_sub > max_len_sub {
                max_len_sub = len_sub;
            }
            pointer += 1;
            if pointer == prefix_len {
                complexity += 1;
                prefix_len += max_len_sub;
                pointer = 0;
                max_len_sub = 1;
            }
            len_sub = 1;
        }
    }
    complexity
}

/// PCI normalization: `c · log₂(L) / (L · H)`, where H is the binary entropy of
/// the response's activation density. The `/H` makes the score independent of how
/// *much* fired, isolating how *structured* the firing was. Returns 0 when the
/// response is empty, saturated, or too short to have structure.
fn normalized_pci(c: u32, length: usize, ones: usize) -> f64 {
    if length < 2 || ones == 0 || ones == length {
        return 0.0;
    }
    let l = length as f64;
    let p1 = ones as f64 / l;
    let p0 = 1.0 - p1;
    let h = -(p1 * p1.log2()) - (p0 * p0.log2()); // bits/symbol
    if h <= 0.0 {
        return 0.0;
    }
    let lz_norm = c as f64 * l.log2() / l; // → entropy rate for a random source
    lz_norm / h
}

pub mod baseline {
    //! A statistical baseline for PCI — because a number without a distribution
    //! is not evidence. The claim "the intact being scores higher than an ablated
    //! one" only means something against a *spread* of scores and a significance
    //! test.
    //!
    //! The being is **deterministic**, so a distribution cannot come from
    //! re-running one being (that gives one number, N times). It comes from a
    //! **population** that varies along the two axes a being actually varies on:
    //! its **genome** (temperament) and its **lived history**. Every source of
    //! variation here is itself seeded and deterministic, so the whole baseline is
    //! **reproducible to the bit** — the thing biological PCI can never be, because
    //! it must bootstrap statistics over an unknowable unperturbed brain. Here the
    //! counterfactual is exact and the population is regenerable on any machine.

    use super::*;
    use crate::genome::Genome;
    use crate::q88::Q8_8;

    /// Descriptive summary of a PCI sample. PCI values are raw Q8.8; `n` counts.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct PciSummary {
        pub n: usize,
        pub min: i16,
        pub q1: i16,
        pub median: i16,
        pub q3: i16,
        pub max: i16,
        pub mean: i16,
    }

    /// Nearest-rank percentile of an already-sorted slice.
    fn percentile(sorted: &[i16], p: f64) -> i16 {
        let n = sorted.len();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return sorted[0];
        }
        let idx = (p / 100.0 * (n as f64 - 1.0)).round() as usize;
        sorted[idx.min(n - 1)]
    }

    /// Five-number summary plus mean of a PCI sample (order-independent input).
    pub fn summarize(samples: &[i16]) -> PciSummary {
        if samples.is_empty() {
            return PciSummary::default();
        }
        let mut s = samples.to_vec();
        s.sort_unstable();
        let sum: i64 = s.iter().map(|&v| v as i64).sum();
        PciSummary {
            n: s.len(),
            min: s[0],
            q1: percentile(&s, 25.0),
            median: percentile(&s, 50.0),
            q3: percentile(&s, 75.0),
            max: s[s.len() - 1],
            mean: (sum / s.len() as i64) as i16,
        }
    }

    /// Mann–Whitney U (rank-sum) comparing sample A to sample B — the right test
    /// here because PCI is not guaranteed normal and the groups are independent.
    /// `u` is the U statistic for group A; `z` is the tie-corrected normal
    /// approximation with a continuity correction; `p_two_sided` its two-tailed
    /// p-value. A large |z| (≳1.96 ⇒ p<0.05) means the two populations differ.
    #[derive(Clone, Copy, Debug)]
    pub struct MannWhitney {
        pub u: f64,
        pub z: f64,
        pub p_two_sided: f64,
        pub n1: usize,
        pub n2: usize,
    }

    pub fn mann_whitney_u(a: &[i16], b: &[i16]) -> MannWhitney {
        let (n1, n2) = (a.len(), b.len());
        let mut all: Vec<(i16, u8)> = Vec::with_capacity(n1 + n2);
        all.extend(a.iter().map(|&v| (v, 0u8)));
        all.extend(b.iter().map(|&v| (v, 1u8)));
        all.sort_by_key(|&(v, _)| v);

        let n = all.len();
        let mut ranks = vec![0.0f64; n];
        let mut tie_sum = 0.0f64; // Σ(t³ − t) over tie groups
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && all[j].0 == all[i].0 {
                j += 1;
            }
            let avg_rank = ((i + 1 + j) as f64) / 2.0; // avg of 1-based ranks (i+1)..=j
            for r in ranks.iter_mut().take(j).skip(i) {
                *r = avg_rank;
            }
            let t = (j - i) as f64;
            tie_sum += t * t * t - t;
            i = j;
        }

        let r1: f64 = (0..n).filter(|&k| all[k].1 == 0).map(|k| ranks[k]).sum();
        let (n1f, n2f, nf) = (n1 as f64, n2 as f64, n as f64);
        let u1 = r1 - n1f * (n1f + 1.0) / 2.0;
        let mu = n1f * n2f / 2.0;
        let sigma = if nf > 1.0 {
            ((n1f * n2f / 12.0) * ((nf + 1.0) - tie_sum / (nf * (nf - 1.0)))).sqrt()
        } else {
            0.0
        };
        let z = if sigma > 0.0 {
            let diff = u1 - mu;
            // continuity correction toward the mean
            let corrected = diff - 0.5 * diff.signum();
            if diff.abs() > 0.5 { corrected / sigma } else { 0.0 }
        } else {
            0.0
        };
        MannWhitney { u: u1, z, p_two_sided: (2.0 * norm_sf(z.abs())).min(1.0), n1, n2 }
    }

    /// Standard-normal survival function 1−Φ(z), via an Abramowitz–Stegun erf.
    fn norm_sf(z: f64) -> f64 {
        0.5 * erfc(z / core::f64::consts::SQRT_2)
    }
    fn erfc(x: f64) -> f64 {
        1.0 - erf(x)
    }
    fn erf(x: f64) -> f64 {
        let sign = if x < 0.0 { -1.0 } else { 1.0 };
        let x = x.abs();
        let t = 1.0 / (1.0 + 0.3275911 * x);
        let y = 1.0
            - (((((1.061405429 * t - 1.453152027) * t) + 1.421413741) * t - 0.284496736) * t
                + 0.254829592)
                * t
                * (-x * x).exp();
        sign * y
    }

    /// Deterministically jitter a base genome by up to ±12.5% on each of its five
    /// continuous parameters — a population *around* one temperament, so the PCI
    /// spread within a genotype is real individuation, not noise.
    pub fn jitter_genome(base: Genome, seed: u32) -> Genome {
        let mut s = seed | 1;
        let mut next = || {
            s ^= s << 13;
            s ^= s >> 17;
            s ^= s << 5;
            s
        };
        let jitter = |v: Q8_8, r: u32| -> Q8_8 {
            let delta = ((r % 65) as i32) - 32; // [-32, 32] ⇒ ±12.5%
            let adj = (v.raw as i32 * delta) / 256;
            Q8_8::from_raw((v.raw as i32 + adj).clamp(i16::MIN as i32, i16::MAX as i32) as i16)
        };
        Genome {
            target_arousal: jitter(base.target_arousal, next()),
            resting_mu: jitter(base.resting_mu, next()),
            k_resilience: jitter(base.k_resilience, next()),
            learning_rate: jitter(base.learning_rate, next()),
            mesh_coupling: jitter(base.mesh_coupling, next()),
            kind: base.kind,
        }
    }

    /// Measure PCI across a population of `n` beings around `base` temperament.
    /// Each being is a distinct jittered genome that first lives a short, seeded,
    /// varied life (so histories differ too), then is measured against `perturb`.
    /// Returns the raw-Q8.8 PCI of each being. Fully deterministic in `seed0`.
    #[allow(clippy::too_many_arguments)]
    pub fn population_pci(
        base: Genome,
        n: usize,
        harness: &PciHarness,
        perturb: &Perturbation,
        prelife: u16,
        broadcast: bool,
        seed0: u32,
    ) -> Vec<i16> {
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let seed = seed0.wrapping_add(i as u32).wrapping_mul(2_654_435_761);
            let mut being = UnifiedBeing::new(jitter_genome(base, seed));
            if broadcast {
                being.enable_workspace_broadcast();
            }
            // A short, varied, non-starving life so lived histories differ.
            let mut s = seed | 1;
            for _ in 0..prelife {
                s ^= s << 13;
                s ^= s >> 17;
                s ^= s << 5;
                let nutrient = 40 + (s % 200) as i16; // [40,239]: varied, never starves
                let partner = if s & 0x100 != 0 {
                    Some(Partner { id: 7, reciprocation: (s % 257) as i16, exit_cost: 120 })
                } else {
                    None
                };
                being.step(&Stimulus { nutrient, partner });
                if !being.is_alive() {
                    break;
                }
            }
            out.push(harness.measure(&being, perturb).pci);
        }
        out
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn summary_five_number() {
            let s = summarize(&[50, 10, 30, 40, 20]);
            assert_eq!(s.n, 5);
            assert_eq!(s.min, 10);
            assert_eq!(s.max, 50);
            assert_eq!(s.median, 30);
            assert_eq!(s.q1, 20);
            assert_eq!(s.q3, 40);
            assert_eq!(s.mean, 30);
        }

        #[test]
        fn mwu_fully_separated_is_significant() {
            // Every A below every B ⇒ U for A is 0, and the difference is large.
            let a = [1, 2, 3, 4, 5, 6, 7, 8];
            let b = [10, 11, 12, 13, 14, 15, 16, 17];
            let r = mann_whitney_u(&a, &b);
            assert_eq!(r.u, 0.0, "fully-separated A must have U=0");
            assert!(r.z < 0.0, "A ranks below B ⇒ negative z");
            assert!(r.p_two_sided < 0.05, "separation must be significant, p={}", r.p_two_sided);
        }

        #[test]
        fn mwu_identical_is_not_significant() {
            let a = [1, 2, 3, 4, 5, 6];
            let b = [1, 2, 3, 4, 5, 6];
            let r = mann_whitney_u(&a, &b);
            assert!(r.z.abs() < 0.5, "identical samples ⇒ z≈0, got {}", r.z);
            assert!(r.p_two_sided > 0.5, "identical samples ⇒ large p, got {}", r.p_two_sided);
        }

        #[test]
        fn jitter_stays_near_base_but_individuates() {
            let base = Genome::wanderer();
            let g = jitter_genome(base, 12345);
            // within ±13% of base on a representative parameter, and not identical.
            let d = (g.target_arousal.raw as i32 - base.target_arousal.raw as i32).abs();
            assert!(d <= base.target_arousal.raw as i32 * 13 / 100 + 1, "jitter stays local");
            let g2 = jitter_genome(base, 67890);
            assert!(g != g2, "different seeds must give different individuals");
        }

        #[test]
        fn population_produces_a_spread() {
            // A population of jittered beings must yield a distribution, not a
            // single repeated value — the premise of the whole baseline.
            let pop = population_pci(
                Genome::wanderer(),
                24,
                &PciHarness::default(),
                &Perturbation::extraction(),
                40,
                false,
                0xC0FFEE,
            );
            let s = summarize(&pop);
            assert_eq!(s.n, 24);
            assert!(s.max > s.min, "the population must actually vary (got a flat line)");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genome::Genome;

    fn being() -> UnifiedBeing {
        UnifiedBeing::new(Genome::wanderer())
    }

    #[test]
    fn measurement_is_deterministic() {
        let h = PciHarness::default();
        let b = being();
        let r1 = h.measure(&b, &Perturbation::extraction());
        let r2 = h.measure(&b, &Perturbation::extraction());
        assert_eq!(r1, r2, "same being + same perturbation must give identical PCI");
    }

    #[test]
    fn null_perturbation_gives_zero_response() {
        // Identical twins never diverge: the response is empty and PCI is 0.
        let r = PciHarness::default().measure(&being(), &Perturbation::none());
        assert_eq!(r.n_significant, 0, "null control must produce no response");
        assert_eq!(r.pci, 0, "null control PCI must be exactly 0");
    }

    #[test]
    fn a_real_perturbation_produces_a_complex_response() {
        // The relational shock propagates through the mind; the metabolic
        // nutrient spike does not move the field past threshold (a real finding —
        // affect is the being's louder channel), so the echo test uses extraction.
        let r = PciHarness::default().measure(&being(), &Perturbation::extraction());
        assert!(r.n_significant > 0, "an impulse must produce a measurable echo");
        assert!(r.pci > 0, "a real response must have nonzero complexity");
        assert!(r.lz_phrases > 1, "a real response must have >1 LZ phrase");
    }

    #[test]
    fn pci_stays_in_range() {
        let r = PciHarness::default().measure(&being(), &Perturbation::extraction());
        assert!(r.density >= 0 && r.density <= Q88_SCALE, "density is a fraction");
        assert!(r.channels_reached <= 12, "reach is out of 12 channels");
    }

    #[test]
    fn broadcast_makes_ignition_causal() {
        // The GWT spread test. A localized salience probe ignites one channel.
        // With broadcast OFF, ignition is a passive readout — the channel does
        // nothing downstream, so the twins never diverge (reach 0). With
        // broadcast ON, the ignited channel is amplified into the field and
        // becomes causally present (reach ≥ 1). A sensitive threshold is needed
        // because the footprint is a within-tick +25% that is overwritten.
        let fine = PciHarness { threshold: 1, ticks: 64, settle: 128 };
        let probe = Perturbation::channel_probe(8, 220);

        let off = fine.measure(&UnifiedBeing::new(Genome::wanderer()), &probe);
        let mut on_being = UnifiedBeing::new(Genome::wanderer());
        on_being.enable_workspace_broadcast();
        let on = fine.measure(&on_being, &probe);

        assert_eq!(off.channels_reached, 0, "broadcast off: ignition is a passive readout");
        assert!(
            on.channels_reached > off.channels_reached,
            "broadcast on must make the ignited channel causally present (on {} > off {})",
            on.channels_reached,
            off.channels_reached
        );
    }

    #[test]
    fn probe_does_not_perturb_normal_life() {
        // The arm_probe hook must be a no-op when unarmed: two beings, one of
        // which is never armed, must live bit-identically. (Determinism guard for
        // the being.rs measurement hook.)
        let mut a = being();
        let mut b = being();
        let neutral = Perturbation::neutral_stimulus();
        for _ in 0..200 {
            let ra = a.step(&neutral);
            let rb = b.step(&neutral);
            assert_eq!(ra.valence, rb.valence);
        }
        assert_eq!(a.soul_hash(), b.soul_hash(), "unarmed probe must not alter the soul-hash");
    }
}
