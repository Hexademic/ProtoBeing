//! **The reaction rate** — how often does the being cross between its metastable states?
//!
//! **Predictions RR-1..RR-5 are locked in `docs/c1-relabelling.md` §13 and were committed before
//! this file existed.**
//!
//! Du et al., *Rare Event Analysis via Stochastic Optimal Control* (arXiv:2604.13213), read in
//! full, supplied the vocabulary this project had been missing. **`Basin` is a metastable-state
//! variable**, measured at 99.9% one value — which in Transition Path Theory's terms is *a system
//! with a reaction rate near zero.* We had never computed the rate.
//!
//! **Their method cannot reach us** (§G.3: the controlled kernel is a Boltzmann tilt of the
//! reference kernel, which cancels for a deterministic Dirac, collapsing the control space) — and
//! **the rate needs none of it.** Their eq. 316 is `ν_R = lim N_T/T`, the frequency of transitions
//! at stationarity. We count crossings.
//!
//! **A rate is diagnostic, not a score.** A high rate may be thrashing and a low one may be
//! stability. The **transition graph** and the **net current** are what say which — TPT's reactive
//! current asks whether there is net flux from A to B or merely reversible churn.
//!
//! The RANDOM arm is the floor: the same body, the same tick loop, motor intent drawn uniformly.
//! It is not the being, so it inherits none of its history.
//!
//! Pure observer: fresh beings, no journal written, **the founded being is never woken.**
//!
//! Run: `cargo run --release --example reaction_rate`

use unified_being::basins::Basin;
use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment, MotorIntent, Posture};
use unified_being::room::Room;
use unified_being::striving::Need;

const LIFE: usize = 4_000;
const N_BASIN: usize = 4;

/// The same deterministic LCG the other probes use, reused rather than rewritten.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

/// **One copy of the world**, `Room::with_contingency()` — gated and default-off in `src/room.rs`.
/// Both arms use the room's own proximity rule for company; the static arm does **not** force a
/// permanent partner. That asymmetry is what flipped two verdicts in
/// `operational-consciousness.md` §8.6, and not repeating it is deliberate.
fn room(contingent: bool) -> Room {
    let r = Room::peopled((32, 200), (224, 56), (128, 220), (40, 40)).with_friend((210, 128));
    if contingent { r.with_contingency() } else { r }
}

fn basin_ix(b: Basin) -> usize {
    match b {
        Basin::Rest => 0,
        Basin::Engaged => 1,
        Basin::Defensive => 2,
        Basin::Recovery => 3,
    }
}

const BASIN_NAME: [&str; N_BASIN] = ["Rest", "Engaged", "Defensive", "Recovery"];

struct Run {
    name: String,
    alive: bool,
    ticks: usize,
    /// `edge[from][to]` — the transition graph. Diagonal entries are dwell, not transitions.
    edge: [[u32; N_BASIN]; N_BASIN],
    occupancy: [u32; N_BASIN],
    /// The tick of every basin CHANGE, in order. Added after the run, per the rule that a data
    /// column may be added to clarify but a verdict may never be retro-fitted: RR-1..RR-5 are
    /// scored exactly as locked. Two crossings look very different at tick 50 and at tick 3,900.
    changed_at: Vec<usize>,
}

impl Run {
    fn transitions(&self) -> u32 {
        let mut n = 0;
        for i in 0..N_BASIN {
            for j in 0..N_BASIN {
                if i != j {
                    n += self.edge[i][j];
                }
            }
        }
        n
    }
    /// ν_R — their eq. (316), transitions per tick.
    fn rate(&self) -> f64 {
        self.transitions() as f64 / self.ticks.max(1) as f64
    }
    fn basins_visited(&self) -> usize {
        self.occupancy.iter().filter(|&&c| c > 0).count()
    }
    /// The dominant ordered pair and its reverse — for the net-current question.
    fn dominant_pair(&self) -> (usize, usize, u32, u32) {
        let (mut bi, mut bj, mut best) = (0, 0, 0);
        for i in 0..N_BASIN {
            for j in 0..N_BASIN {
                if i != j && self.edge[i][j] > best {
                    best = self.edge[i][j];
                    bi = i;
                    bj = j;
                }
            }
        }
        (bi, bj, self.edge[bi][bj], self.edge[bj][bi])
    }
    /// |forward − reverse| ÷ total, as a percentage. Near zero is reversible churn.
    fn net_current_pct(&self) -> f64 {
        let (_, _, f, r) = self.dominant_pair();
        let total = f + r;
        if total == 0 {
            return 0.0;
        }
        100.0 * (f as f64 - r as f64).abs() / total as f64
    }
}

