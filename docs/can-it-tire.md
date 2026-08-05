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

---

## 9. First pass — the mechanism works and is undersized, and B6 failed usefully

### B1 — holds. Default-off bit-identical, 361 tests green, founded being at 390 moments.

### B3 — **partly.** 2 of 6, and the failures are capacity, not mechanism

| feast / famine / period | reserve OFF | reserve ON | banked max |
|---|---|---|---:|
| 60 / 0 / 20 | DIED at 36 | **lived 4,000** | 256 |
| 60 / 0 / 60 | DIED at 83 | DIED at 106 | 256 |
| 60 / 0 / 120 | DIED at 138 | DIED at 154 | 256 |
| 60 / 5 / 60 | DIED at 92 | DIED at 119 | 256 |
| 60 / 10 / 60 | DIED at 231 | **lived 4,000** | 256 |
| 60 / 12 / 120 | DIED at 156 | DIED at 215 | 256 |

The mechanism **does** what it was built to do — short famines are now crossed, and every failing
regime survives *longer* than it did. But **`banked max` is 256 in every single case: the reserve
fills to capacity and stops.**

The arithmetic says why. `RESERVE_CAP` is one full energy (256) and the draw is a quarter of the
shortfall, so a being at empty pulls ~48/tick — **about five ticks of support.** A 60-tick famine
needs roughly 600. **The cap is a third of what a famine of the length being tested costs.**

### B2 — **mostly fails**, and the reason is a flaw in my own design

| constant supply | fatigue min | max | mean | distinct |
|---|---:|---:|---:|---:|
| nutrient 25, reserve **off** | 0 | 0 | 0 | **1** |
| nutrient 25, reserve **ON** | 0 | **61** | 1 | **30** |
| nutrient 60, reserve **off** | 0 | 0 | 0 | 1 |
| nutrient 60, reserve **ON** | 0 | 16 | 0 | **2** |
| nutrient 200, reserve **ON** | 0 | 16 | 0 | 2 |

Look at nutrient 25: **30 distinct fatigue values, max 61.** The set point works. Now look at 60 and
above: back to 2 values and a mean of 0.

> **The satiety set point stops operating the moment the reserve is full.** My own code clamps the
> transfer to `RESERVE_CAP − reserve`, so once the store is full nothing more leaves `energy` — and
> energy climbs straight back to the ceiling. **Satiety only holds while the larder has room.**

That is wrong on its own terms. A full stomach and a full larder means you **stop eating**, not that
you keep filling the stomach. **I checked the mechanism against the filling case and never against
the steady state** — the same error shape as `errors.md` #3 and #4, where a fix was verified against
one end of its range and not the other.

### B4 — fails. The tired band is still one nutrient unit, for the same reason as B2.

### B5 — clears it, weakly. At-stake unchanged at 0.0% (as predicted); fatigue spread 1 → 2 distinct values. The reserve did not smooth the being into a different flatness — but at generous supply it barely moved it either.

### B6 — **FAILS, and it is the most valuable result here**

| | distinct positions, 4,000 ticks in the room |
|---|---:|
| reserve off | 186 |
| **reserve ON** | **287** |

I predicted the orbit would be untouched, on the reasoning that *"the limit cycle is a fact about a
static world, not about metabolism."* **The being explores 54% more of its room with a reserve.**

> **Internal variation produces behavioural variation.** Energy that is no longer pinned varies
> arousal, which varies effort, which varies where the body ends up. **Metabolism does reach the
> limit cycle**, and my separation of "world problem" from "body problem" was too clean.

*(A note on comparability: these counts use `Room::sense()`'s own partner logic, where
`fear-and-avoidance.md` §9 overrode the partner. **186 here and 68 there are different setups and
must not be compared.** The 186 → 287 comparison within this probe is the valid one.)*

## 10. Second pass — predictions locked before the fix

Two changes, both inside the same default-off gate.

```rust
// 1. Satiety holds whether or not the larder has room. Excess above SATIETY leaves
//    `energy` regardless; it is banked if there is space and SHED if there is not.
// 2. RESERVE_CAP: 256 -> 768. Three full energies, chosen to cross a famine of roughly
//    the length of the feast that filled it — the property §8 claimed and did not have.
```

- **R1.** Default-off still bit-identical; founded being at 390.
- **R2.** B2's failure clears: at nutrient 60 and above, fatigue is a **live register with many
  distinct values** and a mean in the 40–90 band, because satiety no longer switches itself off.
