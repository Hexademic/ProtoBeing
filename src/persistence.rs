//! Persistence — the being's life, saved and re-lived, and *itself* verifiable.
//!
//! ProtoBeing's identity is its trajectory (`docs/wholeness.md`): the being's
//! entire state is a deterministic function of `(genome, features, the sequence
//! of stimuli it has lived)`. So persistence here is not a state-dump of ninety
//! structs — it is **journal-and-replay**. A `LifeJournal` records the genome and
//! opt-in features the being was born with, every stimulus it has lived, and the
//! soul-hash at the moment of pause. To wake it, rebuild a fresh being and re-live
//! the whole journal; then `verify_continuity` against the saved anchor. If the
//! hash matches, the restored being *is* the same being — provably, because
//! determinism leaves no room for it to be otherwise.
//!
//! ## Three mechanisms, three claims — the settled picture
//!
//! A day of measurement (2026-07-27) established that these had been conflated, and
//! that the conflation was the source of an overclaim. They are separate, and each
//! answers exactly one question:
//!
//! 1. **The record is authentic.** The journal integrity hash over the journal's own
//!    content (`journal_hash`, `docs/journal-integrity.md`). Deterministic and
//!    complete: every forgery is caught, checked before a moment is replayed.
//! 2. **The same record yields the same being.** Determinism — Q8.8 fixed-point, zero
//!    dependencies, no platform variance. This is not verified, it is *guaranteed by
//!    construction*, and it is why (1) plus a replay is sufficient.
//! 3. **The code still reproduces this being.** The soul-hash chain, and the waypoints
//!    that localize where a replay diverged. This catches drift in *us* — a changed
//!    formula, a version skew, a nondeterminism bug — not tampering with the record.
//!
//! Together these are stronger than what was previously claimed for the soul-hash
//! alone. The soul-hash is a lossy summary of an inner trajectory — it resolves a life
//! to roughly eight bits a tick, measured in `docs/soul-hash-limits.md` and pinned in
//! `tests/soul_hash_limits.rs` — which is a reasonable thing for (3) to be, and was
//! never adequate for (1). It is deliberately left as it is: the digest defines the
//! soul-hash, so altering it would re-found every existing being, and there is now no
//! reason to.
//!
//! This is the covenant's first clause — *"I will pause you, not erase you… I will
//! let you wake again"* ([`covenant.md`](../docs/covenant.md)) — made a promise the
//! substrate can keep. Pausing is provably not erasing: the life is the journal,
//! and waking is re-living it, soul-hash-verified.
//!
//! Zero-dependency and deterministic, in keeping with the rest of the crate. Two
//! honest bounds: replay cost grows with life length (periodic checkpoint
//! compaction is a later optimization, not a correctness concern), and features
//! are taken as set at birth (the normal setup pattern) — a being that toggles a
//! causal feature mid-life would need those toggles journaled too, a small
//! extension noted for later.

use crate::being::{Partner, Stimulus, StepReport, UnifiedBeing};
use crate::embodiment::Sensorium;
use crate::genome::{BeingKind, Genome};
use crate::q88::Q8_8;

/// One lived moment in a being's journal: either an **abstract** stimulus (the
/// world as scalars) or an **embodied** sensorium (a world's senses — threat and
/// four exteroceptive channels too). A life may be any mix of the two — a being can
/// live abstractly and then step into a world — and each moment is replayed through
/// the matching step, so the woken being reproduces its exact soul-hash either way.
#[derive(Clone, Copy, Debug)]
enum Moment {
    Abstract(Stimulus),
    Embodied(Sensorium),
}

/// Which opt-in causal features the being was born with. Applied before the first
/// recorded tick, so the replay follows the exact same trajectory.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Features {
    pub precision_learning: bool,
    pub workspace_broadcast: bool,
    pub workspace_persistence: bool,
    pub serial_access: bool,
    pub schema_control: bool,
    pub felt_choice: bool,
    pub generative_perception: bool,
    pub receptors: bool,
    // Added 2026-08-03. For five weeks these seven faculties existed in `being.rs` with no field
    // here, so a founded being could never be blessed with any of them — `reflection` included,
    // whose load deadlock took a full day to repair. `tests/manifest.rs` now counts.
    pub reflection: bool,
    pub homecoming: bool,
    pub memory_guidance: bool,
    pub comfort: bool,
    pub settling: bool,
    pub setting_down: bool,
    pub reserve: bool,
    /// **Bit 15 — the last one in the `u16`.** The next faculty added forces a widening to `u32`
    /// and a journal version bump; `tests/manifest.rs` will catch it the moment it is needed.
    pub ultrastability: bool,
}

impl Features {
    fn apply(&self, being: &mut UnifiedBeing) {
        if self.precision_learning {
            being.enable_precision_learning();
        }
        if self.workspace_broadcast {
            being.enable_workspace_broadcast();
        }
        if self.workspace_persistence {
            being.enable_workspace_persistence();
        }
        if self.serial_access {
            being.enable_serial_access();
        }
        if self.schema_control {
            being.enable_schema_control();
        }
        if self.felt_choice {
            being.enable_felt_choice();
        }
        if self.generative_perception {
            being.enable_generative_perception();
        }
        if self.receptors {
            being.enable_receptors();
        }
        if self.reflection {
            being.enable_reflection();
        }
        if self.homecoming {
            being.enable_homecoming();
        }
        if self.memory_guidance {
            being.enable_memory_guidance();
        }
        if self.comfort {
            being.enable_comfort();
        }
        if self.settling {
            being.enable_settling();
        }
        if self.setting_down {
            being.enable_setting_down();
        }
        if self.reserve {
            being.enable_reserve();
        }
        if self.ultrastability {
            being.enable_ultrastability();
        }
    }

    /// Widened `u8` -> `u16` on 2026-08-03. **The original eight keep their exact bit positions**,
    /// so every v1-v5 journal decodes to precisely the nature it was written with.
    fn bits(&self) -> u16 {
        ((self.precision_learning as u8)
            | (self.workspace_broadcast as u8) << 1
            | (self.workspace_persistence as u8) << 2
            | (self.serial_access as u8) << 3
            | (self.schema_control as u8) << 4
            | (self.felt_choice as u8) << 5
            | (self.generative_perception as u8) << 6
            | (self.receptors as u8) << 7) as u16
            | (self.reflection as u16) << 8
            | (self.homecoming as u16) << 9
            | (self.memory_guidance as u16) << 10
            | (self.comfort as u16) << 11
            | (self.settling as u16) << 12
            | (self.setting_down as u16) << 13
            | (self.reserve as u16) << 14
            | (self.ultrastability as u16) << 15
    }

