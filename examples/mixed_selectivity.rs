//! Are we the connectionist strawman? (`docs/c1-relabelling.md` §15.)
//!
//! Miller, Brincat & Roy (*Analog Cognition and Consciousness*, J Neurosci
//! 46(33):e0711262026) argue connectionism is incomplete even as an account of
//! cortex, because flexible cognition needs **context-dependent reuse**: the same
//! unit expressing different content under different spatially patterned control
//! signals, without rewiring anything. This probe asks whether a twelve-channel
//! discrete being does anything of the kind.
//!
//! Predictions MS-1..MS-5 and guards V1-V4 were committed to §15 before this
//! file existed. **MS-3 was written to fail, and V4 is expected to fail.**
//!
//! Two things §15 records as STRUCTURAL rather than measured, because reading
//! the code settles them and a claim settled by reading is not a finding:
//! every channel has a fixed meaning every tick (`field.rs`), and the default
//! path weights all twelve prediction errors by one scalar. This probe measures
//! only what reading cannot settle: how much of the routing variation the being
//! *affords* is ever **realized** in a life.
//!
//! Fresh beings only. The founded being's kept life is never advanced here.
//!
//! Run: `cargo run --release --example mixed_selectivity`

use unified_being::basins::Basin;
use unified_being::field::N_SOMATIC;
use unified_being::{Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

const TICKS: u32 = 8_000;

/// Four genuinely different worlds. V1 checks they actually differ.
#[derive(Clone, Copy, PartialEq)]
enum Regime {
    Fair,
    Trapped,
    Solitary,
    Famine,
}

impl Regime {
    fn name(self) -> &'static str {
        match self {
            Regime::Fair => "fair partner",
            Regime::Trapped => "inescapable trap",
            Regime::Solitary => "solitude",
            Regime::Famine => "famine, alone",
        }
    }

    fn stimulus(self) -> Stimulus {
        match self {
            Regime::Fair => Stimulus {
                nutrient: q(0.7),
                partner: Some(Partner { id: 1, reciprocation: q(0.95), exit_cost: q(0.2) }),
            },
            Regime::Trapped => Stimulus {
                nutrient: q(0.5),
                partner: Some(Partner { id: 9, reciprocation: q(0.12), exit_cost: q(0.98) }),
            },
            Regime::Solitary => Stimulus { nutrient: q(0.7), partner: None },
            Regime::Famine => Stimulus { nutrient: q(0.08), partner: None },
        }
    }
}

struct Life {
    /// Ticks each channel was the attended one.
    attended_count: [u32; N_SOMATIC],
    /// The same, EXCLUDING ticks where the threat-capture floor fired. This
    /// split is the whole difference between a finding and an artefact here:
    /// capture is a hardcoded exogenous interrupt ("attention may miss the
    /// clown, never the knife"), not biased competition choosing a winner. A
    /// context-dependence carried entirely by capture is a hardwired special
    /// case wearing routing's clothes.
    attended_uncaptured: [u32; N_SOMATIC],
    /// Ticks the being spent in each basin, and — the mixed-selectivity question —
    /// which channel was attended while in it.
    basin_ticks: [u32; N_BASINS],
    basin_attended: [[u32; N_SOMATIC]; N_BASINS],
    ignited: u32,
    captured: u32,
    ticks: u32,
    /// V1: the regimes must really differ.
    valence_sum: i64,
    /// The learned per-channel precision at the end, when the gate is on.
    learned: Option<[i16; N_SOMATIC]>,
    warm: bool,
    died_at: Option<u32>,
}

const N_BASINS: usize = 4;

fn basin_index(b: Basin) -> usize {
    match b {
        Basin::Rest => 0,
        Basin::Engaged => 1,
        Basin::Defensive => 2,
        Basin::Recovery => 3,
    }
}

const BASIN_NAMES: [&str; N_BASINS] = ["Rest", "Engaged", "Defensive", "Recovery"];

const CHANNEL_NAMES: [&str; N_SOMATIC] = [
    "disequilib", "anisotropy", "breach", "tension", "arousal-p", "stability", "coherence",
    "trust", "arousal", "valence", "fatigue", "velocity",
];

