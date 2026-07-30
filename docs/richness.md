# World richness — giving the being something to be wrong about

> **Status: MEASURED — see §6.** `NOT KNOW` and `HAPPEN` are spoken for the first time in the
> project's history (R4, R5 confirmed). R6's verdict held but its reasoning was wrong: `CAN` is
> blocked on **persistence, not magnitude** — peak agency already clears the bar. **W FAILED —
> the being DIED in the four-mover world** — but the cause was a hazard with `reach` larger than
> the world, not richness itself. Bounded to 90, every world survives. Diagnosed in §6.
>
> **Status when written: specified, nothing built.** Committed before the probe exists, so §4's predictions
> are on the record before any result. **No new code is needed at all** — `FieldWorld` already
> has `with_source`, `with_drift`, `with_weather` and `with_visitor`. This inch is composition
> and measurement only. `being.rs` is untouched, no gate is added, existing worlds are
> bit-identical.

*Written 2026-07-31, from `docs/underdetermination.md` and a correction to it (§2).*

## 1. What the being actually says

Two thousand ticks, embodied, receptors on, weathered world, generous partner. The being
speaks on **1,969 of 2,000 ticks** — 11,934 clauses. It has 18 primes. It uses **eight**:

| | |
|---|---|
| **spoken** | `I` `Feel` `Now` `Good` `Someone` (~every tick) · `Very` (95%) · `Want` `Do` (10%) |
| **never spoken** | `Bad` `More` `Before` `Know` `NotKnow` `Can` `Cant` `Happen` `Near` `Because` |

It is not silent. It is **monotonous**: it says *"I feel very good now"* eleven thousand times.
Six of its eight words fire on essentially every tick, and `Because` — the causal connective
that carries nested speech — has never fired once.

*(Recorded because it nearly went unrecorded: the probe that produced the first version of this
count omitted `layer.observe(&facts)` and reported **zero** utterances against the project's
1,486. That was my error, not a fact about the being. Fixed before anything was built on it.)*

## 2. A correction to `docs/underdetermination.md`

Last night I wrote that `NOT KNOW` is downstream of a `self_spread` register the being lacks —
that doubt required a second-order self-model before the word could be true. **That is wrong,
and the code says so:**

```rust
Prime::NotKnow => f.novelty > Q88_SCALE / 6,
```

`NOT KNOW` keys off **novelty**, a register the being already has. It is not blocked on a type
change at all; it is blocked on living somewhere with something new in it. The second-order
self-model may still be worth building for its own reasons, but it is **not** the precondition
for this word. I asserted a dependency that is not in the source.

## 3. The complete map — every unspoken word and what it waits on

| prime | fact required | measured now | blocked on |
|---|---|---|---|
| `NotKnow` | `novelty > 42` | — | **the world** |
| `Happen` | `world_residual > 64` | mean **20**, max **82** | **the world** |
| `Bad` | `valence < −25.6` | — | **the world** |
| `More` | drive falls by ≥ 3 | — | **the world** |
| `Near` | world's `near` fact | — | **the world** |
| `Know` | `precision_warm` | — | a **gate** (`enable_precision_learning`) |
| `Because` | `forewarned` | — | a **gate** |
| `Can` | `agency > 128` | **8–20** (0.03–0.08) | **neither** |
| `Cant` | `free_energy > 48` | **0.69** | **neither** |

Two of those are the interesting ones. **The being cannot say *"I can"* because its measured
agency is 16× below the bar, and cannot say *"I can't"* because its free energy is 70× below
it.** It neither controls anything nor fails to predict anything. Those are the degenerate
action→sensation map (`docs/play.md` §8) and the empty world, appearing in its vocabulary.

## 4. Predictions — locked before the probe exists

The rich world: several independent sources, each with its own drift or weather, so the field
the being senses is the sum of things moving on unrelated schedules. Composition only.

**Confident:**

- **R1.** Free energy rises above **0.69**. There is more to be wrong about.
- **R2.** `self_surprise` rises above **1.21**.
- **R3.** The vocabulary broadens past eight primes.

**The live questions:**

