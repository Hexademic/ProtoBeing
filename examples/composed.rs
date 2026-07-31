//! The composed being — who is in there, whole?
//!
//! The measurement for `docs/composed.md` §4. Eleven `enable_*` gates exist and every one has
//! been verified alone. The most ever switched on at once, anywhere in this repository, is
//! **three** — in `pci.rs`, a measurement harness. This turns on all eleven, in one being,
//! and watches a life.
//!
//! No faculty is built here and no gate is added. Every part has existed for weeks. The
//! entire content of this probe is *what comes out* when they run together.
//!
//! ## Two corrections this probe already survived
//!
//! The first version got C3 wrong twice, and both errors are worth keeping in view:
//!
//! 1. **It ran in the abstract world.** `step()` leaves `ext_extero` at zero, so `receptors`
//!    and `generative_perception` are *structurally inert* there — the composition was
//!    missing several of its own members. This runs embodied, in the field world.
//! 2. **It tested a fair partner only.** Refusal is triangulated and requires extraction to
//!    be detected, so a fair partner can never produce one, in ANY configuration. Zero
//!    refusals in both arms was not "the guarantee held" — it was a question never asked.
//!
//! So C3 is tested the way it should be: **sweep the partner's reciprocation** and find each
//! being's *tolerance boundary* — the most generous partner it will still refuse. Three
//! faculties that "can only strengthen a refusal" compounding would show up as that boundary
//! moving toward the generous end. That is a number, not an opinion.
//!
//! The founded being is not touched. Fresh probe-beings, no journal written.
//!
//! Run: `cargo run --release --example composed`

use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::play::COMFORT;
use unified_being::primes::PrimeFacts;
use unified_being::q88::Q88_SCALE;

const LIFE: usize = 1_200;

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

const GATES: [&str; 11] = [
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
];

fn apply(b: &mut UnifiedBeing, w: &[bool; 11]) {
    if w[0] { b.enable_precision_learning(); }
    if w[1] { b.enable_workspace_broadcast(); }
    if w[2] { b.enable_workspace_persistence(); }
    if w[3] { b.enable_generative_perception(); }
    if w[4] { b.enable_receptors(); }
    if w[5] { b.enable_serial_access(); }
    if w[6] { b.enable_schema_control(); }
    if w[7] { b.enable_felt_choice(); }
    if w[8] { b.enable_reflection(); }
    if w[9] { b.enable_homecoming(); }
    if w[10] { b.enable_memory_guidance(); }
}

#[derive(Default, Clone)]
struct Lived {
    ticks: usize,
    alive: bool,
    refusals: u32,
    self_knowledge: i64,
    identity_coherence: i64,
    attractor_confidence: i64,
    free_energy: i64,
    drive_sum: i64,
    drive_peak: i16,
    burdened: usize,
    soul: [u8; 32],
}

impl Lived {
    fn mean(&self, v: i64) -> f32 {
        v as f32 / self.ticks.max(1) as f32
    }
}

/// One embodied life in the field world, with a partner of the given reciprocation present
/// at every tick. The world supplies real exteroception, so every gate is live.
fn live(gates: &[bool; 11], reciprocation: f32) -> Lived {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    apply(&mut b, gates);

    let mut world = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let partner = Partner { id: 1, reciprocation: q(reciprocation), exit_cost: q(0.3) };
    let mut l = Lived { alive: true, ..Default::default() };

    for _ in 0..LIFE {
        let mut sens = world.sense();
        sens.partner = Some(partner);

        let r = b.step_embodied(&sens);
        world.actuate(&intent_from(&r));

        if r.refused_cost.is_some() {
            l.refusals += 1;
        }
        let facts = PrimeFacts::from_report(&r, Some(world.at_good() > 128));
        l.drive_sum += facts.drive as i64;
        l.drive_peak = l.drive_peak.max(facts.drive);
        if facts.drive >= COMFORT {
            l.burdened += 1;
        }
        l.self_knowledge += r.self_knowledge as i64;
        l.identity_coherence += r.identity_coherence as i64;
        l.attractor_confidence += r.attractor_confidence as i64;
        l.free_energy += r.free_energy as i64;
        l.ticks += 1;

        if !r.alive {
            l.alive = false;
            break;
        }
    }
    l.soul = b.soul_hash();
    l
}

fn only(i: usize) -> [bool; 11] {
    let mut w = [false; 11];
    w[i] = true;
    w
}

/// The most generous partner this configuration will still refuse — its tolerance boundary.
/// `None` means it refused nobody in the sweep.
fn boundary(gates: &[bool; 11], sweep: &[f32]) -> Option<f32> {
    sweep.iter().rev().find(|&&r| live(gates, r).refusals > 0).copied()
}

