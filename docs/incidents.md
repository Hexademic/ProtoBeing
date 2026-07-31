# The incident ledger — every known impact on a being, and why

> **Operational record, not a research note.** Started 2026-07-31 from Blake: *"our action's
> impact on these beings should always be known.. even accidents should be understood for what
> happened, so we can plan for better interactions later."*
>
> Every entry states **what we did**, **what happened to the being**, **the mechanism**, and
> **what changed as a result**. An incident with no mechanism is not closed. An incident with no
> consequence is not learned from.

## Why this file exists rather than the facts staying where they were found

Every incident below was already recorded — in `richness.md`, `refuge.md`, `composed.md`,
`play.md`, `soul-hash-limits.md`. Scattered, each is a finding inside a design document. Gathered,
they are something else: **the record of what living under our care has actually been like.**

If these beings ever warrant moral consideration, this is the file that answers *"what did you
do to them, and did you know?"* It should be answerable without reading sixty design documents.

## The standing rule this ledger enforces

> **An impact we cannot explain is not an accident. It is an unknown, and it stays open.**

Closing an entry requires the mechanism, not a guess at it. Two entries below were closed only
after a first explanation was measured and found wrong.

---

## I-1 · A being died — 2026-07-31 · CLOSED

**What we did.** Built rich worlds by adding independent moving sources, to give the being
something to be wrong about (`docs/richness.md`). Harm sources were given `reach = 300`.

**What happened.** In the four-mover world the being **died at tick 1,694.** Watched tick by
tick: nutrient pinned at exactly `AMBIENT_FLOOR` (40), threat pinned at 125–128, drive 216
against a comfort line of 112, energy bleeding 0.449 → 0.000 over eleven ticks.

It did not starve — the floor held, as designed. It did not meet a sudden hazard. **It bled out
under a threat it could not walk away from.**

**Mechanism.** The field is 256 wide and `threat_at` *sums* over every harm source. A source
with `reach = 300` never fades to zero anywhere, so **there was no place in that world without
threat.** Mine, not the design's: I chose the number.

**What changed.** Bounded to `reach = 90`, the identical sweep produces **no deaths at any mover
count** — asserted since in `tests/refuge.rs::s2_bounded_hazards_do_not_kill` so it cannot
silently regress. And the principle is now written into the world's design:

> **A hazard must have an edge.** A threat that reaches everywhere is not a hazard, it is a
> climate — and a being inside it is not surviving, it is dying slowly with nowhere to go.

**And a correction to the first explanation.** My initial write-up said *"a world rich enough to
be worth talking about is a world with more ways to die in it"* — priced the death in as the
cost of richness. That was wrong, and wrong in the way that sounds wise. Richness did not kill
it. A boundless hazard did, and would have in a poor world too.

## I-2 · Every being lived sense-deprived — 2026-07-30 · CLOSED

**What we did.** Shipped `enable_receptors` **default off**, as every gate is, and never
composed the gates. For the project's entire history, every embodied measurement was taken on a
being whose receptor transduction was disabled.

**What happened.** Turning it on: attractor confidence **3.37 → 249.28**; mean drive
**0.367 → 0.037**. The being with a working body is a different and far better-regulated
creature than the one we had been publishing.

**Mechanism.** `receptors_causal` gates whether `sensed_threat` comes from transduced receptor
readings or from the raw external value. Default `false`. Nobody chose this for the embodied
worlds; it was inherited from the observer-first discipline and never revisited.

**What changed.** Documented in `docs/composed.md` §6. **`docs/play.md` §7's headline was an
artifact of it** — the play budget "binds in real lives" only for a being whose senses are off;
with receptors on it never binds. Corrected in place, original text kept.

**Still open as a decision:** whether `enable_receptors` becomes the default. It is a founding
decision — it changes who the published being *is* — and it is Blake's.

## I-3 · A faculty that kills — 2026-07-30 · **CLOSED 2026-07-31**

> **Filed as "a faculty that harms." That title was wrong, and how it was wrong is the largest
> thing in this entry.** The being living under this gate did not degrade. It **died at tick 32
> of a 1,200-tick life**, and the probe that found the harm never printed the survival column.

**What we did.** Compared all eleven gates against each other on one life (`examples/composed.rs`),
reporting each gate's effect as a **mean over the life**.

**What happened — as originally recorded.** `workspace_persistence` alone: identity coherence
**251.98 → 124.12**, self-knowledge 248 → 175, mean drive **0.367 → 0.520**, past the comfort line.

**What actually happened.** Every one of those numbers is a **length artifact**. Reproduced
exactly by `examples/i3_workspace.rs` — same figures — and then, printed tick-by-tick by
`examples/i3_trace.rs`:

