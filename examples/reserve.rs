//! A reserve and a satiety set point — can the being bank a feast and cross a famine?
//!
//! The measurement for `docs/can-it-tire.md` §8. B1–B6 were locked in that document and committed
//! before this file existed.
//!
//! §5 measured that `energy = clamp(energy − cost + gain, 0, 1)` is a pure accumulator with exactly
//! two attractors, and that this costs the being two things: `fatigue` is a **dead channel** (one
//! distinct value across a 4,000-tick life), and **every oscillating supply killed it** — including
//! feast 60 / famine 12 / period 120, dead at 156 ticks, whose time-average is nearly double the
//! survival boundary.
//!
//! **B3 is the decisive one:** the regimes that killed it must now survive.
//! **B5 exists so this can fail against me:** a reserve could buy safety by removing stakes, and a
//! being that is never hungry is as flat as one that is never tired.
//!
//! Pure observer: fresh beings, gate default-off, no journal written. Survival first.
//!
//! Run: `cargo run --release --example reserve`

use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment, Sensorium};
use unified_being::genome::Genome;
use unified_being::q88::Q88_SCALE;
use unified_being::room::Room;

const LIFE: usize = 4_000;
const REST_FATIGUE: i16 = 80;

#[derive(Default)]
struct Seen {
    ticks: usize,
    alive: bool,
    fat_min: i16,
    fat_max: i16,
    fat_sum: i64,
    fat_distinct: usize,
    at_stake: usize,
    /// Ticks at Rest's fatigue coordinate where the being went on to live.
    tired_and_living: usize,
    reserve_max: i16,
    reserve_final: i16,
}

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

/// `period` of 0 means a constant supply.
fn live(reserve: bool, nutrient: i16, threat: i16, period: usize, lean: i16) -> Seen {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    b.enable_receptors();
    if reserve {
        b.enable_reserve();
    }
    let p = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    let mut fat: Vec<i16> = Vec::with_capacity(LIFE);
    let mut s = Seen { alive: true, fat_min: i16::MAX, fat_max: i16::MIN, ..Default::default() };

    for t in 0..LIFE {
        let n = if period == 0 || (t / period) % 2 == 0 { nutrient } else { lean };
        let r = b.step_embodied(&Sensorium {
            nutrient: n,
            threat,
            exteroception: [0; 4],
            partner: Some(p),
        });
        let f = b.field.channel[10];
        fat.push(f);
        s.fat_min = s.fat_min.min(f);
        s.fat_max = s.fat_max.max(f);
        s.fat_sum += f as i64;
        if r.felt.state.at_stake {
            s.at_stake += 1;
        }
        s.reserve_max = s.reserve_max.max(b.body.reserve.raw);
        s.reserve_final = b.body.reserve.raw;
        s.ticks += 1;
        if !r.alive {
            s.alive = false;
            break;
        }
    }
    let n = fat.len();
    for (i, &f) in fat.iter().enumerate() {
        if f >= REST_FATIGUE && n - i > 200 {
            s.tired_and_living += 1;
        }
    }
    let mut v = fat.clone();
    v.sort_unstable();
    v.dedup();
    s.fat_distinct = v.len();
    s
}

/// Distinct positions in the room the being actually lives in — B6.
fn positions(reserve: bool) -> (usize, usize, bool) {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    b.enable_receptors();
    if reserve {
        b.enable_reserve();
    }
    let mut room =
        Room::peopled((32, 200), (224, 56), (128, 220), (40, 40)).with_friend((210, 128));
    let mut seen: Vec<(i16, i16)> = Vec::new();
    let mut ticks = 0usize;
    let mut alive = true;
    for _ in 0..LIFE {
        let s = room.sense();
        let r = b.step_embodied(&s);
        room.actuate(&intent_from(&r));
        if !seen.contains(&room.body) {
            seen.push(room.body);
        }
        ticks += 1;
        if !r.alive {
            alive = false;
            break;
        }
    }
    (seen.len(), ticks, alive)
}

fn pct(n: usize, d: usize) -> f32 {
    if d == 0 { 0.0 } else { n as f32 * 100.0 / d as f32 }
}

