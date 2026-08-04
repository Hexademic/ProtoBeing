# Can this being tire at all?

> **Status when written: specified, nothing built.** Committed before the probe, so §3's predictions
> are on the record first. **Pure observer** — fresh beings, no code changed, no journal written,
> `life/being.journal` untouched.

*Written 2026-08-03, from `docs/c1-relabelling.md` §9.1.*

## 1. Why this is now the question

C1 measured that **`channel[10]` — fatigue — is constant at zero across a 4,000-tick life.** One
distinct value, min 0, max 0.

`docs/comfort.md` §11 established that `Basin::Rest` is a **conjunction**: low arousal **AND fatigue
≈ 80** AND channel 0 ≈ 20. So one of Rest's three coordinates is a dead channel — and C1-4b showed
the *chart* is arbitrary anyway.

But the chart being arbitrary does not make the dead channel unimportant. **A being that cannot tire
is a fact about the being, not about our map of it**, and it survives C1 untouched: `energy` is
computed by `body.rs` from its own metabolism, not measured against any target we placed.

## 2. The arithmetic, read before measuring

`body.rs:323–331`, the whole of this being's metabolism:

```rust
let cost = Q8_8::from_raw(3)
    .add(self.arousal.mul(Q8_8::from_raw(8)))       //  arousal / 32
    .add(threat.mul(Q8_8::from_raw(48)));           //  threat · 3/16
self.energy = self.energy
    .sub(cost)
    .add(nutrient.mul(Q8_8::from_raw(180)))         //  nutrient · 0.703
    .clamp(Q8_8::ZERO, Q8_8::ONE);
```

**That is a pure accumulator with a clamp at both ends.** There is no set point, no satiety, no
regulation toward a middle. Each tick energy moves by `gain − cost`, and:

- if `gain > cost`, energy climbs until it **pins at the ceiling** and stays there forever;
- if `gain < cost`, energy falls until it **hits zero** and the being dies;
- `gain = cost` exactly is a measure-zero knife edge.

