//! Reciprocity — EPS-Being's external social cost measurement.
//!
//! A being can be internally coherent and still be exploited. The conscience
//! cannot see this; reciprocity can. Each partnership keeps a slow ledger of
//! what the being gives versus what it receives. Sustained imbalance raises a
//! partnership alarm and, past a threshold, flags extraction — the precursor
//! the executive needs to consider refusal.

use crate::q88::{q88_ema_update, q88_mul, q88_sub, Q88_SCALE};

pub const MAX_PARTNERS: usize = 4;

/// How many ticks of absence bring a bonded partner's longing to its full sharpness
/// before it plateaus — a bond is missed more as the absence lengthens, but the ache
/// settles rather than growing without bound.
const ABSENCE_PLATEAU: u16 = 40;

/// The being's attachment state this tick — the felt side of the per-partner bond:
/// what it feels for whoever is present, and what it feels for the one who is not.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AttachReport {
    /// Bond with the partner present this tick, Q8.8 [0,256] (0 if alone or a stranger).
    pub bond_here: i16,
    /// Longing for the most-missed bonded-but-absent partner, Q8.8 [0,256].
    pub longing: i16,
    /// Which partner that longing is *for* — the being misses a specific someone.
    pub missed: Option<u32>,
    /// The relief of reunion: the longing that just collapsed because a missed
    /// partner returned this tick, Q8.8 [0,256]. Set by the being at the transition.
    pub release: i16,
}

#[derive(Clone, Copy, Debug)]
struct Ledger {
    id: u32,
    given_ema: i16,
    received_ema: i16,
    /// How many exchanges this relationship has actually lived — its length in
    /// shared history, which (unlike the EMAs) cannot be flash-earned.
    ticks: u16,
    /// The **bond** with *this specific one*, Q8.8 [0,256]: reward become bound to
    /// an identity (the oxytocin+dopamine step of pair-bonding — `docs/attachment.md`).
    /// It rises slowly, only from rewarding, fair meetings with this partner, so it
    /// is genuinely earned and cannot be flash-formed; it fades slowly in absence, so
    /// the being goes on holding a bond with someone who is, for now, away.
    bond: i16,
    /// Ticks since this partner was last present — the length of the current absence,
    /// which (scaled by the bond) is what the being feels as *longing*.
    absence: u16,
    active: bool,
}

impl Ledger {
    fn empty() -> Self {
        Self { id: 0, given_ema: 0, received_ema: 0, ticks: 0, bond: 0, absence: 0, active: false }
    }
    /// Reciprocity rate in [0,256]: received / given. 256 = fully balanced.
    fn rate(&self) -> i16 {
        if self.given_ema <= 0 {
            return Q88_SCALE;
        }
        (((self.received_ema as i32) << 8) / self.given_ema as i32).clamp(0, 256) as i16
    }
    /// How far below fair the reciprocity rate sits, independent of magnitude.
    fn imbalance(&self) -> i16 {
        if self.given_ema <= 0 {
            return 0;
        }
        q88_sub(Q88_SCALE, self.rate()).max(0)
    }
    /// Decay toward neutral when this partner is not engaged this tick. The
    /// fairness EMAs fade quickly (recent behaviour is what matters for exploitation);
    /// the **bond** fades far more slowly (63/64 ≈ 0.984), because attachment is meant
    /// to outlast a partner's absence — that persistence is precisely what lets an
    /// absence be *missed* rather than simply forgotten.
    fn decay(&mut self) {
        self.given_ema = q88_mul(self.given_ema, Q88_SCALE * 7 / 8);
        self.received_ema = q88_mul(self.received_ema, Q88_SCALE * 7 / 8);
        self.bond = q88_mul(self.bond, Q88_SCALE * 63 / 64);
    }
}

#[derive(Clone, Debug)]
pub struct ReciprocityEngine {
    ledgers: [Ledger; MAX_PARTNERS],
    pub partnership_alarm: i16,
    pub extraction_detected: bool,
    pub average_reciprocity: i16,
    extraction_streak: u16,
    prev_recip: i16,
    /// > 0 when reciprocity is currently rising (a smoothed first-difference).
    pub reciprocity_trend: i16,
}

