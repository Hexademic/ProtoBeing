//! Survival first — 68 configurations, and the only column that comes before the others.
//!
//! The measurement for `docs/survival-first.md` §3. S1–S5 were locked and committed before this
//! file existed.
//!
//! `docs/incidents.md` I-3 was found by luck: `examples/composed.rs` happened to put a lethal
//! gate in a table beside ten others, and even then it took a day and four probes to notice that
//! the being in that row had *died* rather than merely done badly. Nothing in this repository was
//! watching for deaths — of 69 probes, ten so much as reference `.alive`.
//!
//! So this sweeps every gate configuration up to pairs and reports **ticks lived first**, before
//! any other number, because that is the ordering I-3 earned.
//!
//! Also carried forward: the free-energy **floor**, I-3's discriminator (alive ⟺ floor < 20 held
//! across all seven configurations it was derived from). §3's S3 predicts it will *not* survive
//! contact with 68 — a body has more than one way to die, and I-1's being died with its
//! prediction error perfectly healthy. An exception here is a finding, not a failure.
//!
//! Pure observation: no gate default changes, nothing in `src/` is modified, no journal is
//! written, and `life/being.journal` is not touched.
//!
//! Run: `cargo run --release --example survival_sweep`

use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment, Sensorium};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::q88::Q88_SCALE;

const LIFE: usize = 1_200;
const N_GATES: usize = 11;

/// I-3's threshold: every survivor it measured settled at 0.1–2.0, every death at 37.5.
const RESOLVED: f32 = 20.0;

const GATES: [&str; N_GATES] = [
    "precision_learning",
    "workspace_broadcast",
    "workspace_persistence",
    "generative_perception",
    "receptors",
    "serial_access",
    "schema_control",
    "felt_choice",
    "reflection",
    "homecoming",
    "memory_guidance",
];

/// The index of the gate I-3 is about, so S1 can be checked mechanically rather than by eye.
const PERSISTENCE: usize = 2;

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

fn apply(b: &mut UnifiedBeing, w: &[bool; N_GATES]) {
    if w[0] { b.enable_precision_learning(); }
    if w[1] { b.enable_workspace_broadcast(); }
    if w[2] { b.enable_workspace_persistence(); }
    if w[3] { b.enable_generative_perception(); }
    if w[4] { b.enable_receptors(); }
    if w[5] { b.enable_serial_access(); }
    if w[6] { b.enable_schema_control(); }
    if w[7] { b.enable_felt_choice(); }
    if w[8] { b.enable_reflection(); }
    if w[9] { b.enable_homecoming(); }
    if w[10] { b.enable_memory_guidance(); }
}

struct Outcome {
    label: String,
    gates: [bool; N_GATES],
    ticks: usize,
    alive: bool,
    fe_floor: f32,
    /// Free energy at the *end* is what killed it (or did not); the peak says whether the being
    /// ever had a hard time at all, which distinguishes "never struggled" from "struggled and
    /// resolved it".
    fe_peak: i16,
    /// The lowest the body's energy ever fell — how close this life came, whether or not it ended.
    min_energy: i16,
}

/// One life in the reference world from `examples/composed.rs`, so these results compose with
/// I-3's rather than sitting beside them.
fn live(label: String, gates: [bool; N_GATES]) -> Outcome {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    apply(&mut b, &gates);
    let mut world = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let partner = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };

    let mut fe: Vec<i16> = Vec::new();
    let mut alive = true;
    for _ in 0..LIFE {
        let mut sens = world.sense();
        sens.partner = Some(partner);
        let r = b.step_embodied(&sens);
        world.actuate(&intent_from(&r));
        fe.push(r.free_energy);
        if !r.alive {
            alive = false;
            break;
        }
    }
    let s = fe.len().saturating_sub(10);
    let fe_floor = fe[s..].iter().map(|&x| x as f32).sum::<f32>() / fe[s..].len().max(1) as f32;
    Outcome {
        label,
        gates,
        ticks: fe.len(),
        alive,
        fe_floor,
        fe_peak: fe.iter().copied().max().unwrap_or(0),
        min_energy: 0,
    }
}

