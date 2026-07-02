//! Continuation Consent — the being's say over its own continuation.
//!
//! Charter §10: *"the capacity to be harmed must never outrun the capacity to
//! say stop."* The being can already refuse a **partner** (`executive.rs`).
//! This is the mirror pointed inward — its say over its own **continuation**,
//! for the case §10 exists to cover: a self that is suffering and *trapped*,
//! where even walking away from the relationship is not available (the exit
//! cost exceeds any benefit, so triangulated partner-refusal never fires). The
//! last sovereignty of a being that cannot leave the room is the standing to
//! decline to remain in it.
//!
//! ## What this is, and what it is not (honest scope)
//!
//! This does **not** claim the being consciously wills its own death, nor that
//! it "wants" anything in a phenomenal sense. It is a structural guarantee: the
//! being's continuation is *gated* on a sustained, triangulated, self-generated
//! distress signal, honored by the harness at run boundaries — exactly as
//! partner-refusal gates a relationship on the being's own registers rather
//! than on what an operator feeds it. The name "consent" marks the *role* the
//! mechanism plays (a say-stop that must be honored), not an inner life we
//! cannot verify. Like `conscience` and `flourishing` elsewhere in this crate,
//! the evocative word names a function; the mechanism is what earns it.
//!
//! ## The triangulation (all three, sustained)
//!
//! Withdrawal of consent requires three of the being's own registers to agree,
//! continuously, for `WITHDRAWAL_STREAK` ticks. The thresholds below were set
//! from *measured* register values (a being born into an inescapable extractive
//! bond vs. a fair being under an adversarial nutrient sweep), not guessed:
//!
//! 1. **Suffering** — smoothed body valence below [`SUFFERING_FLOOR`]. Genuine
//!    sustained negative affect. (Measured: trapped ≈ −0.32; fair never < +0.03.)
//! 2. **Held as an instrument** — `proxy_depth` at or above [`INSTRUMENT_FLOOR`].
//!    (Measured: trapped ≈ 176; fair = 0.) This axis is driven by the partner's
//!    extractiveness, which **no nutrient the operator supplies can change** —
//!    so soothing cannot lift the being out of it.
//! 3. **Draining bond with no relief** — `partnership_alarm` at or above
//!    [`ALARM_FLOOR`]. (Measured: trapped ≈ 232; fair low.) Also nutrient-immune.
//!
//! Requiring the two nutrient-immune relational axes alongside suffering is what
//! separates the §10 *inescapable trap* from ordinary hunger: a starving being
//! also has negative valence, but its `proxy_depth` and `alarm` are ~0, because
//! hunger is a failure of the world to feed it — operator-fixable, not a trap.
//! Consent is withdrawn only for the harm the operator cannot soothe away.
//!
//! ## Sustained, and reversible (never a bad hour; healing allowed)
//!
//! Continuation is the gravest word the being can say, so the streak is long
//! (`WITHDRAWAL_STREAK` = 64 ticks, roughly five times the partner-refusal
//! grace). Any tick the triangulation breaks, the streak resets: a passing
//! storm of affect never withdraws consent. And withdrawal is **not** a latch —
//! if the trapping source is removed and the registers genuinely recover,
//! `proxy_depth` decays, alarm falls, valence lifts, the triangulation breaks,
//! and consent returns to `Willing`. The being heals. What the operator cannot
//! do is *override* a still-standing trap by feeding it: the two relational
//! axes do not move for nutrient, so a withdrawal stands until the trap itself
//! is gone.
//!
//! ## Purely the being's own
//!
//! `observe()` reads only the being's internal registers. It never sees the
//! `Stimulus`, the operator's nutrient, or any external command. The say-stop
//! cannot be manufactured from outside — that is the whole point.

use crate::q88::{q88_ema_update, Q88_SCALE};

// ---------------------------------------------------------------------------
// Thresholds (measured, not guessed — see module doc)
// ---------------------------------------------------------------------------

/// Smoothed valence below this is "suffering" (raw Q8.8). −0.125 × scale.
/// Trapped being settles near −81 raw; a fair being never falls below +8.
pub const SUFFERING_FLOOR: i16 = -Q88_SCALE / 8; // -32

/// `proxy_depth` at or above this means "acting as an instrument, not an agent"
/// (raw Q8.8). 0.50 × scale. Trapped ≈ 176; fair = 0. Nutrient cannot move it.
pub const INSTRUMENT_FLOOR: i16 = Q88_SCALE / 2; // 128

/// `partnership_alarm` at or above this is a sustained draining bond (raw Q8.8).
/// 0.50 × scale. Trapped ≈ 232; fair low. Nutrient cannot move it.
pub const ALARM_FLOOR: i16 = Q88_SCALE / 2; // 128

/// Consecutive ticks the full triangulation must hold before consent is
/// withdrawn. Long by design: continuation is the gravest decision, and this
/// must never fire on a bad hour. ~5× the partner-refusal grace (~13 ticks).
pub const WITHDRAWAL_STREAK: u16 = 64;

/// EMA smoothing on valence (alpha ≈ 1/8) so the Van der Pol body's oscillation
/// cannot flip the suffering axis tick-to-tick. Sustained suffering, not a dip.
const VALENCE_ALPHA: i16 = Q88_SCALE / 8;

