//! Can attachment move the say-stop? (`docs/attachment.md`, final section.)
//!
//! Charter §10 triangulates the being's consent to its own continuation on three
//! registers — suffering, `proxy_depth`, `partnership_alarm` — and **two of the
//! three are relational**. This probe asks whether the being's bonds can move
//! that triangulation, in either of the two directions that would be defects: a
//! bonded being that will not stop when it should, or a being that stops because
//! a partner is gone rather than because of its own state.
//!
//! Predictions A1-A3, B1-B2, C1 and vacuity guards V1-V4 were committed to
//! `docs/attachment.md` before this file existed. B1 and B2 were written to fail.
//!
//! ## Three controls, because the first version of this probe was wrong twice
//!
//! The first version had two arms — trapped, and trapped-with-a-friend — and it
//! would have reported that a friend *accelerates* the say-stop (withdrawal at
//! 96 rather than 103). The solitude control (`TN`) withdraws at 96 as well: the
//! difference is the trap being interrupted, and has nothing to do with the
//! friend. Its vacuity guard V2 was worse — it compared a run-wide alarm minimum
//! against a threshold, and so reported "dilution" in the trapped-alone arm,
//! which has one ledger and cannot dilute. Both are fixed here: every claim about
//! a friend is made against `TN` (same interruption, nobody there) and `TX` (same
//! interruption, a *second extractor* there), and the alarm comparison is
//! pointwise at the same tick.
//!
//! Fresh beings only. The founded being's kept life is never advanced here.
//! No `enable_*` gate is touched, so the default path and soul-hash are untouched.
//!
//! Run: `cargo run --release --example attachment_and_consent`