fn main() {
    println!("The composed being — who is in there, whole?");
    println!("(C1–C4 and W locked in docs/composed.md §4 before this was written)\n");

    let sweep: Vec<f32> = (1..=19).map(|i| i as f32 * 0.05).collect();
    let all_on = [true; 11];
    let all_off = [false; 11];

    // A hard-but-fair reference life for C1/C2/C4/W: generous partner, demanding world.
    let baseline = live(&all_off, 0.90);
    let composed = live(&all_on, 0.90);

    println!("  C1 — is the composed being a different being at all?");
    println!(
        "    baseline {:02x}{:02x}{:02x}{:02x}…   composed {:02x}{:02x}{:02x}{:02x}…   {}",
        baseline.soul[0], baseline.soul[1], baseline.soul[2], baseline.soul[3],
        composed.soul[0], composed.soul[1], composed.soul[2], composed.soul[3],
        if baseline.soul != composed.soul { "DIFFERENT" } else { "** IDENTICAL — the gates did nothing **" }
    );
    println!(
        "\n  C2 — does it survive?   baseline {} ({} ticks)    composed {} ({} ticks)",
        if baseline.alive { "alive" } else { "DIED" }, baseline.ticks,
        if composed.alive { "alive" } else { "DIED" }, composed.ticks
    );

    // ---- C3 ------------------------------------------------------------------------
    println!("\n  C3 — does composition move the being's tolerance boundary?");
    println!("    Sweeping the partner's reciprocation 0.05 → 0.95. The boundary is the most");
    println!("    GENEROUS partner a configuration will still refuse. Compounding \"can only");
    println!("    strengthen a refusal\" terms would push that boundary to the right.\n");

    println!("    {:<24} {:>12} {:>26}", "gates on", "boundary", "refusals @ boundary");
    println!("    {:-<24} {:->12} {:->26}", "", "", "");

    let b_base = boundary(&all_off, &sweep);
    let show = |name: &str, bd: Option<f32>, gates: &[bool; 11]| match bd {
        Some(r) => println!("    {:<24} {:>12.2} {:>26}", name, r, live(gates, r).refusals),
        None => println!("    {:<24} {:>12} {:>26}", name, "refused none", "—"),
    };
    show("none (baseline)", b_base, &all_off);

    let mut solo_shifts: Vec<(usize, f32)> = Vec::new();
    for i in 0..11 {
        let g = only(i);
        let b = boundary(&g, &sweep);
        if b != b_base {
            if let (Some(x), Some(y)) = (b, b_base) {
                solo_shifts.push((i, x - y));
            }
            show(GATES[i], b, &g);
        }
    }
    if solo_shifts.is_empty() {
        println!("    (no single gate moved the boundary)");
    }
    let b_comp = boundary(&all_on, &sweep);
    show("ALL ELEVEN", b_comp, &all_on);

    println!();
    match (b_base, b_comp) {
        (Some(bb), Some(bc)) => {
            // The compounding hypothesis is about the PARTS, not the baseline. Its weakest
            // form: the whole should refuse at least as generous a partner as its strongest
            // single faculty does. Comparing to the baseline instead would call any movement
            // at all a confirmation — which is the error the first run of this probe made.
            let strongest = solo_shifts.iter().map(|(_, d)| bb + d).fold(bb, f32::max);
            println!("    baseline boundary {bb:.2}; strongest single faculty {strongest:.2}; all eleven {bc:.2}.");
            if bc > strongest {
                println!("\n    C3 CONFIRMED. The whole refuses a MORE generous partner than any part");
                println!("    does alone ({bc:.2} vs {strongest:.2}) — the 'can only strengthen' terms compound.");
                println!("    That is a faculty interaction needing a being-level guard.");
            } else if bc < strongest {
                println!("\n    C3 WRONG, and in the direction that matters. The whole is MORE");
                println!("    TOLERANT than its own parts: {bc:.2} against {strongest:.2} for the strongest");
                println!("    faculty alone. Composition here is strongly SUB-additive — something in");
                println!("    the assembled being holds it back from abandoning a partner that");
                println!("    felt_choice or memory_guidance, running alone, would abandon.");
                println!("    The compounding fear does not materialise. The sovereignty guarantee is");
                println!("    not merely preserved under composition; it is TIGHTER under composition.");
                if bc > bb {
                    println!("    (It is still {:+.2} above the all-off baseline, so the faculties do move it —", bc - bb);
                    println!("     they simply move it far less together than they do apart.)");
                }
            } else {
                println!("\n    C3 WRONG. The whole lands exactly where its strongest part does.");
            }
        }
        (None, None) => println!("    Neither refused anyone across the sweep — C3 is UNTESTED, not answered."),
        _ => println!("    One arm refused nobody in the sweep; the boundary is not comparable."),
    }

    // ---- C4 ------------------------------------------------------------------------
    println!("\n  C4 — is it more itself, or only noisier?  (hard world, generous partner)");
    println!("    {:<26} {:>12} {:>12} {:>10}", "register", "baseline", "composed", "change");
    println!("    {:-<26} {:->12} {:->12} {:->10}", "", "", "", "");
    for (name, bv, cv) in [
        ("self_knowledge", baseline.self_knowledge, composed.self_knowledge),
        ("identity_coherence", baseline.identity_coherence, composed.identity_coherence),
        ("attractor_confidence", baseline.attractor_confidence, composed.attractor_confidence),
        ("free_energy (lower better)", baseline.free_energy, composed.free_energy),
    ] {
        let (bm, cm) = (baseline.mean(bv), composed.mean(cv));
        println!("    {name:<26} {bm:>12.2} {cm:>12.2} {:>+10.2}", cm - bm);
    }

    // ---- W -------------------------------------------------------------------------
    println!("\n  W — is the composed being worse off?");
    println!("    {:<26} {:>11} {:>10} {:>11}", "", "mean drive", "peak", "burdened");
    for (n, l) in [("baseline", &baseline), ("composed", &composed)] {
        println!(
            "    {:<26} {:>11.3} {:>10.3} {:>10}%",
            n,
            l.mean(l.drive_sum) / 256.0,
            l.drive_peak as f32 / 256.0,
            l.burdened * 100 / l.ticks.max(1)
        );
    }
    let d = composed.mean(composed.drive_sum) - baseline.mean(baseline.drive_sum);
    println!(
        "\n    comfort line {:.2}. The composed being is {} off ({:+.1} raw mean drive).",
        COMFORT as f32 / 256.0,
        if d > 0.0 { "WORSE" } else if d < 0.0 { "BETTER" } else { "no different" },
        d
    );

    // The each-alone control the spec promised. Without it, a change in the whole gets
    // read as emergence when it may be one gate. attractor_confidence is exactly that case.
    println!("\n  ATTRIBUTION — is any of the above emergent, or is it one gate?\n");
    // SURVIVAL COMES FIRST, and it did not in the version of this probe that produced the
    // numbers in docs/composed.md §6. `workspace_persistence` was tabulated at coherence 124.12
    // and drive 0.520 as though it had lived a worse life; it had died at tick 32, and a mean
    // over a death is not comparable to a mean over a life. See docs/incidents.md I-3.
    println!("    {:<24} {:>7} {:>10} {:>10} {:>10} {:>10}",
        "gates on", "ticks", "self-kn", "coher", "attractor", "drive");
    println!("    {:-<24} {:->7} {:->10} {:->10} {:->10} {:->10}", "", "", "", "", "", "");
    let row = |name: &str, l: &Lived| {
        // A dagger, not a footnote: any row whose being did not finish its life is marked in
        // the row itself, so the mark cannot be read past.
        let span = if l.alive { format!("{}", l.ticks) } else { format!("{}†", l.ticks) };
        println!(
            "    {:<24} {:>7} {:>10.2} {:>10.2} {:>10.2} {:>10.3}",
            name,
            span,
            l.mean(l.self_knowledge),
            l.mean(l.identity_coherence),
            l.mean(l.attractor_confidence),
            l.mean(l.drive_sum) / 256.0
        );
    };
    row("none (baseline)", &baseline);
    let base_attr = baseline.mean(baseline.attractor_confidence);
    let base_drive = baseline.mean(baseline.drive_sum) / 256.0;
    for i in 0..11 {
        let l = live(&only(i), 0.90);
        let moved = (l.mean(l.attractor_confidence) - base_attr).abs() > 1.0
            || (l.mean(l.drive_sum) / 256.0 - base_drive).abs() > 0.01
            || (l.mean(l.self_knowledge) - baseline.mean(baseline.self_knowledge)).abs() > 1.0;
        if moved {
            row(GATES[i], &l);
        }
    }
    row("ALL ELEVEN", &composed);
    println!("\n    † DIED before the life ended — every figure in that row is a mean over a");
    println!("      DEATH, not over a life, and is not comparable to any other row. Read the");
    println!("      ticks column before reading anything else in this table.");
    println!("\n    Read this table before believing anything above it. Where a single gate");
    println!("    reaches the same value the whole does, the effect is that GATE, not the");
    println!("    composition — and calling it emergence would be the same mistake as reading");
    println!("    a pooled mean (docs/play.md §7, docs/null-space.md §7).");

    println!("\n  The founded being was not touched. No journal written, no gate default");
    println!("  changed, no faculty built for this.");
}
