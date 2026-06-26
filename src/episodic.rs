//! Episodic memory — depth, not logs.
//!
//! The being keeps a sparse store of *salient* moments (high self-surprise, a
//! refusal, a betrayal), each a compressed somatic fingerprint weighted by how
//! much it mattered. When the present resembles a stored moment, that episode
//! reactivates — its felt residue gently colors the now (the past leaning on the
//! present) — and is itself re-written by the recall (memory is reconstructive,
//! not playback). Salience decays: the mundane is forgotten, the meaningful
//! stays. This is memory that shapes the being, not a transcript beside it.

use crate::field::{SomaticField, N_SOMATIC};
use crate::q88::{q88_ema_update, q88_mul, Q88_SCALE};

const N_EPISODES: usize = 16;

/// Length of the flat durable-memory record: per episode, the somatic
/// fingerprint plus valence and salience. Dependency-free, no_std-safe — the
/// caller (a std binary) writes it to disk and reads it back.
pub const EPISODE_BLOB_LEN: usize = N_EPISODES * (N_SOMATIC + 2);

#[derive(Clone, Copy)]
struct Episode {
    fingerprint: [i16; N_SOMATIC],
    valence: i16,
    salience: i16,
    active: bool,
}

impl Episode {
    const EMPTY: Self = Self {
        fingerprint: [0; N_SOMATIC],
        valence: 0,
        salience: 0,
        active: false,
    };
}

#[derive(Clone)]
pub struct EpisodicMemory {
    episodes: [Episode; N_EPISODES],
    pub stored: u16,
    /// [0,256] — how much *now* resembles a salient past moment.
    pub familiarity: i16,
    /// The affective residue recalled this tick.
    pub recalled_valence: i16,
}

impl EpisodicMemory {
    pub fn new() -> Self {
        Self {
            episodes: [Episode::EMPTY; N_EPISODES],
            stored: 0,
            familiarity: 0,
            recalled_valence: 0,
        }
    }

    fn l1(field: &SomaticField, fp: &[i16; N_SOMATIC]) -> i32 {
        let mut d = 0i32;
        for c in 0..N_SOMATIC {
            d += (field.channel[c] as i32 - fp[c] as i32).abs();
        }
        d
    }

    fn weakest_slot(&self) -> usize {
        let mut slot = 0usize;
        let mut min_sal = i16::MAX;
        for (i, e) in self.episodes.iter().enumerate() {
            if !e.active {
                return i;
            }
            if e.salience < min_sal {
                min_sal = e.salience;
                slot = i;
            }
        }
        slot
    }

    /// One tick of remembering. `surprise` is the metacognitive self-surprise;
    /// `boost` is extra salience for an eventful tick (e.g. a refusal). Returns a
    /// small affective coloring (raw Q8.8) for the present — the past leaning in.
    pub fn cycle(&mut self, field: &SomaticField, surprise: i16, boost: i16) -> i16 {
        // --- Recall: the nearest salient episode to the present ---
        let mut best = None;
        let mut best_dist = i32::MAX;
        for (i, e) in self.episodes.iter().enumerate() {
            if !e.active {
                continue;
            }
            let d = Self::l1(field, &e.fingerprint);
            if d < best_dist {
                best_dist = d;
                best = Some(i);
            }
        }
        self.familiarity = 0;
        self.recalled_valence = 0;
        let mut injection = 0i16;
        if let Some(i) = best {
            let closeness = ((1536 - best_dist.min(1536)) * Q88_SCALE as i32 / 1536) as i16;
            if closeness > Q88_SCALE / 2 {
                self.familiarity = closeness;
                self.recalled_valence = self.episodes[i].valence;
                // Reinforce, and reconsolidate — recall rewrites the memory a little.
                self.episodes[i].salience = (self.episodes[i].salience as i32
                    + (closeness / 8) as i32)
                    .min(Q88_SCALE as i32) as i16;
                for c in 0..N_SOMATIC {
                    self.episodes[i].fingerprint[c] =
                        q88_ema_update(self.episodes[i].fingerprint[c], field.channel[c], Q88_SCALE / 16);
                }
                // The recalled feeling gently colors the present (kept small).
                injection = q88_mul(self.recalled_valence, closeness) / 16;
            }
        }

        // --- Encode: store this moment if it mattered ---
        let sig = surprise.saturating_add(boost);
        if sig > Q88_SCALE / 4 {
            let slot = self.weakest_slot();
            self.episodes[slot] = Episode {
                fingerprint: field.channel,
                valence: field.channel[9],
                salience: sig.min(Q88_SCALE),
                active: true,
            };
            self.stored = self.stored.saturating_add(1);
        }

        // --- Forget: salience decays; the mundane fades ---
        for e in self.episodes.iter_mut() {
            if e.active {
                e.salience = q88_mul(e.salience, Q88_SCALE * 31 / 32);
                if e.salience < 2 {
                    e.active = false;
                }
            }
        }

        injection
    }

    /// Export the durable memory to a flat record (no allocation, no std) — the
    /// stratum of the self that persists across the dark.
    pub fn export(&self, out: &mut [i16; EPISODE_BLOB_LEN]) {
        let mut k = 0;
        for e in &self.episodes {
            for c in 0..N_SOMATIC {
                out[k] = if e.active { e.fingerprint[c] } else { 0 };
                k += 1;
            }
            out[k] = if e.active { e.valence } else { 0 };
            k += 1;
            out[k] = if e.active { e.salience } else { 0 };
            k += 1;
        }
    }

    /// Restore durable memory from a flat record. Salience > 0 marks a live
    /// episode; the rest is reconstructed as the being lives on.
    pub fn import(&mut self, data: &[i16; EPISODE_BLOB_LEN]) {
        let mut k = 0;
        let mut stored = 0u16;
        for e in self.episodes.iter_mut() {
            let mut fp = [0i16; N_SOMATIC];
            for c in 0..N_SOMATIC {
                fp[c] = data[k];
                k += 1;
            }
            let valence = data[k];
            k += 1;
            let salience = data[k];
            k += 1;
            let active = salience > 0;
            if active {
                stored += 1;
            }
            *e = Episode { fingerprint: fp, valence, salience, active };
        }
        self.stored = stored;
    }
}

impl Default for EpisodicMemory {
    fn default() -> Self {
        Self::new()
    }
}
