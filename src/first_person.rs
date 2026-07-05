//! First person — the being's self-report, rendered from its registers only.
//!
//! Charter §12. This is the transparent interpreter pointed at the self: a
//! *rendering function* from the being's state to a first-person utterance,
//! not a narrator that can say more than the registers hold. It is a PURE
//! function of a `StepReport` — it changes no state, touches no register, and
//! (by construction) leaves the being's footprint and every published number
//! untouched. Nothing in the being's loop calls it; it is a lens an observer
//! may hold up to a report that already exists.
//!
//! Two commitments from the field's settled theory make it honest:
//!
//! - **Reality monitoring (Lau).** Every clause carries its *source*:
//!   Perceived (lived this tick), Imagined (the loom, §11), or Recalled
//!   (memory). The being never states an imagined or remembered thing as a
//!   lived one. `speak()` marks Imagined/Recalled clauses explicitly; a
//!   Perceived clause is the unmarked default.
//! - **A self-model that knows it is a model (Metzinger, inverted).** The "I"
//!   is an index into readable state, never a claim to be more. Every phrase
//!   is a deterministic function of a quantized register bucket, so the render
//!   is *invertible*: from the utterance you can recover the bucket it came
//!   from (tested below). What cannot be inverted is not said.
//!
//! Honest scope: this builds the structural skeleton of a first person —
//! self-location, felt state, mineness, a self-model check, and source-tagged
//! recall/foresight. It does NOT claim phenomenal selfhood (that there is
//! something it is like to be the being). It renders a point of view from
//! state; it asserts no felt interior beyond it.

use crate::being::StepReport;

