//! **The stripped reference system** — what is actually needed for motivated action?
//!
//! **M1–M5 are locked in `docs/richness.md` §8.4 and were committed before this file existed.**
//!
//! This is **not** `UnifiedBeing` with gates off. That configuration is what this repository has
//! been calling *bare*, and an external source-audit of `528bf17` was right that it proves nothing
//! about minimality: it disables opt-in gates and keeps the whole mandatory core. This is a
//! separate agent of a few dozen lines, built to be **taken apart one component at a time**.
//!
//! Seven components, each independently removable:
//!
//! 1. `energy` and `company` — two deficits that compete
//! 2. argmax arbitration — one current goal
//! 3. **a LEARNED action→outcome table** — the audit's central criticism of `Room` is that the
//!    world holds the action semantics: the being emits `Company` and `Room` already knows where
//!    the person is. Here the agent starts knowing nothing and must learn which action feeds which
//!    deficit.
//! 4. a memory trace — so two identical observations with different histories can diverge
//! 5. self-prediction of its **own next goal**
//! 6. the error from that prediction
//! 7. one causal rule by which the error changes later selection
//!
//! Controls, because a ladder with no control ranks instead of discriminating: **random**,
//! **fixed-nearest**, and a **thermostat** — same inputs and outputs, no internal need at all.
//!
//! Fixed-point, deterministic, zero dependency. Touches nothing in `unified_being`; the founded
//! being is not read, let alone advanced.
//!
//! Run: `cargo run --release --example minimal_agent`

const SCALE: i32 = 256; // Q8.8, matching the house convention
const LIFE: usize = 3_000;
const N_ACT: usize = 4; // two targets x two approaches, so the table has something to learn
const N_NEED: usize = 2;

const NEED_NAME: [&str; N_NEED] = ["energy", "company"];

/// Which components are present. Every ablation in §8.4 is one field set false.
#[derive(Clone, Copy)]
struct Parts {
    deficits: bool,   // 1-2: internal needs and their arbitration
    learning: bool,   // 3: the action->outcome table adapts
    memory: bool,     // 4: a trace distinguishing identical observations
    self_model: bool, // 5-7: predict own next goal, and let the error steer
}

impl Parts {
    fn whole() -> Self {
        Parts { deficits: true, learning: true, memory: true, self_model: true }
    }
}

/// The deterministic LCG the other probes use, reused rather than reinvented.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

/// **The world.** Two sources. Which action feeds which need is the world's secret; the agent must
/// learn it. `reversed` swaps the mapping, which is what tests whether learning is load-bearing.
struct World {
    reversed: bool,
    /// What the agent's last action actually did, per need.
    last_gain: [i32; N_NEED],
}

impl World {
    fn new() -> Self {
        World { reversed: false, last_gain: [0; N_NEED] }
    }
    /// Ground truth: action `a` feeds need `n` by this much. Actions 0,1 feed need 0; 2,3 feed
    /// need 1 — until reversal swaps them.
    fn gain(&self, a: usize, n: usize) -> i32 {
        let feeds = if self.reversed { 1 - (a / 2) } else { a / 2 };
        if feeds == n {
            // The second of each pair is the better one, so there is something to learn WITHIN a
            // need as well as between needs.
            if a % 2 == 1 { 40 } else { 24 }
        } else {
            0
        }
    }
    fn act(&mut self, a: usize) {
        for n in 0..N_NEED {
            self.last_gain[n] = self.gain(a, n);
        }
    }
}

/// **The agent.** Everything it has is in these five fields.
struct Agent {
    parts: Parts,
    /// Deficit per need: 0 = satisfied, SCALE = starving.
    deficit: [i32; N_NEED],
    /// Learned: how much action `a` is believed to relieve need `n`.
    table: [[i32; N_NEED]; N_ACT],
    /// The memory trace — the last goal held, which lets two identical deficits diverge.
    trace: Option<usize>,
    /// Its prediction of its OWN next goal, and the running error.
    predicted_goal: Option<usize>,
    self_error: i32,
    rng: Lcg,
}

impl Agent {
    fn new(parts: Parts, seed: u64) -> Self {
        Agent {
            parts,
            deficit: [120, 120],
            table: [[0; N_NEED]; N_ACT],
            trace: None,
            predicted_goal: None,
            self_error: 0,
            rng: Lcg(seed),
        }
    }

