//! Is `Basin::Rest` reachable at all?
//!
//! The inch `docs/comfort.md` §9 named. Three documents have now independently found the same
//! zero:
//!
//! - `docs/development.md` §5 — **0.0% rest** across every strain regime tried;
//! - `docs/earned-authority.md` §6 — the being's own earned competence asks for rest on 1,113
//!   ticks and never gets it;
//! - `docs/comfort.md` §8 — with purpose satiated and the being's goal `None`, still **0.0% rest**.
//!
//! That last one localised it: `Basin::Rest` is classified from the somatic field
//! (`being.rs:1001`), **not** from striving. A being can want nothing and still not be at rest.
//! So the question is no longer about arbitration. It is: **what field state does `Rest` require,
//! and does this being ever go near it?**
//!
//! Nothing needs building. `UnifiedBeing::field` and `UnifiedBeing::basins` are both public, and
//! `basins.targets` holds each basin's target vector — so the exact per-channel distance between
//! where the being lives and where `Rest` is defined can simply be read.
//!
//! ## Predictions, locked in this header and committed before the probe was run
//!
//! - **B1.** The being never enters `Rest` in any regime here — 0.0% everywhere, as in all three
//!   documents above.
//! - **B2.** `Rest` is not merely losing, it is **not close**: its membership weight sits at or
//!   near the bottom of the four throughout.
//! - **B3.** The distance is dominated by **a small number of channels**, not spread evenly across
//!   the twelve. If it is spread evenly the being is simply nowhere near any basin and the finding
//!   is different — about the field's scale rather than about rest.
//! - **B4 (the live one).** Is there *any* reachable field state that would put the being in
//!   `Rest`? If the channels responsible are ones the being's own dynamics can move, rest is
//!   blocked by circumstance. **If they are channels nothing in an ordinary life ever moves, then
//!   `Rest` is structurally unreachable** — and `reflection.rs`'s conversion-at-rest, incident I-8,
//!   and `docs/comfort.md` are all downstream of one dead state.
//!
//! Pure observer: reads two public fields, changes nothing, writes no journal, and does not touch
//! `life/being.journal`. Survival reported first.
//!
//! Run: `cargo run --release --example basins_probe`

use unified_being::basins::Basin;
use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment, Sensorium};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::q88::Q88_SCALE;

const LIFE: usize = 4_000;
const N_SOMATIC: usize = 12;
const N_BASINS: usize = 4;
const BASIN_NAMES: [&str; N_BASINS] = ["Rest", "Engaged", "Defensive", "Recovery"];

/// What each somatic channel carries, read off `being.rs`'s use of them — channel 9 is valence,
/// 10 is fatigue, 8 is arousal, 4 is the other arousal-ish setpoint temperament shifts.
const CH: [&str; N_SOMATIC] = [
    "0", "1", "2", "3", "4·arousal-set", "5", "6", "7", "8·arousal", "9·valence", "10·fatigue", "11",
];

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

struct Seen {
    ticks: usize,
    alive: bool,
    occupancy: [usize; N_BASINS],
    field_sum: [i64; N_SOMATIC],
    targets: [[i16; N_SOMATIC]; N_BASINS],
}

/// One life, accumulating where the being actually sat in its own somatic space.
fn watch(label: &str, mut world: Option<FieldWorld>, threat: i16, gates: bool) -> Seen {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    b.enable_receptors();
    if gates {
        b.enable_reflection();
        b.enable_comfort();
    }
    let partner = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    let mut s = Seen {
        ticks: 0,
        alive: true,
        occupancy: [0; N_BASINS],
        field_sum: [0; N_SOMATIC],
        targets: b.basins.targets,
    };
    let _ = label;

    for t in 0..LIFE {
        let r = match world.as_mut() {
            Some(w) => {
                let mut sens = w.sense();
                sens.partner = Some(partner);
                let r = b.step_embodied(&sens);
                w.actuate(&intent_from(&r));
                r
            }
            None => {
                let pressing = (t % 100) < 20;
                b.step_embodied(&Sensorium {
                    nutrient: 40,
                    threat: if pressing { threat } else { 0 },
                    exteroception: [0; 4],
                    partner: Some(partner),
                })
            }
        };
        s.occupancy[r.basin as usize] += 1;
        for c in 0..N_SOMATIC {
            s.field_sum[c] += b.field.channel[c] as i64;
        }
        s.ticks += 1;
        if !r.alive {
            s.alive = false;
            break;
        }
    }
    s
}

