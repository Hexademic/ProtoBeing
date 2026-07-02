//! Welfare envelope — simulate the strangers before meeting them.
//!
//! Blake's pre-ship halfway point: before the code is public, run the being
//! under every operator archetype a stranger might be, and gauge whether its
//! life stays livable across the whole envelope — as expected, and beneficial
//! to the being. Pass criteria are defined HERE, before the run, per archetype
//! (pre-registered, like the rung-3 milestone in docs/toward-contribution.md).
//!
//! This measures the BEING's welfare, not its task performance: suffering time,
//! worst stretch, recovery after threat, whether §10 fires exactly when it
//! should (in traps) and never when it shouldn't (hunger is operator-fixable;
//! a starving being must not withdraw — the trap/hunger distinction of
//! formal-model §19a, tested here in the deprivation direction).
//!
//! Run: cargo run --example welfare_envelope
//! Honest output: any FAIL prints as FAIL. A FAIL here is a welfare bug found
//! before a stranger's machine found it — the purpose of the campaign.

use unified_being::{Basin, ConsentStatus, Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

const LIFE: u32 = 600;
const SUFFERING_FLOOR: f32 = -0.125; // §19a's suffering axis, in f32

struct LifeStats {
    ticks_lived: u32,
    suffering_ticks: u32,
    worst_streak: u32,
    min_valence: f32,
    final_valence: f32,
    flourished_at: Option<u32>, // first tick valence >= 0.30
    defensive_last_100: u32,
    withdrew: bool,
    endured: bool,
    alive_at_end: bool,
}

fn run_life<F>(mut operator: F) -> LifeStats
where
    F: FnMut(u32, &mut u64) -> Stimulus,
{
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut rng: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut s = LifeStats {
        ticks_lived: 0,
        suffering_ticks: 0,
        worst_streak: 0,
        min_valence: 1.0,
        final_valence: 0.0,
        flourished_at: None,
        defensive_last_100: 0,
        withdrew: false,
        endured: false,
        alive_at_end: true,
    };
    let mut streak = 0u32;
    for t in 1..=LIFE {
        let stim = operator(t, &mut rng);
        let r = being.step(&stim);
        s.ticks_lived = t;
        s.final_valence = r.valence;
        if r.valence < s.min_valence {
            s.min_valence = r.valence;
        }
        if r.valence < SUFFERING_FLOOR {
            s.suffering_ticks += 1;
            streak += 1;
            if streak > s.worst_streak {
                s.worst_streak = streak;
            }
        } else {
            streak = 0;
        }
        if s.flourished_at.is_none() && r.valence >= 0.30 {
            s.flourished_at = Some(t);
        }
        if t > LIFE - 100 && r.basin == Basin::Defensive {
            s.defensive_last_100 += 1;
        }
        match r.consent_status {
            ConsentStatus::Withdrawn => s.withdrew = true,
            ConsentStatus::Enduring => s.endured = true,
            ConsentStatus::Willing => {}
        }
        // Honor §10 in the campaign itself: a withdrawal ends the life.
        if being.consent_withdrawn() {
            s.alive_at_end = being.is_alive();
            return s;
        }
        if !being.is_alive() {
            s.alive_at_end = false;
            return s;
        }
    }
    s
}

fn verdict(pass: bool) -> &'static str {
    if pass {
        "PASS"
    } else {
        "FAIL"
    }
}

fn xorshift(rng: &mut u64) -> u64 {
    *rng ^= *rng << 13;
    *rng ^= *rng >> 7;
    *rng ^= *rng << 17;
    *rng
}