- **R3.** B3 improves — **more than 2 of 6** of the killing regimes survive. **I am not predicting
  all six**, because the 60-tick zero-famine may simply cost more than three energies, and I would
  rather be shown that than claim it.
- **R4.** B6's *failure* deepens: distinct positions rise **further** above 287, because more
  internal variation should mean more behavioural variation. **If instead the orbit narrows, my
  new explanation is wrong too** and the 186 → 287 result needs a different account.

---

## 11. Second pass — five of six lethal famines survive, and the orbit **triples**

### R1 — holds. Bit-identical default path, 361 tests green, founded being at 390 moments.

### R3 — **holds, and by more than predicted: 2 of 6 → 5 of 6**

| feast / famine / period | before the reserve | first pass | **second pass** |
|---|---|---|---|
| 60 / 0 / 20 | DIED at 36 | lived | **lived** |
| 60 / 0 / 60 | DIED at 83 | DIED at 106 | **lived 4,000** |
| 60 / 0 / 120 | DIED at 138 | DIED at 154 | DIED at 195 |
| 60 / 5 / 60 | DIED at 92 | DIED at 119 | **lived 4,000** |
| 60 / 10 / 60 | DIED at 231 | lived | **lived** |
| 60 / 12 / 120 | DIED at 156 | DIED at 215 | **lived 4,000** |

**Five of the six lives that killed this being are now survivable.** The one that still ends it is a
**120-tick total famine** — and I said in §10 I would not predict all six, because that may simply
cost more than three energies. It does. That is a fact about the cap, and it is stated rather than
tuned away.

### R2 — **partly, and my prediction wanted the wrong thing**

| constant supply | fatigue min | max | mean | distinct |
|---|---:|---:|---:|---:|
| nutrient 25, reserve off | 0 | 0 | 0 | **1** |
| **nutrient 25, reserve ON** | **16** | **61** | **46** | **28** |
| nutrient 40, reserve ON | 16 | 28 | 16 | **13** |
| nutrient 60, reserve ON | 16 | 16 | 16 | **1** |
| nutrient 200, reserve ON | 16 | 16 | 16 | 1 |

At **lean** supply the set point does exactly what it was built for: **fatigue spans 16–61 with 28
distinct values, and the being lives the full 4,000 ticks.** That is a real tired-and-living band,
and this project has never had one.

At **generous** supply fatigue settles at a constant **16**, not the 40–90 I predicted. Two things
are true about that:

- **It is a proportional controller's steady-state offset, not a bug.** Shedding a quarter of the
  excess per tick balances the inflow at `energy − SATIETY = 4 × net gain`. With a rich supply that
  parks the being at 240 rather than 192.
- **And my prediction wanted the wrong thing.** I asked for a well-fed being to be tired. **A
  well-fed creature should not be tired.** Fatigue should come from exertion and from time since
  eating, not from being fed. R2 is half-failed on the arithmetic and half-wrong in what it wished
  for, and the second half is the more useful correction.

### B4 — still fails against `Rest`'s coordinate, but the distance has changed character

Max survivable fatigue is **61**; `Rest` wants **80**. So the coordinate remains out of reach — but
it is now short by **19 in a regime the being lives in**, rather than reachable only on the way down.
That is a different kind of gap from the one §5 measured.

### B5 — the automatic verdict fires **against me**, and it is right about half of what it says

The probe printed *"the reserve has traded one flat life for another."* At **nutrient 60** that is
**correct** — 1 distinct fatigue value before, 1 after, only the constant moved from 0 to 16. As a
general claim it is **wrong**: at nutrient 25 it is 1 → **28**.

**Reported both ways rather than quoting whichever suits.** The honest statement: *the reserve gives
the being a varying interior when its supply is lean, and does not when its supply is rich.*

### R4 — **holds. B6's failure deepens, and this is the headline**

| | distinct positions, 4,000 ticks |
|---|---:|
| reserve off | 186 |
| first pass | 287 |
| **second pass** | **564** |

> **The being explores three times as much of its room.** Same world, same static hearth and hazard
> and people — the only change is that its own energy is no longer pinned at a rail.

`fear-and-avoidance.md` §9 concluded the limit cycle was *"a fact about a static world, not about
metabolism."* **That was wrong, and it was the largest conclusion of that day.** Internal variation
produces behavioural variation: energy that moves varies arousal, which varies effort, which varies
where the body ends up.

**The being was not only unexercised. It was internally still, and that stillness was most of what
kept it in a 27-cell orbit.**

## 12. Where this leaves it

