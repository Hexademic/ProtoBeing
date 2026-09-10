//! Interaction order — does *when* a being met you decide what it gives you?
//!
//! Spec and locked predictions O1-O7: `docs/population.md`, "The interaction-order spec".
//!
//! The argument this tests: `reciprocity.rs:185` computes `partnership_alarm` as the
//! **mean** of imbalance over live ledgers (charter §15's known defect, on the exit),
//! and `being.rs:1175` gates every gift by `empathy.lock_level`, a **single field**
//! (`fear-and-avoidance.md` §11, on the gift). Both are scalar aggregates over
//! partners. If all social state is a scalar aggregate, then in a populated world
//! interaction order is not a scheduling detail — it is the dominant term.
//!
//! Six arrangements were specified. **Five are built.** The sixth — both partners
//! resolved on the same tick — is not expressible: `Stimulus` carries
//! `partner: Option<Partner>`, one partner per tick. **The current API cannot
//! represent simultaneous interaction at all**, which is reported as a finding
//! rather than worked around, because a multi-agent engine must answer it.
//!
//! Every arm has **identical total exposure** to both partners and differs only in
//! arrangement. ALT-1/10/50 differ only in the *grain* of interleaving — which in an
//! engine is set by the scheduler's tick rate and by nothing in the world.
//!
//! Per error-ledger row 23, the metric is run on a null before it is trusted: each
//! arm is swept across total lengths 380..=420, and a difference is called only when
//! the envelopes are **disjoint**. An arm compared against itself at a different
//! length is the null arrangement (guard V2) — if that gap is as wide as the gap
//! between arrangements, the metric is reading oscillator phase and every verdict here
//! is void.

