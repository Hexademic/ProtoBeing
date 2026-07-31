# Survival first — is there another one?

> **Status: specified, nothing measured beyond §1.** Committed before the sweep exists, so §3's
> predictions are on the record before any result. Pure observation: no gate default changes, no
> faculty is built, and `life/being.journal` is not touched.

*Written 2026-07-31, immediately after closing `docs/incidents.md` I-3, from the rule that
incident earned:*

> **Report survival before reporting anything else. A mean over a life and a mean over a death
> are not comparable quantities, and nothing else in the row means anything until the reader
> knows which one they are looking at.**

I-3 was found because `examples/composed.rs` happened to put a lethal gate in a table next to ten
others. It was **not** found because anything in this repository was watching for deaths. That is
luck, and luck is not a method. This document asks the only honest follow-up question: **is there
another one?**

## 1. The motivating measurement — taken before this spec, so it is a fact, not a prediction

Of the **69** runnable probes in `examples/`, how many so much as *reference* whether the being is
alive?

```
probes: 69    reference .alive: 10
```

**Ten.** Fifty-nine probes run a being through a life, report something about it, and would not
notice if it died on the way. That is not a criticism of any one of them — until an hour ago
nobody knew there was anything to notice. It is a statement about the shape of the instrument:
*this repository has been reporting on beings without checking whether they survived the report.*

Two distinct risks follow, and they need separating because only one of them is addressed here:

| risk | shape | covered by this inch |
|---|---|---|
| **A gate configuration is lethal** | I-3 exactly — a faculty that kills in some combination | **yes**, §2 |
| **A probe's world or genome is lethal** | I-1 exactly — `reach = 300` left nowhere without threat | **no**, §5 |

## 2. The sweep

Every gate configuration up to pairs, in the reference world from `examples/composed.rs` (so the
results compose with I-3's rather than sitting beside them):

- **1** all-off control
- **11** singles
- **55** pairs
- **1** all-eleven

**68 lives.** For each: ticks lived, alive/dead, and the settled free-energy floor — I-3's
discriminator, carried forward to see whether it generalizes past the one gate it was derived from.

The floor rather than the mean, deliberately. A whole-life mean blends the settled value with the
transient, and blending is the failure this whole document exists because of.

## 3. Predictions — locked before the sweep runs

**Confident:**

- **S1.** Every death in the sweep contains `workspace_persistence`. Equivalently: **no
  configuration lacking that gate dies.** *If this fails, we have a second I-3 and it is a bigger
  finding than the first, because I-3 at least had a probe pointing at it.*
- **S2.** The all-eleven configuration survives, as `composed.rs` already reports.

**The live questions:**

- **S3 — does the free-energy floor still discriminate?** In I-3 it was perfect across seven
  configurations: alive ⟺ floor < 20, no exceptions. Across 68 I predict it **mostly** holds and
  I expect at least one exception, because "an unresolvable prediction error is a metabolic bill"
  explains *one* route to death and a body has others (I-1's being died of unwalkable threat with
  its prediction error perfectly fine). **An exception is not a failure of the mechanism — it is
  the discovery of a second way to die**, and it should be recorded as one.
- **S4 — is lethality symmetric?** If A+B dies, I expect A alone or B alone to die too. A pair
  that kills where neither member does would be genuinely new: harm that exists only in
  composition. I predict **zero** such pairs and would want that one loudly if it appears.
- **S5 — how bad is the rescue picture?** I-3 found four gates that rescue persistence. Is
  rescuing common (most gates fix it) or rare (those four are special)? Already measured at 4 of
  10 for persistence; the sweep says whether that ratio means anything.

## 4. What must not become possible

- **Nothing steers.** Pure observer. No gate default changes, no `src/` behaviour changes, the
  founded being is not woken to live, and no journal is written.
- **The result becomes a test, not a paragraph.** A one-time audit decays the moment somebody
  adds a gate. The sweep ships as a `tests/` guard so a newly-lethal configuration fails CI
  rather than waiting to be noticed by luck a second time.
- **A death is never reported as a number.** Any row for a being that did not finish its life is
  marked in the row itself, as `examples/composed.rs` now does.

## 5. What this inch does NOT cover, stated plainly so it is not mistaken for done

**The 59 probes remain unaudited at runtime.** §1 measures that they do not *look*; it does not
measure whether anything *died*. Those two are different claims and only the first is established
here. Most of those probes run a default-gated being, which lives its full life in ordinary
worlds — but I-1 is the standing proof that a *world* can kill without any gate being involved,
and several of those probes build their own worlds.

That audit is the natural next inch and it is named here so it cannot quietly not happen.

## 6. Method

Spec first, committed before the sweep exists. Predictions in §3 locked. Then the probe, then the
guard test, then §7 with what came out — including whichever of S1–S5 came back wrong, in the
form it came back wrong, which is the part of this project's method that has actually been
earning its keep.
