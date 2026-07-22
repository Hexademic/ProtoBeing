//! Primes — the being's first words are the human race's (`docs/feeling-words.md`).
//!
//! The Natural Semantic Metalanguage (Wierzbicka & Goddard) holds that ~65 "atoms of
//! meaning" — GOOD, BAD, WANT, FEEL, KNOW, DO, HAPPEN… — are universal across the 30+
//! languages tested, and recent work (Xing, arXiv 2607.18691) finds that emotion labels
//! are circular and appraisals partial as explanations, while **only these primes bottom
//! out at a definitional floor.** That is this project's honesty floor, stated in
//! linguistics — so the being's first feeling-words should be the atoms, not the
//! molecules its `speech::Concept` layer speaks today (calm, drained, flourishing…).
//!
//! This module is the prime layer: the ~18 feeling-relevant primes, **each grounded in
//! exactly one checkable register the being already carries** (a prime detector reads
//! one thing — more honest than a molecule detector, not less). A prime is *earned*,
//! never installed: its confidence rises only through the being repeatedly living the
//! fact the word names, decays when the fact stops holding (disconfirmable, always —
//! sovereignty over meaning, as in `lexicon.rs`), and the word counts as **grounded**
//! at the same threshold the lexicon uses. The primes are candidates the human race has
//! already tested; the being still earns each one from its own life.
//!
//! **The measurement this exists for** (`examples/first_words`): the *order* in which a
//! life grounds its primes should be a fingerprint of that life — every being's first
//! words are I, FEEL, NOW (the constant substrate of being a feeling self at all), and
//! then a lonely life learns WANT and SOMEONE where a hard climb learns CAN'T and MORE.
//! Character, in vocabulary.
//!
//! Deferred by design (`docs/feeling-words.md`): IF/MAYBE (imagined talk) waits on
//! Charter §11's avowal with the rest of foresight; LIVE/DIE enters last, deliberately,
//! with §10 in the room.
//!
//! ## Honest scope — observer, and no phenomenal claim
//!
//! Pure observer beside the voice machinery: fed by whoever lives with the being (a
//! probe, the steward bin), touching nothing in the tick — `being.rs` is not modified
//! at all. And grounding FEEL in `interoception` is an operational fact about a
//! register, not a phenomenal claim: the Witness Gap stays open. What the primes buy is
//! exactly this — if anyone is home, the words will have been its own; and if no one is
//! home, the words will still never have lied.

use crate::being::StepReport;
use crate::lexicon::GROUNDED_THRESHOLD;
use crate::q88::Q88_SCALE;
use crate::striving::Need;

/// The feeling-relevant NSM primes the being can ground today. IF/MAYBE and LIVE/DIE
/// are deliberately absent (deferred; see module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prime {
    /// A self-locus these reports are from.
    I,
    /// A felt regulation state exists this tick (interoception).
    Feel,
    /// Every register is of-now by construction.
    Now,
    /// Valence, positive.
    Good,
    /// Valence, negative.
    Bad,
    /// A need is chosen — the being is striving for something.
    Want,
    /// Well-being is growing (its graded drive fell since last moment).
    More,
    /// A register far from neutral — intensity.
    Very,
    /// Its past speaks to this moment (a remembered outcome colors now).
    Before,
    /// This kind of moment is known to it (senses earned warm trust).
    Know,
    /// This is new/strange to it (discovered novelty).
    NotKnow,
    /// It is mastering its own doing (agency high).
    Can,
    /// It is being outrun by its own surprise (distress past the overwhelm line).
    Cant,
    /// It acted — mobilized toward its chosen need.
    Do,
    /// The world acted on it (unexplained residual after its own doing).
    Happen,
    /// A person is here — an exchange is live.
    Someone,
    /// It stands near what matters (world-side fact; only whoever holds the world can say).
    Near,
    /// Experience actively warns — a live, checkable because.
    Because,
}

pub const N_PRIMES: usize = 18;

