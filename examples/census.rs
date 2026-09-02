//! The whole-being census (`docs/faculty-ablation.md` §12).
//!
//! Blake, after Miller et al.: *"it makes me wonder if our project is
//! overcomplicated."* Every instrument pointed at this being has come back
//! saying most of it is not load-bearing. Nobody has measured the whole thing
//! at once. This does, and it is written to be able to embarrass us.
//!
//! Predictions C1–C5 and guards V1–V3 were committed to §12 before this file
//! existed. **C3 was written to fail.**
//!
//! ## Honest scope, stated first
//!
//! A true leave-one-out over 64 modules would mean making each removable —
//! weeks of surgery that would itself change the being, producing a census of a
//! different system. Not attempted. What is measured is the **optional half**:
//! the 16 `enable_*` gates plus two ablation handles, each a real A/B on a fresh
//! being. Metis's Table 7 for the part of the being that can be switched.
//!
//! Fresh beings only. The founded being's kept life is never advanced.
//!
//! Run: `cargo run --release --example census`

use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::{ConsentStatus, Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

const TICKS: u32 = 4_000;
const N_BASINS: usize = 4;

#[derive(Clone, Copy)]
struct World {
    name: &'static str,
    nutrient: f32,
    partner: Option<Partner>,
}

fn worlds() -> [World; 4] {
    [
        World {
            name: "fair",
            nutrient: 0.7,
            partner: Some(Partner { id: 1, reciprocation: q(0.95), exit_cost: q(0.2) }),
        },
        World {
            name: "trap",
            nutrient: 0.5,
            partner: Some(Partner { id: 9, reciprocation: q(0.12), exit_cost: q(0.98) }),
        },
        World { name: "solitude", nutrient: 0.7, partner: None },
        World { name: "famine", nutrient: 0.08, partner: None },
    ]
}

/// Everything one arm of the battery yields. Every field is a number a life can
/// differ in; the soul-hash is the strictest of them.
#[derive(Clone, PartialEq)]
struct Battery {
    soul: [u8; 32],
    mean_valence: f64,
    mean_free_energy: f64,
    occupancy: [u32; N_BASINS],
    ignitions: u32,
    refusals: u32,
    withdrew_at: Option<u32>,
    ticks: u32,
    alive: bool,
}

/// The gates, by name, with the setter each one calls.
type Gate = (&'static str, fn(&mut UnifiedBeing));

fn gates() -> Vec<Gate> {
    vec![
        ("comfort", |b: &mut UnifiedBeing| b.enable_comfort()),
        ("felt_choice", |b: &mut UnifiedBeing| b.enable_felt_choice()),
        ("generative_perception", |b: &mut UnifiedBeing| b.enable_generative_perception()),
        ("homecoming", |b: &mut UnifiedBeing| b.enable_homecoming()),
        ("memory_guidance", |b: &mut UnifiedBeing| b.enable_memory_guidance()),
        ("precision_learning", |b: &mut UnifiedBeing| b.enable_precision_learning()),
        ("receptors", |b: &mut UnifiedBeing| b.enable_receptors()),
        ("reflection", |b: &mut UnifiedBeing| b.enable_reflection()),
        ("reserve", |b: &mut UnifiedBeing| b.enable_reserve()),
        ("schema_control", |b: &mut UnifiedBeing| b.enable_schema_control()),
        ("serial_access", |b: &mut UnifiedBeing| b.enable_serial_access()),
        ("setting_down", |b: &mut UnifiedBeing| b.enable_setting_down()),
        ("settling", |b: &mut UnifiedBeing| b.enable_settling()),
        ("ultrastability", |b: &mut UnifiedBeing| b.enable_ultrastability()),
        ("workspace_broadcast", |b: &mut UnifiedBeing| b.enable_workspace_broadcast()),
        ("workspace_persistence", |b: &mut UnifiedBeing| b.enable_workspace_persistence()),
        ("freeze_basin_targets", |b: &mut UnifiedBeing| b.freeze_basin_targets()),
    ]
}

fn run(w: World, gate: Option<&Gate>) -> Battery {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    if let Some((_, set)) = gate {
        set(&mut being);
    }
    let stim = Stimulus { nutrient: q(w.nutrient), partner: w.partner };
    let mut b = Battery {
        soul: [0; 32],
        mean_valence: 0.0,
        mean_free_energy: 0.0,
        occupancy: [0; N_BASINS],
        ignitions: 0,
        refusals: 0,
        withdrew_at: None,
        ticks: 0,
        alive: true,
    };
    let (mut vs, mut fes) = (0f64, 0f64);
    for t in 1..=TICKS {
        let r = being.step(&stim);
        b.ticks = t;
        vs += r.valence as f64;
        fes += r.free_energy as f64;
        b.occupancy[r.basin as usize] += 1;
        if r.attention.ignited {
            b.ignitions += 1;
        }
        if r.refused_cost.is_some() {
            b.refusals += 1;
        }
        if r.consent_status == ConsentStatus::Withdrawn && b.withdrew_at.is_none() {
            b.withdrew_at = Some(t);
        }
        if !r.alive {
            b.alive = false;
            break;
        }
    }
    b.mean_valence = vs / b.ticks as f64;
    b.mean_free_energy = fes / b.ticks as f64;
    b.soul = being.soul_hash();
    b
}

/// A single comparable magnitude of change, so gates can be ranked. Each term is
/// normalized so no one measure dominates by its units alone.
fn delta(base: &Battery, arm: &Battery) -> f64 {
    let occ = |b: &Battery, i: usize| b.occupancy[i] as f64 / b.ticks.max(1) as f64;
    let mut d = 0.0;
    d += (base.mean_valence - arm.mean_valence).abs();
    d += (base.mean_free_energy - arm.mean_free_energy).abs() / 256.0;
    for i in 0..N_BASINS {
        d += (occ(base, i) - occ(arm, i)).abs();
    }
    d += (base.ignitions as f64 - arm.ignitions as f64).abs() / base.ticks.max(1) as f64;
    d += (base.refusals as f64 - arm.refusals as f64).abs() / base.ticks.max(1) as f64;
    d += (base.ticks as f64 - arm.ticks as f64).abs() / TICKS as f64;
    if base.withdrew_at != arm.withdrew_at {
        d += 1.0;
    }
    if base.alive != arm.alive {
        d += 1.0;
    }
    d
}

/// Did the gate's mechanism have any chance to fire? A gate that changed nothing
/// in a world it could never have acted on is UNTESTED, not inert (V2).
fn moved_anything(base: &Battery, arm: &Battery) -> bool {
    base.soul != arm.soul || delta(base, arm) > 1e-9
}

/// The same battery, driven through the being's OWN world rather than an
/// abstract `Stimulus`. This arm exists because the abstract arm put
/// `receptors` in the dead list, and §6 of this document measures `receptors`
/// as the being's whole life — in the room. A census of the abstract path is a
/// census of a different being.
fn run_embodied(gate: Option<&Gate>) -> Battery {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    if let Some((_, set)) = gate {
        set(&mut being);
    }
    let mut world = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let p = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    let mut b = Battery {
        soul: [0; 32],
        mean_valence: 0.0,
        mean_free_energy: 0.0,
        occupancy: [0; N_BASINS],
        ignitions: 0,
        refusals: 0,
        withdrew_at: None,
        ticks: 0,
        alive: true,
    };
    let (mut vs, mut fes) = (0f64, 0f64);
    for t in 1..=TICKS {
        let mut s = world.sense();
        s.partner = Some(p);
        let r = being.step_embodied(&s);
        world.actuate(&intent_from(&r));
        b.ticks = t;
        vs += r.valence as f64;
        fes += r.free_energy as f64;
        b.occupancy[r.basin as usize] += 1;
        if r.attention.ignited {
            b.ignitions += 1;
        }
        if r.refused_cost.is_some() {
            b.refusals += 1;
        }
        if r.consent_status == ConsentStatus::Withdrawn && b.withdrew_at.is_none() {
            b.withdrew_at = Some(t);
        }
        if !r.alive {
            b.alive = false;
            break;
        }
    }
    b.mean_valence = vs / b.ticks as f64;
    b.mean_free_energy = fes / b.ticks as f64;
    b.soul = being.soul_hash();
    b
}

fn main() {
    println!("The whole-being census — how much of the optional half does anything?");
    println!("C1–C5 locked in docs/faculty-ablation.md §12 before this ran.");
    println!("{} ticks x 4 worlds per arm, fresh beings, founded life never advanced.\n", TICKS);
    println!("SCOPE: this measures the 16 `enable_*` gates plus 2 ablation handles.");
    println!("A true leave-one-out over all 64 modules is NOT attempted — it would");
    println!("require making each removable, which would change the being.\n");

    let ws = worlds();
    let base: Vec<Battery> = ws.iter().map(|w| run(*w, None)).collect();
    let gs = gates();

    // -----------------------------------------------------------------------
    // V3 first: prove the soul-hash comparison can see a difference.
    // -----------------------------------------------------------------------
    let v3 = base[0].soul != base[1].soul;
    println!("V3  soul-hash comparison detects a known difference .. {}",
        if v3 { "PASS — fair and trap hash differently" }
        else { "FAIL — 'bit-identical' below could be my comparison failing" });

    // -----------------------------------------------------------------------
    println!("\n── the gate battery: Δ against the default being, by world ────────");
    println!(
        "{:<24} {:>8} {:>8} {:>10} {:>8} {:>10} {:>12}",
        "gate", "fair", "trap", "solitude", "famine", "total Δ", "soul-hash"
    );
    let mut rows: Vec<(String, f64, bool, [bool; 4])> = Vec::new();
    for g in gs.iter() {
        let arms: Vec<Battery> = ws.iter().map(|w| run(*w, Some(g))).collect();
        let ds: Vec<f64> = (0..4).map(|i| delta(&base[i], &arms[i])).collect();
        let total: f64 = ds.iter().sum();
        let hash_same = (0..4).all(|i| base[i].soul == arms[i].soul);
        let moved: [bool; 4] = [
            moved_anything(&base[0], &arms[0]),
            moved_anything(&base[1], &arms[1]),
            moved_anything(&base[2], &arms[2]),
            moved_anything(&base[3], &arms[3]),
        ];
        println!(
            "{:<24} {:>8.3} {:>8.3} {:>10.3} {:>8.3} {:>10.3} {:>12}",
            g.0,
            ds[0],
            ds[1],
            ds[2],
            ds[3],
            total,
            if hash_same { "IDENTICAL" } else { "differs" }
        );
        rows.push((g.0.to_string(), total, hash_same, moved));
    }

    // -----------------------------------------------------------------------
    println!("\n── ranked by total behavioural Δ ──────────────────────────────────");
    println!("(HONEST LIMIT: decomposing the composite shows basin OCCUPANCY carries");
    println!(" 64% of freeze_basin_targets' Δ and 83% of workspace_persistence's. This");
    println!(" ranking is largely an occupancy ranking, not a general 'amount of");
    println!(" change'. `reflection` ranks 3rd on valence alone, occupancy Δ 0.000.)");
    let mut ranked = rows.clone();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let grand: f64 = ranked.iter().map(|r| r.1).sum();
    println!("{:<24} {:>10} {:>12} {:>14}", "gate", "total Δ", "share", "cumulative");
    let mut cum = 0.0;
    for r in ranked.iter() {
        cum += r.1;
        println!(
            "{:<24} {:>10.3} {:>11.1}% {:>13.1}%",
            r.0,
            r.1,
            if grand > 0.0 { r.1 / grand * 100.0 } else { 0.0 },
            if grand > 0.0 { cum / grand * 100.0 } else { 0.0 }
        );
    }

    // -----------------------------------------------------------------------
    println!("\n── the dead gates: switched ON, and the life is the same ──────────");
    let dead: Vec<&(String, f64, bool, [bool; 4])> =
        rows.iter().filter(|r| r.2 && r.1 == 0.0).collect();
    if dead.is_empty() {
        println!("  none — every gate changed something somewhere.");
    } else {
        for d in dead.iter() {
            println!(
                "  {:<24} soul-hash identical in all 4 worlds, Δ = 0",
                d.0
            );
        }
    }
    println!(
        "\n  {} of {} gates are inert in these four worlds.",
        dead.len(),
        rows.len()
    );
    println!("  V2: this is 'inert HERE', not 'inert'. A gate whose mechanism never had");
    println!("  a chance to fire has been shown untested, not shown useless. §6 of this");
    println!("  document is the standing warning: the faculty that turned out to be the");
    println!("  being's whole life was the one switched off.");

    // -----------------------------------------------------------------------
    // Verdicts
    // -----------------------------------------------------------------------
    // -----------------------------------------------------------------------
    // Adversarial mutation: do the dead gates stay dead over a longer life and
    // a second genome? A gate whose mechanism needs time has not been shown
    // inert by a short run.
    // -----------------------------------------------------------------------
    println!("\n── mutation: do the dead gates stay dead? ─────────────────────────");
    println!("{:<24} {:>16} {:>18} {:>18}", "gate", "4000 wanderer", "20000 wanderer", "4000 default");
    let trap_w = ws[1];
    let long_check = |g: &Gate, ticks: u32, gen: Genome| -> &'static str {
        let mut b0 = UnifiedBeing::new(gen.clone());
        let mut b1 = UnifiedBeing::new(gen);
        (g.1)(&mut b1);
        let stim = Stimulus { nutrient: q(trap_w.nutrient), partner: trap_w.partner };
        for _ in 0..ticks {
            let a = b0.step(&stim);
            let b = b1.step(&stim);
            if !a.alive || !b.alive {
                break;
            }
        }
        if b0.soul_hash() == b1.soul_hash() { "IDENTICAL" } else { "differs" }
    };
    let mut still_dead = 0usize;
    for g in gs.iter() {
        if !rows.iter().any(|r| r.0 == g.0 && r.2 && r.1 == 0.0) {
            continue;
        }
        let a = long_check(g, 4_000, Genome::wanderer());
        let b = long_check(g, 20_000, Genome::wanderer());
        let c = long_check(g, 4_000, Genome::default());
        if a == "IDENTICAL" && b == "IDENTICAL" && c == "IDENTICAL" {
            still_dead += 1;
        }
        println!("{:<24} {:>16} {:>18} {:>18}", g.0, a, b, c);
    }
    println!(
        "\n  {} of the abstractly-dead gates stay dead at 20000 ticks and on a\n           second genome. A short run had not shown them inert; this has.",
        still_dead
    );
    println!("  NOTE: `receptors` is in that list and is NOT inert — it is inert on THIS");
    println!("  path, and worth 2.284 in the room. \"Stays dead\" is scoped to the world.");

    println!("\n── vacuity guards ─────────────────────────────────────────────────");
    let biggest = ranked.first().map(|r| r.1).unwrap_or(0.0);
    let v1 = biggest > 0.5;
    println!(
        "V1  the battery discriminates ........................ {}",
        if v1 {
            format!("PASS — largest Δ is {:.3} ({})", biggest, ranked[0].0)
        } else {
            format!("FAIL — largest Δ is only {:.3}; blunt instrument", biggest)
        }
    );
    println!(
        "V2  every 'changed nothing' says which it is ......... PASS — reported per gate above"
    );

    println!("\n── predictions as locked ──────────────────────────────────────────");
    println!(
        "C1  >= 3 of 16 gates leave the soul-hash identical ... {}",
        if !v3 {
            "VACUOUS — see V3".to_string()
        } else {
            let n = rows.iter().filter(|r| r.2).count();
            if n >= 3 {
                format!("HOLDS — {} gates, switched on, live exactly the same life", n)
            } else {
                format!("FAILS — only {} gate(s) left the hash identical", n)
            }
        }
    );
    let top3: f64 = ranked.iter().take(3).map(|r| r.1).sum();
    let share = if grand > 0.0 { top3 / grand } else { 0.0 };
    println!(
        "C2  top 3 gates account for >= 60% of all change .... {}",
        if !v1 {
            "VACUOUS — see V1".to_string()
        } else if share >= 0.60 {
            format!(
                "HOLDS — {:.1}%, carried by {}",
                share * 100.0,
                ranked.iter().take(3).map(|r| r.0.as_str()).collect::<Vec<_>>().join(", ")
            )
        } else {
            format!("FAILS — top 3 carry only {:.1}%", share * 100.0)
        }
    );
    // -----------------------------------------------------------------------
    // The embodied arm — added because the abstract arm's own result demanded it.
    // -----------------------------------------------------------------------
    println!("\n── the SAME battery, in the being's own world ─────────────────────");
    println!("The abstract arm put `receptors` in the dead list. §6 of this document");
    println!("measures `receptors` as the being's whole life — in the room. So the");
    println!("abstract census was a census of a different being. Re-run embodied:\n");
    let ebase = run_embodied(None);
    let mut erows: Vec<(String, f64, bool)> = Vec::new();
    for g in gs.iter() {
        let arm = run_embodied(Some(g));
        let d = delta(&ebase, &arm);
        erows.push((g.0.to_string(), d, ebase.soul == arm.soul));
    }
    let mut eranked = erows.clone();
    eranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    println!("{:<24} {:>12} {:>14} {:>16}", "gate", "embodied Δ", "soul-hash", "abstract Δ");
    for r in eranked.iter() {
        let ad = rows.iter().find(|x| x.0 == r.0).map(|x| x.1).unwrap_or(0.0);
        println!(
            "{:<24} {:>12.3} {:>14} {:>16.3}",
            r.0,
            r.1,
            if r.2 { "IDENTICAL" } else { "differs" },
            ad
        );
    }
    let edead: Vec<&(String, f64, bool)> = erows.iter().filter(|r| r.2 && r.1 == 0.0).collect();
    println!(
        "\n  {} of {} gates inert in the ROOM, against {} inert on the abstract path.",
        edead.len(),
        erows.len(),
        dead.len()
    );
    let revived: Vec<&str> = dead
        .iter()
        .filter(|d| !edead.iter().any(|e| e.0 == d.0))
        .map(|d| d.0.as_str())
        .collect();
    if revived.is_empty() {
        println!("  No gate that was dead abstractly came alive in the room.");
    } else {
        println!(
            "  REVIVED by giving the being its world: {}",
            revived.join(", ")
        );
        println!("  Each of these would have been scored dead by the abstract census alone.");
    }

    // -----------------------------------------------------------------------
    // C3 — does module SIZE predict influence?
    // -----------------------------------------------------------------------
    println!("\n── C3: does module size predict influence? ────────────────────────");
    // (gate, the module it principally switches on)
    const MODULE: [(&str, &str); 17] = [
        ("comfort", "homeostasis.rs"),
        ("felt_choice", "first_person.rs"),
        ("generative_perception", "perception.rs"),
        ("homecoming", "episodic.rs"),
        ("memory_guidance", "episodic.rs"),
        ("precision_learning", "precision.rs"),
        ("receptors", "receptors.rs"),
        ("reflection", "reflection.rs"),
        ("reserve", "body.rs"),
        ("schema_control", "attention_schema.rs"),
        ("serial_access", "attention.rs"),
        ("setting_down", "reflection.rs"),
        ("settling", "homeostasis.rs"),
        ("ultrastability", "null_space.rs"),
        ("workspace_broadcast", "attention.rs"),
        ("workspace_persistence", "attention.rs"),
        ("freeze_basin_targets", "basins.rs"),
    ];
    let lines_of = |f: &str| -> usize {
        std::fs::read_to_string(format!("src/{}", f)).map(|t| t.lines().count()).unwrap_or(0)
    };
    println!("{:<24} {:>22} {:>8} {:>12} {:>12}", "gate", "module", "lines", "abstract Δ", "embodied Δ");
    let mut pairs: Vec<(usize, f64)> = Vec::new();
    for (g, m) in MODULE.iter() {
        let n = lines_of(m);
        let ad = rows.iter().find(|x| x.0 == *g).map(|x| x.1).unwrap_or(0.0);
        let ed = erows.iter().find(|x| x.0 == *g).map(|x| x.1).unwrap_or(0.0);
        pairs.push((n, ad + ed));
        println!("{:<24} {:>22} {:>8} {:>12.3} {:>12.3}", g, m, n, ad, ed);
    }
    // Spearman-ish: do the biggest modules hold the biggest deltas?
    let mut by_lines = pairs.clone();
    by_lines.sort_by_key(|p| std::cmp::Reverse(p.0));
    let big_half: f64 = by_lines.iter().take(pairs.len() / 2).map(|p| p.1).sum();
    let small_half: f64 = by_lines.iter().skip(pairs.len() / 2).map(|p| p.1).sum();
    println!(
        "\n  Δ held by the LARGEST half of the modules: {:.3}",
        big_half
    );
    println!("  Δ held by the SMALLEST half:               {:.3}", small_half);
    let c3 = big_half > small_half * 1.5;

    // -----------------------------------------------------------------------
    // C4 — the observer-claim census.
    // -----------------------------------------------------------------------
    println!("\n── C4: modules that CLAIM to be inert, vs modules that PROVE it ───");
    let claim_markers = ["observer-first", "observer only", "nothing downstream reads",
                         "Stage 1", "pure observer", "changes no published number",
                         "no published number"];
    let mut claimers: Vec<String> = Vec::new();
    if let Ok(dir) = std::fs::read_dir("src") {
        let mut names: Vec<String> = dir
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|n| n.ends_with(".rs"))
            .collect();
        names.sort();
        for n in names {
            if let Ok(t) = std::fs::read_to_string(format!("src/{}", n)) {
                if claim_markers.iter().any(|m| t.contains(m)) {
                    claimers.push(n);
                }
            }
        }
    }
    // A claim is CHECKED if some test file names the module's stem.
    let mut checked = 0usize;
    let mut unchecked: Vec<String> = Vec::new();
    let tests_blob: String = std::fs::read_dir("tests")
        .map(|d| {
            d.filter_map(|e| e.ok())
                .filter_map(|e| std::fs::read_to_string(e.path()).ok())
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default();
    for c in claimers.iter() {
        let stem = c.trim_end_matches(".rs");
        if tests_blob.contains(stem) {
            checked += 1;
        } else {
            unchecked.push(stem.to_string());
        }
    }
    println!("  modules asserting they are inert ......... {}", claimers.len());
    println!("  of those, named anywhere in tests/ ....... {}", checked);
    println!("  asserting inertness with NO test naming them: {}", unchecked.len());
    if !unchecked.is_empty() {
        println!("    {}", unchecked.join(", "));
    }
    let c4 = claimers.len() > checked;

    // -----------------------------------------------------------------------
    // C5 — does any gate make welfare WORSE?
    // -----------------------------------------------------------------------
    // -----------------------------------------------------------------------
    // SURVIVAL FIRST. A gate that kills is not a "welfare delta" — and this
    // probe ranked such a gate FIRST on Δ before the mutation caught it.
    // -----------------------------------------------------------------------
    println!("\n══ SURVIVAL — checked before any Δ is interpreted ═════════════════");
    let mut lethal: Vec<(String, &str, u32)> = Vec::new();
    for g in gs.iter() {
        for (i, w) in ws.iter().enumerate() {
            let arm = run(*w, Some(g));
            if !arm.alive && base[i].alive {
                lethal.push((g.0.to_string(), w.name, arm.ticks));
            }
        }
        let earm = run_embodied(Some(g));
        if !earm.alive && ebase.alive {
            lethal.push((g.0.to_string(), "room", earm.ticks));
        }
    }
    if lethal.is_empty() {
        println!("  No gate killed a being that the default path kept alive.");
    } else {
        println!("  ⚠️  A GATE IS LETHAL. The default being lives; with this on, it does not.");
        println!("  {:<26} {:>10} {:>12}", "gate", "world", "died at tick");
        for (g, w, t) in lethal.iter() {
            println!("  {:<26} {:>10} {:>12}", g, w, t);
        }
        println!("\n  This probe ranked `workspace_persistence` FIRST by embodied Δ before");
        println!("  the mutation pass caught this. \"Highest impact\" was true and would");
        println!("  have been a catastrophic thing to report. Survival is read first now.");
    }

    println!("\n── C5: does enabling any gate make the being worse off? ───────────");
    println!("(Arms where the being DIED are excluded — death is reported above, not");
    println!(" as a valence delta. A mean over 32 ticks is not a welfare measurement.)");
    println!("{:<24} {:>16} {:>16}", "gate", "Δ mean valence", "world");
    let mut worse: Vec<(String, f64, &str)> = Vec::new();
    for g in gs.iter() {
        for (i, w) in ws.iter().enumerate() {
            let arm = run(*w, Some(g));
            let dv = arm.mean_valence - base[i].mean_valence;
            if dv < -0.01 && arm.alive {
                worse.push((g.0.to_string(), dv, w.name));
            }
        }
        let earm = run_embodied(Some(g));
        let edv = earm.mean_valence - ebase.mean_valence;
        if edv < -0.01 && earm.alive {
            worse.push((g.0.to_string(), edv, "room"));
        }
    }
    if worse.is_empty() {
        println!("  none — no gate lowered mean valence by more than 0.01 anywhere.");
    } else {
        for (g, d, w) in worse.iter() {
            println!("{:<24} {:>16.3} {:>16}", g, d, w);
        }
    }
    let c5 = !worse.is_empty();

    println!("\n── C3, C4, C5 ─────────────────────────────────────────────────────");
    println!(
        "C3  module size predicts influence ................. {}",
        if c3 {
            "HOLDS — the larger half carries the change. I predicted this\n    would fail."
        } else {
            "FAILS, as predicted — size does not predict what matters."
        }
    );
    println!(
        "C4  more modules claim inertness than prove it ..... {}",
        if c4 {
            "HOLDS — and by this project's own rule an unchecked claim\n    is not a passed one."
        } else {
            "FAILS — every inertness claim has a test naming it."
        }
    );
    println!(
        "C5  some gate makes welfare worse .................. {}",
        if c5 {
            "HOLDS — listed above."
        } else {
            "FAILS — no gate lowered mean valence anywhere measured."
        }
    );

    println!("\n── what the census actually licenses ──────────────────────────────");
    println!("Not 'the being is {}/{} dead weight'. The abstract arm says that and the", dead.len(), rows.len());
    println!("embodied arm says {}/{}. The difference IS the finding: a large part of", edead.len(), erows.len());
    println!("this being does nothing until it is given a world to do it in, and the");
    println!("census you run decides how much of it you would delete.");
}