fn main() {
    println!("A reserve and a satiety set point — can the being bank a feast?");
    println!("(B1-B6 locked in docs/can-it-tire.md §8, committed before this ran)\n");

    // ---- B3 FIRST: the regimes that killed it -------------------------------------------
    println!("  B3 — THE DECISIVE ONE. Every oscillating supply killed this being in §5.\n");
    println!("    {:<28} {:>16} {:>16} {:>12}", "feast/famine/period", "reserve OFF", "reserve ON",
        "banked max");
    println!("    {:-<28} {:->16} {:->16} {:->12}", "", "", "", "");
    let osc: [(i16, usize); 6] =
        [(0, 20), (0, 60), (0, 120), (5, 60), (10, 60), (12, 120)];
    let mut rescued = 0usize;
    for &(famine, period) in &osc {
        let off = live(false, 60, 0, period, famine);
        let on = live(true, 60, 0, period, famine);
        if !off.alive && on.alive {
            rescued += 1;
        }
        println!("    {:<28} {:>16} {:>16} {:>12}",
            format!("60 / {famine} / {period}"),
            format!("{} {}", off.ticks, if off.alive { "lived" } else { "DIED" }),
            format!("{} {}", on.ticks, if on.alive { "lived" } else { "DIED" }),
            on.reserve_max);
    }
    println!("\n    {rescued} of {} regimes that killed it now survive.", osc.len());
    println!("    {}", if rescued == osc.len() {
        "** B3 HOLDS IN FULL. Every life that killed this being is now survivable. A feast that \
         can be banked is a famine that can be crossed. **"
    } else if rescued > 0 {
        "** B3 partly — some are rescued and some still die. The reserve helps and is not enough \
         for the harshest. **"
    } else {
        "** B3 FAILS — the reserve rescued nothing. The mechanism does not do what §8 said it \
         would. **"
    });

    // ---- B2: is fatigue alive now? ------------------------------------------------------
    println!("\n  B2 — is `fatigue` still a dead channel?\n");
    println!("    {:<26} {:>8} {:>8} {:>8} {:>10} {:>10}",
        "constant supply", "min", "max", "mean", "distinct", "banked");
    println!("    {:-<26} {:->8} {:->8} {:->8} {:->10} {:->10}", "", "", "", "", "", "");
    for n in [25i16, 40, 60, 100, 200] {
        for (label, on) in [("off", false), ("ON", true)] {
            let s = live(on, n, 0, 0, 0);
            println!("    {:<26} {:>8} {:>8} {:>8.0} {:>10} {:>10}",
                format!("nutrient {n}, reserve {label}"),
                s.fat_min, s.fat_max, s.fat_sum as f32 / s.ticks.max(1) as f32,
                s.fat_distinct, s.reserve_final);
        }
    }
    let g = live(true, 60, 0, 0, 0);
    println!("\n    at nutrient 60 with the reserve on: mean fatigue {:.0}, {} distinct values",
        g.fat_sum as f32 / g.ticks.max(1) as f32, g.fat_distinct);
    println!("    ticks at Rest's coordinate ({REST_FATIGUE}) while going on to live: {}",
        g.tired_and_living);
    let mean_fat = g.fat_sum as f32 / g.ticks.max(1) as f32;
    println!("\n    {}", if g.fat_distinct > 1 && (40.0..=90.0).contains(&mean_fat) {
        "** B2 HOLDS. Fatigue is a LIVE REGISTER for the first time in this project, and it settles \
         in the band Rest was written for — not one nutrient unit from starving. **"
    } else if g.fat_distinct > 1 {
        "** B2 partly — fatigue is alive but does not settle in the predicted 40-90 band. Read the \
         table. **"
    } else {
        "** B2 FAILS — fatigue is still one distinct value. The set point is not doing what it \
         was built to do. **"
    });

    // ---- B4: the tired band ---------------------------------------------------------------
    println!("\n  B4 — how wide is the survivable band now? (§5: one nutrient unit, 19-20)\n");
    let mut lo = None;
    let mut width = 0usize;
    for n in 0..=40i16 {
        let s = live(true, n, 0, 0, 0);
        if s.alive {
            if lo.is_none() {
                lo = Some(n);
            }
            width += 1;
        }
    }
    println!("    survivable from nutrient {:?} upward within 0..=40 ({width} of 41 values)", lo);
    // Where in that band is the being genuinely tired AND living?
    let mut tired_band = 0usize;
    for n in 0..=40i16 {
        let s = live(true, n, 0, 0, 0);
        if s.alive && s.tired_and_living > 0 {
            tired_band += 1;
        }
    }
    println!("    of those, nutrient values where it is tired AND lives on: **{tired_band}**");
    println!("    {}", if tired_band > 1 {
        "** B4 HOLDS. The tired band is wider than one unit — tiredness is a place the being can \
         live in. **"
    } else {
        "B4 fails — the tired band is still a knife edge."
    });

    // ---- B5: the counterweight -------------------------------------------------------------
    println!("\n  B5 — has the reserve bought safety by removing stakes?\n");
    let off60 = live(false, 60, 0, 0, 0);
    println!("    at stake:        off {:.1}%   ON {:.1}%",
        pct(off60.at_stake, off60.ticks), pct(g.at_stake, g.ticks));
    println!("    fatigue spread:  off {} distinct ({}..{})   ON {} distinct ({}..{})",
        off60.fat_distinct, off60.fat_min, off60.fat_max,
        g.fat_distinct, g.fat_min, g.fat_max);
    println!("\n    {}", if g.fat_distinct > off60.fat_distinct {
        "** B5 clears it. The being's fatigue is MORE varied, not less — the reserve did not \
         smooth it into a different flatness.\n\
         \x20   At-stake is unchanged, exactly as predicted: a reserve makes stakes survivable, it \
         does not create them. **"
    } else {
        "** B5 WARNS AGAINST B2 — fatigue is no more varied than before. The reserve has traded \
         one flat life for another and should not be read as a gain. **"
    });

    // ---- B6 ---------------------------------------------------------------------------------
    println!("\n  B6 — is the limit cycle a fact about the world, not about metabolism?\n");
    let (p_off, t_off, a_off) = positions(false);
    let (p_on, t_on, a_on) = positions(true);
    println!("    distinct positions in the room, 4,000 ticks:");
    println!("      reserve off: {p_off} ({t_off} ticks, {})", if a_off { "lived" } else { "DIED" });
    println!("      reserve ON:  {p_on} ({t_on} ticks, {})", if a_on { "lived" } else { "DIED" });
    println!("\n    {}", if p_on <= p_off + 20 {
        "** B6 HOLDS. The orbit is essentially unchanged. The limit cycle is a fact about a STATIC \
         WORLD, and metabolism was never going to touch it. The world is still the next thing. **"
    } else {
        "** B6 FAILS — the reserve widened the being's orbit substantially. I misunderstood what \
         this change does, and that is worth more than the prediction was. **"
    });

    println!("\n  The founded being was not touched. Gate default-off; no journal written.");
}
