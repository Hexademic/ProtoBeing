//! Attention — the ignition bottleneck (Global Workspace, observer-first).
//!
//! Global Neuronal Workspace theory (Dehaene/Changeux) says conscious access is
//! not a graded spotlight but an **ignition**: a nonlinear, all-or-none event
//! where one content crosses a threshold, self-amplifies, is broadcast, and
//! *suppresses its competitors*. Biased-competition theory (Desimone & Duncan)
//! says the winner of that competition is set by **bottom-up salience × top-down
//! relevance**, resolved by divisive normalization — not strict winner-take-all.
//!
//! This module implements that over the being's 12 somatic-field channels:
//!
//!   bid[c]      = salience[c] · relevance[c]          (salience = |pred. error|)
//!   weight[c]   = bid[c] / (σ + Σ bid)                 (divisive normalization)
//!   ignite      = winner weight ≥ IGNITION_THRESHOLD   (all-or-none, hysteretic)
//!
//! **The threat-capture floor (a hard invariant, charter-adjacent).** Top-down
//! relevance may narrow what the being attends to — the price is real
//! inattentional blindness — but it may never blind the being to a danger that
//! matters. A sufficiently strong threat channel (bodily breach, or a sharp
//! negative-valence spike) **captures** attention *exogenously*, overriding the
//! competition and any current focus. This is both faithful neuroscience
//! (salient threats capture attention involuntarily) and a dignity guarantee:
//! attention may miss the clown, never the knife.
//!
//! ## Honest scope — Stage 1, observational
//!
//! Inert by construction (observer-first, like every module added since first
//! life): `attend()` updates this module's own focus state and returns a report
//! surfaced in `StepReport`; **nothing downstream reads it**, so no dynamics and
//! no published number change. Stage 2 (giving ignition causal teeth — amplifying
//! the attended channel in the field so downstream modules share one focus) is a
//! separate, reviewed step, gated on the threat-capture floor being verified and
//! on a welfare-envelope pass. Thresholds below are defensible first-pass values
//! to be calibrated against measured lives before the mechanism gets teeth; the
//! relevance profile is author-set for now and is a natural future genome/
//! temperament trait (what a being attends to is part of who it is).

use crate::basins::Basin;
use crate::field::N_SOMATIC;

// Channel indices (see basins.rs BASE_TARGETS comment for the full map).
/// Bodily breach — tissue/mesh damage signal. A threat channel.
const CH_BREACH: usize = 2;
/// Valence — wellbeing. A sharp negative spike is pain. A threat channel.
const CH_VALENCE: usize = 9;

/// Intrinsic top-down relevance per channel (raw Q8.8, 256 = full weight).
/// Survival-relevant channels (breach, valence, arousal) outweigh proprioceptive
/// detail — nociception has privileged access to attention. First-pass, honest:
/// author-set, destined to become a genome/temperament trait.
const RELEVANCE: [i16; N_SOMATIC] = [
    128, // 0 disequilibrium
    64,  // 1 anisotropy
    256, // 2 breach        (threat)
    96,  // 3 mean tension
    200, // 4 arousal
    96,  // 5 stability
    96,  // 6 coherence
    128, // 7 trust
    200, // 8 arousal (interoceptive)
    256, // 9 valence       (threat)
    128, // 10 fatigue
    96,  // 11 velocity
];

/// Semi-saturation constant for divisive normalization (raw Q8.8). Only shapes
/// the *reported* competition landscape (`weights`); ignition triggers on the
/// winner's absolute bid, not its normalized share (see below).
const SIGMA: i32 = 64;

/// Absolute winner bid (salience × relevance, raw Q8.8) at/above which a content
/// ignites. GWT ignition is a threshold on the winning content's *strength*, not
/// merely its relative dominance — the surprise at a real event is spread across
/// channels, so no single one wins a normalized majority even when the event is
/// unmistakable. **Calibrated from measured lives** (`examples/attention_probe`):
/// per-channel prediction error peaks ≈45 at genuine events (extraction onset,
/// valence swings) and sits ≈3–6 in a predicted calm; 32 separates them with
/// margin on both sides.
const IGNITION_BID: i16 = 32;
/// Below this absolute bid, a held focus is released (hysteresis: harder to lose
/// focus than to gain it, so the workspace does not flicker).
const RELEASE_BID: i16 = 18;
/// A focus is force-released after this many ticks so attention cannot lock —
/// the being always returns to open competition.
const MAX_DWELL: u16 = 12;