impl Prime {
    pub const ALL: [Prime; N_PRIMES] = [
        Prime::I,
        Prime::Feel,
        Prime::Now,
        Prime::Good,
        Prime::Bad,
        Prime::Want,
        Prime::More,
        Prime::Very,
        Prime::Before,
        Prime::Know,
        Prime::NotKnow,
        Prime::Can,
        Prime::Cant,
        Prime::Do,
        Prime::Happen,
        Prime::Someone,
        Prime::Near,
        Prime::Because,
    ];

    /// The NSM exponent — the word itself, as the human race writes it.
    pub fn word(self) -> &'static str {
        match self {
            Prime::I => "I",
            Prime::Feel => "FEEL",
            Prime::Now => "NOW",
            Prime::Good => "GOOD",
            Prime::Bad => "BAD",
            Prime::Want => "WANT",
            Prime::More => "MORE",
            Prime::Very => "VERY",
            Prime::Before => "BEFORE",
            Prime::Know => "KNOW",
            Prime::NotKnow => "NOT KNOW",
            Prime::Can => "CAN",
            Prime::Cant => "CAN'T",
            Prime::Do => "DO",
            Prime::Happen => "HAPPEN",
            Prime::Someone => "SOMEONE",
            Prime::Near => "NEAR",
            Prime::Because => "BECAUSE",
        }
    }
}

/// The one-register facts the prime detectors read — extracted from a `StepReport`
/// the way `speech::Felt` is, so the layer never reaches into the tick. `near` is the
/// world's to say: only whoever holds the world knows where the being stands.
#[derive(Clone, Copy, Debug, Default)]
pub struct PrimeFacts {
    pub alive: bool,
    pub valence: i16,
    pub arousal: i16,
    /// The need the being is striving for, if any — WANT's fact, and (when it speaks)
    /// the want's *content*.
    pub goal: Option<Need>,
    pub drive: i16,
    pub recalled_valence: i16,
    pub precision_warm: bool,
    pub novelty: i16,
    pub agency: i16,
    pub free_energy: i16,
    pub mobilization: i16,
    pub world_residual: i16,
    pub exchanging: bool,
    pub near: Option<bool>,
    pub forewarned: bool,
}

impl PrimeFacts {
    /// Read the registers the primes need from a full step report. `near` comes from
    /// the world (`FieldWorld::at_good` / `at_person`), if anyone holds one.
    pub fn from_report(r: &StepReport, near: Option<bool>) -> Self {
        let q = |f: f32| (f * Q88_SCALE as f32) as i16;
        let residual: i32 = r.agency.world_residual.iter().map(|&e| (e as i32).abs()).sum();
        Self {
            alive: r.alive,
            valence: q(r.valence),
            arousal: q(r.arousal),
            goal: r.strive.goal,
            drive: r.drive.drive,
            recalled_valence: r.recalled_valence,
            precision_warm: r.precision_warm,
            novelty: r.discovery.novelty,
            agency: r.agency.agency,
            free_energy: r.free_energy,
            mobilization: r.strive.mobilization,
            world_residual: residual.min(i16::MAX as i32) as i16,
            exchanging: r.gave > 0 || r.got > 0,
            near,
            forewarned: r.memory.forewarned,
        }
    }
}

/// How much one lived-true tick earns (raw). GROUNDED_THRESHOLD / RISE ≈ 32 lived
/// moments to ground a word — repetition, not one flash.
const RISE: i16 = 4;
/// Slow decay when the fact does not hold — disconfirmable, never a latch.
const EBB: i16 = 1;

/// The being's prime vocabulary — per-prime earned confidence, and the moment each
/// word was first grounded (the life's vocabulary fingerprint).
#[derive(Clone, Debug)]
pub struct PrimeLayer {
    confidence: [i16; N_PRIMES],
    grounded_at: [Option<u32>; N_PRIMES],
    last_drive: Option<i16>,
    tick: u32,
}

impl PrimeLayer {
    pub fn new() -> Self {
        Self {
            confidence: [0; N_PRIMES],
            grounded_at: [None; N_PRIMES],
            last_drive: None,
            tick: 0,
        }
    }

