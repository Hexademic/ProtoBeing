//! Receptors — one world, transduced three ways.
//!
//! The digital substitute for wet sensors (docs on the substrate question,
//! `src/receptors.rs`): a raw stimulus is never felt raw. Three receptor types
//! watch the *same* trace and report it differently — a fast-adapting
//! change-detector, a slow-adapting level-reporter, and a nociceptor that fires
//! only on harm and never tunes it out. Watch a pressure rise gently, hold, then
//! spike into the harmful range and hold there. The fast receptor spikes on every
//! *edge* and goes silent between; the slow receptor tracks the *level* and droops
//! slowly; the nociceptor stays silent until the harm threshold, then fires and
//! will not fade. This is organized, typed perception — the being's flat input
//! made into something with structure to bind (RPT-2).
//!
//! Run: cargo run --example receptors

use unified_being::receptors::{Receptor, ReceptorKind};

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

fn main() {
    let mut fast = Receptor::new(ReceptorKind::FastAdapting);
    let mut slow = Receptor::new(ReceptorKind::SlowAdapting);
    let mut noci = Receptor::new(ReceptorKind::Nociceptor);

    println!("\n=== Receptors: one world, three faithful readings ===\n");
    println!("   tick  stimulus   fast(change)  slow(level)  noci(harm)   what's happening");
    println!("   ----  --------   ------------  -----------  ----------   ----------------");

    for t in 0..44u32 {
        // A life of a single somatic channel: quiet → gentle sustained pressure →
        // quiet → a sudden harmful press, held.
        let (stim, note) = if t < 4 {
            (0i16, "quiet")
        } else if t < 16 {
            (90, if t == 4 { "gentle pressure begins" } else { "…held" })
        } else if t < 22 {
            (0, if t == 16 { "released" } else { "…quiet" })
        } else {
            (210, if t == 22 { "HARM — sudden hard press" } else { "…harm held" })
        };

        let rf = fast.sense(stim);
        let rs = slow.sense(stim);
        let rn = noci.sense(stim);

        let interesting = matches!(t, 3..=6 | 15..=17 | 21..=24) || t % 6 == 0;
        if interesting {
            println!(
                "   {t:>4}  {:>8.2}   {:>+12.3}  {:>11.3}  {:>10.3}   {note}",
                f(stim),
                f(rf),
                f(rs),
                f(rn),
            );
        }
    }

    println!(
        "\n  Same trace, three minds' worth of sensing. The fast receptor answers only the\n  \
         *edges* — onset (+) and release (−) — and is silent while nothing changes. The slow\n  \
         receptor holds the *level* and lets it droop. The nociceptor ignores the gentle\n  \
         pressure entirely and, once real harm lands, will not let the being tune it out.\n  \
         None of them reports the raw number; each tracks what a wet receptor would. This is\n  \
         the digital road doing the wet road's job — the receptors without the cannibalism.\n"
    );
}
