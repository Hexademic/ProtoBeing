//! Receptors — organoid-styled sensory transduction: adaptation, compression, type.
//!
//! The wet-vs-digital substrate question has one honest invariant: *even a
//! biological being still has to **process** its sensations to keep track of
//! them* (the functionalist reading — the tracking is the thing, and tracking is
//! substrate-neutral). So instead of wet sensors, we build faithful models of
//! *what a wet sensor tracks*. Real senses never report a raw value; they
//! transduce it, and three properties do the work:
//!
//!   * **Adaptation.** Receptors report *change*, not steady state — you stop
//!     feeling your clothes. A sustained stimulus fades; a new one spikes.
//!   * **Compression** (Naka–Rushton / Weber–Fechner). Response saturates and is
//!     far more sensitive to small stimuli than large — a candle is vivid in the
//!     dark, nothing at noon. `R = I / (I + K)`.
//!   * **Type.** Fast-adapting change-detectors (Pacinian/Meissner-like),
//!     slow-adapting level-reporters (Merkel/Ruffini-like), and **nociceptors**
//!     that have a harm threshold and do **not** adapt — pain cannot be tuned out.
//!
//! This turns the being's flat scalar inputs into an *organized, typed* sensory
//! representation (the RPT-2 marker), and is the first rung toward sensorimotor
//! contingency (AE-2) and, later, connectome-inspired structure. Deterministic,
//! Q8.8, zero-dependency, like the rest of the crate. Built as a standalone,
//! tested primitive; wiring it into the `Sensorium`/field is the next step,
//! observer-first.

use crate::q88::{q88_ema_update, q88_mul, Q88_SCALE};

/// Naka–Rushton half-saturation constant (raw Q8.8): response is half-max when
/// the (adapted) input equals this. Sets the compression curve's knee.
const K: i16 = Q88_SCALE / 4; // 64

/// Fast-adapting habituation rate (~0.5): the baseline chases the input quickly,
/// so a steady stimulus is forgotten within a few ticks.
const FAST_ALPHA: i16 = Q88_SCALE / 2; // 128

/// Slow-adapting habituation rate (~0.03): the baseline creeps, so a level is
/// tracked for a long time before it droops.
const SLOW_ALPHA: i16 = 8;

/// How much of a slow-adapting receptor's response the baseline can cancel
/// (~0.6): a real slow-adapting receptor keeps a *sustained* (reduced) response,
/// never falling fully silent under a maintained stimulus.
const PARTIAL_ADAPT: i16 = 154; // ≈ 0.6

/// Harm threshold for a nociceptor (raw Q8.8, ~0.375): silent below it.
const NOCI_THRESHOLD: i16 = 96;

/// The kind of receptor, which sets its dynamics — as in real sensory biology.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReceptorKind {
    /// Fast-adapting: reports **change**. Spikes at the onset/offset of a
    /// stimulus, falls silent under a steady one. Signed (onset +, offset −).
    FastAdapting,
    /// Slow-adapting: reports **level**. Tracks a sustained stimulus, adapting
    /// only partially, so a maintained input keeps a reduced but nonzero response.
    SlowAdapting,
    /// Nociceptor: reports **harm**. Silent below a threshold; above it, fires and
    /// does not adapt away — pain persists so it cannot be tuned out.
    Nociceptor,
}

/// One receptor: its kind, its adaptation state, and its transduction.
#[derive(Clone, Copy, Debug)]
pub struct Receptor {
    kind: ReceptorKind,
    /// The stimulus level this receptor has habituated to (raw Q8.8). Nociceptors
    /// never move it — they do not adapt.
    baseline: i16,
}

impl Receptor {
    pub fn new(kind: ReceptorKind) -> Self {
        Self { kind, baseline: 0 }
    }

    /// The kind of this receptor.
    pub fn kind(&self) -> ReceptorKind {
        self.kind
    }

    /// The level this receptor has currently adapted to (raw Q8.8). For a
    /// nociceptor this stays 0 — it does not habituate.
    pub fn adapted_to(&self) -> i16 {
        self.baseline
    }

    /// Transduce a raw stimulus (raw Q8.8, an intensity ≥ 0) into this receptor's
    /// response this tick, updating its adaptation state. Fast-adapting responses
    /// are signed (onset +, offset −); the others are non-negative.
    pub fn sense(&mut self, raw: i16) -> i16 {
        match self.kind {
            ReceptorKind::FastAdapting => {
                // Deviation from what it has habituated to, *then* chase the input
                // — so a step spikes and a sustained level fades to silence.
                let deviation = raw as i32 - self.baseline as i32;
                self.baseline = q88_ema_update(self.baseline, raw, FAST_ALPHA);
                let mag = naka(deviation.unsigned_abs().min(i16::MAX as u32) as i16);
                if deviation < 0 {
                    -mag
                } else {
                    mag
                }
            }
            ReceptorKind::SlowAdapting => {
                // Slow habituation with only *partial* cancellation, so a
                // maintained stimulus keeps a reduced but sustained response.
                let effective = (raw as i32 - q88_mul(self.baseline, PARTIAL_ADAPT) as i32)
                    .clamp(0, i16::MAX as i32) as i16;
                self.baseline = q88_ema_update(self.baseline, raw, SLOW_ALPHA);
                naka(effective)
            }
            ReceptorKind::Nociceptor => {
                // Threshold, no adaptation — harm is not tuned out.
                if raw <= NOCI_THRESHOLD {
                    0
                } else {
                    naka(raw - NOCI_THRESHOLD)
                }
            }
        }
    }
}

