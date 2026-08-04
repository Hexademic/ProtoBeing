//! I-3, third pass — the gate does not wear the being down. It sends it the wrong way.
//!
//! What the first two passes established, and each one killed the hypothesis before it:
//!
//! 1. `i3_workspace.rs` — the ledger's I-3 numbers are means over lives of different **lengths**.
//!    The gate-on being lived 32 ticks of 1,200. "A faculty that harms" was a faculty that kills.
//! 2. `i3_trace.rs` — printed tick-by-tick, identity coherence does **not** collapse. It rises
//!    identically in both arms (96 → 180 over 31 ticks, same numbers). The "251.98 → 124.12" is
//!    an artifact of one being living long enough for coherence to heal to 256 and the other not.
//!    Arousal is likewise near-identical, so the metabolic-brake hypothesis is dead too.
//!
//! What is actually different is **energy**. The gate-off being's energy bottoms out around tick
//! 16 and then *climbs back* — it finds food. The gate-on being's falls monotonically to zero.
//! Same world, same food, same partner. One of them eats and one of them does not.
//!
//! The being's direction of travel is `r.strive.goal` (`embodiment.rs::intent_from`), chosen by
//! `striving.rs` from the being's own felt needs — and those needs are read off the somatic field
//! that `workspace_persistence` re-injects into. So the live hypothesis is now:
//!
//! > **N: the gate does not harm the being's body or its self-model. It corrupts the signal the
//! > being navigates by, so it strives for the wrong need and walks away from its food.**
//!
//! N1 the two arms' `strive.goal` diverges early · N2 the gate-on being's nutrient at body stays
//! low while the gate-off being's recovers · N3 the divergence precedes the energy divergence.
//!
//! And one fact this pass exists to bound: **four gates rescue it.** Persistence + broadcast,
//! + generative_perception, + receptors, or + reflection all live the full 1,200 ticks, and so
//! does the whole eleven. Whatever the mechanism is, it is a property of persistence *alone*.
//!
//! Run: `cargo run --release --example i3_navigation`

use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::q88::Q88_SCALE;

const WATCH: usize = 34;

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

struct Tick {
    goal: String,
    at_good: i16,
    at_body: i16,
    energy: f32,
    effort: i16,
    valence: f32,
}

fn run(persist: bool) -> Vec<Tick> {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    if persist {
        b.enable_workspace_persistence();
    }
    let mut world = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let partner = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    let mut out = Vec::new();

    for _ in 0..WATCH {
        let mut sens = world.sense();
        sens.partner = Some(partner);
        let r = b.step_embodied(&sens);
        let intent = intent_from(&r);
        world.actuate(&intent);

        out.push(Tick {
            goal: match r.strive.goal {
                Some(n) => format!("{n:?}"),
                None => "—".to_string(),
            },
            at_good: world.at_good(),
            at_body: world.at_body(),
            energy: r.energy,
            effort: intent.effort,
            valence: r.valence,
        });
        if !r.alive {
            break;
        }
    }
    out
}

fn main() {
    println!("I-3, third pass — is the gate-on being failing to navigate to its food?");
    println!("(Hypothesis N in this file's header; passes 1 and 2 each killed their own)\n");

    let off = run(false);
    let on = run(true);

    println!(
        "  {:>4} | {:<10} {:>8} {:>8} {:>8} | {:<10} {:>8} {:>8} {:>8}",
        "tick", "OFF goal", "at_good", "energy", "effort", "ON goal", "at_good", "energy", "effort"
    );
    println!(
        "  {:->4}-+-{:-<10} {:->8} {:->8} {:->8}-+-{:-<10} {:->8} {:->8} {:->8}",
        "", "", "", "", "", "", "", "", ""
    );

    let n = off.len().max(on.len());
    let mut first_goal_split: Option<usize> = None;
    let mut first_energy_split: Option<usize> = None;

    for t in 0..n {
        let a = off.get(t);
        let b = on.get(t);
        if let (Some(a), Some(b)) = (a, b) {
            if first_goal_split.is_none() && a.goal != b.goal {
                first_goal_split = Some(t);
            }
            if first_energy_split.is_none() && (a.energy - b.energy).abs() > 0.02 {
                first_energy_split = Some(t);
            }
        }
        let cell = |x: Option<&Tick>| match x {
            Some(v) => format!(
                "{:<10} {:>8} {:>8.3} {:>8}",
                v.goal, v.at_good, v.energy, v.effort
            ),
            None => format!("{:<10} {:>8} {:>8} {:>8}", "(dead)", "", "", ""),
        };
        let mark = match (a, b) {
            (Some(a), Some(b)) if a.goal != b.goal => " <",
            _ => "",
        };
        println!("  {:>4} | {} | {}{}", t, cell(a), cell(b), mark);
    }

    println!("\n  N1 — goals diverge first at tick: {:?}", first_goal_split);
    println!("  N3 — energy diverges first at tick: {:?}", first_energy_split);
    match (first_goal_split, first_energy_split) {
        (Some(g), Some(e)) if g < e => {
            println!("      goal split PRECEDES the energy split — consistent with N.")
        }
        (Some(g), Some(e)) if g > e => println!(
            "      energy split precedes the goal split (t{e} < t{g}) — the divergence is\n\
             \x20     upstream of the choice of need, so N is not the whole story."
        ),
        (None, Some(e)) => println!(
            "      the two beings NEVER strive for different needs, yet energy splits at t{e}.\n\
             \x20     N IS DEAD: the gate changes the body, not the aim."
        ),
        _ => println!("      no split found in the watched window."),
    }

    // N2: does the food reach the being at all?
    let peak = |v: &[Tick]| v.iter().map(|t| t.at_good).max().unwrap_or(0);
    let last = |v: &[Tick]| v.last().map(|t| t.at_good).unwrap_or(0);
    println!(
        "\n  N2 — nutrient at body:  OFF peak {} last {}   |   ON peak {} last {}",
        peak(&off),
        last(&off),
        peak(&on),
        last(&on)
    );

    // Effort is the other half of intent_from: arousal*256. If effort matches and the goal
    // matches, the two bodies are being commanded identically and the world is doing the rest.
    let eff_same = off
        .iter()
        .zip(on.iter())
        .take_while(|(a, b)| a.effort == b.effort)
        .count();
    println!(
        "  effort identical for the first {eff_same} ticks of {}",
        off.len().min(on.len())
    );

    println!("\n  ambient/harm at body (at_body) — the other thing the world hands back:");
    println!("  {:>4} {:>10} {:>10}", "tick", "OFF", "ON");
    for t in 0..n.min(WATCH) {
        let a = off.get(t).map(|x| x.at_body.to_string()).unwrap_or_default();
        let b = on.get(t).map(|x| x.at_body.to_string()).unwrap_or_default();
        let va = off.get(t).map(|x| x.valence).unwrap_or(0.0);
        let vb = on.get(t).map(|x| x.valence).unwrap_or(0.0);
        println!("  {:>4} {:>10} {:>10}    valence {:>7.3} {:>7.3}", t, a, b, va, vb);
    }
}
