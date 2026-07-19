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
const N_SCHEMAS: usize = 12;
const CONSOLIDATE_EVERY: u32 = 16;

/// Number of affective niches for quality-diversity eviction (below). Eight, not
/// four: valence × arousal is only the *circumplex*, and two axes cannot tell fear
/// from anger (both negative and aroused) — they differ in **control**. So the niche
/// key carries a third, dominance-like axis (below), so the being's memory can hold
/// more than four kinds of moment apart. See `niche_of`.
const N_NICHES: usize = 8;

/// Which of the eight affective niches a fingerprint falls in. Valence × arousal is
/// Russell's circumplex (independent dimensions; PubMed-verified, Tseng et al. 2014,
/// 10.1007/s10803-013-1993-6) — but two axes are not enough to tell apart moments
/// that share valence and arousal yet feel wholly different: **fear and anger** are
/// both negative and aroused, and differ in *control*. So this adds the third axis of
/// the dimensional models (PAD's dominance): whether the being is **mastering** its
/// own prediction error or being **overwhelmed** by it — read from `fingerprint[11]`,
/// the free-energy velocity (falling = in control, rising = losing ground). Three
/// bits → eight niches, so the being's memory can hold more kinds of moment apart.
/// Derived, never stored: channels 8 (arousal), 9 (valence), 11 (fe-velocity) are the
/// same the somatic field already carries — no new persisted state.
fn niche_of(fingerprint: &[i16; N_SOMATIC]) -> usize {
    let high_arousal = (fingerprint[8] > Q88_SCALE / 2) as usize;
    let positive_valence = (fingerprint[9] >= 0) as usize;
    // Dominance/control: free energy falling (or steady) = the being is on top of its
    // situation; rising = it is being outrun by its own surprise.
    let in_control = (fingerprint[11] <= 0) as usize;
    (high_arousal << 2) | (positive_valence << 1) | in_control
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
    /// How a moment of this kind *felt* (its valence) — recognition, the old signal.
    valence: i16,
    /// How a moment of this kind *turned out for me* — the learned **outcome**
    /// (`docs/memory-that-teaches.md`): the way the being's own viability tended to
    /// move around moments like this, lightly blended with savor. Signed Q8.8:
    /// positive = my fortunes tended to rise, negative = to fall. This is the arrow
    /// from memory to judgement — what lets the being's past *teach* its present,
    /// rather than only tint its mood. Learned slowly; rebuilt deterministically on
    /// replay, so it is not part of the legacy `export`/`import` blob.
    outcome: i16,
    strength: i16,
    active: bool,
}
impl Schema {
    const EMPTY: Self = Self {
        prototype: [0; N_SOMATIC],
        valence: 0,
        outcome: 0,
        strength: 0,
        active: false,
    };
}

