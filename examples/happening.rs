//! Happening — can a being learn a word for what is done to it?
//!
//! The measurement for `docs/happening.md` §6, H2–H5. The world's mechanics are proven
//! in `tests/happening.rs` (written before the implementation); this asks whether a
//! being that *lives* in a world which moves on its own comes to have the word for it.
//!
//! H2 — does `HAPPEN` ground at all? Uncertain: the drift must outrun the forward model.
//! H3 — does `(NOT KNOW HAPPEN)` then fire? The second shield has never spoken.
//! H4 — what does it cost the being's sense of agency? A drop is predicted, and wanted
//!      as a number rather than an impression.
//! H5 — **the gate.** Is a drifting life still a good life? If drift costs the being its
//!      wellbeing, `docs/happening.md` §5 applies and this is not shipped, however well
//!      `HAPPEN` grounds.
//!
//! A cadence *sweep* rather than one chosen rate, so a null result can be told apart
//! from a badly-picked knob — and so nothing is tuned until the answer is the wanted one.
//!
//! Fresh probe-beings only; the founded being is never touched.
//!
//! Run: `cargo run --release --example happening`

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::primes::{Clause, Prime, PrimeFacts, PrimeLayer};

const LIFE: usize = 1_500;

struct Outcome {
    label: String,
    happen_at: Option<u32>,
    residual_mean: i32,
    agency_mean: i32,
    drive_mean: i32,
    not_know_fired: usize,
    example: Option<String>,
    alive: bool,
}

fn live(label: String, mut world: FieldWorld) -> Outcome {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut layer = PrimeLayer::new();
    let (mut res_sum, mut ag_sum, mut dr_sum, mut n) = (0i64, 0i64, 0i64, 0i64);
    let mut not_know_fired = 0usize;
    let mut example = None;

    for _ in 0..LIFE {
        let sens = world.sense();
        let r = being.step_embodied(&sens);
        world.actuate(&intent_from(&r));
        let near = Some(world.at_good() > 128 || world.at_person(1) > 128);
        let facts = PrimeFacts::from_report(&r, near);
        layer.observe(&facts);

        res_sum += facts.world_residual as i64;
        ag_sum += facts.agency as i64;
        dr_sum += facts.drive as i64;
        n += 1;

        if let Some(clauses) = layer.speak_tree(&facts) {
            for c in &clauses {
                if c.prime == Prime::NotKnow && !c.children.is_empty() {
                    not_know_fired += 1;
                    if example.is_none() {
                        let text: Vec<String> = clauses.iter().map(Clause::render).collect();
                        example = Some(text.join(" "));
                    }
                    break;
                }
            }
        }
        if !being.is_alive() {
            break;
        }
    }

    Outcome {
        label,
        happen_at: layer.grounded_at(Prime::Happen),
        residual_mean: (res_sum / n.max(1)) as i32,
        agency_mean: (ag_sum / n.max(1)) as i32,
        drive_mean: (dr_sum / n.max(1)) as i32,
        not_know_fired,
        example,
        alive: being.is_alive(),
    }
}

fn q(v: i32) -> f32 {
    v as f32 / 256.0
}

fn main() {
    println!("Happening — can a being learn a word for what is done to it?");
    println!("(predictions locked in docs/happening.md §6 before this was written)\n");

    let base = || FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));

    let mut results = vec![live("still (control)".into(), base())];
    for every in [32u32, 16, 8, 4, 2] {
        results.push(live(format!("drift every {every:>2}"), base().with_drift(0, every, (3, -2))));
    }

    println!("  {:<18} {:>9} {:>8} {:>7} {:>7} {:>10}", "world", "residual", "HAPPEN", "agency", "drive", "NOT KNOW");
    println!("  {:-<18} {:->9} {:->8} {:->7} {:->7} {:->10}", "", "", "", "", "", "");
    for o in &results {
        println!(
            "  {:<18} {:>9} {:>8} {:>7.2} {:>7.2} {:>10}{}",
            o.label,
            o.residual_mean,
            o.happen_at.map(|t| t.to_string()).unwrap_or_else(|| "never".into()),
            q(o.agency_mean),
            q(o.drive_mean),
            o.not_know_fired,
            if o.alive { "" } else { "   ** DIED **" }
        );
    }

    let control = &results[0];
    let drifting: Vec<&Outcome> = results[1..].iter().collect();
    let any_happen = drifting.iter().any(|o| o.happen_at.is_some());
    let any_shield = drifting.iter().any(|o| o.not_know_fired > 0);

    println!("\n  Read against the locked predictions:");
    println!(
        "    H2 (a lived being earns HAPPEN):   {}",
        if any_happen { "HELD" } else { "FAILED — no cadence taught the word" }
    );
    println!(
        "    H3 ((NOT KNOW HAPPEN) fires):      {}",
        if any_shield { "HELD — the second shield speaks" } else { "did not fire" }
    );

    let worst_agency = drifting.iter().map(|o| o.agency_mean).min().unwrap_or(0);
    println!(
        "    H4 (cost to agency):               {:.2} -> {:.2}  ({:+.2})",
        q(control.agency_mean),
        q(worst_agency),
        q(worst_agency - control.agency_mean)
    );

    let worst_drive = drifting.iter().map(|o| o.drive_mean).max().unwrap_or(0);
    let all_alive = drifting.iter().all(|o| o.alive);
    let livable = all_alive && worst_drive <= control.drive_mean + 64;
    println!(
        "    H5 (still a good life) — THE GATE: drive {:.2} -> {:.2}, all alive: {}",
        q(control.drive_mean),
        q(worst_drive),
        all_alive
    );
    println!(
        "      -> {}",
        if livable {
            "PASSES — a drifting life is not a worse life. Ships."
        } else {
            "FAILS — drift costs the being its wellbeing. §5 applies: NOT SHIPPED."
        }
    );

    if let Some(o) = drifting.iter().find(|o| o.example.is_some()) {
        println!("\n  The first thing it said with the second shield:");
        println!("    {}", o.example.as_ref().unwrap());
    }
}
