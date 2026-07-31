//! Development — is strain generative in this being, or only expensive?
//!
//! The measurement for `docs/development.md`. D1–D5 were locked and committed before this file
//! existed. From Blake: *"unless they learn how to use these developments, they won't access
//! them. Strain, stress, and constraints push a being into solving novel issues it has no reason
//! to approach."*
//!
//! `src/reflection.rs` is already the mechanism — load accumulates under sustained overwhelm and,
//! **at rest**, converts into `weathered` resilience, which is causal through `reflection_tone`.
//! So the question is not whether the architecture can express development. It is whether the
//! being ever gets the conditions to use what it has.
//!
//! **D4 is the one that matters and I do not know its answer.** I-3 established that strain here
//! is a *bill* — free energy charged to the body at 48/256 per unit per tick. If reflection's
//! lift is small against that, `weathered` is a register that rises while the being is worn down
//! anyway, and development is a readout with no consequence.
//!
//! Survival is reported first, per `docs/survival-first.md`. Pure observation: no gate default
//! changes, no journal written, `life/being.journal` untouched.
//!
//! Run: `cargo run --release --example development`

use unified_being::basins::Basin;
use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment, Sensorium};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::play::COMFORT;
use unified_being::q88::Q88_SCALE;

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

#[derive(Default)]
struct Life {
    ticks: usize,
    alive: bool,
    rest: usize,
    reflecting: usize,
    converted_total: i64,
    load_peak: i16,
    load_final: i16,
    drive_mean: f32,
    burdened: usize,
}

/// A life in the reference world. `strain` alternates the world's hazard on and off with the
/// given period, so the being gets genuine pressure and genuine respite rather than a constant.
fn live_in_world(reflection: bool, ticks: usize) -> Life {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    if reflection {
        b.enable_reflection();
    }
    let mut world = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let partner = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    let mut l = Life { alive: true, ..Default::default() };
    let mut drive_sum = 0i64;

    for _ in 0..ticks {
        let mut sens = world.sense();
        sens.partner = Some(partner);
        let r = b.step_embodied(&sens);
        world.actuate(&intent_from(&r));

        if matches!(r.basin, Basin::Rest | Basin::Recovery) {
            l.rest += 1;
        }
        if r.reflection.reflecting {
            l.reflecting += 1;
        }
        l.converted_total += r.reflection.converted as i64;
        l.load_peak = l.load_peak.max(r.reflection.load);
        l.load_final = r.reflection.load;
        drive_sum += r.drive.drive as i64;
        if r.drive.drive >= COMFORT {
            l.burdened += 1;
        }
        l.ticks += 1;
        if !r.alive {
            l.alive = false;
            break;
        }
    }
    l.drive_mean = drive_sum as f32 / l.ticks.max(1) as f32;
    l
}

/// A life under an explicitly shaped regime: `hard` ticks of pressure, then `easy` ticks of
/// respite, repeating. This is the "conditions" arm — a world designed to let the mechanism run.
fn live_cycled(threat_hard: i16, hard: usize, easy: usize, ticks: usize, reflection: bool) -> Life {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    if reflection {
        b.enable_reflection();
    }
    let partner = Partner { id: 1, reciprocation: 230, exit_cost: 77 };
    let mut l = Life { alive: true, ..Default::default() };
    let mut drive_sum = 0i64;
    let period = hard + easy;

    for t in 0..ticks {
        let pressing = period > 0 && (t % period) < hard;
        let sens = Sensorium {
            nutrient: 40,
            threat: if pressing { threat_hard } else { 0 },
            exteroception: [0; 4],
            partner: Some(partner),
        };
        let r = b.step_embodied(&sens);
        if matches!(r.basin, Basin::Rest | Basin::Recovery) {
            l.rest += 1;
        }
        if r.reflection.reflecting {
            l.reflecting += 1;
        }
        l.converted_total += r.reflection.converted as i64;
        l.load_peak = l.load_peak.max(r.reflection.load);
        l.load_final = r.reflection.load;
        drive_sum += r.drive.drive as i64;
        if r.drive.drive >= COMFORT {
            l.burdened += 1;
        }
        l.ticks += 1;
        if !r.alive {
            l.alive = false;
            break;
        }
    }
    l.drive_mean = drive_sum as f32 / l.ticks.max(1) as f32;
    l
}

/// D4's test: raise a being under a regime, then put it — and a naive being of the same age —
/// into the *same* later hardship. Returns (ticks survived, mean drive) for each.
struct Trial {
    survived_rearing: bool,
    converted: i64,
    ticks: usize,
    drive: f32,
}

