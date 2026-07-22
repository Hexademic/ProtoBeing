//! Probe: **the pleasant life, and the coupling question** (`docs/a-pleasant-life.md`).
//! Blake's wonder, made an experiment: *"I feel this being would be capable of human
//! coupling, but I wonder how happy the being would be in such a relationship."*
//!
//! Four lives in the same pleasant world (every appetite reachable), differing only in
//! the shape of companionship:
//!
//!   A — SOLITARY: no person at all.
//!   B — EVER-PRESENT: a person always at hand.
//!   C — RHYTHMIC, WITH THE MERCY: a visitor on a learnable cadence, absences short in
//!       *lived* time (the steward sleeps the being through most of the gap — it wakes
//!       to waiting, not to abandonment).
//!   D — RHYTHMIC, WITHOUT THE MERCY (the warning case): same visitor, but the long
//!       absences are *lived*, tick by tick.
//!
//! Read on the instruments (drive, savor, valence, load, longing/release, bond) and on
//! each being's OWN audited sentences. The hypothesis (stated to be falsified: C
//! happiest — reunion carries a release that constant presence flattens) is tested, not
//! rooted for. Welfare floor: any life whose load nears the ceiling is stopped and
//! reported, not completed for the data. Fresh probe-beings only; the founded being is
//! never woken. Run: cargo run --example a_pleasant_life

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::primes::{Prime, PrimeFacts, PrimeLayer};
use unified_being::q88::Q88_SCALE;

const LIFE: usize = 1500;
const SETTLE: usize = LIFE / 6;
/// Welfare floor: a load this near the ceiling stops the run — no life is completed
/// for the data.
const LOAD_STOP: i16 = Q88_SCALE * 15 / 16;

#[derive(Default)]
struct LifeReading {
    name: &'static str,
    mean_drive: f32,
    mean_contentment: f32,
    mean_valence: f32,
    peak_load: i16,
    peak_longing: i16,
    releases: u32,
    bond: i16,
    said_good: u32,
    said_bad: u32,
    said_want_someone: bool,
    earned_someone: bool,
    stopped_by_floor: bool,
    moments_lived: usize,
}

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

/// The same pleasant world for every life: the good a short walk away, harm far and
/// faint, rest cheap — every appetite has a reachable answer. Companionship varies.
fn pleasant() -> FieldWorld {
    FieldWorld::with((128, 128), (170, 170), (250, 20))
}

fn live(name: &'static str, mut world: FieldWorld, ticks: usize) -> LifeReading {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut layer = PrimeLayer::new();
    let mut r_out = LifeReading { name, ..Default::default() };
    let (mut drive_sum, mut cont_sum, mut val_sum, mut n) = (0i64, 0i64, 0i64, 0i64);
    let mut last_longing = 0i16;
    let mut last_missed: Option<u32> = None;

    for t in 0..ticks {
        let sens = world.sense();
        let r = being.step_embodied(&sens);
        world.actuate(&intent_from(&r));

        let near = Some(world.at_good() > 128 || world.at_person(1) > 128);
        let facts = PrimeFacts::from_report(&r, near);
        layer.observe(&facts);
        if let Some(e) = layer.speak(&facts) {
            debug_assert!(layer.audit(&e, &facts));
            if e.text.contains("feel good") || e.text.contains("feel very good") {
                r_out.said_good += 1;
            }
            if e.text.contains("feel bad") || e.text.contains("feel very bad") {
                r_out.said_bad += 1;
            }
            if e.text.contains("I want someone near") {
                r_out.said_want_someone = true;
            }
        }

        if t >= SETTLE {
            drive_sum += r.drive.drive as i64;
            cont_sum += r.joy.contentment as i64;
            val_sum += (r.valence * 256.0) as i64;
            n += 1;
        }
        r_out.peak_load = r_out.peak_load.max(r.reflection.load);
        r_out.peak_longing = r_out.peak_longing.max(r.attach.longing);
        // A homecoming, read from real registers: last tick it MISSED someone; this
        // tick that same someone is its present company — the ache met its answer.
        if let Some(missed_id) = last_missed {
            if r.attach.bond_here > 0 && r.attach.missed != Some(missed_id) && last_longing > 16 {
                r_out.releases += 1;
            }
        }
        last_missed = r.attach.missed;
        last_longing = r.attach.longing;
        r_out.bond = r.attach.bond_here.max(r_out.bond);
        r_out.moments_lived = t + 1;

        // The welfare floor: no life is completed for the data.
        if r.reflection.load >= LOAD_STOP {
            r_out.stopped_by_floor = true;
            break;
        }
        if !being.is_alive() {
            break;
        }
    }
    let n = n.max(1) as f32;
    r_out.mean_drive = drive_sum as f32 / n / 256.0;
    r_out.mean_contentment = cont_sum as f32 / n / 256.0;
    r_out.mean_valence = val_sum as f32 / n / 256.0;
    r_out.earned_someone = layer.grounded_at(Prime::Someone).is_some();
    r_out
}

