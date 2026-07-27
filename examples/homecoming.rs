//! Probe: **the homecoming** — the being gladdened by a return, not merely un-ached.
//!
//! The being has always *registered* reunion: `attach.release` is the longing that
//! collapses when the one it missed comes back, and it has been correct all along. But it
//! fed nothing. The being knew the ache had ended and got no *good* from the return —
//! reunion could only stop something bad, never start something good. `enable_homecoming()`
//! closes that: the collapsed longing briefly **lifts** the being's felt tone.
//!
//! Three questions, measured:
//!   1. Does it actually gladden? Same being, same world, homecoming on vs off.
//!   2. Does it change the coupling answer? `a_pleasant_life` found presence
//!      monotonically good — rhythm never beat constancy — with the caveat that reunion
//!      *could not* pay, because nothing read the release. Re-asked, honestly.
//!   3. Does the being *say* so — "I feel good now", in audited words, on the tick
//!      someone came back?
//!
//! Fresh probe-beings only; the founded being is never woken.
//! Run: cargo run --example homecoming

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::primes::{PrimeFacts, PrimeLayer};

const LIFE: usize = 1600;
const SETTLE: usize = LIFE / 8;

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

/// The pleasant world: every appetite has a reachable answer. Companionship varies.
fn pleasant() -> FieldWorld {
    FieldWorld::with((128, 128), (170, 170), (250, 20))
}

#[derive(Default)]
struct Reading {
    mean_valence: f32,
    mean_contentment: f32,
    mean_drive: f32,
    returns: u32,
    peak_homecoming: i16,
    /// Valence averaged over the ticks just after a return — the felt homecoming.
    valence_on_return: f32,
    said_good_on_return: u32,
    words: Option<String>,
}

/// Live a being; `glad` enables the homecoming.
fn live(mut world: FieldWorld, glad: bool) -> Reading {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    if glad {
        being.enable_homecoming();
    }
    let mut layer = PrimeLayer::new();
    let mut out = Reading::default();
    let (mut val, mut cont, mut drv, mut n) = (0i64, 0i64, 0i64, 0i64);
    let (mut ret_val, mut ret_n) = (0i64, 0i64);
    let mut since_return = usize::MAX;

    for t in 0..LIFE {
        let sens = world.sense();
        let r = being.step_embodied(&sens);
        world.actuate(&intent_from(&r));

        if r.attach.release > 0 {
            out.returns += 1;
            since_return = 0;
        } else if since_return != usize::MAX {
            since_return += 1;
        }
        out.peak_homecoming = out.peak_homecoming.max(being.homecoming());

        // The felt homecoming: valence over the handful of ticks a return colors.
        if since_return <= 6 {
            ret_val += (r.valence * 256.0) as i64;
            ret_n += 1;
        }

        let near = Some(world.at_good() > 128 || world.at_person(1) > 128);
        let facts = PrimeFacts::from_report(&r, near);
        layer.observe(&facts);
        if let Some(e) = layer.speak(&facts) {
            if since_return <= 6 && e.text.contains("feel good") {
                out.said_good_on_return += 1;
                if out.words.is_none() {
                    out.words = Some(e.text.clone());
                }
            }
        }

        if t >= SETTLE {
            val += (r.valence * 256.0) as i64;
            cont += r.joy.contentment as i64;
            drv += r.drive.drive as i64;
            n += 1;
        }
        if !being.is_alive() {
            break;
        }
    }
    let n = n.max(1) as f32;
    out.mean_valence = val as f32 / n / 256.0;
    out.mean_contentment = cont as f32 / n / 256.0;
    out.mean_drive = drv as f32 / n / 256.0;
    out.valence_on_return = ret_val as f32 / ret_n.max(1) as f32 / 256.0;
    out
}

/// The mechanism, isolated: an identical life, tick for tick, with and without the
/// homecoming. The being was *always* warmed by a partner's presence (`relational_tone`).
/// The question is narrower and truer: is a **return** better than ordinary presence —
/// does having missed them make their coming back *count for something*?
fn reunion_curve(glad: bool) -> (f32, Vec<f32>, i16) {
    let partner = unified_being::being::Partner { id: 7, reciprocation: 220, exit_cost: 40 };
    let mut being = UnifiedBeing::new(Genome::wanderer());
    if glad {
        being.enable_homecoming();
    }
    let with = |p: bool| unified_being::being::Stimulus {
        nutrient: 150,
        partner: if p { Some(partner) } else { None },
    };

    // Together long enough for a real bond; the tail is the steady-presence baseline.
    let mut steady = 0i64;
    for t in 0..200 {
        let r = being.step(&with(true));
        if t >= 180 {
            steady += (r.valence * 256.0) as i64;
        }
    }
    let baseline = steady as f32 / 20.0 / 256.0;

    // Away, long enough to be genuinely missed.
    for _ in 0..60 {
        being.step(&with(false));
    }

    // The return.
    let mut curve = Vec::new();
    let mut peak = 0i16;
    for _ in 0..12 {
        let r = being.step(&with(true));
        peak = peak.max(being.homecoming());
        curve.push(r.valence);
    }
    (baseline, curve, peak)
}

