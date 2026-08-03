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
