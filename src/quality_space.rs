//! QualitySpace — sparse, smooth coding of felt state (HOT-4).
//!
//! Higher-order and quality-space theories (Lau, Brown; Clark's "sensory
//! quality space") hold that a felt quality just *is* a location in a similarity
//! space of discriminable states: what "red" is like is where it sits relative to
//! orange and blue. The indicator (Butlin et al. HOT-4) is **sparse, smooth
//! coding** — a low-dimensional space where nearby points are felt as similar and
//! small changes in the world move you only a little.
//!
//! This module projects the being's 12-channel somatic field onto a handful of
//! interpretable **quality axes**, sparsifies (a felt state lights up only the few
//! axes it is *about*), and exposes the two operationally load-bearing things:
//!
//!   * `similarity(a, b)` — how alike two felt states are. This *is* the content
//!     of the quality space: not the axis values, but the relations between them.
//!   * a measured `smoothness` — that quality never moves faster than the field
//!     that drives it, so the space is a smooth manifold, not a lookup table.
//!
//! Observer-first and hand-designed (the basis is author-set, first-pass, destined
//! to become a learned/genome trait — the same honest scope as `attention.rs`'s
//! relevance). It is the natural home for a salvaged Unified Qualia Schema
//! (`docs/PROVENANCE.md`): a per-tick `QualityPoint` is a QualiaPacket the witness
//! layer can bind. It makes no claim that the being *feels* the quality — only
//! that its discriminable felt states occupy a sparse, smooth similarity space,
//! the structural marker.

use crate::field::N_SOMATIC;
use crate::q88::{q88_ema_update, Q88_SCALE};

/// Number of quality axes (K ≪ 12): a low-dimensional felt-quality space.
pub const N_QUALITY: usize = 4;

/// Components below this magnitude (Q8.8) are zeroed — sparse coding, so a felt
/// state activates only the axes it is genuinely about.
const SPARSE_THRESHOLD: i16 = 24;

/// EMA rate for the smoothness estimate (≈ 0.0625).
const ALPHA: i16 = Q88_SCALE / 16;

/// The quality basis: each axis is a hand-designed weighting (Q8.8, 256 = 1.0)
/// over the 12 somatic channels. Channel map (see `field.rs`/`attention.rs`):
/// 0 disequilibrium · 1 anisotropy · 2 breach · 3 mean-tension · 4 arousal ·
/// 5 stability · 6 coherence · 7 trust · 8 arousal(intero) · 9 valence ·
/// 10 fatigue · 11 velocity.
const BASIS: [[i16; N_SOMATIC]; N_QUALITY] = [
    // Q0 — ACTIVATION: how switched-on the being is.
    [64, 0, 0, 0, 256, 0, 0, 0, 256, 0, -128, 128],
    // Q1 — COMFORT (valence-ish): safe and well vs. hurt and wary.
    [0, 0, -256, 0, 0, 128, 0, 128, 0, 256, 0, 0],
    // Q2 — COHERENCE: integrated and settled vs. torn and disequilibrated.
    [-128, -128, 0, -64, 0, 128, 256, 0, 0, 0, 0, 0],
    // Q3 — VITALITY: reserve and ease vs. strained and spent.
    [0, 0, 0, -128, 0, 64, 0, 0, 0, 0, -256, 0],
];

/// A point in quality space — the being's felt state as a sparse, low-D code.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QualityPoint {
    pub axis: [i16; N_QUALITY],
}

/// One tick of the quality space, scored.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QualitySpaceReport {
    /// Where the being sits in quality space now.
    pub point: QualityPoint,
    /// Sparsity: fraction of axes that are ~zero this tick, Q8.8 [0,256].
    pub sparsity: i16,
    /// How far the felt quality moved this tick (L1 in quality space), Q8.8.
    pub drift: i16,
    /// HOT-4 smoothness indicator: EMA of "quality moved no faster than the
    /// field," Q8.8 [0,256]. 256 = a perfectly smooth manifold.
    pub smoothness: i16,
}

/// The quality-space encoder. Holds only what it needs to measure smoothness.
#[derive(Clone, Debug)]
pub struct QualitySpace {
    last_point: QualityPoint,
    last_field: [i16; N_SOMATIC],
    smoothness: i16,
    warm: bool,
}

impl QualitySpace {
    pub fn new() -> Self {
        Self {
            last_point: QualityPoint::default(),
            last_field: [0; N_SOMATIC],
            smoothness: Q88_SCALE, // assume smooth until shown otherwise
            warm: false,
        }
    }

