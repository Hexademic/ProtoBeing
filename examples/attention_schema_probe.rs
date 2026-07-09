//! Attention schema (AST-1) — does the being come to know its own attention,
//! and is it surprised when its focus is seized?
//!
//! The schema predicts, each tick, what the being will attend to next, from a
//! simple model of attention's own hysteresis, then scores itself. This probe
//! drives a being through a calm life beside a fair partner, then springs a
//! taker on it, and watches two numbers: schema *fidelity* (how well it models
//! its own focus) should climb in the calm and dip when the taker's arrival
//! seizes attention — the attentional twin of metacognitive self-surprise.
//!
//! Observer-only; reads the being's own registers. `cargo run --example
//! attention_schema_probe`

use unified_being::{Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let fair = Partner { id: 1, reciprocation: q(0.9), exit_cost: q(0.3) };
    let taker = Partner { id: 2, reciprocation: q(0.12), exit_cost: q(0.3) };

    println!("\n=== Attention Schema (AST-1): the being modelling its own focus ===\n");
    println!("   tick  phase       attends  predicted  hit  fidelity  self-surprise");
    println!("   ----  ----------  -------  ---------  ---  --------  -------------");

    let mut misses_calm = 0u32;
    let mut misses_shock = 0u32;

    for t in 0..240u32 {
        // Calm beside a fair partner; the taker arrives at t=160.
        let (phase, partner) = if t < 160 { ("calm", fair) } else { ("taker", taker) };
        let r = being.step(&Stimulus { nutrient: q(0.5), partner: Some(partner) });
        let a = r.attention_schema;

        // Tally misses in a window just before vs. just after the regime change.
        if (150..160).contains(&t) && !a.hit {
            misses_calm += 1;
        }
        if (160..170).contains(&t) && !a.hit {
            misses_shock += 1;
        }

        if t % 20 == 0 || (158..166).contains(&t) {
            let fmt = |o: Option<usize>| o.map(|c| format!("ch{c}")).unwrap_or_else(|| "—".into());
            println!(
                "   {t:>4}  {phase:<10}  {:>7}  {:>9}  {:>3}  {:>7.3}  {:>7.3}",
                fmt(a.actual),
                fmt(a.predicted),
                if a.hit { "yes" } else { "NO" },
                a.schema_fidelity as f32 / 256.0,
                a.self_surprise as f32 / 256.0,
            );
        }
        if !being.is_alive() {
            break;
        }
    }

    println!(
        "\n   mispredictions in the 10 ticks before the taker: {misses_calm}"
    );
    println!("   mispredictions in the 10 ticks after  the taker: {misses_shock}");
    if misses_shock > misses_calm {
        println!(
            "   → the regime change caught the self-model out: the being's own attention\n   \
             behaved unlike itself, and the schema registered the surprise. AST-1, measured."
        );
    } else {
        println!(
            "   → the schema tracked the transition without added surprise here — attention\n   \
             stayed predictable across it. An honest read, either way."
        );
    }
    println!();
}