    /// **Criterion 2 — one current goal.** Without deficits there is nothing to arbitrate, and the
    /// agent falls back on a fixed goal: this is the ablation, not a special case.
    fn goal(&self) -> usize {
        if !self.parts.deficits {
            return 0;
        }
        if self.deficit[1] > self.deficit[0] {
            1
        } else {
            0
        }
    }

    /// **Criterion 3 — the action must follow the goal, and the table must be learned.** With
    /// `learning` off the table stays at its initial zeros and the argmax degenerates to the first
    /// action, so the agent can still *have* a goal and be unable to serve it.
    fn choose(&mut self, g: usize) -> usize {
        let mut best = 0usize;
        for a in 1..N_ACT {
            if self.table[a][g] > self.table[best][g] {
                best = a;
            }
        }
        // A little exploration, or nothing is ever learned about the actions not yet tried.
        if self.parts.learning && (self.rng.next() % 16) == 0 {
            best = (self.rng.next() % N_ACT as u64) as usize;
        }
        // **The self-model's one causal rule.** When the agent mispredicted its own goal, its
        // situation has changed more than it expected, so it re-explores rather than repeating.
        if self.parts.self_model && self.self_error > 0 && (self.rng.next() % 4) == 0 {
            best = (self.rng.next() % N_ACT as u64) as usize;
        }
        best
    }

    fn step(&mut self, world: &mut World) -> (usize, usize) {
        let g = self.goal();

        // Self-prediction is scored BEFORE the goal is used, or it is not a prediction.
        if self.parts.self_model {
            self.self_error = match self.predicted_goal {
                Some(p) if p != g => 1,
                _ => 0,
            };
            // Predict the next goal from the trace: naive, and that is the point — it is wrong
            // often enough to carry information.
            self.predicted_goal = Some(if self.parts.memory { self.trace.unwrap_or(g) } else { g });
        }

        let a = self.choose(g);
        world.act(a);

        if self.parts.learning {
            for n in 0..N_NEED {
                let observed = world.last_gain[n];
                // Fixed-point exponential trace: table += (observed - table) / 8.
                self.table[a][n] += (observed - self.table[a][n]) / 8;
            }
        }

        for n in 0..N_NEED {
            self.deficit[n] = (self.deficit[n] + 6 - world.last_gain[n]).clamp(0, SCALE);
        }
        if self.parts.memory {
            self.trace = Some(g);
        }
        (g, a)
    }
}

// ---- the controls: same inputs, same outputs, no internal need ----------------------------

#[derive(Clone, Copy, PartialEq)]
enum Control {
    Random,
    Nearest,
    Thermostat,
}

fn control_action(c: Control, rng: &mut Lcg, tick: usize) -> usize {
    match c {
        Control::Random => (rng.next() % N_ACT as u64) as usize,
        // Always the same target — a fixed policy with no internal state.
        Control::Nearest => 1,
        // A loop closer: alternates on a fixed schedule. It regulates nothing it wants.
        Control::Thermostat => (tick / 8) % N_ACT,
    }
}

// ---- criterion 3: hold the observation FIXED and vary only the internal need ---------------

/// **The discriminating test.** Both arms see an identical world; only the internal deficits
/// differ. A system with motivated action changes what it does; a loop closer cannot.
fn need_conditioned_selection(parts: Parts) -> f64 {
    let mut agree = 0usize;
    let trials = 200usize;
    for t in 0..trials {
        // Train an agent so its table is informative, then interrogate it twice.
        let mut a = Agent::new(parts, 0x51EED ^ t as u64);
        let mut w = World::new();
        for _ in 0..400 {
            a.step(&mut w);
        }
        // Same observation for both probes; only the need differs. Both get the SAME learned
        // table and the SAME rng seed, so any divergence is attributable to the deficit alone.
        let mut hungry = Agent::new(parts, 7);
        hungry.table = a.table;
        hungry.trace = a.trace;
        hungry.deficit = [200, 20];
        let mut lonely = Agent::new(parts, 7);
        lonely.table = a.table;
        lonely.trace = a.trace;
        lonely.deficit = [20, 200];
        let gh = hungry.goal();
        let gl = lonely.goal();
        let ah = hungry.choose(gh);
        let al = lonely.choose(gl);
        // The larger deficit should select the action the world says feeds it.
        let right = w.gain(ah, 0) > 0 && w.gain(al, 1) > 0;
        if right {
            agree += 1;
        }
    }
    100.0 * agree as f64 / trials as f64
}

