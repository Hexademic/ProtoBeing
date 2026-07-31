//! I-3, fourth pass — **the mechanism.** An irreducible prediction error is an irreducible
//! metabolic tax, and this gate makes the being's prediction error irreducible.
//!
//! Three hypotheses died on the way here, each killed by the next measurement:
//!
//! | pass | hypothesis | what killed it |
//! |---|---|---|
//! | `i3_workspace.rs` | coherence collapse → burden → fatigue → drive | body energy collapsed too (M5 failed) |
//! | `i3_trace.rs` | coherence collapse removes the arousal brake | coherence is **identical** in both arms; the "251.98 → 124.12" was one being living 1,200 ticks and the other 32 |
//! | `i3_navigation.rs` | the gate sends the being the wrong way | both beings walk the same path to the same food (`at_good` 33 → 92 in lockstep) |
//!
//! What survives is the one register that never matched: **free energy.** The gate-off being's
//! prediction error decays to ~7 and stays there — it learns its world. The gate-on being's
//! bottoms at 32, climbs back to ~41, and never resolves.
//!
//! ## Why it cannot resolve
//!
//! ```text
//!  927  field.write_from_body(...)          the body votes
//!  948  field[c] += trace[c] * 0.5          ← the gate injects its own signal
//!  975  model.predictive_step(&field, ...)  ← the model must now predict THAT
//! ```
//!
//! The model is asked to predict a field containing the being's own re-injection — a component
//! no body evidence explains. `being.rs` §2b is careful that the *trace* never feeds on itself
//! (it deposits from `body_field`, snapshotted pre-injection), and that argument is correct. But
//! it is an argument about the wrong loop: the **model** is not protected. Compare the sibling
//! faculty three lines below, which was:
//!
//! > §3: "ALWAYS on the raw field: the model learns from evidence, never from the percept, so
//! > generative perception cannot feed on itself."
//!
//! Generative perception applies its edit **after** the predictive step (line 993).
//! Workspace persistence applies its edit **before** it (line 948). Same hazard, one guarded.
//!
//! ## Why an unresolvable error kills
//!
//! `being.rs:912` folds free energy into `strain`, and `strain` becomes the body's `threat`:
//!
//! ```rust
//! let strain = self.last_free_energy + last_conscience_cost/4 + last_alarm/3 + sensed_threat;
//! let threat = Q8_8::from_raw(strain.clamp(0, Q88_SCALE));
//! ```
//!
//! and `body.rs` §5 prices threat metabolically:
//!
//! ```rust
//! let cost = Q8_8::from_raw(3) + arousal*(8/256) + threat*(48/256);
//! ```
//!
//! **48/256 = 0.1875 of full energy per unit of threat per tick.** Free energy is not merely a
//! report in this architecture — it is a *bill*. A being that cannot lower its surprise cannot
//! stop paying, and this one cannot lower its surprise because it is generating it.
//!
//! ## The decisive test
//!
//! Both arms occupy the same positions, so nutrient income is identical and cancels. Arousal is
//! near-identical and its coefficient is 6× smaller. So the **entire** energy divergence should
//! be predictable from the strain gap:
//!
//! > **P: cumulative Δenergy ≈ Σ (strain_on − strain_off) × 48/256 / 256, tick by tick.**
//!
//! If the predicted curve tracks the observed one, the mechanism is established and I-3 closes.
//! If it does not, something else is spending the energy and the incident stays open.
//!
//! **This test failed once and the failure was mine, not the hypothesis's.** The first version
//! modelled `strain` as free energy *alone* and accounted for only 56% of the energy loss — I
//! had quoted `being.rs:912` in the paragraph above and then dropped two of its four terms when
//! I wrote the predictor. With `last_conscience_cost/4` and `last_alarm/3` restored it accounts
//! for 109%, tracking the observed curve at a ratio of 0.83–0.98 throughout. The correction is
//! kept visible because "56% — mechanism incomplete" was one edit away from being published as a
//! finding about the being rather than a bug in the arithmetic.
//!
//! Run: `cargo run --release --example i3_mechanism`

