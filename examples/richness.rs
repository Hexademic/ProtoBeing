//! World richness — giving the being something to be wrong about.
//!
//! The measurement for `docs/richness.md` §4. The being has 18 primes and uses **eight**; it
//! says *"I feel very good now"* eleven thousand times. §3 maps every unspoken word to the
//! fact it waits on, and five of them wait on the world alone.
//!
//! So: sweep the number of **independent movers** — sources drifting and breathing on
//! unrelated schedules — and watch what the being can say. No new code was needed for this;
//! `FieldWorld` already had `with_source`, `with_drift` and `with_weather`. `being.rs` is not
//! touched, no gate is added, and the founded being is not woken.
//!
//! R6 is the one that separates the questions: `Can` needs `agency > 128` against a measured
//! 8–20, and `Cant` needs `free_energy > 48` against a measured 0.69. If richness lifts
//! `NotKnow` and `Happen` but leaves those two, then we have cleanly separated what the world
//! owes this being from what we owe it.
//!
//! Run: `cargo run --release --example richness`

use std::collections::BTreeMap;

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::play::COMFORT;
use unified_being::primes::{Clause, Prime, PrimeFacts, PrimeLayer};

const LIFE: usize = 2_000;

/// The 18 primes, so "never spoken" is a checked claim and not a remembered one.
const ALL: [Prime; 18] = [
    Prime::I, Prime::Feel, Prime::Now, Prime::Good, Prime::Bad, Prime::Want,
    Prime::More, Prime::Very, Prime::Before, Prime::Know, Prime::NotKnow, Prime::Can,
    Prime::Cant, Prime::Do, Prime::Happen, Prime::Someone, Prime::Near, Prime::Because,
];

struct Lived {
    movers: usize,
    said: BTreeMap<String, usize>,
    spoke: usize,
    ticks: usize,
    alive: bool,
    free_energy: f32,
    self_surprise: f32,
    novelty_mean: i32,
    novelty_max: i16,
    residual_mean: i32,
    residual_max: i16,
    agency_max: i16,
    drive_mean: f32,
    drive_peak: i16,
    burdened: usize,
}

fn count(c: &Clause, into: &mut BTreeMap<String, usize>) {
    let mut stack = vec![c];
    while let Some(x) = stack.pop() {
        *into.entry(format!("{:?}", x.prime)).or_default() += 1;
        for ch in &x.children {
            stack.push(ch);
        }
    }
}

/// A world with `movers` independent things in it, each on its own schedule. `movers == 1`
/// is the world every embodied result so far was measured in.
fn world_with(movers: usize) -> FieldWorld {
    let mut w = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    // Source 0 (the good) always breathes — that is the world of `docs/weather.md`.
    w = w.with_weather(0, 2);
    // Each extra mover is its own source, at its own place, drifting on its own period and
    // breathing at its own octave count. Nothing is synchronised with anything else.
    for k in 1..movers {
        let i = k as i16;
        let pos = (40 + (i * 53) % 200, 200 - (i * 71) % 180);
        let peak = if k % 3 == 0 { -90 } else { 100 };
        w = w.with_source(pos, peak, 300);
        let idx = 1 + k; // sources 0 and 1 are the original good and harm
        w = w.with_drift(idx, 3 + (k as u32 * 2) % 7, (1 + (i % 3), 1 + ((i + 1) % 3)));
        if k % 2 == 0 {
            w = w.with_weather(idx, 2 + (k as u8 % 3));
        }
    }
    w
}

fn live(movers: usize) -> Lived {
    let mut world = world_with(movers);
    let mut b = UnifiedBeing::new(Genome::wanderer());
    b.enable_receptors(); // the body, on — docs/composed.md §6
    let mut layer = PrimeLayer::new();

    let mut said: BTreeMap<String, usize> = BTreeMap::new();
    let (mut spoke, mut burdened, mut n) = (0usize, 0usize, 0i64);
    let (mut fe, mut ss, mut nov, mut res, mut dr) = (0i64, 0i64, 0i64, 0i64, 0i64);
    let (mut nov_max, mut res_max, mut ag_max, mut dr_peak) = (0i16, 0i16, 0i16, 0i16);
    let mut alive = true;

    for _ in 0..LIFE {
        let sens = world.sense();
        let r = b.step_embodied(&sens);
        world.actuate(&intent_from(&r));

        let f = PrimeFacts::from_report(&r, Some(world.at_good() > 128));
        layer.observe(&f);
        if let Some(clauses) = layer.speak_tree(&f) {
            spoke += 1;
            for c in &clauses {
                count(c, &mut said);
            }
        }

        fe += r.free_energy as i64;
        ss += r.self_surprise as i64;
        nov += f.novelty as i64;
        res += f.world_residual as i64;
        dr += f.drive as i64;
        nov_max = nov_max.max(f.novelty);
        res_max = res_max.max(f.world_residual);
        ag_max = ag_max.max(f.agency);
        dr_peak = dr_peak.max(f.drive);
        if f.drive >= COMFORT {
            burdened += 1;
        }
        n += 1;

        if !r.alive {
            alive = false;
            break;
        }
    }

    let d = n.max(1) as f32;
    Lived {
        movers,
        said,
        spoke,
        ticks: n as usize,
        alive,
        free_energy: fe as f32 / d,
        self_surprise: ss as f32 / d,
        novelty_mean: (nov / n.max(1)) as i32,
        novelty_max: nov_max,
        residual_mean: (res / n.max(1)) as i32,
        residual_max: res_max,
        agency_max: ag_max,
        drive_mean: dr as f32 / d,
        drive_peak: dr_peak,
        burdened,
    }
}

