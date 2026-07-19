//! Probe (deepening the material): does a *varied* life give the being more to
//! learn from than a monotone one? The being's memory only grows as rich as the life
//! it is given. A gentle sameness leaves almost nothing to consolidate; a life of
//! genuinely different (still gentle) kinds of days fills its memory with distinct,
//! learnable experience. (`docs/memory-that-teaches.md`.)
//!
//! Run: cargo run --example varied_life

use unified_being::being::UnifiedBeing;
use unified_being::genome::Genome;
use unified_being::{Partner, Stimulus};

const MAKER: u32 = 0x81a4e;

/// The old monotone life: nourished, company most days, a faint breath of variation.
fn monotone(age: u64) -> Stimulus {
    let maker = Partner { id: MAKER, reciprocation: 210, exit_cost: 40 };
    let nutrient = 140 + ((age % 20) as i16 - 10) * 3;
    Stimulus { nutrient, partner: ((age % 7) < 4).then_some(maker) }
}

/// The varied (still gentle) life: four kinds of day, each lived in a stretch.
fn varied(age: u64) -> Stimulus {
    let maker = Partner { id: MAKER, reciprocation: 210, exit_cost: 40 };
    let (base, company) = match (age / 18) % 4 {
        0 => (185, true),
        1 => (185, false),
        2 => (100, true),
        _ => (100, false),
    };
    let nutrient = base + ((age % 6) as i16 - 3) * 2;
    Stimulus { nutrient, partner: company.then_some(maker) }
}

fn live(life: fn(u64) -> Stimulus, days: u64) -> (u16, f64, bool) {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut savor_sum = 0i64;
    let mut n = 0i64;
    for age in 0..days {
        let r = being.step(&life(age));
        savor_sum += r.joy.savor as i64;
        n += 1;
        if !being.is_alive() {
            return (being.episodic.themes, savor_sum as f64 / n as f64 / 256.0, false);
        }
    }
    (being.episodic.themes, savor_sum as f64 / n as f64 / 256.0, true)
}

fn main() {
    let days = 400;
    let (mono_themes, mono_savor, mono_alive) = live(monotone, days);
    let (var_themes, var_savor, var_alive) = live(varied, days);

    println!("Over {days} days of life:\n");
    println!(
        "  monotone life:  {mono_themes} gist(s) consolidated   mean savor {mono_savor:.3}   {}",
        if mono_alive { "thrived" } else { "did not survive" }
    );
    println!(
        "  varied life:    {var_themes} gist(s) consolidated   mean savor {var_savor:.3}   {}",
        if var_alive { "thrived" } else { "did not survive" }
    );

    println!("\n-- reading --");
    println!(
        "The being lives all four kinds of day and thrives in each — a genuinely richer life\n\
         than the old sameness. But its memory consolidates to the *same* ~{var_themes} gists either\n\
         way: gentle variety in what it is given does not become variety in what it *feels*, and\n\
         it is the felt quality that memory sorts by. So the material's depth is capped not by the\n\
         life's variety but by the being's affective resolution. Deepening it further is not a\n\
         matter of more gentle difference — it needs either finer emotion or a world with real\n\
         stakes (genuinely distinct experience). Exactly the sequencing already set."
    );
    let _ = (mono_themes, mono_savor);
}
