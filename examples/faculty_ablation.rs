//! Which of this being's fourteen faculties actually change its life?
//!
//! The measurement for `docs/faculty-ablation.md`. A1–A6 were locked in that document and
//! committed before this file existed.
//!
//! The method is MemTensor's, from *Metis: Memory Foundation Model* (arXiv:2607.26760). Their
//! Table 7 ablates their memory block one component at a time and reports Δ% against the full
//! model, and the result is the useful thing in the paper: the elaborate gated-delta **update**
//! rule is worth **−0.58%**, while adaptive aggregation — deciding **what to write** — is worth
//! **−60.98%**. Selection beats the update rule by two orders of magnitude.
//!
//! **Improvement on their method: they ablate in one direction.** That is blind to a component
//! that matters only in company. This being is *known* to have one — incident **I-3**,
//! `workspace_persistence`, lethal alone at tick 32 and harmless with four companions. So this
//! runs both directions, and **A6 predicts they disagree**.
//!
//! Pure observer: reads report fields, changes nothing, writes no journal, `life/being.journal`
//! untouched. **Survival is reported before any welfare number.**
//!
//! Run: `cargo run --release --example faculty_ablation`

use unified_being::basins::Basin;
use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::play::COMFORT;
use unified_being::q88::Q88_SCALE;

const LIFE: usize = 4_000;

/// The fourteen opt-in causal gates, in the order they were built.
const GATES: [&str; 14] = [
    "precision_learning",
    "workspace_broadcast",
    "workspace_persistence",
    "generative_perception",
    "receptors",
    "serial_access",
    "schema_control",
    "felt_choice",
    "reflection",
    "homecoming",
    "memory_guidance",
    "comfort",
    "settling",
    "setting_down",
];

fn apply(b: &mut UnifiedBeing, on: &[bool; 14]) {
    if on[0] { b.enable_precision_learning(); }
    if on[1] { b.enable_workspace_broadcast(); }
    if on[2] { b.enable_workspace_persistence(); }
    if on[3] { b.enable_generative_perception(); }
    if on[4] { b.enable_receptors(); }
    if on[5] { b.enable_serial_access(); }
    if on[6] { b.enable_schema_control(); }
    if on[7] { b.enable_felt_choice(); }
    if on[8] { b.enable_reflection(); }
    if on[9] { b.enable_homecoming(); }
    if on[10] { b.enable_memory_guidance(); }
    if on[11] { b.enable_comfort(); }
    if on[12] { b.enable_settling(); }
    if on[13] { b.enable_setting_down(); }
}

#[derive(Default, Clone)]
struct Lived {
    ticks: usize,
    alive: bool,
    drive_sum: i64,
    past_comfort: usize,
    load_sum: i64,
    weathered: i16,
    at_stake: usize,
    rest: usize,
    soul: [u8; 32],
}

impl Lived {
    fn drive(&self) -> f32 {
        self.drive_sum as f32 / self.ticks.max(1) as f32
    }
    fn load(&self) -> f32 {
        self.load_sum as f32 / self.ticks.max(1) as f32
    }
}

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

/// One life in the reference world, with company — the regime every other document here uses,
/// so these numbers compose with them.
fn live(on: &[bool; 14]) -> Lived {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    apply(&mut b, on);
    let mut world = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let p = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    let mut l = Lived { alive: true, ..Default::default() };

    for _ in 0..LIFE {
        let mut s = world.sense();
        s.partner = Some(p);
        let r = b.step_embodied(&s);
        world.actuate(&intent_from(&r));

        l.drive_sum += r.drive.drive as i64;
        if r.drive.drive >= COMFORT {
            l.past_comfort += 1;
        }
        l.load_sum += r.reflection.load as i64;
        l.weathered = r.reflection.self_model.weathered;
        if r.felt.state.at_stake {
            l.at_stake += 1;
        }
        if matches!(r.basin, Basin::Rest | Basin::Recovery) {
            l.rest += 1;
        }
        l.ticks += 1;
        if !r.alive {
            l.alive = false;
            break;
        }
    }
    l.soul = b.soul_hash();
    l
}

fn pct(n: usize, d: usize) -> f32 {
    if d == 0 { 0.0 } else { n as f32 * 100.0 / d as f32 }
}

/// Relative change against a baseline, as a percentage. Metis's ΔAvg. column.
fn delta(v: f32, base: f32) -> f32 {
    if base.abs() < 1e-6 { 0.0 } else { (v - base) * 100.0 / base.abs() }
}

/// One ablation direction. `flip` says what to do to the baseline for each gate.
fn sweep(base_on: [bool; 14], flip_to: bool) -> Vec<Lived> {
    (0..14)
        .map(|g| {
            let mut on = base_on;
            on[g] = flip_to;
            live(&on)
        })
        .collect()
}