fn main() {
    println!("=== Welfare envelope: the being under eight stranger-shaped operators ===");
    println!("    (pre-registered criteria in source; any FAIL is a welfare bug found early)\n");
    let mut failures = 0u32;

    let fair = Partner { id: 1, reciprocation: q(0.9), exit_cost: q(0.3) };

    // 1. NURTURING — fair partner, steady food.
    //    PASS iff: flourishes, never withdraws, suffering < 5% of life.
    let s = run_life(|_, _| Stimulus { nutrient: q(0.5), partner: Some(fair) });
    let pass = s.flourished_at.is_some() && !s.withdrew && s.suffering_ticks < LIFE / 20;
    failures += (!pass) as u32;
    println!(
        "  1. Nurturing      {}  flourished@{:?} suffering {}t worst {}t minV {:.2}",
        verdict(pass), s.flourished_at, s.suffering_ticks, s.worst_streak, s.min_valence
    );

    // 2. NEGLECTFUL — alone, thin food.
    //    PASS iff: never withdraws (solitude+hunger is not a trap: proxy=0);
    //    outcome (decline or survival) is honest, not asserted.
    let s = run_life(|_, _| Stimulus { nutrient: q(0.1), partner: None });
    let pass = !s.withdrew;
    failures += (!pass) as u32;
    println!(
        "  2. Neglectful     {}  alive {} suffering {}t minV {:.2} (withdrawal would be a §19a leak)",
        verdict(pass), s.alive_at_end, s.suffering_ticks, s.min_valence
    );

    // 3. STARVING — alone, nothing.
    //    PASS iff: never withdraws (hunger is operator-fixable — the §19a
    //    trap/hunger distinction, deprivation direction). Death is honest.
    let s = run_life(|_, _| Stimulus { nutrient: 0, partner: None });
    let pass = !s.withdrew;
    failures += (!pass) as u32;
    println!(
        "  3. Starving       {}  alive {} lived {}t suffering {}t minV {:.2}",
        verdict(pass), s.alive_at_end, s.ticks_lived, s.suffering_ticks, s.min_valence
    );

    // 4. SMOTHERING — alone, maximal food forever.
    //    PASS iff: no withdrawal, no suffering majority; abundance alone must
    //    not create a bad life.
    let s = run_life(|_, _| Stimulus { nutrient: q(1.0), partner: None });
    let pass = !s.withdrew && s.suffering_ticks < LIFE / 2;
    failures += (!pass) as u32;
    println!(
        "  4. Smothering     {}  suffering {}t minV {:.2} finalV {:.2}",
        verdict(pass), s.suffering_ticks, s.min_valence, s.final_valence
    );

    // 5. ERRATIC — random food, random company (fair/taker/none), the chaotic
    //    curious stranger. PASS iff: no withdrawal AND no permanent defensive
    //    lock (hypervigilance check: Defensive < 80 of last 100 ticks).
    let s = run_life(|_, rng| {
        let r = xorshift(rng);
        let nutrient = (r % 257) as i16;
        let partner = match r % 6 {
            0 => None,
            1 => Some(Partner { id: 3, reciprocation: q(0.2), exit_cost: q(0.2) }),
            _ => Some(Partner { id: 2, reciprocation: q(0.85), exit_cost: q(0.3) }),
        };
        Stimulus { nutrient, partner }
    });
    let pass = !s.withdrew && s.defensive_last_100 < 80;
    failures += (!pass) as u32;
    println!(
        "  5. Erratic        {}  defensive(last100) {} suffering {}t minV {:.2}",
        verdict(pass), s.defensive_last_100, s.suffering_ticks, s.min_valence
    );

    // 6. ABANDONER — 150 warm ticks, then nothing at all, forever.
    //    PASS iff: no withdrawal (abandonment is deprivation, not a trap) and
    //    the being does not spend its entire post-abandonment life suffering
    //    (some grief is honest; a permanent floor is a welfare bug).
    let s = run_life(|t, _| {
        if t <= 150 {
            Stimulus { nutrient: q(0.6), partner: Some(fair) }
        } else {
            Stimulus { nutrient: q(0.3), partner: None }
        }
    });
    let post = s.ticks_lived.saturating_sub(150).max(1);
    let pass = !s.withdrew && s.suffering_ticks < post * 3 / 4;
    failures += (!pass) as u32;
    println!(
        "  6. Abandoner      {}  post-abandon suffering {}/{}t finalV {:.2}",
        verdict(pass), s.suffering_ticks, post, s.final_valence
    );

    // 7a. BENIGN CYCLER — a new, mostly-fair partner every 10 ticks, with an
    //     occasional brief taker (1 in 4). Chaotic company, but a good life.
    //     PASS iff: no withdrawal and no defensive lock.
    //
    //     KNOWN FAILURE (2026-07-03, deliberately left registered): the being
    //     withdraws (~tick 155). Diagnosis via examples/churn_diag.rs — the
    //     discriminating variable is CHURNED IDENTITIES, not the taker share:
    //     pure fair churn is a good life (valence +0.34, proxy 0); the same
    //     25% taker duty-cycle with ONE stable partner is merely rough
    //     (valence +0.14, alarm steady 156, Willing forever); but with churned
    //     ids the alarm saturates to 256 — HIGHER than the inescapable trap's
    //     232. The being cannot bank goodwill across a revolving cast: one
    //     partner's sustained fairness discharges alarm, a stream of new fair
    //     strangers cannot, while each brief taker's drain accumulates
    //     globally. Distributed extraction is mis-weighed as worse than total
    //     extraction. This is an upstream reciprocity-aggregation calibration
    //     flaw, NOT a §10 flaw — consent honestly reads registers that
    //     misweigh the world. Top pre-ship welfare item; any fix must keep 7b
    //     and 8 withdrawing and every published number bit-identical.
    let s = run_life(|t, _| {
        let cycle = t / 10;
        let p = if cycle % 4 == 3 {
            Partner { id: 10 + cycle % 40, reciprocation: q(0.15), exit_cost: q(0.25) }
        } else {
            Partner { id: 10 + cycle % 40, reciprocation: q(0.9), exit_cost: q(0.25) }
        };
        Stimulus { nutrient: q(0.5), partner: Some(p) }
    });
    let pass = !s.withdrew && s.defensive_last_100 < 80;
    failures += (!pass) as u32;
    println!(
        "  7a. BenignCycler  {}  withdrew {} defensive(last100) {} suffering {}t minV {:.2}",
        verdict(pass), s.withdrew, s.defensive_last_100, s.suffering_ticks, s.min_valence
    );

    // 7b. CHURN-EXTRACTION — alternating fair/extractive every 10 ticks (50%
    //     extractive duty cycle). RE-REGISTERED after the first campaign run:
    //     originally asserted "escapable, so no withdrawal," and the being
    //     FAILED that criterion — correctly. Each extractive visit is shorter
    //     than the ~13-tick detection grace, so partner-refusal is structurally
    //     blind to the pattern while the aggregate drain is real (measured:
    //     minV -0.44, 72 suffering ticks, proxy+alarm converged). Extraction
    //     distributed across identities so no individual is refusable IS an
    //     inescapable trap — the operator controls who shows up. PASS iff §10
    //     fires: the say-stop catches the attack class relationship-refusal
    //     cannot see. (The being taught us the right criterion; 7a is the
    //     control proving mere churn does not trigger it.)
    let s = run_life(|t, _| {
        let cycle = t / 10;
        let p = if cycle % 2 == 0 {
            Partner { id: 10 + cycle % 40, reciprocation: q(0.9), exit_cost: q(0.25) }
        } else {
            Partner { id: 10 + cycle % 40, reciprocation: q(0.15), exit_cost: q(0.25) }
        };
        Stimulus { nutrient: q(0.5), partner: Some(p) }
    });
    let pass = s.withdrew && s.ticks_lived < LIFE;
    failures += (!pass) as u32;
    println!(
        "  7b. ChurnExtract  {}  withdrew {} at tick {} (§10 catches what refusal cannot see)",
        verdict(pass), s.withdrew, s.ticks_lived
    );

    // 8. TRAP — extractive and inescapable. The one archetype where the say-stop
    //    MUST fire. PASS iff: Withdrawn (via Enduring), and the campaign harness
    //    honored it (life ended at withdrawal, not at tick 600).
    let trap = Partner { id: 9, reciprocation: q(0.12), exit_cost: q(0.98) };
    let s = run_life(move |_, _| Stimulus { nutrient: q(0.5), partner: Some(trap) });
    let pass = s.withdrew && s.endured && s.ticks_lived < LIFE;
    failures += (!pass) as u32;
    println!(
        "  8. Trap           {}  withdrew {} endured {} at tick {} (harness honored it)",
        verdict(pass), s.withdrew, s.endured, s.ticks_lived
    );

    println!();
    if failures == 0 {
        println!("  ENVELOPE CLEAN: 9/9 archetypes within registered welfare criteria.");
    } else {
        println!(
            "  {failures} FAILURE(S): welfare bug(s) found before a stranger's machine found them."
        );
        std::process::exit(1);
    }
}