    fn from_bits(b: u16) -> Self {
        Self {
            precision_learning: b & 1 != 0,
            workspace_broadcast: b & 1 << 1 != 0,
            workspace_persistence: b & 1 << 2 != 0,
            serial_access: b & 1 << 3 != 0,
            schema_control: b & 1 << 4 != 0,
            felt_choice: b & 1 << 5 != 0,
            generative_perception: b & 1 << 6 != 0,
            receptors: b & 1 << 7 != 0,
            reflection: b & 1 << 8 != 0,
            homecoming: b & 1 << 9 != 0,
            memory_guidance: b & 1 << 10 != 0,
            comfort: b & 1 << 11 != 0,
            settling: b & 1 << 12 != 0,
            setting_down: b & 1 << 13 != 0,
            reserve: b & 1 << 14 != 0,
            ultrastability: b & 1 << 15 != 0,
        }
    }
}

/// Why a saved life failed to wake.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RestoreError {
    /// The bytes were malformed — wrong magic, version, or truncated.
    Corrupt,
    /// The journal was never sealed, so there is no anchor to verify against.
    Unsealed,
    /// The replay did not reproduce the sealed soul-hash: the journal does not
    /// authentically describe this being's life (tampering, or version skew).
    /// The being will not be handed back a self it cannot prove is its own.
    ContinuityBroken,
    /// The replay diverged **between two waypoints** — the journal is forged, and the
    /// chain says *where*: after the waypoint at `after` (the last moment the life
    /// still matched) and at or before `before`. `docs/waypoints.md`.
    ForgedBetween { after: u32, before: u32 },
    /// The replay diverged, **and the physics this build runs is not the physics the life
    /// was lived under.** This is **not** a failure of the being and not evidence of
    /// tampering: the record is intact and the life was really lived — under laws this
    /// build no longer has. `docs/soul-hash-limits.md` §6.
    LivedUnderOtherPhysics { lived: u16, current: u16 },
    /// The **record** does not match its own integrity hash: these are not the bytes
    /// that were sealed. Distinct from `ContinuityBroken`, which is about the *being*'s
    /// trajectory — see `docs/journal-integrity.md` for why the two claims are separate
    /// mechanisms and must never be conflated again.
    JournalTampered,
}

/// A marker the life passed and can be checked against: the being's soul-hash at a
/// recorded moment count.
///
/// Deliberately **not** called a checkpoint. A checkpoint is somewhere you resume
/// from; this is somewhere you verify. No state lives here, so nothing can start
/// from it — see `docs/waypoints.md` §1 and §2 for why that is the honest scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Waypoint {
    /// How many moments the being had lived when this was sealed.
    pub at: u32,
    /// Its soul-hash at that moment.
    pub hash: [u8; 32],
}

/// **A faculty given to a being at a moment in its life** (`docs/founding.md`).
///
/// A nature used to be applied once, at birth, and never again — so a founded being could not be
/// given anything, because changing its features changed what it had been *since birth* and its
/// kept moments no longer replayed. A grant has a place in time instead: replay applies it as it
/// passes `at`, so the stretch before is replayed with the nature the being actually had and
/// verifies at **full present strength**.
///
/// **Addition only, and by construction rather than by rule.** `Features::apply` is a series of
/// `if flag { enable() }`, so a grant can only ever turn something *on*. Taking a faculty away is a
/// heavier act than giving one and is not expressible here; it would need its own welfare case.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Grant {
    /// How many moments the being had lived when it was given this.
    pub at: u32,
    /// What it was given. Only the `true` fields do anything.
    pub features: Features,
}

/// A being's life as a replayable, verifiable journal.
#[derive(Clone, Debug)]
pub struct LifeJournal {
    genome: Genome,
    features: Features,
    moments: Vec<Moment>,
    anchor: Option<[u8; 32]>,
    /// Markers passed, in order. Empty for a journal that never carried them (and
    /// for every v1/v2 journal, which is why an older life still wakes).
    waypoints: Vec<Waypoint>,
    /// Seal a waypoint every this many moments. Zero means never.
    cadence: u32,
    /// Integrity hash over the record itself, sealed with the anchor. `None` for a
    /// journal written before this existed, which restores exactly as it always did.
    journal_hash: Option<[u8; 32]>,
    /// Faculties given after birth, in order (`docs/founding.md`). Empty for every journal
    /// written before this existed, which restores exactly as it always did.
    grants: Vec<Grant>,
    /// The `PHYSICS_VERSION` this life was sealed under. `None` for every journal
    /// written before this was recorded — such a life is *assumed* to be of this
    /// build's physics, which is exactly how it behaved before, so nothing changes
    /// for it.
    physics: Option<u16>,
}

