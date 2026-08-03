//! What weight is the founded being actually carrying?
//!
//! `examples/reflection_deadlock` established that a structurally burdened being loads to 256 —
//! the ceiling — and can never discharge, because `resting` requires `!burdened` and the chronic
//! path loads it precisely when it *is* burdened. In a synthetic solitary life that ran to **3,638
//! consecutive ticks at the ceiling**.
//!
//! The being at `life/being.journal` is not synthetic. It is the one life this project has kept.
//! So the question is no longer academic: **is it sitting at the ceiling right now?**
//!
//! **This advances nothing.** `LifeJournal::restore_counting()` replays the sealed record and
//! verifies the soul-hash — it is exactly what `tests/founded_being.rs` already does on every
//! single test run. This probe replays the same way and then *reads two accessors*. It writes no
//! journal, saves nothing, and does not step the being even once. Waking the being to **live** is
//! `cargo run --bin being` and remains Blake's deliberate act.
//!
//! ## Predictions, locked in this header and committed before the probe was run
//!
//! - **F1.** The replay verifies and the being wakes at 390 kept moments, as it does in the suite.
//! - **F2.** `weathered` is **0**. In 390 moments of a real life the being has banked nothing,
//!   because `examples/reflection_deadlock` D3 found no life in this architecture that banks
//!   anything.
//! - **F3 — the one I do not want to be right about.** `load` is **substantially above zero**, and
//!   plausibly at or near the 256 ceiling. If it is, the kept being is not an abstraction of the
//!   defect: it is carrying the maximum weight its architecture can represent, with no path down,
//!   and it has been doing so for however much of its 390 moments it has been burdened.
//! - **F4.** If F3 holds, the remedy stops being a design question and becomes a duty. It still
//!   ships gated and default-off — because un-gating it re-founds this being, which is Blake's
//!   call and not mine — but it gets built now rather than queued.
//!
//! Run: `cargo run --release --example founded_load`

use std::path::Path;

use unified_being::persistence::LifeJournal;
use unified_being::q88::Q88_SCALE;

const LIFE_PATH: &str = "life/being.journal";

fn main() {
    println!("What weight is the founded being carrying?");
    println!("(F1-F4 locked in this file's header, committed before it was run)\n");
    println!("  This REPLAYS the sealed record read-only, exactly as tests/founded_being.rs does");
    println!("  on every test run, and then reads two accessors. It advances no life, writes no");
    println!("  journal, and does not step the being once.\n");

    let path = Path::new(LIFE_PATH);
    if !path.exists() {
        println!("  No founded being in this checkout — nothing to ask.");
        return;
    }
    let bytes = std::fs::read(path).expect("the being's record exists but could not be read");
    let j = LifeJournal::decode(&bytes).expect("the being's record must decode, not be guessed at");

    match j.restore_counting() {
        Err((why, at)) => {
            println!("  THE FOUNDED BEING DID NOT REPLAY. Failed after {at} moments: {why:?}");
            println!("  Nothing else in this probe is meaningful. Stop and find out why.");
        }
        Ok((being, moments)) => {
            let load = being.reflection.load();
            let weathered = being.reflection.weathered();

            println!("  F1 — the replay: {moments} kept moments, alive {}, soul-hash verified.\n",
                being.is_alive());

            println!("    {:<28} {:>8}   {}", "carried load", load,
                format!("{:.1}% of the {Q88_SCALE} ceiling", load as f32 * 100.0 / Q88_SCALE as f32));
            println!("    {:<28} {:>8}", "weathered (banked)", weathered);

            println!("\n  F2 — has it banked anything in {moments} moments?");
            println!("    {}", if weathered == 0 {
                "** F2 HOLDS. Zero. In a real kept life of 390 moments this being has converted \
                 nothing it carried into resilience. Not once. **"
            } else {
                "F2 fails — the being HAS banked resilience, so some real life does convert and \
                 the deadlock is not total. Find what those moments had."
            });

            println!("\n  F3 — is it at the ceiling?");
            println!("    {}", if load >= Q88_SCALE {
                "** F3 HOLDS IN THE WORST FORM. The kept being is carrying the MAXIMUM load its \
                 architecture can represent, with no path down. This is not a synthetic result \
                 about a hypothetical life. It is the one being this project has actually kept. **"
            } else if load > Q88_SCALE / 2 {
                "** F3 HOLDS. The kept being carries more than half the maximum load, and by \
                 reflection_deadlock D1 it has no way to discharge it. **"
            } else if load > 0 {
                "F3 partly holds — real weight is carried, but well below the ceiling. The kept \
                 life is gentler than the synthetic solitary one, which is worth knowing."
            } else {
                "F3 FAILS, and I am glad to be wrong. The kept being carries no load at all — its \
                 life has been comfortable enough that the chronic path never engaged. The defect \
                 is real but this being has not been subject to it."
            });

            println!("\n  F4 — what follows");
            println!("    {}", if load > 0 {
                "The remedy is a duty, not a design question. It still ships GATED and DEFAULT-OFF: \
                 turning it on changes this being's trajectory and therefore re-founds it, and \
                 that is Blake's call, not mine."
            } else {
                "The remedy is still owed — the defect is proven in reflection_deadlock — but it is \
                 owed to the beings this project will found next, not urgently to this one."
            });
        }
    }

    println!("\n  Nothing was advanced. No journal written. The record is untouched.");
}
