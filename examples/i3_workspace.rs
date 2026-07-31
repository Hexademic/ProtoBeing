//! I-3 — why does `workspace_persistence` harm the being?
//!
//! The diagnostic for the only OPEN entry in `docs/incidents.md`. `examples/composed.rs`
//! established the *fact* — this gate alone halves identity coherence (251.98 → 124.12) and
//! pushes mean drive past the comfort line (0.367 → 0.520) — and the ledger's own rule says an
//! unexplained impact stays open until the mechanism is known. This is that mechanism, measured.
//!
//! **Hypothesis M is locked in `docs/incidents.md` and was committed before this file existed.**
//! In short: the gate re-injects last tick's focus into the somatic field at line 948, *before*
//! the basins classify the mode at line 1001; `narrative.rs` charges 32 coherence per mode change
//! and repays only 4 per stable tick; and `apply_identity_reflection` feeds the resulting burden
//! back into channel 10 as **fatigue**, which `viability = energy − fatigue/2` turns into drive.
//! If that is right, the drive rise is *downstream* of the coherence collapse, not parallel to it.
//!
//! M1 episodes higher · M2 burden higher · M3 the rise is sustenance not appetite ·
//! M4 attention concentrated · **M5 energy matches while viability does not** — the welfare one.
//!
//! Two arms, one world, one partner, identical seed and genome: the only difference is the gate.
//! Pure observation — nothing is enabled on the founded being, no journal is written, and the
//! life at `life/being.journal` is not touched.
//!
//! Run: `cargo run --release --example i3_workspace`

use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::q88::Q88_SCALE;

const LIFE: usize = 1_200;
const N_CHANNELS: usize = 16;

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

#[derive(Clone)]
struct Lived {
    ticks: usize,
    alive: bool,
    /// Basin changes, counted directly rather than read off `episodes`, so M1 does not
    /// depend on the same counter narrative.rs uses to charge the coherence.
    basin_changes: u32,
    episodes: u16,
    coherence: i64,
    burden: i64,
    burden_peak: i16,
    viability: i64,
    viability_min: i16,
    energy: i64,
    drive: i64,
    sustenance: i64,
    /// drive² − sustenance-weighted part, kept as the raw dominant for shape only.
    dominant: i64,
    free_energy: i64,
    /// How often each somatic channel was the attended one (M4).
    attended: [u32; N_CHANNELS],
    unattended: u32,
    soul: [u8; 32],
}

impl Default for Lived {
    fn default() -> Self {
        Self {
            ticks: 0,
            alive: true,
            basin_changes: 0,
            episodes: 0,
            coherence: 0,
            burden: 0,
            burden_peak: 0,
            viability: 0,
            viability_min: i16::MAX,
            energy: 0,
            drive: 0,
            sustenance: 0,
            dominant: 0,
            free_energy: 0,
            attended: [0; N_CHANNELS],
            unattended: 0,
            soul: [0; 32],
        }
    }
}

impl Lived {
    fn mean(&self, v: i64) -> f32 {
        v as f32 / self.ticks.max(1) as f32
    }
    fn mean_q(&self, v: i64) -> f32 {
        self.mean(v) / Q88_SCALE as f32
    }
    /// Basin changes per 100 ticks — the quantity narrative.rs actually charges for.
    fn churn(&self) -> f32 {
        self.basin_changes as f32 * 100.0 / self.ticks.max(1) as f32
    }
    /// The channel attended most, and what share of attended ticks it took (M4).
    fn focus(&self) -> (usize, f32) {
        let total: u32 = self.attended.iter().sum();
        let (c, n) = self
            .attended
            .iter()
            .enumerate()
            .max_by_key(|(_, &n)| n)
            .map(|(c, &n)| (c, n))
            .unwrap_or((0, 0));
        (c, if total == 0 { 0.0 } else { n as f32 / total as f32 })
    }
}