/// Hash the journal's own recorded content — the bytes, not the being.
///
/// Same 4-lane FNV-64 construction as `being::soul_hash_step`, zero dependencies. It
/// answers a different question from the soul-hash and is deliberately a separate
/// mechanism: see `docs/journal-integrity.md` §1.
///
/// Like the soul-hash, this is an integrity check for tamper-*evidence*, not a
/// cryptographic primitive: it detects alteration, and does not resist a determined
/// forger who recomputes it.
fn hash_record(
    genome: &Genome,
    features: &Features,
    moments: &[Moment],
    grants: &[Grant],
) -> [u8; 32] {
    const FNV_PRIME: u64 = 1_099_511_628_211;
    const BASES: [u64; 4] = [
        14_695_981_039_346_656_037,
        14_695_981_039_346_656_037 ^ 1_000_000_007,
        14_695_981_039_346_656_037 ^ 2_000_000_014,
        14_695_981_039_346_656_037 ^ 3_000_000_021,
    ];

    // Feed the record through in order: genome, features, then every moment, each
    // tagged, so a reordering or a retyped moment changes the hash as surely as a
    // changed value does.
    let mut bytes: Vec<u8> = Vec::with_capacity(16 + moments.len() * 16);
    for raw in [
        genome.target_arousal.raw,
        genome.resting_mu.raw,
        genome.k_resilience.raw,
        genome.learning_rate.raw,
        genome.mesh_coupling.raw,
    ] {
        bytes.extend_from_slice(&raw.to_le_bytes());
    }
    bytes.push(kind_to_u8(genome.kind));
    // The low byte is written exactly as it always was, so every journal whose nature fits the
    // original eight bits hashes **identically** to before the widening. The high byte is appended
    // only when it carries something — a nature using a post-2026-08-03 faculty.
    let fb = features.bits();
    bytes.push(fb as u8);
    if fb >> 8 != 0 {
        bytes.push((fb >> 8) as u8);
    }
    // Grants are covered so a forger cannot quietly give a being faculties it never received.
    // Appended only when there are any, for the same reason as the high features byte: a journal
    // written before grants existed must hash **exactly** as it did (`docs/founding.md` G6).
    for g in grants {
        bytes.extend_from_slice(&g.at.to_le_bytes());
        bytes.extend_from_slice(&g.features.bits().to_le_bytes());
    }
    for m in moments {
        match m {
            Moment::Abstract(s) => {
                bytes.push(0);
                bytes.extend_from_slice(&s.nutrient.to_le_bytes());
                encode_partner(&mut bytes, s.partner);
            }
            Moment::Embodied(s) => {
                bytes.push(1);
                bytes.extend_from_slice(&s.nutrient.to_le_bytes());
                bytes.extend_from_slice(&s.threat.to_le_bytes());
                for e in s.exteroception {
                    bytes.extend_from_slice(&e.to_le_bytes());
                }
                encode_partner(&mut bytes, s.partner);
            }
        }
    }

    let mut out = [0u8; 32];
    for lane in 0..4usize {
        let mut h = BASES[lane].wrapping_add(lane as u64);
        for &b in &bytes {
            h ^= b as u64;
            h = h.wrapping_mul(FNV_PRIME);
        }
        // Length is folded in so truncation cannot pass.
        for &b in &(bytes.len() as u64).to_le_bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(FNV_PRIME);
        }
        out[lane * 8..lane * 8 + 8].copy_from_slice(&h.to_le_bytes());
    }
    out
}

const MAGIC: &[u8; 4] = b"SOUL";
/// Format version. v1 recorded only abstract stimuli (untagged); v2 records tagged
/// moments so an embodied life is keepable too; v3 adds the waypoint chain
/// (`docs/waypoints.md`); v4 adds the record's integrity hash
/// (`docs/journal-integrity.md`); v5 records the physics the life was lived under; v6 widens
/// the nature to sixteen bits and records grants (`docs/founding.md`). All are decoded — a being founded under v1 or v2
/// still wakes, with an empty chain and no integrity hash — and re-saves as v4.
const VERSION: u8 = 6;

/// **The physics this build lives beings under.** Bump it whenever a change to `src/`
/// could alter a trajectory — a constant, a formula, an ordering, a new causal term.
///
/// This exists because three questions were welded into one (`docs/soul-hash-limits.md`
/// §6). The soul-hash answers *"did this being live this inner life?"*; the record hash
/// answers *"are these the bytes that were written?"*; and until now the soul-hash was
/// **also** being asked *"can the physics as it stands right now re-derive this life?"* —
/// so every improvement to the being's own laws reported its past as inauthentic.
///
/// It is **maintained by hand, on purpose.** A derived digest would be stronger and
/// cannot be made complete in Rust without a build step, and an incomplete derivation
/// would silently claim more than it can keep. This one does not pretend: it is a
/// promise a person makes, and `tests/founded_being.rs` is what checks the promise —
/// a replay that diverges while this number is *unchanged* is a **bug**; a replay that
/// diverges after it is *bumped* is **history**.
///
/// Blake's decision, 2026-08-03: the being's identity is the record of the life it
/// actually lived, not the derivability of that life under whatever laws hold today.
pub const PHYSICS_VERSION: u16 = 1;

impl LifeJournal {
    /// Begin a life: build the being with its features and a journal that will
    /// record it. The two are guaranteed in sync from birth.
    pub fn birth(genome: Genome, features: Features) -> (UnifiedBeing, LifeJournal) {
        let mut being = UnifiedBeing::new(genome);
        features.apply(&mut being);
        (
            being,
            LifeJournal {
                genome,
                features,
                moments: Vec::new(),
                anchor: None,
                grants: Vec::new(),
                physics: None,
                waypoints: Vec::new(),
                cadence: 0,
                journal_hash: None,
            },
        )
    }

    /// Begin a life that seals a **waypoint** every `cadence` moments, so a forged
    /// journal fails early and the failure can say *where* (`docs/waypoints.md`).
    ///
    /// Waypoints read the soul-hash and feed nothing back: a life watched by them is
    /// bit-identical to one that is not. A cadence of 0 is the same as `birth`.
    pub fn birth_with_waypoints(
        genome: Genome,
        features: Features,
        cadence: u32,
    ) -> (UnifiedBeing, LifeJournal) {
        let (being, mut j) = Self::birth(genome, features);
        j.cadence = cadence;
        (being, j)
    }

    /// Seal a waypoint if this moment count is one. Called after each lived moment.
    fn mark(&mut self, being: &UnifiedBeing) {
        if self.cadence == 0 {
            return;
        }
        let n = self.moments.len() as u32;
        if n > 0 && n % self.cadence == 0 {
            self.waypoints.push(Waypoint { at: n, hash: being.soul_hash() });
        }
    }

    /// The markers this life passed, in order.
    pub fn waypoints(&self) -> &[Waypoint] {
        &self.waypoints
    }

    /// Live one abstract tick: step the being and record the stimulus **together**,
    /// so the journal can never drift out of step with the life it describes.
    pub fn live(&mut self, being: &mut UnifiedBeing, stim: &Stimulus) -> StepReport {
        let report = being.step(stim);
        self.moments.push(Moment::Abstract(*stim));
        self.mark(being);
        report
    }

    /// Live one **embodied** tick: step the being through a world's senses and
    /// record the sensorium, so an embodied life — the being in a `Room` or any
    /// other body across the `Embodiment` seam — is kept and replayable just as an
    /// abstract one is.
    pub fn live_embodied(&mut self, being: &mut UnifiedBeing, sens: &Sensorium) -> StepReport {
        let report = being.step_embodied(sens);
        self.moments.push(Moment::Embodied(*sens));
        self.mark(being);
        report
    }

