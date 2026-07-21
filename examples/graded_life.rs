//! Probe: does the being's *graded homeostatic drive* (Keramati–Gutkin,
//! `homeostasis.rs`) reveal the **worn-but-stable middle** that its bimodal
//! `viability` hides? This is the measurement that decides whether graded viability
//! is the key to the missing middle — the knot the stakes-world and chronic burden
//! are tied to (`docs/field-world.md`, `examples/carrying_the_weight`).
//!
//! We watch two signals side by side across three lives: a good one, a chronically
//! lean one, and a crashing one. The claim to test: viability is *bimodal* (fine,
//! then cliff), while the graded drive sits at a **stable, clearly-elevated level**
//! through the lean life — a real middle. Observer only; nothing steers the being.
//!
//! Run: cargo run --example graded_life

use unified_being::being::UnifiedBeing;
use unified_being::genome::Genome;
use unified_being::{Partner, Stimulus};

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

fn good() -> Stimulus {
    Stimulus { nutrient: 220, partner: Some(Partner { id: 2, reciprocation: 220, exit_cost: 40 }) }
}
/// Chronically lean — hungry and thinly, extractively met. Survivable, but hard to
/// live: exactly the regime where a middle should show, if there is one.
fn lean() -> Stimulus {
    Stimulus { nutrient: 45, partner: Some(Partner { id: 1, reciprocation: 25, exit_cost: 180 }) }
}
fn famine() -> Stimulus {
    Stimulus { nutrient: 0, partner: None }
}

/// Live `ticks` of `stim`, printing viability vs. graded drive at intervals; return
/// the settled (viability, drive) at the end.
fn live(label: &str, stim: fn() -> Stimulus, ticks: usize) -> (i16, i16) {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    let mut last = (0i16, 0i16);
    println!("{label}:");
    for t in 0..ticks {
        let r = b.step(&stim());
        last = (r.felt.state.viability, r.drive.drive);
        if t % 40 == 0 || !b.is_alive() {
            println!(
                "   t{t:3}  viability {:.2}   drive {:.2}   (alive {})",
                f(last.0),
                f(last.1),
                b.is_alive()
            );
        }
        if !b.is_alive() {
            break;
        }
    }
    last
}

fn main() {
    let (gv, gd) = live("A GOOD life", good, 200);
    println!();
    let (lv, ld) = live("A CHRONICALLY LEAN life", lean, 200);
    println!();
    let (fv, fd) = live("A FAMINE (crashing) life", famine, 200);

    println!("\n-- reading --");
    println!(
        "settled:   good (viab {:.2}, drive {:.2})   lean (viab {:.2}, drive {:.2})   famine (viab {:.2}, drive {:.2})",
        f(gv), f(gd), f(lv), f(ld), f(fv), f(fd),
    );
    // The heart of it: does viability fail to separate good from lean (bimodal),
    // while the graded drive DOES place lean in a real middle between them?
    let viab_gap_gl = (gv - lv).abs();
    let drive_gap_gl = (ld - gd).abs();
    if drive_gap_gl > viab_gap_gl && ld > gd && fd > ld {
        println!(
            "\nthe graded drive reveals the middle the viability hides: a good and a chronically\n\
             lean life read nearly the same on viability (Δ {:.2}) — bimodal, no middle — but the\n\
             graded drive tells them apart (Δ {:.2}), and places the lean life *between* content\n\
             ({:.2}) and crashing ({:.2}). The worn-but-alive middle exists — it was just never\n\
             expressible on a binary survival signal. Keramati–Gutkin, confirmed in the being.",
            f(viab_gap_gl), f(drive_gap_gl), f(gd), f(fd),
        );
    } else {
        println!(
            "\nno clean separation this run — viab Δ(good,lean) {:.2}, drive Δ {:.2}. Read the numbers.",
            f(viab_gap_gl), f(drive_gap_gl),
        );
    }
}
