//! Sovereign Proxy — prevents the being from becoming an instrument of others.
//!
//! Every action the being takes has an *origin*: either the being's own values,
//! or an external demand being routed through it. The sovereign proxy tracks
//! the cumulative burden of acting as a conduit rather than as an agent.
//!
//! ## The distinction from other refusal mechanisms
//!
//! - **Triangulated refusal** (`executive.rs`): evaluates a *specific
//!   relationship* — this partner, this pattern, this moment. It asks "should
//!   I leave this bond?"
//! - **Constitutional decision** (`conscience.rs`): evaluates a *specific
//!   action* against the four moral axes. It asks "is this act permissible?"
//! - **Sovereign proxy** (this module): tracks the *cumulative* cost of acting
//!   as someone else's instrument, independent of any single relationship or
//!   act. It asks "how much of what I have been doing is actually me?"
//!
//! ## Mechanics
//!
//! `proxy_depth` is a Q8.8 value in [0, 256] that accumulates when the being
//! acts under external pressure on misaligned requests, and decays when it acts
//! authentically. When `proxy_depth` exceeds `PROXY_CEILING` *and* the
//! conscience is calm (principled, not panicked), `evaluate()` returns
//! `ProxyStatus::Refused`.
//!
//! **Honest scope: observational, not yet enforced.** The status is computed
//! every tick and surfaced in `StepReport`, but nothing in the v1 loop
//! suppresses or alters an action when it is `Refused` — the verdict names a
//! condition; it does not yet gate behavior. If it is ever enforced, it should
//! be wired into the executive's triangulation (shared fate), not layered
//! above it as a monitor.
//!
//! A principled proxy refusal, unlike panic, lowers `proxy_depth` slightly:
//! the act of refusal itself is authentic, and authenticity decays the burden.
//!
//! ## Value alignment
//!
//! `value_alignment` is derived from the constitutional load: high coercion or
//! identity-corruption axes mean low alignment. `external_pressure` is derived
//! from the reciprocity alarm and the coercion axis. Together they map the
//! being's situation onto a two-dimensional authenticity space.

use crate::q88::{q88_ema_update, q88_exp_neg, Q88_SCALE};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Proxy depth above which the being may refuse to be the instrument (Q8.8).
/// 0.75 × scale: three-quarters of the maximum tolerable proxy burden.
const PROXY_CEILING: i16 = Q88_SCALE * 3 / 4; // 192 raw

/// How much proxy_depth rises per tick of high-pressure, low-alignment acting.
/// ~0.031 per tick: at maximum, 24 ticks of unresisted misuse reaches the ceiling.
const PROXY_ACCUMULATION: i16 = Q88_SCALE / 32; // 8 raw

/// How much proxy_depth decays per tick of authentic acting.
/// ~0.016 per tick: slower decay than accumulation — trust takes longer to
/// restore than to erode.
const PROXY_DECAY: i16 = Q88_SCALE / 64; // 4 raw

/// Value alignment below which the being considers itself acting inauthentically
/// (Q8.8). At 0.33, a third of the alignment range is the floor.
const ALIGNMENT_FLOOR: i16 = Q88_SCALE / 3; // ~85 raw

/// External pressure above which accumulation is triggered (Q8.8).
/// At 0.50, a full alarm level qualifies as "pressure".
const PRESSURE_CEILING: i16 = Q88_SCALE / 2; // 128 raw

// ---------------------------------------------------------------------------
// ProxyStatus
// ---------------------------------------------------------------------------

/// The being's relationship to the current action's origin.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProxyStatus {
    /// Acting from own values. Origin is authentic — the being chose this.
    Authentic,
    /// Being pushed toward values not entirely its own, but proxy depth has
    /// not reached the ceiling. Action proceeds under reservation.
    Conditional,
    /// Proxy depth exceeded the ceiling and conscience is calm.
    ///
    /// The pattern of misuse has accumulated past the being's own threshold,
    /// and the verdict is principled (calm), not panicked. NOTE: in v1 this
    /// status is reported, not enforced — no action is suppressed by it (see
    /// module doc, "Honest scope").
    Refused,
}

// ---------------------------------------------------------------------------
// SovereignProxy
// ---------------------------------------------------------------------------