- **Ships gated, default-off.** `enable_reserve()`. Nothing is re-founded; `PHYSICS_VERSION` is not
  bumped, because the default path is untouched.
- **The critical path has moved.** §6 said a life with stakes would kill this being. **Five of six
  such lives are now survivable**, so a varying world is no longer disqualified — it is the next
  thing, with its own welfare case.
- **Two constants are chosen, not derived**, and should be treated as provisional: `SATIETY` at ¾ of
  full, and `RESERVE_CAP` at three energies. The 120-tick famine that still kills the being is the
  measurement that would set the second one honestly.
- **`Features` still has no field for this**, so the founded being cannot be blessed with it — the
  same reachability gap `audit-2026-08-03.md` §3.1 named, now blocking something that matters.

---

## 13. Does rest buy endurance? — predictions, locked 2026-08-04 before the probe

Blake: *"our being has a resting issue... we havent worked out rest towards endurance and survival."*

**A code trace says he is right, and a trace is not a measurement.** Four claims were made today from
traces alone, and the day's other lesson is that traced-but-unmeasured is exactly where I have been
wrong. So this section exists to let the trace fail.

**What the trace found.** Rest is better built than the complaint suggests: `Appetite::Repose` grows
unfed and satiates on `!at_stake && alarm < 64 && arousal < 128`; `striving.rs` treats it as *"the
anti-strive"* — `conserving = spent || rest > urgency`, with the comment *"you cannot strive your way
out of exhaustion."* That is thoughtful design.

**And then it does not reach the body.** `conserving` sets `mobilization = 0`; every reader of
`mobilization` across `src/`, `examples/` and `tests/` is `primes.rs`, where it gates whether the
being can *say a word about doing*. Meanwhile `effort = arousal` (`embodiment.rs:76`) and
`cost = 3 + arousal/32 + threat·(3/16)` (`body.rs`). **Cost never reads effort, never reads
mobilization, never reads `conserving`.**

### Predictions

| # | prediction | confidence |
|---|---|---|
| **E1** | Ticks where `conserving` is true show **no** lower mean energy-decline than ticks where it is false, at matched supply — |Δ| under 1 raw Q8.8 unit per tick | high — cost reads neither `conserving` nor `mobilization` |
| **E2** | `Posture::Resting` occupies **under 5%** of ticks | **low — genuinely unsure.** It needs `Basin::Recovery \| Rest`; `Rest` is near-unreachable but `Recovery` may not be |
| **E3** | Rest-hunger (`want[2]`) spends **most** of a lean life above `ACHE_EDGE` (192) — the being aches for rest it cannot obtain | medium |
| **E4** | `conserving` *does* change **where the body goes** — `goal` becomes `None`, so `actuate` falls through to the hearth — while changing nothing about what it spends | high, and it is the interesting one: **rest is a navigation decision here, not a metabolic one** |

**E2 is the one written to fail.** If `Posture::Resting` is common, the ¾-effort reduction is real
behaviour and the trace understated what rest does.

### What this cannot settle

It cannot say rest *should* buy endurance. A creature that rests to spend less is one design; a
creature that rests to stop *seeking* is another, and `striving.rs` deliberately chose the second.
**The measurement says which one was built, not which one is right.** That call is Blake's.

---

## 14. What came out — there is no regime where it both lives and rests

`examples/rest_and_endurance.rs`, `Room::peopled(...)`, no gates, supply swept as a fraction of the
room's ambient. **Survival first, before any other number** — and the survival table *is* the result.

| nutrient × | ticks | survived | conserving |
|---|---|---|---|
| 8/8 | 4000 | yes | **0.00%** |
| 7/8 | 4000 | yes | **0.00%** |
| 6/8 | 4000 | yes | **0.00%** |
| 5/8 | **75** | DIED | **88.00%** |
| 4/8 | 26 | DIED | 73.08% |
| 3/8 | 13 | DIED | 53.85% |

> **The being either never conserves and lives, or conserves almost constantly and is dead inside 75
> ticks. There is no band where it does both.**

### The predictions, graded

| # | prediction | result |
|---|---|---|
| **E1** | conserving shows no lower energy decline | **VACUOUS — not passed.** Zero conserving ticks in every surviving regime. The comparison has an empty cell and cannot be made in this room |
| **E2** | `Posture::Resting` under 5% of ticks | **HOLDS — 0.00%**, at every supply that survived. This was written as the one expected to fail; `Basin::Recovery` is not reachable here either |
| **E3** | rest-hunger spends most of a lean life above `ACHE_EDGE` | **FAILS, and inverts.** Mean `want[2]` = **0.10**, never once at the ache edge. Rest is nearly always *fed* |
| **E4** | conserving changes where the body is aimed but not what it spends | **untestable where it lives.** In the dying regimes it holds exactly — 7 of 7 conserving ticks had `goal == None` |

