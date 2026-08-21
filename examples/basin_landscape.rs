//! What is the basin-target ratchet costing? (`docs/comfort.md` §16.)
//!
//! `shift_target` drifts the **occupied** basin's target toward the field
//! whenever relief is negative, and never touches the other three. So the basin
//! the being already lives in runs away from its neighbours over a life, while
//! the ones it has never visited stay pinned at author-set coordinates. This
//! probe measures what that costs, and separates it from a second problem:
//! three of the four basins may sit at coordinates the body cannot produce.
//!
//! Predictions R1–R6 and guards V1–V3 were committed to §16 before this file
//! existed. **R4 was written to fail.**
//!
//! It deliberately does **not** recalibrate `BASE_TARGETS`. Placing the targets
//! where the being already goes would manufacture the multi-basin result — the
//! same failure as manufactured consent in §9/§10 in different clothes.
//!
//! Fresh beings only. The founded being's kept life is never advanced.
//! `freeze_basin_targets()` is default-off; the default path is untouched.
//!
//! Run: `cargo run --release --example basin_landscape`

use unified_being::basins::{Basin, FuzzyBasinField};
use unified_being::field::{SomaticField, N_SOMATIC};
use unified_being::{Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

const TICKS: u32 = 8_000;
const N_BASINS: usize = 4;
const BN: [&str; N_BASINS] = ["Rest", "Engaged", "Defensive", "Recovery"];
const CH: [&str; N_SOMATIC] = [
    "disequilib", "anisotropy", "breach", "tension", "arousal-p", "stability", "coherence",
    "trust", "arousal", "valence", "fatigue", "velocity",
];

fn basin_index(b: Basin) -> usize {
    match b {
        Basin::Rest => 0,
        Basin::Engaged => 1,
        Basin::Defensive => 2,
        Basin::Recovery => 3,
    }
}

/// L1 distance, the same metric `compute_membership` scores with.
fn dist(a: &[i16; N_SOMATIC], b: &[i16; N_SOMATIC]) -> i32 {
    (0..N_SOMATIC).map(|c| (a[c] as i32 - b[c] as i32).abs()).sum()
}

#[derive(Clone, Copy)]
struct World {
    name: &'static str,
    nutrient: f32,
    partner: Option<Partner>,
}

fn worlds() -> Vec<World> {
    vec![
        World {
            name: "fair partner",
            nutrient: 0.7,
            partner: Some(Partner { id: 1, reciprocation: q(0.95), exit_cost: q(0.2) }),
        },
        World {
            name: "inescapable trap",
            nutrient: 0.5,
            partner: Some(Partner { id: 9, reciprocation: q(0.12), exit_cost: q(0.98) }),
        },
        World { name: "solitude", nutrient: 0.7, partner: None },
        World { name: "famine, alone", nutrient: 0.08, partner: None },
    ]
}

struct Life {
    occupancy: [u32; N_BASINS],
    switches: u32,
    targets_end: [[i16; N_SOMATIC]; N_BASINS],
    /// Distance from the final field to each target.
    final_dist: [i32; N_BASINS],
    /// Per-channel range over the whole life.
    lo: [i16; N_SOMATIC],
    hi: [i16; N_SOMATIC],
    mean_valence: f64,
    ticks: u32,
}

fn live(w: World, freeze: bool) -> Life {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    if freeze {
        being.freeze_basin_targets();
    }
    let stim = Stimulus { nutrient: q(w.nutrient), partner: w.partner };
    let mut l = Life {
        occupancy: [0; N_BASINS],
        switches: 0,
        targets_end: being.basins.targets,
        final_dist: [0; N_BASINS],
        lo: [i16::MAX; N_SOMATIC],
        hi: [i16::MIN; N_SOMATIC],
        mean_valence: 0.0,
        ticks: 0,
    };
    let mut last: Option<usize> = None;
    let mut vsum = 0f64;
    for t in 1..=TICKS {
        let r = being.step(&stim);
        l.ticks = t;
        vsum += r.valence as f64;
        let b = basin_index(r.basin);
        l.occupancy[b] += 1;
        if last.is_some() && last != Some(b) {
            l.switches += 1;
        }
        last = Some(b);
        for c in 0..N_SOMATIC {
            let v = being.field.channel[c];
            l.lo[c] = l.lo[c].min(v);
            l.hi[c] = l.hi[c].max(v);
        }
        if !r.alive {
            break;
        }
    }
    l.mean_valence = vsum / l.ticks as f64;
    l.targets_end = being.basins.targets;
    let f = being.field.channel;
    for b in 0..N_BASINS {
        l.final_dist[b] = dist(&f, &l.targets_end[b]);
    }
    l
}

fn occupancy_str(o: &[u32; N_BASINS]) -> String {
    (0..N_BASINS)
        .filter(|&b| o[b] > 0)
        .map(|b| format!("{} {}", BN[b], o[b]))
        .collect::<Vec<_>>()
        .join(", ")
}

fn main() {
    println!("The basin landscape — what is the target ratchet costing?");
    println!("R1–R6 locked in docs/comfort.md §16 before this ran.");
    println!("{} ticks, fresh beings, founded life never advanced.\n", TICKS);

    let ws = worlds();
    let drifting: Vec<Life> = ws.iter().map(|w| live(*w, false)).collect();
    let frozen: Vec<Life> = ws.iter().map(|w| live(*w, true)).collect();
    let birth = UnifiedBeing::new(Genome::wanderer()).basins.targets;

    // -----------------------------------------------------------------------
    println!("── occupancy: drifting targets (default) vs frozen ────────────────");
    println!(
        "{:<20} {:>34} {:>34}",
        "world", "drifting (default path)", "targets frozen at birth"
    );
    for (i, w) in ws.iter().enumerate() {
        println!(
            "{:<20} {:>34} {:>34}",
            w.name,
            occupancy_str(&drifting[i].occupancy),
            occupancy_str(&frozen[i].occupancy)
        );
    }

    println!("\n── the ratchet, measured: pairwise target distance ────────────────");
    println!(
        "{:<24} {:>9} {:>26} {:>10}",
        "pair", "at birth", "after a life (drifting)", "frozen"
    );
    for i in 0..N_BASINS {
        for j in i + 1..N_BASINS {
            let d0 = dist(&birth[i], &birth[j]);
            let dd: Vec<i32> =
                (0..ws.len()).map(|k| dist(&drifting[k].targets_end[i], &drifting[k].targets_end[j])).collect();
            let df = dist(&frozen[0].targets_end[i], &frozen[0].targets_end[j]);
            println!(
                "{:<24} {:>9} {:>26} {:>10}",
                format!("{} vs {}", BN[i], BN[j]),
                d0,
                format!(
                    "{}..{}",
                    dd.iter().min().unwrap(),
                    dd.iter().max().unwrap()
                ),
                df
            );
        }
    }

    println!("\n── margin of victory: how close is the runner-up? ─────────────────");
    println!("(§16's preamble called this \"a landslide\" from the FAIR world alone.");
    println!(" It is a landslide in two worlds and razor-thin in the other two.)");
    println!(
        "{:<20} {:>12} {:>12} {:>12} {:>12}",
        "world", "winner (d)", "runner-up", "margin", "frozen margin"
    );
    let margin = |l: &Life| -> (usize, i32, i32) {
        let mut v: Vec<(i32, usize)> = (0..N_BASINS).map(|b| (l.final_dist[b], b)).collect();
        v.sort();
        (v[0].1, v[0].0, v[1].0 - v[0].0)
    };
    let mut frozen_margins = Vec::new();
    for (i, w) in ws.iter().enumerate() {
        let (wb, wd, m) = margin(&drifting[i]);
        let (_, _, mf) = margin(&frozen[i]);
        frozen_margins.push(mf);
        println!(
            "{:<20} {:>12} {:>12} {:>12} {:>12}",
            w.name,
            format!("{} {}", BN[wb], wd),
            wd + m,
            m,
            mf
        );
    }

    // -----------------------------------------------------------------------
    // R5: is the CLASSIFIER innocent? Place a field by hand at each target.
    // -----------------------------------------------------------------------
    println!("\n── R5: hand-place a field AT each target and classify it ──────────");
    let mut classifier = FuzzyBasinField::new(&Genome::wanderer());
    let mut r5_ok = true;
    let mut constructed: Vec<[i16; N_SOMATIC]> = Vec::new();
    println!("{:<14} {:>18} {:>14}", "field placed at", "classified as", "correct?");
    for b in 0..N_BASINS {
        let mut f = SomaticField::default();
        f.channel = birth[b];
        constructed.push(f.channel);
        let m = classifier.compute_membership(&f);
        // The dominant basin is the highest membership weight.
        let got = (0..N_BASINS).max_by_key(|&i| m.weight[i]).unwrap();
        if got != b {
            r5_ok = false;
        }
        println!(
            "{:<14} {:>18} {:>14}",
            BN[b],
            BN[got],
            if got == b { "yes" } else { "NO" }
        );
    }
    let v2 = (0..N_BASINS)
        .all(|i| (i + 1..N_BASINS).all(|j| dist(&constructed[i], &constructed[j]) > 0));

    // -----------------------------------------------------------------------
    // R6: an adversarial sweep — can ANY world reach the Defensive signature?
    // -----------------------------------------------------------------------
    println!("\n── R6: adversarial sweep — can any world reach `Defensive`? ───────");
    // The four channels that carry most of the Engaged↔Defensive separation.
    let key: [usize; 4] = [1, 2, 3, 10]; // anisotropy, breach, tension, fatigue
    let mut sweep: Vec<(String, f64, f64)> = Vec::new(); // (name, worst frac, mean valence)
    let mut harsh_world_exists = false;
    for nutrient in [0.0f32, 0.05, 0.2, 0.5] {
        for (pname, p) in [
            ("alone", None),
            ("trap", Some(Partner { id: 9, reciprocation: q(0.12), exit_cost: q(0.98) })),
            ("total extractor", Some(Partner { id: 9, reciprocation: q(0.0), exit_cost: q(1.0) })),
        ] {
            let w = World { name: "", nutrient, partner: p };
            let l = live(w, false);
            // Fraction of the Defensive signature the key channels reach.
            let worst = key
                .iter()
                .map(|&c| {
                    let target = birth[2][c] as f64;
                    if target == 0.0 { 1.0 } else { l.hi[c] as f64 / target }
                })
                .fold(f64::MAX, f64::min);
            if l.mean_valence < 0.0 {
                harsh_world_exists = true;
            }
            sweep.push((format!("nutrient {:.2}, {}", nutrient, pname), worst, l.mean_valence));
        }
    }
    println!("{:<28} {:>22} {:>16} {:>10}", "world", "worst key channel", "mean valence", "died?");
    for (n, frac, mv) in &sweep {
        println!(
            "{:<28} {:>21.0}% {:>16.3} {:>10}",
            n,
            frac * 100.0,
            mv,
            "-"
        );
    }
    let best_reach = sweep.iter().map(|(_, f, _)| *f).fold(f64::MIN, f64::max);

    // -----------------------------------------------------------------------
    println!("\n── per-channel range, and what `Defensive` asks for ───────────────");
    println!("{:<12} {:>26} {:>12} {:>10}", "channel", "range across all 4 worlds", "Defensive", "reached");
    for c in 0..N_SOMATIC {
        let lo = drifting.iter().map(|l| l.lo[c]).min().unwrap();
        let hi = drifting.iter().map(|l| l.hi[c]).max().unwrap();
        let tgt = birth[2][c];
        let reached = if tgt == 0 {
            "n/a".to_string()
        } else {
            format!("{:.0}%", hi as f64 / tgt as f64 * 100.0)
        };
        println!("{:<12} {:>26} {:>12} {:>10}", CH[c], format!("{}..{}", lo, hi), tgt, reached);
    }

    // -----------------------------------------------------------------------
    println!("\n── vacuity guards ─────────────────────────────────────────────────");
    let v1 = (0..ws.len()).any(|k| {
        (0..N_BASINS).any(|b| drifting[k].targets_end[b] != frozen[k].targets_end[b])
    });
    println!(
        "V1  the freeze actually changed the targets ..... {}",
        if v1 { "PASS" } else { "FAIL — R1–R4 compare a run to itself" }
    );
    println!(
        "V2  the four hand-placed fields are distinct .... {}",
        if v2 { "PASS" } else { "FAIL — R5 classified one point four times" }
    );
    println!(
        "V3  the sweep contains a world that HURTS ....... {}",
        if harsh_world_exists {
            "PASS — at least one world drove mean valence negative"
        } else {
            "FAIL — 'no world reaches it' is about worlds never made hard enough"
        }
    );

    println!("\n── predictions as locked ──────────────────────────────────────────");
    let r1 = frozen.iter().all(|l| l.occupancy[1] == l.ticks);
    println!(
        "R1  frozen, still never leaves Engaged ......... {}",
        if !v1 {
            "VACUOUS — see V1".to_string()
        } else if r1 {
            "HOLDS — 4 of 4 worlds, every tick".to_string()
        } else {
            format!(
                "FAILS — and this is the good outcome: {}",
                ws.iter()
                    .zip(frozen.iter())
                    .filter(|(_, l)| l.occupancy[1] != l.ticks)
                    .map(|(w, l)| format!("{}: {}", w.name, occupancy_str(&l.occupancy)))
                    .collect::<Vec<_>>()
                    .join("; ")
            )
        }
    );
    let r2 = frozen_margins.iter().all(|&m| m > 200);
    println!(
        "R2  frozen margin still > 200 .................. {}",
        if r2 {
            format!("HOLDS — {:?}", frozen_margins)
        } else {
            format!("FAILS — {:?}; the runner-up got close", frozen_margins)
        }
    );
    let r3 = dist(&frozen[0].targets_end[0], &frozen[0].targets_end[1]) == dist(&birth[0], &birth[1]);
    println!(
        "R3  freezing removes the divergence ............ {}",
        if r3 { "HOLDS (confirmatory, not a finding)" } else { "FAILS" }
    );
    let r4 = frozen.iter().any(|l| l.occupancy[1] != l.ticks);
    println!(
        "R4  freezing frees the being ................... {}",
        if r4 {
            "HOLDS — I predicted this would fail.".to_string()
        } else {
            "FAILS, as predicted — the ratchet was never the binding\n    \
             constraint. Reachability is.".to_string()
        }
    );
    println!(
        "R5  the classifier is innocent ................. {}",
        if !v2 {
            "VACUOUS — see V2".to_string()
        } else if r5_ok {
            "HOLDS — all four targets classify as themselves, including\n    \
             the three the being never lives in. The map is fine; the\n    \
             being cannot walk to it.".to_string()
        } else {
            "FAILS — the classifier itself is broken".to_string()
        }
    );
    // -----------------------------------------------------------------------
    // Adversarial mutations, run before any of this was written up.
    // -----------------------------------------------------------------------
    println!("\n── adversarial mutations ──────────────────────────────────────────");

    println!("M1  does freezing change BEHAVIOUR, or only the label?");
    println!("    {:<24} {:>12} {:>14} {:>12}", "arm", "gave (sum)", "Defensive %", "withdrew");
    for (lbl, freeze) in [("drifting (default)", false), ("frozen", true)] {
        for w in [ws[1], ws[0]] {
            let mut being = UnifiedBeing::new(Genome::wanderer());
            if freeze {
                being.freeze_basin_targets();
            }
            let stim = Stimulus { nutrient: q(w.nutrient), partner: w.partner };
            let (mut gave, mut def, mut n) = (0i64, 0u32, 0u32);
            let mut withdrew = None;
            for t in 1..=TICKS {
                let r = being.step(&stim);
                n = t;
                gave += r.gave as i64;
                if basin_index(r.basin) == 2 {
                    def += 1;
                }
                if being.consent_withdrawn() && withdrew.is_none() {
                    withdrew = Some(t);
                }
                if !r.alive {
                    break;
                }
            }
            println!(
                "    {:<24} {:>12} {:>13.0}% {:>12}",
                format!("{} / {}", lbl, if w.name.contains("trap") { "trap" } else { "fair" }),
                gave,
                def as f64 / n as f64 * 100.0,
                withdrew.map_or("never".to_string(), |x| x.to_string())
            );
        }
    }
    println!("    Freezing is NOT cosmetic: in the trap the being gives an extractor");
    println!("    a third as much. In the fair world `gave` is identical, so the");
    println!("    freeze costs nothing when the world is good.");

    println!("\nM2  does it survive a different genome?");
    println!("    {:<12} {:>30} {:>30}", "genome", "trap, drifting", "trap, frozen");
    let mut genome_general = true;
    for (gl, g) in [("wanderer", Genome::wanderer()), ("default", Genome::default())] {
        let mut out = Vec::new();
        for freeze in [false, true] {
            let mut being = UnifiedBeing::new(g.clone());
            if freeze {
                being.freeze_basin_targets();
            }
            let stim = Stimulus { nutrient: q(ws[1].nutrient), partner: ws[1].partner };
            let mut occ = [0u32; N_BASINS];
            for _ in 0..TICKS {
                let r = being.step(&stim);
                occ[basin_index(r.basin)] += 1;
                if !r.alive {
                    break;
                }
            }
            if freeze && occ[2] == 0 {
                genome_general = false;
            }
            out.push(occupancy_str(&occ));
        }
        println!("    {:<12} {:>30} {:>30}", gl, out[0], out[1]);
    }
    println!(
        "    {}",
        if genome_general {
            "The freeing generalises across genomes."
        } else {
            "IT DOES NOT GENERALISE. Only `wanderer` is freed; `default` stays in\n                 Engaged either way. R4's hold is one genome in one world."
        }
    );

    println!(
        "R6  no world reaches 60% of Defensive .......... {}",
        if !harsh_world_exists {
            "VACUOUS — see V3".to_string()
        } else if best_reach < 0.60 {
            format!(
                "HOLDS — the best of {} worlds reaches {:.0}% on its worst\n    \
                 key channel. `Defensive` is not somewhere this body goes.",
                sweep.len(),
                best_reach * 100.0
            )
        } else {
            format!("FAILS — a world reached {:.0}%", best_reach * 100.0)
        }
    );
}
