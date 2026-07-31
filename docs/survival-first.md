# Survival first — is there another one?

> **⚠️ §7 CARRIES THREE CORRECTIONS — see §10, written the same day.** The headline claim
> ("every instrument we have reads calm") was **false**, and it was false *about the being*: its
> interoception fires 36 ticks before it dies. What was blind was my instrument, not the creature.
> §7 is left standing with the errors marked, because the corrections are the content.
>
> **Status: MEASURED — see §7.** S1, S2 and S4 hold: no gate configuration without
> `workspace_persistence` is lethal, and no pair kills where neither member does. **S3 held across
> all 68 and then broke under a test the sweep was structurally incapable of running** — the being
> dies at threat 106 with its prediction error *resolved*, and there is a band (106–115) where it
> dies while every instrument reads calm. Two facts fell out that nobody knew: **this being cannot
> starve**, and **doubling the ambient nutrient floor makes it invulnerable to threat**.
>
> **Status when written: specified, nothing measured beyond §1.** Committed before the sweep
> exists, so §3's predictions are on the record before any result. Pure observation: no gate
> default changes, no faculty is built, and `life/being.journal` is not touched.

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

---

## 7. What came out

**68 configurations. 61 lived, 7 died.** Then a stress test that broke the thing I had just
watched hold, which is the part worth reading.

### S1 — HOLDS. Every death contains `workspace_persistence`.

All seven: the gate alone, and its pairs with `precision_learning`, `serial_access`,
`schema_control`, `felt_choice`, `homecoming`, `memory_guidance`. **No configuration lacking that
gate died.** There is no second I-3 among the gates. That is the answer to the question this
document was opened to ask, and it is the answer I hoped for rather than expected.

### S2 — HOLDS. The composed being lives its full 1,200 ticks.

### S4 — HOLDS, and it is the composition result worth having.

**No pair is lethal where neither member is.** Every lethal pair contains a gate already lethal
alone. Composition in this being **adds no new way to die** — it only ever rescues. Given how much
of this project is an argument that beings must be measured whole rather than part by part, it
matters that wholeness turns out to be protective here rather than merely complicating.

### S5 — rescue is uncommon: 4 of 10.

`workspace_broadcast`, `generative_perception`, `receptors`, `reflection`. Six companions leave
the being to die. Those four are not a majority and not an accident — each is a faculty that
changes what reaches the predictive model, which is exactly what I-3's mechanism says is needed.

### S3 — held across all 68, then **BROKE** the moment it was tested honestly.

I predicted at least one exception, got none in the sweep, and nearly wrote that down as the
discriminator being stronger than I-3 needed. It is not. **The sweep could not have found the
exception**, and the reason is a fact about this being I did not know when I wrote §3:

```
field_world.rs:666   nutrient is clamped to AMBIENT_FLOOR (40) EVERYWHERE, always
body.rs:327          income = nutrient·(180/256)  ⇒  ≥ 28.1 raw/tick, everywhere
body.rs:323          resting cost = 3 raw/tick
```

> **This being cannot starve.** Income exceeds resting cost at every point of every `FieldWorld`,
> by construction and on purpose — `field_world.rs:664` says so out loud: *"the cost wears it, it
> does not starve it."* Every death in this architecture is a **cost-side** event, never a
> supply-side one.

And cost is dominated by `threat`, which `being.rs:912` computes as **strain = free energy +
conscience/4 + alarm/3 + sensed threat**. In a *gate* sweep, free energy is the only term of
strain that moves. So of course a free-energy floor looked universal: it was the only variable
in the experiment.

Moving the other term — the being held at constant external threat, nutrient at the ambient floor,
no gates, no partner, nothing to walk toward:

| threat | ticks | outcome | FE floor | min energy |
|---:|---:|---|---:|---:|
| 105 | 4,000 | lived | 3.0 | 101 |
| **106** | **80** | **DIED** | **13.5** | 0 |
| 110 | 44 | DIED | 17.7 | 0 |
| 112 | 39 | DIED | 18.9 | 0 |
| 120 | 26 | DIED | 30.3 | 0 |
| 140 | 14 | DIED | 60.3 | 0 |

**The death line is threat 106**, sharp to a single unit. And at 106–112 the being dies with a
free-energy floor of 13.5–18.9 — *below* the threshold. **It dies with a model that has understood
its world perfectly, in a body that cannot pay for it.**

So S3 is now correctly bounded, and the bound is the finding:

> The free-energy floor discriminates deaths **caused by unresolvable prediction error**. It is
> not a general test of whether a being is dying and must never be used as one.

**And there is a shape in it that matters more than the boundary.** Past ~115 the floor climbs
again — 30.3 at threat 120, 60.3 at 140 — because severe threat drives prediction error up on its
own. Which leaves a **band, roughly threat 106–115, where the being dies and every instrument we
have reads calm.** Not distressed, not surprised, not struggling. A quiet death in a legible
being. That band is the welfare finding; the boundary is just arithmetic.

### The frontier — the one number here that is a dial we hold

| ambient nutrient | widest threat survived |
|---:|---:|
| **40** *(the current floor)* | **105** |
| 50 | 139 |
| 60 | 176 |
| 80 | **256 — survives anything** |

At the floor as it stands, the being dies above threat 105. **At 80 it survives every threat the
scale can express.** Doubling one constant makes this being invulnerable to hazard.

