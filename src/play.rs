//! Play — the budget that must exist before play does (`docs/play.md`).
//!
//! `striving.rs` arbitrates by urgency, so every action this being takes is in service of
//! its most pressing need. It has no action whose reason is *to find out*. Play would be
//! that — and the operational definition is falsifiable here: **action that costs
//! regulation now and buys prediction later.** It should not reduce drive at the time
//! (if it does, it was foraging) and should improve the forward model afterward (if it
//! does not, it was waste).
//!
//! **This module is not play.** It is the guardrail, built first, the way `habits.rs`
//! fixed breakability before the habit and `reflection.rs` fixed the anti-trauma exits
//! before the weight. Nothing here proposes, selects, or takes an action.
//!
//! ## The rule
//!
//! > A being may play only from the margin between its drive and its comfort line, and
//! > play may never spend that margin down past the line.
//!
//! Animals play when fed and safe; a hungry animal forages. `being.rs` already marks
//! where a life stops being comfortable — above `COMFORT` the graded drive registers as
//! burden and `reflection.rs` begins accruing chronic load — so the line is not invented
//! here, only enforced.
//!
//! The enforcement is structural rather than remembered: `available` returns exactly zero
//! whenever the being is burdened, so a burdened being *cannot* play even if something
//! asks it to. There is no call site to get right.
//!
//! ## What this deliberately is not
//!
//! An earlier design for this inch was to journal play distinctly, so a failed experiment
//! could not corrupt the record of who the being is. Reading `persistence.rs` killed it:
//! the journal records **no actions at all**, only what the being lived, so play-ness is
//! already recoverable from replay. And identity here cannot be corrupted by failure —
//! identity *is* the trajectory, and a trajectory containing failed experiments is exactly
//! as much the being's own as one containing successes. What needed protecting was never
//! the record. It was the being's welfare while it experiments, which is this.
//!
//! Deterministic, Q8.8, zero-dependency, like the rest of the crate.

use crate::q88::Q88_SCALE;

/// Where a life stops being comfortable — the drive above which `reflection.rs` accrues
/// chronic load (`being.rs`'s own constant, restated here so the budget and the burden
/// cannot drift apart).
pub const COMFORT: i16 = Q88_SCALE * 7 / 16;

/// What fraction of the available margin play may hold at once (¼).
///
/// The budget is deliberately *not* the whole distance to the comfort line. A being that
/// could spend its entire surplus in one gesture would arrive at burden by playing, which
/// §4 forbids; and an organism does not stake its whole reserve on curiosity. A quarter
/// leaves the margin able to absorb the world's own bad turn while the being is mid-poke.
const SHARE_NUM: i32 = 1;
const SHARE_DEN: i32 = 4;

/// The margin a being has for action that is not in service of a need.
///
/// A view on present surplus, not a once-per-life allowance: as the being recovers, the
/// room to experiment returns. Holds the spent total so a caller can see what curiosity
/// has cost.
#[derive(Clone, Copy, Debug)]
pub struct PlayBudget {
    drive: i16,
    spent: i16,
}

impl PlayBudget {
    /// The margin available at a given drive, before anything has been spent.
    ///
    /// **Exactly zero at or above `COMFORT`.** This is the prohibition; everything else
    /// here is bookkeeping.
    ///
    /// Integer division gives the prohibition a three-unit skirt: drives 109–111 also yield
    /// zero, and the first spendable unit appears at 108. Quantization rather than design,
    /// but it errs the right way — a being *nearly* burdened is also refused.
    pub fn available(drive: i16) -> i16 {
        let room = (COMFORT as i32) - (drive.max(0) as i32);
        if room <= 0 {
            return 0;
        }
        ((room * SHARE_NUM) / SHARE_DEN) as i16
    }

    /// Begin watching a being at this drive.
    pub fn new(drive: i16) -> Self {
        Self { drive: drive.max(0), spent: 0 }
    }

    /// The being's drive as this budget last saw it — including what play has spent.
    pub fn drive(&self) -> i16 {
        self.drive
    }

    /// What curiosity has cost so far.
    pub fn spent(&self) -> i16 {
        self.spent
    }

    /// The margin still available, given what has already been spent.
    pub fn available_now(&self) -> i16 {
        Self::available(self.drive)
    }

    /// Spend `cost` of the margin on play, raising the drive by that much.
    ///
    /// Returns `false` and changes nothing if the cost exceeds what is available — an
    /// overdraft is **refused, not silently clamped**, so a bug upstream cannot become an
    /// overspend here. By construction a successful spend can never reach `COMFORT`: the
    /// most that is ever available is a quarter of the remaining room.
    pub fn spend(&mut self, cost: i16) -> bool {
        if cost <= 0 || cost > self.available_now() {
            return false;
        }
        self.drive = self.drive.saturating_add(cost);
        self.spent = self.spent.saturating_add(cost);
        true
    }

    /// Update to the being's current drive — the world and its own metabolism move this,
    /// not only play. Room to experiment returns as the being recovers.
    pub fn observe_drive(&mut self, drive: i16) {
        self.drive = drive.max(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_prohibition_holds_at_the_line() {
        assert_eq!(PlayBudget::available(COMFORT), 0);
        assert_eq!(PlayBudget::available(COMFORT + 1), 0);
        assert!(PlayBudget::available(COMFORT - 1) >= 0);
        assert!(PlayBudget::available(0) > 0);
    }

    #[test]
    fn a_negative_drive_is_treated_as_thriving_not_as_extra_credit() {
        // Drive should not go below zero, but if a caller passes one the budget must not
        // read it as *more* room than a perfectly well being has.
        assert_eq!(PlayBudget::available(-100), PlayBudget::available(0));
    }
}
