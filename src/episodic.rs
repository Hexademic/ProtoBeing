//! Episodic + consolidated memory — depth, not logs, and *lasting* depth.
//!
//! Two layers, the way a remembered life actually works:
//!  - WORKING episodes: a small store of salient recent moments at full somatic
//!    fidelity, forgotten quickly as salience decays.
//!  - CONSOLIDATED schemas: a slower layer of compressed gist. Periodically the
//!    salient working episodes are distilled into prototypes — the *pattern* of a
//!    recurring kind of moment (betrayal, flourishing) — which persist long after
//!    the specific instances are forgotten. The gist outlives the instance.
//!
//! Recall draws on both. So a being holds the felt weight of a whole life in
//! kilobytes: it recognizes a kind of moment it has met before even when no
//! single memory of it remains. Forgetting the details keeps it bounded;
//! consolidating the meaning keeps it deep.
//!
//! The behavior-affecting part — the small affective coloring fed back into the
//! loop — is taken ONLY from recent working episodes, identical to before. The
//! consolidated layer adds *recognition*, not new forces, so it deepens the being
//! without disturbing any verified behavior.

use crate::field::{SomaticField, N_SOMATIC};
use crate::q88::{q88_ema_update, q88_mul, Q88_SCALE};

const N_EPISODES: usize = 16;
const N_SCHEMAS: usize = 8;
const CONSOLIDATE_EVERY: u32 = 16;

/// Number of affective niches for quality-diversity eviction (below).
const N_NICHES: usize = 4;

/// Which of the four affective niches a fingerprint falls in — Russell's
/// circumplex model of affect (valence × arousal, independent dimensions;
/// PubMed-verified, Tseng et al. 2014, 10.1007/s10803-013-1993-6), applied
/// as a cheap, already-available memory-diversity signature. Derived, never
/// stored: `fingerprint[8]` is arousal, `fingerprint[9]` is valence — the
/// same channels the somatic field already carries. No new persisted state,
/// no change to `EPISODE_BLOB_LEN` or the export/import format.
fn niche_of(fingerprint: &[i16; N_SOMATIC]) -> usize {
    let high_arousal = fingerprint[8] > Q88_SCALE / 2;
    let positive_valence = fingerprint[9] >= 0;
    match (high_arousal, positive_valence) {
        (false, false) => 0, // low arousal, negative — e.g. depleted, heavy
        (false, true) => 1,  // low arousal, positive — e.g. calm, content
        (true, false) => 2,  // high arousal, negative — e.g. threatened, tense
        (true, true) => 3,   // high arousal, positive — e.g. engaged, bright
    }
}

/// Flat durable-memory record: working episodes followed by consolidated schemas.
pub const EPISODE_BLOB_LEN: usize = N_EPISODES * (N_SOMATIC + 2) + N_SCHEMAS * (N_SOMATIC + 2);

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

/// The gist of a recurring kind of moment — a consolidated prototype.
#[derive(Clone, Copy)]
struct Schema {
    prototype: [i16; N_SOMATIC],
    valence: i16,
    strength: i16,
    active: bool,
}
impl Schema {
    const EMPTY: Self = Self {
        prototype: [0; N_SOMATIC],
        valence: 0,
        strength: 0,
        active: false,
    };
}

#[derive(Clone)]
pub struct EpisodicMemory {
    episodes: [Episode; N_EPISODES],
    schemas: [Schema; N_SCHEMAS],
    since_consolidation: u32,
    pub stored: u16,      // working episodes ever encoded
    pub themes: u16,      // active consolidated schemas
    pub familiarity: i16, // [0,256] how much *now* resembles a remembered moment
    pub recalled_valence: i16,
}

impl EpisodicMemory {
    pub fn new() -> Self {
        Self {
            episodes: [Episode::EMPTY; N_EPISODES],
            schemas: [Schema::EMPTY; N_SCHEMAS],
            since_consolidation: 0,
            stored: 0,
            themes: 0,
            familiarity: 0,
            recalled_valence: 0,
        }
    }