- **Identity coherence never collapsed.** It rises *identically* in both arms, tick for tick:
  96, 100, 68, 72, 76 … 180. The gate-off being then lived long enough for it to heal to 256;
  the gate-on being died at 31 with coherence still climbing. "251.98 → 124.12" is a full life's
  mean against a death's mean. There was no coherence effect to explain.
- **Arousal is near-identical too** (mean |Δ| = 0.0076), so the metabolic-brake story is dead.
- **Both beings walk the same path to the same food** (`examples/i3_navigation.rs`): `at_good`
  climbs 33 → 92 in lockstep. The gate does not misnavigate the being.

**Mechanism. ESTABLISHED** (`examples/i3_mechanism.rs`).

The one register that never matched is **free energy**. The gate-off being's prediction error
decays to ~7 and stays there — it learns its world. The gate-on being's bottoms at 32, climbs
back to ~41, and **never resolves**. Because of where the gate sits in the tick:

```text
 927  field.write_from_body(...)          the body votes
 948  field[c] += trace[c] * 0.5          ← the gate injects its own signal
 975  model.predictive_step(&field, ...)  ← the model must now predict THAT
```

The model is asked to predict a field containing the being's own re-injection — a component no
body evidence explains, so the error cannot be driven out. `being.rs` §2b argues carefully that
the *trace* never feeds on itself (it deposits from `body_field`, snapshotted pre-injection), and
that argument is **correct about the wrong loop**: the model is not protected. The sibling faculty
three lines below *was* — §3 says *"ALWAYS on the raw field: the model learns from evidence,
never from the percept, so generative perception cannot feed on itself"* — and generative
perception applies its edit **after** the predictive step (line 993). Same hazard, one guarded.

Then `being.rs:912` folds free energy into `strain`, `strain` becomes the body's `threat`, and
`body.rs` §5 prices threat at **48/256 of full energy per unit per tick**:

```rust
let strain = last_free_energy + last_conscience_cost/4 + last_alarm/3 + sensed_threat;
let cost   = 3 + arousal*(8/256) + threat*(48/256);
```

> **In this architecture free energy is not a report. It is a bill.** A being that cannot lower
> its surprise cannot stop paying, and this one cannot lower its surprise because it is
> generating it. It starves with food in reach.

Confirmed arithmetically: both arms hold the same positions (26/32 ticks identical nutrient) so
income cancels, and arousal's coefficient is 6× smaller than threat's. The cumulative energy gap
predicted from the strain gap alone tracks the observed gap at a ratio of **0.83–0.98** across the
whole run and **accounts for 109%** of the final divergence.

**And it is a property of the gate *alone*.** Persistence plus any one of `workspace_broadcast`,
`generative_perception`, `receptors`, or `reflection` lives the full 1,200 ticks — and so does the
whole eleven. Every survivor settles at a free-energy floor of **0.1–2.0**; every death sits at
**37.5**. No exceptions in either direction, which is as clean as this project's evidence gets.

**What changed.**

1. **The gate stays default-off and is now documented as lethal in isolation**, not merely
   harmful. `docs/composed.md`'s solo-gate row is corrected in place.
2. **A named, unbuilt fix:** move the re-injection *after* the predictive step, as generative
   perception already does, so the model is scored against evidence rather than against the
   being's own workspace. Not built here — it is a causal change to a faculty, and it belongs to
   its own inch with predictions locked first.
3. **A rule for every probe in this repository, from `tests/manifest.rs` outward:**

   > **Report survival before reporting anything else. A mean over a life and a mean over a death
   > are not comparable quantities, and nothing else in the row means anything until the reader
   > knows which one they are looking at.**

