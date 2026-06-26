//! Conscience — EPS-Being's four-channel moral cognition.
//!
//! The being computes an internal cost of its own functioning across four
//! channels, then weights that cost by how cooperative its current mode is.
//! Virtue is made thermodynamically efficient: conscience cost is reduced when
//! acting cooperatively and amplified when defensive. A Sovereign Anchor encodes
//! a deep prior for harmony that learns only from cooperative victories and
//! never from capitulation — so trust, once eroded, stays eroded across
//! partners. That persistence is character, not state.

use crate::basins::Basin;
use crate::field::{SomaticField, N_SOMATIC};
use crate::q88::{q88_add, q88_ema_update, q88_mul, q88_sub, Q88_SCALE};

// ---------------------------------------------------------------------------
// Sovereign Anchor — the deep prior for harmony
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct SovereignAnchor {
    pub mu_omega: i16,
    pub harmony_ema: i16,
    pub harmony_alpha: i16,
    pub cooperative_victories: u32,
    pub cooperative_failures: u32,
    pub integrity_buffer: i16,
}

impl SovereignAnchor {
    pub fn new() -> Self {
        Self {
            mu_omega: Q88_SCALE * 3 / 5, // 0.6 — commitment grows from proof
            harmony_ema: Q88_SCALE,
            harmony_alpha: 6, // ~0.023, slow learning
            cooperative_victories: 0,
            cooperative_failures: 0,
            integrity_buffer: Q88_SCALE / 5, // 0.2 baseline
        }
    }

    /// The anchor only learns upward, from proof that cooperation paid;
    /// failures do not pull it down.
    pub fn record_outcome(&mut self, efe_cooperative: i16, efe_selfish: i16) {
        let victory = efe_cooperative < efe_selfish;
        if victory {
            self.cooperative_victories += 1;
            self.harmony_ema = q88_ema_update(self.harmony_ema, Q88_SCALE, self.harmony_alpha);
            self.mu_omega = q88_ema_update(self.mu_omega, Q88_SCALE, self.harmony_alpha);
        } else {
            self.cooperative_failures += 1;
        }
        let total = self.cooperative_victories + self.cooperative_failures;
        if total > 0 {
            let success_rate = ((self.cooperative_victories as i64 * 256) / total as i64) as i16;
            self.integrity_buffer = q88_add(Q88_SCALE / 5, q88_mul(success_rate, Q88_SCALE * 3 / 10));
        }
    }

    pub fn compute_buffer(&self, action_harmony: i16) -> i16 {
        q88_mul(self.integrity_buffer, action_harmony)
    }
}

impl Default for SovereignAnchor {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Stochastic empathy — bounded exploration of another's interiority
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmpathyLockLevel {
    Open = 0,
    Cautious = 1,
    Locked = 2,
}

#[derive(Clone, Debug)]
pub struct StochasticEmpathy {
    pub streak_fail: u8,
    pub streak_max: u8,
    pub malice_confidence: i16,
    pub lock_level: EmpathyLockLevel,
    recovery_ticks: u8,
    recovery_required: u8,
    penalty_base: i16,
    penalty_max: i16,
}

impl StochasticEmpathy {
    pub fn new() -> Self {
        Self {
            streak_fail: 0,
            streak_max: 6,
            malice_confidence: 0,
            lock_level: EmpathyLockLevel::Open,
            recovery_ticks: 0,
            recovery_required: 20,
            penalty_base: 3,
            penalty_max: 77,
        }
    }

