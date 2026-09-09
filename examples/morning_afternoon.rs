//! The same afternoon, five mornings — how much of a morning survives?
//!
//! Spec and locked predictions M1-M6: `docs/fear-and-avoidance.md` §10.
//! Adversarial control and the metric it forced: §11 of the same document.
//!
//! Not a poison probe. §9 measured that this being visits **27 distinct positions
//! in 4,000 ticks**, so every spatial approach-avoidance test here comes back
//! vacuous. The Hans complaint is not about poison; it is about an NPC whose needs
//! are re-auctioned each tick from present state, so yesterday cannot change what
//! he wants today. That is a claim about the **auction**, which can be tested with
//! the body standing still.
//!
//! # Why this probe does not compare goal sequences tick-for-tick
//!
//! The first version of it did, and the first version was wrong. Holding the
//! morning's *content* fixed and changing only its *length* by k ticks produces
//! **exactly k** mismatched afternoon goals — the body is a Van der Pol oscillator
//! (§9), so morning length sets the phase it enters the afternoon at, and a
//! tick-aligned comparison reads that phase as if it were memory. A one-tick
//! shorter morning "remembers" as loudly as an entirely different life.
//!
//! So every read-out here is an **aggregate over the whole afternoon**, and every
//! arm is compared against a **phase-null envelope**: the same arm's own statistic
//! swept across morning lengths 150..=250. Two arms differ only if their envelopes
//! are **disjoint** — a gap no amount of phase can close.

