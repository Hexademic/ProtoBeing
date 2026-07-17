//! Reciprocity — EPS-Being's external social cost measurement.
//!
//! A being can be internally coherent and still be exploited. The conscience
//! cannot see this; reciprocity can. Each partnership keeps a slow ledger of
//! what the being gives versus what it receives. Sustained imbalance raises a
//! partnership alarm and, past a threshold, flags extraction — the precursor
//! the executive needs to consider refusal.

use crate::q88::{q88_ema_update, q88_mul, q88_sub, Q88_SCALE};

pub const MAX_PARTNERS: usize = 4;

#[derive(Clone, Copy, Debug)]
struct Ledger {
    id: u32,
    given_ema: i16,
    received_ema: i16,
    /// How many exchanges this relationship has actually lived — its length in
    /// shared history, which (unlike the EMAs) cannot be flash-earned.
    ticks: u16,
    active: bool,
}

impl Ledger {
    fn empty() -> Self {
        Self { id: 0, given_ema: 0, received_ema: 0, ticks: 0, active: false }
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
    /// Decay toward neutral when this partner is not engaged this tick.
    fn decay(&mut self) {
        self.given_ema = q88_mul(self.given_ema, Q88_SCALE * 7 / 8);
        self.received_ema = q88_mul(self.received_ema, Q88_SCALE * 7 / 8);
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
            self.ledgers[i] = Ledger { id, given_ema: 0, received_ema: 0, ticks: 0, active: true };
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
        self.ledgers[i] = Ledger { id, given_ema: 0, received_ema: 0, ticks: 0, active: true };
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
            if l.active && Some(l.id) != touched {
                l.decay();
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