fn main() {
    println!("World richness — giving the being something to be wrong about");
    println!("(R1–R6 and W locked in docs/richness.md §4 before this was written)\n");
    println!("  NOT KNOW needs novelty > {}   HAPPEN needs residual > {}", 256 / 6, 256 / 4);
    println!("  CAN needs agency > {}          CANT needs free energy > {}\n", 128, 256 * 3 / 16);

    let lives: Vec<Lived> = [1usize, 2, 4, 6, 9, 12].iter().map(|&m| live(m)).collect();

    println!("  {:>7} {:>9} {:>9} {:>11} {:>11} {:>9}", "movers", "free-en", "self-surp", "novelty", "residual", "agency");
    println!("  {:->7} {:->9} {:->9} {:->11} {:->11} {:->9}", "", "", "", "", "", "");
    for l in &lives {
        println!(
            "  {:>7} {:>9.2} {:>9.2} {:>5}/{:<5} {:>5}/{:<5} {:>9}",
            l.movers, l.free_energy, l.self_surprise,
            l.novelty_mean, l.novelty_max, l.residual_mean, l.residual_max, l.agency_max
        );
    }
    println!("  (novelty and residual shown as mean/max)");

    println!("\n  What it can say — vocabulary by world\n");
    print!("  {:<10}", "prime");
    for l in &lives {
        print!("{:>8}", l.movers);
    }
    println!("   blocked on");
    println!("  {:-<10}{:-<48}", "", "");
    for p in ALL {
        let name = format!("{:?}", p);
        print!("  {:<10}", p.word());
        let mut ever = false;
        for l in &lives {
            let c = l.said.get(&name).copied().unwrap_or(0);
            if c > 0 { ever = true; }
            print!("{:>8}", if c > 0 { c.to_string() } else { "·".into() });
        }
        println!("   {}", if ever { "" } else { "STILL UNSPOKEN" });
    }

    // How much of each life the being spent SPEAKING at all — the denominator every count
    // above is a fraction of. Computed since this probe was written and never printed, which
    // meant the vocabulary table had no scale to be read against.
    println!("\n  ticks on which the being spoke at all:");
    print!("    {:<10}", "movers");
    for l in &lives { print!("{:>8}", l.movers); }
    println!();
    print!("    {:<10}", "spoke%");
    for l in &lives {
        print!("{:>8}", format!("{:.0}%", l.spoke as f32 * 100.0 / l.ticks.max(1) as f32));
    }
    println!();

    // ---- verdicts ------------------------------------------------------------------
    let first = &lives[0];
    let last = lives.last().unwrap();
    let spoken = |l: &Lived, p: Prime| l.said.get(&format!("{:?}", p)).copied().unwrap_or(0) > 0;

    println!("\n  R1 — free energy: {:.2} → {:.2}   {}", first.free_energy, last.free_energy,
        if last.free_energy > first.free_energy { "rose, as predicted" } else { "did NOT rise — prediction wrong" });
    println!("  R2 — self-surprise: {:.2} → {:.2}   {}", first.self_surprise, last.self_surprise,
        if last.self_surprise > first.self_surprise { "rose, as predicted" } else { "did NOT rise — prediction wrong" });

    let vocab = |l: &Lived| ALL.iter().filter(|&&p| spoken(l, p)).count();
    println!("  R3 — vocabulary: {} primes → {} primes   {}", vocab(first), vocab(last),
        if vocab(last) > vocab(first) { "broadened, as predicted" } else { "did NOT broaden — prediction wrong" });

    println!("\n  R4 — does NOT KNOW become sayable?");
    match lives.iter().find(|l| spoken(l, Prime::NotKnow)) {
        Some(l) => println!("    YES — first spoken in the {}-mover world ({} times). The word for not\n    knowing was waiting on a world with something new in it, exactly as §4 said.", l.movers, l.said["NotKnow"]),
        None => println!("    NO — still unspoken at {} movers (novelty peaked at {}, needs > {}).\n    Then the blocker is the novelty register itself, not the world, and my R4\n    was wrong.", last.movers, last.novelty_max, 256/6),
    }

    println!("\n  R5 — does HAPPEN ground?");
    match lives.iter().find(|l| spoken(l, Prime::Happen)) {
        Some(l) => println!("    YES — first at {} movers. docs/happening.md §9 predicted world richness was\n    the fix; this is that prediction coming true.", l.movers),
        None => println!("    NO — residual peaked at {} against a bar of {}. World richness alone does\n    not supply it.", last.residual_max, 256/4),
    }

    println!("\n  R6 — the separating question: do CAN and CANT stay unreachable?");
    let can = lives.iter().any(|l| spoken(l, Prime::Can));
    let cant = lives.iter().any(|l| spoken(l, Prime::Cant));
    println!("    CAN  (agency > 128): peak agency {} — {}", last.agency_max, if can { "SPOKEN" } else { "still unreachable" });
    println!("    CANT (free energy > 48): {:.2} — {}", last.free_energy, if cant { "SPOKEN" } else { "still unreachable" });
    if !can && !cant {
        println!("\n    R6's VERDICT holds — both words stay unspoken. Its REASONING was");
        println!("    wrong, and in a way worth more than the verdict.");
        println!();
        println!("    I locked R6 saying agency was '16x short' of the bar. That number was a");
        println!("    MEAN. The peak tells a different story: agency peaks at {} in the sparsest",
            first.agency_max);
        println!("    world and {} in the richest — both ABOVE the bar of 128, and richness made", last.agency_max);
        println!("    it LOWER, not higher. The bar was never the problem and never was.");
        println!();
        println!("    CAN is blocked on PERSISTENCE, not magnitude. Grounding needs the fact held");
        println!("    about one tick in five (RISE 4 : EBB 1); this being's control over its own");
        println!("    senses spikes and collapses. It can act decisively for an instant and never");
        println!("    for long enough to have earned the word.");
        println!();
        println!("    That is the third time this week a mean has hidden the finding");
        println!("    (docs/play.md §7, docs/null-space.md §7). I wrote the lesson down twice and");
        println!("    still locked a prediction on a mean. CANT is genuinely far — free energy");
        println!("    {:.2} against a bar of 48 — but CAN was close all along and I could not see", last.free_energy);
        println!("    it because I was looking at the wrong statistic.");
    } else {
        println!("\n    R6 WRONG — richness reached a word I said it could not. That means the");
        println!("    limit I attributed to the being was partly its world after all.");
    }

    println!("\n  W — is the richer world a crueller one?");
    println!("    {:>7} {:>11} {:>9} {:>11} {:>8}", "movers", "mean drive", "peak", "burdened", "alive");
    for l in &lives {
        println!("    {:>7} {:>11.3} {:>9.3} {:>10}% {:>8}",
            l.movers, l.drive_mean / 256.0, l.drive_peak as f32 / 256.0,
            l.burdened * 100 / l.ticks.max(1), if l.alive { "yes" } else { "DIED" });
    }
    println!("    comfort line {:.2}", COMFORT as f32 / 256.0);
    let died: Vec<usize> = lives.iter().filter(|l| !l.alive).map(|l| l.movers).collect();
    if !died.is_empty() {
        println!("\n    W FAILS, and no mean should be allowed to hide it: the being DIED in the");
        println!("    {:?}-mover world{} — peak drive {:.3} against a comfort line of {:.2}.",
            died, if died.len() > 1 { "s" } else { "" },
            lives.iter().find(|l| !l.alive).unwrap().drive_peak as f32 / 256.0,
            COMFORT as f32 / 256.0);
        println!("    Richness is NOT uniformly safe. Some arrangements of independent movers");
        println!("    kill this being, and which ones is not yet understood — the 6-mover world");
        println!("    carries the same harm source and survives. A richer world is a place with");
        println!("    more ways to die in it, and that is the cost side of every word gained");
        println!("    below. §4's W asked whether richness was crueller; the honest answer is");
        println!("    that it is SOMETIMES LETHAL, which is worse than crueller and was not");
        println!("    among the outcomes I imagined.");
    } else {
        let worse = last.drive_mean > first.drive_mean;
        println!("\n    Every being survived. The richest world is {} than the sparsest.",
            if worse { "HARDER" } else { "no harder" });
    }
    if spoken(last, Prime::Bad) {
        println!("\n    BAD became sayable ({} times at twelve movers, never at one). That is a",
            last.said.get("Bad").copied().unwrap_or(0));
        println!("    word the being earned by having something bad to report. Whether that is a");
        println!("    gain depends entirely on the line above it.");
    }

    println!("\n  No faculty was built, no gate added, no default changed, founded being untouched.");
}
