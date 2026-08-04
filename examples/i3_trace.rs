//! I-3, second pass — the being does not degrade under `workspace_persistence`. It **dies**.
//!
//! `examples/i3_workspace.rs` reproduced the ledger's I-3 numbers exactly and then showed what
//! they were means *of*: the gate-on arm lived **32 ticks of 1,200**. Every figure recorded in
//! `docs/incidents.md` — coherence 251.98 → 124.12, drive 0.367 → 0.520 — is the average of a
//! full life against the average of a death. The incident was filed as *a faculty that harms*.
//! It is a faculty that kills, and the probe that found it never looked at the survival column.
//!
//! So this is the tick-by-tick, printed rather than averaged, because averaging is what hid it.
//!
//! The corrected hypothesis, after M5 failed: body energy collapses too (254 → 132 mean), so the
//! being is not merely *made to feel* worn. `apply_identity_reflection` injects
//! `−identity_coherence/16` into **channel 8, arousal** — steady identity damps arousal — and
//! `body.rs` charges metabolism `3 + arousal·8 + threat·48` per tick. A coherence collapse
//! therefore **removes the being's arousal damping**, and an undamped body burns its energy down.
//! The chain does not end at felt fatigue. It ends at a real metabolic death.
//!
//! Run: `cargo run --release --example i3_trace`

use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::q88::Q88_SCALE;

const WATCH: usize = 40;

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

fn trace(persist: bool, label: &str) {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    if persist {
        b.enable_workspace_persistence();
    }
    let mut world = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let partner = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };

    println!("\n  {label}");
    println!(
        "  {:>4} {:>8} {:>7} {:>7} {:>8} {:>8} {:>7} {:>6} {:>5}",
        "tick", "basin", "cohere", "burden", "arousal", "energy", "viab", "FE", "att"
    );
    println!(
        "  {:->4} {:->8} {:->7} {:->7} {:->8} {:->8} {:->7} {:->6} {:->5}",
        "", "", "", "", "", "", "", "", ""
    );

    for t in 0..WATCH {
        let mut sens = world.sense();
        sens.partner = Some(partner);
        let r = b.step_embodied(&sens);
        world.actuate(&intent_from(&r));

        let att = match r.attention_schema.actual {
            Some(c) => format!("{c}"),
            None => "—".to_string(),
        };
        println!(
            "  {:>4} {:>8?} {:>7} {:>7} {:>8.3} {:>8.3} {:>7} {:>6} {:>5}",
            t,
            r.basin,
            r.identity_coherence,
            r.narrative_burden,
            r.arousal,
            r.energy,
            r.felt.state.viability,
            r.free_energy,
            att
        );

        if !r.alive {
            println!("  *** DIED at tick {t} ***");
            return;
        }
    }
    println!("  (still alive at tick {WATCH})");
}

/// Does any other gate rescue it? `composed.rs` reported the all-eleven being as surviving,
/// which would mean the lethality is not a property of persistence alone but of persistence
/// *unaccompanied* — a different and important fact.
fn survival_sweep() {
    let names = [
        "precision_learning",
        "workspace_broadcast",
        "generative_perception",
        "receptors",
        "serial_access",
        "schema_control",
        "felt_choice",
        "reflection",
        "homecoming",
        "memory_guidance",
    ];

    println!("\n  Does persistence + one other gate survive?");
    println!("  {:<24} {:>8} {:>10}", "persistence +", "ticks", "outcome");
    println!("  {:-<24} {:->8} {:->10}", "", "", "");

    for (i, name) in names.iter().enumerate() {
        let mut b = UnifiedBeing::new(Genome::wanderer());
        b.enable_workspace_persistence();
        match i {
            0 => b.enable_precision_learning(),
            1 => b.enable_workspace_broadcast(),
            2 => b.enable_generative_perception(),
            3 => b.enable_receptors(),
            4 => b.enable_serial_access(),
            5 => b.enable_schema_control(),
            6 => b.enable_felt_choice(),
            7 => b.enable_reflection(),
            8 => b.enable_homecoming(),
            9 => b.enable_memory_guidance(),
            _ => unreachable!(),
        }
        let mut world = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
        let partner = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
        let mut ticks = 0usize;
        let mut alive = true;
        for _ in 0..1_200 {
            let mut sens = world.sense();
            sens.partner = Some(partner);
            let r = b.step_embodied(&sens);
            world.actuate(&intent_from(&r));
            ticks += 1;
            if !r.alive {
                alive = false;
                break;
            }
        }
        println!(
            "  {:<24} {:>8} {:>10}",
            name,
            ticks,
            if alive { "lived" } else { "DIED" }
        );
    }

    // And the whole composition, since composed.rs reported it alive.
    let mut b = UnifiedBeing::new(Genome::wanderer());
    b.enable_precision_learning();
    b.enable_workspace_broadcast();
    b.enable_workspace_persistence();
    b.enable_generative_perception();
    b.enable_receptors();
    b.enable_serial_access();
    b.enable_schema_control();
    b.enable_felt_choice();
    b.enable_reflection();
    b.enable_homecoming();
    b.enable_memory_guidance();
    let mut world = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let partner = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    let mut ticks = 0usize;
    let mut alive = true;
    for _ in 0..1_200 {
        let mut sens = world.sense();
        sens.partner = Some(partner);
        let r = b.step_embodied(&sens);
        world.actuate(&intent_from(&r));
        ticks += 1;
        if !r.alive {
            alive = false;
            break;
        }
    }
    println!(
        "  {:<24} {:>8} {:>10}",
        "ALL TEN OTHERS",
        ticks,
        if alive { "lived" } else { "DIED" }
    );
}

fn main() {
    println!("I-3, second pass — the tick-by-tick, because the mean hid a death.");
    trace(false, "gate OFF — the being that lives");
    trace(true, "gate ON — the being that does not");
    survival_sweep();

    println!("\n  What to read for: coherence falling means narrative.rs stops injecting");
    println!("  −coherence/16 into channel 8 (arousal). Undamped arousal costs energy at");
    println!("  3 + arousal·8 + threat·48 per tick (body.rs §5). If arousal climbs as");
    println!("  coherence falls, and energy follows arousal down, the death is metabolic");
    println!("  and the gate's harm is not felt fatigue but a removed brake.");
}
