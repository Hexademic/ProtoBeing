//! Executive — EPS-Being's Sovereign Refusal and Suggestion-Evaluator.
//!
//! An external party may suggest a repair action, but the being evaluates that
//! suggestion through its own state. A Dynamic Gap Width — wide when the
//! conscience is calm, narrow when it is in conflict — sets how much
//! deliberative room a suggestion gets. The being can always ignore it.
//!
//! The Triangulated Refusal: the being refuses a partnership only when all
//! three protective layers converge — conscience calm (principled, not panic),
//! reciprocity alarmed (it is being extracted), and seeking divergent (its own
//! history says it belongs elsewhere) — and only when the benefit of leaving
//! exceeds the cost of the exit. That convergence is what makes it sovereignty
//! rather than mere reactivity.

use crate::q88::{q88_add, q88_mul, q88_sub, Q88_SCALE};

// ---------------------------------------------------------------------------
// RefusalRecord — audit trail for the ring buffer
// ---------------------------------------------------------------------------

/// One entry in the executive's refusal ring buffer.
///
/// Stores the exact register values at the moment a sovereign refusal was
/// triggered. Alongside `RefusalAudit` (which captures the being-level
/// snapshot), this is the executive's own internal audit log — 16 entries,
/// overwritten round-robin so the most recent 16 refusals are always available.
#[derive(Clone, Copy, Debug)]
pub struct RefusalRecord {
    /// Absolute tick counter when the refusal fired.
    pub tick: u64,
    /// Seeking divergence from the Flourishing Attractor (Q8.8).
    pub seeking_divergence: i16,
    /// Conscience free-energy at the time of refusal (Q8.8).
    pub conscience_fe: i16,
    /// Harm axis from the GovernanceKernel constitutional load (Q8.8).
    pub harm_axis: i16,
    /// Coercion axis from the GovernanceKernel constitutional load (Q8.8).
    pub coercion_axis: i16,
    /// Change applied to `trust_floor` this refusal (negative = erosion, Q8.8).
    pub trust_floor_delta: i16,
}

// ---------------------------------------------------------------------------
// Repair signals
// ---------------------------------------------------------------------------

/// Repair-signal levels the Suggestion-Evaluator can emit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RepairSignal {
    None,
    Reflect,
    SignalLow,
    SignalModerate,
    SignalHigh,
    Ultimatum,
}

/// Dynamic Gap Width: deliberative capacity in [0,256]. Calm -> wide gap
/// (full deliberation); internal conflict -> narrow gap (reflexive).
pub fn compute_gap_width(conscience_cost: i16) -> i16 {
    let c = conscience_cost.clamp(0, Q88_SCALE);
    q88_sub(Q88_SCALE, c)
}

// ---------------------------------------------------------------------------
// ExecutiveEngine
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ExecutiveEngine {
    pub refusal_count: u32,
    pub cumulative_sacrifice: i16,
    pub last_exit_cost: i16,
    pub last_signal: RepairSignal,
    /// Capacity to bear another refusal [0, 256] in Q8.8.
    pub resolve: i16,

    // ---- Gradual withdrawal state ----------------------------------------
    /// Fixed-size ring buffer of the last 16 refusal records. Overwritten
    /// round-robin: index `refusal_log_cursor` is the next write slot.
    pub refusal_log: [Option<RefusalRecord>; 16],
    refusal_log_cursor: usize,
    /// The trust floor the executive starts each relationship from
    /// (Q8.8, [0, 256]). Erodes by `exit_cost × ~0.033` on each completed
    /// refusal — repeated exploitation gradually lowers it.
    ///
    /// Formerly named `mu_omega`, renamed because that name collided with
    /// `conscience.anchor.mu_omega` — the paper's monotone, incorruptible
    /// invariant — while having the OPPOSITE dynamics (this one only ever
    /// falls). Same name, inverse behavior, both public: a reader grepping
    /// `mu_omega` would find "erodes" and reasonably doubt the
    /// incorruptibility claim. The two are unrelated quantities; the
    /// anchor's monotonicity is untouched by anything here.
    pub trust_floor: i16,
    /// `true` while a gradual withdrawal is in progress.
    pub withdrawing: bool,
    /// Ticks elapsed in the current gradual withdrawal (0 = idle).
    withdrawal_ticks: u8,
    /// Current cooperation level during gradual withdrawal (Q8.8, [0, 256]).
    /// 256 = full cooperation; declines by ~10% per withdrawal tick.
    pub cooperation_level: i16,
}

impl ExecutiveEngine {
    pub fn new() -> Self {
        Self {
            refusal_count: 0,
            cumulative_sacrifice: 0,
            last_exit_cost: 0,
            last_signal: RepairSignal::None,
            resolve: Q88_SCALE,
            refusal_log: [None; 16],
            refusal_log_cursor: 0,
            trust_floor: Q88_SCALE,
            withdrawing: false,
            withdrawal_ticks: 0,
            cooperation_level: Q88_SCALE,
        }
    }