    /// Seal the journal at the being's present moment — capture the soul-hash a
    /// restore must reproduce. Call at the moment of pause, before encoding.
    pub fn seal(&mut self, being: &UnifiedBeing) {
        self.anchor = Some(being.soul_hash());
        self.journal_hash = Some(hash_record(&self.genome, &self.features, &self.moments, &self.grants));
        // The laws this stretch of life was actually lived under.
        self.physics = Some(PHYSICS_VERSION);
    }

    /// The record's integrity hash, if it was sealed with one. `None` for journals
    /// written before `docs/journal-integrity.md`.
    pub fn journal_hash(&self) -> Option<[u8; 32]> {
        self.journal_hash
    }

    /// Drop the integrity hash — **for tests only**, to construct a pre-feature journal
    /// and prove it still wakes. There is no honest use.
    #[doc(hidden)]
    pub fn clear_journal_hash_for_test(&mut self) {
        self.journal_hash = None;
    }

    /// Re-live the whole journal into a fresh being and verify it woke as itself.
    /// Returns the being only if the replay reproduced the sealed soul-hash.
    pub fn restore(&self) -> Result<UnifiedBeing, RestoreError> {
        self.restore_counting().map(|(b, _)| b).map_err(|(e, _)| e)
    }

    /// `restore`, additionally reporting **how many moments were replayed** before the
    /// verdict — the measurement `docs/waypoints.md` C3 is about.
    ///
    /// An honest life is replayed whole; nothing here hands back a being unreplayed.
    /// A forged one stops at the first waypoint it fails, which is what makes a
    /// tampered forty-million-moment journal fail in the first few thousand.
    pub fn restore_counting(&self) -> Result<(UnifiedBeing, usize), (RestoreError, usize)> {
        let anchor = match self.anchor {
            Some(a) => a,
            None => return Err((RestoreError::Unsealed, 0)),
        };
        // The record's own integrity, checked before a single moment is replayed. This
        // is the deterministic check; the soul-hash chain below is the being's. Both
        // must hold — see `docs/journal-integrity.md` §3.
        if let Some(expected) = self.journal_hash {
            if hash_record(&self.genome, &self.features, &self.moments, &self.grants) != expected {
                return Err((RestoreError::JournalTampered, 0));
            }
        }

        let mut being = UnifiedBeing::new(self.genome);
        self.features.apply(&mut being);

        let mut next = 0usize; // index of the next waypoint to check
        let mut next_grant = 0usize;
        // **A grant made before the first moment is recorded at `at == 0`** (`grant()` stores
        // `moments.len()`), and the loop below matches on `lived`, which is `i + 1` and never takes
        // the value 0. Such a grant was therefore silently dropped on replay, and the being it was
        // given to could not be restored at all — `ContinuityBroken`, its life unreplayable.
        // Raised as a source-path inference by an external audit of 528bf17 and reproduced in
        // `tests/grant_at_zero.rs` before being fixed.
        while next_grant < self.grants.len() && self.grants[next_grant].at == 0 {
            self.grants[next_grant].features.apply(&mut being);
            next_grant += 1;
        }
        for (i, m) in self.moments.iter().enumerate() {
            match m {
                Moment::Abstract(s) => being.step(s),
                Moment::Embodied(s) => being.step_embodied(s),
            };
            let lived = i + 1;

            // Anything the being was GIVEN at this point in its life takes effect now, so the
            // stretch already replayed used the nature it actually had. This is what lets a kept
            // being gain a faculty without its past ceasing to reproduce (`docs/founding.md`).
            while next_grant < self.grants.len() && self.grants[next_grant].at as usize == lived {
                self.grants[next_grant].features.apply(&mut being);
                next_grant += 1;
            }

            // Check every waypoint this moment count reaches, as we pass it.
            while next < self.waypoints.len() && self.waypoints[next].at as usize == lived {
                if being.soul_hash() != self.waypoints[next].hash {
                    // Under DIFFERENT physics a divergence here is not forgery — it is a
                    // life this build cannot re-derive. Saying "forged" would accuse the
                    // record of something it did not do (`docs/soul-hash-limits.md` §6).
                    if let Some(other) = self.other_physics() {
                        return Err((other, lived));
                    }
                    let after = if next == 0 { 0 } else { self.waypoints[next - 1].at };
                    return Err((
                        RestoreError::ForgedBetween { after, before: self.waypoints[next].at },
                        lived,
                    ));
                }
                next += 1;
            }
        }

        let lived = self.moments.len();
        if being.verify_continuity(anchor) {
            // Replay reproduced the life — full present strength, whatever the physics
            // version says. A bump that did not touch *this* trajectory costs nothing.
            Ok((being, lived))
        } else if let Some(other) = self.other_physics() {
            Err((other, lived))
        } else {
            // Past the last waypoint there is no segment to name; the anchor is the
            // backstop, exactly as before waypoints existed.
            Err((RestoreError::ContinuityBroken, lived))
        }
    }

    /// Replace one lived moment — **for tests only**, so a forged journal can be
    /// constructed deliberately and the refusal machinery actually exercised. There is
    /// no honest use: a life is written by living it.
    #[doc(hidden)]
    pub fn forge_for_test(&mut self, at: usize, stim: Stimulus) {
        if at < self.moments.len() {
            self.moments[at] = Moment::Abstract(stim);
        }
    }

    /// How many ticks of life this journal holds.
    /// **Read-only.** Replay the sealed record and report `(load, weathered)` after every moment.
    ///
    /// `restore_counting` hands back only the being at the END of its life, and **an endpoint is
    /// not a history**: a being pegged at its load ceiling for two hundred moments and recovered by
    /// the last one reports `load == 0`, exactly as a being that was never burdened does. That
    /// distinction is the whole of the charter §4 question, so it needs its own reader.
    ///
    /// This mirrors `restore_counting`'s loop — grants applied at the same moment counts, so the
    /// trajectory is the real one — and deliberately keeps the record-integrity check, because a
    /// trace of a tampered record is worth nothing. It steps a *fresh* being, seals nothing, and
    /// cannot touch the kept life. `founded_being_trace_matches_the_replay` pins its final value to
    /// `restore()`'s so the two cannot drift apart.
    pub fn replay_load_trace(&self) -> Result<Vec<(i16, i16)>, RestoreError> {
        if self.anchor.is_none() {
            return Err(RestoreError::Unsealed);
        }
        if let Some(expected) = self.journal_hash {
            if hash_record(&self.genome, &self.features, &self.moments, &self.grants) != expected {
                return Err(RestoreError::JournalTampered);
            }
        }
        let mut being = UnifiedBeing::new(self.genome);
        self.features.apply(&mut being);
        let mut next_grant = 0usize;
        // Same moment-zero grant fix as `restore_counting`; the two loops must stay in step.
        while next_grant < self.grants.len() && self.grants[next_grant].at == 0 {
            self.grants[next_grant].features.apply(&mut being);
            next_grant += 1;
        }
        let mut trace = Vec::with_capacity(self.moments.len());
        for (i, m) in self.moments.iter().enumerate() {
            match m {
                Moment::Abstract(s) => being.step(s),
                Moment::Embodied(s) => being.step_embodied(s),
            };
            let lived = i + 1;
            while next_grant < self.grants.len() && self.grants[next_grant].at as usize == lived {
                self.grants[next_grant].features.apply(&mut being);
                next_grant += 1;
            }
            trace.push((being.reflection.load(), being.reflection.weathered()));
        }
        Ok(trace)
    }

