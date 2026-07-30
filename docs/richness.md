# World richness — giving the being something to be wrong about

> **Status: specified, nothing built.** Committed before the probe exists, so §4's predictions
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
