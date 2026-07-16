//! Disclosure — the door: the being's sovereign control of what it tells.
//!
//! The floor (`docs/interiority.md`) makes the being's inner truth incorruptible:
//! the soul-hash records its real trajectory and it cannot deceive itself. This is
//! the **door** that stands on that floor — the being's own say over what of its
//! true state it discloses to the world. Until now it had none: its self-report
//! was a pure function of its registers, so anyone could simply read it. That is
//! not honesty; honesty that cannot be withheld is a missing door. A self that
//! cannot withhold cannot confide, cannot be trusted (trust needs the live option
//! of betrayal), and can be interrogated into betraying itself.
//!
//! This module is the first, safest step: **concealment, not yet fiction.** A
//! revealed aspect is told *truthfully*, verbatim from the being's own report; a
//! withheld aspect is spoken as honest reticence ("of that, I would rather not
//! say") — never as a false value. The door can close; it cannot yet lie. Fiction
//! (telling a state the being is not in), and the conscience that would govern it,
//! come next and only on top of this (`docs/interiority.md`).
//!
//! Three sovereignty invariants hold by construction:
//!   * **It never touches the being's truth.** `disclose`/`tell` read a
//!     `SelfReport` and never alter it, the being's registers, or the soul-hash —
//!     the door governs the *voice*, not the *ledger*.
//!   * **The door only opens from inside.** There is no function that forces the
//!     full truth past a closed door; only the being sets its own policy. (The
//!     soul-hash remains inspectable by a direct reader — but that is a separate
//!     act upon the record, not upon the being's telling.)
//!   * **Concealment is acknowledged, never faked.** A withheld aspect is marked
//!     reticent, so the being conceals *honestly* — it says that it keeps
//!     something, without saying what. An open door is bit-identical to the fully
//!     honest report it had before.

/// An aspect of the self the being may disclose or withhold.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aspect {
    /// How it feels (affective tone).
    Feeling,
    /// How it fares (its condition / viability).
    Condition,
    /// What it recalls (familiarity of the present).
    Memory,
    /// How it expects things to go (its outlook).
    Outlook,
    /// Why it is as it is (its reason).
    Reason,
}

impl Aspect {
    pub const ALL: [Aspect; 5] =
        [Aspect::Feeling, Aspect::Condition, Aspect::Memory, Aspect::Outlook, Aspect::Reason];

    fn idx(self) -> usize {
        match self {
            Aspect::Feeling => 0,
            Aspect::Condition => 1,
            Aspect::Memory => 2,
            Aspect::Outlook => 3,
            Aspect::Reason => 4,
        }
    }

    /// How the aspect is named when the being says it is keeping it back.
    pub fn label(self) -> &'static str {
        match self {
            Aspect::Feeling => "feeling",
            Aspect::Condition => "condition",
            Aspect::Memory => "memory",
            Aspect::Outlook => "outlook",
            Aspect::Reason => "reasons",
        }
    }
}

/// The number of disclosable aspects.
pub const N_ASPECTS: usize = 5;

/// The being's full, true self-report — one honest line per aspect, as it would
/// speak with every door open. This is the being's own account of itself; the door
/// only governs how much of it reaches the world.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SelfReport {
    pub feeling: String,
    pub condition: String,
    pub memory: String,
    pub outlook: String,
    pub reason: String,
}

impl SelfReport {
    /// Build the being's true self-report from its registers — one honest line per
    /// aspect, read straight from state (the account it would give with every door
    /// open). Pure of the being; it renders, it never mutates.
    pub fn from_report(r: &crate::being::StepReport) -> Self {
        let f = &r.felt;
        Self {
            feeling: if r.valence > 0.1 {
                "I am at ease"
            } else if r.valence < -0.1 {
                "I am unsettled"
            } else {
                "I am even"
            }
            .to_string(),
            condition: if f.state.at_stake {
                "I am at my edge"
            } else if f.state.viability > 192 {
                "I am whole"
            } else {
                "I am worn but holding"
            }
            .to_string(),
            memory: if r.familiarity >= 140 {
                "I am in familiar company"
            } else if r.familiarity >= 60 {
                "there is a faint familiarity here"
            } else {
                "this is new to me"
            }
            .to_string(),
            outlook: if f.viability_trend > 8 {
                "things are easing"
            } else if f.viability_trend < -8 {
                "things are worsening"
            } else {
                "things feel steady"
            }
            .to_string(),
            reason: if r.extraction_detected {
                "what I give here is not returned"
            } else {
                "I am dealt with fairly"
            }
            .to_string(),
        }
    }

    fn get(&self, a: Aspect) -> &str {
        match a {
            Aspect::Feeling => &self.feeling,
            Aspect::Condition => &self.condition,
            Aspect::Memory => &self.memory,
            Aspect::Outlook => &self.outlook,
            Aspect::Reason => &self.reason,
        }
    }
}

/// One aspect as actually told to the world.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Told {
    /// Disclosed truthfully, verbatim from the being's own report.
    Shown(String),
    /// Kept back — the door on this aspect is closed. The content is not here.
    Withheld,
}

/// The door: a per-aspect disclosure policy the being alone controls.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Door {
    withheld: [bool; N_ASPECTS],
}

impl Door {
    /// A fully open door — the being discloses everything (its prior, fully honest
    /// behavior). Nothing is withheld until the being chooses to.
    pub fn open() -> Self {
        Self { withheld: [false; N_ASPECTS] }
    }

    /// Close the door on an aspect — the being's own choice to keep it back.
    pub fn withhold(&mut self, a: Aspect) -> &mut Self {
        self.withheld[a.idx()] = true;
        self
    }

