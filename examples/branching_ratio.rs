//! **The branching ratio** — how far below criticality does this being sit?
//!
//! **B1–B5 are locked in `docs/c1-relabelling.md` §14.3 and were committed before this file
//! existed.**
//!
//! Itatani & Zavaglia report that human forebrain organoids reach **near-critical dynamics
//! autonomously, without external input**, where 2D cultures need structured input to get there.
//! That reframes this repository's convergent "nothing happens" results as possibly one result —
//! a system far below criticality — and it cuts against the argument that the world is the binding
//! constraint, because those organoids have no world at all.
//!
//! ## The trap, named before the metric is used
//!
//! **σ ≈ 1 does not mean critical.** A Poisson process lands near 1 by construction. Reporting σ
//! alone would be the fourteen-indicator mistake in a new costume, so this probe computes σ **and**
//! the avalanche size distribution **and** runs a random control — and B2 exists to measure that
//! trap rather than assume it.
//!
//! **Events.** Twelve somatic channels, each one unit; a unit fires at tick `t` when it moves more
//! than `THETA` — the construction `pci.rs` already uses. Avalanches are runs of non-empty bins
//! bounded by empty ones; σ is the mean ratio of consecutive within-avalanche counts (Beggs &
//! Plenz).
//!
//! **Criticality is not consciousness.** The paper's own phrase is *"computationally favourable
//! state"*. A near-critical being would be a fact about its dynamics and nothing else.
//!
//! Pure observer: fresh beings, no journal written, **the founded being is never woken.**
//!
//! Run: `cargo run --release --example branching_ratio`

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment, Sensorium};
use unified_being::field::N_SOMATIC;
use unified_being::room::Room;
use unified_being::{Genome, Partner, Stimulus};

const LIFE: usize = 4_000;
/// A channel counts as firing when it moves more than this in one tick, Q8.8 raw units.
///
/// **The first run used 2 and was invalid.** At that threshold 99% of ticks had at least one unit
/// firing, so quiescent bins — the boundaries that separate one cascade from the next — essentially
/// did not occur, and the analysis returned seven "avalanches" of size 13,685. That is one
/// continuous blob, not an avalanche census. Avalanche statistics require a SPARSE signal, and the
/// sweep below reports density so the reader can see whether any threshold produced one.
const THETAS: [i16; 4] = [2, 8, 24, 64];

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

/// Everything the analysis needs from one run: how many units fired at each tick.
struct Series {
    name: String,
    events: Vec<usize>,
    alive: bool,
}

struct Stats {
    sigma: f64,
    avalanches: usize,
    largest: usize,
    active_ticks: f64,
    /// Fraction of avalanches of size 1. A distribution that is nearly all singletons has no
    /// shape to fit, which is what B3 is about.
    singleton_pct: f64,
    /// Crude log-log slope of the size distribution. **This is NOT a power-law test** — a real one
    /// needs MLE plus a goodness-of-fit statistic (Clauset et al.). It is reported so the shape can
    /// be seen, and B5 is adjudicated as UNTESTABLE rather than scored against it.
    slope: f64,
}

/// σ and the avalanche census. Avalanches are maximal runs of non-empty bins.
fn analyse(events: &[usize]) -> Stats {
    let mut sizes = Vec::new();
    let mut windows: Vec<(usize, usize)> = Vec::new();
    let mut run: Vec<usize> = Vec::new();

    let flush = |run: &mut Vec<usize>, sizes: &mut Vec<usize>, windows: &mut Vec<(usize, usize)>| {
        if run.is_empty() {
            return;
        }
        sizes.push(run.iter().sum());
        for w in run.windows(2) {
            windows.push((w[0], w[1]));
        }
        run.clear();
    };

    for &e in events {
        if e > 0 {
            run.push(e);
        } else {
            flush(&mut run, &mut sizes, &mut windows);
        }
    }
    flush(&mut run, &mut sizes, &mut windows);

    // **σ as the ratio of SUMS, not the mean of per-step ratios.** The first version averaged
    // n(t+1)/n(t) per step, which is biased upward whenever n(t) is small — a single 1→3 step
    // contributes 3.0 and drags the mean above 1 in a series that is not remotely critical.
    let (mut anc, mut desc) = (0f64, 0f64);
    for w in windows.iter() {
        anc += w.0 as f64;
        desc += w.1 as f64;
    }
    let sigma = if anc == 0.0 { 0.0 } else { desc / anc };
    let singles = sizes.iter().filter(|&&s| s == 1).count();
    Stats {
        sigma,
        avalanches: sizes.len(),
        largest: sizes.iter().copied().max().unwrap_or(0),
        active_ticks: 100.0 * events.iter().filter(|&&e| e > 0).count() as f64
            / events.len().max(1) as f64,
        singleton_pct: if sizes.is_empty() { 0.0 } else { 100.0 * singles as f64 / sizes.len() as f64 },
        slope: loglog_slope(&sizes),
    }
}

