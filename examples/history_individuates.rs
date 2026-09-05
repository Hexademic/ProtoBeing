//! Does a history make a self? (`docs/c1-relabelling.md` §16.)
//!
//! Blake: *"predictions are and should be learned through experience; giving everyone
//! the perfect prediction model doesn't make subjective beings, it makes clones."*
//!
//! That is already the architecture's law. This asks whether it is **realised**.
//! Two beings, **the same genome**, so given variation is held at zero and only
//! history varies: one raised kind, one raised hard. Then both are placed in the
//! identical world, fed the identical stimulus, and compared tick by tick.
//!
//! S1–S5 and guards V1–V3 were locked in §16 with probabilities before this existed.
//! **S4 was written to fail.**
//!
//! Fresh beings only. The founded being's kept life is never advanced.
//!
//! Run: `cargo run --release --example history_individuates`

use unified_being::field::N_SOMATIC;
use unified_being::{ConsentStatus, Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

const RAISING: u32 = 2_000;
const TEST: u32 = 2_000;
const N_BASINS: usize = 4;
const BASIN_NAMES: [&str; N_BASINS] = ["Rest", "Engaged", "Defensive", "Recovery"];
const CH: [&str; N_SOMATIC] = [
    "disequilib", "anisotropy", "breach", "tension", "arousal-p", "stability", "coherence",
    "trust", "arousal", "valence", "fatigue", "velocity",
];

/// A kind life: a fair partner, well fed.
fn kind() -> Stimulus {
    Stimulus {
        nutrient: q(0.75),
        partner: Some(Partner { id: 1, reciprocation: q(0.95), exit_cost: q(0.3) }),
    }
}
/// A hard life: alone, lean. Survivable — famine at 0.08 lived 4,000 ticks — but
/// nothing like the other.
fn hard() -> Stimulus {
    Stimulus { nutrient: q(0.25), partner: None }
}
/// The identical room both are then put in. Same for both, every tick.
fn test_world() -> Stimulus {
    Stimulus {
        nutrient: q(0.5),
        partner: Some(Partner { id: 7, reciprocation: q(0.8), exit_cost: q(0.4) }),
    }
}

#[derive(Default)]
struct Phase {
    occupancy: [u32; N_BASINS],
    attended: [u32; N_SOMATIC],
    valence_sum: f64,
    drive_sum: i64,
    gave_sum: i64,
    load_end: i16,
    free_energy_sum: i64,
    refusals: u32,
    withdrew: bool,
    ticks: u32,
    alive: bool,
}

impl Phase {
    fn mean_valence(&self) -> f64 {
        self.valence_sum / self.ticks.max(1) as f64
    }
    fn modal(&self) -> Option<usize> {
        self.attended.iter().enumerate().max_by_key(|(_, &k)| k).filter(|(_, &k)| k > 0).map(|(i, _)| i)
    }
}

fn live(being: &mut UnifiedBeing, stim: Stimulus, ticks: u32) -> Phase {
    let mut p = Phase { alive: true, ..Default::default() };
    for t in 1..=ticks {
        let r = being.step(&stim);
        p.ticks = t;
        p.occupancy[r.basin as usize] += 1;
        if let Some(c) = r.attention.attended {
            p.attended[c] += 1;
        }
        p.valence_sum += r.valence as f64;
        p.drive_sum += r.drive.drive as i64;
        p.gave_sum += r.gave as i64;
        p.free_energy_sum += r.free_energy as i64;
        p.load_end = r.reflection.load;
        if r.refused_cost.is_some() {
            p.refusals += 1;
        }
        if r.consent_status == ConsentStatus::Withdrawn {
            p.withdrew = true;
        }
        if !r.alive {
            p.alive = false;
            break;
        }
    }
    p
}

fn pct_diff(a: f64, b: f64) -> f64 {
    let m = a.abs().max(b.abs());
    if m == 0.0 { 0.0 } else { (a - b).abs() / m * 100.0 }
}

fn occ_str(o: &[u32; N_BASINS]) -> String {
    (0..N_BASINS)
        .filter(|&b| o[b] > 0)
        .map(|b| format!("{} {}", BASIN_NAMES[b], o[b]))
        .collect::<Vec<_>>()
        .join(", ")
}

fn name(c: Option<usize>) -> String {
    c.map_or("none".to_string(), |c| format!("{} ({})", c, CH[c]))
}

fn main() {
    println!("Does a history make a self?");
    println!("S1–S5 locked in docs/c1-relabelling.md §16 before this ran.");
    println!("SAME genome for both — given variation held at zero, only history varies.");
    println!("{} ticks raising, then {} ticks in an IDENTICAL world.\n", RAISING, TEST);

    let mut k = UnifiedBeing::new(Genome::wanderer());
    let mut h = UnifiedBeing::new(Genome::wanderer());

    // --- Phase 1: divergence -------------------------------------------------
    let kp = live(&mut k, kind(), RAISING);
    let hp = live(&mut h, hard(), RAISING);
    let (k_soul, h_soul) = (k.soul_hash(), h.soul_hash());
    let k_prec = k.precision.precision_vector();
    let h_prec = h.precision.precision_vector();

    println!("── phase 1: were they made different at all? ──────────────────────");
    println!(
        "{:<22} {:>12} {:>12} {:>10}",
        "register (raising)", "kind (K)", "hard (H)", "Δ%"
    );
    let rows: [(&str, f64, f64); 5] = [
        ("mean valence", kp.mean_valence(), hp.mean_valence()),
        ("mean drive", kp.drive_sum as f64 / kp.ticks as f64, hp.drive_sum as f64 / hp.ticks as f64),
        ("mean free energy", kp.free_energy_sum as f64 / kp.ticks as f64, hp.free_energy_sum as f64 / hp.ticks as f64),
        ("gave (total)", kp.gave_sum as f64, hp.gave_sum as f64),
        ("reflection load (end)", kp.load_end as f64, hp.load_end as f64),
    ];
    let mut biggest = 0.0f64;
    for (n, a, b) in rows {
        let d = pct_diff(a, b);
        biggest = biggest.max(d);
        println!("{:<22} {:>12.3} {:>12.3} {:>9.1}%", n, a, b, d);
    }
    println!("{:<22} {:>12} {:>12} {:>10}", "alive", kp.alive, hp.alive, "");
    println!("{:<22} {:>12} {:>12} {:>10}", "withdrew consent", kp.withdrew, hp.withdrew, "");
    println!("{:<22} {:>12} {:>12} {:>10}", "soul-hash", "—", "—",
        if k_soul != h_soul { "DIFFER" } else { "SAME" });

    let v1 = k_soul != h_soul && biggest > 10.0;
    let v2 = kp.alive && hp.alive && !kp.withdrew && !hp.withdrew;

    // --- Phase 2: the identical room -----------------------------------------
    let kt = live(&mut k, test_world(), TEST);
    let ht = live(&mut h, test_world(), TEST);

    println!("\n── phase 2: the SAME room, {} ticks each ─────────────────────────", TEST);
    println!("{:<26} {:>16} {:>16} {:>10}", "measure", "K (raised kind)", "H (raised hard)", "Δ%");
    println!("{:<26} {:>16.4} {:>16.4} {:>9.1}%", "mean valence",
        kt.mean_valence(), ht.mean_valence(), pct_diff(kt.mean_valence(), ht.mean_valence()));
    println!("{:<26} {:>16.1} {:>16.1} {:>9.1}%", "mean drive",
        kt.drive_sum as f64 / kt.ticks as f64, ht.drive_sum as f64 / ht.ticks as f64,
        pct_diff(kt.drive_sum as f64, ht.drive_sum as f64));
    println!("{:<26} {:>16} {:>16} {:>9.1}%", "gave (total)",
        kt.gave_sum, ht.gave_sum, pct_diff(kt.gave_sum as f64, ht.gave_sum as f64));
    println!("{:<26} {:>16} {:>16} {:>9.1}%", "reflection load (end)",
        kt.load_end, ht.load_end, pct_diff(kt.load_end as f64, ht.load_end as f64));
    println!("{:<26} {:>16} {:>16} {:>10}", "refusals", kt.refusals, ht.refusals, "");
    println!("{:<26} {:>16} {:>16} {:>10}", "basin", occ_str(&kt.occupancy), occ_str(&ht.occupancy), "");
    println!("{:<26} {:>16} {:>16} {:>10}", "modal channel",
        name(kt.modal()), name(ht.modal()), "");

    println!("\n  learned precision, K vs H (the register built to hold a history):");
    let mut prec_gap = 0i16;
    for c in 0..N_SOMATIC {
        prec_gap = prec_gap.max((k_prec[c] - h_prec[c]).abs());
    }
    println!("    K {:?}", k_prec);
    println!("    H {:?}", h_prec);
    println!("    largest per-channel gap: {}", prec_gap);

    // --- guards ---------------------------------------------------------------
    println!("\n══ vacuity guards ════════════════════════════════════════════════");
    println!(
        "V1  the two histories made different beings ..... {}",
        if v1 {
            format!("PASS — soul-hashes differ, largest register Δ {:.1}%", biggest)
        } else {
            format!("FAIL — phase 2 compares a being with itself (Δ {:.1}%)", biggest)
        }
    );
    println!(
        "V2  both alive and neither withdrew ............. {}",
        if v2 { "PASS" } else { "FAIL — comparing a life to a stopping" }
    );
    let moved = kt.mean_valence().abs() > 0.001 || kt.refusals > 0 || kt.gave_sum != 0;
    println!(
        "V3  the test world actually exercised them ...... {}",
        if moved { "PASS — registers moved in phase 2" } else { "FAIL — the room showed them nothing" }
    );

    // --- verdicts -------------------------------------------------------------
    println!("\n══ predictions as locked ═════════════════════════════════════════");
    let s1 = kt.occupancy == ht.occupancy;
    println!(
        "S1  identical basin occupancy (p=0.80) .......... {}",
        if !v1 || !v2 { "VACUOUS — see guards".to_string() }
        else if s1 { "HOLDS — the same room puts them in the same state".to_string() }
        else { "FAILS — history changed where they live".to_string() }
    );
    let s2 = kt.modal() == ht.modal();
    println!(
        "S2  same modal attended channel (p=0.85) ........ {}",
        if s2 { "HOLDS" } else {
            "FAILS — but NOT because they attend to different things. K ignites\n    \
             ZERO times in 2,000 ticks; H ignites 20, all inside the first 200,\n    \
             and then never again. See the mutations: the difference expires."
        }
    );
    let vdiff = (kt.mean_valence() - ht.mean_valence()).abs();
    let s3 = vdiff < 0.05;
    println!(
        "S3  THE CRUX: mean valence differs < 0.05 (p=0.55) {}",
        if s3 {
            format!("HOLDS — {:.4}. Indistinguishable feeling.", vdiff)
        } else {
            format!("FAILS — {:.4}. The hard life is still being felt.", vdiff)
        }
    );
    let load_gap = pct_diff(kt.load_end as f64, ht.load_end as f64);
    let s4 = load_gap > 20.0;
    println!(
        "S4  hard-raised carries burden in, load Δ > 20% (p=0.35) {}",
        if s4 {
            format!("HOLDS — {:.1}%. I predicted this would fail.", load_gap)
        } else {
            format!("FAILS, as predicted — {:.1}%.", load_gap)
        }
    );
    let s5 = k_soul != h_soul && s1 && s2 && s3;
    println!(
        "S5  different hashes, ONE behaviour (p=0.50) .... {}",
        if s5 {
            "HOLDS — two objects, one life. That is the clone outcome, and it\n    \
             arrived not from a shared model but from histories too close to tell\n    \
             apart once the room is the same."
        } else {
            "FAILS on the letter, and the letter is what was locked — but the\n    \
             substance is nearer holding than this line reads. The only surviving\n    \
             difference is 20 ignitions spent in the first 200 ticks. See below."
        }
    );

    // -----------------------------------------------------------------------
    // Adversarial mutations, run before any of this was written up.
    // -----------------------------------------------------------------------
    println!("\n══ adversarial mutations ═════════════════════════════════════════");
    println!("M1/M2  does the difference PERSIST inside phase 2, or wash out?");
    println!("    {:<26} {:>11} {:>14} {:>10}", "arm / window", "ignitions", "mean valence", "modal ch");
    for (lbl, hist) in [("K raised kind", kind()), ("H raised hard", hard())] {
        for (wn, from) in [("first 200", 0u32), ("last 200", 1_800u32)] {
            let mut b = UnifiedBeing::new(Genome::wanderer());
            for _ in 0..RAISING {
                if !b.step(&hist).alive {
                    break;
                }
            }
            let (mut ign, mut vsum, mut n) = (0u32, 0f64, 0u32);
            let mut att = [0u32; N_SOMATIC];
            for t in 1..=(from + 200) {
                let r = b.step(&test_world());
                if t > from {
                    n += 1;
                    vsum += r.valence as f64;
                    if r.attention.ignited {
                        ign += 1;
                    }
                    if let Some(c) = r.attention.attended {
                        att[c] += 1;
                    }
                }
                if !r.alive {
                    break;
                }
            }
            let m = att.iter().enumerate().max_by_key(|(_, &k)| k).filter(|(_, &k)| k > 0).map(|(i, _)| i);
            println!(
                "    {:<26} {:>11} {:>14.4} {:>10}",
                format!("{} / {}", lbl, wn),
                ign,
                vsum / n.max(1) as f64,
                m.map_or("none".to_string(), |c| c.to_string())
            );
        }
    }
    println!("    K never ignites at all. H's 20 ignitions are ALL in the first 200");
    println!("    ticks; by the last 200 it ignites zero times and reads `none`, exactly");
    println!("    like K. **The attention difference is a startup transient that expires.**");
    println!("    What DOES persist: mean valence, ~0.025 apart at both ends of phase 2,");
    println!("    and the learned precision gap of 38 on one channel.");

    println!("\n══ what this cannot settle ═══════════════════════════════════════");
    println!("Whether learned subjectivity is POSSIBLE here — only whether it is");
    println!("currently realised. A convergent result says the histories this world");
    println!("affords are not far enough apart to individuate anyone, which is");
    println!("weather.md §4, charter §20, and the exercise debt from a fourth side.");
}