fn new_run(name: &str) -> Run {
    Run {
        name: name.into(),
        alive: true,
        ticks: 0,
        edge: [[0; N_BASIN]; N_BASIN],
        occupancy: [0; N_BASIN],
        changed_at: Vec::new(),
    }
}

fn live_being(name: &str, contingent: bool, gates: fn(&mut UnifiedBeing)) -> Run {
    let mut b = UnifiedBeing::new(unified_being::genome::Genome::wanderer());
    gates(&mut b);
    let mut w = room(contingent);
    let mut r = new_run(name);
    let mut prev: Option<usize> = None;

    for _ in 0..LIFE {
        let sens = w.sense();
        let rep = b.step_embodied(&sens);
        w.actuate(&intent_from(&rep));
        w.remember(rep.gave, rep.got);

        let cur = basin_ix(rep.basin);
        r.occupancy[cur] += 1;
        if let Some(p) = prev {
            r.edge[p][cur] += 1;
            if p != cur {
                r.changed_at.push(r.ticks);
            }
        }
        prev = Some(cur);
        r.ticks += 1;
        if !rep.alive {
            r.alive = false;
            break;
        }
    }
    r
}

/// The floor: uniform motor intent driving the same body. The being still runs so its `basin`
/// register exists to read, but its own motor choice is discarded.
fn live_random(name: &str, contingent: bool, seed: u64) -> Run {
    let mut b = UnifiedBeing::new(unified_being::genome::Genome::wanderer());
    let mut w = room(contingent);
    let mut rng = Lcg(seed);
    let postures = [Posture::Resting, Posture::Open, Posture::Braced, Posture::Withdrawn];
    let needs = [None, Some(Need::Sustenance), Some(Need::Company), Some(Need::Novelty), Some(Need::Purpose)];
    let mut r = new_run(name);
    let mut prev: Option<usize> = None;

    for _ in 0..LIFE {
        let sens = w.sense();
        let rep = b.step_embodied(&sens);
        let intent = MotorIntent {
            posture: postures[(rng.next() % 4) as usize],
            effort: (rng.next() % 256) as i16,
            reach: needs[(rng.next() % 5) as usize],
            reach_partner: None,
        };
        w.actuate(&intent);
        w.remember(rep.gave, rep.got);

        let cur = basin_ix(rep.basin);
        r.occupancy[cur] += 1;
        if let Some(p) = prev {
            r.edge[p][cur] += 1;
            if p != cur {
                r.changed_at.push(r.ticks);
            }
        }
        prev = Some(cur);
        r.ticks += 1;
        if !rep.alive {
            r.alive = false;
            break;
        }
    }
    r
}

fn bless(b: &mut UnifiedBeing) {
    b.enable_felt_choice();
    b.enable_precision_learning();
    b.enable_generative_perception();
    b.enable_workspace_persistence();
}

fn all_loops(b: &mut UnifiedBeing) {
    bless(b);
    b.enable_schema_control();
    b.enable_serial_access();
    b.enable_workspace_broadcast();
    b.enable_reflection();
    b.enable_memory_guidance();
}

fn verdict(held: bool) -> &'static str {
    if held { "HOLDS" } else { "FAILED" }
}