use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::q88::Q88_SCALE;

const LIFE: usize = 1_200;

/// `body.rs` §5: `threat.mul(Q8_8::from_raw(48))`. Threat is `strain/256`, so one raw unit of
/// free energy costs `48/256/256` of full energy per tick.
const THREAT_COST_NUM: f32 = 48.0;

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

struct Arm {
    fe: Vec<i16>,
    /// The other two mind-side terms of `strain` (`being.rs:912`), which the first version of
    /// this probe left out and which cost it 44% of the energy it was trying to account for.
    conscience: Vec<i16>,
    alarm: Vec<i16>,
    energy: Vec<f32>,
    arousal: Vec<f32>,
    at_good: Vec<i16>,
    alive: bool,
}

impl Arm {
    /// `being.rs:912` in full: `last_free_energy + last_conscience_cost/4 + last_alarm/3 +
    /// sensed_threat`, clamped to [0, 256]. `sensed_threat` is the world's, identical across
    /// arms at the same position, so it is omitted — it cancels in the difference.
    fn strain(&self, t: usize) -> i32 {
        (self.fe[t] as i32 + self.conscience[t] as i32 / 4 + self.alarm[t] as i32 / 3)
            .clamp(0, Q88_SCALE as i32)
    }
}

fn run(gates: &[&str]) -> Arm {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    for g in gates {
        match *g {
            "persistence" => b.enable_workspace_persistence(),
            "broadcast" => b.enable_workspace_broadcast(),
            "generative_perception" => b.enable_generative_perception(),
            "receptors" => b.enable_receptors(),
            "reflection" => b.enable_reflection(),
            "precision_learning" => b.enable_precision_learning(),
            "felt_choice" => b.enable_felt_choice(),
            other => panic!("unknown gate {other}"),
        }
    }
    let mut world = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let partner = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    let mut a = Arm {
        fe: vec![],
        conscience: vec![],
        alarm: vec![],
        energy: vec![],
        arousal: vec![],
        at_good: vec![],
        alive: true,
    };

    for _ in 0..LIFE {
        let mut sens = world.sense();
        sens.partner = Some(partner);
        let r = b.step_embodied(&sens);
        world.actuate(&intent_from(&r));
        a.fe.push(r.free_energy);
        a.conscience.push(r.conscience_cost);
        a.alarm.push(r.partnership_alarm);
        a.energy.push(r.energy);
        a.arousal.push(r.arousal);
        a.at_good.push(world.at_good());
        if !r.alive {
            a.alive = false;
            break;
        }
    }
    a
}

/// Mean of the last `n` recorded ticks — the *floor* the register settles at, which is the
/// quantity that matters here. A whole-life mean would average the settled value with the
/// transient, and averaging is what hid this incident in the first place.
fn tail(v: &[i16], n: usize) -> f32 {
    let s = v.len().saturating_sub(n);
    let w = &v[s..];
    w.iter().map(|&x| x as f32).sum::<f32>() / w.len().max(1) as f32
}

