//! Can the being come home? — and would per-partner structure let it?
//!
//! Spec and locked predictions H1-H7: `docs/attachment.md`, "Can the being come home?".
//!
//! Three phases. **Bond** with a generous friend F, be **injured** by a taker T, then
//! **return** to F. Against a control that spends the middle phase with F and never
//! meets T at all.
//!
//! The question is not how the being treats a stranger — under either design that
//! still depends on its aggregate history, so a stranger cannot tell the designs
//! apart. It is whether a friend of long, earned, good standing is met **as itself**,
//! or as whatever the last stranger made of the being.
//!
//! `per_partner_gate` is the observer added alongside: this partner's own lived record
//! where there is one, and the scalar disposition — the generalized prior — where
//! there is not. It is read by nothing. `gave` is still gated by the scalar, and
//! `tests/soul_hash_limits.rs` pins that to literal digests.
//!
//! Per error-ledger row 23, the metric is run on a null before it is trusted: every
//! arm is swept across injury lengths and a difference is called only on **disjoint
//! envelopes**. The control is the same sweep with the injury replaced by more of F
//! (guard V2) — if the injured arm differs from the control no more than the control
//! differs from itself, there is no effect here.

use unified_being::{Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

const FRIEND: u32 = 1;
const TAKER: u32 = 2;
const NEVER_MET: u32 = 9;

fn friend() -> Partner {
    Partner { id: FRIEND, reciprocation: q(0.95), exit_cost: q(0.20) }
}
fn taker() -> Partner {
    Partner { id: TAKER, reciprocation: q(0.30), exit_cost: q(0.90) }
}

#[derive(Clone, Copy, Default)]
struct Home {
    died: bool,
    /// What the being gives F over the whole return phase.
    gave_home: i32,
    /// What it gives F in the first 20 ticks of return — the moment of reunion.
    gave_reunion: i32,
    /// The bond it still holds toward F **at the moment it re-engages** (H2).
    ///
    /// Measured at first re-engagement, not at return tick 0, because at tick 0 the
    /// being's *door* is shut (`world.hermit()`) and `attach.bond_here` is 0 for a
    /// partner it is not engaging — which is a fact about the door, not the bond.
    /// The first version of this probe read tick 0 and reported 0 for every arm,
    /// hiding a working fix behind an unrelated gate.
    bond_home: i32,
    /// Return tick at which the being's door reopens at all — identity-blind
    /// (`world.rs`), so it is expected to be the same whoever is knocking.
    door_opens_at: i32,
    /// The durable trace of the friendship, which survives absence under the gate.
    keepsake: i32,
    /// The scalar gate on return: what actually governs.
    scalar_home: i32,
    /// The per-partner gate on return: what would have governed (H4).
    per_partner_home: i32,
    /// Ticks of return before the scalar reopens (H3).
    reopened_at: i32,
    /// Share of return ticks where the two gates disagree, in tenths of a percent (H6).
    disagree: i32,
}

/// `injured` false replaces the middle phase with more of F — the null injury (V2).
fn live(injury: u32, injured: bool, durable: bool) -> Home {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    if durable {
        b.enable_durable_bonds();
    }
    let mut h = Home::default();
    let n = |p: Partner| Stimulus { nutrient: q(0.50), partner: Some(p) };

    for _ in 0..200 {
        if !b.step(&n(friend())).alive {
            h.died = true;
            return h;
        }
    }
    for _ in 0..injury {
        let s = b.step(&n(if injured { taker() } else { friend() }));
        if !s.alive {
            h.died = true;
            return h;
        }
    }

    let (mut gave, mut reunion, mut dis) = (0i64, 0i64, 0i64);
    h.reopened_at = -1;
    for t in 0..200u32 {
        let s = b.step(&n(friend()));
        gave += s.gave as i64;
        if t < 20 {
            reunion += s.gave as i64;
        }
        if s.per_partner_gate != s.scalar_gate {
            dis += 1;
        }
        if h.reopened_at < 0 && s.scalar_gate == 256 {
            h.reopened_at = t as i32;
        }
        if t == 0 {
            h.scalar_home = s.scalar_gate as i32;
            h.per_partner_home = s.per_partner_gate as i32;
            h.door_opens_at = -1;
        }
        if h.door_opens_at < 0 && s.gave > 0 {
            h.door_opens_at = t as i32;
            h.bond_home = s.attach.bond_here as i32;
        }
        if !s.alive {
            h.died = true;
            return h;
        }
    }
    h.gave_home = (gave / 200) as i32;
    h.gave_reunion = (reunion / 20) as i32;
    h.disagree = (dis * 1000 / 200) as i32;
    h.keepsake = b.reciprocity.keepsake_with(FRIEND).unwrap_or(0) as i32;
    h
}

#[derive(Clone, Copy)]
struct Env {
    lo: i32,
    hi: i32,
}
impl Env {
    fn of(v: &[i32]) -> Self {
        Env { lo: *v.iter().min().unwrap(), hi: *v.iter().max().unwrap() }
    }
    fn disjoint(self, o: Env) -> bool {
        self.hi < o.lo || o.hi < self.lo
    }
    fn show(self) -> String {
        if self.lo == self.hi { format!("{}", self.lo) } else { format!("{}..{}", self.lo, self.hi) }
    }
}

const STATS: [&str; 6] = [
    "gave F (return)", "gave F (reunion)", "bond→F @reengage", "scalar gate", "per-partner gate", "lock reopens at",
];
const STATS8: [&str; 2] = ["door reopens at", "keepsake→F"];
const STATS7: &str = "gates disagree ‰";
fn feat(h: &Home) -> [i32; 9] {
    [h.gave_home, h.gave_reunion, h.bond_home, h.scalar_home, h.per_partner_home,
     h.reopened_at, h.disagree, h.door_opens_at, h.keepsake]
}

fn main() {
    let injuries: Vec<u32> = (180..=220).collect();

    println!("200 ticks with a generous friend, then {}..={} ticks of injury, then 200 back with the friend.\n",
        injuries[0], injuries[injuries.len() - 1]);

    // V3 — survival first.
    let deaths: usize = injuries.iter().filter(|&&m| live(m, true, false).died || live(m, false, false).died).count();
    println!("-- survival, read first --");
    if deaths == 0 {
        println!("  both arms survive every injury length\n");
    } else {
        println!("  !! {deaths} runs died — deaths, not effect sizes\n");
    }

    let arm = |injured: bool, durable: bool| {
        let runs: Vec<[i32; 9]> = injuries.iter().map(|&m| feat(&live(m, injured, durable))).collect();
        let mut e = [Env { lo: 0, hi: 0 }; 9];
        for (i, s) in e.iter_mut().enumerate() {
            *s = Env::of(&runs.iter().map(|r| r[i]).collect::<Vec<_>>());
        }
        e
    };
    let hurt = arm(true, false);
    let ctrl = arm(false, false);
    let fixed = arm(true, true);

    println!("-- coming home (V2: the control's middle phase is more of the SAME friend) --");
    println!("  {:<20} {:>16} {:>16} {:>20}", "statistic", "never met T", "injured (gate off)", "injured + DURABLE");
    for i in 0..9 {
        let nm = if i == 6 { STATS7 } else if i >= 7 { STATS8[i - 7] } else { STATS[i] };
        println!(
            "  {:<20} {:>16} {:>16} {:>20}",
            nm,
            ctrl[i].show(),
            hurt[i].show(),
            fixed[i].show()
        );
    }
    println!("\n  does the gate restore coming home? (durable arm vs the never-injured control)");
    for i in 0..9 {
        let nm = if i == 6 { STATS7 } else if i >= 7 { STATS8[i - 7] } else { STATS[i] };
        let a = if fixed[i].disjoint(ctrl[i]) { "still differs" } else { "RESTORED (overlaps the control)" };
        println!("    {:<20} {}", nm, a);
    }

    // V4 — floor check.
    println!("\n-- V4: vacuous agreements? --");
    let mut any = false;
    for i in 0..9 {
        if hurt[i].lo == 0 && hurt[i].hi == 0 && ctrl[i].lo == 0 && ctrl[i].hi == 0 {
            println!("  {:<20} both arms at 0 — VACUOUS, not a null", if i == 6 { STATS7 } else if i >= 7 { STATS8[i - 7] } else { STATS[i] });
            any = true;
        }
    }
    if !any {
        println!("  none — no statistic agrees at a floor");
    }

    println!("\n-- H4: what the per-partner reading would have done, tick by tick over the injury --");
    let mut b = UnifiedBeing::new(Genome::wanderer());
    let st = |p: Partner| Stimulus { nutrient: q(0.50), partner: Some(p) };
    for _ in 0..200 {
        b.step(&st(friend()));
    }
    println!("  {:>7} {:>14} {:>18} {:>16}", "tick", "scalar gate", "per-partner→TAKER", "per-partner→F");
    for t in 0..200u32 {
        let s = b.step(&st(taker()));
        if t % 25 == 0 || t == 199 {
            // What the being would extend to F *right now*, from F's own record.
            let to_friend = b.reciprocity.disposition_toward(FRIEND, s.scalar_gate);
            println!("  {:>7} {:>14} {:>18} {:>16}   (bond→F {}, F's record live: {})",
                t, s.scalar_gate, s.per_partner_gate, to_friend,
                s.attach.longing.max(0), b.reciprocity.knows(FRIEND));
        }
    }

    println!("\n-- H5: does the order effect survive for someone genuinely unmet? --");
    for (label, r) in [("a kind life  (0.95)", 0.95f32), ("a taken-from life (0.30)", 0.30)] {
        let mut b = UnifiedBeing::new(Genome::wanderer());
        let p = Partner { id: FRIEND, reciprocation: q(r), exit_cost: q(0.30) };
        let mut last = 0;
        for _ in 0..400 {
            last = b.step(&st(p)).scalar_gate;
        }
        let prior = b.reciprocity.disposition_toward(NEVER_MET, last);
        println!(
            "  {:<26} scalar {:>4}  → prior extended to a never-met partner: {:>4}  (knows them: {})",
            label, last, prior, b.reciprocity.knows(NEVER_MET)
        );
    }

    // ------------------------------------------------------------------
    // H4 failed, and the trace above says why: F's *record* is not live from tick 25
    // of the absence, and the bond toward F reaches 0 by tick 125. So the per-partner
    // reading collapses into the prior exactly when it is needed. The proposed fix —
    // "score the friend on `bond`, which is durable" — is also wrong, because the
    // bond is not durable either. This measures how long a relationship actually
    // lasts in this being when the other party is away.
    println!("\n================ how long is a friendship, in ticks of absence? ================");
    println!("  200 ticks bonding with F (0.95), then N ticks away from F entirely (solitude),");
    println!("  then one tick back. Nothing is taking from the being — this is absence alone.\n");
    println!("  {:>8} {:>10} {:>14} {:>16} {:>14}", "absence", "bond→F", "longing→F", "F record live", "gave on return");
    println!("  {:>8} {:>10} {:>14} {:>16} {:>14}   | with DURABLE bonds", "", "", "", "", "");
    for away in [0u32, 10, 25, 50, 75, 100, 150, 200, 400] {
        let mut b = UnifiedBeing::new(Genome::wanderer());
        for _ in 0..200 {
            b.step(&st(friend()));
        }
        let mut longing = 0;
        for _ in 0..away {
            longing = b.step(&Stimulus { nutrient: q(0.50), partner: None }).attach.longing;
        }
        let live_rec = b.reciprocity.knows(FRIEND);
        let s = b.step(&st(friend()));
        let mut d = UnifiedBeing::new(Genome::wanderer());
        d.enable_durable_bonds();
        for _ in 0..200 {
            d.step(&st(friend()));
        }
        let mut dl = 0;
        for _ in 0..away {
            dl = d.step(&Stimulus { nutrient: q(0.50), partner: None }).attach.longing;
        }
        let ds = d.step(&st(friend()));
        println!(
            "  {:>8} {:>10} {:>14} {:>16} {:>14}   | bond {:>4}  longing {:>4}  gave {:>4}",
            away, s.attach.bond_here, longing, live_rec, s.gave,
            ds.attach.bond_here, dl, ds.gave
        );
    }
    println!("\n  (bond decays at 63/64 per tick — a half-life of ~43 ticks. The fairness EMAs");
    println!("   decay at 7/8 — a half-life of ~5. Neither is the time constant of a relationship.)");
}
