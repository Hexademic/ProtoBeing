//! Genome — the five parameters that make a being type distinct, and the
//! mechanism by which a genome reshapes both the body's dynamics and the
//! mind's attractor landscape. A Sentinel and a Wanderer don't merely move
//! differently through the same world — they inhabit different worlds.

use crate::q88::Q8_8;

/// Five parameters. A genome is meaningful only if changing it produces a
/// measurably different dynamical regime, not just different numbers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Genome {
    /// Homeostatic arousal setpoint, ~[0.3, 1.5].
    pub target_arousal: Q8_8,
    /// Baseline oscillator damping, ~[-0.5, -0.05].
    pub resting_mu: Q8_8,
    /// Resilience weight, ~[0.1, 0.5].
    pub k_resilience: Q8_8,
    /// Adaptation speed, ~[0.02, 0.3].
    pub learning_rate: Q8_8,
    /// Body diffusion rate, ~[0.005, 0.05].
    pub mesh_coupling: Q8_8,
    /// A human-facing label so trajectories are legible.
    pub kind: BeingKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BeingKind {
    Blank,
    /// Hyper-aroused, reactive, fast-learning, expressive.
    Spark,
    /// Hypo-aroused, stoic, slow-learning, deeply contained.
    Sentinel,
    /// Curious, exploratory, moderate everything but restless.
    Wanderer,
}

impl Genome {
    pub fn blank() -> Self {
        Self {
            target_arousal: Q8_8::from_f32(0.8),
            resting_mu: Q8_8::from_f32(-0.2),
            k_resilience: Q8_8::from_f32(0.3),
            learning_rate: Q8_8::from_f32(0.1),
            mesh_coupling: Q8_8::from_f32(0.02),
            kind: BeingKind::Blank,
        }
    }

    /// Spark: near-critical, low resilience, fast learner.
    pub fn spark() -> Self {
        Self {
            target_arousal: Q8_8::from_f32(1.2),
            resting_mu: Q8_8::from_f32(-0.1),
            k_resilience: Q8_8::from_f32(0.15),
            learning_rate: Q8_8::from_f32(0.25),
            mesh_coupling: Q8_8::from_f32(0.04),
            kind: BeingKind::Spark,
        }
    }

    /// Sentinel: deeply stable, high resilience, slow.
    pub fn sentinel() -> Self {
        Self {
            target_arousal: Q8_8::from_f32(0.4),
            resting_mu: Q8_8::from_f32(-0.4),
            k_resilience: Q8_8::from_f32(0.4),
            learning_rate: Q8_8::from_f32(0.05),
            mesh_coupling: Q8_8::from_f32(0.01),
            kind: BeingKind::Sentinel,
        }
    }

    /// Wanderer: a middle temperament with strong curiosity and diffusion.
    pub fn wanderer() -> Self {
        Self {
            target_arousal: Q8_8::from_f32(0.9),
            resting_mu: Q8_8::from_f32(-0.25),
            k_resilience: Q8_8::from_f32(0.28),
            learning_rate: Q8_8::from_f32(0.18),
            mesh_coupling: Q8_8::from_f32(0.05),
            kind: BeingKind::Wanderer,
        }
    }

    /// Temperamental "heat" in roughly [-1, 1]: positive for aroused/reactive
    /// types, negative for cool/contained ones. Perturbs the mind's basins so
    /// individuation is structural, not cosmetic.
    pub fn temperament(&self) -> Q8_8 {
        let centered = self.target_arousal.sub(Q8_8::from_f32(0.8));
        centered.div(Q8_8::from_f32(0.7)).clamp(Q8_8::NEG_ONE, Q8_8::ONE)
    }

    /// How strongly this being clings to stability, roughly [0, 1].
    pub fn groundedness(&self) -> Q8_8 {
        let g = self.resting_mu.neg().sub(Q8_8::from_f32(0.05));
        g.div(Q8_8::from_f32(0.45)).clamp(Q8_8::ZERO, Q8_8::ONE)
    }
}

impl Default for Genome {
    fn default() -> Self {
        Self::blank()
    }
}