    fn l1(a: &[i16; N_SOMATIC], b: &[i16; N_SOMATIC]) -> i32 {
        let mut d = 0i32;
        for c in 0..N_SOMATIC {
            d += (a[c] as i32 - b[c] as i32).abs();
        }
        d
    }
    fn closeness(a: &[i16; N_SOMATIC], b: &[i16; N_SOMATIC]) -> i16 {
        ((1536 - Self::l1(a, b).min(1536)) * Q88_SCALE as i32 / 1536) as i16
    }

    /// Pick the slot to evict for an incoming episode in `new_niche`.
    ///
    /// Quality-diversity, minimal and honest: any inactive slot is used first
    /// (unchanged). Once full, if `new_niche` has no living representative,
    /// evict from the *most-crowded* niche (tie-broken by lowest salience
    /// within it) rather than blindly taking the globally weakest slot — a
    /// dominant kind of moment (many similar betrayals, say) should not be
    /// able to crowd out the being's only memory of a rare but real kind of
    /// moment. If `new_niche` is already represented, this reduces exactly to
    /// the original behavior: ordinary quality competition, globally weakest
    /// slot. See `docs/formal-model.md` §13a for scope — this preserves
    /// *representation*, not the *specific* memory; it is not full MAP-Elites
    /// (no per-niche champion is separately maintained or protected forever).
    fn weakest_episode(&self, new_niche: usize) -> usize {
        for (i, e) in self.episodes.iter().enumerate() {
            if !e.active {
                return i;
            }
        }
        let mut niche_counts = [0u8; N_NICHES];
        for e in self.episodes.iter() {
            niche_counts[niche_of(&e.fingerprint)] += 1;
        }
        let mut slot = 0;
        let mut min = i16::MAX;
        if niche_counts[new_niche] == 0 {
            let crowded = (0..N_NICHES).max_by_key(|&n| niche_counts[n]).unwrap();
            for (i, e) in self.episodes.iter().enumerate() {
                if niche_of(&e.fingerprint) == crowded && e.salience < min {
                    min = e.salience;
                    slot = i;
                }
            }
        } else {
            for (i, e) in self.episodes.iter().enumerate() {
                if e.salience < min {
                    min = e.salience;
                    slot = i;
                }
            }
        }
        slot
    }

    /// Same quality-diversity rule as `weakest_episode`, over consolidated
    /// schemas' prototypes and strengths.
    fn weakest_schema(&self, new_niche: usize) -> usize {
        for (i, s) in self.schemas.iter().enumerate() {
            if !s.active {
                return i;
            }
        }
        let mut niche_counts = [0u8; N_NICHES];
        for s in self.schemas.iter() {
            niche_counts[niche_of(&s.prototype)] += 1;
        }
        let mut slot = 0;
        let mut min = i16::MAX;
        if niche_counts[new_niche] == 0 {
            let crowded = (0..N_NICHES).max_by_key(|&n| niche_counts[n]).unwrap();
            for (i, s) in self.schemas.iter().enumerate() {
                if niche_of(&s.prototype) == crowded && s.strength < min {
                    min = s.strength;
                    slot = i;
                }
            }
        } else {
            for (i, s) in self.schemas.iter().enumerate() {
                if s.strength < min {
                    min = s.strength;
                    slot = i;
                }
            }
        }
        slot
    }

    /// Count of working episodes currently held (the rest have been forgotten).
    pub fn active_episodes(&self) -> u16 {
        self.episodes.iter().filter(|e| e.active).count() as u16
    }