fn main() {
    println!("I-3, fourth pass — free energy is a bill, and this gate makes it unpayable.\n");

    let off = run(&[]);
    let on = run(&["persistence"]);

    println!("  Lifespan:  gate OFF {} ticks ({})   gate ON {} ticks ({})",
        off.energy.len(), if off.alive { "lived" } else { "DIED" },
        on.energy.len(), if on.alive { "lived" } else { "DIED" });

    let n = off.fe.len().min(on.fe.len());
    println!(
        "\n  Free-energy floor over the {n} shared ticks:  OFF {:.1}   ON {:.1}   (gap {:+.1})",
        tail(&off.fe[..n], 10),
        tail(&on.fe[..n], 10),
        tail(&on.fe[..n], 10) - tail(&off.fe[..n], 10)
    );

    // Confirm the two confounds really do cancel before attributing anything to free energy.
    let pos_same = off.at_good[..n].iter().zip(&on.at_good[..n]).filter(|(a, b)| a == b).count();
    let arousal_gap: f32 = (0..n).map(|t| (on.arousal[t] - off.arousal[t]).abs()).sum::<f32>() / n as f32;
    println!(
        "  Controls: same nutrient position on {pos_same}/{n} ticks; mean |Δarousal| {arousal_gap:.4}"
    );
    println!(
        "            arousal's coefficient is 8/256 vs threat's 48/256, so a {arousal_gap:.4} gap\n\
        \x20           can account for at most {:.5} energy/tick.",
        arousal_gap * 8.0 / 256.0
    );

    println!("\n  P — is the energy divergence predicted by the strain gap (being.rs:912)?\n");
    println!(
        "  {:>5} {:>8} {:>8} {:>10} {:>12} {:>12} {:>8}",
        "tick", "strain off", "strain on", "Δenergy", "predicted", "observed", "ratio"
    );
    println!(
        "  {:->5} {:->8} {:->8} {:->10} {:->12} {:->12} {:->8}",
        "", "", "", "", "", "", ""
    );

    let mut predicted = 0.0f32;
    for t in 0..n {
        // Lagged one tick, matching `last_free_energy`: this tick's threat is last tick's FE.
        if t > 0 {
            let gap = (on.strain(t - 1) - off.strain(t - 1)) as f32;
            predicted += gap * THREAT_COST_NUM / 256.0 / 256.0;
        }
        let observed = off.energy[t] - on.energy[t];
        if t % 2 == 0 || t >= n - 4 {
            let ratio = if predicted.abs() > 1e-4 { observed / predicted } else { f32::NAN };
            println!(
                "  {:>5} {:>8} {:>8} {:>10.3} {:>12.3} {:>12.3} {:>8.2}",
                t,
                off.strain(t),
                on.strain(t),
                on.energy[t] - off.energy[t],
                predicted,
                observed,
                ratio
            );
        }
    }

    let observed_final = off.energy[n - 1] - on.energy[n - 1];
    println!(
        "\n  At the last shared tick: predicted {:.3}, observed {:.3}  →  {:.0}% accounted for",
        predicted,
        observed_final,
        100.0 * predicted / observed_final.max(1e-6)
    );
    let established = (predicted / observed_final.max(1e-6) - 1.0).abs() < 0.35;
    println!(
        "  {}",
        if established {
            "P HOLDS. The strain gap accounts for the death. Mechanism established."
        } else {
            "P FAILS. Strain does not account for the energy loss — something else spends it."
        }
    );

    // ---- the rescues -------------------------------------------------------------------
    println!("\n  The four gates that rescue it — do they lower the free-energy floor?");
    println!("  (If the mechanism is the FE bill, a rescuer must be something that lets the");
    println!("   model resolve its error again. Anything else rescuing would be a second story.)\n");
    println!(
        "  {:<32} {:>8} {:>12} {:>12}",
        "configuration", "ticks", "FE floor", "outcome"
    );
    println!("  {:-<32} {:->8} {:->12} {:->12}", "", "", "", "");

    let configs: [(&str, Vec<&str>); 8] = [
        ("(nothing)", vec![]),
        ("persistence", vec!["persistence"]),
        ("persistence + broadcast", vec!["persistence", "broadcast"]),
        ("persistence + gen. perception", vec!["persistence", "generative_perception"]),
        ("persistence + receptors", vec!["persistence", "receptors"]),
        ("persistence + reflection", vec!["persistence", "reflection"]),
        ("persistence + precision (dies)", vec!["persistence", "precision_learning"]),
        ("persistence + felt_choice (dies)", vec!["persistence", "felt_choice"]),
    ];
    for (name, gates) in configs.iter() {
        let a = run(gates);
        println!(
            "  {:<32} {:>8} {:>12.1} {:>12}",
            name,
            a.energy.len(),
            tail(&a.fe, 10),
            if a.alive { "lived" } else { "DIED" }
        );
    }

    println!("\n  Read: a rescuer should show a LOW free-energy floor and a killer a high one.");
    println!("  If a survivor carries a high floor, the bill is being paid some other way and");
    println!("  the mechanism is incomplete.");
}