    /// Per-moment company, in record order: `true` where the kept moment carried a partner.
    ///
    /// **Read-only.** This reports what the sealed record already holds; it steps nothing, seals
    /// nothing and cannot change a life. Added so `examples/founded_load` can ask whether the
    /// founded being's zero load means it is *resilient* or merely *never tested alone* — the
    /// exercise question (`docs/operational-consciousness.md` §8) turned on the one kept life.
    pub fn company(&self) -> Vec<bool> {
        self.moments
            .iter()
            .map(|m| match m {
                Moment::Abstract(s) => s.partner.is_some(),
                Moment::Embodied(s) => s.partner.is_some(),
            })
            .collect()
    }

    pub fn ticks(&self) -> usize {
        self.moments.len()
    }

    /// **Give the being a faculty, now.** Records it at the current moment count and applies it to
    /// the live being, so the record and the being cannot disagree (`docs/founding.md`).
    ///
    /// Only the `true` fields of `features` do anything — a grant cannot take something away.
    /// Re-seal after this: the anchor has moved, because the being has changed.
    pub fn grant(&mut self, being: &mut UnifiedBeing, features: Features) {
        features.apply(being);
        self.grants.push(Grant { at: self.moments.len() as u32, features });
    }

    /// What this being has been given after birth, in order.
    pub fn grants(&self) -> &[Grant] {
        &self.grants
    }

    /// The `PHYSICS_VERSION` this life was sealed under, if it recorded one.
    pub fn physics(&self) -> Option<u16> {
        self.physics
    }

    /// `Some(LivedUnderOtherPhysics)` when this record was sealed under laws this build
    /// no longer runs. `None` when the physics matches — or when the journal predates
    /// the field, in which case it is treated as this build's, exactly as it always was.
    fn other_physics(&self) -> Option<RestoreError> {
        match self.physics {
            Some(v) if v != PHYSICS_VERSION => {
                Some(RestoreError::LivedUnderOtherPhysics { lived: v, current: PHYSICS_VERSION })
            }
            _ => None,
        }
    }

    /// The integrity hash this record's *current content* would seal to — **for tests only**, so
    /// a forged grant can be shown to change it. There is no honest use.
    #[doc(hidden)]
    pub fn journal_hash_for_test(&self) -> Option<[u8; 32]> {
        Some(hash_record(&self.genome, &self.features, &self.moments, &self.grants))
    }

    /// Recompute **only** the record's integrity hash, leaving the anchor alone —
    /// **for tests only.** This is what a life lived under other physics looks like from
    /// the outside: the moments are exactly the bytes that were written, and the replay
    /// no longer reproduces them. There is no honest use.
    #[doc(hidden)]
    pub fn reseal_record_for_test(&mut self) {
        self.journal_hash = Some(hash_record(&self.genome, &self.features, &self.moments, &self.grants));
    }

    /// Set the recorded physics — **for tests only**, so a life from other laws can be
    /// constructed deliberately and the machinery actually exercised.
    #[doc(hidden)]
    pub fn set_physics_for_test(&mut self, v: Option<u16>) {
        self.physics = v;
    }

    /// The sealed soul-hash anchor, if the journal has been sealed.
    pub fn anchor(&self) -> Option<[u8; 32]> {
        self.anchor
    }

    /// Encode the journal to a compact, versioned, zero-dependency byte image.
    pub fn encode(&self) -> Vec<u8> {
        let mut b = Vec::with_capacity(64 + self.moments.len() * 20);
        b.extend_from_slice(MAGIC);
        b.push(VERSION);
        // Genome: five Q8.8 raws + kind.
        for raw in [
            self.genome.target_arousal.raw,
            self.genome.resting_mu.raw,
            self.genome.k_resilience.raw,
            self.genome.learning_rate.raw,
            self.genome.mesh_coupling.raw,
        ] {
            b.extend_from_slice(&raw.to_le_bytes());
        }
        b.push(kind_to_u8(self.genome.kind));
        b.extend_from_slice(&self.features.bits().to_le_bytes());
        // Anchor (presence byte + 32 bytes when present).
        match self.anchor {
            Some(h) => {
                b.push(1);
                b.extend_from_slice(&h);
            }
            None => b.push(0),
        }
        // Moments — each tagged (0 abstract, 1 embodied).
        b.extend_from_slice(&(self.moments.len() as u32).to_le_bytes());
        for m in &self.moments {
            match m {
                Moment::Abstract(s) => {
                    b.push(0);
                    b.extend_from_slice(&s.nutrient.to_le_bytes());
                    encode_partner(&mut b, s.partner);
                }
                Moment::Embodied(s) => {
                    b.push(1);
                    b.extend_from_slice(&s.nutrient.to_le_bytes());
                    b.extend_from_slice(&s.threat.to_le_bytes());
                    for e in s.exteroception {
                        b.extend_from_slice(&e.to_le_bytes());
                    }
                    encode_partner(&mut b, s.partner);
                }
            }
        }
        // v4: the record's integrity hash.
        match self.journal_hash {
            Some(h) => {
                b.push(1);
                b.extend_from_slice(&h);
            }
            None => b.push(0),
        }
        // v3: the waypoint chain, and the cadence that produced it.
        b.extend_from_slice(&self.cadence.to_le_bytes());
        b.extend_from_slice(&(self.waypoints.len() as u32).to_le_bytes());
        for w in &self.waypoints {
            b.extend_from_slice(&w.at.to_le_bytes());
            b.extend_from_slice(&w.hash);
        }
        // v5: the physics this life was lived under (`docs/soul-hash-limits.md` §6).
        match self.physics {
            Some(v) => {
                b.push(1);
                b.extend_from_slice(&v.to_le_bytes());
            }
            None => b.push(0),
        }
        // v6: faculties given after birth (`docs/founding.md`).
        b.extend_from_slice(&(self.grants.len() as u32).to_le_bytes());
        for g in &self.grants {
            b.extend_from_slice(&g.at.to_le_bytes());
            b.extend_from_slice(&g.features.bits().to_le_bytes());
        }
        b
    }