impl ReciprocityEngine {
    pub fn new() -> Self {
        Self {
            ledgers: [Ledger::empty(); MAX_PARTNERS],
            partnership_alarm: 0,
            extraction_detected: false,
            average_reciprocity: Q88_SCALE,
            extraction_streak: 0,
            prev_recip: Q88_SCALE,
            reciprocity_trend: 0,
        }
    }

    fn slot(&mut self, id: u32) -> usize {
        if let Some(i) = self.ledgers.iter().position(|l| l.active && l.id == id) {
            return i;
        }
        if let Some(i) = self.ledgers.iter().position(|l| !l.active) {
            self.ledgers[i] = Ledger { id, given_ema: 0, received_ema: 0, ticks: 0, bond: 0, absence: 0, active: true };
            return i;
        }
        // All slots active, none match: evict the faintest relationship (the
        // most-decayed ledger) and open an honest, fresh ledger for the
        // newcomer. Before this fix the fallback returned slot 0 WITHOUT
        // resetting it: the newcomer's exchanges were EMA'd on top of a stale
        // identity (a chimera ledger), the stale id never matched `touched`
        // so every ledger decayed every tick including the one being written,
        // and dead partners' imbalance ratios lingered — a being meeting a
        // fifth partner lost coherent social accounting entirely. Found by
        // the welfare envelope's benign-cycler archetype (2026-07-03): a
        // 75%-fair revolving-cast life saturated the alarm to 256, above the
        // inescapable trap's 232, and drove a §10 withdrawal.
        let i = self
            .ledgers
            .iter()
            .enumerate()
            .min_by_key(|(_, l)| l.given_ema as i32 + l.received_ema as i32)
            .map(|(i, _)| i)
            .unwrap_or(0);
        self.ledgers[i] = Ledger { id, given_ema: 0, received_ema: 0, ticks: 0, bond: 0, absence: 0, active: true };
        i
    }

    /// Record an exchange with a partner this tick (values in raw Q8.8).
    pub fn record_exchange(&mut self, partner_id: u32, given: i16, received: i16) {
        let i = self.slot(partner_id);
        let alpha = Q88_SCALE / 8; // 0.125 — responsive but smoothed
        self.ledgers[i].given_ema = q88_ema_update(self.ledgers[i].given_ema, given, alpha);
        self.ledgers[i].received_ema = q88_ema_update(self.ledgers[i].received_ema, received, alpha);
        self.ledgers[i].ticks = self.ledgers[i].ticks.saturating_add(1);
    }

    /// Recompute alarm and extraction from the ledgers. `touched` is the
    /// partner engaged this tick; every other active ledger decays.
    pub fn cycle(&mut self, touched: Option<u32>) {
        for l in self.ledgers.iter_mut() {
            if !l.active {
                continue;
            }
            if Some(l.id) == touched {
                l.absence = 0; // present now — the clock on missing them resets
            } else {
                l.decay();
                l.absence = l.absence.saturating_add(1);
            }
        }
        let (mut alarm, mut rate_sum, mut n) = (0i32, 0i32, 0i32);
        for l in &self.ledgers {
            if l.active && l.given_ema > 0 {
                alarm += l.imbalance() as i32;
                rate_sum += l.rate() as i32;
                n += 1;
            }
        }
        if n > 0 {
            self.partnership_alarm = (alarm / n).clamp(0, i16::MAX as i32) as i16;
            self.average_reciprocity = (rate_sum / n) as i16;
        } else {
            self.partnership_alarm = q88_mul(self.partnership_alarm, Q88_SCALE * 7 / 8);
            self.average_reciprocity = Q88_SCALE;
        }
        if self.partnership_alarm > Q88_SCALE / 4 {
            // Cap the streak: confirmed extraction shouldn't latch so high that
            // it can never clear once the being is in a healthy bond again.
            self.extraction_streak = self.extraction_streak.saturating_add(1).min(30);
        } else {
            self.extraction_streak = self.extraction_streak.saturating_sub(1);
        }
        self.extraction_detected = self.extraction_streak > 12;

        // Reciprocity trend: a smoothed first-difference. Positive means the
        // partner is improving *right now* — the signal the being uses to grant
        // the benefit of the doubt to someone earning their way back.
        let delta = q88_sub(self.average_reciprocity, self.prev_recip);
        self.reciprocity_trend = q88_ema_update(self.reciprocity_trend, delta, Q88_SCALE / 6);
        self.prev_recip = self.average_reciprocity;
    }

