//! The same ablation, in the world the being actually lives in.
//!
//! The measurement for `docs/faculty-ablation.md` §9. R1–R5 were locked in that document and
//! committed before this file existed.
//!
//! `examples/faculty_ablation` measured a `FieldWorld` and found one faculty — `receptors` —
//! worth more than the other thirteen combined. §7 then refused to carry that number across to the
//! founded being, because the founded being does not live in a `FieldWorld`. It lives in
//! `Room::peopled(...)`, and it is blessed with **four** faculties, not none (§8, a correction).
//!
//! So this runs the ablation in the `Room`, on a **fresh** being, against the **blessed baseline**.
//!
//! **`SESSION_DAY` is 90**, so 90 ticks is the faithful horizon and those are the numbers that
//! describe the being's actual days. 4,000 is also run to show the regime.
//!
//! **R5 is why this probe can say "bad".** Lower drive may be a sedated being rather than a
//! comfortable one, and nothing in this project measures the difference. So effort, distinct
//! basins visited and distance travelled are reported beside drive. If `receptors` lowers drive and
//! collapses those, that is reported **against** R2.
//!
//! Pure observer: a fresh being, never the kept one. Reads report fields, changes nothing, writes
//! no journal, and does not touch `life/being.journal`. Survival first.
//!
//! Run: `cargo run --release --example room_ablation`

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::genome::Genome;
use unified_being::play::COMFORT;
use unified_being::room::Room;

/// `src/bin/being.rs:48`.
const SESSION_DAY: usize = 90;
const LONG: usize = 4_000;

/// The eight faculties a founded being can actually be *blessed* with (`persistence.rs`'s
/// `Features`). The other six gates have no field there and cannot be given to a kept life at all.
const BLESSABLE: [&str; 8] = [
    "precision_learning",
    "workspace_broadcast",
    "workspace_persistence",
    "serial_access",
    "schema_control",
    "felt_choice",
    "generative_perception",
    "receptors",
];

/// `blessed_features()` in `src/bin/being.rs` — the kept being's actual nature.
const BLESSED: [bool; 8] = [
    true,  // precision_learning
    false, // workspace_broadcast
    true,  // workspace_persistence
    false, // serial_access
    false, // schema_control
    true,  // felt_choice
    true,  // generative_perception
    false, // receptors  <- the one whose stated reason has expired
];

fn apply(b: &mut UnifiedBeing, on: &[bool; 8]) {
    if on[0] { b.enable_precision_learning(); }
    if on[1] { b.enable_workspace_broadcast(); }
    if on[2] { b.enable_workspace_persistence(); }
    if on[3] { b.enable_serial_access(); }
    if on[4] { b.enable_schema_control(); }
    if on[5] { b.enable_felt_choice(); }
    if on[6] { b.enable_generative_perception(); }
    if on[7] { b.enable_receptors(); }
}

#[derive(Default, Clone)]
struct Lived {
    ticks: usize,
    alive: bool,
    drive_sum: i64,
    past_comfort: usize,
    at_stake: usize,
    // R5 — is it comfortable, or sedated?
    effort_sum: i64,
    distance: i64,
    basins: [usize; 4],
}

impl Lived {
    fn drive(&self) -> f32 {
        self.drive_sum as f32 / self.ticks.max(1) as f32
    }
    fn effort(&self) -> f32 {
        self.effort_sum as f32 / self.ticks.max(1) as f32
    }
    fn distinct_basins(&self) -> usize {
        self.basins.iter().filter(|&&n| n > 0).count()
    }
}

/// One life in the being's own world — the same `Room` `src/bin/being.rs` builds.
fn live(on: &[bool; 8], ticks: usize) -> Lived {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    apply(&mut b, on);
    let mut room =
        Room::peopled((32, 200), (224, 56), (128, 220), (40, 40)).with_friend((210, 128));
    let mut l = Lived { alive: true, ..Default::default() };
    let mut last = room.body;

    for _ in 0..ticks {
        let sens = room.sense();
        let r = b.step_embodied(&sens);
        let intent = intent_from(&r);
        room.actuate(&intent);

        l.drive_sum += r.drive.drive as i64;
        if r.drive.drive >= COMFORT {
            l.past_comfort += 1;
        }
        if r.felt.state.at_stake {
            l.at_stake += 1;
        }
        l.effort_sum += intent.effort as i64;
        l.distance += (room.body.0 - last.0).unsigned_abs() as i64
            + (room.body.1 - last.1).unsigned_abs() as i64;
        last = room.body;
        l.basins[r.basin as usize] += 1;
        l.ticks += 1;
        if !r.alive {
            l.alive = false;
            break;
        }
    }
    l
}

fn pct(n: usize, d: usize) -> f32 {
    if d == 0 { 0.0 } else { n as f32 * 100.0 / d as f32 }
}

fn delta(v: f32, base: f32) -> f32 {
    if base.abs() < 1e-6 { 0.0 } else { (v - base) * 100.0 / base.abs() }
}