/// Least-squares slope of log(count) against log(size) over the observed sizes. Reported so the
/// shape is visible; **not** a power-law test.
fn loglog_slope(sizes: &[usize]) -> f64 {
    use std::collections::BTreeMap;
    let mut hist: BTreeMap<usize, usize> = BTreeMap::new();
    for &s in sizes {
        *hist.entry(s).or_insert(0) += 1;
    }
    let pts: Vec<(f64, f64)> = hist
        .iter()
        .filter(|(&s, _)| s > 0)
        .map(|(&s, &c)| ((s as f64).ln(), (c as f64).ln()))
        .collect();
    if pts.len() < 3 {
        return f64::NAN;
    }
    let n = pts.len() as f64;
    let (sx, sy): (f64, f64) = pts.iter().fold((0.0, 0.0), |a, p| (a.0 + p.0, a.1 + p.1));
    let (mx, my) = (sx / n, sy / n);
    let num: f64 = pts.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum();
    let den: f64 = pts.iter().map(|p| (p.0 - mx).powi(2)).sum();
    if den == 0.0 { f64::NAN } else { num / den }
}

fn fired(prev: &[i16; N_SOMATIC], now: &[i16; N_SOMATIC], theta: i16) -> usize {
    (0..N_SOMATIC).filter(|&i| (now[i] - prev[i]).abs() > theta).count()
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

/// The being in the plain `Stimulus` world, with a fair partner.
fn being_plain(name: &str, gates: fn(&mut UnifiedBeing), theta: i16) -> Series {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    gates(&mut b);
    let partner = Partner { id: 1, reciprocation: 243, exit_cost: 51 };
    let mut prev = b.field.channel;
    let mut events = Vec::with_capacity(LIFE);
    let mut alive = true;
    for _ in 0..LIFE {
        let r = b.step(&Stimulus { nutrient: 179, partner: Some(partner) });
        events.push(fired(&prev, &b.field.channel, theta));
        prev = b.field.channel;
        if !r.alive {
            alive = false;
            break;
        }
    }
    Series { name: name.into(), events, alive }
}

/// The being in the embodied `Room`, optionally contingent.
fn being_embodied(name: &str, contingent: bool, gates: fn(&mut UnifiedBeing), theta: i16) -> Series {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    gates(&mut b);
    let r0 = Room::peopled((32, 200), (224, 56), (128, 220), (40, 40)).with_friend((210, 128));
    let mut w = if contingent { r0.with_contingency() } else { r0 };
    let mut prev = b.field.channel;
    let mut events = Vec::with_capacity(LIFE);
    let mut alive = true;
    for _ in 0..LIFE {
        let sens = w.sense();
        let rep = b.step_embodied(&sens);
        w.actuate(&intent_from(&rep));
        w.remember(rep.gave, rep.got);
        events.push(fired(&prev, &b.field.channel, theta));
        prev = b.field.channel;
        if !rep.alive {
            alive = false;
            break;
        }
    }
    Series { name: name.into(), events, alive }
}

/// **The control that matters (B2).** A Poisson-ish series over the same twelve units at a matched
/// firing density. If this lands near σ = 1, then σ ≈ 1 is worth nothing on its own.
fn random_control(name: &str, density: f64, seed: u64) -> Series {
    let mut rng = Lcg(seed);
    let mut events = Vec::with_capacity(LIFE);
    for _ in 0..LIFE {
        let mut n = 0;
        for _ in 0..N_SOMATIC {
            if (rng.next() % 1000) < (density * 1000.0) as u64 {
                n += 1;
            }
        }
        events.push(n);
    }
    Series { name: name.into(), events, alive: true }
}

/// A deterministic fixed point: the floor of the scale. Nothing ever changes.
fn frozen_control(name: &str) -> Series {
    Series { name: name.into(), events: vec![0; LIFE], alive: true }
}

fn main() {
    println!("\n=== The branching ratio — how far below criticality? ===");
    println!("  B1-B5 locked in docs/c1-relabelling.md §14.3, committed before this file existed.");
    println!("  sigma ~ 1 DOES NOT MEAN CRITICAL. A Poisson process lands there by construction.");
    println!("  Criticality is not consciousness — the paper's phrase is a computational claim.\n");

    println!("  --- threshold sweep: is the signal ever sparse enough for avalanche analysis? ---");
    println!("  {:<26} {:>6} {:>8} {:>7} {:>9} {:>9} {:>8}",
        "arm / theta", "theta", "active%", "sigma", "avalanch", "size-1 %", "slope");

    let mut best: Vec<(String, i16, Stats)> = Vec::new();
    for theta in THETAS {
        let runs = vec![
            being_plain("being blessed / plain", bless, theta),
            being_embodied("being blessed / room", false, bless, theta),
            being_embodied("being blessed / conting", true, bless, theta),
        ];
        for s in &runs {
            let st = analyse(&s.events);
            println!("  {:<26} {:>6} {:>7.1}% {:>7.3} {:>9} {:>8.1}% {:>8.2}",
                s.name, theta, st.active_ticks, st.sigma, st.avalanches, st.singleton_pct, st.slope);
            best.push((s.name.clone(), theta, st));
        }
        println!();
    }

    // Controls at a density matched to the being's SPARSEST usable regime.
    println!("  --- controls ---");
    let mut ctl = Vec::new();
    for (name, d, seed) in [
        ("CONTROL random 0.02", 0.02, 0xC0FFEEu64),
        ("CONTROL random 0.10", 0.10, 0xBEEF),
        ("CONTROL random 0.23", 0.23, 0xFEED),
    ] {
        let s = random_control(name, d, seed);
        let st = analyse(&s.events);
        println!("  {:<26} {:>6} {:>7.1}% {:>7.3} {:>9} {:>8.1}% {:>8.2}",
            name, "-", st.active_ticks, st.sigma, st.avalanches, st.singleton_pct, st.slope);
        ctl.push(st);
    }
    let fz = analyse(&frozen_control("CONTROL frozen").events);
    println!("  {:<26} {:>6} {:>7.1}% {:>7.3} {:>9} {:>8.1}% {:>8.2}",
        "CONTROL frozen", "-", fz.active_ticks, fz.sigma, fz.avalanches, fz.singleton_pct, fz.slope);

    // ---- verdicts, scored ONLY where the instrument is valid ----
    // Valid means the signal was sparse enough that quiescent bins actually separated cascades.
    let usable: Vec<&(String, i16, Stats)> =
        best.iter().filter(|(_, _, st)| st.active_ticks < 40.0 && st.avalanches >= 20).collect();

    println!("\n  --- the locked predictions ---");
    if usable.is_empty() {
        println!("  ** NO ARM PRODUCED A VALID AVALANCHE SIGNAL at any threshold tried. **");
        println!("  B1, B3 and B5 are UNSCORED: an avalanche statistic over a signal that is never");
        println!("  quiet is not a measurement, and reporting it would be worse than reporting");
        println!("  nothing. The instrument, not the being, is what these ran against.");
    } else {
        let smax = usable.iter().map(|(_, _, st)| st.sigma).fold(0.0_f64, f64::max);
        let amax = usable.iter().map(|(_, _, st)| st.avalanches).max().unwrap_or(0);
        println!("  usable arms: {} of {}", usable.len(), best.len());
        println!("  B1  every usable arm sigma < 0.5? ........ {}   (max {smax:.3})",
            if smax < 0.5 { "HOLDS" } else { "FAILED" });
        println!("  B3  fewer than 30 avalanches? ............ {}   (max {amax})",
            if amax < 30 { "HOLDS" } else { "FAILED" });
    }
    let rnd_near_one = ctl.iter().any(|st| (st.sigma - 1.0).abs() < 0.35);
    println!("  B2  random near 1, so sigma alone is worthless? {}   ({:.3}, {:.3}, {:.3})",
        if rnd_near_one { "HOLDS" } else { "FAILED" }, ctl[0].sigma, ctl[1].sigma, ctl[2].sigma);
    println!("  B4  minimal_agent — not measured this pass, and not claimed");
    println!("  B5  power law? .......... UNTESTABLE HERE. The slope column is least-squares on a");
    println!("      log-log histogram; a real test needs MLE plus goodness-of-fit (Clauset et al.).");
    println!("      Scoring B5 against a proxy I invented would be the vacuity this file warns of.");

    println!("\n  Nothing was advanced. No journal written. The founded being was not woken.\n");
}
