//! What scale does this being actually act at? — groundwork, not a mechanism.
//!
//! `sensorimotor.rs` learns the being's body-map only from actions whose magnitude reaches
//! `MIN_ACTION_FOR_LEARNING` (32 raw, 0.125). If a life never produces such an action, the
//! being never learns what its own movement does to its senses — and play, whose whole
//! purchase is "costs regulation now, buys prediction later", would have somewhere real to
//! bite. If ordinary life clears that bar easily, play has to justify itself some other way.
//!
//! This measures the status quo before anything is specified. It asserts nothing and
//! changes nothing.
//!
//! Run: `cargo run --release --example action_scale`

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, motor_scalar, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;

const LIFE: usize = 1_500;
const LEARN_BAR: i16 = 32; // sensorimotor::MIN_ACTION_FOR_LEARNING

struct Seen {
    label: String,
    ticks: usize,
    over_bar: usize,
    mag_max: i16,
    mag_mean: i32,
    gains: [i16; 4],
    still: usize,
}

fn watch(label: String, mut world: FieldWorld) -> Seen {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let (mut sum, mut n) = (0i64, 0i64);
    let (mut over, mut mx, mut still) = (0usize, 0i16, 0usize);

    for _ in 0..LIFE {
        let sens = world.sense();
        let r = being.step_embodied(&sens);
        let intent = intent_from(&r);
        let a = motor_scalar(&intent);
        world.actuate(&intent);

        let m = a.saturating_abs();
        if m >= LEARN_BAR {
            over += 1;
        }
        if m == 0 {
            still += 1;
        }
        mx = mx.max(m);
        sum += m as i64;
        n += 1;
        if !being.is_alive() {
            break;
        }
    }

    Seen {
        label,
        ticks: n as usize,
        over_bar: over,
        mag_max: mx,
        mag_mean: (sum / n.max(1)) as i32,
        gains: *being.forward_model.gains(),
        still,
    }
}

fn main() {
    println!("The being's own action scale — what does it have to learn from?\n");
    println!("  the forward model learns only from |action| >= {LEARN_BAR} (0.125)\n");

    let long = || FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let short = || FieldWorld::with((128, 128), (140, 140), (250, 250)).with_person(1, (20, 20));

    let lives = vec![
        watch("long crossing".into(), long()),
        watch("long + weathered".into(), long().with_weather(0, 2)),
        watch("beside its food".into(), short()),
        watch("beside food + weather".into(), short().with_weather(0, 2)),
    ];

    println!(
        "  {:<22} {:>6} {:>6} {:>8} {:>9} {:>28}",
        "life", "mean", "max", "at rest", "teachable", "learned gains"
    );
    println!("  {:-<22} {:->6} {:->6} {:->8} {:->9} {:->28}", "", "", "", "", "", "");
    for s in &lives {
        println!(
            "  {:<22} {:>6} {:>6} {:>7}% {:>8}% {:>28}",
            s.label,
            s.mag_mean,
            s.mag_max,
            s.still * 100 / s.ticks.max(1),
            s.over_bar * 100 / s.ticks.max(1),
            format!("{:?}", s.gains)
        );
    }

    let any_learns = lives.iter().any(|s| s.over_bar > 0);
    let all_learn_often = lives.iter().all(|s| s.over_bar * 100 / s.ticks.max(1) > 25);

    println!("\n  Does ordinary life teach this being its own body?");
    if !any_learns {
        println!("    NO. Not one action in any life reached the learning bar, so the forward");
        println!("    model's gains are still zero at death: the being never learned what its");
        println!("    own movement does to its senses. That is a real hole, and it is exactly");
        println!("    the hole play is shaped to fill.");
    } else if all_learn_often {
        println!("    YES, routinely — every life clears the bar on more than a quarter of its");
        println!("    ticks. Play cannot justify itself as 'the only way the body gets learned';");
        println!("    it would have to earn its place on the *quality* of what it samples.");
    } else {
        println!("    UNEVENLY — it depends on the life. Play's case is then about which beings");
        println!("    are starved of contingency, not about the being in general.");
        for s in &lives {
            println!(
                "      {:<22} teachable on {:>3}% of ticks, gains {:?}",
                s.label,
                s.over_bar * 100 / s.ticks.max(1),
                s.gains
            );
        }
    }
}