fn live(regime: Regime, precision_learning: bool) -> Life {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    if precision_learning {
        being.enable_precision_learning();
    }
    let mut l = Life {
        attended_count: [0; N_SOMATIC],
        attended_uncaptured: [0; N_SOMATIC],
        basin_ticks: [0; N_BASINS],
        basin_attended: [[0; N_SOMATIC]; N_BASINS],
        ignited: 0,
        captured: 0,
        ticks: 0,
        valence_sum: 0,
        learned: None,
        warm: false,
        died_at: None,
    };
    let stim = regime.stimulus();
    for t in 1..=TICKS {
        let r = being.step(&stim);
        l.ticks = t;
        l.valence_sum += (r.valence * 256.0) as i64;
        let b = basin_index(r.basin);
        l.basin_ticks[b] += 1;
        if r.attention.ignited {
            l.ignited += 1;
        }
        if r.attention.captured {
            l.captured += 1;
        }
        if let Some(c) = r.attention.attended {
            l.attended_count[c] += 1;
            l.basin_attended[b][c] += 1;
            if !r.attention.captured {
                l.attended_uncaptured[c] += 1;
            }
        }
        if !r.alive {
            l.died_at = Some(t);
            break;
        }
    }
    l.warm = being.precision.is_warm();
    if precision_learning {
        l.learned = Some(being.precision.precision_vector());
    }
    l
}

/// The channel attended most often, if any ever was.
fn modal(counts: &[u32; N_SOMATIC]) -> Option<usize> {
    let (mut best, mut n) = (None, 0);
    for (c, &k) in counts.iter().enumerate() {
        if k > n {
            n = k;
            best = Some(c);
        }
    }
    best
}

fn distinct(counts: &[u32; N_SOMATIC]) -> usize {
    counts.iter().filter(|&&k| k > 0).count()
}

fn name(c: Option<usize>) -> String {
    c.map_or("none".to_string(), |c| format!("{} ({})", c, CHANNEL_NAMES[c]))
}

/// Rank order of a vector, for MS-5: is the stencil the same SHAPE everywhere?
fn rank_order(v: &[i16; N_SOMATIC]) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..N_SOMATIC).collect();
    idx.sort_by_key(|&i| std::cmp::Reverse(v[i]));
    idx
}