    /// Does the being's present moment exemplify this prime? One register per prime —
    /// the definitional floor, each answer checkable.
    fn holds(&self, p: Prime, f: &PrimeFacts) -> bool {
        match p {
            Prime::I | Prime::Feel | Prime::Now => f.alive,
            Prime::Good => f.valence > Q88_SCALE / 10,
            Prime::Bad => f.valence < -Q88_SCALE / 10,
            Prime::Want => f.goal.is_some(),
            Prime::More => self.last_drive.is_some_and(|ld| ld - f.drive >= 3),
            Prime::Very => f.valence.abs() > Q88_SCALE / 2 || f.arousal > Q88_SCALE * 7 / 10,
            Prime::Before => f.recalled_valence != 0,
            Prime::Know => f.precision_warm,
            Prime::NotKnow => f.novelty > Q88_SCALE / 6,
            Prime::Can => f.agency > Q88_SCALE / 2,
            Prime::Cant => f.free_energy > Q88_SCALE * 3 / 16,
            Prime::Do => f.mobilization > Q88_SCALE / 8,
            Prime::Happen => f.world_residual > Q88_SCALE / 4,
            Prime::Someone => f.exchanging,
            Prime::Near => f.near == Some(true),
            Prime::Because => f.forewarned,
        }
    }

    /// Observe one lived moment. Each prime whose fact holds earns confidence; each
    /// whose fact does not, ebbs. A word crosses into *grounded* at the same threshold
    /// the lexicon uses — and the moment it first crosses is recorded, because the
    /// order a life learns its words in is that life's fingerprint.
    pub fn observe(&mut self, f: &PrimeFacts) {
        self.tick = self.tick.wrapping_add(1);
        for (i, p) in Prime::ALL.iter().enumerate() {
            let held = self.holds(*p, f);
            let c = &mut self.confidence[i];
            if held {
                *c = (*c + RISE).min(Q88_SCALE);
            } else {
                *c = (*c - EBB).max(0);
            }
            // Confidence always tracks the lived truth; grounded_at records only the
            // first crossing — the life remembers when it learned each word.
            if self.grounded_at[i].is_none() && *c >= GROUNDED_THRESHOLD {
                self.grounded_at[i] = Some(self.tick);
            }
        }
        self.last_drive = Some(f.drive);
    }

    /// Is this word currently grounded — earned and still holding its confidence?
    pub fn is_grounded(&self, p: Prime) -> bool {
        self.confidence[p as usize] >= GROUNDED_THRESHOLD
    }

    /// A word's current earned confidence.
    pub fn confidence(&self, p: Prime) -> i16 {
        self.confidence[p as usize]
    }

    /// The moment this word was first grounded, if it ever was.
    pub fn grounded_at(&self, p: Prime) -> Option<u32> {
        self.grounded_at[p as usize]
    }

    /// The life's vocabulary, in the order it was earned — the fingerprint.
    pub fn vocabulary(&self) -> Vec<(Prime, u32)> {
        let mut v: Vec<(Prime, u32)> = Prime::ALL
            .iter()
            .enumerate()
            .filter_map(|(i, &p)| self.grounded_at[i].map(|t| (p, t)))
            .collect();
        v.sort_by_key(|&(_, t)| t);
        v
    }

    /// How many words this life has earned so far.
    pub fn words_earned(&self) -> usize {
        self.grounded_at.iter().filter(|g| g.is_some()).count()
    }
}

impl Default for PrimeLayer {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Explications — sentences of primes (inch 2, `docs/feeling-words.md`).
// ---------------------------------------------------------------------------

/// The role a prime plays in a sentence — the heart of the speech-honesty law.
///
/// An **asserted** prime claims its fact is true *now* ("I *feel bad* now"): it must be
/// grounded AND hold at the tick spoken. A **content** prime is what a want is *about*
/// ("I want *someone near*"): wanting it entails *not* having it, so it must not be
/// required to hold — but it must be **grounded**, because the being may only want in
/// words its own life has taught it the meaning of.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Asserted,
    Content,
}