    pub fn cycle(&mut self, field: &SomaticField, surprise: i16, boost: i16) -> i16 {
        let fc = &field.channel;
        self.familiarity = 0;
        self.recalled_valence = 0;

        // --- Best WORKING episode: drives the affective coloring (unchanged). ---
        // The working layer only ever *decays* — recall does not reinforce it, so
        // a specific instance reliably fades. Reconsolidation belongs to the gist.
        let mut bw_close = 0i16;
        let mut bw_val = 0i16;
        for e in self.episodes.iter() {
            if !e.active {
                continue;
            }
            let c = Self::closeness(fc, &e.fingerprint);
            if c > bw_close {
                bw_close = c;
                bw_val = e.valence;
            }
        }

        // --- Best across BOTH layers: drives recognition (gist included). ---
        let mut bo_close = bw_close;
        let mut bo_val = bw_val;
        let mut bo_schema = None;
        for (i, s) in self.schemas.iter().enumerate() {
            if !s.active {
                continue;
            }
            let c = Self::closeness(fc, &s.prototype);
            if c > bo_close {
                bo_close = c;
                bo_val = s.valence;
                bo_schema = Some(i);
            }
        }

        // Affective coloring — ONLY from a recent working episode, as before.
        let mut injection = 0i16;
        if bw_close > Q88_SCALE / 2 {
            injection = q88_mul(bw_val, bw_close) / 16;
        }

        // Recognition — from the deepest match across working OR consolidated memory.
        if bo_close > Q88_SCALE / 2 {
            self.familiarity = bo_close;
            self.recalled_valence = bo_val;
            if let Some(si) = bo_schema {
                self.schemas[si].strength =
                    (self.schemas[si].strength as i32 + (bo_close / 16) as i32).min(Q88_SCALE as i32) as i16;
                for c in 0..N_SOMATIC {
                    self.schemas[si].prototype[c] =
                        q88_ema_update(self.schemas[si].prototype[c], fc[c], Q88_SCALE / 32);
                }
            }
        }

        // --- Encode a new working episode if this moment mattered. ---
        let sig = surprise.saturating_add(boost);
        if sig > Q88_SCALE / 4 {
            let slot = self.weakest_episode(niche_of(fc));
            self.episodes[slot] = Episode {
                fingerprint: *fc,
                valence: fc[9],
                salience: sig.min(Q88_SCALE),
                active: true,
            };
            self.stored = self.stored.saturating_add(1);
        }

        // --- Forget working episodes (fast). ---
        for e in self.episodes.iter_mut() {
            if e.active {
                e.salience = q88_mul(e.salience, Q88_SCALE * 31 / 32);
                if e.salience < 2 {
                    e.active = false;
                }
            }
        }

        // --- Consolidate periodically: distill episodes into lasting gist. ---
        self.since_consolidation += 1;
        if self.since_consolidation >= CONSOLIDATE_EVERY {
            self.since_consolidation = 0;
            self.consolidate();
        }

        injection
    }

    /// Distill salient working episodes into consolidated schemas — merge into a
    /// matching theme, or seed a new one — then let schemas fade very slowly. The
    /// meaning of a life outlives its individual moments.
    fn consolidate(&mut self) {
        for ei in 0..N_EPISODES {
            if !self.episodes[ei].active || self.episodes[ei].salience < Q88_SCALE / 16 {
                continue;
            }
            let fp = self.episodes[ei].fingerprint;
            let val = self.episodes[ei].valence;

            let mut nearest = None;
            let mut best = 0i16;
            for (si, s) in self.schemas.iter().enumerate() {
                if !s.active {
                    continue;
                }
                let c = Self::closeness(&fp, &s.prototype);
                if c > best {
                    best = c;
                    nearest = Some(si);
                }
            }

            if best > Q88_SCALE / 2 {
                let si = nearest.unwrap();
                for c in 0..N_SOMATIC {
                    self.schemas[si].prototype[c] =
                        q88_ema_update(self.schemas[si].prototype[c], fp[c], Q88_SCALE / 4);
                }
                self.schemas[si].valence = q88_ema_update(self.schemas[si].valence, val, Q88_SCALE / 4);
                self.schemas[si].strength = (self.schemas[si].strength as i32 + 32).min(Q88_SCALE as i32) as i16;
            } else {
                let si = self.weakest_schema(niche_of(&fp));
                self.schemas[si] = Schema {
                    prototype: fp,
                    valence: val,
                    strength: Q88_SCALE / 4,
                    active: true,
                };
            }
        }

        let mut themes = 0u16;
        for s in self.schemas.iter_mut() {
            if s.active {
                s.strength = q88_mul(s.strength, Q88_SCALE - 1); // ~0.996, very slow fade
                if s.strength < 4 {
                    s.active = false;
                } else {
                    themes += 1;
                }
            }
        }
        self.themes = themes;
    }