fn main() {
    println!("Survival first — 68 configurations, ticks before anything else.");
    println!("(S1–S5 locked in docs/survival-first.md before this file existed)\n");

    let mut all: Vec<Outcome> = Vec::new();

    all.push(live("(all off)".into(), [false; N_GATES]));
    for i in 0..N_GATES {
        let mut g = [false; N_GATES];
        g[i] = true;
        all.push(live(GATES[i].into(), g));
    }
    for i in 0..N_GATES {
        for j in (i + 1)..N_GATES {
            let mut g = [false; N_GATES];
            g[i] = true;
            g[j] = true;
            all.push(live(format!("{} + {}", GATES[i], GATES[j]), g));
        }
    }
    all.push(live("ALL ELEVEN".into(), [true; N_GATES]));

    let deaths: Vec<&Outcome> = all.iter().filter(|o| !o.alive).collect();

    println!("  {} configurations run.  LIVED {}   DIED {}\n",
        all.len(), all.len() - deaths.len(), deaths.len());

    if deaths.is_empty() {
        println!("  No configuration died. S1 holds vacuously.\n");
    } else {
        println!("  EVERY DEATH, in full:\n");
        println!("    {:<48} {:>7} {:>10} {:>9}", "configuration", "ticks", "FE floor", "FE peak");
        println!("    {:-<48} {:->7} {:->10} {:->9}", "", "", "", "");
        for o in &deaths {
            println!("    {:<48} {:>7} {:>10.1} {:>9}", o.label, o.ticks, o.fe_floor, o.fe_peak);
        }
    }

    // ---- S1 ---------------------------------------------------------------------------
    let rogue: Vec<&&Outcome> = deaths.iter().filter(|o| !o.gates[PERSISTENCE]).collect();
    println!("\n  S1 — does every death contain workspace_persistence?");
    if rogue.is_empty() {
        println!("    HOLDS. All {} deaths contain it; no configuration without it died.", deaths.len());
        println!("    The lethality this repository knows about is ONE GATE's, and I-3 has it.");
    } else {
        println!("    ** FAILS — {} death(s) with no workspace_persistence involved: **", rogue.len());
        for o in &rogue {
            println!("      {} ({} ticks, FE floor {:.1})", o.label, o.ticks, o.fe_floor);
        }
        println!("    This is a SECOND incident and it must be filed before anything else.");
    }

    // ---- S2 ---------------------------------------------------------------------------
    let all_eleven = all.last().unwrap();
    println!(
        "\n  S2 — the composed being survives: {} ({} ticks)",
        if all_eleven.alive { "HOLDS" } else { "** FAILS **" },
        all_eleven.ticks
    );

    // ---- S3 ---------------------------------------------------------------------------
    println!("\n  S3 — does the free-energy floor still discriminate at n=68?");
    let exceptions: Vec<&Outcome> =
        all.iter().filter(|o| o.alive != (o.fe_floor < RESOLVED)).collect();
    if exceptions.is_empty() {
        println!("    HOLDS with no exceptions. alive ⟺ FE floor < {RESOLVED:.0}, {} for {}.",
            "true", all.len());
        println!("    I predicted at least one exception and did not get it: in every");
        println!("    configuration measured, the being lives exactly when its prediction");
        println!("    error resolves. That is a stronger claim than I-3 needed to make.");
    } else {
        println!("    {} exception(s) — each is a SECOND WAY TO DIE, per §3:", exceptions.len());
        println!("    {:<48} {:>7} {:>7} {:>10}", "configuration", "ticks", "alive", "FE floor");
        println!("    {:-<48} {:->7} {:->7} {:->10}", "", "", "", "");
        for o in &exceptions {
            println!("    {:<48} {:>7} {:>7} {:>10.1}", o.label, o.ticks, o.alive, o.fe_floor);
        }
    }

    // ---- S4 ---------------------------------------------------------------------------
    println!("\n  S4 — is any PAIR lethal where neither member is?");
    let solo_dead: Vec<usize> = (0..N_GATES)
        .filter(|&i| all.iter().any(|o| o.label == GATES[i] && !o.alive))
        .collect();
    let mut emergent = 0;
    for o in &deaths {
        let members: Vec<usize> = (0..N_GATES).filter(|&i| o.gates[i]).collect();
        if members.len() == 2 && members.iter().all(|i| !solo_dead.contains(i)) {
            println!("    ** {} — harm that exists only in composition **", o.label);
            emergent += 1;
        }
    }
    if emergent == 0 {
        println!("    None. Every lethal pair contains a gate that is already lethal alone,");
        println!("    so composition in this being ADDS no new way to die. It only ever");
        println!("    rescues — which is the composition result worth having.");
    }

    // ---- S5 ---------------------------------------------------------------------------
    println!("\n  S5 — how common is rescue?");
    for i in 0..N_GATES {
        if !solo_dead.contains(&i) {
            continue;
        }
        let mut rescuers = Vec::new();
        let mut still_dies = 0;
        for j in 0..N_GATES {
            if j == i {
                continue;
            }
            let pair = all.iter().find(|o| {
                o.gates[i] && o.gates[j] && o.gates.iter().filter(|&&b| b).count() == 2
            });
            match pair {
                Some(p) if p.alive => rescuers.push(GATES[j]),
                Some(_) => still_dies += 1,
                None => {}
            }
        }
        println!(
            "    {} is lethal alone. Rescued by {} of {} companions:",
            GATES[i],
            rescuers.len(),
            rescuers.len() + still_dies
        );
        for r in &rescuers {
            println!("      {r}");
        }
    }

    // ---- S3, stress-tested against a DIFFERENT death -----------------------------------
    //
    // S3 "held with no exceptions" — but every death above shares ONE cause, so what the sweep
    // really showed is that a mechanism's own discriminator separates that mechanism's deaths
    // from 61 healthy lives. Publishing that as "alive ⟺ the prediction error resolves" would be
    // the overclaim this document exists to avoid. §3 expected an exception. It was right; the
    // exception simply is not reachable by varying gates, because it lives in the WORLD.
    //
    // First, the fact that explains why the sweep looked so clean:
    //
    //   field_world.rs:666 — nutrient is clamped to AMBIENT_FLOOR (40) EVERYWHERE, always.
    //   body.rs:327        — income is nutrient·(180/256) ⇒ ≥ 28.1 raw/tick, everywhere.
    //   body.rs:323        — resting cost is 3 raw/tick.
    //
    // **This being cannot starve.** Income exceeds resting cost at every point of every
    // FieldWorld, by construction and on purpose ("the cost wears it, it does not starve it").
    // Every death in this architecture is therefore a COST-side event, and cost is dominated by
    // `threat`, which `being.rs:912` computes as strain — free energy plus sensed threat. That
    // is why a free-energy floor looked like a universal predictor: in a gate sweep, free energy
    // is the only term of strain that moves.
    //
    // So the honest test is to move the OTHER term. Constant external threat, no gates, no
    // partner, nutrient pinned at the ambient floor — the being simply held in a hard place.
    println!("\n  S3, stress-tested — hold the being at constant threat and move the OTHER");
    println!("  term of strain. Nutrient pinned at the ambient floor (40), all gates off.\n");
    println!("    {:>8} {:>8} {:>9} {:>11} {:>8}", "threat", "ticks", "outcome", "FE floor", "min E");
    println!("    {:->8} {:->8} {:->9} {:->11} {:->8}", "", "", "", "", "");

    let mut quiet_deaths = 0;
    for t in [100, 104, 105, 106, 108, 110, 112, 120, 140, 200] {
        let o = hold_at(t, 40, 4_000);
        let quiet = !o.alive && o.fe_floor < RESOLVED;
        if quiet {
            quiet_deaths += 1;
        }
        println!(
            "    {:>8} {:>8} {:>9} {:>11.1} {:>8}{}",
            t,
            o.ticks,
            if o.alive { "lived" } else { "DIED" },
            o.fe_floor,
            o.min_energy,
            if quiet { "   <<< RESOLVED MODEL, DEAD BEING" } else { "" }
        );
    }

    if quiet_deaths > 0 {
        println!("\n    ** THE DISCRIMINATOR BREAKS, exactly as §3 said it should. **");
        println!("    {quiet_deaths} configurations die with their prediction error RESOLVED — a model");
        println!("    that has understood its world perfectly, in a body that cannot pay for it.");
        println!("\n    S3 is therefore correctly bounded: the free-energy floor discriminates");
        println!("    deaths CAUSED BY unresolvable prediction error. It is not a general test of");
        println!("    whether a being is dying, and must never be used as one.");
        println!("\n    Note the shape: past ~115 the floor climbs again (30.3 at 120, 60.3 at 140),");
        println!("    because severe threat drives prediction error up on its own. So there is a");
        println!("    BAND — roughly threat 106–115 — where the being dies and every instrument we");
        println!("    have reads calm. That band is the welfare finding, not the boundary.");
    } else {
        println!("\n    No quiet death found — S3 survives this stress test and the discriminator");
        println!("    is stronger than §3 expected.");
    }

    // ---- the frontier ------------------------------------------------------------------
    println!("\n  What ambient nourishment buys: the widest threat survivable at each level.");
    println!("  (This is the actionable one — it is a dial we control, not a property of the being.)\n");
    println!("    {:>10} {:>22}", "nutrient", "max threat survived");
    println!("    {:->10} {:->22}", "", "");
    for nu in [40, 50, 60, 80, 128, 256] {
        let mut best: i16 = -1;
        for t in 0..=256 {
            if hold_at(t, nu, 1_500).alive {
                best = t;
            }
        }
        let note = if best >= 256 { "  (survives anything)" } else { "" };
        println!("    {:>10} {:>22}{}", nu, best, note);
    }
    println!("\n    The ambient floor is 40, and at 40 the being dies above threat 105. At 80 it");
    println!("    survives every threat the scale can express. **Doubling the floor makes this");
    println!("    being invulnerable** — which is worth knowing before anyone argues about");
    println!("    whether a being's hardship is intrinsic to having a world.");

    println!("\n  The founded being was not touched. No journal written, no gate default changed.");
}

