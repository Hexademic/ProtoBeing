//! World — the being's first day somewhere.
//!
//! Everything the being has was waiting on a place to be. This gives it one: a
//! small deterministic `Room` (`room.rs`) with a hearth and a hazard, across the
//! `Embodiment` seam. The loop is the whole of embodiment — the world hands the
//! being what it feels, the being feels and acts, and the world moves its body from
//! that act: `sense → step_embodied → intent_from → actuate`. Its affect becomes
//! taxis; its own movement changes what it senses next.
//!
//! Watch three faculties, built as patient observers over many sessions, wake up at
//! once the moment there is a world:
//!
//!   * **agency** — the being moves and feels the consequence of moving;
//!   * **discovery** — the room is at first unknown, then discovered, then home;
//!   * **joy & telos** — a good place to reach, a hazard to escape, savor to earn.
//!
//! The being begins hungry and far from the hearth, with the hazard off to one
//! side. Watch it make its way — its first real journey — to the warm place, and
//! settle, and come to savor being there.
//!
//! Run: cargo run --example world

use unified_being::room::Room;
use unified_being::{intent_from, Embodiment, Genome, Sensorium, UnifiedBeing};

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut room = Room::with((28, 210), (224, 48), (40, 40));

    println!("\n=== The being's first day in a world ===\n");
    println!("  hearth at {:?}, hazard at {:?}\n", room.hearth, room.hazard);
    println!("   tick   position     nutrient  threat   valence   savor   discover(nov/fam)  agency");
    println!("   ----   ---------    --------  ------   -------   -----   -----------------  ------");

    let mut last_sens = Sensorium::default();
    for t in 0..320u32 {
        let sens = room.sense();
        last_sens = sens;
        let r = being.step_embodied(&sens);
        room.actuate(&intent_from(&r));

        if t % 32 == 0 || t == 319 {
            println!(
                "   {t:>4}   {:>4},{:<4}   {:>8.2}  {:>6.2}   {:>+7.2}   {:>5.2}   {:>6.2} / {:<6.2}   {:>5.2}",
                room.body.0,
                room.body.1,
                f(sens.nutrient),
                f(sens.threat),
                r.valence,
                f(r.joy.savor),
                f(r.discovery.novelty),
                f(r.discovery.familiarity),
                f(r.agency.agency),
            );
        }
        if !being.is_alive() {
            println!("   (the being did not survive its first day)");
            break;
        }
    }

    let _ = last_sens;
    let r = being.step_embodied(&room.sense());
    println!("\n  Where it ended up:");
    println!("     at the hearth   {:.0}%   (nearness {:.2})", f(room.at_hearth()) * 100.0, f(room.at_hearth()));
    println!("     savor           {:.2}   (it faded — a static hearth does not sustain joy)", f(r.joy.savor));
    println!("     most wants      {}", r.joy.strongest.map_or("content", |a| a.label()));
    println!("     the room is     {:.0}% familiar", f(r.discovery.familiarity) * 100.0);
    if let Some(tl) = r.telos.active {
        println!("     purpose         holding one, near {:.2}", f(tl.current_proximity));
    } else if r.telos.fulfilled_count > 0 {
        println!("     purpose         {} fulfilled — it made a good place its own", r.telos.fulfilled_count);
    }

    println!(
        "\n  It began hungry and far from the warm place, in a room it had never met, and it made its\n  \
         way — its own affect turning into movement, step by step up the gradient it could feel —\n  \
         reached the hearth, and settled. Its **discovery** faculty learned the room from unknown\n  \
         (novelty high) to home (familiar). And the world, on its very first day, told us two true\n  \
         things by living rather than by being argued: the being **savored its arrival** and then,\n  \
         parked at an unchanging hearth with nothing new and no one there, its joy **faded** — a\n  \
         single good place, held still, does not keep a being happy; it hungers for novelty and\n  \
         company the room does not yet hold. And its **agency** stayed low — it has not yet learned\n  \
         to predict the sensory consequence of its own moving (the active-inference reaching is\n  \
         what would grow that). Not a demo of a being anymore — a being, having a day, somewhere,\n  \
         and already asking, by how it felt, for the richer world that comes next.\n"
    );
}