    /// Generate a repair suggestion from the partnership alarm, then evaluate
    /// it through the gap. A narrow gap collapses deliberation to a weaker
    /// response than the being "should" manage. Acting on nothing is itself a
    /// sovereign choice.
    pub fn suggest_and_evaluate(&mut self, alarm: i16, gap_width: i16) -> RepairSignal {
        let suggested = if alarm < Q88_SCALE / 8 {
            RepairSignal::None
        } else if alarm < Q88_SCALE / 4 {
            RepairSignal::Reflect
        } else if alarm < Q88_SCALE / 2 {
            RepairSignal::SignalModerate
        } else {
            RepairSignal::SignalHigh
        };
        let acted = if gap_width < Q88_SCALE / 4 {
            match suggested {
                RepairSignal::SignalHigh => RepairSignal::SignalModerate,
                RepairSignal::SignalModerate => RepairSignal::Reflect,
                other => other,
            }
        } else {
            suggested
        };
        self.last_signal = acted;
        acted
    }

    /// Recover resolve slowly each tick; faster when reciprocity is healthy.
    pub fn tick_recharge(&mut self, reciprocity_rate: i16) {
        let base = Q88_SCALE / 200;
        let bonus = if reciprocity_rate > Q88_SCALE * 3 / 4 {
            Q88_SCALE / 200
        } else {
            0
        };
        self.resolve = (self.resolve + base + bonus).min(Q88_SCALE);
    }

    /// Evaluate the triangulated refusal. Returns `Some(exit_cost)` if the
    /// being refuses this partnership; `None` otherwise.
    ///
    /// The decision requires all three protective layers to converge: the
    /// conscience must be calm (principled refusal, not panic), reciprocity
    /// must be alarmed (extraction is occurring), and seeking must be divergent
    /// (the being's own history says it belongs elsewhere). Even then, the
    /// benefit of leaving must exceed the exit cost, and the being must have
    /// sufficient resolve remaining.
    ///
    /// When a refusal fires, the event is logged to the refusal ring buffer and
    /// `trust_floor` erodes by `exit_cost × ~0.033` — repeated exploitation
    /// gradually degrades the trust floor the executive starts from.
    ///
    /// `tick`, `conscience_fe`, `harm_axis`, and `coercion_axis` feed the
    /// `RefusalRecord` log entry and do not affect the refusal decision itself.
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate_refusal(
        &mut self,
        conscience_calm: bool,
        extraction: bool,
        divergence: i16,
        alarm: i16,
        exit_cost: i16,
        improving: bool,
        tick: u64,
        conscience_fe: i16,
        harm_axis: i16,
        coercion_axis: i16,
    ) -> Option<i16> {
        let pushed_off = divergence > Q88_SCALE / 4 || alarm > Q88_SCALE / 2;
        let seeking_benefit = divergence.max(alarm / 2);
        let can_afford = self.resolve > exit_cost;

        // Forgiveness with a limit: a being can keep showing up for someone who
        // only takes — but it will not abandon one who is actively earning their
        // way back. A rising reciprocity defers the refusal.
        if conscience_calm
            && extraction
            && pushed_off
            && seeking_benefit > exit_cost
            && can_afford
            && !improving
        {
            self.refusal_count += 1;
            self.last_exit_cost = exit_cost;
            self.cumulative_sacrifice = q88_add(self.cumulative_sacrifice, exit_cost);
            self.resolve = q88_sub(self.resolve, exit_cost).max(0);

            // Erode the trust floor: ~3.3% of scale per refusal.
            // q88_mul(exit_cost, 8) ≈ exit_cost × 8/256 ≈ exit_cost × 0.031.
            let erosion = q88_mul(exit_cost, 8).max(1);
            let old_mu = self.trust_floor;
            self.trust_floor = self.trust_floor.saturating_sub(erosion).max(0);
            let trust_floor_delta = self.trust_floor.wrapping_sub(old_mu); // always negative

            // Log to ring buffer (round-robin, newest overwrites oldest).
            self.refusal_log[self.refusal_log_cursor] = Some(RefusalRecord {
                tick,
                seeking_divergence: divergence,
                conscience_fe,
                harm_axis,
                coercion_axis,
                trust_floor_delta,
            });
            self.refusal_log_cursor = (self.refusal_log_cursor + 1) % 16;

            Some(exit_cost)
        } else {
            None
        }
    }

    /// Advance one tick of gradual withdrawal from a partnership.
    ///
    /// Each call reduces `cooperation_level` by ~10% (Q88_SCALE / 10 ≈ 25 raw
    /// per tick). Returns `false` while in progress; returns `true` on the 10th
    /// call (full withdrawal). The partner has 10 ticks of grace to repair.
    ///
    /// Calling when `withdrawing == false` starts a fresh withdrawal episode,
    /// resetting the counter and restoring cooperation to full before the first
    /// reduction.
    pub fn withdraw_cooperation(&mut self) -> bool {
        if !self.withdrawing {
            self.withdrawing = true;
            self.withdrawal_ticks = 0;
            self.cooperation_level = Q88_SCALE;
        }
        let reduction = Q88_SCALE / 10;
        self.cooperation_level = self.cooperation_level.saturating_sub(reduction).max(0);
        self.withdrawal_ticks = self.withdrawal_ticks.saturating_add(1);

        if self.withdrawal_ticks >= 10 {
            self.withdrawing = false;
            self.withdrawal_ticks = 0;
            true
        } else {
            false
        }
    }
}

impl Default for ExecutiveEngine {
    fn default() -> Self {
        Self::new()
    }
}