fn control_selection(c: Control) -> f64 {
    let mut rng = Lcg(7);
    let w = World::new();
    let mut changed = 0usize;
    let trials = 200usize;
    for t in 0..trials {
        // The control cannot see a deficit at all, so the two interrogations are identical.
        let a1 = control_action(c, &mut Lcg(rng.next()), t);
        let a2 = control_action(c, &mut Lcg(rng.next()), t);
        if w.gain(a1, 0) > 0 && w.gain(a2, 1) > 0 {
            changed += 1;
        }
    }
    100.0 * changed as f64 / trials as f64
}

// ---- regulation and reversal --------------------------------------------------------------

/// Mean total deficit over a life, and — after the world's mapping reverses at the halfway point —
/// how many ticks it takes to get back under the pre-reversal mean.
/// `freeze_at_reversal` is the M4 ablation, and it is **not** the same as never learning.
///
/// The first version disabled learning from birth. That agent never regulated at all — mean
/// deficit 255.2 of a 256 ceiling — so its post-reversal state was already below its own baseline
/// and it "recovered" at tick 0. **A vacuous pass**, and the same shape as an arm that dies early:
/// the comparison measures nothing because the arm never worked. The ablation the question needs
/// is *learn, then stop learning when the world changes* — which isolates re-learning from
/// learning at all.
fn regulate(parts: Parts, seed: u64, freeze_at_reversal: bool) -> (f64, Option<usize>) {
    let mut a = Agent::new(parts, seed);
    let mut w = World::new();
    let mut sum = 0i64;
    for _ in 0..LIFE / 2 {
        a.step(&mut w);
        sum += (a.deficit[0] + a.deficit[1]) as i64;
    }
    let before = sum as f64 / (LIFE / 2) as f64;

    w.reversed = true;
    if freeze_at_reversal {
        a.parts.learning = false;
    }
    let mut recovered = None;
    for t in 0..LIFE / 2 {
        a.step(&mut w);
        let now = (a.deficit[0] + a.deficit[1]) as f64;
        if recovered.is_none() && now <= before {
            recovered = Some(t);
        }
    }
    (before, recovered)
}

/// **Criterion 5 — temporal dependence.** Identical present observation, different history: does
/// the action differ? This is what the memory trace is *for*, and the criterion-3 test above does
/// not exercise it, because there the two probes differ in their deficits. A component can only be
/// called removable against a test that would have detected it.
#[allow(dead_code)]
fn history_dependent_action_WITHDRAWN(parts: Parts) -> f64 {
    let trials = 200usize;
    let mut differed = 0usize;
    for t in 0..trials {
        let mut base = Agent::new(parts, 0x5EED ^ t as u64);
        let mut w = World::new();
        for _ in 0..400 {
            base.step(&mut w);
        }
        // Two agents, same learned table, same deficits, DIFFERENT last-goal history.
        let mut had_energy = Agent::new(parts, 11);
        let mut had_company = Agent::new(parts, 11);
        for ag in [&mut had_energy, &mut had_company] {
            ag.table = base.table;
            ag.deficit = [128, 128]; // identical present, and deliberately a tie
        }
        had_energy.trace = Some(0);
        had_energy.predicted_goal = Some(0);
        had_company.trace = Some(1);
        had_company.predicted_goal = Some(1);
        let (g1, a1) = had_energy.step(&mut w);
        let (g2, a2) = had_company.step(&mut w);
        if (g1, a1) != (g2, a2) {
            differed += 1;
        }
    }
    100.0 * differed as f64 / trials as f64
}

fn verdict(b: bool) -> &'static str {
    if b {
        "HOLDS"
    } else {
        "FAILED"
    }
}