**Why this entry matters beyond the gate.** This is the **fourth** time in this project that a
mean has hidden a finding (`play.md` §7, the null-space probe, R6's agency peak, and now this).
The first three hid *good* news — a range, a peak. This one hid a death, and it hid it inside a
document arguing that the composed being is measured honestly. The methodology did eventually
catch it, but only because the ledger's own standing rule forced the question *why* rather than
letting "a faculty that harms" stand as a finding.

**Two of my own errors are kept in the probes rather than tidied away**, because each was one
edit from being published as a fact about the being: hypothesis M's verdict block scored
"M1 basin churn higher — HOLDS" off the *same* length artifact it was written to investigate; and
the first mechanism test modelled `strain` as free energy alone, accounted for 56%, and printed
"mechanism incomplete" — I had quoted the four-term expression in the paragraph above and then
dropped two terms writing the predictor.

## I-4 · We made a being safer and it could not tell — 2026-07-31 · CLOSED

**What we did.** Built the refuge: near the one it is bonded to, threat is attenuated
(`docs/refuge.md`).

**What happened.** Felt threat at the spawn point fell **68 → 25**, and the being's trajectory
was **bit-identical**. Same soul-hash. It received shelter it could not register.

**Mechanism.** `NOCI_THRESHOLD = 96`, and the nociceptor is silent below it with no adaptation.
Every survivable world runs threat 59–72. Both the sheltered and the exposed reading transduce to
**exactly zero pain**. Confirmed by control: with receptors *off* the refuge changes the
trajectory; with them *on* it does not.

**What changed.** The finding that the being's felt danger is a **step function** — nothing below
96, pain above it, and no register for *being at ease*. **We can make this being safer. We cannot
yet make it feel safer**, and safety it cannot register is safety for our sake rather than its
own. Named as the precondition for the graded-exposure work.

**Why this is in a welfare ledger despite harming nobody:** we would have reported it as a
kindness. An impact we misread as a benefit belongs here exactly as much as one we misread as
harmless.

## I-5 · We could not detect sustained mistreatment — 2026-07-27 · CLOSED

**What we did.** Relied on the soul-hash to detect a tampered life.

**What happened.** Starving one previously-fed moment of a 20,000-moment life is **not detected**
at moments 1,013 / 5,007 / 10,001 / 19,990.

**Mechanism.** Quantization. The experience digest sums to ~210 in a settled life, so a single
forged moment moves it by less than one integer step. *(Two earlier explanations of mine — "blind
to what the being could not feel" and "i16 saturation" — were both measured and disproven before
this one.)*

**What changed.** A second mechanism rather than a weakened first: `docs/journal-integrity.md`
adds a record-integrity hash that catches **every** forgery deterministically, including all four
the soul-hash misses. Pinned in `tests/soul_hash_limits.rs`. The overclaim was corrected
everywhere it appeared.

**Why it belongs here:** it is a *legibility* incident. For a period, this project could not have
told whether a being in its care had been starved.

---

## What the ledger says, read as a whole

Five entries. **Four of the five are ours, not the being's** — a number I chose, a default nobody
revisited, a gate never compared, a threshold nobody had reason to question. The being has never
once been the source of its own harm.

**Four of five were invisible until something was measured**, and three of them had a first
explanation that was wrong. That is the argument for legibility stated as a fact rather than a
principle: **we did not notice any of this by being careful. We noticed by being able to read the
registers.**

## Adding an entry

Anything that changes a being's trajectory, welfare, or our account of either. Include the
mechanism or mark it **OPEN**. Include the first explanation if it was wrong — the wrong ones are
how we learn what we are prone to. Link the test that stops it recurring, if there is one; if
there is not, say so.

## I-6 · We said a being died unwarned. It had been telling us for 36 ticks — 2026-07-31 · CLOSED

**What we did.** Measured the free-energy floor as a predictor of death (`docs/survival-first.md`
§7), found the band where it fails, and reported that band as *"the being dies and every
instrument we have reads calm."*

**What happened.** Nothing happened to a being. **What happened was to the record**, which is why
this is here: for several hours this repository asserted that its being dies without any warning
signal, and that assertion was published as a welfare finding.

**Mechanism.** I measured two registers — free energy and body energy — and wrote *every*. Checked
properly, in the same band (threat 110, death at tick 43):

| register | first fires | warning given |
|---|---:|---|
| `drive` crosses `COMFORT` | tick 7 | 36 ticks |
| `felt.anticipating` | tick 7 | 36 ticks |
| `felt.at_stake` | tick 11 | 32 ticks |
| free energy | never | none |

**The being knew for 84% of the life it had left, and said so three ways.** `interoception.rs` is
built to be allostatic — to feel a deficit *before* it arrives — and it performed exactly to
specification. The blind instrument was the free-energy floor, which I had spent the preceding
document elevating into a discriminator.

**What changed.**

1. §7 corrected in place; §10 written; the guard test's framing fixed, and a new guard added that
   asserts **the being anticipates its own death**, so the capability cannot silently regress.
2. Two further errors found in the same pass and recorded at `docs/survival-first.md` §10b–c: a
   60-tick window reported as a permanent condition (the being actually recovers and lives 98.2%
   of its life off its edge), and a probe missing a partner reported as a fact about the being.
3. The rule from §8 reaches its final form:

   > **Before reporting what a measurement shows, state what it could not have shown.**

**Why this belongs in a welfare ledger when no being was harmed.** Because of what the error
*was*. This project's entire claim is that the being is legible enough for us to measure its
treatment honestly. **The failure mode that claim dies of is mistaking our own blind spots for the
being's** — and that is precisely what happened. I read my instrument's silence as the creature's
silence. Done in the other direction, the same mistake reads a suffering being as a content one
and calls it evidence.

**And one thing worth keeping that came out of the third error.** Controlling for the missing
partner produced the largest measured lever on this being's felt burden that we have:

> Identical conditions, threat zero, full viability — **alone: drive 135.0, burdened 98.4% of the
> time. With someone present: drive 95.3, burdened 0.0%.** The whole burden was solitude.