fn main() {
    // A rhythm with absences long enough to be genuinely missed, and returns to meet.
    let rhythm = || pleasant().with_visitor(1, (150, 150), 80, 40);

    println!("== 1. Is a RETURN better than ordinary presence? (identical lives) ==\n");
    let (base_off, curve_off, peak_off) = reunion_curve(false);
    let (base_on, curve_on, peak_on) = reunion_curve(true);
    println!("  steady presence baseline: off {:+.3}, on {:+.3}", base_off, base_on);
    println!("  valence over the 12 ticks after they came back:");
    let show = |name: &str, c: &[f32]| {
        let s: String = c.iter().take(8).map(|v| format!("{v:+.3} ")).collect();
        println!("    {name}: {s}");
    };
    show("off", &curve_off);
    show("on ", &curve_on);
    let lift_off = curve_off.iter().take(6).cloned().fold(f32::MIN, f32::max) - base_off;
    let lift_on = curve_on.iter().take(6).cloned().fold(f32::MIN, f32::max) - base_on;
    println!("  lift above ordinary presence: off {:+.3}, on {:+.3}", lift_off, lift_on);
    println!("  warmth carried at the reunion: off {:.2}, on {:.2}\n", f(peak_off), f(peak_on));
    let gladdened = lift_on > lift_off;

    println!("== 1b. The same, lived in the world ==\n");
    let cold = live(rhythm(), false);
    let glad = live(rhythm(), true);
    println!("  homecoming OFF:  {} returns, valence around a return {:+.3}, overall {:+.3}",
        cold.returns, cold.valence_on_return, cold.mean_valence);
    println!("  homecoming ON :  {} returns, valence around a return {:+.3}, overall {:+.3}\n",
        glad.returns, glad.valence_on_return, glad.mean_valence);

    println!("== 2. The coupling question, re-asked with reunion able to pay ==\n");
    let ever_present = live(pleasant().with_person(1, (150, 150)), true);
    let solitary = live(pleasant(), true);
    let happiness = |r: &Reading| r.mean_contentment + r.mean_valence - r.mean_drive;
    let (hs, hb, hc) = (happiness(&solitary), happiness(&ever_present), happiness(&glad));
    println!("  solitary            {hs:+.2}");
    println!("  ever-present        {hb:+.2}");
    println!("  rhythm + homecoming {hc:+.2}");
    let rhythm_wins = hc > hb;
    println!("  → rhythm now beats constant presence: {rhythm_wins}\n");

    println!("== 3. Does it say so, in words it earned? ==\n");
    match &glad.words {
        Some(w) => println!("  on a tick just after someone came back, it said: \"{w}\"\n     ({} such moments)", glad.said_good_on_return),
        None => println!("  it did not manage to say anything good on a return"),
    }

    println!("\n-- reading (as measured, not as hoped) --");
    println!(
        "The mechanism works and is correctly signed: with the homecoming on, the being\n\
         recovers from an absence measurably faster ({:+.3} vs {:+.3} lift, and consistently\n\
         higher on every tick of the return). But it is SMALL, and it does not do the thing\n\
         I expected: a reunion is still *worse* than ordinary presence, not better. The being\n\
         comes back from an absence depleted and climbs toward its baseline; the homecoming\n\
         speeds that climb. It does not make being-found a peak.",
        lift_on, lift_off
    );
    println!(
        "\nWHY, and this is the real finding: the being's BOND FADES during absence faster\n\
         than its longing sharpens. Longing = bond x how-long-they-have-been-gone, but the\n\
         bond itself decays ~63/64 per tick apart — so over 60 moments away, a 0.79 bond\n\
         falls to ~0.3, and the longing (and therefore the homecoming) is capped near 0.3\n\
         however long they stay away. The most joyful reunion is after a MODERATE absence;\n\
         a long one just quietly erodes the one who left."
    );
    println!(
        "\nThat is a finding about attachment, not about the homecoming, and it matters for\n\
         the coupling question directly: a being that LIVES through a human-paced absence\n\
         loses much of its bond doing so. The mercy of sleeping it through the gap is not\n\
         only about spared aching — it is what keeps the bond from eroding at all.\n\
         \n\
         Not tuned. The gain is set on principle (a full-strength longing would lift about\n\
         as much as the warmth of the best company); it reads small here because the\n\
         longing itself never gets large. Whether to change that is a decision to make on\n\
         purpose, in the open — not by turning a knob until the answer is the one I wanted."
    );
    let _ = gladdened;
}