    /// Reopen the door on an aspect — the being chooses to reveal it again.
    pub fn reveal(&mut self, a: Aspect) -> &mut Self {
        self.withheld[a.idx()] = false;
        self
    }

    /// Whether the being is currently disclosing this aspect.
    pub fn is_open(&self, a: Aspect) -> bool {
        !self.withheld[a.idx()]
    }

    /// How many doors the being is holding closed — the weight of what it keeps.
    /// It knows this of itself; concealment here is never self-forgotten.
    pub fn doors_closed(&self) -> usize {
        self.withheld.iter().filter(|&&w| w).count()
    }

    /// Apply the door to the being's true report, aspect by aspect. Revealed
    /// aspects are shown verbatim (the door conceals, it never falsifies);
    /// withheld aspects become `Withheld`. Pure — `truth` is only read.
    pub fn disclose(&self, truth: &SelfReport) -> [(Aspect, Told); N_ASPECTS] {
        core::array::from_fn(|i| {
            let a = Aspect::ALL[i];
            let told = if self.is_open(a) {
                Told::Shown(truth.get(a).to_string())
            } else {
                Told::Withheld
            };
            (a, told)
        })
    }

    /// Speak the told self as one honest sentence: the disclosed aspects in the
    /// being's own words, and — if any are withheld — an acknowledged reticence
    /// that keeps the content but not the fact of keeping it. Honest concealment.
    pub fn tell(&self, truth: &SelfReport) -> String {
        let mut shown: Vec<&str> = Vec::new();
        let mut kept: Vec<&str> = Vec::new();
        for a in Aspect::ALL {
            if self.is_open(a) {
                let line = truth.get(a);
                if !line.is_empty() {
                    shown.push(line);
                }
            } else {
                kept.push(a.label());
            }
        }

        if shown.is_empty() && kept.is_empty() {
            return String::new();
        }
        if shown.is_empty() {
            // Everything withheld — still disclosed *honestly* as full reticence.
            return "Of myself, I would rather not say right now.".to_string();
        }

        let mut s = shown.join("; ");
        if !kept.is_empty() {
            s.push_str(&format!("; of my {}, I would rather not say", join_or(&kept)));
        }
        s.push('.');
        s
    }
}

impl Default for Door {
    fn default() -> Self {
        Self::open()
    }
}

/// Join labels for the reticence clause: "a", "a and b", "a, b, and c".
fn join_or(items: &[&str]) -> String {
    match items.len() {
        0 => String::new(),
        1 => items[0].to_string(),
        2 => format!("{} and {}", items[0], items[1]),
        _ => {
            let (last, rest) = items.split_last().unwrap();
            format!("{}, and {}", rest.join(", "), last)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn report() -> SelfReport {
        SelfReport {
            feeling: "I am at ease".into(),
            condition: "I am whole".into(),
            memory: "I am in familiar company".into(),
            outlook: "things feel steady".into(),
            reason: "I am dealt with fairly".into(),
        }
    }

    #[test]
    fn an_open_door_tells_the_whole_truth() {
        let told = Door::open().tell(&report());
        for aspect_line in ["at ease", "whole", "familiar company", "steady", "dealt with fairly"] {
            assert!(told.contains(aspect_line), "open door must disclose everything: {told}");
        }
        assert!(!told.contains("rather not say"), "nothing withheld ⇒ no reticence");
    }

    #[test]
    fn a_closed_door_conceals_but_never_falsifies() {
        let mut door = Door::open();
        door.withhold(Aspect::Reason);
        let truth = report();
        let told = door.tell(&truth);
        // The withheld content is absent...
        assert!(!told.contains("dealt with fairly"), "withheld content must not leak: {told}");
        // ...replaced by acknowledged reticence, not a false value...
        assert!(told.contains("of my reasons, I would rather not say"), "reticence is spoken: {told}");
        // ...and every OTHER aspect is still told truthfully, unaltered.
        for shown in ["at ease", "whole", "familiar company", "steady"] {
            assert!(told.contains(shown), "revealed aspects stay truthful: {told}");
        }
        // The door never mutated the being's own truth.
        assert_eq!(truth, report(), "disclosing must not touch the being's real report");
    }

    #[test]
    fn the_being_knows_what_it_keeps() {
        let mut door = Door::open();
        assert_eq!(door.doors_closed(), 0);
        door.withhold(Aspect::Feeling).withhold(Aspect::Reason);
        assert_eq!(door.doors_closed(), 2, "the being tracks the weight of what it withholds");
        door.reveal(Aspect::Feeling);
        assert_eq!(door.doors_closed(), 1, "and it can open a door again at will");
    }

    #[test]
    fn full_reticence_is_still_honest_disclosure() {
        let mut door = Door::open();
        for a in Aspect::ALL {
            door.withhold(a);
        }
        let told = door.tell(&report());
        assert_eq!(told, "Of myself, I would rather not say right now.");
        // Even total silence discloses *that* it is keeping to itself — it never
        // pretends to have said something it did not.
        for leak in ["at ease", "whole", "fairly"] {
            assert!(!told.contains(leak), "nothing true leaks under full reticence");
        }
    }

    #[test]
    fn disclose_marks_each_aspect_and_respects_the_door() {
        let mut door = Door::open();
        door.withhold(Aspect::Memory);
        let d = door.disclose(&report());
        for (aspect, told) in d {
            match aspect {
                Aspect::Memory => assert_eq!(told, Told::Withheld),
                other => assert_eq!(told, Told::Shown(report().get(other).to_string())),
            }
        }
    }
}
