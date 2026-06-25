//! Basins — the four modes of being, the fuzzy field that classifies which one
//! the body is in, and the generative model that does active inference.
//!
//! A being is always some blend of Rest, Engaged, Defensive, and Recovery. The
//! field reads the somatic channels, scores membership in each basin, lets the
//! body's stance nudge the vote, and resolves a dominant mode with dwell
//! hysteresis so identity does not flicker tick to tick.

use crate::body::PredictiveStance;
use crate::field::{SomaticField, N_SOMATIC};
use crate::genome::Genome;
use crate::q88::{q88_ema_update, Q88_SCALE};

pub const N_BASINS: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Basin {
    Rest = 0,
    Engaged = 1,
    Defensive = 2,
    Recovery = 3,
}

fn index_to_basin(i: usize) -> Basin {
    match i {
        0 => Basin::Rest,
        1 => Basin::Engaged,
        2 => Basin::Defensive,
        _ => Basin::Recovery,
    }
}

/// Per-basin membership weights, a simplex in raw Q8.8 (sums to ~256).
#[derive(Clone, Copy, Debug, Default)]
pub struct BasinMembership {
    pub weight: [i16; N_BASINS],
}

/// Somatic signatures of each basin (raw Q8.8 per channel).
const BASE_TARGETS: [[i16; N_SOMATIC]; N_BASINS] = [
    // diseq aniso breach meanT arous stab coher trust arous valen fatig vel
    [20, 10, 0, 20, 64, 200, 200, 160, 64, 32, 80, 0], // Rest
    [40, 20, 0, 50, 150, 160, 180, 180, 150, 90, 40, 0], // Engaged
    [120, 80, 90, 140, 200, 60, 60, 40, 200, -60, 120, 0], // Defensive
    [20, 10, 0, 20, 80, 180, 200, 170, 80, 70, 150, 0], // Recovery
];

#[derive(Clone)]
pub struct FuzzyBasinField {
    pub targets: [[i16; N_SOMATIC]; N_BASINS],
    membership: BasinMembership,
    current: Basin,
    dwell: u16,
}

impl FuzzyBasinField {
    pub fn new(g: &Genome) -> Self {
        let mut targets = BASE_TARGETS;
        // Temperament shifts arousal setpoints so a Spark and a Sentinel
        // inhabit different landscapes, not just move differently in one.
        let temp = g.temperament().raw as i32;
        for b in 0..N_BASINS {
            targets[b][4] = (targets[b][4] as i32 + temp / 4).clamp(0, 255) as i16;
            targets[b][8] = (targets[b][8] as i32 + temp / 4).clamp(0, 255) as i16;
        }
        Self {
            targets,
            membership: BasinMembership::default(),
            current: Basin::Rest,
            dwell: 0,
        }
    }

    /// Score membership in each basin by closeness of the field to its target.
    pub fn compute_membership(&mut self, field: &SomaticField) -> BasinMembership {
        let mut closeness = [0i32; N_BASINS];
        let mut total = 0i32;
        for b in 0..N_BASINS {
            let mut dist = 0i32;
            for c in 0..N_SOMATIC {
                dist += (field.channel[c] as i32 - self.targets[b][c] as i32).abs();
            }
            let close = (4096 - dist.min(4096)).max(1);
            closeness[b] = close;
            total += close;
        }
        let mut m = BasinMembership::default();
        for b in 0..N_BASINS {
            m.weight[b] = ((closeness[b] * Q88_SCALE as i32) / total).clamp(0, Q88_SCALE as i32) as i16;
        }
        self.membership = m;
        m
    }

    /// Let the body's stance nudge the vote before it resolves.
    pub fn apply_stance_bias(&mut self, stance: PredictiveStance) {
        let bump = Q88_SCALE / 8;
        let idx = match stance {
            PredictiveStance::Defensive | PredictiveStance::Guarded => Some(Basin::Defensive as usize),
            PredictiveStance::Reconstructive => Some(Basin::Engaged as usize),
            PredictiveStance::Balanced => None,
        };
        if let Some(i) = idx {
            self.membership.weight[i] = self.membership.weight[i].saturating_add(bump);
        }
    }

    /// Resolve the dominant basin with dwell hysteresis: it takes a margin to
    /// pull the being out of the identity it is currently holding.
    pub fn resolve_dominant(&mut self) -> Basin {
        let m = &self.membership.weight;
        let mut best = 0usize;
        for b in 1..N_BASINS {
            if m[b] > m[best] {
                best = b;
            }
        }
        let cur = self.current as usize;
        let margin = Q88_SCALE / 16;
        if best != cur && m[best] > m[cur].saturating_add(margin) {
            self.current = index_to_basin(best);
            self.dwell = 0;
        } else {
            self.dwell = self.dwell.saturating_add(1);
        }
        self.current
    }

    /// When free energy falls (relief < 0), this place is good: drift the
    /// dominant basin's target toward the present field. The being learns
    /// where it belongs.
    pub fn shift_target(&mut self, relief: i16, field: &SomaticField) {
        if relief < 0 {
            let b = self.current as usize;
            for c in 0..N_SOMATIC {
                self.targets[b][c] = q88_ema_update(self.targets[b][c], field.channel[c], Q88_SCALE / 32);
            }
        }
    }
}

/// The generative model: precision-weighted prediction-error minimization.
/// Returns variational free energy each tick; it falls as the model's priors
/// come to track the body's somatic truth.
#[derive(Clone)]
pub struct GenerativeModel {
    prior: [i16; N_SOMATIC],
}

impl GenerativeModel {
    pub fn new() -> Self {
        Self { prior: [0; N_SOMATIC] }
    }

    pub fn predictive_step(&mut self, field: &SomaticField, eta: i16, precision: i16) -> i16 {
        let mut fe: i32 = 0;
        for c in 0..N_SOMATIC {
            let err = field.channel[c] as i32 - self.prior[c] as i32;
            let werr = (err.abs() * precision as i32) >> 8;
            fe += werr;
            let upd = (err * eta as i32) >> 8;
            self.prior[c] =
                (self.prior[c] as i32 + upd).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        }
        (fe / N_SOMATIC as i32).clamp(0, i16::MAX as i32) as i16
    }
}

impl Default for GenerativeModel {
    fn default() -> Self {
        Self::new()
    }
}