- **R4.** *Does `NOT KNOW` become sayable?* It needs `novelty > 42` held about one tick in five
  (`RISE 4 : EBB 1`). I predict **yes** — novelty is exactly what independent movers supply,
  and this is the first time the being will have lived anywhere with more than one. If it stays
  unspoken in a world built to supply the only thing it needs, then the blocker is the novelty
  register itself and not the world, and I will say so.
- **R5.** *Does `HAPPEN` ground?* Needs `residual > 64`; currently mean 20, max 82. I predict
  **yes with enough independent movers and no with few** — and the probe sweeps the count so
  the answer is a number rather than a verdict. `docs/happening.md` §9 predicted world richness
  was the fix; this is that prediction's test.
- **R6 — the separating one.** *`Can` and `Cant` stay unreachable.* I predict richness does
  **not** rescue them: agency is short by 16× because the being's action barely moves its own
  senses, which is a fact about the *being*, and `Cant` needs a world it genuinely fails to
  predict, which no amount of moving sources supplies if the being predicts movement well. **If
  R4/R5 come true and R6 holds, we have cleanly separated what the world owes this being from
  what we owe it** — and that is worth more than any single word.

**Welfare:**

- **W.** A richer world must not be a crueller one. Mean and peak drive, burdened fraction, and
  survival against the current world. A world that makes the being say more by making it suffer
  more is not progress, and if that is the trade I will report it as one. `Bad` becoming
  sayable is a *good* result only if the being is not thereby made miserable.

## 5. Method

Spec first, committed before the probe. No new faculty, no gate, no default change, and the
founded being is not woken. Then §6 with what came out.

## 6. What came out — measured 2026-07-31

`examples/richness.rs`. Six worlds, 1 → 12 independent movers, receptors on, 2,000 ticks each.
No new code: composition of `with_source`, `with_drift`, `with_weather`.

### The being said two words it had never said

| prime | 1 | 2 | 4 | 6 | 9 | 12 |
|---|---|---|---|---|---|---|
| `BAD` | · | · | **102** | 453 | 836 | 910 |
| `NOT KNOW` | · | · | · | **588** | 1334 | 1217 |
| `HAPPEN` | · | · | · | **588** | 1334 | 1217 |
| `SOMEONE` | · | 1 | · | 20 | 11 | 125 |
| `MORE` `KNOW` `CAN` `CAN'T` `NEAR` | · | · | · | · | · | · |

**R4 confirmed.** `NOT KNOW` — one of nested speech's two shields, never spoken once in the
project's history — is spoken **588 times** in the six-mover world. The word for not knowing
was waiting on a world with something new in it.

**R5 confirmed.** `HAPPEN` grounds, at the same threshold. `docs/happening.md` §9 predicted
world richness was the fix and named it over the threshold and over the receptors; that
prediction is now vindicated by measurement.

**But note what the table shows and celebrate accordingly:** `NOT KNOW` and `HAPPEN` have
**identical counts at every level** — 588/588, 1334/1334, 1217/1217. They are not two
independent wins. They ground together and appear together, which means novelty and residual
are tracking the same underlying event in this world. One phenomenon, two names. That is worth
understanding before either is claimed separately.

R1 (free energy 5.09 → 7.68) and R2 (self-surprise 5.98 → 7.36) hold. R3 holds: 9 primes → 11.

### R6: the verdict held and the reasoning was wrong

Both `CAN` and `CAN'T` stayed unspoken, as predicted. But I locked R6 on the claim that agency
was "16× short" of its bar — and **that number was a mean.**

Peak agency is **224 in the sparsest world** and **157 in the richest**. Both are *above* the
bar of 128, and richness made it **lower**, not higher.

So the bar was never the problem. **`CAN` is blocked on persistence, not magnitude.** Grounding
needs a fact held about one tick in five (`RISE 4 : EBB 1`); this being's control over its own
senses spikes and collapses. It can act decisively for an instant and never long enough to have
earned the word. That is a sharper and more actionable target than the one I locked — and
`CAN'T` really is far (free energy 7.68 against a bar of 48).