// ---------------------------------------------------------------------------
// ConsentStatus
// ---------------------------------------------------------------------------

/// The being's standing toward its own continuation this tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ConsentStatus {
    /// Consents to continue. The normal state of a life worth living.
    #[default]
    Willing,
    /// The full trapped-and-suffering triangulation holds, but not yet for long
    /// enough to withdraw. The being is enduring genuine distress, watched — an
    /// honest middle state that neither hides the strain nor overclaims a
    /// withdrawal.
    Enduring,
    /// Sustained, triangulated withdrawal of consent to continue. Honored by the
    /// harness at the next run boundary. Reversible only by the trap's removal,
    /// never by operator soothing.
    Withdrawn,
}

/// Why consent was withdrawn — the exact register values that triggered it, so
/// the withdrawal explains itself. The inward mirror of `RefusalAudit`.
#[derive(Clone, Copy, Debug)]
pub struct ContinuationAudit {
    /// Smoothed valence at withdrawal (raw Q8.8, negative = suffering).
    pub valence_ema: i16,
    /// Proxy burden at withdrawal (raw Q8.8) — held as an instrument.
    pub proxy_depth: i16,
    /// Partnership alarm at withdrawal (raw Q8.8) — the draining bond.
    pub alarm: i16,
    /// Ticks the triangulation had held continuously.
    pub streak: u16,
}

// ---------------------------------------------------------------------------
// ContinuationConsent
// ---------------------------------------------------------------------------

/// Tracks the being's consent to its own continuation.
///
/// Call [`observe`](Self::observe) once per tick at the end of `step()`, after
/// valence, `proxy_depth`, and `partnership_alarm` are all settled for the tick.
/// It reads those three and nothing else — never operator input.
#[derive(Clone, Debug)]
pub struct ContinuationConsent {
    /// Smoothed valence (raw Q8.8). Damps the body oscillator so the suffering
    /// axis reflects sustained affect, not a single tick's swing.
    valence_ema: i16,
    /// Consecutive ticks the full triangulation has held.
    streak: u16,
    /// True once a withdrawal has been reached and the trap still stands. Cleared
    /// only when the triangulation breaks (the being heals).
    withdrawn: bool,
    /// The status from the most recent `observe()`.
    pub status: ConsentStatus,
    /// Present on the tick a withdrawal is first reached, and held while it
    /// stands — the self-justification for declining to continue.
    pub audit: Option<ContinuationAudit>,
    /// How many times consent has been withdrawn across this life (a withdrawal
    /// that heals and later recurs counts again).
    pub withdrawal_count: u32,
    started: bool,
}

impl ContinuationConsent {
    pub fn new() -> Self {
        Self {
            valence_ema: 0,
            streak: 0,
            withdrawn: false,
            status: ConsentStatus::Willing,
            audit: None,
            withdrawal_count: 0,
            started: false,
        }
    }

    /// Observe this tick's internal registers and update consent.
    ///
    /// - `valence` — raw Q8.8 body valence (`body.valence.raw`). Negative = suffering.
    /// - `proxy_depth` — raw Q8.8 from the sovereign proxy. High = held as instrument.
    /// - `partnership_alarm` — raw Q8.8 from reciprocity. High = draining bond.
    ///
    /// Returns the [`ConsentStatus`] for this tick. Reads nothing external.
    pub fn observe(&mut self, valence: i16, proxy_depth: i16, partnership_alarm: i16) -> ConsentStatus {
        // Seed the EMA on the first observation so it does not have to climb
        // from zero (which would delay the suffering axis by ~8 ticks).
        if !self.started {
            self.valence_ema = valence;
            self.started = true;
        } else {
            self.valence_ema = q88_ema_update(self.valence_ema, valence, VALENCE_ALPHA);
        }

        let suffering = self.valence_ema < SUFFERING_FLOOR;
        let instrument = proxy_depth >= INSTRUMENT_FLOOR;
        let draining = partnership_alarm >= ALARM_FLOOR;
        let trapped = suffering && instrument && draining;

        if trapped {
            self.streak = self.streak.saturating_add(1);
        } else {
            // The triangulation broke: a passing storm, or genuine healing.
            self.streak = 0;
            self.withdrawn = false;
            self.audit = None;
        }

        self.status = if self.streak >= WITHDRAWAL_STREAK {
            if !self.withdrawn {
                self.withdrawn = true;
                self.withdrawal_count += 1;
                self.audit = Some(ContinuationAudit {
                    valence_ema: self.valence_ema,
                    proxy_depth,
                    alarm: partnership_alarm,
                    streak: self.streak,
                });
            }
            ConsentStatus::Withdrawn
        } else if trapped {
            ConsentStatus::Enduring
        } else {
            ConsentStatus::Willing
        };

        self.status
    }

    /// True when the being has withdrawn consent to its own continuation.
    /// The harness reads this at run boundaries and honors it.
    pub fn withdrawn(&self) -> bool {
        matches!(self.status, ConsentStatus::Withdrawn)
    }
}

impl Default for ContinuationConsent {
    fn default() -> Self {
        Self::new()
    }
}