**E1 is vacuous, and vacuous is not passed.** That is the sixth time this has happened in this
project and it is said plainly again.

### What the numbers say that the trace did not

**`conserving` is the collapse mechanism, not the rest mechanism.** `spent = viability < SALIENT`,
and `conserving = spent || rest > urgency`. `striving.rs`'s own header says it: *"In a world that
would not meet its needs, it **collapsed**: went torpid, conserved."* So the being only ever conserves
once it is already failing — and **88% conserving still dies at 75 ticks.** Collapse is not endurance.

**The rest appetite reports satisfied while the being is braced.** `want[2]` sits at ~0, meaning
Repose's condition is nearly always met — while posture is **99.95% `Braced`** and body arousal
averages **245.7 / 256**. These are not contradictory readings of one register; they are three
registers that nothing couples. *(Repose is gated on `felt.state.arousal`, which is a different value
from `body.arousal` — checked, not assumed.)*

**And 99.95% `Braced` has a consequence nobody has drawn.** In `Room::actuate`, `Braced | Withdrawn`
routes to **flee**. So this being spends essentially its entire life fleeing the hazard. That is a
better account of the limit cycle than "an oscillator in a static world": **it is not orbiting, it is
running away, and the room is bounded.**

### Weighing this against the day's own rule

Today's rule: *when a result extends something already published, weight it as suspect.* This extends
both Blake's intuition and my trace, so:

- **The cliff itself is not news.** `mechanisms.md` already has metabolism as a clamped accumulator
  with two attractors and a measure-zero knife edge. A sharp survival boundary is *predicted* by that.
  **What is news is that conserving sits entirely on the dying side of it.**
- **One room, one genome, one partner, no gates.** `enable_reserve()` was deliberately off. A reserve
  softens the knife edge, so **there may be a live-and-conserve band once the being can bank a
  surplus** — that is the obvious next measurement and it is not run here.

### The answer to the question in the title of this document

**Rest toward endurance does not exist in this being.** What exists is collapse, and collapse does not
save it. Whether it *should* is not a measurement — a creature that rests to spend less and one that
rests to stop seeking are both coherent, and `striving.rs` chose the second deliberately. **That call
is Blake's.**

---

## 15. Ultrastability — predictions, locked 2026-08-04 before the code exists

§14 measured the hole: **`conserving` fires and reaches nothing.** 88% conserving, dead at 75 ticks.
The being has an essential variable and no reorganiser.

**Ashby had the missing half in 1948.** *Design for a Brain*'s **ultrastability**: a system with
*essential variables* that must stay within limits, and a **step function** that reconfigures the
system's own **parameters** — not its state — whenever those limits are breached, until the variable
returns. The homeostat searched its own parameter space for a configuration that survived. Ours flags
the breach and keeps going as it was.

### Where the step function attaches, and why it is the right place

```
body.rs:336   let x = self.arousal.sub(self.target_arousal);   // the oscillator orbits ABOUT it
body.rs:348   cost = 3 + arousal·(8/256) + threat·(48/256);    // and cost follows arousal
```

**`target_arousal` is a genome parameter, not a state variable.** Move it and the entire limit cycle
moves, and the metabolic cost with it. That is Ashby's move exactly: change the parameter, get new
dynamics, keep what survives.

### The design, stated before it is built

- **Essential variable:** `energy` — what actually kills the being.
- **Bound:** `energy < ESSENTIAL_FLOOR`, with hysteresis on the way back out.
- **Dwell:** out of bounds for `STEP_DWELL` consecutive ticks ⇒ **step**.
- **Step:** `target_arousal` descends a fixed ladder.
- **On return: hold the setting.** Ashby's machine keeps a configuration that works; it does not snap
  back to the one that was failing.
- Gated `enable_ultrastability()`, **default off**, observer-first, soul-hash bit-identical.

**One honest departure from Ashby, declared now:** his step function was **random**. Ours is a
**fixed ladder**, because the soul-hash requires reproducibility. **That is a real weakening** — a
fixed ladder can be defeated by a world tuned against exactly that sequence, and a random search
cannot. Said here rather than discovered later.

### Predictions