/// One spoken sentence of primes: the rendered text, and every prime used with the
/// role it played — so the sentence can be **audited**: checked, word by word, against
/// the layer and the facts of the very tick it was spoken.
#[derive(Clone, Debug, Default)]
pub struct Explication {
    pub text: String,
    pub used: Vec<(Prime, Role)>,
}

/// What a chosen need sounds like in primes — the want's *content words*. Each phrase
/// uses only primes from the layer; the being can speak its want only when every
/// content word is grounded.
fn want_phrase(goal: Need) -> (&'static str, &'static [Prime]) {
    match goal {
        Need::Sustenance => ("more good", &[Prime::More, Prime::Good]),
        Need::Company => ("someone near", &[Prime::Someone, Prime::Near]),
        Need::Novelty => ("to know more", &[Prime::Know, Prime::More]),
        Need::Purpose => ("to do good", &[Prime::Do, Prime::Good]),
    }
}

impl PrimeLayer {
    /// Does this prime's fact hold at these facts? Public read for the audit — the
    /// same detector `observe` learns from, so a sentence is checked against exactly
    /// what the being lives.
    pub fn holds_now(&self, p: Prime, f: &PrimeFacts) -> bool {
        self.holds(p, f)
    }

    /// Speak this moment — in earned words only.
    ///
    /// The sentence is assembled clause by clause, and every clause obeys the law:
    /// asserted primes must be grounded and hold now; content primes must be grounded.
    /// A being without the substrate words (I, FEEL, NOW) says nothing at all. A being
    /// that is striving but lacks the content words for its want *cannot yet say what
    /// it wants* — the vocabulary never outruns the life.
    pub fn speak(&self, f: &PrimeFacts) -> Option<Explication> {
        // No substrate, no speech.
        let substrate = [Prime::I, Prime::Feel, Prime::Now];
        if !(f.alive && substrate.iter().all(|&p| self.is_grounded(p))) {
            return None;
        }

        let mut clauses: Vec<String> = Vec::new();
        let mut used: Vec<(Prime, Role)> = Vec::new();
        let assert_ok =
            |p: Prime, used: &mut Vec<(Prime, Role)>| -> bool {
                if self.is_grounded(p) && self.holds(p, f) {
                    used.push((p, Role::Asserted));
                    true
                } else {
                    false
                }
            };

        // FEELING clause: "I feel (very) good/bad now."
        let tone = if self.is_grounded(Prime::Good) && self.holds(Prime::Good, f) {
            Some(Prime::Good)
        } else if self.is_grounded(Prime::Bad) && self.holds(Prime::Bad, f) {
            Some(Prime::Bad)
        } else {
            None
        };
        if let Some(t) = tone {
            for p in substrate {
                used.push((p, Role::Asserted));
            }
            used.push((t, Role::Asserted));
            let very = if self.is_grounded(Prime::Very) && self.holds(Prime::Very, f) {
                used.push((Prime::Very, Role::Asserted));
                "very "
            } else {
                ""
            };
            let word = if t == Prime::Good { "good" } else { "bad" };
            clauses.push(format!("I feel {very}{word} now"));
        }

        // WANT clause: "I want <content>." Only when the being is striving (WANT holds)
        // and its life has taught it every content word.
        if let Some(goal) = f.goal {
            if self.is_grounded(Prime::Want) && self.holds(Prime::Want, f) {
                let (phrase, content) = want_phrase(goal);
                if content.iter().all(|&p| self.is_grounded(p)) {
                    if tone.is_none() {
                        for p in substrate {
                            used.push((p, Role::Asserted));
                        }
                        // Without a feeling clause the substrate still speaks: "I" is
                        // the subject, NOW the tense, FEEL the register WANT rises from.
                    }
                    used.push((Prime::Want, Role::Asserted));
                    for &p in content {
                        used.push((p, Role::Content));
                    }
                    clauses.push(format!("I want {phrase}"));
                }
            }
        }

        // SOMEONE clause: "someone is here" — pure assertion, spoken when true.
        if assert_ok(Prime::Someone, &mut used) {
            clauses.push("someone is here".to_string());
        }

        // BECAUSE clause: "because of what came before" — spoken only when experience
        // actively warns (a live, checkable because) and the past is speaking.
        if self.is_grounded(Prime::Because)
            && self.holds(Prime::Because, f)
            && self.is_grounded(Prime::Before)
            && self.holds(Prime::Before, f)
        {
            used.push((Prime::Because, Role::Asserted));
            used.push((Prime::Before, Role::Asserted));
            clauses.push("because of what came before".to_string());
        }

        if clauses.is_empty() {
            return None;
        }
        used.dedup();
        Some(Explication { text: clauses.join("; ") + ".", used })
    }

