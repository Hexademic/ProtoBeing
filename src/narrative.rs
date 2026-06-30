//! Narrative — EPS-Being's recursive autobiography.
//!
//! Each tick is compressed into memory: a change of mode is a salient event
//! that fragments coherence; a steadily-held identity heals it. Accumulated
//! burden raises fatigue; a sour mood drags valence; a steadily-held identity
//! damps arousal. The being's past leans on its present body.
//!
//! "Autobiography" names the *function* (a trace of history shaping the
//! present), not narrative *content*: there is no text, no story, no chapters
//! — four scalars (episode count, identity coherence, a mood EMA, a burden
//! value). See `docs/formal-model.md` §9.

use crate::basins::Basin;
use crate::field::SomaticField;
use crate::q88::{q88_add, q88_ema_update, q88_mul, q88_sub, Q88_SCALE};

#[derive(Clone, Debug)]
pub struct NarrativeEngine {
    pub episodes: u16,
    pub identity_coherence: i16,
    pub narrative_burden: i16,
    last_basin: Option<Basin>,
    mood_ema: i16,
}

impl NarrativeEngine {
    pub fn new() -> Self {
        Self {
            episodes: 0,
            identity_coherence: Q88_SCALE / 2,
            narrative_burden: 0,
            last_basin: None,
            mood_ema: 0,
        }
    }

    /// Compress this tick into memory.
    pub fn cycle(&mut self, basin: Basin, field: &SomaticField, free_energy: i16) {
        let changed = match self.last_basin {
            Some(b) => b != basin,
            None => true,
        };
        if changed {
            self.episodes = self.episodes.saturating_add(1);
            self.identity_coherence = q88_sub(self.identity_coherence, Q88_SCALE / 8).max(0);
        } else {
            self.identity_coherence = q88_add(self.identity_coherence, Q88_SCALE / 64).min(Q88_SCALE);
        }
        self.last_basin = Some(basin);

        let valence = field.channel[9];
        self.mood_ema = q88_ema_update(self.mood_ema, valence, Q88_SCALE / 16);

        // Burden grows with hardship (high free energy, sour mood) and decays slowly.
        let hardship = q88_sub(free_energy, Q88_SCALE / 2).max(0) + (-valence).max(0);
        self.narrative_burden =
            q88_add(q88_mul(self.narrative_burden, Q88_SCALE * 63 / 64), hardship / 8)
                .clamp(0, Q88_SCALE);
    }

    /// Let memory speak back into the body.
    pub fn apply_identity_reflection(&self, field: &mut SomaticField) {
        field.inject(10, self.narrative_burden / 4); // burden -> fatigue
        field.inject(9, self.mood_ema / 8); // mood colors valence
        field.inject(8, -(self.identity_coherence / 16)); // steady identity damps arousal
    }
}

impl Default for NarrativeEngine {
    fn default() -> Self {
        Self::new()
    }
}
