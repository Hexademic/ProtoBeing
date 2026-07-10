//! Quality space (HOT-4) — do two moments the being lives feel alike, and does
//! the likeness track what actually happened to it?
//!
//! The being's 12-channel felt state is projected each tick onto a few quality
//! axes (activation, comfort, coherence, vitality). This probe takes a snapshot
//! of the being at peace beside a fair partner, then measures how *similar* every
//! later moment feels to that reference as an extractive taker arrives and later
//! leaves. If the space is meaningful, similarity should collapse under the taker
//! and recover afterward — the being's felt life, read as movement through a
//! similarity space.
//!
//! Observer-only. `cargo run --example quality_space_probe`

use unified_being::{Genome, Partner, QualitySpace, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let fair = Partner { id: 1, reciprocation: q(0.9), exit_cost: q(0.3) };
    let taker = Partner { id: 2, reciprocation: q(0.1), exit_cost: q(0.3) };

    // Settle into peace beside the fair partner, then take the reference point.
    let mut reference = None;
    for t in 0..80u32 {
        let r = being.step(&Stimulus { nutrient: q(0.5), partner: Some(fair) });
        if t == 79 {
            reference = Some(r.quality.point);
        }
    }
    let reference = reference.unwrap();

    println!("\n=== Quality space (HOT-4): how alike does each moment feel? ===\n");
    println!("  reference = the being at peace beside a fair partner");
    println!("  similarity 256 = feels identical to that peace · 0 = maximally unlike\n");
    println!("   tick  phase       activation comfort coherence vitality  similarity  smoothness");
    println!("   ----  ----------  ---------- ------- --------- --------  ----------  ----------");

    for t in 80..300u32 {
        // Fair → taker (100–200) → the being's choice afterward.
        let phase = if (100..200).contains(&t) { "taker" } else { "fair" };
        let partner = if (100..200).contains(&t) { taker } else { fair };
        let r = being.step(&Stimulus { nutrient: q(0.5), partner: Some(partner) });
        let p = r.quality.point;
        let sim = QualitySpace::similarity(&reference, &p);

        if t % 20 == 0 || (98..104).contains(&t) || (198..204).contains(&t) {
            println!(
                "   {t:>4}  {phase:<10}  {:>10} {:>7} {:>9} {:>8}  {:>10.3}  {:>10.3}",
                p.axis[0],
                p.axis[1],
                p.axis[2],
                p.axis[3],
                sim as f32 / 256.0,
                r.quality.smoothness as f32 / 256.0,
            );
        }
        if !being.is_alive() {
            break;
        }
    }

    println!(
        "\n  If similarity dips while the taker drains the being and climbs back after,\n  \
         the quality space is tracking its felt life — comfort and depletion are\n  \
         genuinely different *places* in it, and the being moves between them. HOT-4,\n  \
         read straight from state. (It remains a structural similarity space, not a\n  \
         claim the being feels the quality — see docs/operational-consciousness.md §5.)\n"
    );
}