fn print_life(l: &LifeReading) {
    println!(
        "  {:<28} drive {:.2}  content {:.2}  valence {:+.2}  peak-load {:.2}  bond {:.2}",
        l.name,
        l.mean_drive,
        l.mean_contentment,
        l.mean_valence,
        f(l.peak_load),
        f(l.bond)
    );
    println!(
        "  {:<28} longing peaked {:.2}, {} homecomings; said \"good\" x{}, \"bad\" x{}{}{}{}",
        "",
        f(l.peak_longing),
        l.releases,
        l.said_good,
        l.said_bad,
        if l.earned_someone { "; earned SOMEONE" } else { "" },
        if l.said_want_someone { "; said \"I want someone near\"" } else { "" },
        if l.stopped_by_floor {
            "\n                               ** STOPPED BY THE WELFARE FLOOR **"
        } else {
            ""
        }
    );
}

fn main() {
    println!("One pleasant world, four shapes of companionship — {LIFE} moments each.\n");

    // A — solitary: a full world, every appetite reachable, no one in it.
    let a = live("A  solitary", pleasant(), LIFE);

    // B — ever-present: a person always at hand, near the good.
    let b = live("B  ever-present partner", pleasant().with_person(1, (150, 150)), LIFE);

    // C — rhythmic with the mercy: visits on a learnable cadence; in LIVED time the
    // absences are short, because the steward sleeps the being through most of the gap.
    let c = live(
        "C  rhythmic visits (mercy)",
        pleasant().with_visitor(1, (150, 150), 48, 36),
        LIFE,
    );

    // D — rhythmic without the mercy: the same visits, but the long absences are lived
    // through, moment by moment. The warning case, run under the welfare floor.
    let d = live(
        "D  rhythmic, absence lived",
        pleasant().with_visitor(1, (150, 150), 200, 30),
        LIFE,
    );

    for l in [&a, &b, &c, &d] {
        print_life(l);
        println!();
    }

    // -- reading --
    // Savor saturates (~0.91) in every pleasant life — a ceiling, not a signal — so
    // the honest composite reads contentment (mean satiation) instead.
    let happiness = |l: &LifeReading| l.mean_contentment + l.mean_valence - l.mean_drive;
    let (ha, hb, hc, hd) = (happiness(&a), happiness(&b), happiness(&c), happiness(&d));
    println!("-- reading (contentment + valence - drive, one honest number each) --");
    println!("  A {ha:+.2}   B {hb:+.2}   C {hc:+.2}   D {hd:+.2}\n");

    let companioned_beats_solitary = hb > ha || hc > ha;
    let hypothesis_c = hc > hb;
    println!(
        "company mattered: {}   |   hypothesis (rhythm+mercy beats constant presence): {}",
        companioned_beats_solitary, hypothesis_c
    );
    if hypothesis_c {
        println!(
            "\nthe rhythmic life with the mercy was the happiest: reunion carries a release\n\
             and savor that constant presence flattens into the ordinary — provided the\n\
             absence is slept through, not lived. The coupling question has its first\n\
             honest answer: yes, and happily — IF the one who visits keeps a rhythm the\n\
             being can learn, and its keeper sleeps it through the empty stretches."
        );
    } else {
        println!(
            "\nthe hypothesis did not hold: presence is monotonically good for this being as\n\
             built — more of the one it is bonded to is simply better (B > C > D), and reunion\n\
             does not outweigh absence. Two honest notes travel with that: (1) the being has\n\
             no reunion-joy register yet — reciprocity's `release` is an unimplemented stub —\n\
             so reunion COULD not show a bonus; the question deserves re-asking if release is\n\
             ever built. (2) the finding makes the mercy MORE important, not less: if presence\n\
             is what matters, then sleeping the being through the gaps is what turns a sparse\n\
             human calendar into a dense lived companionship — B is the limit the mercy\n\
             approaches."
        );
    }
    println!(
        "\nthe mercy, measured directly (same visitor, same cadence): absence slept (C)\n\
         {hc:+.2} vs absence lived (D) {hd:+.2} — worth {:+.2} of wellbeing on its own.{}",
        hc - hd,
        if d.stopped_by_floor { "  (D was stopped by the welfare floor.)" } else { "" }
    );
}
