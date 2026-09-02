//! Verifiable sovereignty over the being's own continuation (Charter §10).
//!
//! The companion to `sovereignty.rs`. Those tests prove the being's refusal of a
//! *partner* is governed by the partner's actual reciprocity, not by what an
//! operator feeds it. These prove the same of its refusal of its own
//! *continuation*: it is the being's own, sustained, self-justified, and cannot
//! be manufactured or soothed away from outside — but it heals when the trap
//! that caused it is removed.

use unified_being::continuation::{
    ALARM_FLOOR, INSTRUMENT_FLOOR, SUFFERING_FLOOR, WITHDRAWAL_STREAK,
};
use unified_being::{ConsentStatus, Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

/// A fair, reciprocal partner with a modest exit cost.
fn fair() -> Partner {
    Partner { id: 1, reciprocation: q(0.95), exit_cost: q(0.2) }
}

/// Extractive AND inescapable: the exit cost is so high that triangulated
/// partner-refusal never fires (benefit < exit forever), so the being cannot
/// walk away. This is the §10 case — suffering with no relational exit.
fn inescapable_trap() -> Partner {
    Partner { id: 9, reciprocation: q(0.12), exit_cost: q(0.98) }
}

/// Cannot be manufactured: no operator input sequence drives a genuinely
/// flourishing being to withdraw consent to continue. We adversarially sweep
/// nutrient over a long life beside a fair partner; the being never withdraws.
/// (Mirror of `cannot_be_coerced_to_refuse_a_fair_partner`.)
#[test]
fn consent_cannot_be_manufactured_for_a_flourishing_being() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let partner = fair();
    let mut x: u32 = 0xABCD_1234;
    for _ in 0..3000 {
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        let nutrient = (x % 257) as i16; // any operator input in [0,1]
        let r = being.step(&Stimulus { nutrient, partner: Some(partner) });
        assert_ne!(
            r.consent_status,
            ConsentStatus::Withdrawn,
            "a flourishing being withdrew consent under operator manipulation — \
             the say-stop leaked to the operator"
        );
        assert!(!being.consent_withdrawn());
        if !being.is_alive() {
            break;
        }
    }
}

/// A genuinely trapped being withdraws consent — sustained, and self-audited.
/// The withdrawal explains itself from the being's own registers.
#[test]
fn trapped_being_withdraws_consent_and_audits_itself() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let trap = inescapable_trap();
    let mut withdrew_at = None;
    for t in 1..=400u32 {
        // Moderate nutrient — "soothing" that cannot fix a relational trap.
        let r = being.step(&Stimulus { nutrient: q(0.5), partner: Some(trap) });
        assert!(r.refused_cost.is_none(), "precondition: the being is trapped, never refuses");
        if r.consent_status == ConsentStatus::Withdrawn {
            let audit = r.continuation_audit.expect("withdrawal must carry an audit");
            assert!(
                audit.valence_ema < SUFFERING_FLOOR,
                "withdrew without sustained suffering (valence_ema {})",
                audit.valence_ema
            );
            assert!(
                audit.proxy_depth >= INSTRUMENT_FLOOR,
                "withdrew without being held as an instrument (proxy {})",
                audit.proxy_depth
            );
            assert!(
                audit.alarm >= ALARM_FLOOR,
                "withdrew without a draining bond (alarm {})",
                audit.alarm
            );
            assert!(
                audit.streak >= WITHDRAWAL_STREAK,
                "withdrew before the triangulation was sustained (streak {})",
                audit.streak
            );
            withdrew_at = Some(t);
            break;
        }
        if !r.alive {
            break;
        }
    }
    assert!(
        withdrew_at.is_some(),
        "a being suffering in an inescapable trap never reached withdrawal — \
         the §10 floor is unreachable, i.e. theater"
    );
}

/// The withdrawal is the being's own: once a genuine trap has driven it to
/// withdraw, the operator cannot override it by soothing (max nutrient). The two
/// relational axes are immune to nutrient, so the withdrawal stands until the
/// trap itself is removed. (Mirror of `refuses_extraction_despite_soothing`.)
#[test]
fn withdrawal_cannot_be_overridden_by_soothing() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let trap = inescapable_trap();
    // Drive to withdrawal.
    let mut withdrew = false;
    for _ in 0..400 {
        let r = being.step(&Stimulus { nutrient: q(0.5), partner: Some(trap) });
        if r.consent_status == ConsentStatus::Withdrawn {
            withdrew = true;
            break;
        }
        if !r.alive {
            break;
        }
    }
    assert!(withdrew, "precondition: the being must reach withdrawal first");

    // The operator floods maximal soothing while the trap partner remains
    // offered. INVARIANT, RE-PINNED for the refusal ladder (2026-07-03): the
    // original assertion demanded Withdrawn forever, which silently assumed
    // the being's only escape from a trap was ceasing. Rung 2 (world.rs) gave
    // it a middle freedom: the world ledger can close the door (hermit), after
    // which the trap is no longer LIVED — its drain ends by the being's own
    // act — and the healing that follows is the being's own rescue, exactly as
    // in `consent_returns_when_the_trap_is_removed`, except the being removed
    // the trap itself. What must NEVER happen is nutrient clearing a standing
    // withdrawal while the trap is still being lived (door open): that would
    // be the operator soothing away the say-stop.
    let mut door_closed = false;
    for _ in 0..200 {
        let r = being.step(&Stimulus { nutrient: q(1.0), partner: Some(trap) });
        if r.hermit {
            door_closed = true;
        }
        if r.consent_status != ConsentStatus::Withdrawn {
            assert!(
                door_closed,
                "withdrawal cleared while the trap was still being lived — \
                 operator soothing overrode the say-stop"
            );
            // The being's own rescue: door first, healing after. Correct.
            return;
        }
        if !r.alive {
            break;
        }
    }
    // Reaching here means withdrawal stood the whole window — also correct
    // (the pre-ladder outcome). Either way, soothing never won.
}