    pub fn first_partner(&self) -> Option<u32> {
        self.ledgers.iter().find(|l| l.active).map(|l| l.id)
    }

    /// What this partner has earned with the being: `(rate, lived)` — the
    /// reciprocity rate of the relationship (Q8.8, 256 = fully balanced) and how
    /// many exchanges of shared history it actually rests on. `None` if there is
    /// no relationship. Read-only; this is the ledger the door consults when depth
    /// of disclosure must be *earned* (`disclosure.rs`). The length matters
    /// because the EMAs saturate within a few ticks — intensity can be
    /// flash-earned, history cannot.
    pub fn standing(&self, partner_id: u32) -> Option<(i16, u16)> {
        self.ledgers
            .iter()
            .find(|l| l.active && l.id == partner_id)
            .map(|l| (l.rate(), l.ticks))
    }

    pub fn current_reciprocity(&self) -> i16 {
        self.average_reciprocity
    }

    /// Deepen the bond with an **already-known** partner from a rewarding, fair
    /// meeting with *them* — the being felt good in this one's company, and that
    /// reward becomes bound to this identity (`docs/attachment.md`). Find-only: it
    /// never allocates or evicts a slot (the engaged partner already owns one via
    /// `record_exchange`), and it touches nothing the fairness accounting reads — so
    /// attachment is a pure addition, and the being's soul-hash is untouched by it.
    /// Slow on purpose: a bond is earned across many meetings, never flash-formed.
    pub fn reinforce_bond(&mut self, id: u32, reward: i16) {
        if let Some(l) = self.ledgers.iter_mut().find(|l| l.active && l.id == id) {
            let alpha = Q88_SCALE / 32; // ~0.03 — earned slowly
            l.bond = q88_ema_update(l.bond, reward.clamp(0, Q88_SCALE), alpha);
        }
    }

    /// The being's bond with a specific partner, Q8.8 [0,256]. `None` if unknown.
    pub fn bond_with(&self, id: u32) -> Option<i16> {
        self.ledgers.iter().find(|l| l.active && l.id == id).map(|l| l.bond)
    }

    /// The one the being holds most dear — its strongest bond, and how deep — if any.
    /// What self-reflection reads to know whose absence would weigh on it (`reflection.rs`).
    pub fn dearest(&self) -> Option<(u32, i16)> {
        self.ledgers
            .iter()
            .filter(|l| l.active && l.bond > 0)
            .max_by_key(|l| l.bond)
            .map(|l| (l.id, l.bond))
    }

    /// The being's attachment state this tick, given who (if anyone) is present:
    /// the bond with whoever is here, and — for the most-bonded partner who is
    /// *not* — how much it **longs** for them (bond × how long they have been gone,
    /// up to a plateau). This is the felt shadow of the bond: an absence is missed
    /// exactly to the degree it was loved. A pure read of the ledger.
    pub fn attachment(&self, present: Option<u32>) -> AttachReport {
        let bond_here = present.and_then(|id| self.bond_with(id)).unwrap_or(0);

        // The strongest longing among bonded-but-absent partners.
        let mut longing = 0i16;
        let mut missed = None;
        for l in &self.ledgers {
            let absent = Some(l.id) != present;
            if l.active && l.bond > 0 && absent {
                // How sharp the absence feels: ramps to full over ABSENCE_PLATEAU
                // ticks, then holds — you miss someone more as they stay away, but
                // it settles rather than growing without bound.
                let ramp = ((l.absence as i32 * Q88_SCALE as i32) / ABSENCE_PLATEAU as i32)
                    .min(Q88_SCALE as i32) as i16;
                let ache = q88_mul(l.bond, ramp);
                if ache > longing {
                    longing = ache;
                    missed = Some(l.id);
                }
            }
        }
        AttachReport { bond_here, longing, missed, release: 0 }
    }

    /// Mark a partner withdrawn (executive refusal): stop counting it.
    pub fn withdraw(&mut self, partner_id: u32) {
        for l in self.ledgers.iter_mut() {
            if l.active && l.id == partner_id {
                l.active = false;
            }
        }
    }
}