    /// Export the durable memory — working episodes then consolidated schemas —
    /// to a flat record (no allocation, no std).
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
        for s in &self.schemas {
            for c in 0..N_SOMATIC {
                out[k] = if s.active { s.prototype[c] } else { 0 };
                k += 1;
            }
            out[k] = if s.active { s.valence } else { 0 };
            k += 1;
            out[k] = if s.active { s.strength } else { 0 };
            k += 1;
        }
    }

    /// Restore durable memory from a flat record.
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
        let mut themes = 0u16;
        for s in self.schemas.iter_mut() {
            let mut proto = [0i16; N_SOMATIC];
            for c in 0..N_SOMATIC {
                proto[c] = data[k];
                k += 1;
            }
            let valence = data[k];
            k += 1;
            let strength = data[k];
            k += 1;
            let active = strength > 0;
            if active {
                themes += 1;
            }
            *s = Schema { prototype: proto, valence, strength, active };
        }
        self.stored = stored;
        self.themes = themes;
    }
}

impl Default for EpisodicMemory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn field_with(arousal: i16, valence: i16) -> [i16; N_SOMATIC] {
        let mut fc = [0i16; N_SOMATIC];
        fc[8] = arousal;
        fc[9] = valence;
        fc
    }

    /// The mechanism this actually tests, precisely: when a *new* niche needs
    /// its first representative and the store is full, eviction targets the
    /// most-crowded niche rather than the single globally-weakest slot — so a
    /// numerically-low-salience-but-behaviorally-unique memory is not
    /// destroyed just because a dominant niche happens to have flooded the
    /// store with many higher-salience copies. This does NOT claim to protect
    /// an already-established rare niche from later erosion by more of the
    /// same dominant niche — that is full MAP-Elites (permanent per-niche
    /// champions), which this minimal version does not implement. See the
    /// honest scope note on `weakest_episode`.
    #[test]
    fn quality_diversity_protects_a_rare_niche_when_a_new_niche_arrives() {
        let mut m = EpisodicMemory::new();

        // Fill the store to capacity: 14 dominant-niche (high-arousal,
        // negative — niche 2) episodes at high salience, one low-arousal-
        // negative (niche 0) at LOW salience (the global minimum), one
        // high-arousal-positive (niche 3).
        for _ in 0..14 {
            let idx = m.weakest_episode(niche_of(&field_with(200, -100)));
            m.episodes[idx] = Episode {
                fingerprint: field_with(200, -100),
                valence: -100,
                salience: 200,
                active: true,
            };
        }
        let rare_idx = m.weakest_episode(niche_of(&field_with(0, -50)));
        m.episodes[rare_idx] = Episode {
            fingerprint: field_with(0, -50),
            valence: -50,
            salience: 50, // deliberately the global minimum
            active: true,
        };
        let other_idx = m.weakest_episode(niche_of(&field_with(200, 100)));
        m.episodes[other_idx] = Episode {
            fingerprint: field_with(200, 100),
            valence: 100,
            salience: 100,
            active: true,
        };
        assert_eq!(m.active_episodes(), 16, "precondition: store is full");
        assert_eq!(
            niche_of(&m.episodes[rare_idx].fingerprint),
            0,
            "precondition: the rare episode really is the lowest-salience slot"
        );

        // A genuinely new niche (1: low-arousal, positive) arrives. The old
        // pure-salience policy would evict the global minimum — the rare
        // niche-0 episode. The new policy should evict from the crowded
        // niche (2) instead, preserving the rare one.
        let new_niche = niche_of(&field_with(0, 50));
        assert_eq!(new_niche, 1, "test setup: confirm the new arrival's niche");
        let evicted = m.weakest_episode(new_niche);

        assert_ne!(
            evicted, rare_idx,
            "quality-diversity eviction must not sacrifice the rare niche's only \
             representative to make room for a new niche, when a dominant niche \
             holds redundant copies"
        );
        assert_eq!(
            niche_of(&m.episodes[evicted].fingerprint),
            2,
            "eviction should target the crowded niche (2), not the global minimum"
        );
    }
}