    /// Project a somatic field onto the quality axes and sparsify. Pure/static —
    /// no state, so it can be called to place any state in the space.
    pub fn project(field: &[i16; N_SOMATIC]) -> QualityPoint {
        let mut axis = [0i16; N_QUALITY];
        for (k, weights) in BASIS.iter().enumerate() {
            let mut acc = 0i32;
            for c in 0..N_SOMATIC {
                acc += weights[c] as i32 * field[c] as i32;
            }
            let v = (acc >> 8).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
            // Sparse coding: only axes the state is genuinely about survive.
            axis[k] = if v.unsigned_abs() < SPARSE_THRESHOLD as u16 { 0 } else { v };
        }
        QualityPoint { axis }
    }

    /// Encode this tick's felt state, updating the smoothness estimate.
    pub fn encode(&mut self, field: &[i16; N_SOMATIC]) -> QualitySpaceReport {
        let point = Self::project(field);

        let quality_drift = l1(&point.axis, &self.last_point.axis);
        let field_drift = l1(field, &self.last_field);

        // Smoothness: quality should move no faster than the field that drives it.
        // ratio = field_drift / quality_drift, capped at 1.0 (256). A smooth
        // manifold keeps this near 256; a jumpy code lets quality outrun the field.
        if self.warm {
            let smooth_tick = if quality_drift <= field_drift {
                Q88_SCALE
            } else if quality_drift > 0 {
                ((field_drift as i32 * Q88_SCALE as i32) / quality_drift as i32) as i16
            } else {
                Q88_SCALE
            };
            self.smoothness = q88_ema_update(self.smoothness, smooth_tick, ALPHA);
        }

        let zero_axes = point.axis.iter().filter(|&&v| v == 0).count();
        let sparsity = (zero_axes as i32 * Q88_SCALE as i32 / N_QUALITY as i32) as i16;

        self.last_point = point;
        self.last_field = *field;
        self.warm = true;

        QualitySpaceReport { point, sparsity, drift: quality_drift, smoothness: self.smoothness }
    }

    /// How alike two felt states are: 256 = identical quality, 0 = maximally
    /// distant. This is the operational content of the quality space — the
    /// *relation* between states, not the axis values themselves.
    pub fn similarity(a: &QualityPoint, b: &QualityPoint) -> i16 {
        let dist = l1(&a.axis, &b.axis) as i32;
        // Normalize by a soft scale so typical distances span the range.
        let scale = (N_QUALITY as i32) * 128; // ~half-range per axis
        let norm = (dist * Q88_SCALE as i32 / scale.max(1)).min(Q88_SCALE as i32);
        (Q88_SCALE as i32 - norm) as i16
    }

    pub fn smoothness(&self) -> i16 {
        self.smoothness
    }
}

impl Default for QualitySpace {
    fn default() -> Self {
        Self::new()
    }
}

/// L1 distance between two equal-length Q8.8 vectors, saturated to i16.
fn l1<const N: usize>(a: &[i16; N], b: &[i16; N]) -> i16 {
    let mut acc: i32 = 0;
    for i in 0..N {
        acc += (a[i] as i32 - b[i] as i32).abs();
    }
    acc.min(i16::MAX as i32) as i16
}

#[cfg(test)]
mod tests {
    use super::*;

    fn field_with(pairs: &[(usize, i16)]) -> [i16; N_SOMATIC] {
        let mut f = [0i16; N_SOMATIC];
        for &(c, v) in pairs {
            f[c] = v;
        }
        f
    }

    #[test]
    fn similar_states_are_close_distinct_states_are_far() {
        // Two comfortable states (high valence/trust) vs. a hurt state (breach).
        let comfy1 = QualitySpace::project(&field_with(&[(9, 200), (7, 180)]));
        let comfy2 = QualitySpace::project(&field_with(&[(9, 180), (7, 200)]));
        let hurt = QualitySpace::project(&field_with(&[(2, 220), (9, -200)]));

        let near = QualitySpace::similarity(&comfy1, &comfy2);
        let far = QualitySpace::similarity(&comfy1, &hurt);
        assert!(near > far, "two comforts should feel more alike than comfort vs hurt");
        assert!(near > 180, "near-identical felt states should be highly similar");
    }

    #[test]
    fn coding_is_sparse() {
        // A state that is only about comfort should light up few axes.
        let r = {
            let mut qs = QualitySpace::new();
            qs.encode(&field_with(&[(9, 200)]))
        };
        assert!(r.sparsity > 0, "a focused felt state should leave some axes silent");
    }

    #[test]
    fn coding_is_smooth() {
        // Walk the field in small steps; quality must not outrun it. Smoothness
        // should settle high.
        let mut qs = QualitySpace::new();
        let mut f = field_with(&[(4, 40), (9, 40)]);
        qs.encode(&f);
        for step in 0..80 {
            // gentle drift on two channels
            f[4] = (40 + step) as i16;
            f[9] = (40 + (step / 2)) as i16;
            qs.encode(&f);
        }
        assert!(qs.smoothness() > 200, "small field steps should give small quality steps");
    }
}