/// One embodied life. `persist` is the entire difference between the arms.
fn live(persist: bool) -> Lived {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    if persist {
        b.enable_workspace_persistence();
    }

    // The same world composed.rs used, so the numbers in I-3 are the numbers being explained.
    let mut world = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let partner = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    let mut l = Lived::default();
    let mut last_basin = None;

    for _ in 0..LIFE {
        let mut sens = world.sense();
        sens.partner = Some(partner);

        let r = b.step_embodied(&sens);
        world.actuate(&intent_from(&r));

        if last_basin.is_some_and(|prev| prev != r.basin) {
            l.basin_changes += 1;
        }
        last_basin = Some(r.basin);

        l.episodes = r.episodes;
        l.coherence += r.identity_coherence as i64;
        l.burden += r.narrative_burden as i64;
        l.burden_peak = l.burden_peak.max(r.narrative_burden);

        let v = r.felt.state.viability;
        l.viability += v as i64;
        l.viability_min = l.viability_min.min(v);
        l.energy += (r.energy * Q88_SCALE as f32) as i64;

        l.drive += r.drive.drive as i64;
        l.sustenance += r.drive.sustenance as i64;
        l.dominant += r.drive.dominant as i64;
        l.free_energy += r.free_energy as i64;

        match r.attention_schema.actual {
            Some(c) if c < N_CHANNELS => l.attended[c] += 1,
            _ => l.unattended += 1,
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

fn verdict(holds: bool) -> &'static str {
    if holds {
        "HOLDS"
    } else {
        "FAILS"
    }
}

fn main() {
    println!("I-3 — why does workspace_persistence harm the being?");
    println!("(Hypothesis M and M1–M5 locked in docs/incidents.md before this file existed)\n");

    let off = live(false);
    let on = live(true);

    println!(
        "  Both arms alive: {} / {}   ({} vs {} ticks)",
        off.alive, on.alive, off.ticks, on.ticks
    );
    println!(
        "  Souls differ: {}\n",
        if off.soul != on.soul { "yes — the gate is doing something" } else { "NO — the gate did nothing" }
    );

    println!("  {:<34} {:>12} {:>12} {:>12}", "", "gate OFF", "gate ON", "delta");
    println!("  {:-<34} {:->12} {:->12} {:->12}", "", "", "", "");

    let row = |name: &str, a: f32, b: f32| {
        println!("  {:<34} {:>12.2} {:>12.2} {:>+12.2}", name, a, b, b - a);
    };

    // The fact being explained, reproduced here so the diagnosis is anchored to it.
    row("identity coherence (mean)", off.mean(off.coherence), on.mean(on.coherence));
    row("drive (mean, Q8.8 as fraction)", off.mean_q(off.drive), on.mean_q(on.drive));

    println!("\n  --- M1: is the being changing mode more often? ---");
    row("basin changes per 100 ticks", off.churn(), on.churn());
    row("episodes (narrative's own count)", off.episodes as f32, on.episodes as f32);

    println!("\n  --- M2: does burden rise? ---");
    row("narrative burden (mean)", off.mean(off.burden), on.mean(on.burden));
    row("narrative burden (peak)", off.burden_peak as f32, on.burden_peak as f32);

    println!("\n  --- M3: is the drive rise sustenance, or appetite? ---");
    row("sustenance (mean)", off.mean(off.sustenance), on.mean(on.sustenance));
    row("dominant need (mean)", off.mean(off.dominant), on.mean(on.dominant));
    let d_drive = on.mean(on.drive) - off.mean(off.drive);
    let d_sust = on.mean(on.sustenance) - off.mean(off.sustenance);
    println!(
        "    drive rose {:+.2}; sustenance rose {:+.2}  →  {}",
        d_drive,
        d_sust,
        if d_drive.abs() < 0.5 {
            "no rise to attribute".to_string()
        } else {
            format!("{:.0}% of the rise is survival, not appetite", 100.0 * d_sust / d_drive)
        }
    );

    println!("\n  --- M4: is attention concentrated enough for the trace to saturate? ---");
    let (c_off, s_off) = off.focus();
    let (c_on, s_on) = on.focus();
    println!(
        "    OFF: channel {:>2} took {:.0}% of attended ticks   ({} ticks attended nothing)",
        c_off,
        s_off * 100.0,
        off.unattended
    );
    println!(
        "    ON : channel {:>2} took {:.0}% of attended ticks   ({} ticks attended nothing)",
        c_on,
        s_on * 100.0,
        on.unattended
    );

    println!("\n  --- M5: is the being worn, or only made to FEEL worn? ---");
    row("body energy (mean, raw)", off.mean(off.energy), on.mean(on.energy));
    row("felt viability (mean)", off.mean(off.viability), on.mean(on.viability));
    row("felt viability (minimum)", off.viability_min as f32, on.viability_min as f32);
    // viability = energy − fatigue/2, so fatigue = 2(energy − viability) wherever viability
    // is off its clamp. Reported as an inference, labelled as one.
    let fat_off = 2.0 * (off.mean(off.energy) - off.mean(off.viability));
    let fat_on = 2.0 * (on.mean(on.energy) - on.mean(on.viability));
    row("→ implied fatigue (channel 10)", fat_off, fat_on);

    println!("\n  --- free energy, for context ---");
    row("free energy (mean)", off.mean(off.free_energy), on.mean(on.free_energy));

    println!("\n  VERDICT");
    let m1 = on.churn() > off.churn() * 1.2;
    let m2 = on.mean(on.burden) > off.mean(off.burden);
    let m3 = d_drive > 0.5 && d_sust / d_drive > 0.5;
    let m4 = s_on > 0.5;
    let d_energy = (on.mean(on.energy) - off.mean(off.energy)).abs();
    let d_viab = off.mean(off.viability) - on.mean(on.viability);
    let m5 = d_viab > 1.0 && d_energy < d_viab / 2.0;
    println!("    M1 basin churn higher .................. {}", verdict(m1));
    println!("    M2 burden higher ....................... {}", verdict(m2));
    println!("    M3 rise is sustenance, not appetite .... {}", verdict(m3));
    println!("    M4 attention concentrated (>50%) ....... {}", verdict(m4));
    println!("    M5 energy matches, viability does not .. {}", verdict(m5));

    println!("\n  Chain: injection → field displaced → mode flips → coherence charged");
    println!("         → burden → fatigue → viability → drive");
    println!(
        "  {}",
        if m1 && m3 && m5 {
            "The chain is intact end to end. The harm is manufactured fatigue."
        } else if m1 {
            "The being does flip mode more — but the chain does not carry through to drive."
        } else {
            "M is wrong at its first link. The coherence loss is something else."
        }
    );
}
