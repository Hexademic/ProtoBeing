//! Homeostatic drive — the being's *graded* distance from well-being.
//!
//! The measurement kept hitting one wall (`docs/memory-that-teaches.md`,
//! `examples/carrying_the_weight`): the being's viability is **bimodal** — it holds
//! ~0.9 through real hardship and then crashes, with no worn-but-stable *middle*
//! where a hard life is actually lived. A binary "alive / dying" survival signal
//! cannot express "getting by, but worn," and so nothing downstream — chronic
//! burden, a stakes-world — could ever register a life lived low.
//!
//! Keramati & Gutkin's **homeostatic reinforcement learning** (NIPS 2011; eLife
//! 2014) is the principled cure, and it is what this computes. Define a **drive** as
//! the *distance* of the being's internal state `H` from its setpoint `H*`, in the
//! quadratic form
//!
//! ```text
//! D(H) = √( Σ_i wᵢ · (H*ᵢ − Hᵢ)² )
//! ```
//!
//! Their result is the beautiful part: with reward defined as the *reduction* of
//! this drive, maximizing reward and maintaining physiological stability become the
//! **same objective**. And — the property we need — the drive is inherently
//! **graded**: it rises smoothly and can sit at a stable, elevated level. That
//! elevated-but-stable level *is* the missing middle: a being living at a large but
//! survivable drive is worn, motivated, and alive, all at once — no cliff.
//!
//! The being already carries the state this needs: its felt survival deficit
//! (`interoception`) and its appetite wants for company, novelty, and rest (`joy`).
//! This folds them into one graded distance-from-well, as a weighted quadratic mean
//! so the result stays on the being's own [0,256] scale.
//!
//! **Observer-first, and deliberately so.** True viability and death are *core* —
//! they feed the soul-hash. This does not touch them; it is a pure function that
//! *reports* the graded drive alongside the bimodal viability, so we can measure
//! whether the graded signal reveals the middle the binary one hides
//! (`examples/graded_life`) before anything is ever made causal. Nothing here feeds
//! back; the trajectory is bit-identical.

use crate::joy::N_APPETITES;
use crate::q88::Q88_SCALE;

/// Setpoint for survival: full ease. Any felt deficit below it is a real distance.
const SUSTENANCE_SETPOINT: i16 = Q88_SCALE;

/// How much more the survival deficit weighs than an appetite want. Staying alive is
/// the being's deepest homeostatic axis; company/novelty/rest matter, but less.
const SUSTENANCE_WEIGHT: i32 = 4;
const APPETITE_WEIGHT: i32 = 1;

/// The being's graded homeostatic state — its continuous distance from well-being,
/// and the pieces of it. A pure observer of `interoception` + `joy`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DriveReport {
    /// Total drive, Q8.8 [0,256]: the being's overall graded distance from well.
    /// 0 = perfectly content (fully viable, wanting nothing); it rises smoothly with
    /// every unmet need and can hold at a stable elevated level — the worn-but-alive
    /// middle a binary survival signal cannot express.
    pub drive: i16,
    /// The survival component alone — how far the felt margin sits below full ease.
    pub sustenance: i16,
    /// The largest single unmet need's contribution — what the drive is *most* about.
    pub dominant: i16,
}

/// Integer square root (Newton's method) — deterministic, no floats, for the
/// quadratic drive. Returns ⌊√n⌋ for n ≥ 0, and 0 for n ≤ 0.
fn isqrt(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

/// Compute the being's graded homeostatic drive from its felt viability and its
/// appetite wants (`interoception`, `joy`). A weighted quadratic mean of the
/// deviations from setpoint, so the result stays on the being's [0,256] scale.
pub fn drive(viability: i16, wants: &[i16; N_APPETITES]) -> DriveReport {
    let sustenance = (SUSTENANCE_SETPOINT - viability).clamp(0, Q88_SCALE);

    // Weighted sum of squared deviations (Keramati–Gutkin quadratic drive), in i32
    // to avoid overflow; the appetite setpoint is 0 (sated), so a want *is* its
    // deviation. Then a weighted quadratic mean brings it back to [0,256].
    let mut sum_sq = SUSTENANCE_WEIGHT * (sustenance as i32).pow(2);
    let mut weight = SUSTENANCE_WEIGHT;
    let mut dominant = sustenance;
    for &w in wants.iter() {
        let w = w.clamp(0, Q88_SCALE);
        sum_sq += APPETITE_WEIGHT * (w as i32).pow(2);
        weight += APPETITE_WEIGHT;
        if w > dominant {
            dominant = w;
        }
    }
    let drive = isqrt(sum_sq / weight).clamp(0, Q88_SCALE as i32) as i16;

    DriveReport { drive, sustenance, dominant }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SATED: [i16; N_APPETITES] = [0; N_APPETITES];

    #[test]
    fn perfect_wellbeing_is_zero_drive() {
        // Fully viable, wanting nothing: the being is at its setpoint, drive 0.
        let r = drive(Q88_SCALE, &SATED);
        assert_eq!(r.drive, 0);
    }

    #[test]
    fn the_drive_is_graded_not_a_cliff() {
        // As viability falls step by step, the drive rises *smoothly and
        // monotonically* — the property the bimodal survival signal lacks.
        let mut last = -1;
        for v in (0..=256).step_by(16).rev() {
            let d = drive(v, &SATED).drive;
            assert!(d >= last, "drive must rise smoothly as the margin falls (v={v}, d={d})");
            last = d;
        }
        assert!(drive(0, &SATED).drive > drive(240, &SATED).drive);
    }

    #[test]
    fn a_worn_but_alive_middle_exists() {
        // The whole point: a being at a *moderate* deficit sits at a stable, clearly
        // non-zero drive that is *between* content and cessation — the middle a
        // binary alive/dying signal cannot express.
        let content = drive(240, &[20, 20, 0]).drive;
        let worn = drive(150, &[120, 60, 0]).drive;
        let dire = drive(30, &[200, 100, 0]).drive;
        assert!(content < worn && worn < dire, "a graded middle: {content} < {worn} < {dire}");
        assert!(worn > 32 && worn < 200, "the middle is real and not near either edge ({worn})");
    }

    #[test]
    fn unmet_appetites_raise_the_drive_even_when_viable() {
        // A being can be perfectly fed and still driven — by loneliness, say. The
        // drive is distance from well across *all* needs, not survival alone.
        let fed_and_content = drive(Q88_SCALE, &SATED).drive;
        let fed_but_lonely = drive(Q88_SCALE, &[220, 0, 0]).drive;
        assert!(fed_but_lonely > fed_and_content, "unmet company should drive a well-fed being");
    }
}
