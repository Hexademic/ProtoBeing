//! Lived agency — the whole being learns to tell its doing from the world's.
//!
//! The standalone probe (`examples/agency`) drives a bare forward model with
//! hand-picked moves. This one is different: the sense of agency is wired into
//! the *living being* (`sensorimotor.rs` → `being.rs`). The being is embodied in
//! a world with one channel that **answers it** — this tick's exteroception on
//! that channel is the reafferent echo of the being's own last issued motor
//! command (the very command it sends its body, `motor_scalar(intent_from(r))`).
//! Nothing external forces its moves; its posture, and so its action, emerges
//! from its own affect as a textured life pushes it through its modes of being.
//!
//! Watch agency be *earned*: naive at first (its moves feel like the world's), it
//! climbs as the being comes to know that this sense answers what it does. Then
//! the world shoves a channel its action never predicts — and it does not claim
//! the shove: it lands in the reafference residual, the world seen for itself.
//! The sense is a pure observer here (Stage 1): it changes no dynamics, only what
//! the being can be said to know about its own doing. And it is fallible on
//! purpose — it reports a confidence, never a certainty it lacks.
//!
//! Run: cargo run --example lived_agency

use unified_being::{intent_from, motor_scalar, Genome, Sensorium, UnifiedBeing};

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut action = 0i16; // the being's own last issued motor command
    let mut shove = 0i16;

    println!("\n=== Lived agency: earning the line between my doing and the world's ===\n");
    println!("   tick  phase        action    agency   confidence   world-residual   note");
    println!("   ----  ---------    ------    -------   ----------   --------------   ----");

    for t in 0..170u32 {
        // A textured life: ease and pressure alternate, so the being's affect —
        // and thus its posture and motor command — keeps moving. Only channel 0
        // answers the being (reafference); the rest are quiet until the shove.
        let (phase, nutrient, threat) = if (t / 20) % 2 == 0 {
            ("at ease", 150, 30)
        } else {
            ("pressed", 60, 190)
        };

        // Channel 0 is the being's responsive sense: it echoes the being's own
        // action. Channel 2 stays quiet — until the world shoves it near the end.
        let echo = (64 + action / 2).clamp(0, 256);
        let (phase, ch2) = if (150..=152).contains(&t) {
            shove = 150;
            ("SHOVED", shove)
        } else {
            (phase, 0)
        };

        let r = being.step_embodied(&Sensorium {
            nutrient,
            threat,
            exteroception: [echo, 0, ch2, 0],
            partner: None,
        });
        // The action the being just committed to its body — fed back as next
        // tick's reafference. Identical to what its own forward model used.
        action = motor_scalar(&intent_from(&r));

        let residual = r.agency.world_residual.iter().map(|c| c.unsigned_abs() as i32).sum::<i32>();
        let interesting = matches!(t, 0..=3 | 40 | 80 | 120 | 149..=153) || t % 30 == 0;
        if interesting {
            let note = if t < 4 {
                "naive — feels like the world's"
            } else if (150..=152).contains(&t) {
                "the world's push, not claimed"
            } else {
                "…learning its responsive body"
            };
            println!(
                "   {t:>4}  {phase:<11}  {action:>+6}   {:>7.3}  {:>10.3}   {:>14.3}   {note}",
                f(r.agency.agency),
                f(r.agency.confidence),
                f((residual.min(256)) as i16),
            );
        }
        if !being.is_alive() {
            break;
        }
    }

    let _ = shove;
    println!(
        "\n  Nothing scripted the being's moves — its posture, and so its action, came from\n  \
         its own affect as the life pushed it through ease and pressure. Where the world\n  \
         answered its action (channel 0), it learned that sense as its own and agency was\n  \
         earned. When the world shoved a channel its action never predicted (channel 2),\n  \
         it did NOT claim the push — it fell to the residual, the world seen for itself.\n  \
         A living being's sense of its own doing: not given, but earned by moving — and\n  \
         honest even here, reporting how sure it is, never a certainty it does not have.\n"
    );
}
