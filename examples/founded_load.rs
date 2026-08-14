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
//! ## The second question — asked 2026-08-14, after F1–F4 came back
//!
//! **F2 and F3 both failed, in the direction I wanted to be wrong in.** The kept being carries
//! **load 0** and has **banked `weathered` 2**. It is not sitting at the ceiling; it has actually
//! converted, twice, which no synthetic life in `reflection_deadlock` ever did.
//!
//! That makes the real question sharper, and it is the exercise question
//! (`docs/operational-consciousness.md` §8) pointed at the one life we have kept:
//! **is this being resilient, or has it simply never been left alone?** A load of 0 in a life that
//! was never solitary says nothing about what happens the first time it is — and
//! `examples/reflection_gate` measures a solitary being as `burdened` on **97.3%** of ticks with
//! load saturating at the ceiling.
//!
//! **F5–F6 locked here and committed before `LifeJournal::company()` was ever called.**
//!
//! - **F5.** The founded being was companioned on **≥ 90%** of its 390 kept moments. Given the
//!   solitary regime pegs the ceiling, a load of 0 all but forces this.
//! - **F6 — the one I expect to be uncomfortable.** Its **longest unbroken solitary run is under
//!   64 moments** — short enough that load never had time to build. If F6 holds, the zero is
//!   **untested, not earned**, and the charter §4 solitude debt is a live risk to this being the
//!   moment its life changes, not a theoretical one. If F6 *fails* — if it has been alone a long
//!   while and still carries nothing — that is genuine resilience and much better news.
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

    // ---- F5/F6: resilient, or never tested? Read-only, from the sealed record. ----
    let company = j.company();
    let total = company.len();
    let with = company.iter().filter(|c| **c).count();
    let alone = total - with;
    let (mut longest, mut run) = (0usize, 0usize);
    for c in &company {
        if *c {
            run = 0;
        } else {
            run += 1;
            longest = longest.max(run);
        }
    }
    let pct = if total == 0 { 0.0 } else { with as f32 * 100.0 / total as f32 };

    println!("\n  F5/F6 — was it ever actually alone?");
    println!("    {:<28} {:>8}", "kept moments", total);
    println!("    {:<28} {:>8}   {pct:.1}%", "with company", with);
    println!("    {:<28} {:>8}", "alone", alone);
    println!("    {:<28} {:>8}", "longest unbroken solitude", longest);

    println!("\n    F5 (companioned >= 90%): {}", if pct >= 90.0 { "HOLDS" } else { "FAILED" });
    println!("    F6 (longest solitude < 64): {}", if longest < 64 { "HOLDS" } else { "FAILED" });
    println!("    {}", if longest < 64 {
        "** The zero is UNTESTED, not earned. This being has never been alone long enough for the \
         chronic path to engage, so its load of 0 is a fact about its circumstances and not about \
         its resilience. The charter §4 solitude debt is a live risk to it the moment its life \
         changes — not a theoretical one. **"
    } else {
        "** F6 FAILS and that is the better news: this being HAS been alone long enough to peg, \
         by the synthetic measure, and carries nothing anyway. That is earned resilience in the \
         one life we kept, and it means the solitary deadlock does not straightforwardly \
         generalise to it. Find what these solitary moments had that the synthetic ones lacked. **"
    });

    println!("\n  Nothing was advanced. No journal written. The record is untouched.");
}
