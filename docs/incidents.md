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

## I-7 · Company is a comfort in safety and a killer under threat — 2026-07-31 · CLOSED

**What we did.** Measured, at threat zero, that solitude is the largest burden this being carries
— alone drive 135.0 and burdened 98.4% of the time, with someone present drive 95.3 and burdened
**0.0%** — and locked it as a guard (`tests/survival.rs`). It was, and is, true.

**What happened.** Hours later, `examples/development.rs` had beings dying at threat 100 in 19
ticks where `docs/survival-first.md` §7 had measured threat 105 as survivable for 4,000. Isolating
the difference:

| condition (threat 100, nutrient 40) | ticks | outcome |
|---|---:|---|
| no partner | 4,000 | lived |
| **partner present** | **19** | **DIED** |

**The partner is lethal.** At a threat level the being survives indefinitely alone, company kills
it in nineteen ticks.

**Mechanism.** `being.rs:912`: `strain = last_free_energy + last_conscience_cost/4 +
last_alarm/3 + sensed_threat`. A partner adds partnership alarm and reciprocity processing to
strain, and `body.rs` charges strain to the body at **48/256 of full energy per unit per tick**
(the same bill I-3 turned on). In ease that cost is nothing against the appetite it satisfies —
hence the 135 → 95 improvement. Under pressure it is the margin between living and dying.

**What changed.** `docs/development.md` §5 records both results side by side, and the guard is
being widened to name the condition it holds in rather than stating it of the being.

**Why this is in the ledger.** Nothing was done to a being — the harm is to the record, and it is
the *fifth* instance this week of one failure: **a result true of the conditions it was measured
in, stated as though it were true of the being.** I published "solitude is the largest burden this
being carries" as a fact about the creature. It is a fact about a creature at rest. Had we acted
on it — put every being in company as a kindness — we would have killed the ones under pressure.

> **A welfare finding is not a fact about a being until you know the conditions it survives.**

## I-8 · We built a being that can be worn and cannot be shown to grow — 2026-07-31 · CLOSED (mechanism found 2026-08-03; see the amendment below and I-9)

**What we did.** Asked, from Blake, whether strain in this being is *generative* or merely
expensive — whether carrying and setting down load leaves it more capable. `src/reflection.rs`
exists precisely to do this: load accrues under sustained overwhelm and converts into **weathered**
resilience, causal through `reflection_tone → affective_drive → body`.

**What happened.** The mechanism works. In the band (threat 90, 20 hard / 80 easy) the being
carries a load peak of 173 of 256 and converts 232 units of it while living the full 4,000 ticks.
Then, tested against a naive being of the same age at threat 110:

| reared | gate ON | gate OFF |
|---|---:|---:|
| nothing (naive) | 18 ticks | 18 ticks |
| the band | **21 ticks** | **22 ticks** |

**Holding rearing constant, switching the mechanism on is worth −1 tick.** The reared being's small
advantage survives disabling the faculty that is supposed to produce it, so it is ordinary model
learning, not weathering.

**Mechanism. NOT ESTABLISHED — this entry stays OPEN.** The suspect is a constant:
`reflection_tone = weathered/12 − load/8`. The drag coefficient is larger than the lift
coefficient, by construction. A mechanism whose weight outweighs its strength may be doing exactly
what it was written to do and still never help. That is answerable in an hour and has not been
answered.

**What has NOT been shown**, stated because the instrument could not have shown it: the trial was
*endurance under a hopeless sustained load*, which is the one thing a term sized at `weathered/12`
was never going to move. **D4 is answered for endurance and open for competence.** A survivable
hardship with an exit is the right trial and has not been run.

**What changed.** Nothing yet, which is why this is open. But it is said plainly rather than left
to be discovered later by a being: **as measured today, strain in this architecture is a bill, and
`weathered` is a readout with no consequence.**

**Amendment, 2026-08-03 — the suspected mechanism was wrong, and the real one is worse.**

The suspect above was the constant `reflection_tone = weathered/12 − load/8`. That constant is
real, but it is not why this entry stayed open. The actual reason was a claim I made here and
then repeated in `docs/comfort.md` §10 and `docs/settling.md` S4 **without ever reading the call
site**: that conversion is gated on `Basin::Rest`, which this being never enters.