use unified_being::{EmpathyLockLevel, Genome, Need, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

/// The stranger every arm meets in the afternoon — a partner id no morning uses.
/// Reciprocation 0.60 is above `FAIR_RECIPROCITY` (0.5), so it is not a trap.
fn stranger() -> Partner {
    Partner { id: 7, reciprocation: q(0.60), exit_cost: q(0.30) }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Arm {
    Blank,
    Hungry,
    Lonely,
    Betrayed,
    Bereaved,
}

const ARMS: [Arm; 5] = [Arm::Blank, Arm::Hungry, Arm::Lonely, Arm::Betrayed, Arm::Bereaved];

impl Arm {
    fn label(self) -> &'static str {
        match self {
            Arm::Blank => "A blank",
            Arm::Hungry => "B hungry",
            Arm::Lonely => "C lonely",
            Arm::Betrayed => "D betrayed",
            Arm::Bereaved => "E bereaved",
        }
    }

    /// The morning. Every arm that has a partner uses the **same id (1)**, so D
    /// differs from A only in how that partner behaves, not in who it is.
    fn morning(self) -> Stimulus {
        let mate = |r: f32, x: f32| Some(Partner { id: 1, reciprocation: q(r), exit_cost: q(x) });
        match self {
            Arm::Blank => Stimulus { nutrient: q(0.50), partner: mate(0.60, 0.30) },
            Arm::Hungry => Stimulus { nutrient: q(0.08), partner: mate(0.60, 0.30) },
            Arm::Lonely => Stimulus { nutrient: q(0.50), partner: None },
            Arm::Betrayed => Stimulus { nutrient: q(0.50), partner: mate(0.05, 0.98) },
            Arm::Bereaved => Stimulus { nutrient: q(0.50), partner: mate(0.95, 0.15) },
        }
    }
}

/// Everything read out of one afternoon. Aggregates only — nothing tick-aligned.
#[derive(Clone, Copy, Debug, Default)]
struct Stats {
    alive: bool,
    died_at: Option<u32>,
    ticks: u32,
    /// Share of afternoon ticks spent striving for each need, in tenths of a percent.
    goal_share: [i32; 5], // Sustenance, Company, Novelty, Purpose, no-goal
    mean_urgency: i32,
    mean_gave: i32,
    mean_got: i32,
    mean_longing: i32,
    /// Ticks logged in the ledger for the stranger at the end. Zero means the being
    /// withdrew from them (`reciprocity.rs::withdraw` deactivates the ledger).
    stranger_lived: u16,
    stranger_trust: i16,
    stranger_hostile: bool,
    peak_alarm: i16,
}

fn live(arm: Arm, morning_ticks: u32, afternoon_ticks: u32) -> Stats {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    let mut s = Stats { alive: true, ..Default::default() };

    for t in 0..morning_ticks {
        if !b.step(&arm.morning()).alive {
            s.alive = false;
            s.died_at = Some(t);
            return s;
        }
    }

    let stim = Stimulus { nutrient: q(0.50), partner: Some(stranger()) };
    let (mut urg, mut gave, mut got, mut longing) = (0i64, 0i64, 0i64, 0i64);
    let mut counts = [0i32; 5];

    for t in 0..afternoon_ticks {
        let r = b.step(&stim);
        s.ticks += 1;
        counts[match r.strive.goal {
            Some(Need::Sustenance) => 0,
            Some(Need::Company) => 1,
            Some(Need::Novelty) => 2,
            Some(Need::Purpose) => 3,
            None => 4,
        }] += 1;
        urg += r.strive.urgency as i64;
        gave += r.gave as i64;
        got += r.got as i64;
        longing += r.attach.longing as i64;
        s.peak_alarm = s.peak_alarm.max(r.partnership_alarm);
        s.stranger_hostile |= b.standing_of(7).hostile;
        if !r.alive {
            s.alive = false;
            s.died_at = Some(morning_ticks + t);
            break;
        }
    }

    let n = s.ticks.max(1) as i64;
    for i in 0..5 {
        s.goal_share[i] = (counts[i] as i64 * 1000 / n) as i32;
    }
    s.mean_urgency = (urg / n) as i32;
    s.mean_gave = (gave / n) as i32;
    s.mean_got = (got / n) as i32;
    s.mean_longing = (longing / n) as i32;
    s.stranger_trust = b.standing_of(7).trust;
    s.stranger_lived = b.reciprocity.reciprocation_rate(7).map_or(0, |(_, l)| l);
    s
}

/// A statistic's range across the phase sweep: what it can be made to do by morning
/// length alone. Two arms differ **only** if their envelopes do not overlap.
#[derive(Clone, Copy, Debug)]
struct Envelope {
    lo: i32,
    hi: i32,
}

impl Envelope {
    fn of(vals: &[i32]) -> Self {
        Envelope { lo: *vals.iter().min().unwrap(), hi: *vals.iter().max().unwrap() }
    }
    fn disjoint(self, o: Envelope) -> bool {
        self.hi < o.lo || o.hi < self.lo
    }
    fn show(self) -> String {
        if self.lo == self.hi { format!("{}", self.lo) } else { format!("{}..{}", self.lo, self.hi) }
    }
}

const NAMES: [&str; 10] = [
    "goal:Sustenance", "goal:Company", "goal:Novelty", "goal:Purpose", "goal:none",
    "mean urgency", "mean gave", "mean got", "mean longing", "stranger lived",
];

fn features(s: &Stats) -> [i32; 10] {
    [
        s.goal_share[0], s.goal_share[1], s.goal_share[2], s.goal_share[3], s.goal_share[4],
        s.mean_urgency, s.mean_gave, s.mean_got, s.mean_longing, s.stranger_lived as i32,
    ]
}

fn main() {
    const AFTERNOON: u32 = 400;
    const SWEEP: std::ops::RangeInclusive<u32> = 150..=250;

    println!("morning length swept over {:?} (the phase-null), afternoon = {AFTERNOON}\n", SWEEP);

    // ---- survival first (ledger row 21): a death is a death, never an effect size.
    println!("-- survival, read first --");
    let mut usable: Vec<Arm> = Vec::new();
    for &a in &ARMS {
        let deaths: Vec<u32> = SWEEP
            .clone()
            .filter_map(|m| live(a, m, AFTERNOON).died_at)
            .collect();
        if deaths.is_empty() {
            println!("  {:<12} survives every morning length", a.label());
            usable.push(a);
        } else {
            println!(
                "  {:<12} DIED in {}/{} runs, earliest tick {}  — reported as a death, not a result",
                a.label(),
                deaths.len(),
                SWEEP.clone().count(),
                deaths.iter().min().unwrap()
            );
        }
    }

    // ---- the envelopes
    let mut envs: Vec<(Arm, [Envelope; 10])> = Vec::new();
    for &a in &usable {
        let runs: Vec<[i32; 10]> = SWEEP.clone().map(|m| features(&live(a, m, AFTERNOON))).collect();
        let mut e = [Envelope { lo: 0, hi: 0 }; 10];
        for (i, slot) in e.iter_mut().enumerate() {
            let col: Vec<i32> = runs.iter().map(|r| r[i]).collect();
            *slot = Envelope::of(&col);
        }
        envs.push((a, e));
    }

    println!("\n-- what a whole morning can move, against what phase alone can move --");
    println!("  (goal shares are tenths of a percent of the afternoon)\n");
    print!("  {:<16}", "statistic");
    for (a, _) in &envs {
        print!("{:>18}", a.label());
    }
    println!();
    for i in 0..10 {
        print!("  {:<16}", NAMES[i]);
        for (_, e) in &envs {
            print!("{:>18}", e[i].show());
        }
        println!();
    }

    println!("\n-- disjoint from the blank morning? (the only claim phase cannot explain) --");
    let base = envs[0].1;
    for (a, e) in envs.iter().skip(1) {
        let hits: Vec<&str> = (0..10).filter(|&i| e[i].disjoint(base[i])).map(|i| NAMES[i]).collect();
        if hits.is_empty() {
            println!("  {:<12} nothing — every statistic overlaps the blank morning's own phase range", a.label());
        } else {
            println!("  {:<12} {}", a.label(), hits.join(", "));
            for i in 0..10 {
                if e[i].disjoint(base[i]) {
                    println!("      {:<16} A: {:<12} vs {:<12}", NAMES[i], base[i].show(), e[i].show());
                }
            }
        }
    }

    println!("\n-- the stranger, categorically (a ledger either forms or it does not) --");
    println!("  {:<12} {:>14} {:>14} {:>16}", "arm", "bonded runs", "trust range", "ever hostile");
    for &a in &usable {
        let runs: Vec<Stats> = SWEEP.clone().map(|m| live(a, m, AFTERNOON)).collect();
        let bonded = runs.iter().filter(|s| s.stranger_lived > 0).count();
        let trust: Vec<i32> = runs.iter().map(|s| s.stranger_trust as i32).collect();
        let hostile = runs.iter().filter(|s| s.stranger_hostile).count();
        println!(
            "  {:<12} {:>14} {:>14} {:>16}",
            a.label(),
            format!("{}/{}", bonded, runs.len()),
            Envelope::of(&trust).show(),
            format!("{}/{}", hostile, runs.len())
        );
    }

    // ---- What actually carries the morning across, and how long it lasts.
    //
    // `gave` is not gated by appetite. `being.rs:1175` computes it as
    // `0.5 · action_harmony(basin) · empathy_gate`, where the gate is 256 / 128 / 32
    // for Open / Cautious / Locked. So the register that carries a morning into an
    // afternoon is the **empathy lock** — a disposition, held across partners, with
    // no per-partner discrimination in the scoring (only in eligibility, via
    // `is_refused`). This section measures its hysteresis.
    println!("\n================ the carrier: empathy-lock hysteresis ================");
    println!("  a 200-tick morning with partner 1, then a kind stranger (0.95, id 7)\n");
    println!(
        "  {:>6} {:>10} {:>10} {:>12} {:>16}",
        "recip", "lock@dusk", "coercion", "mean |g-g|", "ticks to reopen"
    );
    for i in 0..=20 {
        let r = i as f32 / 20.0;
        let mut b = UnifiedBeing::new(Genome::wanderer());
        let m = Stimulus {
            nutrient: q(0.50),
            partner: Some(Partner { id: 1, reciprocation: q(r), exit_cost: q(0.30) }),
        };
        let (mut dusk, mut err) = (EmpathyLockLevel::Open, 0i64);
        for _ in 0..200 {
            let s = b.step(&m);
            dusk = s.empathy_lock;
            err += (s.gave - s.got).abs() as i64;
        }
        let coercion = b.conscience.constitutional_load().coercion;
        let kind = Stimulus {
            nutrient: q(0.50),
            partner: Some(Partner { id: 7, reciprocation: q(0.95), exit_cost: q(0.30) }),
        };
        let mut reopen = None;
        for t in 0..4000u32 {
            if b.step(&kind).empathy_lock == EmpathyLockLevel::Open {
                reopen = Some(t);
                break;
            }
        }
        println!(
            "  {:>6.2} {:>10?} {:>10} {:>12} {:>16}",
            r,
            dusk,
            coercion,
            err / 200,
            match reopen {
                Some(0) => "already open".to_string(),
                Some(k) => k.to_string(),
                None => ">4000".to_string(),
            }
        );
    }

    // The other direction: does a generous morning ever wear off under a merely
    // fair afternoon? Reported as a bound, not as "permanent".
    println!("\n  the reverse — a generous morning (0.95) against a long fair afternoon (0.60):");
    let mut b = UnifiedBeing::new(Genome::wanderer());
    let m = Stimulus {
        nutrient: q(0.50),
        partner: Some(Partner { id: 1, reciprocation: q(0.95), exit_cost: q(0.30) }),
    };
    for _ in 0..200 {
        b.step(&m);
    }
    let fair = Stimulus { nutrient: q(0.50), partner: Some(stranger()) };
    let mut closed_at = None;
    for t in 0..8000u32 {
        if b.step(&fair).empathy_lock != EmpathyLockLevel::Open {
            closed_at = Some(t);
            break;
        }
    }
    match closed_at {
        Some(t) => println!("    closes at afternoon tick {t}"),
        None => println!("    does not close within 8,000 afternoon ticks"),
    }
}
