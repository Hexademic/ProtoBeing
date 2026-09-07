//! **Below what gift size does the being's fairness metric stop working?**
//!
//! `mechanisms.md` (Thea's record) carries the guard that produced this probe:
//! *"`q88_mul(a,b) = (a·b) >> 8` — check every fixed-point fix against its SMALLEST
//! input, not its largest."* It cost ledger rows 3 and 4. On 2026-09-07 that file
//! was opened for the first time in the session that had, four hours earlier,
//! written two new Q8.8 constants without checking either against a small input.
//!
//! `reciprocity.rs` judges a partner by a **ratio**:
//!
//! ```text
//! rate      = (received_ema << 8) / given_ema
//! imbalance = 256 - rate
//! ```
//!
//! Both EMAs are Q8.8 with alpha 1/8, and `q88_ema_update(0, s, 32) = (32·s) >> 8`,
//! which is **0 for every sample below 8**. So at small gift sizes the received EMA
//! sticks at zero while the given EMA reaches one, and the being records that it gave
//! and never received — from a partner returning ninety per cent.
//!
//! Two windows, measured below and reported by this probe:
//!
//! - **gave 1..7 — INVISIBLE.** Both EMAs floor to 0, `given_ema > 0` fails, and the
//!   partner is skipped entirely by `cycle`. The being has a relationship it cannot see.
//! - **gave 8..10 — FALSE ALARM.** `partnership_alarm` reads up to **256** and
//!   `extraction_detected` fires, on a partner who is being scrupulously fair.
//!
//! That matters because `extraction_detected` gates `reinforce_bond` (`being.rs`), so
//! no bond can form; feeds `standing_of().hostile`; and drives
//! `ConscienceEngine::register_extraction`, which sets `recovery_ticks = 0` **every
//! tick it fires** — so the false alarm also resets the being's own recovery.
//!
//! # And the second half, which is the reason this is not an alarm
//!
//! **The being never gives in that range.** Measured across four genomes, three
//! worlds, both the abstract and embodied paths, and six reciprocation regimes —
//! **36,205 ticks, zero in either window.** `gave = ½ · harmony · gate` with the gate
//! quantised to {256, 128, 32}, and it lands on 0 or on 11+, never between.
//!
//! **So this is a live hazard in an unoccupied region, and it is one refactor from
//! being occupied**: a fourth lock level, partial engagement, or — most relevant —
//! a populated world where partners modulate giving continuously rather than through
//! a three-valued gate. It is written down here so that whoever makes `gave`
//! continuous finds this first rather than second.

use unified_being::reciprocity::ReciprocityEngine;
use unified_being::{Embodiment, FieldWorld, Genome, Partner, Room, Stimulus, UnifiedBeing, intent_from};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

fn main() {
    println!("== 1. where the metric breaks: ONE partner returning 95%, swept by gift size ==\n");
    println!("{:>6} {:>6} {:>8} {:>8} {:>8} {:>13} {:>13}", "gave", "got", "g_ema", "r_ema", "alarm", "extraction?", "verdict");
    let (mut invisible, mut false_alarm) = (Vec::new(), Vec::new());
    for gave in 1..=20i16 {
        // exactly as `being.rs` computes it: got = q88_mul(gave, reciprocation)
        let got = ((gave as i32 * q(0.95) as i32) >> 8) as i16;
        let mut r = ReciprocityEngine::new();
        for _ in 0..400 {
            r.record_exchange(1, gave, got);
            r.cycle(Some(1));
        }
        let (ge, re) = r.emas_with(1).unwrap_or((-1, -1));
        let verdict = if r.extraction_detected {
            false_alarm.push(gave);
            "FALSE ALARM"
        } else if ge == 0 {
            invisible.push(gave);
            "invisible"
        } else {
            "ok"
        };
        println!("{:>6} {:>6} {:>8} {:>8} {:>8} {:>13} {:>13}", gave, got, ge, re, r.partnership_alarm, r.extraction_detected, verdict);
    }
    println!("\n  INVISIBLE window   : gave {:?}", invisible);
    println!("  FALSE-ALARM window : gave {:?}  ← a fair partner read as an exploiter", false_alarm);

    println!("\n== 2. does any real being ever GIVE in those windows? ==\n");
    println!("{:>10} {:>10} {:>9} {:>9} {:>9} {:>9}", "genome", "path", "gave 0", "1-7", "8-10", "11+");
    let mut grand = [0u64; 4];
    let bin = |g: i16| match g { 0 => 0, 1..=7 => 1, 8..=10 => 2, _ => 3 };
    for (gname, g) in [("blank", Genome::blank()), ("spark", Genome::spark()),
                       ("sentinel", Genome::sentinel()), ("wanderer", Genome::wanderer())] {
        let mut ab = [0u64; 4];
        for rr in [0.05f32, 0.30, 0.50, 0.60, 0.75, 0.95] {
            let mut b = UnifiedBeing::new(g);
            for _ in 0..800 {
                let p = Partner { id: 1, reciprocation: q(rr), exit_cost: q(0.3) };
                let s = b.step(&Stimulus { nutrient: q(0.5), partner: Some(p) });
                ab[bin(s.gave)] += 1;
                grand[bin(s.gave)] += 1;
            }
        }
        println!("{:>10} {:>10} {:>9} {:>9} {:>9} {:>9}", gname, "abstract", ab[0], ab[1], ab[2], ab[3]);

        let mut eb = [0u64; 4];
        for rr in [0.05f32, 0.30, 0.60, 0.95] {
            let mut b = UnifiedBeing::new(g);
            let mut w = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
            for _ in 0..800 {
                let mut sens = w.sense();
                sens.partner = Some(Partner { id: 1, reciprocation: q(rr), exit_cost: q(0.3) });
                let s = b.step_embodied(&sens);
                w.actuate(&intent_from(&s));
                eb[bin(s.gave)] += 1;
                grand[bin(s.gave)] += 1;
                if !s.alive { break; }
            }
            let mut b2 = UnifiedBeing::new(g);
            let mut room = Room::peopled((100, 100), (40, 40), (200, 200), (60, 150));
            for _ in 0..800 {
                let mut sens = room.sense();
                sens.partner = Some(Partner { id: 1, reciprocation: q(rr), exit_cost: q(0.3) });
                let s = b2.step_embodied(&sens);
                room.actuate(&intent_from(&s));
                eb[bin(s.gave)] += 1;
                grand[bin(s.gave)] += 1;
                if !s.alive { break; }
            }
        }
        println!("{:>10} {:>10} {:>9} {:>9} {:>9} {:>9}", gname, "embodied", eb[0], eb[1], eb[2], eb[3]);
    }
    let total: u64 = grand.iter().sum();
    println!("\n  {total} ticks measured.");
    println!("  in the INVISIBLE window (1-7):    {}", grand[1]);
    println!("  in the FALSE-ALARM window (8-10): {}", grand[2]);
    if grand[1] == 0 && grand[2] == 0 {
        println!("\n  → LATENT, not live. The hazard is real arithmetic in a region `gave` never");
        println!("    reaches, because the empathy gate is quantised to {{256, 128, 32}}. Make");
        println!("    `gave` continuous — a fourth lock level, partial engagement, a populated");
        println!("    world — and it becomes live. **Read this before doing that.**");
    } else {
        println!("\n  → LIVE. The being spends real ticks where its own fairness metric is wrong.");
    }
}