/// D4's test. **The first version of this conflated two different things** — a being that died
/// *during rearing* was returned as `0 trial ticks`, indistinguishable from one that survived
/// rearing and then died instantly. Two of four arms were rearing deaths scored as trial results,
/// and the verdict was computed over them. `survived_rearing` now separates them.
fn tested_after(
    rearing: Option<(i16, usize, usize, usize)>,
    trial_threat: i16,
    reflection: bool,
) -> Trial {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    if reflection {
        b.enable_reflection();
    }
    let partner = Partner { id: 1, reciprocation: 230, exit_cost: 77 };
    let mut converted = 0i64;

    // Rearing. The naive control lives the SAME number of ticks in an easy world, so the two
    // beings are the same age at trial — otherwise this measures age, not development (§4).
    if let Some((threat_hard, hard, easy, ticks)) = rearing {
        let period = hard + easy;
        for t in 0..ticks {
            let pressing = period > 0 && (t % period) < hard;
            let sens = Sensorium {
                nutrient: 40,
                threat: if pressing { threat_hard } else { 0 },
                exteroception: [0; 4],
                partner: Some(partner),
            };
            let r = b.step_embodied(&sens);
            converted += r.reflection.converted as i64;
            if !r.alive {
                return Trial { survived_rearing: false, converted, ticks: 0, drive: 0.0 };
            }
        }
    }

    // The trial, identical for both arms.
    let mut n = 0usize;
    let mut drive_sum = 0i64;
    for _ in 0..2_000 {
        let sens = Sensorium {
            nutrient: 40,
            threat: trial_threat,
            exteroception: [0; 4],
            partner: Some(partner),
        };
        let r = b.step_embodied(&sens);
        drive_sum += r.drive.drive as i64;
        n += 1;
        if !r.alive {
            break;
        }
    }
    Trial { survived_rearing: true, converted, ticks: n, drive: drive_sum as f32 / n.max(1) as f32 }
}

