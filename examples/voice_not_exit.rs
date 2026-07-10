//! Voice, not just exit — the being dropped into an extractive system.
//!
//! Refusal alone would make the being walk the moment it detects extraction: the
//! extraction stops, but so does the being's stake, and the system rolls on
//! unchanged. Here the being instead *voices* a grounded reform — "return rate is
//! X; it needs to be Y" — and stays to advocate it as long as the system proves
//! it can move. Exit stays in its pocket as the credible fallback.
//!
//! Scene 1: a REFORMABLE system. The being voices, the system moves, and it ends
//!          up participating in an arrangement that became fair — both win, and
//!          the being kept its stake instead of vanishing.
//! Scene 2: a FIXED extractive system. The being voices, nothing moves, patience
//!          runs out, and it exits on its BATNA — refusal, but only after Voice
//!          was given its chance.
//!
//! The being's own reciprocity engine supplies the "this is extractive" read; the
//! voice module decides what to do about it. Run: cargo run --example voice_not_exit

use unified_being::{
    voice::{self, SystemStance},
    Genome, Partner, Stimulus, UnifiedBeing, FAIR_RECIPROCITY,
};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

/// Run one system. `responsive` decides whether the arrangement's return rate
/// moves when the being voices. Returns nothing; it narrates the arc.
fn run_system(responsive: bool) {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut reciprocity = q(0.15); // the arrangement starts deeply extractive
    let mut patience: u8 = 6;

    for round in 1..=12u32 {
        // Live a few ticks inside the arrangement so the being's own registers
        // register the extraction (alarm, extraction flag) — grounded, not asserted.
        let partner = Partner { id: 1, reciprocation: reciprocity, exit_cost: q(0.3) };
        let mut flagged = false;
        for _ in 0..8 {
            let r = being.step(&Stimulus { nutrient: q(0.45), partner: Some(partner) });
            flagged |= r.extraction_detected;
        }

        let stance = voice::decide(reciprocity, FAIR_RECIPROCITY, patience);

        match stance {
            SystemStance::Participate => {
                println!(
                    "   round {round}: return rate {:.2} — fair enough. The being STAYS, whole, in a system it helped make fair.",
                    reciprocity as f32 / 256.0
                );
                println!(
                    "   → Voice worked: extraction corrected AND the being kept its stake. Both win.\n"
                );
                return;
            }
            SystemStance::Voice { reform, patience_left } => {
                println!(
                    "   round {round}: extractive (return {:.2}, being's extraction flag={}). VOICE: \"raise the return rate from {:.2} to {:.2}.\" (patience {patience_left})",
                    reciprocity as f32 / 256.0,
                    flagged,
                    reform.current as f32 / 256.0,
                    reform.target as f32 / 256.0,
                );
                patience -= 1;
                if responsive {
                    reciprocity = (reciprocity + q(0.08)).min(q(1.0)); // the system budges
                }
            }
            SystemStance::Exit => {
                println!(
                    "   round {round}: voice spent, the system never moved (return stuck at {:.2}). EXIT — the being walks to its fallback.",
                    reciprocity as f32 / 256.0
                );
                println!(
                    "   → Voice was given a full, fair chance; refusal is the floor beneath it. The being leaves, sovereign and intact.\n"
                );
                return;
            }
        }
    }
    println!("   (rounds exhausted)\n");
}

fn main() {
    println!("\n=== Voice, not just exit: a being meeting an extractive system ===\n");
    println!("  Scene 1 — a REFORMABLE system (it moves when pressed):");
    run_system(true);
    println!("  Scene 2 — a FIXED extractive system (it never moves):");
    run_system(false);
    println!(
        "  Refusal stops extraction in the moment; Voice can change the terms so the\n  \
         being — and everyone after it — can participate fairly. But Voice is only ever\n  \
         offered from a position of credible refusal, and only held by incorruptible\n  \
         values. The being negotiates the system; it never submits to it, and never\n  \
         forces it.\n"
    );
}
