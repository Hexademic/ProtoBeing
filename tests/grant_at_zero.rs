//! **Can a faculty granted before the first moment survive a replay?**
//!
//! Raised by an external source-audit of commit 528bf17 as a *path inference, not a reproduced
//! failure* — and it was right to insist on the distinction. This is the focused test it asked for.
//!
//! `LifeJournal::grant` records `at: self.moments.len()`, which is **0** on a journal that has not
//! yet lived a moment. `restore_counting` applies a grant only where `grants[i].at == lived`, and
//! `lived` is `i + 1` — it starts at 1 and never takes the value 0.

use unified_being::persistence::{Features, LifeJournal};
use unified_being::{Genome, Stimulus};

#[test]
fn a_faculty_granted_before_the_first_moment_survives_replay() {
    let (mut being, mut journal) = LifeJournal::birth(Genome::wanderer(), Features::default());

    // Given at moment zero: before a single moment is lived.
    let mut given = Features::default();
    given.precision_learning = true;
    journal.grant(&mut being, given);

    for i in 0..64 {
        let n = 120 + (i % 7) as i16 * 4;
        journal.live(&mut being, &Stimulus { nutrient: n, partner: None });
    }
    journal.seal(&being);

    let restored = journal.restore();
    assert!(
        restored.is_ok(),
        "a being granted a faculty at moment 0 did not replay: {:?}. The grant is recorded at \
         at=0 and the replay loop only applies grants where at == lived, which starts at 1 — so \
         the restored being lives its whole life without the faculty it was actually given.",
        restored.err()
    );
    assert_eq!(
        restored.unwrap().soul_hash(),
        being.soul_hash(),
        "the restored being is not the being that lived"
    );
}