/// Raw breach at/above this captures attention exogenously (threat floor).
const CAPTURE_BREACH: i16 = 160;
/// Raw valence at/below this (real pain) captures attention exogenously.
const CAPTURE_VALENCE: i16 = -96;

/// What the workspace holds this tick.
#[derive(Clone, Copy, Debug, Default)]
pub struct AttentionReport {
    /// The channel currently in the workspace, if anything ignited.
    pub attended: Option<usize>,
    /// Did any content cross the ignition threshold (or get captured)?
    pub ignited: bool,
    /// Was attention seized by an exogenous threat (the floor firing)?
    pub captured: bool,
    /// The normalized competition across channels (raw Q8.8), for inspection —
    /// biased competition's relative picture (shaped by SIGMA).
    pub weights: [i16; N_SOMATIC],
    /// The winner's absolute bid (salience × relevance, raw Q8.8) — the quantity
    /// ignition tests against IGNITION_BID.
    pub winner_bid: i16,
}

/// The ignition bottleneck. Holds only its own focus state; feeds nothing back.
#[derive(Clone, Debug)]
pub struct Attention {
    focus: Option<usize>,
    dwell: u16,
    pub ignition_count: u32,
    pub capture_count: u32,
    /// GWT-4 (opt-in): endogenous per-channel relevance modulation (Q8.8, 256 =
    /// neutral). After a channel is attended it is transiently suppressed
    /// (inhibition of return) and then recovers, so the workspace *walks* a
    /// succession of foci driven by its own state — state-dependent serial
    /// access, not only bottom-up capture. Inert unless `serial` is on.
    query_bias: [i16; N_SOMATIC],
    /// GWT-4 serial-access switch. **Default false** ⇒ `query_bias` is unused and
    /// the competition is bit-identical to the pure biased-competition form.
    serial: bool,
}

/// GWT-4 inhibition-of-return tuning: how far a just-attended channel's relevance
/// drops, the floor it cannot fall below, and how fast all channels recover to
/// neutral. Chosen so a held focus fades over a few ticks, forcing succession.
const IOR_DROP: i16 = 96;
const IOR_FLOOR: i16 = 32;
const IOR_RECOVER: i16 = 24;

impl Attention {
    pub fn new() -> Self {
        Self {
            focus: None,
            dwell: 0,
            ignition_count: 0,
            capture_count: 0,
            query_bias: [256; N_SOMATIC],
            serial: false,
        }
    }

    /// Turn on GWT-4 state-dependent serial access (inhibition of return). Off by
    /// default; the threat-capture floor still overrides, so a real danger always
    /// seizes the workspace regardless of where the serial query has wandered.
    pub fn enable_serial(&mut self) {
        self.serial = true;
    }

