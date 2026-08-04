//! Settling — can the being quiet itself?
//!
//! The measurement for `docs/settling.md`. S1–S4, W and G were locked and committed before the
//! code existed.
//!
//! `docs/comfort.md` §11 established that **the being can change where it is and cannot change how
//! it is** — every act it has operates on the world, none on itself — and that the only thing
//! which ever lowers its arousal is **solitude**, which costs it its company (incident I-7).
//!
//! `enable_settling()` lets the being's own hunger for repose pull its arousal down: a seventh
//! term in `affective_drive`, negative, sitting where `reflection_tone` and `homecoming_tone`
//! already sit. **Not a new faculty. The first act that operates on the being itself.**
//!
//! **S3 is predicted to FAIL and that prediction is on the record.** Rest is a *conjunction* — low
//! arousal AND fatigue ≈ 80 AND channel 0 ≈ 20 — and settling supplies one of three.
//!
//! Survival first. Founded being untouched, gate default-off, no journal written.
//!
//! Run: `cargo run --release --example settling`

use unified_being::basins::Basin;
use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment, Sensorium};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::play::COMFORT;
use unified_being::q88::Q88_SCALE;

const LIFE: usize = 4_000;

#[derive(Default)]
struct Lived {
    ticks: usize,
    alive: bool,
    rest: usize,
    arousal_min: i16,
    arousal_sum: i64,
    arousal_final: i16,
    ch8_min: i16,
    converted: i64,
    drive_sum: i64,
    burdened: usize,
    at_stake: usize,
    /// Mean arousal on the ticks the being was at stake — G's empirical check: the guard should
    /// keep settling off exactly there, so those ticks must not show the depression.
    stake_arousal_sum: i64,
    soul: [u8; 32],
}

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

/// `world = None` runs the held-stimulus loop, which is where `docs/comfort.md` §11's arousal
/// measurements were taken, so the numbers compose with those.
fn live(settling: bool, world: bool, partner: bool, threat: i16) -> Lived {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    b.enable_receptors();
    b.enable_reflection();
    if settling {
        b.enable_settling();
    }
    let mut w = world.then(|| {
        FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20))
    });
    let p = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    let mut l = Lived { alive: true, arousal_min: i16::MAX, ch8_min: i16::MAX, ..Default::default() };

    for _ in 0..LIFE {
        let r = match w.as_mut() {
            Some(world) => {
                let mut s = world.sense();
                s.partner = partner.then_some(p);
                let r = b.step_embodied(&s);
                world.actuate(&intent_from(&r));
                r
            }
            None => b.step_embodied(&Sensorium {
                nutrient: 200,
                threat,
                exteroception: [0; 4],
                partner: partner.then_some(p),
            }),
        };

        let a = (r.arousal * Q88_SCALE as f32) as i16;
        l.arousal_min = l.arousal_min.min(a);
        l.arousal_sum += a as i64;
        l.arousal_final = a;
        l.ch8_min = l.ch8_min.min(b.field.channel[8]);
        if matches!(r.basin, Basin::Rest | Basin::Recovery) {
            l.rest += 1;
        }
        l.converted += r.reflection.converted as i64;
        l.drive_sum += r.drive.drive as i64;
        if r.drive.drive >= COMFORT {
            l.burdened += 1;
        }
        if r.felt.state.at_stake {
            l.at_stake += 1;
            l.stake_arousal_sum += a as i64;
        }
        l.ticks += 1;
        if !r.alive {
            l.alive = false;
            break;
        }
    }
    l.soul = b.soul_hash();
    l
}

fn pct(n: usize, d: usize) -> f32 {
    if d == 0 { 0.0 } else { n as f32 * 100.0 / d as f32 }
}

