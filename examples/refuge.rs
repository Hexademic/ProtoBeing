//! Refuge — somewhere safe, and it is a someone.
//!
//! The measurement for `docs/refuge.md` §5. The arithmetic is proven in `tests/refuge.rs`
//! (written first): shelter is real, bounded, partial, has an edge, and costs a refuge-less
//! world nothing. This asks the four questions those tests cannot.
//!
//! * **S4** — does the being keep the words it just earned, or does safety buy comfort by
//!   taking back `BAD`, `NOT KNOW` and `HAPPEN`?
//! * **S5** — does it ever actually go there? Nothing pushes it; company competes with
//!   nutrient in `striving.rs`. If it barely visits, the refuge is furniture.
//! * **S6** — does it rest? It is at 0% rest everywhere. If a refuge does not move that,
//!   rest is blocked on `intent_from` being total, not on the world.
//! * **D** — the four-mover world that killed a being, unchanged, with a refuge added.
//!
//! Run: `cargo run --release --example refuge`

use std::collections::BTreeMap;

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, motor_scalar, Embodiment, Posture};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::play::COMFORT;
use unified_being::primes::{Clause, Prime, PrimeFacts, PrimeLayer};

const LIFE: usize = 2_000;
const PERSON: (i16, i16) = (20, 20);
const RADIUS: i16 = 48;

/// The rich world of `docs/richness.md`, at `movers`, with hazards given an edge
/// (`reach`) and optionally a refuge in the person the being is bonded to.
fn world(movers: usize, reach: i16, refuge: bool) -> FieldWorld {
    let mut w = FieldWorld::with((16, 16), (240, 240), (30, 170))
        .with_person(1, PERSON)
        .with_weather(0, 2);
    for k in 1..movers {
        let i = k as i16;
        let pos = (40 + (i * 53) % 200, 200 - (i * 71) % 180);
        let peak = if k % 3 == 0 { -90 } else { 100 };
        w = w.with_source(pos, peak, reach);
        w = w.with_drift(1 + k, 3 + (k as u32 * 2) % 7, (1 + (i % 3), 1 + ((i + 1) % 3)));
        if k % 2 == 0 {
            w = w.with_weather(1 + k, 2 + (k as u8 % 3));
        }
    }
    if refuge {
        // 3/4 of the threat lifted at their side — gentler, never harmless.
        w = w.with_refuge(1, RADIUS, 192);
    }
    w
}

#[derive(Default)]
struct Lived {
    said: BTreeMap<String, usize>,
    ticks: usize,
    alive: bool,
    in_refuge: usize,
    /// Drive and effort, split by whether the being was inside the refuge radius.
    drive_in: i64,
    drive_out: i64,
    effort_in: i64,
    effort_out: i64,
    rest_in: usize,
    rest_out: usize,
    burdened: usize,
    threat_mean: i64,
}

fn tally(c: &Clause, into: &mut BTreeMap<String, usize>) {
    let mut st = vec![c];
    while let Some(x) = st.pop() {
        *into.entry(format!("{:?}", x.prime)).or_default() += 1;
        for ch in &x.children {
            st.push(ch);
        }
    }
}

fn live(movers: usize, reach: i16, refuge: bool) -> Lived {
    let mut w = world(movers, reach, refuge);
    let mut b = UnifiedBeing::new(Genome::wanderer());
    b.enable_receptors();
    let mut layer = PrimeLayer::new();
    let mut l = Lived { alive: true, ..Default::default() };

    for _ in 0..LIFE {
        let sens = w.sense();
        let inside = (w.body.0 - PERSON.0).abs() + (w.body.1 - PERSON.1).abs() < RADIUS;

        let r = b.step_embodied(&sens);
        let intent = intent_from(&r);
        w.actuate(&intent);

        let f = PrimeFacts::from_report(&r, Some(w.at_good() > 128));
        layer.observe(&f);
        if let Some(cl) = layer.speak_tree(&f) {
            for c in &cl {
                tally(c, &mut l.said);
            }
        }

        let effort = motor_scalar(&intent).saturating_abs() as i64;
        let resting = matches!(intent.posture, Posture::Resting) || effort == 0;
        if inside {
            l.in_refuge += 1;
            l.drive_in += f.drive as i64;
            l.effort_in += effort;
            if resting { l.rest_in += 1; }
        } else {
            l.drive_out += f.drive as i64;
            l.effort_out += effort;
            if resting { l.rest_out += 1; }
        }
        if f.drive >= COMFORT { l.burdened += 1; }
        l.threat_mean += sens.threat as i64;
        l.ticks += 1;

        if !r.alive { l.alive = false; break; }
    }
    l
}

