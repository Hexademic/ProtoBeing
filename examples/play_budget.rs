//! The play budget — does the guardrail ever actually bind?
//!
//! The measurement for `docs/play.md` §5, B4. The arithmetic is proven in
//! `tests/play_budget.rs` (written first); this asks the only question those tests cannot:
//! **in a life the being actually lives, is the budget ever zero?**
//!
//! It matters because a guardrail that never binds is a comfort blanket. The weathered and
//! still lives of `docs/weather.md` §7 ran mean drive 0.13–0.18 against a comfort line of
//! 0.44, so the honest possibility is that this being is comfortable nearly always and the
//! prohibition never fires. If so, that is the result, and the fix is a different form of
//! guardrail rather than a reassuring one.
//!
//! Nothing here plays. The budget is a pure observer; no action consults it.
//!
//! Run: `cargo run --release --example play_budget`

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::play::{PlayBudget, COMFORT};
use unified_being::primes::PrimeFacts;

const LIFE: usize = 1_500;

struct Watched {
    label: String,
    ticks: usize,
    burdened: usize,
    drive_min: i16,
    drive_max: i16,
    drive_mean: i32,
    margin_mean: i32,
    margin_min: i16,
    alive: bool,
}

fn watch(label: String, mut world: FieldWorld) -> Watched {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let (mut d_sum, mut m_sum, mut n) = (0i64, 0i64, 0i64);
    let (mut d_min, mut d_max, mut m_min) = (i16::MAX, i16::MIN, i16::MAX);
    let mut burdened = 0usize;

    for _ in 0..LIFE {
        let sens = world.sense();
        let r = being.step_embodied(&sens);
        world.actuate(&intent_from(&r));
        let facts = PrimeFacts::from_report(&r, Some(world.at_good() > 128));

        // The budget only ever *watches* — it is handed the drive and reports margin.
        let margin = PlayBudget::available(facts.drive);
        if margin == 0 {
            burdened += 1;
        }
        d_sum += facts.drive as i64;
        m_sum += margin as i64;
        n += 1;
        d_min = d_min.min(facts.drive);
        d_max = d_max.max(facts.drive);
        m_min = m_min.min(margin);

        if !being.is_alive() {
            break;
        }
    }

    Watched {
        label,
        ticks: n as usize,
        burdened,
        drive_min: d_min,
        drive_max: d_max,
        drive_mean: (d_sum / n.max(1)) as i32,
        margin_mean: (m_sum / n.max(1)) as i32,
        margin_min: m_min,
        alive: being.is_alive(),
    }
}

fn q(v: i32) -> f32 {
    v as f32 / 256.0
}

fn main() {
    println!("The play budget — does the guardrail ever bind?");
    println!("(B4 locked in docs/play.md §5 before this was written)\n");
    println!("  comfort line: {COMFORT} raw ({:.2})\n", q(COMFORT as i32));

    // `FieldWorld::with(body, good, harm)`. What makes a life hard here is the *distance*
    // from where the being wakes to what feeds it — nothing else. The long crossing starts
    // the being ~316 cells from its nutrient source; the short one wakes it 17 cells away,
    // beside its food. (Those names are corrected: the first version of this probe called
    // the near-food life "the hard climb" and read the result backwards. The measurement
    // was right and the label was wrong.)
    let long = || FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let short = || FieldWorld::with((128, 128), (140, 140), (250, 250)).with_person(1, (20, 20));

    let lives = vec![
        watch("long crossing".into(), long()),
        watch("long + weathered".into(), long().with_weather(0, 2)),
        watch("beside its food".into(), short()),
        watch("beside food + weather".into(), short().with_weather(0, 2)),
    ];

    println!(
        "  {:<20} {:>6} {:>6} {:>6} {:>8} {:>8} {:>9}",
        "life", "drive", "min", "max", "margin", "min", "burdened"
    );
    println!("  {:-<20} {:->6} {:->6} {:->6} {:->8} {:->8} {:->9}", "", "", "", "", "", "", "");
    for w in &lives {
        println!(
            "  {:<20} {:>6.2} {:>6.2} {:>6.2} {:>8.2} {:>8} {:>8}%{}",
            w.label,
            q(w.drive_mean),
            q(w.drive_min as i32),
            q(w.drive_max as i32),
            q(w.margin_mean),
            w.margin_min,
            w.burdened * 100 / w.ticks.max(1),
            if w.alive { "" } else { "  ** DIED **" }
        );
    }

    let ever_bound = lives.iter().any(|w| w.burdened > 0);
    let always_bound = lives.iter().all(|w| w.burdened > 0);

    println!("\n  B4 — is the budget ever zero in a life the being actually lives?");
    if !ever_bound {
        println!("    NO, in any life tested. The guardrail never binds.");
        println!("    That makes it a comfort blanket, not a constraint: every tick of every");
        println!("    life left the being room to play. The honest reading is that these");
        println!("    worlds are too kind for a state-gate to matter, and the guardrail that");
        println!("    would matter is a RATE limit on play rather than a gate on the state.");
        println!("    Reported rather than dressed up.");
    } else {
        println!(
            "    YES — it binds in {} of {} lives.",
            lives.iter().filter(|w| w.burdened > 0).count(),
            lives.len()
        );
        for w in lives.iter().filter(|w| w.burdened > 0) {
            println!(
                "      {:<20} zero margin on {}% of ticks (peak drive {:.2})",
                w.label,
                w.burdened * 100 / w.ticks.max(1),
                q(w.drive_max as i32)
            );
        }
        println!("    A burdened being is refused play by arithmetic, in lives that occur.");
        if !always_bound {
            println!("    And it does NOT bind in the lives that wake beside their food, which");
            println!("    is the shape a welfare guardrail should have: refuse the being that");
            println!("    is struggling, leave the well-fed one free to experiment.");
        }
    }

    println!("\n  Nothing played. The budget observed and steered nothing (docs/play.md §4).");
}