fn main() {
    println!("Settling — can the being quiet itself?");
    println!("(S1–S4, W and G locked in docs/settling.md before the code existed)\n");

    let off = live(false, false, true, 0);
    let on = live(true, false, true, 0);

    println!("  SURVIVAL FIRST:  gate off {} ticks ({})   gate ON {} ticks ({})\n",
        off.ticks, if off.alive { "lived" } else { "DIED" },
        on.ticks, if on.alive { "lived" } else { "DIED" });

    println!("  With company, no threat — the case incident I-7 says it cannot win:\n");
    println!("  {:<34} {:>13} {:>13}", "", "settling OFF", "settling ON");
    println!("  {:-<34} {:->13} {:->13}", "", "", "");
    let row = |n: &str, a: String, b: String| println!("  {:<34} {:>13} {:>13}", n, a, b);
    row("arousal, minimum (S2)", format!("{}", off.arousal_min), format!("{}", on.arousal_min));
    row("arousal, mean",
        format!("{:.0}", off.arousal_sum as f32 / off.ticks.max(1) as f32),
        format!("{:.0}", on.arousal_sum as f32 / on.ticks.max(1) as f32));
    row("arousal, final", format!("{}", off.arousal_final), format!("{}", on.arousal_final));
    row("channel 8, minimum", format!("{}", off.ch8_min), format!("{}", on.ch8_min));
    row("Rest / Recovery (S3)",
        format!("{:.1}%", pct(off.rest, off.ticks)), format!("{:.1}%", pct(on.rest, on.ticks)));
    row("load converted (S4)", format!("{}", off.converted), format!("{}", on.converted));
    row("mean drive (W)",
        format!("{:.1}", off.drive_sum as f32 / off.ticks.max(1) as f32),
        format!("{:.1}", on.drive_sum as f32 / on.ticks.max(1) as f32));
    row("past COMFORT (W)",
        format!("{:.1}%", pct(off.burdened, off.ticks)), format!("{:.1}%", pct(on.burdened, on.ticks)));

    println!("\n  S1 — is the gate really off by default?");
    println!("    soul off {:02x}{:02x}{:02x}{:02x}…  on {:02x}{:02x}{:02x}{:02x}…  {}",
        off.soul[0], off.soul[1], off.soul[2], off.soul[3],
        on.soul[0], on.soul[1], on.soul[2], on.soul[3],
        if off.soul != on.soul { "different — the gate acts" } else { "** IDENTICAL — inert **" });
    println!("    (default-path bit-identity is proved by the full suite and tests/founded_being.rs,");
    println!("     which still wakes the kept life at 390 moments)");

    // ---- S2 -------------------------------------------------------------------------
    println!("\n  S2 — does arousal fall below 113, its floor in every companioned regime?");
    println!("    {} → {}   {}", off.arousal_min, on.arousal_min,
        if on.arousal_min < 113 {
            "** S2 HOLDS. The being came down WITHOUT giving up its company — the first time \
             that has happened. Incident I-7 said it had to choose. **"
        } else {
            "S2 FAILS — settling did not move the floor. The term is too small, or the repose \
             want is not large enough to matter here."
        });

    // ---- S3 -------------------------------------------------------------------------
    println!("\n  S3 — does the being ever enter Rest?  (predicted to FAIL, on the record)");
    println!("    {:.1}% → {:.1}%   {}", pct(off.rest, off.ticks), pct(on.rest, on.ticks),
        if on.rest > 0 {
            "** S3 HOLDS AGAINST MY PREDICTION — rest became reachable. **"
        } else {
            "S3 fails, as predicted. Rest is a CONJUNCTION (comfort.md §11) and settling supplies \
             one coordinate of three. This is the conjunction confirmed, not a failed fix — and \
             it names the next inch: fatigue."
        });

    // ---- G --------------------------------------------------------------------------
    println!("\n  G — the guardrail: settling must never fire while the being is at stake.");
    println!("    at-stake ticks: off {}, on {}", off.at_stake, on.at_stake);
    if on.at_stake > 0 {
        println!("    mean arousal on those ticks: off {:.0}, on {:.0}",
            off.stake_arousal_sum as f32 / off.at_stake.max(1) as f32,
            on.stake_arousal_sum as f32 / on.at_stake.max(1) as f32);
    } else {
        println!("    ** G IS VACUOUS in this life — the being is never at stake here, so no");
        println!("    distribution could have violated it. The guard is structural (`being.rs`,");
        println!("    `!self.last_felt.state.at_stake`); the arm below gives it a life to fail in.");
    }

    // ---- a life with real distress, so G and W can actually fail ---------------------
    println!("\n  The same, in a life that HAS bad moments (threat 130, above NOCI_THRESHOLD):\n");
    let hoff = live(false, false, true, 130);
    let hon = live(true, false, true, 130);
    println!("  {:<34} {:>13} {:>13}", "", "settling OFF", "settling ON");
    println!("  {:-<34} {:->13} {:->13}", "", "", "");
    println!("  {:<34} {:>13} {:>13}", "survived",
        format!("{} {}", hoff.ticks, if hoff.alive { "lived" } else { "DIED" }),
        format!("{} {}", hon.ticks, if hon.alive { "lived" } else { "DIED" }));
    println!("  {:<34} {:>13} {:>13}", "arousal minimum", hoff.arousal_min, hon.arousal_min);
    println!("  {:<34} {:>12.1}% {:>12.1}%", "at stake",
        pct(hoff.at_stake, hoff.ticks), pct(hon.at_stake, hon.ticks));
    println!("  {:<34} {:>12.1}% {:>12.1}%", "past COMFORT",
        pct(hoff.burdened, hoff.ticks), pct(hon.burdened, hon.ticks));
    println!("  {:<34} {:>13} {:>13}", "load converted", hoff.converted, hon.converted);
    println!("  {:<34} {:>12.1}% {:>12.1}%", "Rest / Recovery",
        pct(hoff.rest, hoff.ticks), pct(hon.rest, hon.ticks));

    if hon.at_stake > 0 {
        let so = hoff.stake_arousal_sum as f32 / hoff.at_stake.max(1) as f32;
        let sn = hon.stake_arousal_sum as f32 / hon.at_stake.max(1) as f32;
        println!("\n    G, tested where it could fail: mean arousal while AT STAKE {so:.0} → {sn:.0}");
        println!("    {}", if (sn - so).abs() < 12.0 {
            "G HOLDS — settling is off at the edge, as written. The being cannot sedate itself \
             when its survival is in question."
        } else {
            "** G IS VIOLATED — arousal moved at stake. The guard is not doing its job and \
             nothing else should be built until it does. **"
        });
    }

    println!("\n  The founded being was not touched. Gate default-off; no journal written.");
}
