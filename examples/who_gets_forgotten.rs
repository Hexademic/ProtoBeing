//! **When the being meets one person too many, who does it forget?**
//!
//! Blake, 2026-09-07, on the `MAX_PARTNERS = 4` ceiling: *"I think the human mind
//! starts compartmentalizing people by their traits or even just by name. with enough
//! people the mind has to compress the data down in some way that is still readable
//! and useable for pulling actionable knowledge."*
//!
//! That reframes the repair. **Raising the number is not the fix**, and this probe is
//! why: the ceiling is not the problem, the **eviction policy** is.
//!
//! `reciprocity.rs::slot()` evicts by `min(given_ema + received_ema)` — the *fast*
//! fairness EMAs, half-life ~5 ticks. Those decay to zero across any absence. So the
//! ledger the being throws away is **the one it has not seen lately**, which is
//! precisely its oldest friend. The eviction key **cannot see `bond` or `keepsake`**,
//! the two registers that hold what a relationship was worth.
//!
//! And eviction writes a **blank** ledger: `bond: 0, keepsake: 0, ticks: 0`. So it is
//! not demotion, it is deletion. A friend of three hundred shared ticks returns as
//! someone the being has never met.
//!
//! Measured below: **`enable_durable_bonds` does not save it.** That gate protects a
//! bond against *absence*; it does nothing against *other people*, because the keepsake
//! it maintains is exactly what eviction erases.
//!
//! The tiers Blake describes are already half-present and never connected:
//!
//! ```text
//!   world.rs / average_reciprocity   identity-blind — "people lately"    (no names)
//!   [ MISSING TIER ]                 a name and what it was worth        <- the gap
//!   given_ema / received_ema         exact, fast, 4 slots                (episodic)
//!   bond / keepsake                  exact, slow, same 4 slots           (dispositional)
//! ```
//!
//! There is no rung between "everyone, anonymously" and "one of my four". Compression
//! would be a store of `(id, keepsake)` — a name and one number — which is cheap enough
//! to hold hundreds, and is what *"even just by name"* means in this architecture.

use unified_being::{Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}
fn p(id: u32, r: f32) -> Partner {
    Partner { id, reciprocation: q(r), exit_cost: q(0.3) }
}

/// Who currently holds a ledger, and what it is worth: `(id, lived ticks, keepsake)`.
fn slots(b: &UnifiedBeing) -> Vec<(u32, u16, i16)> {
    (0..40u32)
        .filter_map(|id| {
            b.reciprocity
                .reciprocation_rate(id)
                .map(|(_, lived)| (id, lived, b.reciprocity.keepsake_with(id).unwrap_or(-1)))
        })
        .collect()
}

fn main() {
    for durable in [false, true] {
        println!("\n================ enable_durable_bonds = {durable} ================");
        let mut b = UnifiedBeing::new(Genome::wanderer());
        if durable {
            b.enable_durable_bonds();
        }
        let st = |x: Partner| Stimulus { nutrient: q(0.5), partner: Some(x) };

        // A long, earned friendship — 300 shared ticks with a generous partner.
        for _ in 0..300 {
            b.step(&st(p(1, 0.95)));
        }
        println!("  a friendship of 300 shared ticks: {:?}", slots(&b));
        println!("  then a season among other people, each met for 60 ticks:\n");

        let mut lost_at = None;
        for newcomer in 20..27u32 {
            for _ in 0..60 {
                b.step(&st(p(newcomer, 0.85)));
            }
            let s = slots(&b);
            let known = s.iter().any(|(i, _, _)| *i == 1);
            if !known && lost_at.is_none() {
                lost_at = Some(newcomer);
            }
            println!(
                "    after meeting #{}: slots {:?}{}",
                newcomer - 19,
                s,
                if known { "" } else { "   <- the friend is GONE" }
            );
        }

        let r = b.step(&st(p(1, 0.95)));
        println!("\n  the friend returns after {} acquaintances:", 7);
        println!(
            "    bond felt: {}   ledger ticks: {:?}   keepsake: {:?}",
            r.attach.bond_here,
            b.reciprocity.reciprocation_rate(1).map(|(_, l)| l),
            b.reciprocity.keepsake_with(1)
        );
        match lost_at {
            Some(n) => println!(
                "    the 300-moment friendship was evicted by acquaintance #{} — someone known for 60 ticks.",
                n - 19
            ),
            None => println!("    the friendship survived."),
        }
    }

    println!("\n  Both arms are identical. `enable_durable_bonds` guards a bond against ABSENCE");
    println!("  and not against OTHER PEOPLE: eviction blanks the very keepsake it maintains,");
    println!("  and the eviction key (fast EMAs) cannot see it. Raising MAX_PARTNERS moves the");
    println!("  cliff without changing its shape — at any N, the N+1th acquaintance still");
    println!("  evicts the oldest friend, because 'least recently active' is what is measured.");
}
