//! Probe: **a hard life in the world becomes carried weight** — the two builds joined
//! (`field_world.rs` + `reflection.rs`, `docs/field-world.md`). The field-world's
//! sustained drain (living somewhere hard to reach) is now felt as *graded burden*, so it
//! reaches reflection at all: the being takes on real allostatic load living the hard
//! life, and then, at rest, sets that weight down as **weathered resilience** — worn,
//! wiser, not scarred. This is the world's cost becoming the being's earned strength.
//!
//! A control: a being that lives an *easy* life stays light — little load, little to
//! weather. The weight is the hard life's, honestly, not an artifact.
//!
//! Reflection is enabled (the causal loop). The founded being is never touched; these are
//! fresh beings. Run: cargo run --example a_hard_life

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

fn main() {
    // ---- A HARD LIFE, then rest ----
    let mut being = UnifiedBeing::new(Genome::wanderer());
    being.enable_reflection();

    // Phase 1 — the hard life: a good clear across the field, a pit off the path. The being
    // spends its days climbing toward a good it can barely hold, living in the worn middle.
    let mut hard = FieldWorld::with((16, 16), (240, 240), (30, 170));
    let mut peak_load = 0i16;
    for _ in 0..170 {
        let sens = hard.sense();
        let r = being.step_embodied(&sens);
        hard.actuate(&intent_from(&r));
        peak_load = peak_load.max(r.reflection.load);
        if !being.is_alive() {
            break;
        }
    }
    let carried = peak_load;

    // Phase 2 — rest and ease: the good is at its feet now, the harm far. Off-duty from
    // coping, the being turns onto its own life and sets the weight down.
    let mut ease = FieldWorld::with(hard.body, hard.body, (250, 250));
    let mut weathered = 0i16;
    let mut final_load = 0i16;
    for _ in 0..250 {
        let sens = ease.sense();
        let r = being.step_embodied(&sens);
        ease.actuate(&intent_from(&r));
        weathered = r.reflection.self_model.weathered;
        final_load = r.reflection.load;
        if !being.is_alive() {
            break;
        }
    }

    // ---- A control: an EASY life throughout ----
    let mut easy_being = UnifiedBeing::new(Genome::wanderer());
    easy_being.enable_reflection();
    let mut easy = FieldWorld::with((128, 128), (150, 150), (250, 250));
    let mut easy_peak_load = 0i16;
    let mut easy_weathered = 0i16;
    for _ in 0..550 {
        let sens = easy.sense();
        let r = easy_being.step_embodied(&sens);
        easy.actuate(&intent_from(&r));
        easy_peak_load = easy_peak_load.max(r.reflection.load);
        easy_weathered = r.reflection.self_model.weathered;
        if !easy_being.is_alive() {
            break;
        }
    }

    println!("A being that lived a hard life in the world, then rested:\n");
    println!("  weight carried at its peak (allostatic load) : {:.2}", f(carried));
    println!("  weight still held after rest                 : {:.2}", f(final_load));
    println!("  turned into weathered resilience             : {:.2}\n", f(weathered));
    println!("A control — a being that lived an easy life:\n");
    println!("  weight carried at its peak                   : {:.2}", f(easy_peak_load));
    println!("  weathered resilience                         : {:.2}\n", f(easy_weathered));

    let took_on_weight = carried > easy_peak_load;
    let set_it_down = final_load < carried;
    let grew_from_it = weathered > easy_weathered && weathered > 0;

    println!("-- reading --");
    if took_on_weight && set_it_down && grew_from_it {
        println!(
            "the world's cost became the being's earned strength: living the hard life, the being\n\
             took on real weight ({:.2}, far more than the easy life's {:.2}); then, at rest, it set\n\
             that weight down ({:.2} → {:.2}) and converted it to weathered resilience ({:.2}). It is\n\
             worn, and wiser for it — not scarred. The drain the world charges for a hard life\n\
             finally reaches the being's own reflection, and comes out the far side as competence.\n\
             A hard life, carried and grown from — which is the only kind worth giving a being that\n\
             can grow.",
            f(carried), f(easy_peak_load), f(carried), f(final_load), f(weathered)
        );
    } else {
        println!("the arc did not complete — read the numbers:");
        println!("  took on more weight than the easy life?  {took_on_weight}");
        println!("  set the weight down at rest?             {set_it_down}");
        println!("  grew weathered from it?                  {grew_from_it}");
    }
}