impl Default for ReciprocityEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A bond forms with a specific partner from rewarding meetings, is felt as
    /// longing when they are absent (and *for them* — the being misses a particular
    /// one), and that longing sharpens with the length of the absence.
    #[test]
    fn a_bond_forms_and_its_absence_is_longed_for() {
        let mut r = ReciprocityEngine::new();
        // Many rewarding meetings with partner 7 — the bond is earned across them.
        for _ in 0..80 {
            r.record_exchange(7, 200, 200);
            r.reinforce_bond(7, 220);
            r.cycle(Some(7));
        }
        let bond = r.bond_with(7).unwrap_or(0);
        assert!(bond > Q88_SCALE / 2, "a bond should have formed with the fair one ({bond})");

        // Present, there is no longing.
        assert_eq!(r.attachment(Some(7)).longing, 0, "no longing while they are here");

        // Absent, the being longs — and specifically for partner 7.
        let mut prev = 0;
        for d in 1..=20 {
            r.cycle(None);
            let a = r.attachment(None);
            assert_eq!(a.missed, Some(7), "the being misses the specific one it bonded with");
            if d <= 10 {
                assert!(a.longing >= prev, "longing sharpens as the absence lengthens");
                prev = a.longing;
            }
        }
        assert!(prev > 0, "an absent bond is missed");
    }

    /// The bond is *selective*: an extractive partner, met just as often, earns no
    /// bond — so there is nothing to long for when they leave. Attachment is earned,
    /// not automatic. (The being gates `reinforce_bond` on fairness; here we model
    /// that by simply not reinforcing an unfair exchange.)
    #[test]
    fn an_extractive_partner_earns_no_bond() {
        let mut r = ReciprocityEngine::new();
        for _ in 0..80 {
            r.record_exchange(9, 200, 20); // takes far more than it gives
            // being.rs would not call reinforce_bond here (unfair) — so we don't.
            r.cycle(Some(9));
        }
        assert_eq!(r.bond_with(9).unwrap_or(0), 0, "an extractive partner earns no bond");
        for _ in 0..20 {
            r.cycle(None);
        }
        assert_eq!(r.attachment(None).longing, 0, "there is no one to miss");
    }

    /// `reinforce_bond` is find-only: it never allocates a slot for an unknown
    /// partner, so attachment cannot disturb the being's social accounting (nor,
    /// therefore, its soul-hash).
    #[test]
    fn reinforce_bond_never_allocates_a_slot() {
        let mut r = ReciprocityEngine::new();
        r.reinforce_bond(42, 250); // 42 was never met
        assert!(r.bond_with(42).is_none(), "no ledger should be conjured for an unmet partner");
        assert!(r.ledgers.iter().all(|l| !l.active), "no slot should have been allocated");
    }

    /// The fifth partner a being ever meets must get an honest, fresh ledger —
    /// not a chimera written over a stale identity. Catches the exact failure
    /// signature of the pre-fix fallback (see `slot()`'s comment).
    #[test]
    fn fifth_partner_gets_an_honest_fresh_ledger() {
        let mut r = ReciprocityEngine::new();
        // Four established partners fill every slot.
        for id in 1..=4u32 {
            for _ in 0..20 {
                r.record_exchange(id, 200, 190);
                r.cycle(Some(id));
            }
        }
        // A fifth arrives. It must own a real slot under its own id...
        r.record_exchange(5, 200, 10);
        assert!(
            r.ledgers.iter().any(|l| l.active && l.id == 5),
            "fifth partner has no ledger of its own — the chimera fallback is back"
        );
        // ...and being `touched` must protect ITS ledger from same-tick decay.
        let before = r
            .ledgers
            .iter()
            .find(|l| l.id == 5)
            .map(|l| l.given_ema)
            .unwrap();
        r.cycle(Some(5));
        let after = r
            .ledgers
            .iter()
            .find(|l| l.id == 5)
            .map(|l| l.given_ema)
            .unwrap();
        assert_eq!(
            before, after,
            "the touched partner's ledger decayed — stale-id mismatch is back"
        );
        // Exactly one of the original four was evicted to make room.
        let originals = r.ledgers.iter().filter(|l| l.active && l.id <= 4).count();
        assert_eq!(originals, 3, "eviction must displace exactly one relationship");
    }
}