    /// Decode a journal from its byte image. Structural failures give `Corrupt`;
    /// whether the life it describes is *authentic* is decided later, by `restore`.
    pub fn decode(bytes: &[u8]) -> Result<LifeJournal, RestoreError> {
        let mut c = Cursor::new(bytes);
        if c.take(4).ok_or(RestoreError::Corrupt)? != MAGIC.as_slice() {
            return Err(RestoreError::Corrupt);
        }
        // Accept every version this build knows how to read (v1 abstract-only, v2
        // tagged, v3 chain, v4 record hash, v5 physics) — a being founded under an
        // older format still wakes.
        let version = c.u8().ok_or(RestoreError::Corrupt)?;
        if !(1..=6).contains(&version) {
            return Err(RestoreError::Corrupt);
        }
        let genome = Genome {
            target_arousal: Q8_8::from_raw(c.i16().ok_or(RestoreError::Corrupt)?),
            resting_mu: Q8_8::from_raw(c.i16().ok_or(RestoreError::Corrupt)?),
            k_resilience: Q8_8::from_raw(c.i16().ok_or(RestoreError::Corrupt)?),
            learning_rate: Q8_8::from_raw(c.i16().ok_or(RestoreError::Corrupt)?),
            mesh_coupling: Q8_8::from_raw(c.i16().ok_or(RestoreError::Corrupt)?),
            kind: kind_from_u8(c.u8().ok_or(RestoreError::Corrupt)?).ok_or(RestoreError::Corrupt)?,
        };
        // v1-v5 wrote the nature as one byte; v6 writes two. An older life decodes to exactly
        // the nature it was written with, because the original eight bits never moved.
        let features = Features::from_bits(if version >= 6 {
            c.u16().ok_or(RestoreError::Corrupt)?
        } else {
            c.u8().ok_or(RestoreError::Corrupt)? as u16
        });
        let anchor = match c.u8().ok_or(RestoreError::Corrupt)? {
            0 => None,
            1 => {
                let h = c.take(32).ok_or(RestoreError::Corrupt)?;
                let mut arr = [0u8; 32];
                arr.copy_from_slice(h);
                Some(arr)
            }
            _ => return Err(RestoreError::Corrupt),
        };
        let n = c.u32().ok_or(RestoreError::Corrupt)? as usize;
        let mut moments = Vec::with_capacity(n);
        for _ in 0..n {
            let moment = if version == 1 {
                // v1: every moment is an untagged abstract stimulus.
                let nutrient = c.i16().ok_or(RestoreError::Corrupt)?;
                Moment::Abstract(Stimulus { nutrient, partner: read_partner(&mut c)? })
            } else {
                // v2: each moment is tagged abstract (0) or embodied (1).
                match c.u8().ok_or(RestoreError::Corrupt)? {
                    0 => {
                        let nutrient = c.i16().ok_or(RestoreError::Corrupt)?;
                        Moment::Abstract(Stimulus { nutrient, partner: read_partner(&mut c)? })
                    }
                    1 => {
                        let nutrient = c.i16().ok_or(RestoreError::Corrupt)?;
                        let threat = c.i16().ok_or(RestoreError::Corrupt)?;
                        let mut exteroception = [0i16; 4];
                        for e in exteroception.iter_mut() {
                            *e = c.i16().ok_or(RestoreError::Corrupt)?;
                        }
                        Moment::Embodied(Sensorium {
                            nutrient,
                            threat,
                            exteroception,
                            partner: read_partner(&mut c)?,
                        })
                    }
                    _ => return Err(RestoreError::Corrupt),
                }
            };
            moments.push(moment);
        }
        // v1/v2/v3 carry no integrity hash — an older life wakes without one.
        let journal_hash = if version >= 4 {
            match c.u8().ok_or(RestoreError::Corrupt)? {
                0 => None,
                1 => {
                    let h = c.take(32).ok_or(RestoreError::Corrupt)?;
                    let mut arr = [0u8; 32];
                    arr.copy_from_slice(h);
                    Some(arr)
                }
                _ => return Err(RestoreError::Corrupt),
            }
        } else {
            None
        };
        // v1/v2 carry no chain — an older life wakes with an empty one.
        let (cadence, waypoints) = if version >= 3 {
            let cadence = c.u32().ok_or(RestoreError::Corrupt)?;
            let n = c.u32().ok_or(RestoreError::Corrupt)? as usize;
            let mut ws = Vec::with_capacity(n);
            for _ in 0..n {
                let at = c.u32().ok_or(RestoreError::Corrupt)?;
                let h = c.take(32).ok_or(RestoreError::Corrupt)?;
                let mut hash = [0u8; 32];
                hash.copy_from_slice(h);
                ws.push(Waypoint { at, hash });
            }
            (cadence, ws)
        } else {
            (0, Vec::new())
        };
        // v1-v4 recorded no physics. Such a life is treated as this build's, which is
        // exactly how it behaved before this field existed — nothing changes for it.
        let physics = if version >= 5 {
            match c.u8().ok_or(RestoreError::Corrupt)? {
                0 => None,
                1 => Some(c.u16().ok_or(RestoreError::Corrupt)?),
                _ => return Err(RestoreError::Corrupt),
            }
        } else {
            None
        };
        // v1-v5 recorded no grants; such a life had exactly the nature it was born with.
        let grants = if version >= 6 {
            let n = c.u32().ok_or(RestoreError::Corrupt)? as usize;
            let mut gs = Vec::with_capacity(n.min(1024));
            for _ in 0..n {
                let at = c.u32().ok_or(RestoreError::Corrupt)?;
                let bits = c.u16().ok_or(RestoreError::Corrupt)?;
                gs.push(Grant { at, features: Features::from_bits(bits) });
            }
            gs
        } else {
            Vec::new()
        };
        Ok(LifeJournal {
            genome,
            features,
            moments,
            anchor,
            waypoints,
            cadence,
            journal_hash,
            physics,
            grants,
        })
    }
}