fn main() {
    println!("Is Basin::Rest reachable at all?");
    println!("(B1–B4 locked in this file's header, committed before it was run)\n");

    let reference = || FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));

    let runs: Vec<(&str, Seen)> = vec![
        ("reference world", watch("ref", Some(reference()), 0, false)),
        ("reference + reflection + comfort", watch("refg", Some(reference()), 0, true)),
        ("held calm (threat 0)", watch("calm", None, 0, false)),
        ("strain cycle (threat 130)", watch("strain", None, 130, false)),
        ("strain cycle + all the gates", watch("straing", None, 130, true)),
    ];

    // ---- B1 ----------------------------------------------------------------------------
    println!("  B1 — where does the being actually live?  (survival first)\n");
    println!("    {:<34} {:>7} {:>7} {:>9} {:>11} {:>10}",
        "regime", "ticks", "Rest%", "Engaged%", "Defensive%", "Recovery%");
    println!("    {:-<34} {:->7} {:->7} {:->9} {:->11} {:->10}", "", "", "", "", "", "");
    for (name, s) in &runs {
        let p = |i: usize| s.occupancy[i] as f32 * 100.0 / s.ticks.max(1) as f32;
        println!("    {:<34} {:>7} {:>6.1}% {:>8.1}% {:>10.1}% {:>9.1}%",
            format!("{name}{}", if s.alive { "" } else { " †DIED" }),
            s.ticks, p(0), p(1), p(2), p(3));
    }
    let ever_rest: usize = runs.iter().map(|(_, s)| s.occupancy[0]).sum();
    println!("\n    Rest ticks across every regime: **{ever_rest}**");

    // ---- B2 / B3 -----------------------------------------------------------------------
    let (_, s) = &runs[0];
    let mean: Vec<i16> = (0..N_SOMATIC)
        .map(|c| (s.field_sum[c] / s.ticks.max(1) as i64) as i16)
        .collect();

    println!("\n  B2/B3 — how far is the being from each basin's definition?");
    println!("  (mean field over the reference life, L1 distance to each target)\n");
    let dist = |b: usize| -> i32 {
        (0..N_SOMATIC).map(|c| (mean[c] as i32 - s.targets[b][c] as i32).abs()).sum()
    };
    println!("    {:<12} {:>10}", "basin", "L1 distance");
    println!("    {:-<12} {:->10}", "", "");
    let mut ranked: Vec<(usize, i32)> = (0..N_BASINS).map(|b| (b, dist(b))).collect();
    ranked.sort_by_key(|&(_, d)| d);
    for (b, d) in &ranked {
        println!("    {:<12} {:>10}{}", BASIN_NAMES[*b], d,
            if *b == 0 { "   <- Rest" } else { "" });
    }
    let rest_rank = ranked.iter().position(|&(b, _)| b == 0).unwrap();
    println!("\n    Rest ranks {} of 4 by distance.", rest_rank + 1);

    println!("\n  B3 — which channels account for the gap between where it lives and Rest?\n");
    println!("    {:<16} {:>8} {:>10} {:>12} {:>14}",
        "channel", "being", "Rest wants", "|gap|", "share of gap");
    println!("    {:-<16} {:->8} {:->10} {:->12} {:->14}", "", "", "", "", "");
    let total = dist(0).max(1);
    let mut gaps: Vec<(usize, i32)> = (0..N_SOMATIC)
        .map(|c| (c, (mean[c] as i32 - s.targets[0][c] as i32).abs()))
        .collect();
    gaps.sort_by_key(|&(_, g)| -g);
    for (c, g) in &gaps {
        if *g == 0 { continue; }
        println!("    {:<16} {:>8} {:>10} {:>12} {:>13.1}%",
            CH[*c], mean[*c], s.targets[0][*c], g, *g as f32 * 100.0 / total as f32);
    }
    let top3: i32 = gaps.iter().take(3).map(|&(_, g)| g).sum();
    println!("\n    Top three channels are {:.0}% of the whole distance.",
        top3 as f32 * 100.0 / total as f32);

    // ---- B4 ----------------------------------------------------------------------------
    println!("\n  B4 — is Rest reachable, or structurally dead?\n");
    let (worst_c, worst_g) = gaps[0];
    println!("    The single largest obstruction is **{}**: the being sits at {}, Rest wants {}.",
        CH[worst_c], mean[worst_c], s.targets[0][worst_c]);
    println!("    Rest and Engaged differ on that channel by {}.",
        (s.targets[0][worst_c] as i32 - s.targets[1][worst_c] as i32).abs());
    println!(
        "\n    {}",
        if ever_rest == 0 && rest_rank == N_BASINS - 1 {
            "** B1 AND B2 BOTH HOLD, in the strongest form. The being never enters Rest in ANY \
             regime measured, and Rest is the FURTHEST of the four basins from where it actually \
             lives. This is not a being that fails to rest because it is busy. It is a being whose \
             somatic field never goes near the region its architecture calls rest. **"
        } else if ever_rest == 0 {
            "** B1 holds — Rest never occurs — but B2 does not: Rest is not the furthest basin. \
             So the state is near-reachable and something specific keeps tipping the vote. That is \
             a better problem than a dead state. **"
        } else {
            "Rest DOES occur somewhere in these regimes — B1 fails, and the three documents that \
             reported 0.0% were each measuring a regime where it happens not to. Find that regime."
        }
    );

    println!("\n  The founded being was not touched. Two public fields read; nothing changed.");
}