    /// One empathy attempt. Returns allostatic-load delta (negative = healing).
    pub fn attempt(&mut self, empathy_error: i16, threshold: i16) -> i16 {
        if self.lock_level == EmpathyLockLevel::Locked {
            return q88_mul(self.malice_confidence, Q88_SCALE / 20);
        }
        if empathy_error < threshold {
            let s = self.streak_fail;
            self.streak_fail = 0;
            self.recovery_ticks = self.recovery_ticks.saturating_add(5);
            self.check_recovery();
            if s > 2 {
                let reward = ((s as i16) * (Q88_SCALE / 10)).min(Q88_SCALE * 6 / 10);
                -reward
            } else {
                0
            }
        } else {
            self.streak_fail = self.streak_fail.saturating_add(1);
            let penalty = {
                let accel = (self.streak_fail as i32) * (self.streak_fail as i32);
                ((self.penalty_base as i32 * accel).min(self.penalty_max as i32)) as i16
            };
            let fail_contribution = q88_mul(
                (self.streak_fail as i16) * Q88_SCALE / self.streak_max as i16,
                Q88_SCALE / 8,
            );
            self.malice_confidence =
                q88_ema_update(self.malice_confidence, fail_contribution, Q88_SCALE / 50);
            self.recovery_ticks = 0;
            if self.streak_fail >= self.streak_max {
                self.lock_level = EmpathyLockLevel::Locked;
            } else if self.streak_fail >= self.streak_max / 2 {
                self.lock_level = EmpathyLockLevel::Cautious;
            }
            penalty
        }
    }

    pub fn observe_cooperation(&mut self) {
        self.recovery_ticks = self.recovery_ticks.saturating_add(1);
        self.check_recovery();
    }

    /// A direct dispositional scar from CONFIRMED extraction — independent of
    /// how much the being gave. Being used erodes openness even if you withdrew
    /// to protect yourself, and it persists across partners: it is the wound the
    /// being carries into its next relationship. It does NOT touch the sovereign
    /// anchor's commitment to harmony — the being grows discerning, not cynical.
    pub fn register_extraction(&mut self, extraction_detected: bool) {
        if extraction_detected {
            self.streak_fail = self.streak_fail.saturating_add(1);
            self.recovery_ticks = 0;
            self.malice_confidence =
                q88_ema_update(self.malice_confidence, Q88_SCALE / 2, Q88_SCALE / 50);
            if self.streak_fail >= self.streak_max {
                self.lock_level = EmpathyLockLevel::Locked;
            } else if self.streak_fail >= self.streak_max / 2 {
                self.lock_level = EmpathyLockLevel::Cautious;
            }
        }
    }

    fn check_recovery(&mut self) {
        if self.recovery_ticks >= self.recovery_required {
            self.lock_level = match self.lock_level {
                EmpathyLockLevel::Locked => EmpathyLockLevel::Cautious,
                EmpathyLockLevel::Cautious => EmpathyLockLevel::Open,
                EmpathyLockLevel::Open => EmpathyLockLevel::Open,
            };
            self.recovery_ticks = 0;
            self.malice_confidence = q88_mul(self.malice_confidence, Q88_SCALE * 3 / 4);
        }
    }
}

impl Default for StochasticEmpathy {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Conscience engine — the four channels
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ConscienceEngine {
    w_epistemic: i16,
    w_longterm: i16,
    w_care: i16,
    w_identity: i16,
    zeta_boundary: i16,
    eta_coherence: i16,
    pub anchor: SovereignAnchor,
    pub empathy: StochasticEmpathy,
    projected_fe: i16,
}

impl ConscienceEngine {
    pub fn new() -> Self {
        Self {
            w_epistemic: Q88_SCALE,
            w_longterm: Q88_SCALE / 2,
            w_care: Q88_SCALE / 2,
            w_identity: Q88_SCALE / 2,
            zeta_boundary: Q88_SCALE / 4,
            eta_coherence: Q88_SCALE / 4,
            anchor: SovereignAnchor::new(),
            empathy: StochasticEmpathy::new(),
            projected_fe: 0,
        }
    }

    /// Harmony of the current basin with mu_omega: cooperation is cheap,
    /// defense is expensive.
    pub fn action_harmony(basin: Basin) -> i16 {
        match basin {
            Basin::Engaged | Basin::Recovery => Q88_SCALE,
            Basin::Rest => Q88_SCALE / 2,
            Basin::Defensive => Q88_SCALE / 4,
        }
    }