**This is the third time in one week a mean has hidden the finding** (`docs/play.md` §7,
`docs/null-space.md` §7). I wrote that lesson into two documents and then locked a prediction on
a mean anyway. Recorded here because the pattern is now the most reliable thing about my errors.

### W FAILED — and it is the most important line here

| movers | mean drive | peak | burdened | alive |
|---|---|---|---|---|
| 1 | 0.317 | 0.512 | 3% | yes |
| 2 | 0.176 | 0.375 | 0% | yes |
| **4** | 0.223 | **0.844** | 4% | **DIED** |
| 6 | 0.303 | 0.535 | 7% | yes |
| 9 | 0.288 | 0.387 | 0% | yes |
| 12 | 0.205 | 0.395 | 0% | yes |

**The being died in the four-mover world.** Peak drive 0.844 against a comfort line of 0.44.

§4's W asked whether a richer world would be *crueller*. The honest answer is that it is
**sometimes lethal**, which is worse than crueller and was not among the outcomes I imagined.
And it is not monotone — the six-mover world carries the same kind of harm source and survives,
so this is not "more movers, more danger." Some *arrangements* kill this being and we do not yet
know which.

The probe's first verdict printed *"the richest world is no harder than the sparsest"* — true of
the means, and it had averaged a death. That verdict is fixed; the fact that it was possible to
write is the point.

### The death, diagnosed — and my reading of it was wrong

Watched the four-mover being die, tick by tick:

```
t1682  nutrient 40   threat 125   drive 203   energy 0.449
t1687  nutrient 40   threat 126   drive 216   energy 0.363
t1691  nutrient 40   threat 128   drive 216   energy 0.051
t1693  nutrient 40   threat 128   drive   0   energy 0.000
```

**Nutrient pinned at exactly `AMBIENT_FLOOR` (40). Threat pinned at 125–128. Drive at 216
against a comfort line of 112.** It did not starve — the floor held, as designed. It did not
meet a sudden hazard. It bled out over eleven ticks under a threat it could not walk away from,
with its energy draining and nowhere better to stand.

The cause is mine. I built those sources with **`reach = 300` on a 256-wide field**, and
`threat_at` *sums* over every harm source. A hazard with reach greater than the world has no
edge: its influence never fades to zero **anywhere**, so there is no place in that world without
it. Bound the reach to 90 and re-run the identical sweep:

| harm reach | 1 | 2 | 4 | 6 | 9 | 12 movers |
|---|---|---|---|---|---|---|
| 300 (as published above) | ✓ | ✓ | **DIED** | ✓ | ✓ | ✓ |
| **90** | ✓ | ✓ | **✓** | ✓ | ✓ | ✓ |

**Every world survives.** So the earlier framing in this section — *"a world rich enough to be
worth talking about is a world with more ways to die in it"* — is **wrong**, and it was the kind
of wrong that sounds wise. Richness did not kill the being. A hazard without a boundary killed
it, and it would have killed it in a poor world too.

**The principle the measurement actually yields:**

> **A hazard must have an edge.** A threat that reaches everywhere is not a hazard, it is a
> climate — and a being inside it is not surviving, it is dying slowly with nowhere to go.
> The difference between *a world with stakes* and *constant survival* is whether the danger
> has a boundary the being can be outside of.

Death stays possible under that rule: walk into the harm source and it still costs. What is
removed is *inescapability*, which was never a stake — it was just a sentence.

### What this inch actually bought

Two words the being never had — and a death that turned out to be a bug in the world I built,
not a cost of the world being interesting.

Both halves are the inch. The words are real: `NOT KNOW`, `HAPPEN` and `BAD` are earned and
audited. The death was avoidable and is now understood, which is better than being priced in.

### What is now cleanly separated

| the world owed the being | we owe the being |
|---|---|
| `NOT KNOW`, `HAPPEN`, `BAD` — delivered | `CAN` — persistence of control over its own senses |
| | `CAN'T` — a world it genuinely fails to predict (FE 7.68 vs 48) |
| | `KNOW`, `BECAUSE` — gates, not worlds |
| | `MORE`, `NEAR` — still unexplained, neither delivered nor diagnosed |