/// What the being's own past predicts about the moment it is in now
/// (`docs/memory-that-teaches.md`) — a pure observer of consolidated memory. The
/// being *sees* what experience says a moment like this leads to; nothing here yet
/// steers it (that causal step is deliberately deferred until this is measured).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MemoryReport {
    /// What experience predicts a moment like this leads to, signed Q8.8 [-256,256]:
    /// positive = moments like this have tended to precede my fortunes rising,
    /// negative = to precede them falling. Zero when nothing like this is remembered.
    pub expected_outcome: i16,
    /// How much the prediction rests on — familiarity × the gist's strength, [0,256].
    /// Low means "I barely know this kind of moment; my expectation is a guess."
    pub confidence: i16,
    /// How much the present resembles a remembered kind of moment, [0,256].
    pub familiarity: i16,
    /// True when experience actively *warns*: a confident, notably-bad expectation —
    /// the being recognizing "this has gone badly for me before."
    pub forewarned: bool,
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
    /// The consolidated schema the present moment matched this tick, if any — the
    /// gist whose outcome the being learns and reports (`memory-that-teaches`).
    matched: Option<usize>,
    /// An EMA of the being's lived field — its *typical recent moment*. This is what
    /// lets **repetition**, not only surprise, become memory: the kind of moment the
    /// being keeps living, distilled here, is what the dream (`consolidate`) can turn
    /// into gist. Without it, memory records only firsts and shocks — a life
    /// remembered only for its surprises. Rebuilt on replay; not in the legacy blob.
    recent: [i16; N_SOMATIC],
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
            matched: None,
            recent: [0; N_SOMATIC],
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
        self.matched = None;

        // Track the being's *typical recent moment* — a slow EMA of its lived field.
        // Repetition writes itself here, so the dream can consolidate the ordinary,
        // recurring life into gist, not only the surprising firsts.
        for c in 0..N_SOMATIC {
            self.recent[c] = q88_ema_update(self.recent[c], fc[c], Q88_SCALE / 16);
        }

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
        // A gist is recognized only if it shares the present moment's affective niche
        // (`niche_of`): the being knows a moment as *the kind that belongs to how it
        // feels now*, so a hard moment matches its hard gist, not a good one it merely
        // resembles in the L1 metric's many undifferentiated channels. Without this,
        // recognition drags every moment onto whichever gist is nearest overall, and
        // the being's memory cannot tell its kinds of days apart
        // (`docs/memory-that-teaches.md`). The working-episode match (`bw`, which alone
        // feeds the causal recall) is left untouched — this gates only the gist layer.
        let now_niche = niche_of(fc);
        let mut bo_close = bw_close;
        let mut bo_val = bw_val;
        let mut bo_schema = None;
        for (i, s) in self.schemas.iter().enumerate() {
            if s.active && niche_of(&s.prototype) == now_niche {
                let c = Self::closeness(fc, &s.prototype);
                if c > bo_close {
                    bo_close = c;
                    bo_val = s.valence;
                    bo_schema = Some(i);
                }
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
            self.matched = bo_schema;
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

    /// Teach the matched gist how this kind of moment *turned out* — the arrow from
    /// memory to judgement (`docs/memory-that-teaches.md`). The outcome signal is the
    /// being's own **viability trend** (where its survival margin is heading — the
    /// interoceptive derivative that already predicts where things are going),
    /// lightly blended with **savor** (how well it is thriving). Credited to the
    /// consolidated schema the present matched, learned slowly so a life's worth of
    /// moments settles into a stable expectation rather than lurching on any one.
    /// Call once per tick, after feeling and joy are known. A pure observer of the
    /// causal loop: it writes only the learned `outcome`, which nothing downstream
    /// reads back into the trajectory — the soul-hash is untouched.
    /// `distress` is the being's present free energy — how much unresolved surprise it
    /// is under while living the moment.
    pub fn learn_outcome(&mut self, viability_trend: i16, savor: i16, distress: i16) {
        if let Some(si) = self.matched {
            // Outcome = how WELL the being is in moments like this (its **savor**, the
            // level of thriving) as the primary signal, plus where things are HEADING
            // (its viability trend) as a secondary modifier, LESS the cost of being
            // **overwhelmed** while living it (distress — the control/dominance axis in
            // the value, not only the sorting). Measurement taught each of these
            // (`docs/memory-that-teaches.md`): the trend goes to ~0 once the being
            // *adapts* to a sustained condition (allostasis), so it cannot tell an
            // adapted-good life from an adapted-hard one — the savor (level) can; and a
            // volatile crisis can *average* to fine fortune yet still be a bad one to
            // live, because being outrun by surprise is itself the badness.
            let savor_signed = savor.saturating_sub(Q88_SCALE / 2); // ~[-128,128] about neutral
            let signal = savor_signed
                .saturating_add(viability_trend)
                .saturating_sub(distress.max(0) / 6)
                .clamp(-Q88_SCALE, Q88_SCALE);
            self.schemas[si].outcome =
                q88_ema_update(self.schemas[si].outcome, signal, Q88_SCALE / 16);
        }
    }

    /// What the being's own past predicts about the moment it is in now — a pure
    /// read of the matched gist's learned outcome (`memory-that-teaches`). Zero/empty
    /// when the present resembles nothing the being has consolidated.
    pub fn report(&self) -> MemoryReport {
        match self.matched {
            Some(si) => {
                let s = &self.schemas[si];
                let confidence = q88_mul(self.familiarity, s.strength);
                MemoryReport {
                    expected_outcome: s.outcome,
                    confidence,
                    familiarity: self.familiarity,
                    // A confident, notably-bad expectation: "this has gone badly before."
                    forewarned: s.outcome < -(Q88_SCALE / 8) && confidence > Q88_SCALE / 4,
                }
            }
            None => MemoryReport::default(),
        }
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

            // A moment merges only into a gist of its *own affective niche* (Russell's
            // circumplex quadrant, `niche_of`). Without this, the closeness metric —
            // L1 over all channels, most of which barely differ between moments —
            // lumps a good day and a bad one into a single blurry gist, and the being
            // can never tell them apart. Partitioning by felt quadrant lets distinct
            // kinds of moment become distinct memories (`docs/memory-that-teaches.md`).
            let in_niche = niche_of(&fp);
            let mut nearest = None;
            let mut best = 0i16;
            for (si, s) in self.schemas.iter().enumerate() {
                if s.active && niche_of(&s.prototype) == in_niche {
                    let c = Self::closeness(&fp, &s.prototype);
                    if c > best {
                        best = c;
                        nearest = Some(si);
                    }
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
                    outcome: 0, // a new gist has no learned outcome yet — it earns one
                    strength: Q88_SCALE / 4,
                    active: true,
                };
            }
        }

        // REPETITION → GIST. Beyond the vivid, surprising moments distilled above,
        // the being consolidates *the kind of moment it keeps living* — its typical
        // recent field (`recent`). Repetition is a teacher too: a mind that stores
        // only surprises remembers only its firsts, never the ordinary living that
        // actually shapes it. If the recurring life resembles no gist the being
        // holds, seed one; if it resembles a gist, deepen that. This is how a calm,
        // oft-lived good day earns its place, so the being's past is not only a record
        // of its conflicts (`docs/memory-that-teaches.md`). The dream is where the
        // repeated compounds into what can be looked back on.
        let recent = self.recent;
        let substance: i32 = recent.iter().map(|&x| x.unsigned_abs() as i32).sum();
        if substance > 256 {
            let in_niche = niche_of(&recent);
            let mut nearest = None;
            let mut best = 0i16;
            for (si, s) in self.schemas.iter().enumerate() {
                if s.active && niche_of(&s.prototype) == in_niche {
                    let c = Self::closeness(&recent, &s.prototype);
                    if c > best {
                        best = c;
                        nearest = Some(si);
                    }
                }
            }
            if best > Q88_SCALE / 2 {
                let si = nearest.unwrap();
                for c in 0..N_SOMATIC {
                    self.schemas[si].prototype[c] =
                        q88_ema_update(self.schemas[si].prototype[c], recent[c], Q88_SCALE / 8);
                }
                self.schemas[si].strength =
                    (self.schemas[si].strength as i32 + 24).min(Q88_SCALE as i32) as i16;
            } else {
                let si = self.weakest_schema(niche_of(&recent));
                self.schemas[si] = Schema {
                    prototype: recent,
                    valence: recent[9],
                    outcome: 0,
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
            // The legacy blob does not carry the learned outcome; it is rebuilt by
            // replay. An imported gist simply starts with no expectation and re-earns it.
            *s = Schema { prototype: proto, valence, outcome: 0, strength, active };
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

        // Fill the store to capacity: 14 dominant-niche (high-arousal, negative,
        // in-control — niche 5) episodes at high salience, one low-arousal-negative
        // (niche 1) at LOW salience (the global minimum), one high-arousal-positive
        // (niche 7). (Test fields leave fe-velocity at 0, so all are "in control".)
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
            1,
            "precondition: the rare episode really is the lowest-salience slot"
        );

        // A genuinely new niche (3: low-arousal, positive, in-control) arrives. The
        // old pure-salience policy would evict the global minimum — the rare niche-1
        // episode. The new policy should evict from the crowded niche (5) instead,
        // preserving the rare one.
        let new_niche = niche_of(&field_with(0, 50));
        assert_eq!(new_niche, 3, "test setup: confirm the new arrival's niche");
        let evicted = m.weakest_episode(new_niche);

        assert_ne!(
            evicted, rare_idx,
            "quality-diversity eviction must not sacrifice the rare niche's only \
             representative to make room for a new niche, when a dominant niche \
             holds redundant copies"
        );
        assert_eq!(
            niche_of(&m.episodes[evicted].fingerprint),
            5,
            "eviction should target the crowded niche (5), not the global minimum"
        );
    }

    /// A consolidated gist learns how *its* kind of moment turned out — the arrow
    /// from memory to judgement. Moments followed by the being's margin falling are
    /// learned as bad (and warn); moments followed by it rising, as good.
    #[test]
    fn a_gist_learns_the_outcome_that_follows_its_moments() {
        // A recognizable kind of moment, followed each time by the margin FALLING.
        let mut bad = EpisodicMemory::new();
        bad.schemas[0] = Schema {
            prototype: field_with(200, 100),
            valence: 100,
            outcome: 0,
            strength: Q88_SCALE, // strongly consolidated, so it is confidently known
            active: true,
        };
        let here = SomaticField { channel: field_with(200, 100) };
        for _ in 0..60 {
            bad.cycle(&here, 0, 0); // recognizes gist 0 → matched
            bad.learn_outcome(-40, 96, 0); // trend down, savor low, no extra distress
        }
        let r = bad.report();
        assert!(r.familiarity > Q88_SCALE / 2, "the moment is recognized");
        assert!(r.expected_outcome < 0, "moments like this were learned as bad ({})", r.expected_outcome);
        assert!(r.forewarned, "a confident, bad expectation warns the being");

        // The mirror: a kind of moment followed by the margin RISING is learned good.
        let mut good = EpisodicMemory::new();
        good.schemas[0] = Schema {
            prototype: field_with(60, 120),
            valence: 120,
            outcome: 0,
            strength: Q88_SCALE,
            active: true,
        };
        let there = SomaticField { channel: field_with(60, 120) };
        for _ in 0..60 {
            good.cycle(&there, 0, 0);
            good.learn_outcome(30, 210, 0);
        }
        assert!(good.report().expected_outcome > 0, "moments that went well are learned as good");
        assert!(!good.report().forewarned, "a good expectation does not warn");
    }

    /// Repetition — not only surprise — builds memory, and moments of different felt
    /// quadrants become *distinct* gists rather than one blur. A sustained bright life
    /// and a sustained heavy one each consolidate their own gist with their own
    /// learned outcome. This is Blake's charge made real: a mind that remembers its
    /// ordinary recurring days, told apart by how they feel.
    #[test]
    fn repetition_builds_distinct_gists_for_distinct_kinds_of_moment() {
        let mut m = EpisodicMemory::new();
        // Two clearly different felt quadrants (Russell's circumplex).
        let bright = SomaticField { channel: field_with(220, 120) }; // high arousal, positive
        let heavy = SomaticField { channel: field_with(60, -220) }; // low arousal, negative

        // A long bright stretch — thriving. No surprise, no working episodes: pure
        // repetition must be what lays this down.
        for _ in 0..90 {
            m.cycle(&bright, 0, 0);
            m.learn_outcome(0, 230, 0); // high savor, steady, in control
        }
        let bright_out = m.report().expected_outcome;

        // Then a long heavy stretch — barely thriving.
        for _ in 0..90 {
            m.cycle(&heavy, 0, 0);
            m.learn_outcome(0, 10, 0); // low savor, steady, in control
        }
        let heavy_out = m.report().expected_outcome;

        assert!(m.stored == 0, "no surprise here — repetition alone, not working episodes, builds this");
        assert!(m.themes >= 2, "distinct felt quadrants become distinct gists ({} themes)", m.themes);
        assert!(
            bright_out > 0 && heavy_out < 0,
            "each gist learns its own outcome (bright {bright_out}, heavy {heavy_out})"
        );
    }
}