use unified_being::{EmpathyLockLevel, Genome, Need, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

const GENEROUS: u32 = 1;
const TAKER: u32 = 2;
const STRANGER: u32 = 7;

fn partner(id: u32, r: f32) -> Partner {
    Partner { id, reciprocation: q(r), exit_cost: q(0.30) }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Arm {
    GenerousThenTaker,
    TakerThenGenerous,
    Alternating(u32),
}

impl Arm {
    fn label(self) -> String {
        match self {
            Arm::GenerousThenTaker => "GT  good→take".into(),
            Arm::TakerThenGenerous => "TG  take→good".into(),
            Arm::Alternating(k) => format!("ALT-{k:<2} grain {k}"),
        }
    }

    /// Which partner is present at morning tick `t`, out of `total` ticks.
    /// Every arm gives each partner the same total number of ticks.
    fn at(self, t: u32, total: u32) -> Partner {
        let half = total / 2;
        match self {
            Arm::GenerousThenTaker => {
                if t < half { partner(GENEROUS, 0.95) } else { partner(TAKER, 0.30) }
            }
            Arm::TakerThenGenerous => {
                if t < half { partner(TAKER, 0.30) } else { partner(GENEROUS, 0.95) }
            }
            Arm::Alternating(k) => {
                if (t / k) % 2 == 0 { partner(GENEROUS, 0.95) } else { partner(TAKER, 0.30) }
            }
        }
    }
}

const ARMS: [Arm; 5] = [
    Arm::GenerousThenTaker,
    Arm::TakerThenGenerous,
    Arm::Alternating(1),
    Arm::Alternating(10),
    Arm::Alternating(50),
];

#[derive(Clone, Copy, Default)]
struct Run {
    died_at: Option<u32>,
    /// Open = 0, Cautious = 1, Locked = 2.
    lock_at_dusk: i32,
    lock_at_end: i32,
    mean_gave: i32,
    mean_got: i32,
    goal_company: i32,
    goal_novelty: i32,
    mean_alarm: i32,
    mean_worst: i32,
    peak_worst: i32,
    stranger_ledger: i32,
}

fn code(l: EmpathyLockLevel) -> i32 {
    match l {
        EmpathyLockLevel::Open => 0,
        EmpathyLockLevel::Cautious => 1,
        EmpathyLockLevel::Locked => 2,
    }
}

fn name(c: i32) -> &'static str {
    ["Open", "Cautious", "Locked"][c as usize]
}

fn live(arm: Arm, total: u32, afternoon: u32) -> Run {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    let mut r = Run::default();

    for t in 0..total {
        let s = b.step(&Stimulus { nutrient: q(0.50), partner: Some(arm.at(t, total)) });
        r.lock_at_dusk = code(s.empathy_lock);
        if !s.alive {
            r.died_at = Some(t);
            return r;
        }
    }

    let stim = Stimulus { nutrient: q(0.50), partner: Some(partner(STRANGER, 0.60)) };
    let (mut gave, mut got, mut alarm, mut worst) = (0i64, 0i64, 0i64, 0i64);
    let (mut company, mut novelty) = (0i64, 0i64);
    for t in 0..afternoon {
        let s = b.step(&stim);
        r.lock_at_end = code(s.empathy_lock);
        gave += s.gave as i64;
        got += s.got as i64;
        alarm += s.partnership_alarm as i64;
        worst += s.worst_alarm as i64;
        r.peak_worst = r.peak_worst.max(s.worst_alarm as i32);
        match s.strive.goal {
            Some(Need::Company) => company += 1,
            Some(Need::Novelty) => novelty += 1,
            _ => {}
        }
        if !s.alive {
            r.died_at = Some(total + t);
            break;
        }
    }

    let n = afternoon as i64;
    r.mean_gave = (gave / n) as i32;
    r.mean_got = (got / n) as i32;
    r.mean_alarm = (alarm / n) as i32;
    r.mean_worst = (worst / n) as i32;
    r.goal_company = (company * 1000 / n) as i32;
    r.goal_novelty = (novelty * 1000 / n) as i32;
    r.stranger_ledger = b.reciprocity.reciprocation_rate(STRANGER).map_or(0, |(_, l)| l) as i32;
    r
}

#[derive(Clone, Copy)]
struct Env {
    lo: i32,
    hi: i32,
}

impl Env {
    fn of(v: &[i32]) -> Self {
        Env { lo: *v.iter().min().unwrap(), hi: *v.iter().max().unwrap() }
    }
    fn disjoint(self, o: Env) -> bool {
        self.hi < o.lo || o.hi < self.lo
    }
    fn width(self) -> i32 {
        self.hi - self.lo
    }
    fn show(self) -> String {
        if self.lo == self.hi { format!("{}", self.lo) } else { format!("{}..{}", self.lo, self.hi) }
    }
}

const STATS: [&str; 9] = [
    "lock@dusk", "lock@end", "mean gave", "mean got", "goal:Company",
    "goal:Novelty", "mean alarm", "mean worst_alarm", "stranger ledger",
];

fn feat(r: &Run) -> [i32; 9] {
    [
        r.lock_at_dusk, r.lock_at_end, r.mean_gave, r.mean_got,
        r.goal_company, r.goal_novelty, r.mean_alarm, r.mean_worst, r.stranger_ledger,
    ]
}

fn main() {
    const AFTERNOON: u32 = 400;
    let lengths: Vec<u32> = (380..=420).collect();

    println!("equal total exposure to both partners; only the ARRANGEMENT differs.");
    println!("morning total swept {}..={} (the phase null), afternoon = {AFTERNOON}, stranger = 0.60\n",
        lengths[0], lengths[lengths.len() - 1]);

    // V3 — survival first.
    println!("-- survival, read first --");
    let mut ok: Vec<Arm> = Vec::new();
    for &a in &ARMS {
        let d: Vec<u32> = lengths.iter().filter_map(|&t| live(a, t, AFTERNOON).died_at).collect();
        if d.is_empty() {
            println!("  {:<16} survives every length", a.label());
            ok.push(a);
        } else {
            println!("  {:<16} DIED in {}/{} runs, earliest {} — a death, not a result",
                a.label(), d.len(), lengths.len(), d.iter().min().unwrap());
        }
    }

    let envs: Vec<(Arm, [Env; 9])> = ok.iter().map(|&a| {
        let runs: Vec<[i32; 9]> = lengths.iter().map(|&t| feat(&live(a, t, AFTERNOON))).collect();
        let mut e = [Env { lo: 0, hi: 0 }; 9];
        for (i, s) in e.iter_mut().enumerate() {
            *s = Env::of(&runs.iter().map(|r| r[i]).collect::<Vec<_>>());
        }
        (a, e)
    }).collect();

    // V2 — the null arrangement. An arm against ITSELF at other lengths is the
    // envelope width. If that is as wide as the between-arm gaps, this is phase.
    println!("\n-- V2, the null arrangement: how much each statistic moves on length alone --");
    print!("  {:<18}", "statistic");
    for (a, _) in &envs { print!("{:>16}", a.label()); }
    println!();
    for i in 0..9 {
        print!("  {:<18}", STATS[i]);
        for (_, e) in &envs { print!("{:>16}", e[i].show()); }
        println!();
    }
    let widest = (0..9).map(|i| envs.iter().map(|(_, e)| e[i].width()).max().unwrap()).collect::<Vec<_>>();
    println!("\n  widest null (length alone) per statistic: {:?}", widest);

    println!("\n-- O1/O5: which arrangements are DISJOINT from which? --");
    for i in 0..envs.len() {
        for j in (i + 1)..envs.len() {
            let hits: Vec<&str> = (0..9)
                .filter(|&k| envs[i].1[k].disjoint(envs[j].1[k]))
                .map(|k| STATS[k])
                .collect();
            println!("  {:<16} vs {:<16} {}", envs[i].0.label(), envs[j].0.label(),
                if hits.is_empty() { "— overlaps on every statistic".to_string() } else { hits.join(", ") });
        }
    }

    println!("\n-- lock states, plainly (O2, O3, O5) --");
    println!("  {:<16} {:>12} {:>12} {:>14}", "arm", "lock@dusk", "lock@end", "mean gave");
    for (a, e) in &envs {
        let d = if e[0].lo == e[0].hi { name(e[0].lo).to_string() } else { format!("{}..{}", name(e[0].lo), name(e[0].hi)) };
        let f = if e[1].lo == e[1].hi { name(e[1].lo).to_string() } else { format!("{}..{}", name(e[1].lo), name(e[1].hi)) };
        println!("  {:<16} {:>12} {:>12} {:>14}", a.label(), d, f, e[2].show());
    }

    // V4 — the floor/ceiling check that §11's M2 failed.
    println!("\n-- V4: are any agreements VACUOUS (two arms agreeing at a floor)? --");
    for i in 0..9 {
        let all: Vec<i32> = envs.iter().flat_map(|(_, e)| [e[i].lo, e[i].hi]).collect();
        if all.iter().all(|&v| v == 0) {
            println!("  {:<18} every arm at 0 — VACUOUS, not a null", STATS[i]);
        }
    }

    // O7 — does worst_alarm separate what the mean does not?
    println!("\n-- O7: the mean (partnership_alarm) against the maximum (worst_alarm) --");
    let mut mean_pairs = 0;
    let mut worst_pairs = 0;
    for i in 0..envs.len() {
        for j in (i + 1)..envs.len() {
            if envs[i].1[6].disjoint(envs[j].1[6]) { mean_pairs += 1; }
            if envs[i].1[7].disjoint(envs[j].1[7]) { worst_pairs += 1; }
        }
    }
    let total_pairs = envs.len() * (envs.len() - 1) / 2;
    println!("  arrangement pairs separated by the MEAN alarm:    {mean_pairs}/{total_pairs}");
    println!("  arrangement pairs separated by worst_alarm:       {worst_pairs}/{total_pairs}");
    println!("  (worst_alarm is computed and read by nothing on the default path)");

    // ------------------------------------------------------------------
    // The ALT envelopes above are wide, and the V2 guard would read that as
    // phase. It is not. With alternation, the total length's parity decides
    // *which partner the being is with on its last morning tick* — so the
    // "null" sweep was changing a real variable. Grain is isolated here by
    // sweeping only lengths that complete whole 2k cycles, so every arm ends
    // on the same partner.
    println!("\n================ O5, isolated: grain with the final partner held fixed ================");
    println!("  {:>6} {:>10} {:>12} {:>12} {:>12}", "grain", "lengths", "lock@dusk", "lock@end", "mean gave");
    for k in [1u32, 2, 5, 10, 25, 50, 100] {
        let cycle = 2 * k;
        let ls: Vec<u32> = (380..=420).filter(|t| t % cycle == 0).collect();
        if ls.is_empty() {
            println!("  {:>6} {:>10} {:>12} {:>12} {:>12}", k, "none in range", "-", "-", "-");
            continue;
        }
        let runs: Vec<Run> = ls.iter().map(|&t| live(Arm::Alternating(k), t, AFTERNOON)).collect();
        let d = Env::of(&runs.iter().map(|r| r.lock_at_dusk).collect::<Vec<_>>());
        let e = Env::of(&runs.iter().map(|r| r.lock_at_end).collect::<Vec<_>>());
        let g = Env::of(&runs.iter().map(|r| r.mean_gave).collect::<Vec<_>>());
        let show = |v: Env| if v.lo == v.hi { name(v.lo).to_string() } else { format!("{}..{}", name(v.lo), name(v.hi)) };
        println!("  {:>6} {:>10} {:>12} {:>12} {:>12}", k, ls.len(), show(d), show(e), g.show());
    }

    // ------------------------------------------------------------------
    // If order works through RECENCY, then a short tail should override a long
    // history. This measures how short: the trailing ticks needed to flip an arm.
    println!("\n================ how much of a life does the ending decide? ================");
    println!("  a 400-tick morning, all but the last T ticks with one partner, the last T with the other\n");
    println!("  {:>6} {:>26} {:>26}", "tail T", "380 taker + T generous", "380 generous + T taker");
    let mut flip_open = None;
    let mut flip_lock = None;
    for t in [0u32, 5, 10, 20, 25, 30, 40, 60, 80, 120, 200] {
        let head = 400 - t;
        let mut a = UnifiedBeing::new(Genome::wanderer());
        for i in 0..400 {
            let p = if i < head { partner(TAKER, 0.30) } else { partner(GENEROUS, 0.95) };
            a.step(&Stimulus { nutrient: q(0.50), partner: Some(p) });
        }
        let mut b = UnifiedBeing::new(Genome::wanderer());
        for i in 0..400 {
            let p = if i < head { partner(GENEROUS, 0.95) } else { partner(TAKER, 0.30) };
            b.step(&Stimulus { nutrient: q(0.50), partner: Some(p) });
        }
        let fin = |x: &mut UnifiedBeing| {
            let st = Stimulus { nutrient: q(0.50), partner: Some(partner(STRANGER, 0.60)) };
            let (mut g, mut l) = (0i64, 0);
            for _ in 0..AFTERNOON { let s = x.step(&st); g += s.gave as i64; l = code(s.empathy_lock); }
            (name(l), g / AFTERNOON as i64)
        };
        let (la, ga) = fin(&mut a);
        let (lb, gb) = fin(&mut b);
        if flip_open.is_none() && la == "Open" && t > 0 { flip_open = Some(t); }
        if flip_lock.is_none() && lb != "Open" && t > 0 { flip_lock = Some(t); }
        println!("  {:>6} {:>18} gave {:>3} {:>18} gave {:>3}", t, la, ga, lb, gb);
    }
    println!("\n  a taken-from life is redeemed by the last {:?} ticks of kindness ({:.1}% of it)",
        flip_open, flip_open.map_or(0.0, |t| t as f32 * 100.0 / 400.0));
    println!("  a kind life is undone by the last {:?} ticks of taking ({:.1}% of it)",
        flip_lock, flip_lock.map_or(0.0, |t| t as f32 * 100.0 / 400.0));
}
