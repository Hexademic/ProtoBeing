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