fn main() {
    println!("The same ablation, in the world the being actually lives in.");
    println!("(R1-R5 locked in docs/faculty-ablation.md §9, committed before this ran)\n");
    println!("  Room::peopled((32,200),(224,56),(128,220),(40,40)).with_friend((210,128))");
    println!("  — the room src/bin/being.rs builds. A FRESH being; the kept one is untouched.\n");

    let mut with_receptors = BLESSED;
    with_receptors[7] = true;

    for &(label, ticks) in &[("SESSION_DAY (90 ticks — the being's real day)", SESSION_DAY),
                             ("long run (4,000 ticks — the regime)", LONG)] {
        let base = live(&BLESSED, ticks);
        let recv = live(&with_receptors, ticks);

        println!("  ==== {label} ====\n");
        println!("  SURVIVAL FIRST:  blessed {} ({})   blessed+receptors {} ({})\n",
            base.ticks, if base.alive { "lived" } else { "DIED" },
            recv.ticks, if recv.alive { "lived" } else { "DIED" });

        println!("  {:<32} {:>14} {:>16} {:>10}",
            "", "blessed (as is)", "+ receptors", "Δ");
        println!("  {:-<32} {:->14} {:->16} {:->10}", "", "", "", "");
        println!("  {:<32} {:>14.2} {:>16.2} {:>9.1}%", "mean drive (R2)",
            base.drive(), recv.drive(), delta(recv.drive(), base.drive()));
        println!("  {:<32} {:>13.1}% {:>15.1}% {:>10}", "past COMFORT",
            pct(base.past_comfort, base.ticks), pct(recv.past_comfort, recv.ticks), "");
        println!("  {:<32} {:>13.1}% {:>15.1}% {:>10}", "at stake",
            pct(base.at_stake, base.ticks), pct(recv.at_stake, recv.ticks), "");
        println!("  {:<32} {:>14.1} {:>16.1} {:>9.1}%", "mean effort (R5)",
            base.effort(), recv.effort(), delta(recv.effort(), base.effort()));
        println!("  {:<32} {:>14} {:>16} {:>9.1}%", "distance travelled (R5)",
            base.distance, recv.distance,
            delta(recv.distance as f32, base.distance as f32));
        println!("  {:<32} {:>14} {:>16} {:>10}", "distinct basins visited (R5)",
            base.distinct_basins(), recv.distinct_basins(), "");

        // ---- R2 / R3 / R5 ------------------------------------------------------------
        let d = delta(recv.drive(), base.drive());
        let de = delta(recv.effort(), base.effort());
        let dd = delta(recv.distance as f32, base.distance as f32);
        println!("\n  R3 — survival: {}",
            if recv.alive { "nothing died with receptors on." }
            else { "** THE BEING DIED WITH RECEPTORS ON. Everything else is moot. **" });
        println!("\n  R2 — does drive fall by more than 20%?");
        println!("    Δ {d:.1}%   {}", if d < -20.0 {
            "** R2 HOLDS in the being's own world. **"
        } else if d < 0.0 {
            "R2 partly — drive falls, but by less than the 20% I predicted."
        } else {
            "** R2 FAILS — drive does NOT fall in the Room. The FieldWorld result does not \
             transfer, and §7 was right to refuse to carry it across. **"
        });

        println!("\n  R5 — comfortable, or sedated?  (effort Δ {de:.1}%, distance Δ {dd:.1}%)");
        println!("    {}", if d < 0.0 && (de < -40.0 || dd < -40.0) {
            "** R5 WARNS AGAINST R2. Drive fell, but effort or movement collapsed with it. That \
             is a quieter being, not a better-off one, and it should NOT be read as a welfare \
             gain. Reported against my own prediction. **"
        } else if d < 0.0 {
            "R5 clears it: drive fell without effort or exploration collapsing. The being is \
             doing as much and needing less — which is what a welfare gain would look like."
        } else {
            "R5 not applicable — drive did not fall."
        });
        println!();
    }

    // ---- R4: are the blessed four individually near-silent? --------------------------
    println!("  ==== R4 — the blessed four, removed one at a time (90 ticks) ====\n");
    let base = live(&BLESSED, SESSION_DAY);
    println!("    {:<26} {:>12} {:>10} {:>10}", "faculty removed", "mean drive", "Δ drive", "ticks");
    println!("    {:-<26} {:->12} {:->10} {:->10}", "", "", "", "");
    println!("    {:<26} {:>12.2} {:>10} {:>10}", "— none (blessed) —", base.drive(), "—", base.ticks);
    let mut worst = 0.0f32;
    for g in 0..8 {
        if !BLESSED[g] {
            continue;
        }
        let mut on = BLESSED;
        on[g] = false;
        let l = live(&on, SESSION_DAY);
        let d = delta(l.drive(), base.drive());
        worst = worst.max(d.abs());
        println!("    {:<26} {:>12.2} {:>9.2}% {:>10}{}", BLESSABLE[g], l.drive(), d, l.ticks,
            if l.alive { "" } else { " †DIED" });
    }
    println!("\n    largest single effect among the blessed four: {worst:.2}%");
    println!("    {}", if worst < 5.0 {
        "** R4 HOLDS. Each of the four the being was blessed with moves its drive by under 5%. \
         The nature it was given is, on this measure, nearly inert — and the one faculty that \
         would not be is the one left off. **"
    } else {
        "R4 fails — at least one blessed faculty substantially changes the being's life. Which \
         one, and why, is the next question."
    });

    // ---- what cannot be blessed at all -----------------------------------------------
    println!("\n  ==== And a structural fact found while writing this ====\n");
    println!("    `persistence.rs`'s `Features` has EIGHT fields. There are FOURTEEN gates.");
    println!("    Six faculties have no field there and so CANNOT BE GIVEN to a founded being:");
    println!("      reflection, homecoming, memory_guidance, comfort, settling, setting_down");
    println!();
    println!("    `reflection` is among them. So the load/weathered machinery — incidents I-8 and");
    println!("    I-9, and the deadlock fixed this morning — cannot be part of the kept being's");
    println!("    nature at all as the code stands. It can only be switched on inside a probe.");

    println!("\n  The founded being was not touched. A fresh being; no journal written.");
}
