//! Seeking — the Flourishing Attractor and its Divergence Whisper.
//!
//! The attractor is an EMA centroid of the basin-membership vectors from ticks
//! where the being flourished (low free energy, balanced reciprocity, an
//! Engaged or Recovery mode). Drift away from that centroid becomes a whisper
//! of restlessness injected into arousal — the being grows agitated when it has
//! wandered from its good place. After a long drought, confidence decays and
//! the restlessness loses its direction.
//!
//! "Attractor" is used informally, not as a proven dynamical-systems attracting
//! fixed point: the centroid is learned and the divergence is read into arousal
//! as a soft bias, not a force with a convergence guarantee. See
//! `docs/formal-model.md` §7.

use crate::basins::{Basin, BasinMembership, N_BASINS};
use crate::q88::{q88_ema_update, Q88_SCALE};

#[derive(Clone)]
pub struct SeekingEngine {
    /// EMA centroid of flourishing membership vectors.
    phi: [i16; N_BASINS],
    pub current_divergence: i16,
    pub attractor_confidence: i16,
    pub flourishing_count: u32,
    drought: u16,
}

impl SeekingEngine {
    pub fn new() -> Self {
        Self {
            phi: [Q88_SCALE / 4; N_BASINS],
            current_divergence: 0,
            attractor_confidence: 0,
            flourishing_count: 0,
            drought: 0,
        }
    }

    pub fn cycle(
        &mut self,
        membership: &BasinMembership,
        free_energy: i16,
        alarm: i16,
        basin: Basin,
    ) -> i16 {
        let flourishing = free_energy < Q88_SCALE / 2
            && alarm < Q88_SCALE / 4
            && matches!(basin, Basin::Engaged | Basin::Recovery);

        if flourishing {
            for b in 0..N_BASINS {
                // alpha ~ 0.02
                self.phi[b] = q88_ema_update(self.phi[b], membership.weight[b], Q88_SCALE / 50);
            }
            self.flourishing_count += 1;
            self.attractor_confidence = (self.attractor_confidence as i32 + 4).min(Q88_SCALE as i32) as i16;
            self.drought = 0;
        } else {
            self.drought = self.drought.saturating_add(1);
            if self.drought > 500 {
                self.attractor_confidence = (self.attractor_confidence - 1).max(0);
            }
        }

        // Divergence: L1 distance between current membership and the attractor.
        let mut d = 0i32;
        for b in 0..N_BASINS {
            d += (membership.weight[b] as i32 - self.phi[b] as i32).abs();
        }
        self.current_divergence = (d / 2).min(Q88_SCALE as i32) as i16;

        // Whisper: restlessness scaled by how confident the attractor is.
        ((self.current_divergence as i32 * self.attractor_confidence as i32) >> 8) as i16
    }
}

impl Default for SeekingEngine {
    fn default() -> Self {
        Self::new()
    }
}