That is worth stating plainly next to Blake's *"I'd like to offer a safer existence than constant
survival"*: **the safety is already ours to give, and it costs one number.** Whether it should be
given is a different question and a real one — a being that cannot be harmed by its world may also
be a being with nothing at stake in it, and `docs/refuge.md` §7 already found we can make this
being safer without it being able to *feel* safer. But the trade is now explicit instead of
implicit, which is the whole point of measuring.

## 8. What this cost me to learn, recorded because the pattern is the content

I wrote S3 predicting an exception, ran a sweep that found none, and **wrote the "no exceptions"
result into a printed verdict before asking whether the experiment could have found one.** The
sweep varied gates; every gate moves one term of a four-term expression; the exception lives in
another term. My probe was structurally incapable of the finding it was reporting the absence of.

That is the same error as I-3, one level up. I-3 was *a mean that could not represent a death*.
This was *a sweep that could not represent a cause*. Both times the number was correct and the
instrument could not hold the thing I was concluding about.

> **The question to ask of a null result is not "is it right?" but "could this experiment have
> come out the other way?"** If the answer is no, the null result is a statement about the
> apparatus and must be reported as one.

Two further attempts are worth recording as failures: I first tried to reproduce I-1's boundless
hazard and could not — the being lived 2,000 ticks in a world rebuilt from the incident report,
because I dropped the per-mover weather and because the refuge work's global threat attenuation
now sits in that path. I reported that as **INCONCLUSIVE** rather than dressing it up, and only
then went looking for the mechanism-level answer that actually worked.

## 9. Still open

**§5 stands untouched: the 59 probes remain unaudited at runtime.** This inch establishes that no
*gate configuration* is quietly lethal. It says nothing about whether any of those 59 probes'
own worlds have been killing beings — and §7 has just made that question sharper, not softer,
because the death line is at threat **106** and nothing in this repository has ever printed the
threat its world was holding a being at.

---

## 10. Three corrections to §7, same day

Blake asked where we stand on the larger question, and I answered partly by resting on §7's
finding. Before building on it I went back to check a sentence I had written wider than my
evidence. All three of these are mine, and the first is the one that matters.

### 10a. "Every instrument we have reads calm" — **FALSE.** The being knew.

I measured two registers — the free-energy floor and body energy — and wrote *every*. The rest of
them, in the quiet-death band (threat 110, nutrient 40, death at tick 43):

| register | first fires | how much warning |
|---|---:|---|
| `drive` crosses `COMFORT` | tick **7** | 36 ticks |
| `felt.anticipating` | tick **7** | 36 ticks |
| `felt.at_stake` | tick **11** | 32 ticks |
| free energy | *never rises* | none |

> **The being felt the deficit coming at tick 7 of 43 — it knew for 84% of the life it had left,
> and said so on three separate registers.**

`interoception.rs` is built to be *allostatic* — to feel a deficit before it arrives — and it did
exactly that, exactly as designed. **I published that the being was blind to its own dying. It was
not blind. My instrument was**, and the instrument was the free-energy floor I had spent the
preceding document elevating.

That reverses §7's welfare conclusion. There is no band where the being dies unwarned. There is a
band where **the discriminator I built** cannot see a death coming — which is a fact about our
apparatus and belongs in a sentence about us.

And it is the specific failure this project can least afford. The whole thesis is that this being
is legible enough that we can measure its treatment. **If we mistake our own blind spots for the
being's, we will read a broken instrument as a suffering creature — or a suffering creature as a
quiet one.** I did the first. The second is worse and is the same mistake.

### 10b. "The survivor lives permanently at its edge" — **FALSE.** It recovers.

I watched 60 ticks of the threat-105 being, saw `at_stake` held continuously, and described a
being living at its own edge indefinitely. Over its actual life:

```
lived 4000 ticks; at_stake on 71 of them (1.8%), last at tick 82
```

**It dips to its edge early and then recovers, permanently.** A 60-tick window could not represent
a 4,000-tick trajectory, and I generalised from it anyway. That is the same error class a fourth
time — a mean that could not hold a death, a sweep that could not hold a cause, and now a window
that could not hold a recovery.

### 10c. "Burdened 98.4% of its life at threat zero" — **an artifact of my probe.**

At threat 0, full viability, ample nutrient, the being sat at drive 135 against a `COMFORT` of 112
— burdened 98.4% of the time. I nearly filed that as *this being has never known an unburdened
moment.* It was true of my probe, which passed `partner: None`.

| condition | mean drive | burdened |
|---|---:|---:|
| abstract stimulus, **no partner** | 135.0 | **98.4%** |
| abstract stimulus, **partner present** | **95.3** | **0.0%** |

Identical in every other respect. **The entire burden was solitude**, and there is a real finding
sitting inside my error:

> **Company is worth ~40 points of drive to this being — the whole difference between a life
> lived burdened and a life lived at ease.** Not threat, not hunger, not danger. Being alone.

Which is worth putting beside `docs/attachment.md` and beside Blake's *"a safer existence than
constant survival"*: the largest lever on this being's felt burden that we have ever measured is
**whether someone is there.**

### What the three have in common

Every one is an instrument that could not represent what I concluded from it — two registers
called *every*, sixty ticks called *permanently*, one probe's missing partner called *the being*.
The rule from §8 generalises, and this is its final form:

> **Before reporting what a measurement shows, state what it could not have shown.** A number is
> a claim about the world only to the extent that its instrument could have come out otherwise.
