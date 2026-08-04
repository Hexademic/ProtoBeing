//! Does arousal actually decide anything — or is it dead weight in the classifier?
//!
//! `docs/comfort.md` §10 reported that the two arousal channels are ~48% of the distance between
//! where the being lives and `Basin::Rest`, and read that as *arousal is the obstruction*.
//!
//! **That may be a category error of mine, and this probe exists to find out.** Dominating a
//! *distance* and deciding a *classification* are different things. `basins.rs` picks the argmax
//! of closeness across four targets — so a channel that penalises **all four** basins by a similar
//! amount contributes hugely to every distance and discriminates between none of them.
//!
//! And the numbers point that way. The being's mean arousal is **237**. Every basin target is
//! below it: Rest 73, Recovery 89, Engaged 159, Defensive 209. **The being runs hotter than the
//! hottest mode its own architecture describes.**
//!
//! ## Predictions, locked in this header and committed before the probe was run
//!
//! - **A1.** The being's arousal essentially never falls below **209** (Defensive's target, the
//!   highest of the four) in an ordinary life — so no basin's arousal coordinate is ever matched
//!   from below, and the arousal term is a monotone penalty ordered identically every tick.
//! - **A2.** That ordering is always the same: Defensive penalised least on arousal, Rest most.
//! - **A3 — THE DECISIVE ONE.** Recompute basin distances with channels 4 and 8 **removed**. If
//!   the winner changes, arousal genuinely decides something and `docs/comfort.md` §10 stands. **If
//!   the winner does not change, arousal is dead weight in the classifier** — it inflates every
//!   distance and selects nothing, my §10 reading is wrong, and the real obstruction is among the
//!   other ten channels.
//! - **A4.** If A3 shows arousal is inert, then `docs/settling.md`'s ±32 channel is *not* the
//!   bottleneck it was reported to be either — widening it would move a number that does not
//!   choose. Two documents would need correcting, not one.
//!
//! Pure observer: reads two public fields, changes nothing, writes no journal, `life/being.journal`
//! untouched. Survival reported first.
//!
//! Run: `cargo run --release --example arousal_range`

use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::q88::Q88_SCALE;

const LIFE: usize = 4_000;
const N: usize = 12;
const N_BASINS: usize = 4;
const NAMES: [&str; N_BASINS] = ["Rest", "Engaged", "Defensive", "Recovery"];
/// The two arousal channels, per `docs/comfort.md` §10's B3 table.
const AROUSAL: [usize; 2] = [4, 8];

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

