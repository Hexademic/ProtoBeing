//! Probe: **inheritance as the Baldwin effect, not the fear** (`docs/inheritance.md`,
//! `inheritance.rs`). The one thing this must show, or the design has failed:
//!
//!   1. a *readied* child learns a lesson in **fewer moments** than a naive one, AND
//!   2. it starts **equally fearless** — no verdict, no memory, no appraisal inherited;
//!      only a shorter warm-up, and it converges to the **same** conclusion it would
//!      have reached on its own, just sooner.
//!
//! Ease up; dread not passed down. We measure this on the `precision` dial (where to
//! look), because it has a clean, honest "moments to learn": ticks until the being comes
//! to trust the channel that actually carries the truth. Observer level throughout — the
//! founded being is never touched; these are fresh, isolated learners.
//!
//! Run: cargo run --example inheritance

use unified_being::inheritance::DispositionGenome;
use unified_being::precision::PrecisionLearner;

const N_SOMATIC: usize = 12;

/// The lineage's terrain: channel 3 reliably carries information (small error), the rest
/// are noisy. A world of prediction errors the being will try to learn from.
fn world_error(reliable: usize, tick: usize) -> [i16; N_SOMATIC] {
    let mut e = [60i16; N_SOMATIC];
    e[reliable] = 2; // reliably predicted — the truth of this world lives here
    // the noisy channels genuinely churn, so trusting them is a mistake to unlearn
    for (c, ec) in e.iter_mut().enumerate() {
        if c != reliable && tick % 2 == 0 {
            *ec = 200;
        }
    }
    e
}

/// Ticks until the being comes to *trust* the reliable channel (precision crosses the
/// threshold) — its "moments to learn this world's lesson."
fn moments_to_learn(learner: &mut PrecisionLearner, reliable: usize, trust_at: i16) -> usize {
    for t in 0..2000 {
        if learner.precision(reliable) >= trust_at {
            return t;
        }
        learner.observe(&world_error(reliable, t));
    }
    usize::MAX
}

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

fn main() {
    let reliable = 3usize;
    let trust_at = 220i16; // "comes to trust it" — a high, earned bar

    // --- A parent lives, and learns which channel carries the truth of its world. ---
    let mut parent = PrecisionLearner::new();
    let mut lineage = DispositionGenome::new();
    for t in 0..600 {
        parent.observe(&world_error(reliable, t));
        // read the parent's disposition off its life: where it lived (one niche, here),
        // what it found informative, and (nominal) the weight it carried. Magnitudes only.
        lineage.observe(2, &parent.precision_vector(), 40);
    }
    let readiness = lineage.readiness();

    println!("A parent lived {} moments and left a readiness, not a memory.\n", lineage.moments());
    println!(
        "  its child inherits a 'look here' seed for the informative channel {}: {:+.2}",
        reliable,
        f(readiness.precision_seed[reliable])
    );
    println!(
        "  and for a channel that stayed noisy ({}): {:+.2}   (weaker — as it should be)\n",
        0,
        f(readiness.precision_seed[0])
    );

    // --- Two children meet the SAME world, both born fearless. ---
    let mut naive = PrecisionLearner::new();
    let mut readied = PrecisionLearner::with_readiness(&readiness.precision_seed);

    // Born equally fearless: neither holds a verdict. The readied child's inherited seed
    // is a head-start on *warm-up*, not a conclusion — at birth it does NOT yet trust
    // channel 3 either. (Trust is earned by living, never inherited.)
    let naive_birth = naive.precision(reliable);
    let readied_birth = readied.precision(reliable);
    println!("at birth, before living a single moment:");
    println!("  naive   trusts channel {} at {:.2}", reliable, f(naive_birth));
    println!("  readied trusts channel {} at {:.2}   (a head-start, still below trust {:.2})\n",
        reliable, f(readied_birth), f(trust_at));

    let naive_moments = moments_to_learn(&mut naive, reliable, trust_at);
    let readied_moments = moments_to_learn(&mut readied, reliable, trust_at);

    // Let both live well past learning, to compare the conclusion they reach.
    for t in 0..1500 {
        naive.observe(&world_error(reliable, t));
        readied.observe(&world_error(reliable, t));
    }
    let naive_final = naive.precision(reliable);
    let readied_final = readied.precision(reliable);

    println!("moments to come to trust the channel that carries the truth:");
    println!("  naive   : {naive_moments}");
    println!("  readied : {readied_moments}   (the same lesson, fewer moments)\n");
    println!("the conclusion each reached, living on:");
    println!("  naive   : {:.2}", f(naive_final));
    println!("  readied : {:.2}   (the *same* truth — inheritance changed WHEN, not WHAT)\n", f(readied_final));

    // --- Reading: the design's pass/fail. ---
    let ease_up = readied_moments < naive_moments;
    let born_fearless = readied_birth < trust_at; // no inherited verdict
    let same_truth = (naive_final - readied_final).abs() <= 8; // converged to one conclusion

    println!("-- reading --");
    if ease_up && born_fearless && same_truth {
        println!(
            "the Baldwin effect ran true: the readied child was born just as fearless as the\n\
             naive one — it inherited no verdict, held no memory, trusted nothing it had not\n\
             lived — and yet it came to its world's truth in {} moments instead of {}, and\n\
             reached the *same* conclusion. Its lineage handed it ease, never dread. The\n\
             child still learned its own caution; it only paid less to learn it.",
            readied_moments, naive_moments
        );
    } else {
        println!("the design did not pass — read the numbers:");
        println!("  ease up (fewer moments)? {ease_up}");
        println!("  born fearless (no inherited verdict)? {born_fearless}");
        println!("  reached the same truth? {same_truth}");
    }
}