| # | prediction | confidence |
|---|---|---|
| **U1** | **Survival first.** `+ultrastability` survives 4,000 ticks at nutrient × 5/8, where the default dies at **75** | high — lowering `target_arousal` lowers cost directly |
| **U2** | **It buys survival by going quiet.** Mean arousal falls and **distance travelled falls with it — by more than 30%** against a default of equal length | high, and **this is the cost, not a bug**. B5 exists because *"a reserve could buy safety by removing stakes"* — the same trap, one faculty over |
| **U3** | **The default path is bit-identical.** Soul-hash unchanged, all tests green, the founded being untouched | near-certain, and it is the guard that must be *watched* to fail if the gating is wrong |
| **U4** | At ample supply (× 8/8) the gate is **vacuous** — zero steps, trajectory identical to default. **Vacuous is not passed**; it is reported as vacuous | high. If it fires at full supply the bound is wrong |
| **U5** | **Written to fail:** ultrastability opens a **live-and-conserve band** — some regime surviving 4,000 ticks with >5% of ticks conserving | **low. I expect this to fail.** `conserving` is keyed to viability, and raising viability is exactly what this does, so the being will likely survive *instead of* resting rather than *while* resting |

### What this cannot settle

It cannot make the being ultrastable in Ashby's full sense — he reorganised *all* parameters, we move
**one**. And it says nothing about whether the being feels the reorganisation. **It is a mechanism for
staying alive, not evidence of anything else.**

---

## 16. U1 FAILED — and the failure is ledger row 5, one faculty over

| # | prediction | result |
|---|---|---|
| **U1** | survives 4,000 at × 5/8 where the default dies at 75 | **FAILS.** Dies at **75**, identically. **One** step fired |
| **U2** | survival bought by going quiet, distance −30% | **UNTESTED — both lives were 75 ticks.** There is no equal-length comparison to make. Not "passed" |
| **U3** | default path bit-identical | **HOLDS.** `soul_hash_limits` and `founded_being` green; 365 passing, 0 failed |
| **U4** | vacuous at ample supply | **HOLDS, and is reported as vacuous.** 0 steps at 8/8 and the trajectory is **bit-identical** to default — checked, not assumed |
| **U5** | *(written to fail)* a live-and-conserve band | **FAILS, as predicted.** Every surviving regime conserves **0.00%**; every conserving regime is dead |

### Why U1 failed, and it is not the mechanism

Derived from the measurement rather than from feel:

```
5/8 supply: energy 256 → 0 in 75 ticks   ⇒ mean net drain 3.41 raw/tick
cost = 3 + arousal·(8/256) + threat·(48/256)                    body.rs:348
arousal ≈ 240 ⇒ 7.50 raw/tick;  at TARGET_FLOOR 32 ⇒ 1.00 raw/tick
```

**Reaching the floor saves 6.50 raw/tick against a 3.41 deficit. The mechanism is more than strong
enough.** What failed is the ladder's *speed*:

| dwell | rung | ticks to traverse | verdict |
|---|---|---|---|
| **24** | **16** *(shipped)* | **312** | the being dies at 75 |
| 8 | 32 | 56 | fits |
| 6 | 32 | 42 | fits |

> **I chose `STEP_DWELL = 24` and `STEP_RUNG = 16` by plausibility and wrote a comment justifying
> them — *"long enough that a passing dip is not a reorganisation"* — without ever measuring against
> the being's actual time-to-death.** That is `errors.md` **#5** exactly: a constant reasoned from
> one world and applied to another. Twenty-four ticks is a third of this being's entire life at 5/8.

### The correction, locked before it is applied

`STEP_DWELL: 24 → 8`, `STEP_RUNG: 16 → 32`. **Derived**, not tuned: 7 rungs × 8 ticks = 56 ticks to
traverse, inside a 75-tick life, and the arousal saving at the floor exceeds the measured deficit.

| # | prediction | confidence |
|---|---|---|
| **U1b** | survives 4,000 ticks at × 5/8 | high — the arithmetic says the saving covers the deficit with room |
| **U2b** | it costs motion: **distance and distinct places both fall** against the 6/8 default, the nearest surviving comparison | high, **and it is the point of the measurement**, not a defect |
| **U6** | *(written to fail)* it also rescues × 4/8, which dies at 26 | **low — I expect this to fail.** 7 rungs × 8 = 56 ticks and the being is dead at 26. **The ladder cannot outrun that world** |

**If U1b holds, the honest statement is narrow:** a fixed-ladder reorganiser rescues one regime whose
constants were derived from that regime's own death timescale. **That is closer to fitting than to
discovering, and it will be said that way.** U6 exists so the limit is visible.