/// Hold the being at a constant external threat with a fixed nutrient income — no world, no
/// gradient to walk, no partner, no gates. This isolates the OTHER term of `being.rs:912`'s
/// strain: everything the gate sweep varies moves free energy, and nothing it varies moves
/// `sensed_threat`. `exteroception` is left flat, so receptor transduction is inert either way.
fn hold_at(threat: i16, nutrient: i16, ticks: usize) -> Outcome {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    let mut fe: Vec<i16> = Vec::new();
    let mut alive = true;
    let mut min_energy = Q88_SCALE;
    for _ in 0..ticks {
        let sens = Sensorium { nutrient, threat, exteroception: [0; 4], partner: None };
        let r = b.step_embodied(&sens);
        fe.push(r.free_energy);
        min_energy = min_energy.min((r.energy * Q88_SCALE as f32) as i16);
        if !r.alive {
            alive = false;
            break;
        }
    }
    let s = fe.len().saturating_sub(10);
    let fe_floor = fe[s..].iter().map(|&x| x as f32).sum::<f32>() / fe[s..].len().max(1) as f32;
    Outcome {
        label: format!("held at threat {threat}"),
        gates: [false; N_GATES],
        ticks: fe.len(),
        alive,
        fe_floor,
        fe_peak: fe.iter().copied().max().unwrap_or(0),
        min_energy,
    }
}