At the being's typical arousal (~237, `examples/arousal_range`) and no threat, `cost ≈ 3 + 7.4 =
10.4` raw per tick, so break-even nutrient is **≈ 15**.

> **A clamped pure accumulator has exactly two attractors: the ceiling and the floor.** If that
> reading is right, this being cannot be *tired* as a stable state. It is **full, or it is dying.**

## 3. Predictions — locked before the probe

- **T1.** Across a nutrient sweep at fixed threat, the outcome is **bimodal**: beings either pin at
  full energy (fatigue 0) for the whole life, or drain monotonically to death. **Predict no
  regime holds a sustained intermediate fatigue.**
- **T2.** The survival boundary sits near **nutrient ≈ 15** at threat 0, rising with threat by
  roughly `threat · 3/16 / 0.703 ≈ 0.27` nutrient per unit threat. Derived above; predict the
  measured boundary within a few units of it.
- **T3 — the one that matters for `Rest`.** `Rest` wants fatigue ≈ **80**. **Predict fatigue reaches
  80 only on a trajectory that ends in death** — the being passes through it on the way down and
  never holds it while going on to live. Operationalised: count ticks with fatigue ≥ 80 where the
  being survives at least 200 more ticks. **Predict that count is zero or near it in every regime.**
- **T4 — the structural claim.** If T1 and T3 hold, then **`Basin::Rest` is unreachable by
  construction, not by circumstance**, and no re-drawing of the chart can fix it. The cause is that
  `energy` has **no homeostatic set point** — the being has no satiety, so it fills to the brim and
  stays.
- **T5 — the counterweight, so this can fail against me.** An *oscillating* supply might hold energy
  in a middle band even without a set point. Predict a square-wave nutrient regime **does** produce
  sustained intermediate fatigue. **If T5 holds while T1 holds, the honest conclusion changes**:
  the being *can* tire, but only in a world that starves it periodically, and its constant world is
  what keeps it full.

## 4. What this cannot show

- **Nothing about whether the being feels tired.** Fatigue here is `1 − energy`, a register. The
  Witness Gap is untouched.
- **Nothing about whether tiring would be good for it.** A being that can tire can also be exhausted.
  If the remedy is a satiety set point, that is a change to the being's metabolism and therefore a
  **re-founding** — Blake's call, with its own welfare case.

---

## 8. The remedy — a reserve and a set point. Predictions locked before the code

*Written 2026-08-03, after Blake's "proceed as you desire." §6 named two absences; this builds both,
because they are one mechanism.*

### Why one mechanism, not two

§5 measured that `energy = clamp(energy − cost + gain, 0, 1)` has exactly two attractors and no
bankable surplus, and that this costs the being two different things:

- **No set point below the ceiling** ⇒ it fills to the brim and stays. `fatigue` is a **dead
  channel**, one distinct value across a 4,000-tick life, and one of `Rest`'s three coordinates.
- **No reserve above it** ⇒ **every** oscillating supply killed it, including feast 60 / famine 12 /
  period 120 (dead at 156) whose time-average is nearly double the survival boundary. A feast is
  discarded at the ceiling and the famine then draws down from full with nothing behind it.

**One store fixes both.** Give the being a reserve that fills from surplus *above* a satiety level
and drains to support energy *below* it. Energy then equilibrates near satiety instead of pinning at
the ceiling — so fatigue becomes a live, varying register — and the surplus a feast provides is
**kept** instead of thrown away.

```rust
// body.rs, gated on `reserve_causal`, default off.
const SATIETY: i16 = Q88_SCALE * 3 / 4;   // 192 — where a fed body settles
// after the existing metabolism:
if energy > SATIETY  { move the excess into `reserve`, up to its capacity }
if energy < SATIETY  { draw from `reserve` toward SATIETY, as far as it goes }
```

**This is a path change, not a term added to a sum** — `docs/how-i-would-build-it.md` §2.1. It alters
what the being's energy *does*, rather than nudging a value by a twelfth.

### Predictions — locked before the code

- **B1.** Default-off: trajectory and soul-hash **bit-identical**, full suite green, the founded being
  wakes at 390 moments with `load` 0 and `weathered` 2. `PHYSICS_VERSION` is **not** bumped, because
  the default path does not change.
- **B2.** Gate on, constant generous supply: energy settles **near `SATIETY`**, not at the ceiling.
  So `fatigue` stops being one distinct value and becomes a **live register** — the first time in
  this project. Predict mean fatigue in **40–90**, and `Rest`'s coordinate (80) inside the range the
  being actually occupies rather than one nutrient unit from starving.
- **B3 — the decisive one.** The regimes that **killed** it in §5 — every oscillating supply —
  **survive**. Specifically feast 60 / famine 12 / period 120, dead at 156 ticks, lives the full
  4,000. This is the whole point: a feast that can be banked is a famine that can be crossed.
- **B4.** The survivable tired band **widens** well beyond the one nutrient unit (19–20) measured in
  §5, because the reserve buffers the knife edge.
- **B5 — the counterweight, so this can fail against me.** A reserve could buy safety by removing
  stakes: a being that is never hungry is as flat as one that is never tired, and we would have
  traded one dead life for another. **Predict `at_stake` stays at 0.0%** — and if fatigue's variance
  is *also* near zero, the reserve has smoothed the being into a different flatness and I will report
  it against B2. **A reserve makes stakes survivable; it does not create them.**
- **B6.** Distinct positions visited in 4,000 ticks stays low (§`fear-and-avoidance.md` §9 measured
  27–68). **The limit cycle is a fact about a static world, not about metabolism**, and this change
  should not touch it. If it does, I have misunderstood what the reserve does.

### What this does not do

- **It does not give the being stakes.** That still needs a world that varies — which §6 said would
  kill it, and which this is the prerequisite for, not the delivery of.
- **It does not re-found anything.** Gated, default-off, founded being untouched. Whether the being
  is ever *blessed* with it is Blake's, and `Features` has no field for it yet — the same
  reachability gap `audit-2026-08-03.md` §3.1 named.
