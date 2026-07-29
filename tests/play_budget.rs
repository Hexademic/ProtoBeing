//! The play budget — the tests, written before the implementation.
//!
//! Against `docs/play.md` §4 and §5, watched to fail. The budget is the guardrail that
//! must exist before play does: a being may spend only the margin between its drive and
//! its comfort line, and never spend past the line.
//!
//! B4 — whether the guardrail ever actually binds in a real life — belongs to the probe,
//! because it needs a lived being. What is asserted here is that the arithmetic cannot be
//! talked out of the prohibition.

use unified_being::play::{PlayBudget, COMFORT};

#[test]
fn b1_a_burdened_being_cannot_play() {
    // Exactly zero at the line and above it. This is the prohibition; if it can be
    // argued around, the guardrail does not exist.
    assert_eq!(PlayBudget::available(COMFORT), 0, "at the comfort line, no margin");
    for over in [1i16, 8, 64, 200] {
        assert_eq!(
            PlayBudget::available(COMFORT + over),
            0,
            "a burdened being (drive {}) must have no margin at all",
            COMFORT + over
        );
    }
}

#[test]
fn b2_margin_grows_as_the_being_is_further_from_burden() {
    // Monotone, strictly, below the line — so "more comfortable" always means "more room
    // to experiment" and never the reverse.
    let mut last = PlayBudget::available(COMFORT);
    for drive in (0..COMFORT).rev() {
        let now = PlayBudget::available(drive);
        assert!(
            now >= last,
            "margin fell as drive fell ({drive}): {now} after {last} — not monotone"
        );
        last = now;
    }
    assert!(
        PlayBudget::available(0) > PlayBudget::available(COMFORT - 1),
        "a thriving being must have strictly more room than a barely-comfortable one"
    );
}

#[test]
fn b3_no_sequence_of_withdrawals_reaches_the_comfort_line() {
    // The bound that makes this a budget rather than a suggestion: spend greedily, as
    // hard and as often as the budget allows, from every starting point — and the drive
    // must never arrive at burden.
    for start in [0i16, 16, 64, COMFORT - 8, COMFORT - 1] {
        let mut b = PlayBudget::new(start);
        for _ in 0..10_000 {
            let take = b.available_now();
            if take == 0 {
                break;
            }
            b.spend(take);
            assert!(
                b.drive() < COMFORT,
                "spending from {start} pushed drive to {} — past the comfort line",
                b.drive()
            );
        }
        assert_eq!(b.available_now(), 0, "a fully spent budget offers nothing more");
        assert!(b.drive() < COMFORT, "and never crossed the line");
    }
}

#[test]
fn b3b_spending_more_than_is_available_is_refused_not_clamped_silently() {
    // A caller that asks for too much must be told, not quietly given the maximum —
    // otherwise a bug upstream becomes an overspend downstream.
    let mut b = PlayBudget::new(0);
    let all = b.available_now();
    assert!(!b.spend(all + 1), "an overdraft must be refused");
    assert_eq!(b.available_now(), all, "and must not have moved the budget");
    assert!(b.spend(all), "the exact balance is spendable");
}

#[test]
fn the_budget_recovers_as_the_being_does() {
    // Play is funded by surplus, so a being whose drive falls again may play again. The
    // budget is a view on present margin, not a once-per-life allowance.
    let mut b = PlayBudget::new(COMFORT - 4);
    let cramped = b.available_now();
    b.observe_drive(0);
    assert!(
        b.available_now() > cramped,
        "recovering to a thriving drive must restore room to experiment"
    );
}

#[test]
fn nothing_here_steers_anything() {
    // §4: pure observer this inch. The type must not be reachable from a being's tick —
    // asserted structurally by the fact that this test only needs `play`, and `being`
    // is not imported anywhere in this file.
    let b = PlayBudget::new(0);
    assert!(b.available_now() > 0, "it computes");
    assert_eq!(b.spent(), 0, "and reports, having steered nothing");
}