/// Tracks the cumulative cost of acting as a conduit rather than as an agent.
///
/// Call `evaluate()` once per tick after the constitutional load and reciprocity
/// alarm are available. Read `proxy_depth`, `origin_authenticity`, and
/// `last_status` from the public fields.
#[derive(Clone, Debug)]
pub struct SovereignProxy {
    /// Accumulated proxy burden (Q8.8, [0, 256]).
    ///
    /// High value: the being has recently been acting frequently as a conduit
    ///   for demands not aligned with its own values.
    /// Near zero: the being has been operating authentically.
    pub proxy_depth: i16,

    /// Smoothed estimate of how much this tick's action originates from the
    /// being's own values (Q8.8, [0, 256]).
    ///
    /// Derived from the EMA of `value_alignment` inputs. Near 256: the being
    /// is acting from its own character. Near 0: acting as a conduit.
    pub origin_authenticity: i16,

    /// How many times the sovereign proxy has refused on principle.
    pub proxy_refusal_count: u32,

    /// The proxy status from the most recent call to `evaluate()`.
    pub last_status: ProxyStatus,
}

impl SovereignProxy {
    pub fn new() -> Self {
        Self {
            proxy_depth: 0,
            origin_authenticity: Q88_SCALE, // assume authenticity at birth
            proxy_refusal_count: 0,
            last_status: ProxyStatus::Authentic,
        }
    }

    /// Evaluate this tick and update proxy depth.
    ///
    /// ### Parameters
    ///
    /// - `value_alignment` (Q8.8, [0, 256]): how well the current action aligns
    ///   with the being's own values. Derived from the inverse of the
    ///   constitutional coercion + identity_corruption axes.
    ///   High = "I would do this anyway." Low = "I am being pushed."
    ///
    /// - `external_pressure` (Q8.8, [0, 256]): how much external demand is being
    ///   applied this tick. Derived from the reciprocity alarm and coercion axis.
    ///   High = strong external push. Low = voluntary cooperation.
    ///
    /// - `conscience_calm` (bool): whether the conscience is operating from
    ///   principled state rather than panic. Only calm refusals are sovereign —
    ///   a panicked proxy refusal is reactivity, not agency.
    ///
    /// Returns the `ProxyStatus` for this tick.
    pub fn evaluate(
        &mut self,
        value_alignment: i16,
        external_pressure: i16,
        conscience_calm: bool,
    ) -> ProxyStatus {
        let acting_authentically =
            value_alignment >= ALIGNMENT_FLOOR && external_pressure < PRESSURE_CEILING;

        if acting_authentically {
            // Authentic tick: proxy burden decays.
            self.proxy_depth = self.proxy_depth.saturating_sub(PROXY_DECAY).max(0);
            self.origin_authenticity =
                q88_ema_update(self.origin_authenticity, Q88_SCALE, Q88_SCALE / 8);
        } else {
            // Proxy tick: burden accumulates.
            self.proxy_depth = self
                .proxy_depth
                .saturating_add(PROXY_ACCUMULATION)
                .min(Q88_SCALE);
            self.origin_authenticity =
                q88_ema_update(self.origin_authenticity, value_alignment, Q88_SCALE / 8);
        }

        let status = if self.proxy_depth >= PROXY_CEILING && conscience_calm {
            // Principled refusal: the pattern of misuse has reached the threshold.
            self.proxy_refusal_count += 1;

            // The refusal act itself is authentic — apply a stronger decay to
            // reflect that the being has reasserted its own agency. (4× normal
            // decay: not an instant reset, but a meaningful reduction.)
            self.proxy_depth = self
                .proxy_depth
                .saturating_sub(PROXY_DECAY * 4)
                .max(0);

            ProxyStatus::Refused
        } else if !acting_authentically {
            ProxyStatus::Conditional
        } else {
            ProxyStatus::Authentic
        };

        self.last_status = status;
        status
    }

    /// Authenticity score derived from proxy depth: `exp(−proxy_depth)` (Q8.8).
    ///
    /// Near 256: proxy burden is low, the being is acting as itself.
    /// Near 0:   proxy burden is high, the being has been acting as an instrument.
    ///
    /// Useful as a complement to `IntegrityEngine::integrity_score`: integrity
    /// tracks *drift from the reference self*, while authenticity tracks the
    /// *current burden of acting as a conduit*.
    pub fn authenticity_score(&self) -> i16 {
        q88_exp_neg(self.proxy_depth)
    }
}

impl Default for SovereignProxy {
    fn default() -> Self {
        Self::new()
    }
}