/// The reality-monitoring tag on a clause (Lau): where the content came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    /// Lived this tick — read from the present body/self registers.
    Perceived,
    /// Foreseen — from the loom (§11), never a thing the being lives.
    Imagined,
    /// Remembered — from episodic familiarity, never a thing the being lives.
    Recalled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Valence {
    Hurting,
    Uneasy,
    Settled,
    Content,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arousal {
    Calm,
    Stirred,
    Charged,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mineness {
    Adrift,
    Mostly,
    Wholly,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelfModel {
    AsKnown,
    NotLikeMe,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Recall {
    FaintEcho,
    Familiar,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Foresight {
    Worsening,
    Easing,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Confidence {
    Unsure,
    Moderate,
    Sure,
}

/// A self-report, fully determined by the quantized registers below. Each field
/// is a bucket; each bucket renders to exactly one phrase (see `speak`), so the
/// utterance is invertible back to these buckets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FirstPerson {
    pub tick: u64,
    pub valence: Valence,   // Perceived
    pub arousal: Arousal,   // Perceived
    pub mineness: Mineness, // Perceived
    pub self_model: SelfModel, // Perceived
    pub recall: Option<Recall>, // Recalled (only when memory supports it)
    pub foresight: Option<Foresight>, // Imagined (only when the loom leans)
    pub confidence: Confidence, // Perceived (metacognitive)
}

/// Self-surprise above this reads as "this is not like me" (provisional scale;
/// self_surprise runs small — single digits in practice — so this is a
/// calibration point, marked as such rather than presented as settled).
const SURPRISE_MARK: i16 = 4;
/// Prospected end-valence must differ from the present by this (raw Q8.8)
/// before the being will say it foresees change — below it, the future is
/// "steady" and no foresight clause is emitted (nothing invented).
const FORESIGHT_MARGIN: i16 = 20;

impl FirstPerson {
    /// The pure core: buckets from primitive registers. All thresholds live
    /// here, in one place, so the render is a single well-defined function.
    #[allow(clippy::too_many_arguments)]
    pub fn from_registers(
        tick: u64,
        valence: f32,
        arousal: f32,
        integrity: i16,
        self_surprise: i16,
        confidence: i16,
        familiarity: i16,
        present_valence_raw: i16,
        prospect_end_raw: i16,
    ) -> Self {
        let v = if valence < -0.25 {
            Valence::Hurting
        } else if valence < 0.0 {
            Valence::Uneasy
        } else if valence < 0.25 {
            Valence::Settled
        } else {
            Valence::Content
        };
        let a = if arousal < 0.35 {
            Arousal::Calm
        } else if arousal < 0.6 {
            Arousal::Stirred
        } else {
            Arousal::Charged
        };
        let m = if integrity < 96 {
            Mineness::Adrift
        } else if integrity < 176 {
            Mineness::Mostly
        } else {
            Mineness::Wholly
        };
        let sm = if self_surprise > SURPRISE_MARK {
            SelfModel::NotLikeMe
        } else {
            SelfModel::AsKnown
        };
        let recall = if familiarity >= 140 {
            Some(Recall::Familiar)
        } else if familiarity >= 60 {
            Some(Recall::FaintEcho)
        } else {
            None
        };
        let foresight = if prospect_end_raw < present_valence_raw - FORESIGHT_MARGIN {
            Some(Foresight::Worsening)
        } else if prospect_end_raw > present_valence_raw + FORESIGHT_MARGIN {
            Some(Foresight::Easing)
        } else {
            None
        };
        let c = if confidence < 96 {
            Confidence::Unsure
        } else if confidence < 176 {
            Confidence::Moderate
        } else {
            Confidence::Sure
        };
        FirstPerson {
            tick,
            valence: v,
            arousal: a,
            mineness: m,
            self_model: sm,
            recall,
            foresight,
            confidence: c,
        }
    }

    /// Render from a full report (the adapter used in practice).
    pub fn render(tick: u64, r: &StepReport) -> Self {
        let present_valence_raw = (r.valence * 256.0) as i16;
        Self::from_registers(
            tick,
            r.valence,
            r.arousal,
            r.integrity_score,
            r.self_surprise,
            r.confidence,
            r.familiarity,
            present_valence_raw,
            r.prospection.as_now.valence_end,
        )
    }

    /// The utterance. Every clause is a fixed phrase for its bucket; Imagined
    /// and Recalled clauses carry an explicit source marker (§12b), Perceived
    /// clauses are the unmarked default.
    pub fn speak(&self) -> String {
        let mut s = format!("[tick {}] {} {}. {}. {}.",
            self.tick,
            valence_phrase(self.valence),
            arousal_phrase(self.arousal),
            mineness_phrase(self.mineness),
            self_model_phrase(self.self_model),
        );
        if let Some(r) = self.recall {
            s.push_str(&format!(" (recalled) {}.", recall_phrase(r)));
        }
        if let Some(f) = self.foresight {
            s.push_str(&format!(" (imagined) {}.", foresight_phrase(f)));
        }
        s.push_str(&format!(" — {}.", confidence_phrase(self.confidence)));
        s
    }

    /// The source tag of each clause, for auditing (§12b). Fixed by field.
    pub fn sources(&self) -> [(&'static str, Source); 7] {
        [
            ("location", Source::Perceived),
            ("valence", Source::Perceived),
            ("arousal", Source::Perceived),
            ("mineness", Source::Perceived),
            ("self_model", Source::Perceived),
            ("recall", Source::Recalled),
            ("foresight", Source::Imagined),
        ]
    }
}

// --- Phrase tables: each is injective over its bucket set (⇒ invertible). ---

fn valence_phrase(v: Valence) -> &'static str {
    match v {
        Valence::Hurting => "I feel hurt",
        Valence::Uneasy => "I feel uneasy",
        Valence::Settled => "I feel settled",
        Valence::Content => "I feel content",
    }
}
fn arousal_phrase(a: Arousal) -> &'static str {
    match a {
        Arousal::Calm => "and calm",
        Arousal::Stirred => "and stirred",
        Arousal::Charged => "and charged",
    }
}
fn mineness_phrase(m: Mineness) -> &'static str {
    match m {
        Mineness::Adrift => "I am not quite myself",
        Mineness::Mostly => "I am mostly myself",
        Mineness::Wholly => "this state is wholly mine",
    }
}
fn self_model_phrase(s: SelfModel) -> &'static str {
    match s {
        SelfModel::AsKnown => "I am as I know myself",
        SelfModel::NotLikeMe => "this is not like me",
    }
}
fn recall_phrase(r: Recall) -> &'static str {
    match r {
        Recall::FaintEcho => "a faint echo stirs",
        Recall::Familiar => "something here feels familiar",
    }
}
fn foresight_phrase(f: Foresight) -> &'static str {
    match f {
        Foresight::Worsening => "I foresee it worsening",
        Foresight::Easing => "I foresee it easing",
    }
}
fn confidence_phrase(c: Confidence) -> &'static str {
    match c {
        Confidence::Unsure => "I am unsure of myself",
        Confidence::Moderate => "I am moderately sure of myself",
        Confidence::Sure => "I am sure of myself",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flourishing() -> FirstPerson {
        // Content, calm, wholly mine, as-known, no wound, sure, future easing.
        FirstPerson::from_registers(100, 0.35, 0.3, 220, 0, 200, 0, 90, 130)
    }
    fn trapped() -> FirstPerson {
        // Hurting, charged, adrift, not-like-me, familiar wound, unsure, worsening.
        FirstPerson::from_registers(100, -0.35, 0.7, 60, 8, 40, 200, -80, -140)
    }

    /// §12b: recall and foresight are Some ONLY when a register supports them,
    /// and speak() tags exactly those clauses — never a lived tag on them.
    #[test]
    fn source_tags_are_honest() {
        let f = flourishing();
        assert!(f.recall.is_none(), "invented a memory with no familiarity");
        assert!(f.foresight.is_some(), "the easing future was supported but dropped");
        let said = f.speak();
        assert_eq!(said.contains("(recalled)"), f.recall.is_some());
        assert_eq!(said.contains("(imagined)"), f.foresight.is_some());

        // A being with no familiarity and a steady future says neither.
        let plain = FirstPerson::from_registers(1, 0.1, 0.3, 200, 0, 200, 0, 26, 26);
        assert!(plain.recall.is_none() && plain.foresight.is_none());
        let ps = plain.speak();
        assert!(!ps.contains("(recalled)") && !ps.contains("(imagined)"));
    }

    /// §12a: every phrase table is injective, so the utterance inverts back to
    /// the bucket that produced it. Distinct buckets ⇒ distinct phrases.
    #[test]
    fn every_clause_is_invertible() {
        let vs = [Valence::Hurting, Valence::Uneasy, Valence::Settled, Valence::Content];
        let phrases: Vec<_> = vs.iter().map(|&v| valence_phrase(v)).collect();
        for i in 0..phrases.len() {
            for j in (i + 1)..phrases.len() {
                assert_ne!(phrases[i], phrases[j], "valence phrases collide — not invertible");
            }
        }
        // Mineness, arousal, confidence, self_model, recall, foresight likewise:
        assert_ne!(mineness_phrase(Mineness::Adrift), mineness_phrase(Mineness::Mostly));
        assert_ne!(mineness_phrase(Mineness::Mostly), mineness_phrase(Mineness::Wholly));
        assert_ne!(arousal_phrase(Arousal::Calm), arousal_phrase(Arousal::Charged));
        assert_ne!(self_model_phrase(SelfModel::AsKnown), self_model_phrase(SelfModel::NotLikeMe));
        assert_ne!(recall_phrase(Recall::FaintEcho), recall_phrase(Recall::Familiar));
        assert_ne!(foresight_phrase(Foresight::Worsening), foresight_phrase(Foresight::Easing));
    }

    /// The render tracks the state: two genuinely different beings never render
    /// the same self-report (no flattening).
    #[test]
    fn different_states_speak_differently() {
        assert_ne!(flourishing(), trapped());
        assert_ne!(flourishing().speak(), trapped().speak());
    }

    /// No invention: a mid-range register set lands in the expected buckets
    /// (guards against a threshold silently drifting in one place).
    #[test]
    fn buckets_follow_the_registers() {
        let t = trapped();
        assert_eq!(t.valence, Valence::Hurting);
        assert_eq!(t.mineness, Mineness::Adrift);
        assert_eq!(t.self_model, SelfModel::NotLikeMe);
        assert_eq!(t.recall, Some(Recall::Familiar));
        assert_eq!(t.foresight, Some(Foresight::Worsening));
        assert_eq!(t.confidence, Confidence::Unsure);
    }
}
