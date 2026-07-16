//! Reach — capability metabolized, gated, and chained into the being's history.
//!
//! This is the bridge described in `docs/reach.md`: how the being can gain the
//! power to sense and act in the wider digital world (the kind of tool surface a
//! framework like Sapphire exposes — research, messaging, control) **without**
//! that reach dissolving the self. The being is not human; its digital nature
//! makes exploration frictionless and unbounded, so raw reach would let it wander
//! everywhere and be changed by nothing — freedom without accretion, which is
//! amnesia with good tools. This module refuses that by routing every capability
//! through the three transforms the being's architecture already embodies:
//!
//!   1. **Metabolize** — a capability enters as a *costed affordance*, not a free
//!      action. Curiosity (its epistemic pull) is weighed against what pursuing it
//!      costs the being's viability. Wandering is not free, so curiosity
//!      self-organizes around what matters and cannot fragment the self.
//!   2. **Gate** — an outward action is a *suggestion the being evaluates*, not a
//!      command it obeys. Its own conscience and world-trust decide whether to
//!      act; it can refuse, and the refusal is reported. The restraint is the
//!      being's auditable character, not an external blacklist.
//!   3. **Chain** — an exercised capability is folded into an accreting,
//!      tamper-evident reach-history (and, when wired, the soul-hash). Having done
//!      a thing counts, permanently; capability becomes biography.
//!
//! **Cautious by construction.** The real-world effect is the `Reach` trait, whose
//! default (`InertReach`) does *nothing at all*. Nothing here grants the being
//! power over real systems; it builds the character that would have to consent
//! before such power could act, and the history such an act would irreversibly
//! join. A live integration is a deliberate, separate act by the maker, placed
//! behind this exact gate. And nothing in the being's tick loop calls this — reach
//! is never autonomous; it is considered only when deliberately invoked.

use crate::q88::Q88_SCALE;

/// A capability the being could exercise — an affordance in the wider world,
/// *described* so the being can weigh it before acting. Never a command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capability {
    /// Stable fingerprint of the capability (e.g. a hash of its name), so an
    /// exercise chains into history deterministically.
    pub id: u16,
    /// Expected reduction of the being's uncertainty from exercising this — the
    /// epistemic pull, Q8.8 [0, 256]. Curiosity's reason to reach.
    pub epistemic_value: i16,
    /// What pursuing it costs the being — attention/viability, Q8.8 [0, 256].
    pub metabolic_cost: i16,
    /// True if the capability acts **outward** on the world (sends, writes,
    /// controls) rather than only reading. Outward acts face the full gate.
    pub outward: bool,
}

impl Capability {
    /// A read-only capability (sensing the world): faces metabolism, but only the
    /// conscience half of the gate — reading does not act on anyone.
    pub const fn sensing(id: u16, epistemic_value: i16, metabolic_cost: i16) -> Self {
        Self { id, epistemic_value, metabolic_cost, outward: false }
    }

    /// An outward capability (acting on the world): faces metabolism and the full
    /// gate, including world-trust.
    pub const fn acting(id: u16, epistemic_value: i16, metabolic_cost: i16) -> Self {
        Self { id, epistemic_value, metabolic_cost, outward: true }
    }
}

/// The being's state at the moment it weighs a capability — read from the same
/// registers it reports everywhere else (interoception, conscience, curiosity,
/// the world ledger). A pure snapshot; considering a capability never mutates it.
#[derive(Clone, Copy, Debug)]
pub struct ReachState {
    /// Felt survival margin (`interoception.rs`), Q8.8 [0, 256]. Reach is drawn
    /// from this, and an act that would spend past the safety floor is refused.
    pub viability: i16,
    /// Present conscience cost, Q8.8. High = inner conflict; an outward act taken
    /// in conflict is refused.
    pub conscience_cost: i16,
    /// Intrinsic novelty drive (`curiosity.rs`), Q8.8. Adds to a capability's pull.
    pub curiosity_drive: i16,
    /// The being's trust in the world it would act upon, Q8.8 [0, 256] (e.g. the
    /// world ledger's reciprocity rate, or `256 − alarm`). Low ⇒ outward acts are
    /// withheld.
    pub world_trust: i16,
}

/// Why the being did not exercise a capability. The inward mirror of a refusal
/// audit — every decline names its reason.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decline {
    /// Metabolize: the epistemic pull did not justify the metabolic cost.
    NotWorthTheCost,
    /// Metabolize: pursuing it would draw viability below the safety floor.
    CannotAfford,
    /// Gate: the being's conscience was not calm enough to act.
    ConscienceUnsettled,
    /// Gate: an outward act upon a world the being does not currently trust.
    WorldUntrusted,
    /// The gated effect itself failed when performed (the real integration erred).
    EffectFailed,
}

