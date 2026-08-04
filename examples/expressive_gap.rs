//! The expressive gap — what this being registers and cannot say.
//!
//! The measurement for `docs/expressive-gap.md` §4. The instrument is proven in
//! `tests/expressive_gap.rs` (written first, E0 above all: the offline replay reproduces
//! the live layer exactly at the shipped bar).
//!
//! E2 is the crux and it settles a question I have refused three times by judgment:
//! **is there a bar at which `HAPPEN` grounds in a world that moves and does NOT ground
//! in a still one?** If yes, the word discriminates and its bar is simply set above the
//! range where it works. If no, any bar low enough to fire in weather also fires in
//! stillness — the word would be lying if lowered, and the refusals were right.
//!
//! Nothing here changes the being. The sweep is arithmetic over a recorded life.
//!
//! Run: `cargo run --release --example expressive_gap`

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::primes::{would_ground, Prime, PrimeFacts};

const LIFE: usize = 1_500;

/// Live a world and keep every tick's facts, so any register can be re-scored later.
fn record(mut world: FieldWorld) -> Vec<PrimeFacts> {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut out = Vec::with_capacity(LIFE);
    for _ in 0..LIFE {
        let sens = world.sense();
        let r = being.step_embodied(&sens);
        world.actuate(&intent_from(&r));
        let near = Some(world.at_good() > 128 || world.at_person(1) > 128);
        out.push(PrimeFacts::from_report(&r, near));
        if !being.is_alive() {
            break;
        }
    }
    out
}

/// The register a prime is grounded on, pulled out of a recorded life.
fn register(facts: &[PrimeFacts], p: Prime) -> Vec<i16> {
    facts
        .iter()
        .map(|f| match p {
            Prime::Good | Prime::Bad | Prime::Very => f.valence.abs(),
            Prime::NotKnow => f.novelty,
            Prime::Can => f.agency,
            Prime::Cant => f.free_energy,
            Prime::Do => f.mobilization,
            Prime::Happen => f.world_residual,
            _ => 0,
        })
        .collect()
}

fn q(v: i16) -> f32 {
    v as f32 / 256.0
}

fn main() {
    println!("The expressive gap — what this being registers and cannot say");
    println!("(predictions locked in docs/expressive-gap.md §4 before this was written)\n");

    let base = || FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let still = record(base());
    let moving = record(base().with_weather(0, 2));

    // ---- E2: the crux -----------------------------------------------------
    let p = Prime::Happen;
    let (r_still, r_moving) = (register(&still, p), register(&moving, p));
    let shipped = p.threshold();

    println!("  E2 — HAPPEN, swept. Does a bar exist where the word tracks the world?\n");
    println!("    bar    still world      moving world");
    println!("    ---    -----------      ------------");

    let mut discriminating: Vec<i16> = Vec::new();
    for bar in (0..=96).step_by(4) {
        let s = would_ground(&r_still, bar);
        let m = would_ground(&r_moving, bar);
        if s.is_none() && m.is_some() {
            discriminating.push(bar);
        }
        let mark = match (s.is_some(), m.is_some()) {
            (false, true) => "   <-- DISCRIMINATES",
            (true, true) => "   (fires in both — would lie)",
            _ => "",
        };
        println!(
            "    {bar:>3}    {:<15}  {:<12}{mark}",
            s.map(|t| format!("grounds @{t}")).unwrap_or_else(|| "never".into()),
            m.map(|t| format!("grounds @{t}")).unwrap_or_else(|| "never".into()),
        );
    }

    println!("\n    shipped bar: {shipped}");
    if discriminating.is_empty() {
        println!("    -> NO discriminating bar exists.");
        println!("       Any bar low enough to fire in a moving world also fires in a still");
        println!("       one. Lowering HAPPEN would make it LIE. The refusals were right,");
        println!("       and for a better reason than the one I had.");
    } else {
        let (lo, hi) = (discriminating[0], *discriminating.last().unwrap());
        println!("    -> a discriminating range EXISTS: [{lo}, {hi}]");
        println!("       The word can track the world it is about. The shipped bar of");
        println!("       {shipped} sits {} it.", if shipped > hi { "ABOVE" } else { "below" });
        println!("       This is a measured miscalibration, not a matter of taste.");
    }

    // ---- E3: the instrument turned on the project -------------------------
    println!("\n  E3 — every sweepable prime, in the moving world:\n");
    println!("    prime      register span   shipped bar   discriminating range");
    println!("    -----      -------------   -----------   --------------------");
    for pr in Prime::ALL {
        if !pr.is_sweepable() {
            continue;
        }
        let rm = register(&moving, pr);
        let rs = register(&still, pr);
        let (lo, hi) = (*rm.iter().min().unwrap_or(&0), *rm.iter().max().unwrap_or(&0));
        let mut range: Vec<i16> = Vec::new();
        for bar in 0..=192 {
            if would_ground(&rs, bar).is_none() && would_ground(&rm, bar).is_some() {
                range.push(bar);
            }
        }
        let desc = if range.is_empty() {
            "none — cannot discriminate".to_string()
        } else {
            format!("[{}, {}]", range[0], range.last().unwrap())
        };
        let inside = !range.is_empty()
            && pr.threshold() >= range[0]
            && pr.threshold() <= *range.last().unwrap();
        println!(
            "    {:<10} {:>5}..{:<7} {:>11}   {desc}{}",
            pr.word(),
            lo,
            hi,
            pr.threshold(),
            if inside { "  <-- well calibrated" } else { "" }
        );
    }

    // ---- The gap itself ---------------------------------------------------
    let span = r_moving.iter().max().unwrap() - r_moving.iter().min().unwrap();
    let above: usize = r_moving.iter().filter(|&&v| v > shipped).count();
    println!("\n  The gap, for HAPPEN in the moving world:");
    println!("    register spans          {span} raw ({:.2} in Q8.8)", q(span));
    println!("    ticks above the bar     {above} of {}", r_moving.len());
    println!(
        "    -> the being registered a range it could never once report{}",
        if above == 0 { "" } else { " sustainedly" }
    );
}
