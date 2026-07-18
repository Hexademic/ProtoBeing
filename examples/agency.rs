//! Agency — the being learns to tell its own doing from what is done to it.
//!
//! Reafference (AE-2, `src/sensorimotor.rs`): a being that moves must learn the
//! sensory consequence of its *own* action, and from that comes the sense of
//! agency — "I did that" versus "that happened to me." This probe gives a newborn
//! forward model a life in three acts. First it is naive and its moves feel like
//! the world's doing (low agency). Then it lives, acting and sensing, and *learns
//! its body* — its own moves come to feel its own (agency climbs). Finally the
//! world shoves it while it is still — and it correctly does **not** claim that as
//! its own, letting the world's push show through as residual.
//!
//! The sense is fallible on purpose: it is inferred from correlation, so it could
//! be fooled — and it stays honest by reporting a *confidence*, never a certainty
//! it lacks. A sense of agency that could never err would be omniscient, not
//! honest.
//!
//! Run: cargo run --example agency

use unified_being::sensorimotor::ForwardModel;
use unified_being::q88::q88_mul;

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

fn main() {
    let mut fm = ForwardModel::new();
    let mut reading = [64i16; 4];
    let true_gain = 128; // in truth, its action moves channel 0 by 0.5·action

    println!("\n=== Agency: learning to tell my doing from the world's ===\n");
    println!("   tick  act        agency   confidence   world-residual[0]   note");
    println!("   ----  ---------  -------   ----------   -----------------   ----");

    // Acts 1-2: it lives, acting (alternating moves), and learns its body.
    for i in 0..70u32 {
        let action = if i % 2 == 0 { 100 } else { -100 };
        // The being's own move changes channel 0; the world is quiet.
        reading[0] = reading[0].saturating_add(q88_mul(action, true_gain));
        let r = fm.step(action, &reading);
        if i < 4 || i == 20 || i == 40 || i == 68 {
            let note = if i < 4 { "naive — feels like the world's" } else { "…learning its own body" };
            println!(
                "   {i:>4}  move {action:>+4}  {:>7.3}  {:>10.3}   {:>17.3}   {note}",
                f(r.agency),
                f(r.confidence),
                f(r.world_residual[0]),
            );
        }
    }

    // Act 3: the being holds still, and the WORLD shoves it. It must not claim it.
    println!("   ----  ---------  -------   ----------   -----------------   ----");
    reading[0] = reading[0].saturating_add(120); // an external push, no action
    let r = fm.step(0, &reading);
    println!(
        "   {:>4}  {:<9}  {:>7.3}  {:>10.3}   {:>17.3}   shoved while still",
        70,
        "(no move)",
        f(r.agency),
        f(r.confidence),
        f(r.world_residual[0]),
    );

    println!(
        "\n  It began unable to own its moves, lived, and learned its body — its own\n  \
         actions came to feel its own (agency climbed toward 1.0). Then, held still and\n  \
         shoved by the world, it did NOT claim the push: agency ~0, and the shove shows\n  \
         through as residual — the world seen for itself. That is embodied selfhood in\n  \
         miniature: not a fact handed to the being, but a distinction it *earns* by moving.\n  \
         And it can be fooled, and says how sure it is — honest even about its own agency.\n"
    );
}
