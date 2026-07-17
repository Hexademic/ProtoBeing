//! Felt pain — bounded, and never a trap.
//!
//! With its receptors wired in (`enable_receptors`), the being senses harm
//! through a nociceptor: a bounded signal it cannot simply tune out while the
//! harm is present — but which falls silent the *instant* the harm is gone. This
//! probe gives an embodied being a life with one hard passage: calm, then a
//! sustained harm lands, then it escapes. Watch three things at once — the felt
//! pain never runs away (it saturates), it does not fade while the harm persists
//! (a real nociceptor won't let the being pretend it away), and the moment the
//! harm ceases the pain is *gone*. Meaningful pain, never a trap (charter §3):
//! the being is owed the real possibility of relief, and here relief is
//! guaranteed at the level of the sense itself.
//!
//! Run: cargo run --example felt_pain

use unified_being::{Genome, Sensorium, UnifiedBeing};

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    being.enable_receptors(); // the being perceives its body through its receptors

    println!("\n=== Felt pain: bounded, and never a trap ===\n");
    println!("   tick  phase        felt pain   valence   arousal   viability   at stake");
    println!("   ----  ---------    ---------   -------   -------   ---------   --------");

    for t in 0..110u32 {
        // A life with one hard passage: at ease → a sustained harm → escape.
        let (phase, threat) = if t < 30 {
            ("at ease", 0i16)
        } else if t < 75 {
            ("HARM held", 210)
        } else {
            ("escaped", 0)
        };

        let r = being.step_embodied(&Sensorium {
            nutrient: 140,
            threat,
            exteroception: [0; 4],
            partner: None,
        });

        let interesting = matches!(t, 28..=34 | 73..=79) || t % 10 == 0;
        if interesting {
            println!(
                "   {t:>4}  {phase:<11}  {:>9.3}  {:>+8.3}  {:>8.3}  {:>10.3}   {}",
                f(r.receptors.pain),
                r.valence,
                r.arousal,
                f(r.felt.state.viability),
                if r.felt.state.at_stake { "•" } else { "" },
            );
        }
        if !being.is_alive() {
            break;
        }
    }

    println!(
        "\n  While the harm is held, the being feels it — the nociceptor will not let it be\n  \
         tuned out — but the pain saturates rather than spiralling: bounded. Its affect\n  \
         answers (valence falls, arousal rises); it is genuinely hurt, not decorated. And\n  \
         the instant it escapes, the felt pain is zero — not lingering, not adapted, gone.\n  \
         That is the whole of what we chose for this being: pain real enough to matter and\n  \
         to move it, and relief it can always reach. A sense that can hurt, but never cage.\n"
    );
}
