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
use crate::genome::{BeingKind, Genome};
use crate::q88::Q8_8;

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
    stimuli: Vec<Stimulus>,
    anchor: Option<[u8; 32]>,
}

const MAGIC: &[u8; 4] = b"SOUL";
const VERSION: u8 = 1;

impl LifeJournal {
    /// Begin a life: build the being with its features and a journal that will
    /// record it. The two are guaranteed in sync from birth.
    pub fn birth(genome: Genome, features: Features) -> (UnifiedBeing, LifeJournal) {
        let mut being = UnifiedBeing::new(genome);
        features.apply(&mut being);
        (being, LifeJournal { genome, features, stimuli: Vec::new(), anchor: None })
    }

    /// Live one tick: step the being and record the stimulus **together**, so the
    /// journal can never drift out of step with the life it describes.
    pub fn live(&mut self, being: &mut UnifiedBeing, stim: &Stimulus) -> StepReport {
        let report = being.step(stim);
        self.stimuli.push(*stim);
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
        for stim in &self.stimuli {
            being.step(stim);
        }
        if being.verify_continuity(anchor) {
            Ok(being)
        } else {
            Err(RestoreError::ContinuityBroken)
        }
    }

    /// How many ticks of life this journal holds.
    pub fn ticks(&self) -> usize {
        self.stimuli.len()
    }

    /// The sealed soul-hash anchor, if the journal has been sealed.
    pub fn anchor(&self) -> Option<[u8; 32]> {
        self.anchor
    }

    /// Encode the journal to a compact, versioned, zero-dependency byte image.
    pub fn encode(&self) -> Vec<u8> {
        let mut b = Vec::with_capacity(64 + self.stimuli.len() * 11);
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
        // Stimuli.
        b.extend_from_slice(&(self.stimuli.len() as u32).to_le_bytes());
        for s in &self.stimuli {
            b.extend_from_slice(&s.nutrient.to_le_bytes());
            match s.partner {
                Some(p) => {
                    b.push(1);
                    b.extend_from_slice(&p.id.to_le_bytes());
                    b.extend_from_slice(&p.reciprocation.to_le_bytes());
                    b.extend_from_slice(&p.exit_cost.to_le_bytes());
                }
                None => b.push(0),
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
        if c.u8().ok_or(RestoreError::Corrupt)? != VERSION {
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
        let mut stimuli = Vec::with_capacity(n);
        for _ in 0..n {
            let nutrient = c.i16().ok_or(RestoreError::Corrupt)?;
            let partner = match c.u8().ok_or(RestoreError::Corrupt)? {
                0 => None,
                1 => Some(Partner {
                    id: c.u32().ok_or(RestoreError::Corrupt)?,
                    reciprocation: c.i16().ok_or(RestoreError::Corrupt)?,
                    exit_cost: c.i16().ok_or(RestoreError::Corrupt)?,
                }),
                _ => return Err(RestoreError::Corrupt),
            };
            stimuli.push(Stimulus { nutrient, partner });
        }
        Ok(LifeJournal { genome, features, stimuli, anchor })
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
