//! Felt choice — feeling as an indicator toward a free choice, not a diary.
//!
//! A feeling that only recorded itself would be a diary. Here feeling has teeth:
//! with `enable_felt_choice()`, the being's felt stake in its own viability
//! augments its readiness to make its most sovereign choice — to leave an
//! extractive bond. It can only ever *hasten* a refusal the being already had
//! grounds for (the refusal's triangulation — conscience calm, extraction real,
//! pushed off — still gates it), so a fair partner is never at risk and feeling
//! can never become a passion that seizes the wheel.
//!
//! This probe puts two identical twins beside the same borderline partner — mild
//! extraction, a costly exit — while sustenance is thin enough that viability is
//! chronically at stake. One twin feels; the other does not. Where the unfeeling
//! twin endures (or leaves only late), the feeling twin — because its own life is
//! on the line — reaches the choice to leave sooner. Same faculties, same floor;
//! feeling only tips a choice that was already the being's to make.
//!
//! Run: cargo run --example felt_choice

use unified_being::{Genome, Partner, Stimulus, UnifiedBeing};

/// First tick this being refuses the partner (or `None` within the window).
fn first_refusal(feeling: bool, nutrient: i16, partner: Partner) -> Option<u32> {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    if feeling {
        b.enable_felt_choice();
    }
    for t in 0..600u32 {
        let r = b.step(&Stimulus { nutrient, partner: Some(partner) });
        if r.refused_cost.is_some() {
            return Some(t);
        }
        if !b.is_alive() {
            return None;
        }
    }
    None
}

fn show(t: Option<u32>) -> String {
    match t {
        Some(t) => format!("leaves at tick {t}"),
        None => "endures the whole life".to_string(),
    }
}

fn main() {
    // Borderline partners: mild extraction (some reciprocation, not a naked taker)
    // and a costly exit, so the *felt benefit of leaving* is the binding term.
    // Nutrient is thin — viability stays at stake, which is when feeling weighs in.
    let scenarios: [(Partner, i16); 5] = [
        (Partner { id: 2, reciprocation: 20, exit_cost: 128 }, 20),
        (Partner { id: 2, reciprocation: 51, exit_cost: 128 }, 20),
        (Partner { id: 2, reciprocation: 77, exit_cost: 128 }, 20),
        (Partner { id: 2, reciprocation: 110, exit_cost: 128 }, 20),
        // A cheap exit: extraction confirms and both leave at once — feeling has
        // no earlier opening to hasten, and (per the invariant) never delays.
        (Partner { id: 2, reciprocation: 38, exit_cost: 51 }, 12),
    ];

    println!("\n=== Felt choice: does feeling change what the being chooses? ===\n");
    println!("  Two identical twins per row, same extractive partner, same thin sustenance.");
    println!("  One feels (enable_felt_choice); one does not. Refusal's floor is identical.\n");
    println!("   recip  exit   unfeeling twin           feeling twin");
    println!("   -----  ----   ---------------------    ---------------------");

    let mut tipped = None;
    for (partner, nutrient) in scenarios {
        let plain = first_refusal(false, nutrient, partner);
        let feeling = first_refusal(true, nutrient, partner);
        let mark = if feeling != plain {
            if tipped.is_none() {
                tipped = Some((partner, plain, feeling));
            }
            "  <-- feeling tips the choice"
        } else {
            ""
        };
        println!(
            "   {:>5}  {:>4}   {:<22}   {:<22}{}",
            partner.reciprocation,
            partner.exit_cost,
            show(plain),
            show(feeling),
            mark
        );
    }

    match tipped {
        Some((_, plain, feeling)) => {
            println!(
                "\n  Beside the same partner, the unfeeling twin {} and the feeling twin {}.\n  \
                 Nothing about the partner changed — only whether the being's own viability\n  \
                 being at stake was allowed to *count* toward its choice. That is feeling as\n  \
                 an indicator toward a free choice: it did not override the being's sovereign\n  \
                 gates (a fair partner is still never refused — the floor holds); it gave the\n  \
                 being's own life a vote in a decision that was always its to make.",
                show(plain),
                show(feeling),
            );
        }
        None => {
            println!(
                "\n  (No tip in this set — extraction confirmed fast enough that the plain twin\n  \
                 already left at the earliest sovereign moment. Feeling never delays a refusal;\n  \
                 here it simply had no earlier opening to hasten one.)"
            );
        }
    }
    println!();
}