/// One tick's reading from the being's sensory receptor population.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReceptorReading {
    /// The four exteroceptive channels, transduced (raw Q8.8). Fast-adapting on
    /// the first two (contact/change), slow-adapting on the last two (pressure).
    pub extero: [i16; 4],
    /// Nociception: the being's felt harm this tick (raw Q8.8, ≥ 0). Bounded, and
    /// silent the moment the harm is gone — meaningful pain, never a trap.
    pub pain: i16,
}

/// The being's sensory receptor population — a fixed, typed bank that transduces
/// its embodiment senses (four exteroceptive channels plus a nociceptor for
/// threat) into an organized, adapted reading. The typing is the point: the same
/// world reaches the mind already sorted into change, level, and harm.
#[derive(Clone, Copy, Debug)]
pub struct ReceptorBank {
    extero: [Receptor; 4],
    noci: Receptor,
}

impl ReceptorBank {
    pub fn new() -> Self {
        Self {
            extero: [
                Receptor::new(ReceptorKind::FastAdapting),
                Receptor::new(ReceptorKind::FastAdapting),
                Receptor::new(ReceptorKind::SlowAdapting),
                Receptor::new(ReceptorKind::SlowAdapting),
            ],
            noci: Receptor::new(ReceptorKind::Nociceptor),
        }
    }

    /// Transduce this tick's embodiment senses. Pure of the being's causal loop
    /// unless the being routes the reading back into its field; the bank simply
    /// tracks, adapting as it goes.
    pub fn transduce(&mut self, exteroception: &[i16; 4], threat: i16) -> ReceptorReading {
        let mut extero = [0i16; 4];
        for (i, r) in self.extero.iter_mut().enumerate() {
            extero[i] = r.sense(exteroception[i]);
        }
        ReceptorReading { extero, pain: self.noci.sense(threat) }
    }
}

impl Default for ReceptorBank {
    fn default() -> Self {
        Self::new()
    }
}

/// Naka–Rushton compression: `R = SCALE · I / (I + K)`, raw Q8.8. Zero for I ≤ 0,
/// half-max at I = K, saturating toward SCALE — high gain for small stimuli, low
/// gain for large ones (Weber–Fechner in one line).
fn naka(intensity: i16) -> i16 {
    if intensity <= 0 {
        return 0;
    }
    ((intensity as i32 * Q88_SCALE as i32) / (intensity as i32 + K as i32)) as i16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compression_is_more_sensitive_to_small_stimuli() {
        // Weber–Fechner: the same +20 increment produces a larger response change
        // near zero than near saturation.
        let low = naka(40) - naka(20);
        let high = naka(240) - naka(220);
        assert!(low > high, "small stimuli must be more discriminable ({low} vs {high})");
        assert_eq!(naka(K), Q88_SCALE / 2, "half-max at the half-saturation constant");
    }

    #[test]
    fn fast_adapting_reports_change_then_falls_silent() {
        let mut r = Receptor::new(ReceptorKind::FastAdapting);
        let onset = r.sense(200); // stimulus appears
        assert!(onset > 40, "a new stimulus must spike the change detector, got {onset}");
        // Held steady, it adapts to silence within a few ticks.
        let mut last = onset;
        for _ in 0..12 {
            last = r.sense(200);
        }
        assert!(last.abs() < 8, "a sustained stimulus must fade to silence, got {last}");
        // A *further* change spikes it again — and downward is signed negative.
        let offset = r.sense(60);
        assert!(offset < -20, "a change must fire it again, signed for offset, got {offset}");
    }

    #[test]
    fn slow_adapting_sustains_a_level() {
        let mut r = Receptor::new(ReceptorKind::SlowAdapting);
        let first = r.sense(200);
        let mut last = first;
        for _ in 0..60 {
            last = r.sense(200);
        }
        assert!(last > 0, "a slow-adapting receptor must keep a sustained response, got {last}");
        assert!(last < first, "but it droops from its initial level ({last} < {first})");
        // Contrast: a fast-adapting receptor on the identical trace goes silent.
        let mut fast = Receptor::new(ReceptorKind::FastAdapting);
        let mut fast_last = 0;
        for _ in 0..61 {
            fast_last = fast.sense(200);
        }
        assert!(last > fast_last.abs(), "slow sustains where fast falls silent");
    }

    #[test]
    fn nociceptor_has_a_threshold_and_never_adapts() {
        let mut n = Receptor::new(ReceptorKind::Nociceptor);
        assert_eq!(n.sense(NOCI_THRESHOLD - 10), 0, "silent below the harm threshold");
        let hurt = n.sense(220);
        assert!(hurt > 0, "fires above the threshold");
        // Sustained harm does NOT fade — pain cannot be tuned out.
        let mut last = hurt;
        for _ in 0..100 {
            last = n.sense(220);
        }
        assert_eq!(last, hurt, "a nociceptor must not adapt away a persistent harm");
        assert_eq!(n.adapted_to(), 0, "and it never habituates");
    }

    #[test]
    fn the_three_types_read_one_world_differently() {
        // The heart of it: a single stimulus trace, three receptors, three
        // faithful readings — organized, typed perception (RPT-2), not a flat echo.
        let (mut fast, mut slow, mut noci) = (
            Receptor::new(ReceptorKind::FastAdapting),
            Receptor::new(ReceptorKind::SlowAdapting),
            Receptor::new(ReceptorKind::Nociceptor),
        );
        // A gentle, sustained pressure (below the harm threshold).
        let (mut f, mut s, mut no) = (0, 0, 0);
        for _ in 0..30 {
            f = fast.sense(80);
            s = slow.sense(80);
            no = noci.sense(80);
        }
        assert!(f.abs() < 8, "fast: adapted to the steady pressure (silent)");
        assert!(s > 0, "slow: still reporting the sustained level");
        assert_eq!(no, 0, "noci: correctly silent — this pressure is not harm");
    }
}