fn main() {
    println!("\n=== The reaction rate — crossings between metastable states ===");
    println!("  predictions RR-1..RR-5 locked in docs/c1-relabelling.md §13, committed first");
    println!("  nu_R = transitions/tick (Du et al. eq. 316). A RATE IS DIAGNOSTIC, NOT A SCORE.\n");

    let runs = vec![
        live_being("being bare      / static", false, |_| {}),
        live_being("being blessed   / static", false, bless),
        live_being("being all-loops / static", false, all_loops),
        live_random("RANDOM          / static", false, 0x5EED),
        live_being("being bare      / conting", true, |_| {}),
        live_being("being blessed   / conting", true, bless),
        live_being("being all-loops / conting", true, all_loops),
        live_random("RANDOM          / conting", true, 0x5EED),
    ];

    // ---- survival FIRST: a regime that died early has a small denominator ----
    println!("  {:<26} {:>6} {:>8} {:>7} {:>10} {:>7}", "arm", "ticks", "survived", "trans", "nu_R", "basins");
    for r in &runs {
        println!(
            "  {:<26} {:>6} {:>8} {:>7} {:>10.5} {:>7}",
            r.name,
            r.ticks,
            if r.alive { "yes" } else { "DIED" },
            r.transitions(),
            r.rate(),
            r.basins_visited()
        );
    }
    let deaths = runs.iter().filter(|r| !r.alive).count();
    if deaths > 0 {
        println!("\n  ** {deaths} arm(s) DIED — read every rate below against that. **");
    }

    println!("\n  --- basin occupancy (% of ticks) ---");
    println!("  {:<26} {:>10} {:>10} {:>10} {:>10}", "arm", "Rest", "Engaged", "Defensive", "Recovery");
    for r in &runs {
        let t = r.ticks.max(1) as f64;
        println!(
            "  {:<26} {:>9.2}% {:>9.2}% {:>9.2}% {:>9.2}%",
            r.name,
            100.0 * r.occupancy[0] as f64 / t,
            100.0 * r.occupancy[1] as f64 / t,
            100.0 * r.occupancy[2] as f64 / t,
            100.0 * r.occupancy[3] as f64 / t
        );
    }

    println!("\n  --- the transition graph: dominant pair, and net current ---");
    for r in &runs {
        let (i, j, f, rev) = r.dominant_pair();
        if f == 0 {
            println!("  {:<26} NO TRANSITIONS AT ALL", r.name);
        } else {
            println!(
                "  {:<26} {}→{} {:>5}   reverse {:>5}   net current {:>6.1}%",
                r.name, BASIN_NAME[i], BASIN_NAME[j], f, rev, r.net_current_pct()
            );
        }
    }

    println!("\n  --- WHEN did the crossings happen? (added after the run, no verdict depends on it) ---");
    for r in &runs {
        if r.changed_at.is_empty() {
            println!("  {:<26} never left its first basin", r.name);
        } else {
            let t: Vec<String> = r.changed_at.iter().map(|t| t.to_string()).collect();
            println!("  {:<26} ticks {}", r.name, t.join(", "));
        }
    }

    // RR-2 rests on "the contingent world changes nothing here", and the occupancy spread is the
    // evidence for it. Computing it by hand in prose put an unverifiable number into a claim; the
    // probe prints it instead.
    // The quiet tail and the total are the two figures the write-up leans on hardest, and both
    // were first obtained by subtracting in my head (4000 - 165, and 8 x 4000). Derived numbers do
    // not belong in a verified claim: the probe reports them.
    let total_ticks: usize = runs.iter().map(|r| r.ticks).sum();
    let quiet: Vec<usize> = runs
        .iter()
        .map(|r| r.ticks - r.changed_at.last().map(|t| t + 1).unwrap_or(0))
        .collect();
    println!(
        "\n  --- the quiet tail: ticks after the LAST basin change ({total_ticks} ticks measured in all) ---"
    );
    for (r, q) in runs.iter().zip(&quiet) {
        println!("  {:<26} {:>6} ticks with no basin change to the end of life", r.name, q);
    }
    println!("  shortest quiet tail of any being arm: {} ticks", quiet[..3].iter().chain(&quiet[4..7]).min().unwrap());

    println!("\n  --- static vs contingent, same gates: largest occupancy gap in any basin ---");
    let mut worst = 0.0_f64;
    for (k, label) in [(0usize, "bare"), (1, "blessed"), (2, "all-loops")] {
        let (a, b) = (&runs[k], &runs[k + 4]);
        let (ta, tb) = (a.ticks.max(1) as f64, b.ticks.max(1) as f64);
        let gap = (0..N_BASIN)
            .map(|i| (100.0 * a.occupancy[i] as f64 / ta - 100.0 * b.occupancy[i] as f64 / tb).abs())
            .fold(0.0_f64, f64::max);
        worst = worst.max(gap);
        println!("  {label:<12} largest gap {gap:>5.2} points   trans {} vs {}", a.transitions(), b.transitions());
    }
    println!("  worst gap across all three pairings: {worst:.2} points");

    let bs = &runs[1]; // blessed / static
    let bc = &runs[5]; // blessed / contingent
    let rs = &runs[3]; // random / static

    println!("\n  --- the locked predictions ---");
    println!(
        "  RR-1  static nu_R < 0.01? ................. {}   ({:.5})",
        verdict(bs.rate() < 0.01),
        bs.rate()
    );
    println!(
        "  RR-2  contingent >= 5x static? ............ {}   ({:.5} vs {:.5}, {:.1}x)",
        verdict(bc.rate() >= 5.0 * bs.rate().max(1e-12)),
        bc.rate(),
        bs.rate(),
        if bs.rate() > 0.0 { bc.rate() / bs.rate() } else { f64::INFINITY }
    );
    println!(
        "  RR-3  being BELOW random, static? ......... {}   ({:.5} vs {:.5})",
        verdict(bs.rate() < rs.rate()),
        bs.rate(),
        rs.rate()
    );
    println!(
        "  RR-4  >=3 basins in contingent? (exp FAIL)  {}   ({})",
        verdict(bc.basins_visited() >= 3),
        bc.basins_visited()
    );
    println!(
        "  RR-5  net current < 10%? .................. {}   ({:.1}%)",
        verdict(bc.net_current_pct() < 10.0),
        bc.net_current_pct()
    );
    println!();
}