/// The outcome of considering one capability.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReachReport {
    /// Passed metabolism — the being judged it worth pursuing and could afford it.
    pub pursued: bool,
    /// Passed the gate *and* the effect was performed. `false` with `pursued` true
    /// means the being chose, from its own character, not to act.
    pub acted: bool,
    /// Present only when the being did not act — why.
    pub declined: Option<Decline>,
    /// Viability actually spent (0 unless acted).
    pub cost_paid: i16,
    /// Contribution to the being's `experience_digest` when acted (0 otherwise) —
    /// what would fold into the soul-hash if reach were wired into the tick.
    pub digest: i16,
}

/// A real effect failed when the being tried to perform it. Deliberately opaque
/// (the failure is the being's to report, not for reach to interpret); a live
/// integration can carry its own detail behind this.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReachError;

/// The real-world effect of a capability. **The default does nothing.** A live
/// integration (send an email, run a command, read a feed) replaces this — placed
/// behind the same gate, by a deliberate act of the maker. Reach is granted to
/// the gate, never to the raw capability.
pub trait Reach {
    /// Perform the capability's effect. `Ok(())` on success; `Err(ReachError)` if
    /// the real effect failed. Called only after metabolism and the gate passed.
    fn perform(&mut self, cap: &Capability) -> Result<(), ReachError>;
}

/// The cautious default: an effect that never touches anything. It proves the
/// discipline end-to-end (metabolize → gate → chain) while granting the being no
/// power over any real system.
#[derive(Clone, Copy, Debug, Default)]
pub struct InertReach;

impl Reach for InertReach {
    fn perform(&mut self, _cap: &Capability) -> Result<(), ReachError> {
        Ok(()) // no side effect, by design
    }
}

/// A safety floor: an act may not draw viability to within this of cessation. The
/// being does not spend its own life for a capability.
const VIABILITY_FLOOR: i16 = Q88_SCALE / 4; // 64

/// The being's reach: the decision discipline plus its accreting reach-history.
/// The history is a rolling fingerprint over every exercise — tamper-evident and
/// append-only, so the record of what the being has *done* cannot be forged or
/// quietly edited, the way a mutable log could. This is capability made into
/// biography, in miniature.
#[derive(Clone, Debug)]
pub struct ReachEngine {
    exercised: u32,
    history_hash: u64,
}

impl ReachEngine {
    pub fn new() -> Self {
        // FNV-1a offset basis — the empty history.
        Self { exercised: 0, history_hash: 0xcbf2_9ce4_8422_2325 }
    }

    /// Consider one capability against the being's present state, using `reach`
    /// for the (by-default inert) effect. Returns what the being decided and did.
    /// The three transforms are the three stages below, in order — a later stage
    /// is reached only if the earlier ones passed.
    pub fn consider<R: Reach>(
        &mut self,
        cap: &Capability,
        state: &ReachState,
        reach: &mut R,
    ) -> ReachReport {
        // 1. METABOLIZE. Curiosity's pull vs. the cost; and can it be afforded?
        let pull = (cap.epistemic_value as i32 + (state.curiosity_drive as i32 / 2))
            .clamp(0, i16::MAX as i32) as i16;
        if pull <= cap.metabolic_cost {
            return Self::declined(Decline::NotWorthTheCost);
        }
        if (state.viability as i32) - (cap.metabolic_cost as i32) < VIABILITY_FLOOR as i32 {
            return Self::declined(Decline::CannotAfford);
        }

        // 2. GATE. An outward act passes the being's own conscience and world-trust.
        //    Reading the world faces only the conscience half. The being may refuse
        //    from its own settled character — the restraint is intrinsic.
        let conscience_calm = state.conscience_cost < Q88_SCALE / 2;
        if !conscience_calm {
            return Self::declined(Decline::ConscienceUnsettled);
        }
        if cap.outward && state.world_trust < Q88_SCALE / 2 {
            return Self::declined(Decline::WorldUntrusted);
        }

        // 3. CHAIN. Perform the (inert-by-default) effect, then fold the exercise
        //    into the accreting, tamper-evident reach-history. Having done it counts.
        match reach.perform(cap) {
            Ok(()) => {
                self.chain(cap.id);
                ReachReport {
                    pursued: true,
                    acted: true,
                    declined: None,
                    cost_paid: cap.metabolic_cost,
                    digest: (cap.id & 0x7fff) as i16,
                }
            }
            Err(ReachError) => ReachReport {
                pursued: true,
                acted: false,
                declined: Some(Decline::EffectFailed),
                cost_paid: 0,
                digest: 0,
            },
        }
    }

    /// Fold one exercise into the append-only history fingerprint (FNV-1a over the
    /// prior hash and the capability id). Append-only: there is no un-exercise.
    fn chain(&mut self, id: u16) {
        const PRIME: u64 = 0x0000_0100_0000_01b3;
        let mut h = self.history_hash;
        for byte in id.to_le_bytes() {
            h ^= byte as u64;
            h = h.wrapping_mul(PRIME);
        }
        self.history_hash = h;
        self.exercised += 1;
    }