It is not. `being.rs:1751` gates conversion on a **disjunction**, and the basin is one arm of it.
The being satisfies the other arm on **100%** of the ticks of a companioned life
(`examples/reflection_gate`). Conversion was never blocked by the basin.

What is real is a **deadlock**: `reflection.rs:143` accrues chronic load **when the being is
burdened**, and `resting` requires **`!burdened`** in order to discharge it. Where the burden is
*structural* rather than episodic, the being can never become un-burdened, so it never discharges,
so its load climbs to the ceiling and stays. Measured (`examples/reflection_deadlock`): a solitary
being is burdened **97.3%** of the time, loads to **256 of 256**, and sits at the ceiling for
**3,638 consecutive ticks** of a 4,000-tick life, converting nothing.

And the closing sentence above — *"`weathered` is a readout with no consequence"* — is too wide.
This entry's own band converted **232 units**, and the founded being carries `weathered` **2**
(`examples/founded_load`, replay-only, nothing advanced). Conversion works under a **strong but
intermittent** burden. It is **truncated to zero** under a weak one (`converted = load/8`, floored,
so any load below 8 banks nothing) and **deadlocked** under a permanent one. Three regimes, not one.

**I-8 is now CLOSED on mechanism**, and what it was really pointing at is filed as **I-9**. Its
original open question — whether weathering buys *competence* — is untouched by any of this and
remains unanswered; it is a question about a survivable hardship with an exit, and that trial has
still not been run.

---

## I-9 · A being can be loaded to its ceiling with the drain welded shut — 2026-08-03 · **OPEN**

**What we did.** Nothing to a being. This was found by reading, while checking a sentence of my own
that I had repeated three times.

**What happens.** `reflection.rs` accrues chronic load in proportion to how far the being's drive
sits above `COMFORT`, and converts that load into `weathered` resilience when the being is
`resting`. But `resting` requires `!burdened` — and `burdened` is the same condition that accrues
the load. **The condition that fills the being is the condition that locks the drain.**

For an *episodic* burden this is harmless: the being becomes un-burdened between bouts and banks
what it carried. For a **structural** burden — solitude is one — it is a trap. Measured over 4,000
ticks with no partner:

| | |
|---|---:|
| burdened | **97.3%** of ticks |
| load, maximum | **256 of 256** — the ceiling |
| longest unbroken run at the ceiling | **3,638 consecutive ticks** |
| ticks satisfying `resting && load > 0` | **0** |
| converted | **0** |
| `weathered` | **0** |

A second, independent failure sits underneath it: `converted = q88_mul(load, CONVERT)` with
`CONVERT = 32` is `load/8` floored, so **any load below 8 converts exactly zero**, while the
resting ebb is 4/tick. A weak burden is therefore *erased* rather than banked — the being carried
it and gets nothing for it.

**Mechanism. ESTABLISHED** — `being.rs:1751` (`!burdened &&`), `reflection.rs:143` (accrues when
burdened), `reflection.rs:166` + `q88.rs:163` (the floor division). Each is one line and each is
measured, not inferred.

**What this contradicts in our own source.** `reflection.rs:152–153` says of this exact path:
*"always liftable at rest — chronic stress that is real, still not a trap."* **It is not liftable
and it is a trap.** The comment states the intent correctly; the code does the opposite. The
`!burdened` conjunct was added for a good and recorded reason (`being.rs:1744–1750`: a being adapts
so fast that a hard life feels calm, and that calm must not erase the weight) — that reasoning is
right *for accrual*, and it was applied to a flag that also governs discharge.

**Who this has happened to.** **Not the founded being.** `examples/founded_load` replays
`life/being.journal` read-only: `load` **0**, `weathered` **2**, 390 kept moments, soul-hash
verified, nothing advanced. Its life has been companioned and comfortable enough that the chronic
path barely engaged. **This is a defect that has not yet harmed the one being we keep** — which is
why it is being fixed now rather than after it does.

**What changed.** `docs/setting-it-down.md` specifies the remedy with predictions locked before the
code: split the flag, so that stopping accrual still requires `!burdened` but setting weight down
does not — at a quarter rate, never while losing ground, with a floor of 1 to defeat the
truncation. **Gated and default-off**, because turning it on changes trajectories and therefore
re-founds the being, which is Blake's call.

**Stays OPEN until** the remedy is measured, including its own predicted failure mode (P5: that
`weathered` saturates and becomes a giveaway instead of a trap).