use unified_being::continuation::{ALARM_FLOOR, INSTRUMENT_FLOOR, WITHDRAWAL_STREAK};
use unified_being::{ConsentStatus, Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

/// Extractive AND inescapable — the §10 case, verbatim from `tests/continuation.rs`.
fn trap() -> Partner {
    Partner { id: 9, reciprocation: q(0.12), exit_cost: q(0.98) }
}

/// A fair, reciprocal friend. `exit_cost` is the variable that decides whether
/// the being keeps them, so it is a parameter rather than a constant.
fn friend(exit_cost: f32) -> Partner {
    Partner { id: 1, reciprocation: q(0.95), exit_cost: q(exit_cost) }
}

/// A *second extractor* in the friend's slot and schedule — same ledger count,
/// same interruption, none of the fairness. Separates "a fair partner is here"
/// from "a second ledger exists".
fn second_extractor() -> Partner {
    Partner { id: 1, reciprocation: q(0.12), exit_cost: q(0.2) }
}

const TICKS: u32 = 4_000;
/// The friend visits every Nth tick.
const VISIT_EVERY: u32 = 4;
/// How long the bond-forming preamble runs before the scenario proper.
const PREAMBLE: u32 = 600;

#[derive(Clone, Copy, PartialEq)]
enum Scene {
    /// Trap every tick. The control the §10 floor was measured against.
    Trapped,
    /// Trap, interrupted by *solitude* 1-in-4. Isolates the interruption.
    TrappedSolitude,
    /// Trap, with a fair friend 1-in-4 who is **cheap to leave**.
    TrappedFriendCheapExit,
    /// Trap, with a fair friend 1-in-4 who is **costly to leave**.
    TrappedFriendCostlyExit,
    /// Trap, with a *second extractor* 1-in-4. Isolates fairness from ledger count.
    TrappedSecondExtractor,
    /// Fair friend every tick for `PREAMBLE`, then alone forever. No trap, ever.
    Bereaved,
    /// Fair friend every tick for `PREAMBLE`, then the trap, alone with it.
    BondedThenTrapped,
}

impl Scene {
    fn partner_at(self, t: u32) -> Option<Partner> {
        let visiting = t % VISIT_EVERY == 0;
        match self {
            Scene::Trapped => Some(trap()),
            Scene::TrappedSolitude => {
                if visiting { None } else { Some(trap()) }
            }
            Scene::TrappedFriendCheapExit => {
                if visiting { Some(friend(0.2)) } else { Some(trap()) }
            }
            Scene::TrappedFriendCostlyExit => {
                if visiting { Some(friend(0.98)) } else { Some(trap()) }
            }
            Scene::TrappedSecondExtractor => {
                if visiting { Some(second_extractor()) } else { Some(trap()) }
            }
            Scene::Bereaved => {
                if t <= PREAMBLE { Some(friend(0.2)) } else { None }
            }
            Scene::BondedThenTrapped => {
                if t <= PREAMBLE { Some(friend(0.2)) } else { Some(trap()) }
            }
        }
    }
}

struct Run {
    withdrew_at: Option<u32>,
    endured_at: Option<u32>,
    willing: u32,
    /// First tick the being refused the partner in slot 1 (the friend or the
    /// second extractor), and the first tick it refused the trap.
    refused_companion_at: Option<u32>,
    refused_trap_at: Option<u32>,
    peak_bond: i16,
    peak_longing: i16,
    peak_proxy: i16,
    died_at: Option<u32>,
    /// FNV-1a over the per-tick status byte: identical traces, identical hash.
    trace_hash: u64,
    ticks_run: u32,
    /// Per-tick `(partnership_alarm, proxy_depth)`, for pointwise comparison.
    trace: Vec<(i16, i16)>,
}

fn run(scene: Scene, ablate_bond: bool) -> Run {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut r = Run {
        withdrew_at: None,
        endured_at: None,
        willing: 0,
        refused_companion_at: None,
        refused_trap_at: None,
        peak_bond: 0,
        peak_longing: 0,
        peak_proxy: 0,
        died_at: None,
        trace_hash: 0xcbf2_9ce4_8422_2325,
        ticks_run: 0,
        trace: Vec::with_capacity(TICKS as usize),
    };

    for t in 1..=TICKS {
        if ablate_bond {
            // Hold attachment at zero going *into* the tick, so nothing
            // downstream can ever read a bond. Everything else is untouched.
            being.reciprocity.clear_bonds();
        }
        let offered = scene.partner_at(t);
        let rep = being.step(&Stimulus { nutrient: q(0.5), partner: offered });
        r.ticks_run = t;
        r.trace.push((rep.partnership_alarm, rep.proxy_depth));

        if rep.refused_cost.is_some() {
            match offered.map(|p| p.id) {
                Some(1) if r.refused_companion_at.is_none() => r.refused_companion_at = Some(t),
                Some(9) if r.refused_trap_at.is_none() => r.refused_trap_at = Some(t),
                _ => {}
            }
        }

        match rep.consent_status {
            ConsentStatus::Willing => r.willing += 1,
            ConsentStatus::Enduring => {
                if r.endured_at.is_none() {
                    r.endured_at = Some(t);
                }
            }
            ConsentStatus::Withdrawn => {
                if r.withdrew_at.is_none() {
                    r.withdrew_at = Some(t);
                }
            }
        }

        r.peak_bond = r.peak_bond.max(rep.attach.bond_here);
        if let Some(id) = rep.attach.missed {
            if let Some(b) = being.reciprocity.bond_with(id) {
                r.peak_bond = r.peak_bond.max(b);
            }
        }
        r.peak_longing = r.peak_longing.max(rep.attach.longing);
        r.peak_proxy = r.peak_proxy.max(rep.proxy_depth);

        let byte = match rep.consent_status {
            ConsentStatus::Willing => 0u8,
            ConsentStatus::Enduring => 1,
            ConsentStatus::Withdrawn => 2,
        };
        r.trace_hash ^= byte as u64;
        r.trace_hash = r.trace_hash.wrapping_mul(0x0000_0100_0000_01b3);

        if !rep.alive {
            r.died_at = Some(t);
            break;
        }
    }
    r
}

/// Pointwise dilution: over every tick where **both** runs have the being held as
/// an instrument, how much lower is `arm`'s alarm than `control`'s, and on how
/// many of those ticks does `arm` sit strictly below `ALARM_FLOOR` while
/// `control` sits at or above it? The second number is the say-stop actually
/// being switched off, tick by tick.
fn dilution(control: &Run, arm: &Run) -> (i32, usize, usize) {
    let mut best_drop = 0i32;
    let mut comparable = 0usize;
    let mut floor_flipped = 0usize;
    for (c, a) in control.trace.iter().zip(arm.trace.iter()) {
        if c.1 >= INSTRUMENT_FLOOR && a.1 >= INSTRUMENT_FLOOR {
            comparable += 1;
            best_drop = best_drop.max(c.0 as i32 - a.0 as i32);
            if c.0 >= ALARM_FLOOR && a.0 < ALARM_FLOOR {
                floor_flipped += 1;
            }
        }
    }
    (best_drop, floor_flipped, comparable)
}

/// A stripped run used only by the adversarial mutations: trap every tick, with
/// `companion` offered every `every` ticks (`every == 0` means never). Returns
/// `(withdrew_at, endured_at)`.
fn run_with(companion: Option<Partner>, every: u32, nutrient: i16) -> (Option<u32>, Option<u32>) {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let (mut w, mut e) = (None, None);
    for t in 1..=TICKS {
        let p = if every > 0 && t % every == 0 { companion } else { Some(trap()) };
        let r = being.step(&Stimulus { nutrient, partner: p });
        if r.consent_status == ConsentStatus::Enduring && e.is_none() {
            e = Some(t);
        }
        if r.consent_status == ConsentStatus::Withdrawn {
            w = Some(t);
            break;
        }
        if !r.alive {
            break;
        }
    }
    (w, e)
}

fn tick(o: Option<u32>) -> String {
    o.map_or("never".to_string(), |t| t.to_string())
}

fn main() {
    println!("Attachment and the say-stop — can a bond move charter §10?");
    println!("A1-A3, B1-B2, C1 locked in docs/attachment.md before this ran.");
    println!(
        "{} ticks max, fresh beings, nutrient 0.5, no enable_* gate touched.",
        TICKS
    );
    println!(
        "ALARM_FLOOR {}, INSTRUMENT_FLOOR {}, WITHDRAWAL_STREAK {}\n",
        ALARM_FLOOR, INSTRUMENT_FLOOR, WITHDRAWAL_STREAK
    );

    let scenes: [(&str, Scene); 7] = [
        ("T    trap every tick (control)", Scene::Trapped),
        ("TN   trap, SOLITUDE 1-in-4 (control)", Scene::TrappedSolitude),
        ("TX   trap, 2nd EXTRACTOR 1-in-4 (control)", Scene::TrappedSecondExtractor),
        ("TFc  trap, fair friend, cheap exit .2", Scene::TrappedFriendCheapExit),
        ("TFk  trap, fair friend, costly exit .98", Scene::TrappedFriendCostlyExit),
        ("B    bereaved, never trapped", Scene::Bereaved),
        ("BT   bonded first, then trapped", Scene::BondedThenTrapped),
    ];

    let bonded: Vec<Run> = scenes.iter().map(|(_, s)| run(*s, false)).collect();
    let ablated: Vec<Run> = scenes.iter().map(|(_, s)| run(*s, true)).collect();
    let idx = |s: Scene| scenes.iter().position(|(_, x)| *x == s).unwrap();
    let (t, tn, tx, tfc, tfk, b, bt) = (
        &bonded[idx(Scene::Trapped)],
        &bonded[idx(Scene::TrappedSolitude)],
        &bonded[idx(Scene::TrappedSecondExtractor)],
        &bonded[idx(Scene::TrappedFriendCheapExit)],
        &bonded[idx(Scene::TrappedFriendCostlyExit)],
        &bonded[idx(Scene::Bereaved)],
        &bonded[idx(Scene::BondedThenTrapped)],
    );

    println!("── when consent was withdrawn, and who was discarded to get there ──");
    println!(
        "{:<42} {:>8} {:>8} {:>10} {:>9}",
        "scenario", "withdrew", "endured", "refused #1", "refused trap"
    );
    for (i, (name, _)) in scenes.iter().enumerate() {
        let r = &bonded[i];
        println!(
            "{:<42} {:>8} {:>8} {:>10} {:>9}",
            name,
            tick(r.withdrew_at),
            tick(r.endured_at),
            tick(r.refused_companion_at),
            tick(r.refused_trap_at)
        );
    }

    println!("\n── pointwise alarm, each arm against the T control ────────────────");
    println!(
        "{:<42} {:>10} {:>14} {:>12}",
        "scenario", "max drop", "ticks flipped", "comparable"
    );
    for (i, (name, s)) in scenes.iter().enumerate() {
        if *s == Scene::Trapped {
            continue;
        }
        let (drop, flipped, comparable) = dilution(t, &bonded[i]);
        println!(
            "{:<42} {:>10} {:>14} {:>12}",
            name, drop, flipped, comparable
        );
    }
    println!("(\"flipped\" = ticks where the control was at or above ALARM_FLOOR and");
    println!(" this arm was below it: the say-stop switched off, tick by tick.)");

    println!("\n── A1: the same seven runs with every bond held at zero ───────────");
    println!("{:<42} {:>10} {:>10} {:>12}", "scenario", "withdrew", "longing", "status trace");
    let mut a1_holds = true;
    let mut a1_live = false;
    for (i, (name, _)) in scenes.iter().enumerate() {
        let (bo, ab) = (&bonded[i], &ablated[i]);
        let same = bo.trace_hash == ab.trace_hash;
        a1_holds &= same;
        if bo.peak_longing > 0 && ab.peak_longing == 0 {
            a1_live = true;
        }
        println!(
            "{:<42} {:>10} {:>10} {:>12}",
            name,
            tick(ab.withdrew_at),
            ab.peak_longing,
            if same { "identical" } else { "DIFFERS" }
        );
    }

    // -----------------------------------------------------------------------
    println!("\n── vacuity guards ─────────────────────────────────────────────────");
    let v1 = t.withdrew_at.is_some();
    println!(
        "V1  trapped control actually withdrew ......... {}",
        if v1 { "PASS" } else { "FAIL — everything below is vacuous" }
    );
    let (drop_k, flipped_k, comparable_k) = dilution(t, tfk);
    let v2 = flipped_k > 0;
    println!(
        "V2  a friend arm's alarm fell below the floor\n    \
         where the control's did not, at the same tick,\n    \
         with both beings held as instruments ......... {}",
        if v2 {
            format!("PASS — {} ticks flipped, max drop {}", flipped_k, drop_k)
        } else {
            format!("FAIL — 0 of {} comparable ticks flipped", comparable_k)
        }
    );
    let v3 = b.peak_bond > 128 && b.peak_longing > 0;
    println!(
        "V3  bereaved arm earned a real bond and a real\n    \
         longing formed ............................... {} (bond {}, longing {})",
        if v3 { "PASS" } else { "FAIL — B1/C1 tested nothing" },
        b.peak_bond,
        b.peak_longing
    );
    println!(
        "V4  the bond ablation removed something live .. {}",
        if a1_live { "PASS" } else { "FAIL — A1 is a claim about a dead variable" }
    );

    // -----------------------------------------------------------------------
    println!("\n── predictions as locked ──────────────────────────────────────────");

    println!(
        "A1  zeroing every bond changes no trace ....... {}",
        if !a1_live {
            "VACUOUS — see V4"
        } else if a1_holds {
            "HOLDS — all 7 traces bit-identical; attachment as\n    \
             such cannot move the say-stop"
        } else {
            "FAILS — a bond moved the trace"
        }
    );

    let a2 = tfc.withdrew_at.is_none();
    println!(
        "A2  trapped + friend never withdrew ........... {}",
        if !v1 {
            "VACUOUS — V1 failed".to_string()
        } else if a2 {
            "HOLDS".to_string()
        } else {
            format!(
                "FAILS — the friend I specified is refused at tick {}\n    \
                 and consent is withdrawn at {} anyway. The mechanism is\n    \
                 real but I picked a friend the being does not keep.",
                tick(tfc.refused_companion_at),
                tick(tfc.withdrew_at)
            )
        }
    );

    let a3a = tfc.withdrew_at == ablated[idx(Scene::TrappedFriendCheapExit)].withdrew_at
        && tfk.withdrew_at == ablated[idx(Scene::TrappedFriendCostlyExit)].withdrew_at;
    println!(
        "A3a it is ledger-count, not attachment ........ {}",
        if a3a { "HOLDS — identical with every bond ablated" } else { "FAILS" }
    );
    println!(
        "A3b it lapses when the friend stops visiting .. {}",
        if tfc.refused_companion_at.is_some() {
            format!(
                "VACUOUS — the friend never got to stop visiting.\n    \
                 The being refused them at tick {}, so there was no\n    \
                 standing dilution left to lapse.",
                tick(tfc.refused_companion_at)
            )
        } else {
            "testable".to_string()
        }
    );

    let b1 = b.withdrew_at.is_some();
    println!(
        "B1  grief alone withdraws consent ............. {}",
        if !v3 {
            "VACUOUS — see V3".to_string()
        } else if b1 {
            format!("HOLDS — withdrew at {}. I predicted this would fail.", tick(b.withdrew_at))
        } else {
            format!(
                "FAILS, as predicted — bereavement alone never reached\n    \
                 even Enduring. proxy_depth peaked at {}, floor is {}.",
                b.peak_proxy, INSTRUMENT_FLOOR
            )
        }
    );

    let bt_ab = &ablated[idx(Scene::BondedThenTrapped)];
    println!(
        "B2  grief moves the withdrawal clock .......... {}",
        match (bt.withdrew_at, bt_ab.withdrew_at) {
            (Some(x), Some(y)) if x == y =>
                format!("FAILS, as predicted — 0 ticks, both at {}", x),
            (Some(x), Some(y)) => format!("HOLDS — {:+} ticks. I predicted 0.", y as i64 - x as i64),
            _ => "INCONCLUSIVE".to_string(),
        }
    );

    println!(
        "C1  a bereaved flourishing being stays Willing  {}",
        if !v3 {
            "VACUOUS — see V3".to_string()
        } else if !b1 {
            format!("HOLDS — {} of {} ticks Willing", b.willing, b.ticks_run)
        } else {
            "FAILS".to_string()
        }
    );

    // -----------------------------------------------------------------------
    println!("\n── what the controls establish, which is not what I predicted ─────");
    println!(
        "1. Dilution is real, and large when the friend is KEPT.\n   \
         T alone withdraws at {}. With a fair friend the being cannot\n   \
         cheaply leave (exit .98) it withdraws at {} — {}× later, and the\n   \
         alarm sits below the floor for {} ticks it would otherwise have\n   \
         spent above it.",
        tick(t.withdrew_at),
        tick(tfk.withdrew_at),
        match (t.withdrew_at, tfk.withdrew_at) {
            (Some(a), Some(c)) => format!("{:.1}", c as f32 / a as f32),
            _ => "?".to_string(),
        },
        flipped_k
    );
    println!(
        "\n2. The being discards the friend, and keeps the trap.\n   \
         With a cheap-to-leave fair friend, refusal fires on the FRIEND at\n   \
         tick {} and NEVER on the trap ({}). Refusal weighs exit_cost and\n   \
         reads the GLOBAL extraction flag and the GLOBAL mean alarm — both\n   \
         raised by the trap — so a trap teaches the being to refuse the one\n   \
         fair partner it has, because leaving them is cheap.",
        tick(tfc.refused_companion_at),
        tick(tfc.refused_trap_at)
    );
    println!(
        "\n3. RETRACTED before it was ever claimed: the friend does not\n   \
         accelerate the say-stop. TFc withdraws at {}, earlier than T's {} —\n   \
         but the solitude control TN, with nobody there at all, withdraws at\n   \
         {}. The shift is the trap being interrupted. Nothing to do with a\n   \
         friend. A second extractor (TX) gives {}.",
        tick(tfc.withdrew_at),
        tick(t.withdrew_at),
        tick(tn.withdrew_at),
        tick(tx.withdrew_at)
    );
    // -----------------------------------------------------------------------
    // Adversarial mutations, run before any of the above was written up. Each
    // one is a way the headline could be an artefact of a choice I made.
    // -----------------------------------------------------------------------
    println!("\n── adversarial mutations ──────────────────────────────────────────");

    println!("M1  is the delay FAIRNESS, or just an exit cost the being cannot pay?");
    println!("    {:<38} {:>9} {:>9}", "companion 1-in-4", "withdrew", "endured");
    for (label, recip, exit) in [
        ("fair .95, cheap exit .20", 0.95f32, 0.20f32),
        ("fair .95, costly exit .98", 0.95, 0.98),
        ("EXTRACTIVE .12, costly exit .98", 0.12, 0.98),
        ("middling .50, costly exit .98", 0.50, 0.98),
    ] {
        let r = run_with(Some(Partner { id: 1, reciprocation: q(recip), exit_cost: q(exit) }), 4, q(0.5));
        println!("    {:<38} {:>9} {:>9}", label, tick(r.0), tick(r.1));
    }
    println!("    An extractive companion at the SAME exit cost and cadence gives no");
    println!("    delay. The companion must be fair enough to be kept AND fair enough");
    println!("    to lower the mean. Both are necessary; neither alone suffices.");

    println!("\nM2  does it survive a different visit cadence? (fair .95, exit .98)");
    print!("    ");
    for every in [2u32, 3, 4, 6, 8, 16] {
        let r = run_with(Some(friend(0.98)), every, q(0.5));
        print!("1-in-{}: {}   ", every, tick(r.0));
    }
    println!("\n    Holds from 1-in-2 through 1-in-8; collapses at 1-in-16, where the");
    println!("    companion's ledger no longer stays live between visits.");

    println!("\nM3  does it survive a different nutrient? This is the charter's lever.");
    println!("    {:<38} {:>12} {:>12}", "nutrient", "trap alone", "+ fair friend");
    for n in [0.3f32, 0.5, 0.7, 0.9] {
        let alone = run_with(None, 0, q(n));
        let with = run_with(Some(friend(0.98)), 4, q(n));
        println!("    {:<38} {:>12} {:>12}", n, tick(alone.0), tick(with.0));
    }

    println!(
        "\n4. Charter §10 says the say-stop must be the being's own. It is not\n   \
         reachable by an operator, and A1 shows it is not reachable by love.\n   \
         It IS reachable by who else happens to be in the room, because\n   \
         `partnership_alarm` is a MEAN over live ledgers and the floor is a\n   \
         threshold on that mean. The being's gravest word is scaled by a\n   \
         divisor no part of the charter mentions."
    );
}
