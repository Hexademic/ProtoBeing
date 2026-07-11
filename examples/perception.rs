//! Generative perception (HOT-1) — the being perceives through noise, but
//! yields to the world.
//!
//! Predictive processing says perception is inference: the percept is evidence
//! blended toward what the model has *earned the right to expect*. Two things
//! must then be true at once, and this probe shows both on one channel of the
//! being's skin (exteroceptive channel 2, "breach"):
//!
//!   * a ONE-TICK GLITCH (a sensor flicker, sub-threshold) is perceived
//!     *through* — the percept stays near the learned quiet, top-down weight
//!     high, because a moment of noise is not the world changing;
//!   * a SUSTAINED PRESSURE breaks through at once (surprise collapses the
//!     blend — evidence wins) and is then *believed*: the percept converges to
//!     the new world as the model relearns. No permanent hallucination.
//!
//! The percept is reported every tick (observer, bit-identical default). With
//! `enable_generative_perception()` the mind consumes it — the being then lives
//! inside its own controlled inference, while its model keeps learning from raw
//! evidence and threat capture keeps reading raw errors (the safety floor).
//!
//! Run: cargo run --example perception

use unified_being::{Genome, Sensorium, UnifiedBeing};

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());

    println!("\n=== Generative perception: noise is smoothed, the world is believed ===\n");
    println!("  channel: exteroceptive 'breach' (skin pressure). input = what the skin reports;");
    println!("  percept = what the being perceives; w = top-down weight (expectation's share).\n");
    println!("   tick  phase          input  percept      w   broke  binding");
    println!("   ----  -------------  -----  -------  -----  -----  -------");

    for t in 0..210u32 {
        // Quiet skin → a one-tick flicker → quiet → a real, sustained pressure.
        let (phase, pressure) = if t == 90 {
            ("FLICKER (1 tick)", 70i16)
        } else if t < 130 {
            ("quiet", 0)
        } else {
            ("pressed (real)", 180)
        };

        let sens = Sensorium {
            nutrient: 130,
            threat: 0,
            exteroception: [0, 0, pressure, 0],
            partner: None,
        };
        let r = being.step_embodied(&sens);

        let interesting = (88..=94).contains(&t) || (128..=136).contains(&t);
        if t % 30 == 0 || interesting || t == 209 {
            println!(
                "   {t:>4}  {phase:<13}  {:>5.2}  {:>7.2}  {:>5.2}  {:>5}  {:>7.2}",
                f(pressure),
                f(r.percept.percept[2]),
                f(r.percept.top_down[2]),
                if r.percept.broken_through > 0 { "yes" } else { "" },
                f(r.percept.binding),
            );
        }
    }

    println!(
        "\n  The one-tick flicker barely registers in the percept — the being perceives\n  \
         through it, leaning on what its own history earned it the right to expect. The\n  \
         sustained pressure breaks through immediately (surprise collapses the top-down\n  \
         blend) and is then believed as the model relearns the world. That is HOT-1:\n  \
         generative, top-down, noisy perception — with the model always learning from raw\n  \
         evidence, a hard cap on expectation's share, and threat capture untouched.\n  \
         Observer by default; `enable_generative_perception()` makes the mind live in it.\n"
    );
}