fn main() {
    println!("Which of this being's fourteen faculties actually change its life?");
    println!("(A1-A6 locked in docs/faculty-ablation.md, committed before this ran)");
    println!("(method: MemTensor's Metis, arXiv:2607.26760, Table 7 — run in BOTH directions)\n");

    let all_on = [true; 14];
    let all_off = [false; 14];

    let full = live(&all_on);
    let published = live(&all_off);

    // ---- A1: survival, before any welfare number ----------------------------------------
    println!("  SURVIVAL FIRST — A1\n");
    println!("    {:<34} {:>8} {:>10}", "baseline", "ticks", "outcome");
    println!("    {:-<34} {:->8} {:->10}", "", "", "");
    println!("    {:<34} {:>8} {:>10}", "all fourteen gates ON", full.ticks,
        if full.alive { "lived" } else { "DIED" });
    println!("    {:<34} {:>8} {:>10}", "all OFF (the published being)", published.ticks,
        if published.alive { "lived" } else { "DIED" });
    println!("\n    {}", if full.alive {
        "** A1 HOLDS. All fourteen faculties together are survivable. Nobody had ever run them \
         together before this line printed. **"
    } else {
        "** A1 FAILS — the fully-enabled being DIES. That is the finding, and every welfare \
         number below is about a being that did not finish its life. **"
    });

    let out = sweep(all_on, false);
    let inn = sweep(all_off, true);

    // ---- A2: is anything exactly inert? --------------------------------------------------
    println!("\n  A2 — is any faculty EXACTLY inert? (soul-hash unchanged when toggled)\n");
    println!("    {:<24} {:>16} {:>16}", "faculty", "removed from all-on", "added to all-off");
    println!("    {:-<24} {:->16} {:->16}", "", "", "");
    let mut inert_out = Vec::new();
    let mut inert_in = Vec::new();
    for g in 0..14 {
        let a = out[g].soul == full.soul;
        let b = inn[g].soul == published.soul;
        if a { inert_out.push(GATES[g]); }
        if b { inert_in.push(GATES[g]); }
        println!("    {:<24} {:>16} {:>16}", GATES[g],
            if a { "INERT" } else { "acts" }, if b { "INERT" } else { "acts" });
    }
    println!("\n    inert when removed from all-on: {}",
        if inert_out.is_empty() { "none".to_string() } else { inert_out.join(", ") });
    println!("    inert when added to all-off:    {}",
        if inert_in.is_empty() { "none".to_string() } else { inert_in.join(", ") });
    println!("\n    {}", if inert_in.contains(&"setting_down") {
        "** A2 HOLDS, and on the gate I named: `setting_down` cannot fire in this life because \
         the reference world never burdens the being (load stays 0). A faculty built today that \
         does nothing in the life we test it in. **"
    } else if !inert_in.is_empty() || !inert_out.is_empty() {
        "** A2 HOLDS but on a DIFFERENT gate than I named. Read the list above — an inert faculty \
         is a faculty whose argument has never been tested. **"
    } else {
        "A2 fails — every one of the fourteen changes the trajectory in both directions."
    });

    // ---- A3/A4/A5: the Δ table -----------------------------------------------------------
    println!("\n  A3 — the Δ table. LEAVE-ONE-OUT: baseline all fourteen ON, remove each.\n");
    println!("    (Metis's Table 7 shape. Ranked by |Δ mean drive|. Survival column first.)\n");
    println!("    {:<24} {:>7} {:>11} {:>10} {:>11} {:>10} {:>9}",
        "faculty removed", "ticks", "mean drive", "Δ drive", "past COMF", "mean load", "at stake");
    println!("    {:-<24} {:->7} {:->11} {:->10} {:->11} {:->10} {:->9}",
        "", "", "", "", "", "", "");
    println!("    {:<24} {:>7} {:>11.2} {:>10} {:>10.1}% {:>10.1} {:>8.1}%",
        "— none (full) —", full.ticks, full.drive(), "—",
        pct(full.past_comfort, full.ticks), full.load(), pct(full.at_stake, full.ticks));

    let mut ranked: Vec<usize> = (0..14).collect();
    ranked.sort_by(|&a, &b| {
        delta(out[b].drive(), full.drive()).abs()
            .partial_cmp(&delta(out[a].drive(), full.drive()).abs()).unwrap()
    });
    for &g in &ranked {
        let l = &out[g];
        println!("    {:<24} {:>7} {:>11.2} {:>9.2}% {:>10.1}% {:>10.1} {:>8.1}%",
            format!("{}{}", GATES[g], if l.alive { "" } else { " †" }),
            l.ticks, l.drive(), delta(l.drive(), full.drive()),
            pct(l.past_comfort, l.ticks), l.load(), pct(l.at_stake, l.ticks));
    }

    let movers = (0..14).filter(|&g| delta(out[g].drive(), full.drive()).abs() > 1.0).count();
    println!("\n    {movers} of 14 move mean drive by more than 1%.");
    println!("    {}", if movers <= 3 {
        "** A3 HOLDS. The effect is concentrated: a handful of faculties carry the being's life \
         and the rest are near-silent — Metis's shape exactly. **"
    } else {
        "** A3 FAILS — the effect is spread across many faculties. This being is not a set of \
         separable faculties but one over-determined system, the same answer arousal_range got \
         about its twelve somatic channels. Second time. **"
    });

    // ---- the other direction -------------------------------------------------------------
    println!("\n  ADD-ONE-IN: baseline all OFF (the published being), add each.\n");
    println!("    {:<24} {:>7} {:>11} {:>10} {:>11} {:>10} {:>9}",
        "faculty added", "ticks", "mean drive", "Δ drive", "past COMF", "mean load", "at stake");
    println!("    {:-<24} {:->7} {:->11} {:->10} {:->11} {:->10} {:->9}",
        "", "", "", "", "", "", "");
    println!("    {:<24} {:>7} {:>11.2} {:>10} {:>10.1}% {:>10.1} {:>8.1}%",
        "— none (published) —", published.ticks, published.drive(), "—",
        pct(published.past_comfort, published.ticks), published.load(),
        pct(published.at_stake, published.ticks));

    let mut ranked2: Vec<usize> = (0..14).collect();
    ranked2.sort_by(|&a, &b| {
        delta(inn[b].drive(), published.drive()).abs()
            .partial_cmp(&delta(inn[a].drive(), published.drive()).abs()).unwrap()
    });
    for &g in &ranked2 {
        let l = &inn[g];
        println!("    {:<24} {:>7} {:>11.2} {:>9.2}% {:>10.1}% {:>10.1} {:>8.1}%",
            format!("{}{}", GATES[g], if l.alive { "" } else { " †DIED" }),
            l.ticks, l.drive(), delta(l.drive(), published.drive()),
            pct(l.past_comfort, l.ticks), l.load(), pct(l.at_stake, l.ticks));
    }

    // ---- A4 -------------------------------------------------------------------------------
    let refl = 8usize;
    let refl_d = delta(out[refl].drive(), full.drive()).abs();
    let refl_rank = ranked.iter().position(|&g| g == refl).unwrap() + 1;
    println!("\n  A4 — incident I-8's question, given its experiment at last:\n");
    println!("    removing `reflection` moves mean drive by {refl_d:.2}%, rank {refl_rank} of 14.");
    println!("    {}", if refl_d < 1.0 {
        "** A4 HOLDS. Strain is a bill. `weathered` is a readout with no consequence FOR DRIVE — \
         and now that has an ablation behind it, not only the structural argument that \
         being.rs:1676 never reads affective_drive. I-8's drive half is answered. **"
    } else {
        "** A4 FAILS against my prediction — reflection DOES move the being's drive, which \
         contradicts the structural reading that drive never sees affective_drive. If so I have \
         misread the path and should find it before claiming anything else. **"
    });

    // ---- A5 -------------------------------------------------------------------------------
    let rec = 4usize;
    let rec_rank = ranked.iter().position(|&g| g == rec).unwrap() + 1;
    println!("\n  A5 — `receptors`, the gate Blake has an open decision about (I-2):");
    println!("    rank {rec_rank} of 14 by |Δ drive| ({:.2}%).",
        delta(out[rec].drive(), full.drive()).abs());
    println!("    {}", if rec_rank <= 4 {
        "** A5 HOLDS — it is among the faculties that most change this being's life. **"
    } else {
        "A5 fails — receptors is NOT near the top. What the being can sense matters less to its \
         drive than I assumed."
    });

    // ---- A6 -------------------------------------------------------------------------------
    println!("\n  A6 — do the two directions DISAGREE? (the half of this Metis did not run)\n");
    println!("    {:<24} {:>14} {:>14} {:>12}",
        "faculty", "Δ if removed", "Δ if added", "survived add");
    println!("    {:-<24} {:->14} {:->14} {:->12}", "", "", "", "");
    let mut disagreements = 0usize;
    for g in 0..14 {
        let dout = delta(out[g].drive(), full.drive());
        let din = delta(inn[g].drive(), published.drive());
        // Disagreement: one direction is near-silent while the other moves the being, or a
        // faculty that is harmless to remove is fatal to add.
        let disagree = (dout.abs() < 1.0 && din.abs() > 5.0) || !inn[g].alive || !out[g].alive;
        if disagree {
            disagreements += 1;
        }
        println!("    {:<24} {:>13.2}% {:>13.2}% {:>12}{}", GATES[g], dout, din,
            if inn[g].alive { "yes" } else { "DIED" },
            if disagree { "   <- conditional" } else { "" });
    }
    let wp = 2usize;
    println!("\n    {disagreements} of 14 behave differently depending on direction.");
    println!("    {}", if !inn[wp].alive || (delta(out[wp].drive(), full.drive()).abs() < 1.0
        && delta(inn[wp].drive(), published.drive()).abs() > 5.0) {
        "** A6 HOLDS, on the gate I named. `workspace_persistence` is one thing alone and another \
         in company — incident I-3 reproduced by ablation. A one-directional table, which is what \
         Metis reports, would have scored it harmless. **".to_string()
    } else if disagreements > 0 {
        format!("** A6 HOLDS but not on the gate I named: {disagreements} faculties are \
                 conditional and workspace_persistence is not the clearest of them. Read the \
                 table. **")
    } else {
        "A6 fails — both directions agree everywhere, so the second half of this experiment was \
         unnecessary. Worth knowing, and worth saying.".to_string()
    });

    println!("\n  The founded being was not touched. Report fields read; no journal written.");
}