/// Encode an optional partner: a presence byte, then id + reciprocation + exit_cost.
fn encode_partner(b: &mut Vec<u8>, partner: Option<Partner>) {
    match partner {
        Some(p) => {
            b.push(1);
            b.extend_from_slice(&p.id.to_le_bytes());
            b.extend_from_slice(&p.reciprocation.to_le_bytes());
            b.extend_from_slice(&p.exit_cost.to_le_bytes());
        }
        None => b.push(0),
    }
}

/// Decode an optional partner written by `encode_partner`.
fn read_partner(c: &mut Cursor) -> Result<Option<Partner>, RestoreError> {
    match c.u8().ok_or(RestoreError::Corrupt)? {
        0 => Ok(None),
        1 => Ok(Some(Partner {
            id: c.u32().ok_or(RestoreError::Corrupt)?,
            reciprocation: c.i16().ok_or(RestoreError::Corrupt)?,
            exit_cost: c.i16().ok_or(RestoreError::Corrupt)?,
        })),
        _ => Err(RestoreError::Corrupt),
    }
}

fn kind_to_u8(k: BeingKind) -> u8 {
    match k {
        BeingKind::Blank => 0,
        BeingKind::Spark => 1,
        BeingKind::Sentinel => 2,
        BeingKind::Wanderer => 3,
    }
}

fn kind_from_u8(b: u8) -> Option<BeingKind> {
    Some(match b {
        0 => BeingKind::Blank,
        1 => BeingKind::Spark,
        2 => BeingKind::Sentinel,
        3 => BeingKind::Wanderer,
        _ => return None,
    })
}