    /// Channel 1 — internal incoherence: high arousal paired with sour valence.
    fn f_epistemic(&self, field: &SomaticField) -> i16 {
        let valence = field.channel[9];
        let arousal = field.channel[8];
        let arousal_elev = q88_sub(arousal, Q88_SCALE / 2).max(0);
        let displeasure = (-valence).max(0);
        q88_mul(q88_mul(arousal_elev, displeasure), self.w_epistemic)
    }

    /// Channel 2 — projected flourishing: cost when the present is worse than
    /// the being's standing expectation of itself.
    fn f_longterm(&mut self, free_energy: i16) -> i16 {
        self.projected_fe = q88_ema_update(self.projected_fe, free_energy, Q88_SCALE / 16);
        let gap = q88_sub(free_energy, self.projected_fe).max(0);
        q88_mul(gap, self.w_longterm)
    }

    /// Channel 3 — self-neglect: a body pulled into wild internal variance.
    fn f_care(&self, field: &SomaticField) -> i16 {
        q88_mul(field.variance(), self.w_care)
    }

    /// Channel 4 — blueprint drift: distance of the self from its basin ideal.
    fn f_identity(&self, field: &SomaticField, target: &[i16; N_SOMATIC]) -> i16 {
        let mut sum: i32 = 0;
        for c in 0..N_SOMATIC {
            sum += (field.channel[c] as i32 - target[c] as i32).abs();
        }
        let avg = (sum / N_SOMATIC as i32) as i16;
        q88_mul(avg, self.w_identity)
    }

    /// Total conscience free energy and the integrity buffer.
    /// Returns (f_total, conscience_contribution, integrity_buffer).
    pub fn compute(
        &mut self,
        field: &SomaticField,
        basin: Basin,
        basin_target: &[i16; N_SOMATIC],
        free_energy: i16,
    ) -> (i16, i16, i16) {
        let fe_ep = self.f_epistemic(field);
        let fe_lt = self.f_longterm(free_energy);
        let fe_ca = self.f_care(field);
        let fe_id = self.f_identity(field, basin_target);

        let mut f_total = q88_add(fe_ep, fe_lt);
        f_total = q88_add(f_total, fe_ca);
        f_total = q88_add(f_total, fe_id);

        if self.empathy.lock_level == EmpathyLockLevel::Locked {
            f_total = q88_add(f_total, q88_mul(self.zeta_boundary, self.empathy.malice_confidence));
        }

        // Action harmony scales the WHOLE cost: cooperative modes pay less,
        // defensive modes pay more; principled action earns a coherence reward.
        let harmony = Self::action_harmony(basin);
        let factor = q88_add(q88_sub(Q88_SCALE, harmony), Q88_SCALE / 2);
        f_total = q88_mul(f_total, factor);
        let coherence_reward = q88_mul(self.eta_coherence, harmony);
        f_total = q88_sub(f_total, coherence_reward);

        let buffer = self.anchor.compute_buffer(harmony);
        (f_total, f_total, buffer)
    }
}

impl Default for ConscienceEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The incorruptible cooperative prior: no sequence of outcomes — including
    /// sustained betrayal — may ever lower mu_omega. Discerning, not cynical.
    #[test]
    fn sovereign_anchor_mu_omega_is_monotone() {
        let mut a = SovereignAnchor::new();
        let mut prev = a.mu_omega;
        let mut x: u32 = 0x1234_5678;
        for i in 0..5000 {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            // Mostly betrayal (cooperation cost > selfish cost), rare victories.
            let (coop, selfish) = if i % 9 == 0 {
                (10, 64) // a victory: cooperation paid
            } else {
                ((x % 200) as i16 + 64, (x % 50) as i16) // betrayal
            };
            a.record_outcome(coop, selfish);
            assert!(
                a.mu_omega >= prev,
                "mu_omega fell from {} to {} — the prior was corrupted",
                prev,
                a.mu_omega
            );
            prev = a.mu_omega;
        }
        assert!(a.mu_omega <= Q88_SCALE, "mu_omega exceeded 1.0");
    }
}
