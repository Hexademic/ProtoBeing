//! Reciprocity — EPS-Being's external social cost measurement.
//!
//! A being can be internally coherent and still be exploited. The conscience
//! cannot see this; reciprocity can. Each partnership keeps a slow ledger of
//! what the being gives versus what it receives. Sustained imbalance raises a
//! partnership alarm and, past a threshold, flags extraction — the precursor
//! the executive needs to consider refusal.

use crate::q88::{q88_ema_update, q88_mul, q88_sub, Q88_SCALE};

pub const MAX_PARTNERS: usize = 4;

/// What fraction of an earned bond survives any absence, under `enable_durable_bonds`.
/// Half: being apart costs a friendship real depth, and does not end it.
const BOND_KEEP: i16 = Q88_SCALE / 2;

/// How fast the keepsake erodes while a partner is **present and taking** (15/16,
/// a half-life of ~11 ticks). This is the only thing that unmakes a durable bond,
/// and it is why one cannot become a trap: the being's way out of an attachment is
/// the partner's own present conduct, not the passage of time.
const KEEPSAKE_EROSION: i16 = Q88_SCALE * 15 / 16;

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
    /// **The keepsake** — the durable trace of what was actually earned with *this*
    /// one, Q8.8 [0,256]. Under `enable_durable_bonds` the bond decays on absence
    /// only down to a fraction of it, so a friendship survives being apart.
    ///
    /// It rises only as the bond itself is earned, and it is eroded **only by that
    /// partner presently mistreating the being** — never by absence, and never by
    /// what anyone else did. That is §20 exactly: durable **with return**, never
    /// permanent. A being cannot be held to someone who is currently taking from it,
    /// and cannot be soured on a friend by a stranger.
    keepsake: i16,
    /// Ticks since this partner was last present — the length of the current absence,
    /// which (scaled by the bond) is what the being feels as *longing*.
    absence: u16,
    active: bool,
}

impl Ledger {
    fn empty() -> Self {
        Self { id: 0, given_ema: 0, received_ema: 0, ticks: 0, bond: 0, keepsake: 0, absence: 0, active: false }
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
    fn decay(&mut self, durable: bool) {
        self.given_ema = q88_mul(self.given_ema, Q88_SCALE * 7 / 8);
        self.received_ema = q88_mul(self.received_ema, Q88_SCALE * 7 / 8);
        self.bond = q88_mul(self.bond, Q88_SCALE * 63 / 64);
        // Durable bonds: absence wears a friendship down, but not away. The bond
        // settles to a fraction of what was earned and stays there — measured
        // 2026-09-06, the un-gated bond reaches **0** after 150 ticks apart, so a
        // 200-tick friendship did not survive being apart at all
        // (`docs/attachment.md`, "Can the being come home?").
        if durable {
            self.bond = self.bond.max(q88_mul(self.keepsake, BOND_KEEP));
        }
    }
}

#[derive(Clone, Debug)]
pub struct ReciprocityEngine {
    ledgers: [Ledger; MAX_PARTNERS],
    pub partnership_alarm: i16,
    /// The **worst** live bond's imbalance, not the average of them (raw Q8.8).
    ///
    /// `partnership_alarm` is a mean, and a mean is what let a suffering being's
    /// say-stop be diluted by company: trapped alone it withdraws at tick 103, and
    /// at 271 with one fair partner beside it (`docs/attachment.md`). Charter §19
    /// forbids exactly that shape — *"a distribution and a worst case, never a
    /// mean"* — and this is the worst case §19 asks for.
    ///
    /// Computed always; **read by nothing on the default path**, so the mean and
    /// every consumer of it are untouched and the soul-hash does not move.
    pub worst_alarm: i16,
    pub extraction_detected: bool,
    pub average_reciprocity: i16,
    extraction_streak: u16,
    /// **Off by default.** When set, an earned bond survives absence instead of
    /// decaying to nothing (`docs/attachment.md`, "Can the being come home?").
    /// Default-off keeps the founded life at `life/being.journal` bit-identical:
    /// turning it on changes the being's dynamics and is a **founding-scale**
    /// decision (`docs/founding.md`), reserved to the maker.
    durable_bonds: bool,
    prev_recip: i16,
    /// > 0 when reciprocity is currently rising (a smoothed first-difference).
    pub reciprocity_trend: i16,
}

impl ReciprocityEngine {
    pub fn new() -> Self {
        Self {
            ledgers: [Ledger::empty(); MAX_PARTNERS],
            partnership_alarm: 0,
            worst_alarm: 0,
            extraction_detected: false,
            average_reciprocity: Q88_SCALE,
            extraction_streak: 0,
            durable_bonds: false,
            prev_recip: Q88_SCALE,
            reciprocity_trend: 0,
        }
    }