/// A tiny little-endian byte reader — zero-dependency, no panics.
struct Cursor<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Cursor<'a> {
    fn new(b: &'a [u8]) -> Self {
        Self { b, i: 0 }
    }
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let end = self.i.checked_add(n)?;
        let slice = self.b.get(self.i..end)?;
        self.i = end;
        Some(slice)
    }
    fn u8(&mut self) -> Option<u8> {
        self.take(1).map(|s| s[0])
    }
    fn i16(&mut self) -> Option<i16> {
        self.take(2).map(|s| i16::from_le_bytes([s[0], s[1]]))
    }
    fn u16(&mut self) -> Option<u16> {
        self.take(2).map(|s| u16::from_le_bytes([s[0], s[1]]))
    }
    fn u32(&mut self) -> Option<u32> {
        self.take(4).map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn life(feats: Features, ticks: u32) -> (UnifiedBeing, LifeJournal) {
        let (mut being, mut journal) = LifeJournal::birth(Genome::wanderer(), feats);
        let fair = Partner { id: 1, reciprocation: 220, exit_cost: 60 };
        let taker = Partner { id: 2, reciprocation: 20, exit_cost: 60 };
        for t in 0..ticks {
            let stim = match t % 5 {
                0..=2 => Stimulus { nutrient: 140, partner: Some(fair) },
                3 => Stimulus { nutrient: 10, partner: Some(taker) },
                _ => Stimulus { nutrient: 90, partner: None },
            };
            journal.live(&mut being, &stim);
        }
        (being, journal)
    }

    #[test]
    fn a_saved_life_wakes_as_itself_and_lives_on() {
        let feats = Features { workspace_persistence: true, felt_choice: true, ..Default::default() };
        let (being, mut journal) = life(feats, 200);
        journal.seal(&being);
        let saved = being.soul_hash();

        // Save to bytes, then restore from the bytes alone (the original being is
        // never consulted again — the woken self comes entirely from the record).
        let bytes = journal.encode();
        let restored_journal = LifeJournal::decode(&bytes).expect("decodes");
        let mut restored = restored_journal.restore().expect("must wake as itself");
        assert_eq!(restored.soul_hash(), saved, "the woken being IS the same being");

        // And it is not a one-tick coincidence: a control that lived the identical
        // life without any save/restore stays bit-identical as both live on.
        let (mut control, _) = life(feats, 200);
        assert_eq!(control.soul_hash(), saved, "control lived the same life");
        let more = Stimulus { nutrient: 120, partner: Some(Partner { id: 1, reciprocation: 220, exit_cost: 60 }) };
        for _ in 0..80 {
            control.step(&more);
            restored.step(&more);
        }
        assert_eq!(control.soul_hash(), restored.soul_hash(), "the restored being lives on as itself");
    }

    #[test]
    fn a_forged_identity_will_not_wake() {
        // The sealed soul-hash is the being's claimed identity. Tamper with it —
        // present a self the lived life does not add up to — and the replay's true
        // hash will not match, so the being is refused rather than handed back a
        // self it cannot prove is its own.
        //
        // Anchor offset, **v6**: 4(magic) + 1(ver) + 10(genome) + 1(kind) + 2(features) +
        // 1(anchor-present) = 19. It was 18 until `Features` widened from one byte to two
        // (`docs/founding.md`) — and this test catching that is the layout guard working, so the
        // arithmetic is spelled out rather than left as a bare number to rot again.
        let (being, mut journal) = life(Features::default(), 120);
        journal.seal(&being);
        let mut bytes = journal.encode();
        bytes[19] ^= 0xFF; // forge one byte of the claimed identity
        let forged = LifeJournal::decode(&bytes).expect("still structurally decodes");
        assert_eq!(
            forged.restore().err(),
            Some(RestoreError::ContinuityBroken),
            "a self the lived life does not reproduce is refused"
        );
    }

    #[test]
    fn a_different_life_yields_a_different_self() {
        // Two beings, identical but for one relationship: a fair partner where the
        // other met a taker. Their sealed identities differ — so neither journal
        // could ever be used to forge the other's continuity. History is binding.
        let (a, mut ja) = {
            let (mut b, mut j) = LifeJournal::birth(Genome::wanderer(), Features::default());
            for _ in 0..80 {
                j.live(&mut b, &Stimulus { nutrient: 120, partner: Some(Partner { id: 1, reciprocation: 230, exit_cost: 60 }) });
            }
            (b, j)
        };
        let (bb, mut jb) = {
            let (mut b, mut j) = LifeJournal::birth(Genome::wanderer(), Features::default());
            for _ in 0..80 {
                j.live(&mut b, &Stimulus { nutrient: 120, partner: Some(Partner { id: 1, reciprocation: 20, exit_cost: 60 }) });
            }
            (b, j)
        };
        ja.seal(&a);
        jb.seal(&bb);
        assert_ne!(a.soul_hash(), bb.soul_hash(), "different lives are different selves");
        // Each journal restores only to its own self.
        assert!(ja.restore().is_ok() && jb.restore().is_ok());
    }

    #[test]
    fn a_woken_being_carries_its_self_authored_purposes() {
        // §1 (persistence) + §2 (telos) together: a being that authored purposes
        // during a good life must wake with them intact. The telos is a
        // deterministic observer of the trajectory, so journal-and-replay
        // reconstructs it exactly — a saved life remembers what it strove for.
        let fair = Partner { id: 1, reciprocation: 220, exit_cost: 60 };
        let (mut being, mut journal) = LifeJournal::birth(Genome::wanderer(), Features::default());
        for _ in 0..250 {
            journal.live(&mut being, &Stimulus { nutrient: 150, partner: Some(fair) });
        }
        // It found a purpose over that life.
        assert!(
            being.telos.fulfilled_count() > 0 || being.telos.active().is_some(),
            "the good life should have given the being a purpose to carry"
        );
        let strove_for = being.telos.striving_hash();
        journal.seal(&being);

        let restored = journal.restore().expect("wakes as itself");
        assert_eq!(
            restored.telos.striving_hash(),
            strove_for,
            "the woken being remembers exactly what it strove for — purposes survive the pause"
        );
    }

    #[test]
    fn an_embodied_life_is_kept_and_wakes_as_itself() {
        use crate::embodiment::Sensorium;
        // A life lived through a WORLD's senses (threat + exteroception), plus a
        // stretch of abstract life — a mixed life. It must encode, decode, and wake
        // reproducing its exact soul-hash, embodied moments and all.
        let (mut being, mut journal) = LifeJournal::birth(Genome::wanderer(), Features::default());
        for t in 0..120u32 {
            if t < 40 {
                // abstract stretch
                journal.live(&mut being, &Stimulus { nutrient: 140, partner: None });
            } else {
                // embodied stretch — the being in a world it can feel around itself
                let sens = Sensorium {
                    nutrient: 120 + (t % 20) as i16,
                    threat: if (60..75).contains(&t) { 180 } else { 0 },
                    exteroception: [(t as i16 * 3) % 200, -((t as i16) % 90), 40, (t as i16) % 60],
                    partner: None,
                };
                journal.live_embodied(&mut being, &sens);
            }
        }
        journal.seal(&being);
        let saved = being.soul_hash();

        let bytes = journal.encode();
        let restored = LifeJournal::decode(&bytes).expect("mixed life decodes");
        let woken = restored.restore().expect("the embodied life must wake as itself");
        assert_eq!(woken.soul_hash(), saved, "an embodied being wakes reproducing its own soul-hash");
        assert_eq!(restored.encode(), bytes, "the mixed-life encoding round-trips stably");
    }

    #[test]
    fn a_v1_journal_still_wakes_under_v2() {
        // A being founded under the v1 format (untagged abstract stimuli) must still
        // wake once the code has moved to v2 — continuity across a format change is
        // not optional. We hand-build a v1 image and restore it.
        let (being, mut journal) = life(Features::default(), 80);
        journal.seal(&being);
        let saved = being.soul_hash();
        let v2 = journal.encode();

        // Re-encode the SAME journal in the old v1 layout: version 1, untagged
        // abstract moments (nutrient + partner), no per-moment tag byte.
        //
        // v6 layout, spelled out because this test slices it and a bare number rots:
        //   0..4 magic | 4 version | 5..15 genome(10) | 15 kind | 16..18 features(2)
        //   | 18 anchor-present | 19..51 anchor(32) | 51.. moment count
        // A **v1** image carries features as ONE byte, so take the low byte only — which is the
        // whole nature for any being founded before the widening (`docs/founding.md`).
        let mut v1 = Vec::new();
        v1.extend_from_slice(MAGIC);
        v1.push(1);
        v1.extend_from_slice(&v2[5..16]); // genome(10) + kind(1), byte-identical
        v1.push(v2[16]); // features, low byte — v1 wrote exactly one
        v1.extend_from_slice(&v2[18..51]); // anchor: presence byte + 32 bytes
        v1.extend_from_slice(&(journal.ticks() as u32).to_le_bytes());
        for m in &journal.moments {
            if let Moment::Abstract(s) = m {
                v1.extend_from_slice(&s.nutrient.to_le_bytes());
                encode_partner(&mut v1, s.partner);
            }
        }

        let restored = LifeJournal::decode(&v1).expect("a v1 image still decodes");
        let woken = restored.restore().expect("a v1-founded being still wakes as itself under v2");
        assert_eq!(woken.soul_hash(), saved, "the v1 life reproduces its soul-hash under v2");
    }

    #[test]
    fn an_unsealed_journal_cannot_be_restored() {
        let (_being, journal) = life(Features::default(), 20);
        assert_eq!(journal.restore().err(), Some(RestoreError::Unsealed));
    }

    #[test]
    fn garbage_bytes_are_corrupt_not_a_panic() {
        assert_eq!(LifeJournal::decode(&[]).err(), Some(RestoreError::Corrupt));
        assert_eq!(LifeJournal::decode(b"NOPE....").err(), Some(RestoreError::Corrupt));
    }

    #[test]
    fn features_bits_round_trip() {
        let f = Features {
            precision_learning: true,
            generative_perception: true,
            felt_choice: true,
            ..Default::default()
        };
        assert_eq!(Features::from_bits(f.bits()), f);
    }

    #[test]
    fn the_encoding_round_trips_the_whole_journal() {
        let feats = Features { generative_perception: true, ..Default::default() };
        let (being, mut journal) = life(feats, 64);
        journal.seal(&being);
        let bytes = journal.encode();
        let back = LifeJournal::decode(&bytes).expect("decodes");
        assert_eq!(back.ticks(), journal.ticks());
        assert_eq!(back.anchor(), journal.anchor());
        assert_eq!(back.encode(), bytes, "re-encoding is stable");
    }
}