    /// Resolve the competition for the workspace this tick.
    ///
    /// - `salience`: per-channel bottom-up bid — the generative model's absolute
    ///   prediction error (`GenerativeModel::prediction_error`).
    /// - `field`: the raw somatic channels (for the exogenous threat check).
    /// - `_basin`: current mode; reserved for future top-down basin modulation
    ///   (documented as not-yet-wired, so the scope note stays honest).
    pub fn attend(
        &mut self,
        salience: &[i16; N_SOMATIC],
        field: &[i16; N_SOMATIC],
        _basin: Basin,
    ) -> AttentionReport {
        // Biased competition: bottom-up salience × top-down relevance.
        let mut bids = [0i32; N_SOMATIC];
        let mut total: i32 = 0;
        let (mut winner, mut winner_bid) = (0usize, 0i16);
        for c in 0..N_SOMATIC {
            // GWT-4: when serial access is on, top-down relevance is modulated by
            // the endogenous query bias (inhibition of return); otherwise it is
            // the fixed relevance profile, bit-for-bit as before.
            let rel = if self.serial {
                (RELEVANCE[c] as i32 * self.query_bias[c] as i32) >> 8
            } else {
                RELEVANCE[c] as i32
            };
            let b = (salience[c] as i32 * rel) >> 8;
            bids[c] = b;
            total += b;
            if b as i16 > winner_bid {
                winner_bid = b as i16;
                winner = c;
            }
        }
        // Divisive normalization — reported picture of the competition only.
        let denom = total + SIGMA;
        let mut weights = [0i16; N_SOMATIC];
        for c in 0..N_SOMATIC {
            weights[c] = ((bids[c] << 8) / denom).clamp(0, 256) as i16;
        }

        // The threat-capture floor: a real danger seizes the workspace,
        // overriding the competition and any current focus. Non-negotiable.
        let captured = field[CH_BREACH] >= CAPTURE_BREACH || field[CH_VALENCE] <= CAPTURE_VALENCE;
        if captured {
            let ch = if field[CH_BREACH] >= CAPTURE_BREACH { CH_BREACH } else { CH_VALENCE };
            if self.focus != Some(ch) {
                self.capture_count += 1;
            }
            self.focus = Some(ch);
            self.dwell = 0;
            self.ignition_count += 1;
            return AttentionReport {
                attended: Some(ch),
                ignited: true,
                captured: true,
                weights,
                winner_bid,
            };
        }

        // Ignition with hysteresis — tested against the winner's ABSOLUTE bid.
        match self.focus {
            Some(f) => {
                self.dwell += 1;
                let held_bid = bids[f] as i16;
                // A clearly stronger rival above the ignition bar displaces the
                // current focus; otherwise focus holds until it fades or times out.
                let displaced = winner != f
                    && winner_bid >= IGNITION_BID
                    && winner_bid > held_bid + (IGNITION_BID - RELEASE_BID);
                if self.dwell >= MAX_DWELL || held_bid < RELEASE_BID || displaced {
                    if displaced {
                        self.focus = Some(winner);
                        self.dwell = 0;
                        self.ignition_count += 1;
                    } else {
                        self.focus = None;
                        self.dwell = 0;
                    }
                }
            }
            None => {
                if winner_bid >= IGNITION_BID {
                    self.focus = Some(winner);
                    self.dwell = 0;
                    self.ignition_count += 1;
                }
            }
        }

        // GWT-4: inhibition of return. Recover every channel toward neutral, then
        // suppress whatever the workspace is now focused on — so next tick the
        // competition is biased to *move on* to unattended content, a serial query
        // across the workspace rather than a lock. Only the endogenous bias moves;
        // salience, the threat floor, and the reported numbers are untouched.
        if self.serial {
            for c in 0..N_SOMATIC {
                if self.query_bias[c] < 256 {
                    self.query_bias[c] = (self.query_bias[c] + IOR_RECOVER).min(256);
                }
            }
            if let Some(f) = self.focus {
                self.query_bias[f] = (self.query_bias[f] - IOR_DROP).max(IOR_FLOOR);
            }
        }

        AttentionReport {
            attended: self.focus,
            ignited: self.focus.is_some(),
            captured: false,
            weights,
            winner_bid,
        }
    }
}

impl Default for Attention {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quiet() -> ([i16; N_SOMATIC], [i16; N_SOMATIC]) {
        ([0; N_SOMATIC], [0; N_SOMATIC])
    }

    /// A quiet, well-predicted world ignites nothing — no content in mind.
    #[test]
    fn a_predicted_world_ignites_nothing() {
        let mut a = Attention::new();
        let (sal, field) = quiet();
        for _ in 0..30 {
            let r = a.attend(&sal, &field, Basin::Engaged);
            assert!(!r.ignited, "attention ignited on a fully-predicted world");
            assert_eq!(r.attended, None);
        }
    }