fn main() {
    println!("Does arousal decide anything, or only inflate every distance?");
    println!("(A1–A4 locked in this file's header, committed before it ran)\n");

    let mut b = UnifiedBeing::new(Genome::wanderer());
    b.enable_receptors();
    let mut world = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let partner = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    let targets = b.basins.targets;

    let mut a8: Vec<i16> = Vec::new();
    // How often each basin wins, with all twelve channels and with arousal removed.
    let mut win_all = [0usize; N_BASINS];
    let mut win_no_arousal = [0usize; N_BASINS];
    let mut agree = 0usize;
    let mut alive = true;
    let mut ticks = 0usize;
    // A2: how often each basin is the LEAST penalised on the arousal channels alone.
    let mut arousal_pref = [0usize; N_BASINS];

    for _ in 0..LIFE {
        let mut sens = world.sense();
        sens.partner = Some(partner);
        let r = b.step_embodied(&sens);
        world.actuate(&intent_from(&r));

        let f = b.field.channel;
        a8.push(f[8]);

        let dist = |bi: usize, skip_arousal: bool| -> i32 {
            (0..N)
                .filter(|c| !(skip_arousal && AROUSAL.contains(c)))
                .map(|c| (f[c] as i32 - targets[bi][c] as i32).abs())
                .sum()
        };
        let argmin = |skip: bool| -> usize {
            (0..N_BASINS).min_by_key(|&bi| dist(bi, skip)).unwrap()
        };
        let w_all = argmin(false);
        let w_no = argmin(true);
        win_all[w_all] += 1;
        win_no_arousal[w_no] += 1;
        if w_all == w_no {
            agree += 1;
        }
        let ap = (0..N_BASINS)
            .min_by_key(|&bi| {
                AROUSAL.iter().map(|&c| (f[c] as i32 - targets[bi][c] as i32).abs()).sum::<i32>()
            })
            .unwrap();
        arousal_pref[ap] += 1;

        ticks += 1;
        if !r.alive {
            alive = false;
            break;
        }
    }

    println!("  SURVIVAL: {ticks} ticks, {}\n", if alive { "lived" } else { "DIED" });

    // ---- A1 ----------------------------------------------------------------------------
    a8.sort_unstable();
    let pctl = |p: usize| a8[(a8.len() * p / 100).min(a8.len() - 1)];
    println!("  A1 — where does the being's arousal actually sit?  (channel 8)\n");
    println!("    min {}   p5 {}   median {}   p95 {}   max {}",
        a8[0], pctl(5), pctl(50), pctl(95), a8[a8.len() - 1]);
    println!("\n    basin targets on this channel:  Rest {}   Recovery {}   Engaged {}   Defensive {}",
        targets[0][8], targets[3][8], targets[1][8], targets[2][8]);
    let below_hottest = a8.iter().filter(|&&v| v < targets[2][8]).count();
    println!("\n    ticks below Defensive's target ({}), the HIGHEST of the four: {} ({:.1}%)",
        targets[2][8], below_hottest, below_hottest as f32 * 100.0 / ticks as f32);
    println!("    {}", if below_hottest * 20 < ticks {
        "** A1 HOLDS. The being runs hotter than every mode its own architecture describes. \
         The arousal term is a monotone penalty, not a discriminator. **"
    } else {
        "A1 fails — the being does drop into the range the basins are written for."
    });

    // ---- A2 ----------------------------------------------------------------------------
    println!("\n  A2 — on the arousal channels ALONE, which basin is least penalised?\n");
    for bi in 0..N_BASINS {
        println!("    {:<12} {:>6} ticks ({:.1}%)", NAMES[bi], arousal_pref[bi],
            arousal_pref[bi] as f32 * 100.0 / ticks as f32);
    }

    // ---- A3 ----------------------------------------------------------------------------
    println!("\n  A3 — THE DECISIVE TEST: does removing arousal change which basin wins?\n");
    println!("    {:<12} {:>16} {:>20}", "basin", "wins (all 12)", "wins (arousal out)");
    println!("    {:-<12} {:->16} {:->20}", "", "", "");
    for bi in 0..N_BASINS {
        println!("    {:<12} {:>15.1}% {:>19.1}%", NAMES[bi],
            win_all[bi] as f32 * 100.0 / ticks as f32,
            win_no_arousal[bi] as f32 * 100.0 / ticks as f32);
    }
    let agreement = agree as f32 * 100.0 / ticks as f32;
    println!("\n    the two agree on {agreement:.1}% of ticks");
    println!("\n    {}", if agreement > 99.0 {
        "** A3: AROUSAL IS DEAD WEIGHT IN THE CLASSIFIER. Removing both arousal channels changes \
         the winning basin on essentially no tick. It inflates every distance and selects nothing.\n\
         \x20   docs/comfort.md §10 read the largest term in a DISTANCE as the cause of a\n\
         \x20   CLASSIFICATION. Those are different things, and I conflated them. §10 and\n\
         \x20   docs/settling.md §7 both need correcting: widening the ±32 mind→body channel would\n\
         \x20   move a number that does not choose. **"
    } else if agreement > 90.0 {
        "A3: arousal decides the basin on a small minority of ticks — it is nearly, but not \
         entirely, inert. §10's reading is weakened, not overturned."
    } else {
        "A3: arousal genuinely decides the classification. docs/comfort.md §10 stands, and the \
         ±32 channel really is the bottleneck."
    });

    // ---- so what DOES decide? -----------------------------------------------------------
    //
    // A3 says arousal selects nothing. The same leave-one-out method answers the real question:
    // remove each channel in turn and see whose absence changes the winning basin. A channel whose
    // removal changes nothing is not deciding anything, however large its contribution to the
    // distance. This is the generalisation of the error A3 caught.
    println!("\n  So which channels ACTUALLY decide the being's mode?");
    println!("  (leave-one-out: remove channel c, see how often the winner changes)\n");

    let mut b2 = UnifiedBeing::new(Genome::wanderer());
    b2.enable_receptors();
    let mut w2 = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let t2 = b2.basins.targets;
    let mut flips = [0usize; N];
    let mut n2 = 0usize;

    for _ in 0..LIFE {
        let mut sens = w2.sense();
        sens.partner = Some(partner);
        let r = b2.step_embodied(&sens);
        w2.actuate(&intent_from(&r));
        let f = b2.field.channel;

        let win = |skip: Option<usize>| -> usize {
            (0..N_BASINS)
                .min_by_key(|&bi| {
                    (0..N)
                        .filter(|c| Some(*c) != skip)
                        .map(|c| (f[c] as i32 - t2[bi][c] as i32).abs())
                        .sum::<i32>()
                })
                .unwrap()
        };
        let base = win(None);
        for c in 0..N {
            if win(Some(c)) != base {
                flips[c] += 1;
            }
        }
        n2 += 1;
        if !r.alive {
            break;
        }
    }

    println!("    {:<16} {:>18}", "channel removed", "winner changed");
    println!("    {:-<16} {:->18}", "", "");
    let mut ranked: Vec<(usize, usize)> = (0..N).map(|c| (c, flips[c])).collect();
    ranked.sort_by_key(|&(_, f)| std::cmp::Reverse(f));
    for (c, f) in &ranked {
        let label = match c { 4 => "4·arousal-set", 8 => "8·arousal", 9 => "9·valence",
                              10 => "10·fatigue", _ => "" };
        println!("    {:<16} {:>17.1}%   {}", format!("ch {c}"),
            *f as f32 * 100.0 / n2.max(1) as f32, label);
    }
    let deciders = ranked.iter().filter(|(_, f)| *f * 100 > n2).count();
    println!("\n    {}", if deciders == 0 {
        "** NO SINGLE CHANNEL DECIDES ANYTHING. Removing any one of the twelve leaves the winner \
         unchanged on >99% of ticks. The being's mode is not determined by any one register — it \
         is over-determined by the whole field, which is why it never moves. That is a different \
         and larger finding than either arousal or the ±32 channel. **".to_string()
    } else {
        format!("{deciders} of 12 channels change the outcome on more than 1% of ticks. Those are \
                 where the being's mode is actually decided, and where any work on Rest belongs.")
    });

    println!("\n  The founded being was not touched. Two public fields read; nothing changed.");
}