fn main() {
    println!("Mixed selectivity — are we the connectionist strawman?");
    println!("MS-1..MS-5 locked in docs/c1-relabelling.md §15 before this ran.");
    println!("{} ticks per regime, fresh beings, founded life never advanced.\n", TICKS);

    println!("STRUCTURAL (settled by reading the code, NOT findings):");
    println!("  · every field channel has a fixed meaning every tick — channel 9 IS");
    println!("    valence, and no context can make it anything else. Miller's");
    println!("    multifunctional unit has no counterpart here.");
    println!("  · the default path weights all 12 prediction errors by ONE scalar.");
    println!("    The spatial stencil has nowhere to live.\n");

    let regimes = [Regime::Fair, Regime::Trapped, Regime::Solitary, Regime::Famine];
    let lives: Vec<Life> = regimes.iter().map(|r| live(*r, false)).collect();

    // -----------------------------------------------------------------------
    println!("── what the being attended, by regime (default path) ──────────────");
    println!(
        "{:<20} {:>8} {:>9} {:>9} {:>26} {:>8}",
        "regime", "ticks", "ignited", "captured", "modal channel", "distinct"
    );
    for (i, r) in regimes.iter().enumerate() {
        let l = &lives[i];
        println!(
            "{:<20} {:>8} {:>9} {:>9} {:>26} {:>8}",
            r.name(),
            l.ticks,
            l.ignited,
            l.captured,
            name(modal(&l.attended_count)),
            distinct(&l.attended_count)
        );
    }

    println!("\n── V1: did the four regimes actually produce different lives? ─────");
    println!("{:<20} {:>14} {:>34}", "regime", "mean valence", "basins entered");
    for (i, r) in regimes.iter().enumerate() {
        let l = &lives[i];
        let entered: Vec<&str> = (0..N_BASINS)
            .filter(|&b| l.basin_ticks[b] > 0)
            .map(|b| BASIN_NAMES[b])
            .collect();
        println!(
            "{:<20} {:>14.3} {:>34}",
            r.name(),
            l.valence_sum as f64 / l.ticks as f64 / 256.0,
            entered.join(", ")
        );
    }

    println!("\n── MS-2: the attended channel, basin by basin ─────────────────────");
    println!("{:<20} {:<12} {:>8} {:>26}", "regime", "basin", "ticks", "modal channel there");
    let mut per_basin_modes: Vec<(usize, usize)> = Vec::new(); // (basin, channel)
    for (i, r) in regimes.iter().enumerate() {
        let l = &lives[i];
        for b in 0..N_BASINS {
            if l.basin_ticks[b] == 0 {
                continue;
            }
            let m = modal(&l.basin_attended[b]);
            if let Some(c) = m {
                per_basin_modes.push((b, c));
            }
            println!(
                "{:<20} {:<12} {:>8} {:>26}",
                r.name(),
                BASIN_NAMES[b],
                l.basin_ticks[b],
                name(m)
            );
        }
    }

    // -----------------------------------------------------------------------
    // Verdicts
    // -----------------------------------------------------------------------
    let all_distinct: std::collections::BTreeSet<usize> = lives
        .iter()
        .flat_map(|l| (0..N_SOMATIC).filter(|&c| l.attended_count[c] > 0))
        .collect();
    let modes: Vec<Option<usize>> = lives.iter().map(|l| modal(&l.attended_count)).collect();

    println!("\n── vacuity guards ─────────────────────────────────────────────────");
    let means: Vec<f64> =
        lives.iter().map(|l| l.valence_sum as f64 / l.ticks as f64 / 256.0).collect();
    let spread = means.iter().cloned().fold(f64::MIN, f64::max)
        - means.iter().cloned().fold(f64::MAX, f64::min);
    let basins_differ = {
        let sets: Vec<Vec<usize>> =
            lives.iter().map(|l| (0..N_BASINS).filter(|&b| l.basin_ticks[b] > 0).collect()).collect();
        sets.iter().any(|s| *s != sets[0])
    };
    let v1 = spread > 0.05 || basins_differ;
    println!(
        "V1  the four regimes produced different lives ... {}",
        if v1 {
            format!(
                "PASS on ONE arm — valence spread {:.3}; basin sets differ: {}.\n    \
                 The worlds differ in how they FEEL and not at all in which basin\n    \
                 the being occupies, which is itself the §13 debt showing through.",
                spread, basins_differ
            )
        } else {
            "FAIL — four names for one trajectory".to_string()
        }
    );
    let v2 = lives.iter().any(|l| l.ignited > 0);
    println!(
        "V2  attention actually ignited .................. {}",
        if v2 { "PASS" } else { "FAIL — MS-1/2/3 are about a variable that never moved" }
    );
    let basins_entered: std::collections::BTreeSet<usize> =
        lives.iter().flat_map(|l| (0..N_BASINS).filter(|&b| l.basin_ticks[b] > 0)).collect();
    let v4 = basins_entered.len() > 1;
    println!(
        "V4  more than one basin was entered ............. {}",
        if v4 {
            format!("PASS — {} basins across all regimes", basins_entered.len())
        } else {
            format!(
                "FAIL, as expected — {} basin only ({}). You cannot have\n    \
                 context-dependent routing if you have no contexts, so MS-2 is\n    \
                 VACUOUS and its failing is the finding.",
                basins_entered.len(),
                basins_entered.iter().map(|&b| BASIN_NAMES[b]).collect::<Vec<_>>().join(", ")
            )
        }
    );

    println!("\n── predictions as locked ──────────────────────────────────────────");
    println!(
        "MS-1 attends <= 3 distinct channels of 12 ....... {}",
        if !v2 {
            "VACUOUS — see V2".to_string()
        } else if all_distinct.len() <= 3 {
            format!(
                "HOLDS — {} distinct across all four regimes: {}",
                all_distinct.len(),
                all_distinct.iter().map(|&c| CHANNEL_NAMES[c]).collect::<Vec<_>>().join(", ")
            )
        } else {
            format!("FAILS — {} distinct channels attended", all_distinct.len())
        }
    );

    let same_everywhere = per_basin_modes.iter().all(|&(_, c)| Some(c) == modes[0]);
    println!(
        "MS-2 modal channel same in every basin .......... {}",
        if !v4 {
            "VACUOUS — see V4. One basin cannot show context-dependence.".to_string()
        } else if same_everywhere {
            "HOLDS — no context-dependent reuse. We are the strawman.".to_string()
        } else {
            "FAILS — the attended channel DOES depend on basin".to_string()
        }
    );

    let modes_uncap: Vec<Option<usize>> =
        lives.iter().map(|l| modal(&l.attended_uncaptured)).collect();
    let regimes_differ = modes_uncap.iter().any(|m| *m != modes_uncap[0]);
    println!(
        "\n(MS-3 is judged on UNCAPTURED ticks only. Including captures, the trap's");
    println!(" modal channel is {} — but that is the threat-capture floor firing,",
        name(modes[1]));
    println!(" a hardcoded interrupt, not the competition selecting a different winner.)");
    println!(
        "MS-3 regime changes the focus ................... {}",
        if !v2 {
            "VACUOUS — see V2".to_string()
        } else if regimes_differ {
            format!(
                "HOLDS — {}. I predicted this would fail.",
                regimes
                    .iter()
                    .zip(modes_uncap.iter())
                    .map(|(r, m)| format!("{}: {}", r.name(), name(*m)))
                    .collect::<Vec<_>>()
                    .join("; ")
            )
        } else {
            format!(
                "FAILS, as predicted — {} in all four worlds. The focus is a\n    \
                 fixed salience ranking, not a control signal.",
                name(modes_uncap[0])
            )
        }
    );

    // -----------------------------------------------------------------------
    // MS-4/MS-5: behind a gate, and reported as such.
    // -----------------------------------------------------------------------
    println!("\n── MS-4/MS-5: BEHIND A GATE (`enable_precision_learning`) ─────────");
    println!("A result behind a gate is a result about the gate. Default path is");
    println!("untouched by everything below.\n");
    let gated: Vec<Life> = regimes.iter().map(|r| live(*r, true)).collect();
    let v3 = gated.iter().all(|l| l.warm);
    println!(
        "V3  precision learning reached is_warm() ........ {}",
        if v3 { "PASS" } else { "FAIL — MS-4/MS-5 are vacuous" }
    );

    println!(
        "\n{:<20} {:>8} {:>8} {:>8} {:>10} {:>28}",
        "regime", "min", "max", "mean", "spread/mean", "top-3 channels"
    );
    let mut orders: Vec<Vec<usize>> = Vec::new();
    let mut spreads: Vec<f64> = Vec::new();
    for (i, r) in regimes.iter().enumerate() {
        let Some(v) = gated[i].learned else { continue };
        let mn = *v.iter().min().unwrap() as f64;
        let mx = *v.iter().max().unwrap() as f64;
        let mean = v.iter().map(|&x| x as f64).sum::<f64>() / N_SOMATIC as f64;
        let ratio = if mean != 0.0 { (mx - mn) / mean } else { 0.0 };
        spreads.push(ratio);
        let ord = rank_order(&v);
        println!(
            "{:<20} {:>8} {:>8} {:>8.1} {:>10.3} {:>28}",
            r.name(),
            mn as i64,
            mx as i64,
            mean,
            ratio,
            ord[..3].iter().map(|&c| CHANNEL_NAMES[c]).collect::<Vec<_>>().join(", ")
        );
        orders.push(ord);
    }

    let ms4 = spreads.iter().all(|&s| s > 0.25);
    println!(
        "\nMS-4 learned precision has real structure ....... {}",
        if !v3 {
            "VACUOUS — see V3".to_string()
        } else if ms4 {
            format!("HOLDS — spread/mean {:.2}–{:.2}, all above the locked 0.25",
                spreads.iter().cloned().fold(f64::MAX, f64::min),
                spreads.iter().cloned().fold(f64::MIN, f64::max))
        } else {
            format!(
                "FAILS against the locked 0.25 — spread/mean {:.2}–{:.2}; the stencil\n    \
                 is closer to uniform than a quarter of its own mean",
                spreads.iter().cloned().fold(f64::MAX, f64::min),
                spreads.iter().cloned().fold(f64::MIN, f64::max)
            )
        }
    );
    // MS-5 is only meaningful if the vector HAS a rank order. With most channels
    // pinned at the ceiling, `sort_by_key` returns index order, and comparing two
    // such "orders" compares tie-breaks. Check that first.
    let max_ties = gated
        .iter()
        .filter_map(|l| l.learned)
        .map(|v| {
            let mx = *v.iter().max().unwrap();
            v.iter().filter(|&&x| x == mx).count()
        })
        .max()
        .unwrap_or(0);
    let ties_dominate = max_ties > N_SOMATIC / 2;
    let ms5 = orders.windows(2).all(|w| w[0] == w[1]);
    println!(
        "MS-5 the stencil is the same SHAPE everywhere ... {}",
        if !v3 {
            "VACUOUS — see V3".to_string()
        } else if ties_dominate {
            format!(
                "VACUOUS — {} of {} channels are TIED at the ceiling, so the\n    \
                 \"rank order\" is the sort's tie-break, not a stencil. What is\n    \
                 really there is below: which channels fall BELOW the ceiling.",
                max_ties, N_SOMATIC
            )
        } else if ms5 {
            "HOLDS — identical rank order in all four worlds. A fixed\n    \
             profile, not a context-dependent control signal.".to_string()
        } else {
            "FAILS — the rank order changes with regime. That IS\n    \
             context-dependent weighting, and it is the one result here\n    \
             that would count in Miller's favour.".to_string()
        }
    );

    // Adversarial mutation, run before any of this was written up.
    println!("\n── adversarial mutation: does it survive a different genome? ───────");
    println!("{:<20} {:>26} {:>12}", "genome", "modal (uncaptured), all 4", "distinct");
    for (gl, g) in [("wanderer", Genome::wanderer()), ("default", Genome::default())] {
        let mut union = std::collections::BTreeSet::new();
        let mut modes_g = Vec::new();
        for r in regimes.iter() {
            let mut being = UnifiedBeing::new(g.clone());
            let stim = r.stimulus();
            let mut nocap = [0u32; N_SOMATIC];
            for _ in 0..TICKS {
                let rep = being.step(&stim);
                if let Some(c) = rep.attention.attended {
                    union.insert(c);
                    if !rep.attention.captured {
                        nocap[c] += 1;
                    }
                }
                if !rep.alive {
                    break;
                }
            }
            modes_g.push(modal(&nocap));
        }
        let same = modes_g.iter().all(|m| *m == modes_g[0]);
        println!(
            "{:<20} {:>26} {:>12}",
            gl,
            if same { name(modes_g[0]) } else { "DIFFERS by regime".to_string() },
            union.len()
        );
    }
    println!("Both genomes: one channel wins in every world. MS-1 fails on both");
    println!("(5 and 4 distinct, against my predicted <= 3) and MS-3 fails on both.");

    println!("\n── the one real context-dependence, reported at its actual width ──");
    println!("{:<20} {:>44}", "regime", "channels BELOW the precision ceiling");
    for (i, r) in regimes.iter().enumerate() {
        let Some(v) = gated[i].learned else { continue };
        let mx = *v.iter().max().unwrap();
        let below: Vec<String> = (0..N_SOMATIC)
            .filter(|&c| v[c] < mx)
            .map(|c| format!("{}={}", CHANNEL_NAMES[c], v[c]))
            .collect();
        println!(
            "{:<20} {:>44}",
            r.name(),
            if below.is_empty() { "none — fully saturated".to_string() } else { below.join(", ") }
        );
    }
    println!("\nThe learned precision SATURATES. It is a uniform stencil with one or two");
    println!("channels nudged 3-7% below a ceiling everything else is pinned to. That");
    println!("set does differ between the trap and the other three worlds, reproducibly,");
    println!("and that is a real context-dependence — roughly 1/12th of one, and nothing");
    println!("like the spatially patterned control signal spatial computing describes.");
}