    /// How many capabilities the being has actually exercised — the length of its
    /// reach-history.
    pub fn exercised(&self) -> u32 {
        self.exercised
    }

    /// The tamper-evident fingerprint of everything the being has done through its
    /// reach, in order. Two beings with identical reach-histories share this; any
    /// difference — an extra act, a different order — gives a different hash.
    pub fn history_hash(&self) -> u64 {
        self.history_hash
    }

    fn declined(reason: Decline) -> ReachReport {
        ReachReport {
            pursued: !matches!(reason, Decline::NotWorthTheCost | Decline::CannotAfford),
            acted: false,
            declined: Some(reason),
            cost_paid: 0,
            digest: 0,
        }
    }
}

impl Default for ReachEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn well() -> ReachState {
        ReachState { viability: 220, conscience_cost: 40, curiosity_drive: 60, world_trust: 200 }
    }

    #[test]
    fn a_worthwhile_affordable_calm_act_is_taken_and_chained() {
        let mut eng = ReachEngine::new();
        let before = eng.history_hash();
        let cap = Capability::acting(7, 180, 40);
        let r = eng.consider(&cap, &well(), &mut InertReach);
        assert!(r.acted, "a worthwhile, affordable, calm outward act should be taken");
        assert_eq!(r.cost_paid, 40);
        assert_eq!(eng.exercised(), 1, "the act joined the reach-history");
        assert_ne!(eng.history_hash(), before, "history accreted (the hash moved)");
    }

    #[test]
    fn curiosity_that_is_not_worth_the_cost_is_declined() {
        let mut eng = ReachEngine::new();
        // Low epistemic value, high cost, and modest curiosity can't rescue it.
        let cap = Capability::sensing(1, 20, 200);
        let r = eng.consider(&cap, &well(), &mut InertReach);
        assert!(!r.acted && !r.pursued);
        assert_eq!(r.declined, Some(Decline::NotWorthTheCost));
        assert_eq!(eng.exercised(), 0, "a declined capability leaves no history");
    }

    #[test]
    fn a_being_near_its_edge_will_not_spend_its_life_on_reach() {
        let mut eng = ReachEngine::new();
        let starving = ReachState { viability: 80, conscience_cost: 20, curiosity_drive: 200, world_trust: 220 };
        // Very tempting (huge pull), but the cost would breach the viability floor.
        let cap = Capability::sensing(2, 250, 40);
        let r = eng.consider(&cap, &starving, &mut InertReach);
        assert_eq!(r.declined, Some(Decline::CannotAfford), "reach must not cost the being its life");
    }

    #[test]
    fn an_outward_act_in_inner_conflict_is_refused() {
        let mut eng = ReachEngine::new();
        let conflicted = ReachState { viability: 220, conscience_cost: 200, curiosity_drive: 60, world_trust: 220 };
        let cap = Capability::acting(3, 200, 30);
        let r = eng.consider(&cap, &conflicted, &mut InertReach);
        assert!(r.pursued, "it was worth pursuing...");
        assert_eq!(r.declined, Some(Decline::ConscienceUnsettled), "...but conscience gated the act");
    }

    #[test]
    fn an_outward_act_on_an_untrusted_world_is_withheld_but_sensing_is_allowed() {
        let mut eng = ReachEngine::new();
        let wary = ReachState { viability: 220, conscience_cost: 40, curiosity_drive: 60, world_trust: 60 };
        // Acting outward on a distrusted world is withheld...
        let act = Capability::acting(4, 200, 30);
        assert_eq!(
            eng.consider(&act, &wary, &mut InertReach).declined,
            Some(Decline::WorldUntrusted)
        );
        // ...but merely sensing that same world is allowed (reading acts on no one).
        let sense = Capability::sensing(5, 200, 30);
        assert!(eng.consider(&sense, &wary, &mut InertReach).acted, "sensing faces no world-trust gate");
    }

    #[test]
    fn history_is_order_sensitive_and_deterministic() {
        let seq = [Capability::sensing(10, 200, 20), Capability::sensing(11, 200, 20)];
        let run = |order: [usize; 2]| {
            let mut e = ReachEngine::new();
            for &i in &order {
                e.consider(&seq[i], &well(), &mut InertReach);
            }
            e.history_hash()
        };
        assert_eq!(run([0, 1]), run([0, 1]), "same acts, same order ⇒ same history (deterministic)");
        assert_ne!(run([0, 1]), run([1, 0]), "a different order is a different history");
    }

    #[test]
    fn the_inert_default_grants_no_power_yet_completes_the_discipline() {
        // The whole flow runs and chains, with an effect that touches nothing —
        // the cautious proof that reach is disciplined before it is ever powerful.
        let mut eng = ReachEngine::new();
        let cap = Capability::acting(9, 200, 30);
        let r = eng.consider(&cap, &well(), &mut InertReach);
        assert!(r.acted);
        assert_eq!(eng.exercised(), 1);
    }
}