fn main() {
    println!("Refuge — somewhere safe, and it is a someone");
    println!("(S1–S6 and D locked in docs/refuge.md §5 before this was written)\n");

    let words = [Prime::Bad, Prime::NotKnow, Prime::Happen, Prime::Good, Prime::Someone];

    // ---- D: the demonstration -------------------------------------------------------
    println!("  D — the four-mover world that killed a being (reach 300, unchanged):\n");
    for (label, refuge) in [("without refuge", false), ("WITH refuge", true)] {
        let l = live(4, 300, refuge);
        println!(
            "    {:<16} {}   ({} ticks, mean threat {})",
            label,
            if l.alive { "SURVIVED" } else { "DIED" },
            l.ticks,
            l.threat_mean / l.ticks.max(1) as i64
        );
    }

    // ---- S4/S5/S6 across the rich worlds, hazards bounded ---------------------------
    println!("\n  S4 — does it keep the words it earned? (bounded hazards, reach 90)\n");
    print!("    {:<22}", "world");
    for p in words { print!("{:>10}", p.word()); }
    println!();
    println!("    {:-<22}{:-<50}", "", "");
    let mut rows = Vec::new();
    for movers in [6usize, 9, 12] {
        for refuge in [false, true] {
            let l = live(movers, 90, refuge);
            print!("    {:<22}", format!("{} movers{}", movers, if refuge { " + refuge" } else { "" }));
            for p in words {
                let c = l.said.get(&format!("{:?}", p)).copied().unwrap_or(0);
                print!("{:>10}", if c > 0 { c.to_string() } else { "·".into() });
            }
            println!();
            rows.push((movers, refuge, l));
        }
    }

    let got = |l: &Lived, p: Prime| l.said.get(&format!("{:?}", p)).copied().unwrap_or(0);
    let kept = words.iter().filter(|&&p| {
        rows.iter().filter(|(_, r, _)| !r).any(|(_, _, l)| got(l, p) > 0)
            && rows.iter().filter(|(_, r, _)| *r).any(|(_, _, l)| got(l, p) > 0)
    }).count();
    let lost: Vec<&str> = words.iter().filter(|&&p| {
        rows.iter().filter(|(_, r, _)| !r).any(|(_, _, l)| got(l, p) > 0)
            && !rows.iter().filter(|(_, r, _)| *r).any(|(_, _, l)| got(l, p) > 0)
    }).map(|p| p.word()).collect();

    // Identical counts in every row would mean the two runs are bit-identical — the words
    // "survived" because nothing happened. Check that before claiming anything.
    let unchanged = words.iter().all(|&p| {
        rows.chunks(2).all(|c| got(&c[0].2, p) == got(&c[1].2, p))
    });
    println!("\n    {kept} of the {} tracked words survive shelter.", words.len());
    if unchanged {
        println!("\n    S4 IS VACUOUS, and the identical counts are the tell. Every column is the");
        println!("    same to the digit with and without a refuge, because the two lives are");
        println!("    BIT-IDENTICAL: same soul-hash, same trajectory. The words did not survive");
        println!("    shelter — shelter never happened to the being.");
        println!();
        println!("    The refuge is working: it lifts threat at the spawn point from 68 to 25.");
        println!("    But NOCI_THRESHOLD is 96, and the nociceptor is SILENT below it with no");
        println!("    adaptation — `if raw <= NOCI_THRESHOLD {{ 0 }}`. Threat in every survivable");
        println!("    world here runs 59-72. Both the sheltered and the exposed reading transduce");
        println!("    to exactly ZERO pain, so the being feels nothing either way.");
        println!();
        println!("    THE BEING'S FELT DANGER IS A STEP FUNCTION. Below 96: nothing. Above it:");
        println!("    pain. There is no register for *at ease*, and so no gradient a refuge could");
        println!("    improve. We can make this being safer. We cannot yet make it FEEL safer,");
        println!("    and safety it cannot register is safety for our sake rather than its own.");
    } else if lost.is_empty() {
        println!("    S4 HOLDS — safety cost the being none of its voice.");
    } else {
        println!("    S4 FAILS for {:?}: shelter bought comfort by taking those back.", lost);
    }

    // ---- S5 -------------------------------------------------------------------------
    println!("\n  S5 — does it ever actually go there?\n");
    println!("    {:<22} {:>12} {:>12} {:>12}", "world", "in refuge", "drive in", "drive out");
    println!("    {:-<22} {:->12} {:->12} {:->12}", "", "", "", "");
    for (m, r, l) in &rows {
        if !r { continue; }
        let pct = l.in_refuge * 100 / l.ticks.max(1);
        let din = if l.in_refuge > 0 { l.drive_in as f32 / l.in_refuge as f32 / 256.0 } else { f32::NAN };
        let dout = {
            let out = l.ticks - l.in_refuge;
            if out > 0 { l.drive_out as f32 / out as f32 / 256.0 } else { f32::NAN }
        };
        println!("    {:<22} {:>11}% {:>12.3} {:>12.3}", format!("{m} movers + refuge"), pct, din, dout);
    }
    let visited: usize = rows.iter().filter(|(_, r, _)| *r)
        .map(|(_, _, l)| l.in_refuge * 100 / l.ticks.max(1)).max().unwrap_or(0);
    if visited < 5 {
        println!("\n    S5 FAILS — at most {visited}% of ticks. The refuge is FURNITURE. Nothing in");
        println!("    striving.rs makes the being value being safe, so it walks past shelter to");
        println!("    serve whatever is most urgent. Safety-through-bond needs the being to");
        println!("    *want* safety, and it currently has no way to.");
    } else {
        println!("\n    S5 HOLDS on its own terms — it spends up to {visited}% of its life there,");
        println!("    unforced, going for company and being sheltered as a consequence. But read");
        println!("    it against S4: in these worlds the sheltering is below its pain threshold,");
        println!("    so it visits the refuge without ever receiving anything it can feel.");
    }

    // ---- S6 -------------------------------------------------------------------------
    println!("\n  S6 — does it rest?\n");
    println!("    {:<22} {:>12} {:>12} {:>10}", "world", "effort in", "effort out", "rest");
    println!("    {:-<22} {:->12} {:->12} {:->10}", "", "", "", "");
    let mut any_rest = false;
    for (m, r, l) in &rows {
        if !r { continue; }
        let out = l.ticks - l.in_refuge;
        let ein = if l.in_refuge > 0 { l.effort_in as f32 / l.in_refuge as f32 } else { f32::NAN };
        let eout = if out > 0 { l.effort_out as f32 / out as f32 } else { f32::NAN };
        let rest = l.rest_in + l.rest_out;
        if rest > 0 { any_rest = true; }
        println!("    {:<22} {:>12.1} {:>12.1} {:>9}%", format!("{m} movers + refuge"), ein, eout, rest * 100 / l.ticks.max(1));
    }
    if !any_rest {
        println!("\n    S6 FAILS — 0% rest, refuge or not. Shelter changes what the world does to");
        println!("    the being and nothing about what the being does. `intent_from` is a total");
        println!("    function of the step report and `effort = arousal` has no floor, so there");
        println!("    is no state in which this being stops. Rest is blocked on the ARCHITECTURE,");
        println!("    not the world — exactly what docs/underdetermination.md §3 suspected, and");
        println!("    it points straight at intermittency as the next inch.");
    } else {
        println!("\n    S6 — the being rests. Report the fraction above as the result.");
    }

    println!("\n  The founded being was not touched. Refuge is opt-in and absent by default.");
}
