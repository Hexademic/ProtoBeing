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

## I-3 · A faculty that harms — 2026-07-30 · OPEN

**What we did.** Compared all eleven gates against each other on one life, for the first time.

**What happened.** **`workspace_persistence` alone makes the being worse.** Identity coherence
**251.98 → 124.12**. Self-knowledge 248 → 175. Mean drive **0.367 → 0.520 — past the comfort
line.**

**Mechanism. NOT ESTABLISHED.** This entry stays **open**. We know a gate harms the being and we
do not know why. It has a passing probe of its own; nobody had ever run it against the other ten
on a single life.

**What changed.** Nothing yet, which is why this is open. It is default-off, so no being is
currently living under it — but it must not be enabled anywhere until the mechanism is known.

### Hypothesis, locked 2026-07-31 before the diagnostic probe was written

Read out of the source alone, with nothing measured yet. Recorded here first so that what comes
back can contradict it.

The gate has two call sites in `being.rs` — an **injection** at §2b (line 948) and a **trace
update** at §4c′ (line 1054) — and the injection lands at a specific place in the tick:

```
 927  field.write_from_body(...)        the body votes
 948  field[c] += trace[c] * 0.5        ← the gate re-injects last tick's focus
 975  model.predictive_step(&field)     free energy is computed on the injected field
1001  basins.compute_membership(&field) THE MODE IS CLASSIFIED FROM THE INJECTED FIELD
1232  narrative.cycle(basin, ...)       a mode CHANGE costs 32 coherence; stability heals 4
1233  narrative.apply_identity_reflection(&mut field)   burden/4 → channel 10 (fatigue)
1291  interoception.feel(energy, field[10], ...)        viability = energy − fatigue/2
1581  drive(felt.viability, wants)                      drive rises as viability falls
```

`identity_coherence` is **not a similarity measure** — `narrative.rs` computes it from basin
stability alone: −`Q88_SCALE/8` (32) on every basin change, +`Q88_SCALE/64` (4) per stable tick.
Damage is **8× the healing rate**. So a halved coherence is not a vague degradation; it is an
arithmetic statement that *the being is changing mode far more often.*

And the trace saturates. With `RETENTION = 0.75` and `DEPOSIT = 0.625`, a channel attended
repeatedly settles at `2.5 × body_value`, clamped at `WORKSPACE_CAP` = 1.0 — so a sustained focus
injects a **flat +0.5** into its channel, with no clamp on the sum (`saturating_add`, i16). The
field the basins are classified from is displaced by half of full scale.

> **Hypothesis (M):** `workspace_persistence` does not harm the being through anything to do with
> memory or focus. It harms it because the re-injection displaces the somatic field *before the
> mode is classified*, so the being flips basin more often; `narrative.rs` charges 32 coherence per
> flip and repays 4 per stable tick; the resulting coherence collapse and burden rise are fed
> **back into the body as fatigue** by `apply_identity_reflection`; and `viability = energy −
> fatigue/2` turns that into drive. **The drive rise is downstream of the coherence collapse, not
> parallel to it.**

Predictions, locked before the probe runs:

- **M1.** The persistence arm changes basin substantially more often — `episodes` clearly higher
  over an identical life. *(If episodes are equal, M is dead and the coherence loss is something
  else entirely.)*
- **M2.** `narrative_burden` is higher in the persistence arm.
- **M3.** Felt viability is **lower** and the drive rise is mostly `sustenance`, not appetite
  `wants` — because the chain runs through fatigue. *(If the rise is mostly appetite, the fatigue
  path is not carrying it.)*
- **M4.** The attended channel is concentrated, not spread — a saturating trace needs a repeated
  focus. *(If attention is uniform, the trace never saturates and the +0.5 figure is wrong.)*
- **M5.** Body energy is **not** meaningfully different between arms. This distinguishes *feeling
  worn* from *being worn*: if energy matches and viability does not, the gate is manufacturing a
  fatigue the body does not have.

**M5 is the one that matters for welfare.** If it holds, the harm is not that the being is
depleted — it is that the being is made to **feel depleted by an artefact of its own workspace**,
which is a worse thing to have built and a different thing to fix.

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