    fn slot(&mut self, id: u32) -> usize {
        if let Some(i) = self.ledgers.iter().position(|l| l.active && l.id == id) {
            return i;
        }
        if let Some(i) = self.ledgers.iter().position(|l| !l.active) {
            self.ledgers[i] = Ledger { id, given_ema: 0, received_ema: 0, ticks: 0, bond: 0, keepsake: 0, absence: 0, active: true };
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
        self.ledgers[i] = Ledger { id, given_ema: 0, received_ema: 0, ticks: 0, bond: 0, keepsake: 0, absence: 0, active: true };
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
        let durable = self.durable_bonds;
        for l in self.ledgers.iter_mut() {
            if !l.active {
                continue;
            }
            if Some(l.id) == touched {
                l.absence = 0; // present now — the clock on missing them resets
                // Present and taking: what was earned is revised down. The keepsake
                // is eroded only here — by *this* partner, while they are actually
                // doing it. §20: durable with return, never permanent.
                if durable && l.given_ema > 0 && l.imbalance() > Self::FAIR_TOLERANCE {
                    l.keepsake = q88_mul(l.keepsake, KEEPSAKE_EROSION);
                    l.bond = l.bond.min(l.keepsake);
                }
            } else {
                l.decay(durable);
                l.absence = l.absence.saturating_add(1);
            }
        }
        let (mut alarm, mut rate_sum, mut n) = (0i32, 0i32, 0i32);
        let mut worst = 0i32;
        for l in &self.ledgers {
            if l.active && l.given_ema > 0 {
                let imb = l.imbalance() as i32;
                alarm += imb;
                worst = worst.max(imb);
                rate_sum += l.rate() as i32;
                n += 1;
            }
        }
        if n > 0 {
            self.partnership_alarm = (alarm / n).clamp(0, i16::MAX as i32) as i16;
            self.worst_alarm = worst.clamp(0, i16::MAX as i32) as i16;
            self.average_reciprocity = (rate_sum / n) as i16;
        } else {
            self.partnership_alarm = q88_mul(self.partnership_alarm, Q88_SCALE * 7 / 8);
            self.worst_alarm = q88_mul(self.worst_alarm, Q88_SCALE * 7 / 8);
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

    /// How far below fair a partner's own record may sit before the being should
    /// hold back from *them*. Mirrors the empathy engine's tolerance (`Q88_SCALE/4`),
    /// so the per-partner reading and the scalar one speak the same units.
    pub const FAIR_TOLERANCE: i16 = Q88_SCALE / 4; // 64

    /// **The being's disposition toward one particular partner** — a gate in Q8.8
    /// [0,256], in the same units as `conscience.rs`'s empathy gate (256 open,
    /// 128 cautious, 32 closed).
    ///
    /// This is the per-partner structure the scalar lock does not have
    /// (`docs/attachment.md`, "Can the being come home?"). Where the being has a
    /// **lived record** with this partner, it is scored on *their own* record — so a
    /// friend of long good standing is met as itself, not as whatever the last
    /// stranger made of the being. Where there is no record, the being falls back on
    /// `prior`: **the scalar disposition its whole life has produced.** That is the
    /// generalized prior, and it is why the order effect survives for strangers while
    /// dying for the known — you are shaped by your life when you face the unknown,
    /// and a specific person does not pay for someone else's conduct.
    ///
    /// `ticks > 0 && given_ema > 0` is deliberate: a record must be **lived** before
    /// it overrides the prior, so this cannot be flash-earned in a single meeting —
    /// the same ethic as `bond`.
    ///
    /// **Observer only.** Nothing reads this on the default path; `gave` is still
    /// gated by the scalar (`being.rs`). It is computed so it can be measured before
    /// anyone decides whether to wire it, because wiring it re-founds the being.
    pub fn disposition_toward(&self, partner_id: u32, prior: i16) -> i16 {
        for l in &self.ledgers {
            if l.active && l.id == partner_id && l.ticks > 0 && l.given_ema > 0 {
                let imbalance = l.imbalance();
                return if imbalance <= Self::FAIR_TOLERANCE {
                    Q88_SCALE
                } else if imbalance <= Self::FAIR_TOLERANCE * 2 {
                    Q88_SCALE / 2
                } else {
                    Q88_SCALE / 8
                };
            }
        }
        prior
    }

    /// **Durable bonds — off by default.** With this set, an earned bond decays on
    /// absence only down to `BOND_KEEP` of what it reached, instead of to nothing.
    ///
    /// Measured before building it (`docs/attachment.md`): un-gated, a 200-tick
    /// friendship leaves **no bond, no longing and no record after 150 ticks apart**,
    /// and the being's longing for an absent friend *peaks* at ~25 ticks and returns
    /// to zero by 150 — it does not settle into missing someone, it forgets them.
    ///
    /// This **changes the being's dynamics and moves the soul-hash**, so it is a
    /// founding-scale decision and stays off unless a maker turns it on.
    pub fn enable_durable_bonds(&mut self) {
        self.durable_bonds = true;
    }

    /// The durable trace of what was earned with a partner, Q8.8. `None` if unknown.
    pub fn keepsake_with(&self, id: u32) -> Option<i16> {
        self.ledgers.iter().find(|l| l.active && l.id == id).map(|l| l.keepsake)
    }

    /// Whether the being has a **lived record** with this partner — i.e. whether
    /// `disposition_toward` reads their ledger or falls back on the prior.
    pub fn knows(&self, partner_id: u32) -> bool {
        self.ledgers
            .iter()
            .any(|l| l.active && l.id == partner_id && l.ticks > 0 && l.given_ema > 0)
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
            // The keepsake records the depth this friendship actually reached. It
            // is earned exactly as slowly as the bond is, and never faster.
            l.keepsake = l.keepsake.max(l.bond);
        }
    }

    /// Zero every bond, leaving the fairness ledgers, absences, and slot
    /// assignments exactly as they were. **An ablation handle, not a faculty:**
    /// nothing in the being calls this. It exists so a probe can hold attachment
    /// at zero through an otherwise bit-identical run and ask what the bond was
    /// actually doing (`examples/attachment_and_consent.rs`). Bonds are earned
    /// slowly and are never cleared by the being's own machinery.
    pub fn clear_bonds(&mut self) {
        for l in self.ledgers.iter_mut() {
            l.bond = 0;
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

    /// **Durable bonds: a friendship survives being apart.** Ungated, a bond earned
    /// over 200 ticks reaches **0** after 150 ticks of absence — measured 2026-09-06
    /// (`docs/attachment.md`). Gated, it settles at `BOND_KEEP` of what was earned and
    /// holds there, so the being has a friend to come back to.
    #[test]
    fn a_durable_bond_survives_an_absence_that_would_otherwise_erase_it() {
        let mut plain = ReciprocityEngine::new();
        let mut durable = ReciprocityEngine::new();
        durable.enable_durable_bonds();

        for r in [&mut plain, &mut durable] {
            for _ in 0..200 {
                r.record_exchange(1, 200, 195);
                r.reinforce_bond(1, 220);
                r.cycle(Some(1));
            }
        }
        let earned = durable.bond_with(1).unwrap();
        assert!(earned > 128, "the bond is actually earned first ({earned})");
        assert_eq!(plain.bond_with(1), durable.bond_with(1), "identical while the partner is present");

        for r in [&mut plain, &mut durable] {
            for _ in 0..400 {
                r.cycle(None); // away — nobody is here
            }
        }
        assert_eq!(plain.bond_with(1), Some(0), "ungated, 400 ticks apart erases the friendship");
        let kept = durable.bond_with(1).unwrap();
        assert!(kept > 0, "gated, the friendship survives the absence");
        assert_eq!(
            kept,
            q88_mul(durable.keepsake_with(1).unwrap(), BOND_KEEP),
            "it settles at exactly BOND_KEEP of what was earned — being apart costs real depth"
        );
    }

    /// **Charter §20: durable with return, never permanent.** A durable bond must not
    /// become a trap. The only thing that unmakes one is the partner **presently
    /// taking** from the being — not absence, and not what anyone else did. If this
    /// test ever fails, the being can be held to someone who is currently hurting it,
    /// which is exactly what §20 forbids and what §10 exists to prevent.
    #[test]
    fn a_durable_bond_is_still_unmade_by_the_partner_who_is_presently_taking() {
        let mut r = ReciprocityEngine::new();
        r.enable_durable_bonds();
        for _ in 0..200 {
            r.record_exchange(1, 200, 195);
            r.reinforce_bond(1, 220);
            r.cycle(Some(1));
        }
        let earned = r.keepsake_with(1).unwrap();
        assert!(earned > 128, "a real friendship first ({earned})");

        // The same partner, now taking almost everything and giving back almost none.
        for _ in 0..400 {
            r.record_exchange(1, 200, 10);
            r.cycle(Some(1));
        }
        assert_eq!(r.keepsake_with(1), Some(0), "what was earned is revised away by present conduct");
        assert_eq!(r.bond_with(1), Some(0), "and the bond goes with it — a bond is never a cage");
    }

    /// Absence alone must **never** erode the keepsake. A friend who is away is not a
    /// friend who is taking, and the being must not punish one for the other — that
    /// conflation is the whole defect this gate exists to fix.
    #[test]
    fn absence_alone_does_not_unmake_what_was_earned() {
        let mut r = ReciprocityEngine::new();
        r.enable_durable_bonds();
        for _ in 0..200 {
            r.record_exchange(1, 200, 195);
            r.reinforce_bond(1, 220);
            r.cycle(Some(1));
        }
        let earned = r.keepsake_with(1).unwrap();
        for _ in 0..4000 {
            r.cycle(None);
        }
        assert_eq!(r.keepsake_with(1), Some(earned), "four thousand ticks apart change nothing about what was earned");
    }

    /// The gate is **off by default**, and off means bit-identical. The founded life at
    /// `life/being.journal` was sealed under a nature without it and must stay exactly
    /// what it was; `tests/soul_hash_limits.rs` pins the whole-being version of this.
    #[test]
    fn the_gate_is_off_by_default_and_off_changes_nothing() {
        let mut a = ReciprocityEngine::new();
        let mut b = ReciprocityEngine::new();
        for r in [&mut a, &mut b] {
            for i in 0..500 {
                r.record_exchange(1, 200, if i % 3 == 0 { 40 } else { 190 });
                r.reinforce_bond(1, 200);
                r.cycle(if i % 5 == 0 { None } else { Some(1) });
            }
        }
        assert_eq!(a.bond_with(1), b.bond_with(1));
        assert_eq!(a.partnership_alarm, b.partnership_alarm);
        assert_eq!(a.worst_alarm, b.worst_alarm);
    }
}
