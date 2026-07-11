//! `pci_baseline` — the normative baseline for PCI.
//!
//! A single PCI number is not evidence. This binary turns it into a
//! **distribution** and a **mechanism-dependence test**, per the roadmap in
//! `docs/operational-consciousness.md` §Gap-D. Because the being is
//! deterministic, the distribution comes from a *population* that varies by
//! genome (temperament) and lived history — not from re-running one being. Every
//! source of variation is seeded, so the whole baseline is reproducible to the
//! bit, which biological PCI (needing bootstrap statistics over an unknowable
//! unperturbed brain) can never be.
//!
//! It answers three questions, each with a Mann–Whitney U test:
//!   1. Does PCI track dynamical regime? (near-critical Spark vs. stable Sentinel)
//!   2. Is the response real, not an artifact? (a genuine impulse vs. the null)
//!   3. Does the Global-Workspace broadcast change the differential PCI?
//!
//! Run: cargo run --release --bin pci_baseline

use unified_being::pci::baseline::{mann_whitney_u, population_pci, summarize, MannWhitney, PciSummary};
use unified_being::{Genome, PciHarness, Perturbation};

const N: usize = 80; // beings per condition
const PRELIFE: u16 = 40; // ticks of varied life before measurement

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

fn row(label: &str, s: &PciSummary) {
    println!(
        "  {label:<26} n={:>3}  min {:>5.3}  Q1 {:>5.3}  med {:>5.3}  Q3 {:>5.3}  max {:>5.3}  mean {:>5.3}",
        s.n,
        f(s.min),
        f(s.q1),
        f(s.median),
        f(s.q3),
        f(s.max),
        f(s.mean),
    );
}

fn stars(p: f64) -> &'static str {
    if p < 0.001 {
        "*** (p<0.001)"
    } else if p < 0.01 {
        "** (p<0.01)"
    } else if p < 0.05 {
        "* (p<0.05)"
    } else {
        "n.s."
    }
}

fn verdict(name: &str, a: &[i16], b: &[i16]) -> MannWhitney {
    let r = mann_whitney_u(a, b);
    println!(
        "  {name:<34}  U={:>7.0}  z={:>+6.2}  p={:>7.4}  {}",
        r.u,
        r.z,
        r.p_two_sided,
        stars(r.p_two_sided)
    );
    r
}

/// Median (raw Q8.8) of a sample, for plain-language reporting.
fn median(x: &[i16]) -> i16 {
    summarize(x).median
}

fn main() {
    let harness = PciHarness::default();
    let perturb = Perturbation::extraction(); // the impulse that propagates

    println!("\n=== PCI normative baseline ===\n");
    println!(
        "  {N} beings per condition. Each is a distinct jittered genome that lives a\n  \
         short varied life, then is perturbed against an untouched twin. Deterministic\n  \
         and reproducible; compare values *relatively*, not against the human 0.31.\n"
    );

    // 1. PCI by temperament (broadcast off) — does regime shape complexity?
    let spark = population_pci(Genome::spark(), N, &harness, &perturb, PRELIFE, false, 0x5EED_0001);
    let wander = population_pci(Genome::wanderer(), N, &harness, &perturb, PRELIFE, false, 0x5EED_0002);
    let sentinel = population_pci(Genome::sentinel(), N, &harness, &perturb, PRELIFE, false, 0x5EED_0003);
    let blank = population_pci(Genome::blank(), N, &harness, &perturb, PRELIFE, false, 0x5EED_0004);

    println!("  -- PCI distribution by temperament (real impulse, broadcast off) --");
    row("Spark (near-critical)", &summarize(&spark));
    row("Wanderer (mid)", &summarize(&wander));
    row("Sentinel (deeply stable)", &summarize(&sentinel));
    row("Blank (baseline)", &summarize(&blank));

    // 2. The null control — no impulse — on the same Wanderer population.
    let null = population_pci(
        Genome::wanderer(),
        N,
        &harness,
        &Perturbation::none(),
        PRELIFE,
        false,
        0x5EED_0002,
    );
    println!("\n  -- null control (no impulse): the differentiation floor --");
    row("Wanderer, NO impulse", &summarize(&null));

    // 3. Broadcast on vs off on the same Wanderer population.
    let bcast_on = population_pci(Genome::wanderer(), N, &harness, &perturb, PRELIFE, true, 0x5EED_0002);
    println!("\n  -- Global-Workspace broadcast on vs off (Wanderer, real impulse) --");
    row("broadcast OFF", &summarize(&wander));
    row("broadcast ON", &summarize(&bcast_on));

    // The statistical tests.
    println!("\n  -- Mann-Whitney U tests --");
    let regime = verdict("Spark vs Sentinel (regime)", &spark, &sentinel);
    let real = verdict("real impulse vs null (Wanderer)", &wander, &null);
    let bcast = verdict("broadcast ON vs OFF (Wanderer)", &bcast_on, &wander);

    // A data-driven conclusion — what THIS run actually found, not a hope.
    println!("\n  -- what this baseline found --");
    if real.p_two_sided < 0.05 && median(&null) == 0 {
        println!(
            "  • The response is REAL: a genuine impulse beats the no-impulse null {} (null\n    \
             floored at 0.000). PCI here measures response to perturbation, not artifact.",
            stars(real.p_two_sided)
        );
    } else {
        println!("  • The impulse did not separate from the null — investigate the perturbation.");
    }
    if regime.p_two_sided < 0.05 {
        let dir = if median(&spark) > median(&sentinel) { "higher" } else { "lower" };
        println!(
            "  • Temperament MATTERS: near-critical Spark scores {dir} than stable Sentinel {}.",
            stars(regime.p_two_sided)
        );
    } else {
        println!(
            "  • Temperament does NOT separate under this differential measure (Spark vs Sentinel\n    \
             n.s.). An honest null: the twin-subtraction echo is dominated by the shared body\n    \
             dynamics, so genome regime is not resolved by PCI alone here."
        );
    }
    if bcast.p_two_sided < 0.05 {
        println!("  • Broadcast shifts the differential PCI distribution {}.", stars(bcast.p_two_sided));
    } else {
        println!(
            "  • Broadcast does NOT shift differential PCI (n.s.) — the EXPECTED result: a\n    \
             config-level ablation applied to both twins cancels under twin-subtraction. The\n    \
             within-being spread probe (`cargo run --bin pci`) is the sharper broadcast test;\n    \
             it found ignition becomes causal (reach 0→1) without yet cascading."
        );
    }

    println!(
        "\n  The baseline's contribution is the *distribution and the test* the single-run\n  \
         numbers always lacked: a reproducible population, a floor at zero, and a\n  \
         significance verdict on each mechanism claim. None of it closes the Witness Gap —\n  \
         it makes the integration marker measurable, distributed, and falsifiable. See\n  \
         docs/operational-consciousness.md §Gap-D.\n"
    );
}