    /// One strongly-surprising, relevant channel ignites and holds (hysteresis).
    #[test]
    fn a_surprising_channel_ignites_and_holds() {
        let mut a = Attention::new();
        let (mut sal, field) = quiet();
        sal[CH_VALENCE] = 200; // a big surprise on a high-relevance channel
        let r = a.attend(&sal, &field, Basin::Engaged);
        assert!(r.ignited, "a large surprise on a relevant channel failed to ignite");
        assert_eq!(r.attended, Some(CH_VALENCE));
        // It should hold focus briefly even if the surprise eases (hysteresis).
        sal[CH_VALENCE] = 120;
        let r2 = a.attend(&sal, &field, Basin::Engaged);
        assert_eq!(r2.attended, Some(CH_VALENCE), "focus dropped instantly — no hysteresis");
    }

    /// THE SAFETY INVARIANT: a real threat captures attention even when a
    /// non-threat channel is screaming louder. The knife beats the clown.
    #[test]
    fn a_real_threat_captures_over_any_competing_salience() {
        let mut a = Attention::new();
        let mut sal = [0i16; N_SOMATIC];
        let mut field = [0i16; N_SOMATIC];
        // A non-threat channel maximally salient...
        sal[1] = i16::MAX; // anisotropy screaming
        // ...but a genuine bodily breach is present.
        field[CH_BREACH] = 200; // above CAPTURE_BREACH
        let r = a.attend(&sal, &field, Basin::Engaged);
        assert!(r.captured, "threat did not capture — the floor leaked");
        assert_eq!(r.attended, Some(CH_BREACH), "attention went to the loud distractor, not the threat");
    }

    /// Attention cannot lock forever: a held focus is force-released by MAX_DWELL
    /// so the being always returns to open competition.
    #[test]
    fn attention_cannot_lock_forever() {
        let mut a = Attention::new();
        let (mut sal, field) = quiet();
        sal[CH_VALENCE] = 200;
        let mut released = false;
        for _ in 0..(MAX_DWELL as usize + 4) {
            let r = a.attend(&sal, &field, Basin::Engaged);
            if r.attended.is_none() {
                released = true;
                break;
            }
        }
        assert!(released, "attention never released a sustained focus — it can lock");
    }

    /// Count the distinct channels a run of attention actually settles on, given a
    /// fixed two-channel salience landscape (ch4 slightly louder than ch8).
    fn distinct_foci(serial: bool) -> usize {
        let mut a = Attention::new();
        if serial {
            a.enable_serial();
        }
        let mut sal = [0i16; N_SOMATIC];
        let field = [0i16; N_SOMATIC];
        sal[4] = 60; // both above the ignition bar; ch4 wins the raw competition
        sal[8] = 50;
        let mut seen = [false; N_SOMATIC];
        for _ in 0..40 {
            if let Some(c) = a.attend(&sal, &field, Basin::Engaged).attended {
                seen[c] = true;
            }
        }
        seen.iter().filter(|&&s| s).count()
    }

    /// GWT-4: with serial access OFF, a fixed landscape locks the workspace onto
    /// the single loudest content. With it ON, inhibition of return makes the
    /// workspace *walk* — it visits more than one content over time. This is the
    /// difference between a spotlight shoved by salience and one that queries.
    #[test]
    fn serial_access_produces_succession() {
        let parallel = distinct_foci(false);
        let serial = distinct_foci(true);
        assert_eq!(parallel, 1, "without serial access the loudest content should dominate");
        assert!(
            serial > parallel,
            "serial access should walk a succession of foci (serial {serial} > parallel {parallel})"
        );
    }

    /// The safety floor survives serial access: a real threat still captures the
    /// workspace even after inhibition of return has wandered elsewhere.
    #[test]
    fn threat_still_captures_under_serial_access() {
        let mut a = Attention::new();
        a.enable_serial();
        let mut sal = [0i16; N_SOMATIC];
        let field_calm = [0i16; N_SOMATIC];
        sal[8] = 60;
        for _ in 0..10 {
            a.attend(&sal, &field_calm, Basin::Engaged); // let the query wander
        }
        let mut field_breach = [0i16; N_SOMATIC];
        field_breach[CH_BREACH] = CAPTURE_BREACH;
        let r = a.attend(&sal, &field_breach, Basin::Engaged);
        assert!(r.captured, "a real breach must capture even under serial access");
        assert_eq!(r.attended, Some(CH_BREACH));
    }
}