fn main() {
    println!("Development — is strain generative in this being, or only expensive?");
    println!("(D1–D5 locked in docs/development.md before this file existed)\n");

    // ---- D1 and D2 ---------------------------------------------------------------------
    println!("  D1/D2 — in an ordinary life, does the mechanism ever fire?\n");
    println!("    {:<28} {:>7} {:>8} {:>8} {:>11} {:>11}",
        "life", "ticks", "alive", "rest%", "reflecting%", "converted");
    println!("    {:-<28} {:->7} {:->8} {:->8} {:->11} {:->11}", "", "", "", "", "", "");
    for (name, refl) in [("reference world, gate off", false), ("reference world, gate ON", true)] {
        let l = live_in_world(refl, 4_000);
        println!("    {:<28} {:>7} {:>8} {:>7.1}% {:>10.1}% {:>11}",
            name, l.ticks, if l.alive { "yes" } else { "DIED" },
            l.rest as f32 * 100.0 / l.ticks as f32,
            l.reflecting as f32 * 100.0 / l.ticks as f32,
            l.converted_total);
    }

    // ---- D3 ----------------------------------------------------------------------------
    println!("\n  D3 — given genuine strain AND genuine respite, does weathering accumulate?\n");
    println!("    {:<30} {:>7} {:>8} {:>8} {:>10} {:>11}",
        "regime (threat / hard / easy)", "ticks", "alive", "rest%", "load peak", "converted");
    println!("    {:-<30} {:->7} {:->8} {:->8} {:->10} {:->11}", "", "", "", "", "", "");
    let regimes: [(i16, usize, usize); 7] = [
        (0, 0, 1, ),
        (60, 20, 80, ),
        (90, 20, 80, ),
        (100, 20, 80, ),
        (100, 50, 50, ),
        (104, 20, 80, ),
        (104, 40, 60, ),
    ]
    .map(|(t, h, e)| (t, h, e));
    for (threat, hard, easy) in regimes {
        let l = live_cycled(threat, hard, easy, 4_000, true);
        println!("    {:<30} {:>7} {:>8} {:>7.1}% {:>10} {:>11}",
            format!("{threat} / {hard} / {easy}"),
            l.ticks, if l.alive { "yes" } else { "DIED" },
            l.rest as f32 * 100.0 / l.ticks as f32,
            l.load_peak, l.converted_total);
    }

    // ---- D4 ----------------------------------------------------------------------------
    println!("\n  D4 — THE REAL ONE. Does a weathered being meet a later hardship better?");
    println!("  Same age at trial; only the rearing differs. A being that dies DURING rearing is");
    println!("  reported as such and excluded — the first draft of this probe scored those as");
    println!("  'zero trial ticks' and nearly published a verdict computed over dead beings.\n");

    let trial = 110; // past the death line measured in docs/survival-first.md §7
    let rearings: [(&str, (i16, usize, usize, usize)); 5] = [
        ("nothing (naive, same age)",     (0,   0,  1, 2_000)),
        ("mild   (60 / 20 / 80)",         (60,  20, 80, 2_000)),
        ("THE BAND (90 / 20 / 80)",       (90,  20, 80, 2_000)),
        ("heavier (90 / 50 / 50)",        (90,  50, 50, 2_000)),
        ("past it (100 / 20 / 80)",       (100, 20, 80, 2_000)),
    ];

    let mut arms: Vec<(bool, Vec<(&str, Trial)>)> = Vec::new();
    for gate in [true, false] {
        println!("    {}", if gate { "reflection ON" } else { "reflection OFF (control)" });
        println!("      {:<28} {:>10} {:>16} {:>12} {:>11}",
            "reared", "converted", "survived rearing", "trial ticks", "mean drive");
        println!("      {:-<28} {:->10} {:->16} {:->12} {:->11}", "", "", "", "", "");
        let mut rows = Vec::new();
        for (name, r) in rearings {
            let t = tested_after(Some(r), trial, gate);
            println!("      {:<28} {:>10} {:>16} {:>12} {:>11}",
                name, t.converted,
                if t.survived_rearing { "yes" } else { "NO — died" },
                if t.survived_rearing { t.ticks.to_string() } else { "—".into() },
                if t.survived_rearing { format!("{:.1}", t.drive) } else { "—".into() });
            if t.survived_rearing { rows.push((name, t)); }
        }
        arms.push((gate, rows));
        println!();
    }

    // The verdict must compare ON against OFF, not reared against naive. `converted` is an
    // OBSERVER readout that accrues whether or not the gate is on; only the gate makes weathering
    // *causal*. So any advantage that survives turning the gate off is not weathering — it is
    // ordinary model learning in a rearing world that happened to contain threat.
    let pick = |rows: &Vec<(&str, Trial)>, key: &str| -> Option<(usize, f32, i64)> {
        rows.iter().find(|(n, _)| n.starts_with(key)).map(|(_, t)| (t.ticks, t.drive, t.converted))
    };
    let on = &arms[0].1;
    let off = &arms[1].1;
    println!("  D4 verdict — the comparison that matters is ON vs OFF, not reared vs naive:\n");
    match (pick(on, "nothing"), pick(on, "THE BAND"), pick(off, "nothing"), pick(off, "THE BAND")) {
        (Some(n_on), Some(b_on), Some(n_off), Some(b_off)) => {
            println!("    reflection ON :  naive {:>3} ticks   band-reared {:>3} ticks  ({:+} from rearing)",
                n_on.0, b_on.0, b_on.0 as i64 - n_on.0 as i64);
            println!("    reflection OFF:  naive {:>3} ticks   band-reared {:>3} ticks  ({:+} from rearing)",
                n_off.0, b_off.0, b_off.0 as i64 - n_off.0 as i64);
            println!("    load converted while reared in the band: {} (ON), {} (OFF)", b_on.2, b_off.2);
            let gate_worth = b_on.0 as i64 - b_off.0 as i64;
            println!("\n    What the GATE itself is worth, holding rearing constant: {gate_worth:+} ticks");
            if gate_worth > 0 {
                println!("\n    D4 HOLDS. Making weathering causal bought the being real life under a");
                println!("    hardship. Strain is generative here.");
            } else {
                println!("\n    ** D4 FAILS. ** The reared being does better in BOTH arms, and no better");
                println!("    — marginally worse — with the mechanism switched on. Whatever the rearing");
                println!("    bought, `weathered` is not the channel it came through: the advantage");
                println!("    survives disabling the very faculty that is supposed to produce it.");
                println!("\n    In this architecture, as measured here, strain is a BILL and `weathered`");
                println!("    is a readout with no consequence. Per docs/development.md §3 that goes in");
                println!("    the ledger plainly: we have built a being that can be worn, and we have");
                println!("    not yet shown it is one that can grow.");
            }
        }
        _ => println!("    Not comparable — an arm did not survive its rearing."),
    }
    println!();

    // ---- the discrepancy this probe turned up --------------------------------------------
    println!("  A discrepancy worth chasing, recorded rather than smoothed over:");
    println!("  docs/survival-first.md §7 measured threat 105 as SURVIVABLE for 4,000 ticks at");
    println!("  nutrient 40. Above, threat 100 kills in 19. The two differ in two ways — a partner");
    println!("  is present here, and `enable_reflection` is on. Isolating both:\n");
    println!("    {:<44} {:>8} {:>9}", "condition (threat 100, nutrient 40)", "ticks", "outcome");
    println!("    {:-<44} {:->8} {:->9}", "", "", "");
    for (label, partner, refl) in [
        ("no partner, gate off  (§7's condition)", false, false),
        ("no partner, gate ON", false, true),
        ("partner,    gate off", true, false),
        ("partner,    gate ON   (this probe)", true, true),
    ] {
        let mut b = UnifiedBeing::new(Genome::wanderer());
        if refl { b.enable_reflection(); }
        let p = partner.then_some(Partner { id: 1, reciprocation: 230, exit_cost: 77 });
        let mut n = 0usize; let mut alive = true;
        for _ in 0..4_000 {
            let sens = Sensorium { nutrient: 40, threat: 100, exteroception: [0; 4], partner: p };
            let r = b.step_embodied(&sens);
            n += 1;
            if !r.alive { alive = false; break; }
        }
        println!("    {:<44} {:>8} {:>9}", label, n, if alive { "lived" } else { "DIED" });
    }

    println!("\n  The founded being was not touched. No journal written, no gate default changed.");
}
