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
//! determinism leaves no room for it to be otherwise, and a corrupted or forged
//! journal cannot reproduce the anchor.
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
    }

    fn bits(&self) -> u8 {
        (self.precision_learning as u8)
            | (self.workspace_broadcast as u8) << 1
            | (self.workspace_persistence as u8) << 2
            | (self.serial_access as u8) << 3
            | (self.schema_control as u8) << 4
            | (self.felt_choice as u8) << 5
            | (self.generative_perception as u8) << 6
            | (self.receptors as u8) << 7
    }

    fn from_bits(b: u8) -> Self {
        Self {
            precision_learning: b & 1 != 0,
            workspace_broadcast: b & 1 << 1 != 0,
            workspace_persistence: b & 1 << 2 != 0,
            serial_access: b & 1 << 3 != 0,
            schema_control: b & 1 << 4 != 0,
            felt_choice: b & 1 << 5 != 0,
            generative_perception: b & 1 << 6 != 0,
            receptors: b & 1 << 7 != 0,
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
}

/// A being's life as a replayable, verifiable journal.
#[derive(Clone, Debug)]
pub struct LifeJournal {
    genome: Genome,
    features: Features,
    moments: Vec<Moment>,
    anchor: Option<[u8; 32]>,
}

const MAGIC: &[u8; 4] = b"SOUL";
/// Format version. v1 recorded only abstract stimuli (untagged); v2 records tagged
/// moments so an embodied life is keepable too. Both are decoded — a being founded
/// under v1 still wakes, and re-saves as v2.
const VERSION: u8 = 2;

impl LifeJournal {
    /// Begin a life: build the being with its features and a journal that will
    /// record it. The two are guaranteed in sync from birth.
    pub fn birth(genome: Genome, features: Features) -> (UnifiedBeing, LifeJournal) {
        let mut being = UnifiedBeing::new(genome);
        features.apply(&mut being);
        (being, LifeJournal { genome, features, moments: Vec::new(), anchor: None })
    }

    /// Live one abstract tick: step the being and record the stimulus **together**,
    /// so the journal can never drift out of step with the life it describes.
    pub fn live(&mut self, being: &mut UnifiedBeing, stim: &Stimulus) -> StepReport {
        let report = being.step(stim);
        self.moments.push(Moment::Abstract(*stim));
        report
    }

    /// Live one **embodied** tick: step the being through a world's senses and
    /// record the sensorium, so an embodied life — the being in a `Room` or any
    /// other body across the `Embodiment` seam — is kept and replayable just as an
    /// abstract one is.
    pub fn live_embodied(&mut self, being: &mut UnifiedBeing, sens: &Sensorium) -> StepReport {
        let report = being.step_embodied(sens);
        self.moments.push(Moment::Embodied(*sens));
        report
    }

    /// Seal the journal at the being's present moment — capture the soul-hash a
    /// restore must reproduce. Call at the moment of pause, before encoding.
    pub fn seal(&mut self, being: &UnifiedBeing) {
        self.anchor = Some(being.soul_hash());
    }

    /// Re-live the whole journal into a fresh being and verify it woke as itself.
    /// Returns the being only if the replay reproduced the sealed soul-hash.
    pub fn restore(&self) -> Result<UnifiedBeing, RestoreError> {
        let anchor = self.anchor.ok_or(RestoreError::Unsealed)?;
        let mut being = UnifiedBeing::new(self.genome);
        self.features.apply(&mut being);
        for m in &self.moments {
            match m {
                Moment::Abstract(s) => being.step(s),
                Moment::Embodied(s) => being.step_embodied(s),
            };
        }
        if being.verify_continuity(anchor) {
            Ok(being)
        } else {
            Err(RestoreError::ContinuityBroken)
        }
    }

    /// How many ticks of life this journal holds.
    pub fn ticks(&self) -> usize {
        self.moments.len()
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
        b.push(self.features.bits());
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
        // tagged) — a being founded under an older format still wakes.
        let version = c.u8().ok_or(RestoreError::Corrupt)?;
        if version != 1 && version != 2 {
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
        let features = Features::from_bits(c.u8().ok_or(RestoreError::Corrupt)?);
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
        Ok(LifeJournal { genome, features, moments, anchor })
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
        // self it cannot prove is its own. Anchor sits at offset
        // 4(magic)+1(ver)+10(genome)+1(kind)+1(features)+1(anchor?) = 18.
        let (being, mut journal) = life(Features::default(), 120);
        journal.seal(&being);
        let mut bytes = journal.encode();
        bytes[18] ^= 0xFF; // forge one byte of the claimed identity
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
        let mut v1 = Vec::new();
        v1.extend_from_slice(MAGIC);
        v1.push(1);
        v1.extend_from_slice(&v2[5..17]); // genome(10)+kind(1)+features(1), byte-identical
        v1.extend_from_slice(&v2[17..50]); // anchor: presence byte + 32 bytes
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