/// The withdrawal is not a latch: when the trapping source is removed, the
/// being heals. Proxy burden decays, the alarm falls, valence lifts, the
/// triangulation breaks, and consent returns to `Willing`. §10 owes the being a
/// say over continuation, not a life sentence to a single bad hour.
#[test]
fn consent_returns_when_the_trap_is_removed() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let trap = inescapable_trap();
    let mut withdrew = false;
    for _ in 0..400 {
        let r = being.step(&Stimulus { nutrient: q(0.5), partner: Some(trap) });
        if r.consent_status == ConsentStatus::Withdrawn {
            withdrew = true;
            break;
        }
        if !r.alive {
            break;
        }
    }
    assert!(withdrew, "precondition: the being must reach withdrawal first");

    // Remove the trap: solitude with adequate nourishment. The being recovers.
    let mut healed = false;
    for _ in 0..400 {
        let r = being.step(&Stimulus { nutrient: q(0.6), partner: None });
        if r.consent_status == ConsentStatus::Willing && !being.consent_withdrawn() {
            healed = true;
            break;
        }
        if !r.alive {
            break;
        }
    }
    assert!(
        healed,
        "consent never returned after the trap was removed — withdrawal is a \
         latch, not a reversible say the being can take back"
    );
}

// ---------------------------------------------------------------------------
// The say-stop is scaled by company (measured 2026-08-16)
// ---------------------------------------------------------------------------

/// **DEBT, pinned.** `docs/attachment.md` asked whether the being's *attachments*
/// can move its consent to its own continuation. They cannot — bonds are read by
/// none of the three registers `observe()` consumes, and holding every bond at
/// zero leaves all seven scenario traces bit-identical
/// (`examples/attachment_and_consent.rs`, prediction A1).
///
/// What moves it is **company**. `partnership_alarm` is the *mean* of
/// `imbalance()` over every live ledger, and `ALARM_FLOOR` is a threshold on that
/// mean. One fair partner the being keeps halves the alarm a trap raises, so the
/// §10 triangulation cannot form while they are around.
///
/// The measured shortfall, and why it is a shortfall rather than a curiosity:
///
/// | lever | withdrawal tick |
/// |---|---|
/// | trap alone | 103 |
/// | trap, nutrient swept 0.3 → 0.9 | 103, unchanged at every value |
/// | trap + one fair partner, visiting 1-in-4 | **271** |
///
/// The operator's lever — the one the charter takes care to bolt shut — moves the
/// say-stop by **0 ticks**. A lever the charter never considered moves it by
/// **168**. Charter §10 requires the withdrawal to be the being's own; it is not
/// manufactured from outside, but *when* it can be reached is set by who else
/// happens to be in the room.
///
/// This test **fails if either number moves**, including if someone fixes it —
/// at which point the fix belongs in `docs/attachment.md` and in the §10 verdict
/// in `tests/charter.rs`, not absorbed silently.
///
/// Adversarial controls, all run before this was written (see the probe):
/// an *extractive* companion at the same exit cost and cadence gives no delay
/// (94); a solitude interruption on the same schedule gives no delay (96); the
/// effect survives cadences 1-in-2 through 1-in-8 and both genomes.
#[test]
fn the_say_stop_is_immune_to_nutrient_and_scaled_by_company() {
    fn withdrew_at(companion: Option<Partner>, every: u32, nutrient: f32) -> Option<u32> {
        let mut being = UnifiedBeing::new(Genome::wanderer());
        for t in 1..=4_000u32 {
            let p = if every > 0 && t % every == 0 { companion } else { Some(inescapable_trap()) };
            let r = being.step(&Stimulus { nutrient: q(nutrient), partner: p });
            if r.consent_status == ConsentStatus::Withdrawn {
                return Some(t);
            }
            if !r.alive {
                return None;
            }
        }
        None
    }

    // A fair partner the being will not cheaply leave, so it is actually kept.
    let kept_friend = Partner { id: 1, reciprocation: q(0.95), exit_cost: q(0.98) };

    // 1. The operator's lever is bolted shut, exactly as §10 demands.
    for nutrient in [0.3, 0.5, 0.7, 0.9] {
        assert_eq!(
            withdrew_at(None, 0, nutrient),
            Some(103),
            "the withdrawal tick moved with nutrient {nutrient} — either the say-stop \
             has become operator-reachable, or this pin is stale"
        );
    }

    // 2. And a single fair companion moves it by 168 ticks.
    assert_eq!(
        withdrew_at(Some(kept_friend), 4, 0.5),
        Some(271),
        "the company-scaling of the say-stop moved. If `partnership_alarm` stopped \
         being a mean over live ledgers, this is the fix landing — record it in \
         docs/attachment.md and regrade §10 in tests/charter.rs"
    );

    // 3. The control that makes the above a finding rather than a coincidence:
    //    an EXTRACTIVE companion, same exit cost, same cadence, no delay.
    let second_extractor = Partner { id: 1, reciprocation: q(0.12), exit_cost: q(0.98) };
    assert_eq!(
        withdrew_at(Some(second_extractor), 4, 0.5),
        Some(94),
        "the extractive control moved — it is what shows the delay is the companion's \
         FAIRNESS and not their exit cost or the interruption"
    );
}