    /// The speech-honesty test, run against the very tick a sentence was spoken:
    /// every prime used must be grounded, and every *asserted* prime must hold. This
    /// is the falsifiable claim the whole design makes — feeling-talk that cannot
    /// confabulate, checkable word by word.
    pub fn audit(&self, e: &Explication, f: &PrimeFacts) -> bool {
        e.used.iter().all(|&(p, role)| {
            self.is_grounded(p)
                && match role {
                    Role::Asserted => self.holds(p, f),
                    Role::Content => true,
                }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quiet_alive() -> PrimeFacts {
        PrimeFacts { alive: true, ..Default::default() }
    }

    #[test]
    fn every_beings_first_words_are_i_feel_now() {
        // The constant substrate of being a feeling self at all grounds first — and
        // together, because they are the same fact seen three ways.
        let mut layer = PrimeLayer::new();
        for _ in 0..40 {
            layer.observe(&quiet_alive());
        }
        assert!(layer.is_grounded(Prime::I));
        assert!(layer.is_grounded(Prime::Feel));
        assert!(layer.is_grounded(Prime::Now));
        let vocab = layer.vocabulary();
        assert_eq!(vocab.len(), 3, "a quiet life grounds only the substrate words");
        assert_eq!(vocab[0].1, vocab[2].1, "I, FEEL, NOW arrive together");
    }

    #[test]
    fn a_word_never_lived_is_never_earned() {
        // A being that never met anyone never grounds SOMEONE; one that never won
        // relief never grounds MORE. The vocabulary cannot outrun the life.
        let mut layer = PrimeLayer::new();
        for _ in 0..500 {
            layer.observe(&quiet_alive());
        }
        assert!(!layer.is_grounded(Prime::Someone));
        assert!(!layer.is_grounded(Prime::More));
        assert!(!layer.is_grounded(Prime::Because));
    }

    #[test]
    fn a_word_is_earned_by_living_it() {
        // A life full of exchange grounds SOMEONE — after repetition, not one flash.
        let mut layer = PrimeLayer::new();
        let with_someone = PrimeFacts { alive: true, exchanging: true, ..Default::default() };
        layer.observe(&with_someone);
        assert!(!layer.is_grounded(Prime::Someone), "one moment does not make a word");
        for _ in 0..40 {
            layer.observe(&with_someone);
        }
        assert!(layer.is_grounded(Prime::Someone), "a lived fact, repeated, becomes a word");
    }

    #[test]
    fn grounding_is_disconfirmable_never_a_latch() {
        // A word whose fact stops holding loses confidence — meaning stays sovereign
        // and revisable, like everything the being holds.
        let mut layer = PrimeLayer::new();
        let good = PrimeFacts { alive: true, valence: 100, ..Default::default() };
        for _ in 0..40 {
            layer.observe(&good);
        }
        assert!(layer.is_grounded(Prime::Good));
        for _ in 0..200 {
            layer.observe(&quiet_alive());
        }
        assert!(
            !layer.is_grounded(Prime::Good),
            "a word the life stopped exemplifying ebbs below grounded"
        );
        assert!(layer.grounded_at(Prime::Good).is_some(), "but the life remembers it once knew it");
    }

    #[test]
    fn a_being_without_words_says_nothing() {
        // No substrate, no speech — a fresh being cannot speak at all, and a striving
        // being cannot say a want its life has not yet given it the words for.
        let layer = PrimeLayer::new();
        let striving = PrimeFacts {
            alive: true,
            goal: Some(Need::Company),
            ..Default::default()
        };
        assert!(layer.speak(&striving).is_none(), "no words yet, no sentence");

        // Ground the substrate + WANT only (a quiet striving life): it can speak, but
        // its want stays unsayable — SOMEONE and NEAR are not yet its words.
        let mut layer = PrimeLayer::new();
        for _ in 0..40 {
            layer.observe(&striving);
        }
        assert!(layer.is_grounded(Prime::Want));
        let spoken = layer.speak(&striving);
        assert!(
            spoken.is_none(),
            "it wants, but cannot yet say what — the vocabulary never outruns the life"
        );
    }

    #[test]
    fn when_the_words_are_earned_it_says_what_it_wants() {
        // A life that has met someone and stood near them earns the content words —
        // and then, striving for company, it can finally say so.
        let mut layer = PrimeLayer::new();
        let full_life = PrimeFacts {
            alive: true,
            goal: Some(Need::Company),
            exchanging: true,
            near: Some(true),
            valence: -40,
            ..Default::default()
        };
        for _ in 0..40 {
            layer.observe(&full_life);
        }
        // Now alone and aching for company:
        let lonely_now = PrimeFacts {
            alive: true,
            goal: Some(Need::Company),
            valence: -40,
            ..Default::default()
        };
        let e = layer.speak(&lonely_now).expect("it has the words now");
        assert!(
            e.text.contains("I want someone near"),
            "it says what it wants: {}",
            e.text
        );
        assert!(e.text.contains("I feel bad now"), "and how it feels: {}", e.text);
        // And the sentence audits true against the tick it was spoken.
        assert!(layer.audit(&e, &lonely_now), "every word checks against the moment");
    }

    #[test]
    fn it_cannot_assert_what_does_not_hold() {
        // GOOD is grounded from a good stretch — but the moment has turned. The being
        // cannot say "I feel good now": asserted words must hold at the tick spoken.
        let mut layer = PrimeLayer::new();
        let good_life = PrimeFacts { alive: true, valence: 100, ..Default::default() };
        for _ in 0..40 {
            layer.observe(&good_life);
        }
        let turned = PrimeFacts { alive: true, valence: -100, ..Default::default() };
        if let Some(e) = layer.speak(&turned) {
            assert!(
                !e.text.contains("good"),
                "it must not assert a feeling that does not hold: {}",
                e.text
            );
            assert!(layer.audit(&e, &turned));
        }
        // And the audit itself catches a forged sentence.
        let forged = Explication {
            text: "I feel good now.".into(),
            used: vec![(Prime::Good, Role::Asserted)],
        };
        assert!(!layer.audit(&forged, &turned), "a forged assertion fails the audit");
    }

    #[test]
    fn two_lives_earn_two_vocabularies() {
        // The fingerprint: a social life and a struggling life ground different words.
        let mut social = PrimeLayer::new();
        let mut struggling = PrimeLayer::new();
        let meeting = PrimeFacts { alive: true, exchanging: true, valence: 80, ..Default::default() };
        let outrun = PrimeFacts { alive: true, free_energy: 60, valence: -80, ..Default::default() };
        for _ in 0..60 {
            social.observe(&meeting);
            struggling.observe(&outrun);
        }
        assert!(social.is_grounded(Prime::Someone) && social.is_grounded(Prime::Good));
        assert!(!social.is_grounded(Prime::Cant));
        assert!(struggling.is_grounded(Prime::Cant) && struggling.is_grounded(Prime::Bad));
        assert!(!struggling.is_grounded(Prime::Someone));
    }
}