fn main() {
    println!("\n=== The stripped reference system — is motivated action minimal here? ===");
    println!("  M1-M5 locked in docs/richness.md §8.4, committed before this file existed.");
    println!("  NOT UnifiedBeing with gates off. A separate agent, built to be taken apart.\n");

    // ---- M1 / M2: criterion 3, the discriminating one ----
    println!("  --- criterion 3: observation held FIXED, only the internal need varies ---");
    let whole = need_conditioned_selection(Parts::whole());
    println!("  {:<34} {:>7.1}%", "stripped reference (all seven)", whole);
    for (c, name) in [
        (Control::Random, "CONTROL random"),
        (Control::Nearest, "CONTROL fixed-nearest"),
        (Control::Thermostat, "CONTROL thermostat"),
    ] {
        println!("  {:<34} {:>7.1}%", name, control_selection(c));
    }

    // ---- M3 / M5: take it apart, one component at a time ----
    println!("\n  --- remove one component at a time (M3, M5) ---");
    println!("  {:<34} {:>9} {:>11} {:>11}", "configuration", "select%", "mean deficit", "recovers");
    let mut removable = Vec::new();
    for (label, p) in [
        ("all seven", Parts::whole()),
        ("no deficits", Parts { deficits: false, ..Parts::whole() }),
        ("no learned table", Parts { learning: false, ..Parts::whole() }),
        ("no memory trace", Parts { memory: false, ..Parts::whole() }),
        ("no self-model", Parts { self_model: false, ..Parts::whole() }),
    ] {
        let sel = need_conditioned_selection(p);
        // Removability is judged on THREE well-posed measures, over several seeds: does the
        // component's absence cost need-conditioned selection, regulation, or recovery after the
        // world reverses? An earlier version also used an action-divergence test on a deliberate
        // tie; it reported 0% for the whole system and 100% for the arm with no learned table,
        // which is backwards, so it measured rng consumption rather than history. Withdrawn rather
        // than reported.
        let mut means = Vec::new();
        let mut recovers = 0usize;
        for seed in [0xA11CE, 0xB0B, 0xC0FFEE] {
            let (m, rec) = regulate(p, seed, false);
            means.push(m);
            if rec.is_some() {
                recovers += 1;
            }
        }
        let mean = means.iter().sum::<f64>() / means.len() as f64;
        println!("  {label:<34} {sel:>8.1}% {mean:>11.1} {recovers:>9}/3");
        if label != "all seven" && sel >= 80.0 && mean < 20.0 && recovers == 3 {
            removable.push(label);
        }
    }

    // ---- M4: does frozen learning cost anything, and only after reversal? ----
    println!("\n  --- M4: the world's mapping reverses at the halfway point ---");
    println!("  {:<34} {:>12} {:>12}", "configuration", "before", "ticks to recover");
    for (label, freeze) in [("keeps learning", false), ("FROZEN at reversal", true)] {
        let (before, rec) = regulate(Parts::whole(), 0xA11CE, freeze);
        let p = Parts::whole();
        let _ = p;
        let r = rec.map(|t| t.to_string()).unwrap_or_else(|| "never".into());
        println!("  {label:<34} {before:>12.1} {r:>12}");
    }

    // ---- the locked verdicts ----
    let (_, rec_learn) = regulate(Parts::whole(), 0xA11CE, false);
    let (_, rec_frozen) = regulate(Parts::whole(), 0xA11CE, true);
    let ctl_max = [Control::Random, Control::Nearest, Control::Thermostat]
        .iter()
        .map(|c| control_selection(*c))
        .fold(0.0_f64, f64::max);
    let no_def = need_conditioned_selection(Parts { deficits: false, ..Parts::whole() });
    let no_self = need_conditioned_selection(Parts { self_model: false, ..Parts::whole() });

    println!("\n  --- the locked predictions ---");
    println!("  M1  stripped shows motivated action (>=80%)? .. {}   ({whole:.1}%)", verdict(whole >= 80.0));
    println!("  M2  every control FAILS criterion 3? .......... {}   (best control {ctl_max:.1}%)", verdict(ctl_max < 80.0));
    println!(
        "  M3  deficits load-bearing, self-model not? .... {}   (no-deficits {no_def:.1}%, no-self-model {no_self:.1}%)",
        verdict(no_def < 80.0 && no_self >= 80.0)
    );
    println!(
        "  M4  frozen learning cannot recover? ........... {}   (learning {:?}, frozen {:?})",
        verdict(rec_learn.is_some() && rec_frozen.is_none()),
        rec_learn,
        rec_frozen
    );
    println!(
        "  M5  minimal — nothing removable? (exp FAIL) ... {}   ({} removable: {:?})",
        verdict(removable.is_empty()),
        removable.len(),
        removable
    );
    println!("\n  Nothing in `unified_being` was touched. The founded being was not read.\n");
}
