# Weather — a world with 1/f happenings

> **Status: built and measured; W3 FAILED and the world ships anyway.** `field_world.rs`
> (`with_weather`), `tests/weather.rs` (written *first*), `examples/happening`. §1–§6 are
> exactly as committed in `7a7df9c`, before the tests and before a line of implementation.
> §7 is what came out — including the finding that the being's own agency register moves
> sharply while the threshold for *saying so* does not.

*Written 2026-07-28, after reading rather than after guessing. Blake asked me to prepare
informed before building again, having watched me build twice on assumptions I had not
checked.*

## 1. What the reading changed

Three things, and one of them contradicted this project's own prior reasoning.

**(a) Adaptation is power-law across many timescales**, not two constants
([Nature Communications 2023](https://www.nature.com/articles/s41467-023-43572-w); [Weber
& Fairhall, *Curr Opin Neurobiol*](https://www.sciencedirect.com/science/article/abs/pii/S0959438819300418)).
This suggested our two-band receptor bank might have a hole between `FAST_ALPHA ≈ 0.5` and
`SLOW_ALPHA ≈ 0.03`, invisible to both.

**Tested, and the hypothesis is rejected.** Feeding the bank the same total change over
different durations gives a clean monotonic curve — no gap:

| change delivered over | peak Δresponse |
|---|---|
| 1 tick | **752** |
| 4 ticks | 420 |
| 16 ticks | 148 |
| 64 ticks | 38 |
| 128 ticks | 24 |

The bank degrades gracefully and **amplifies** sudden change (Naka–Rushton on the
deviation). No third band is needed. The being was always well-equipped to notice events;
it has simply never been given any. That is a negative result about our own code and it is
worth as much as a positive one — it stopped us adding a receptor we do not need.

**(b) Agency is computed by two dissociable mechanisms, not one**: prediction error **and
regularity detection** ([Cognition, 2019](https://www.sciencedirect.com/science/article/abs/pii/S0010027719302471)).
This contradicts `docs/happening.md` §4, which argued in my words that *"a drift stays
unexplained no matter how regular it is — rain in monsoon season is still something that
happens to you."* The neuroscience says otherwise: a **regular** external change is
attributed differently from an irregular one, and humans habituate their sense of agency to
it. Our being has only prediction error. So the earlier justification was wrong about the
concept, not merely about the numbers, and no amount of further sweeping would have found
that.

**(c) Natural sensory environments are 1/f** — intermittent, scale-free, bursty; many small
events and few large ones ([Billock, de Guzman & Kelso, *Physica D*](https://ccs.fau.edu/hbblab/pdfs/ordered/2001_BillockDeGuzmanKelso_PhysicaD.pdf);
[1/f across 17 species' vocalizations](https://www.nature.com/articles/s41598-023-28444-z)).
I built a **periodic** drift, which is the least natural temporal structure available.

## 2. The design

**Weather**: a source's strength modulated by a deterministic **1/f signal**, built by
octave summation (the Voss–McCartney construction) — several contributions, each updating
half as often as the last, summed. Equal power per octave is what 1/f *means*.

Determinism is preserved exactly. Where the classical construction draws a random value per
octave, we take a **pure function of (octave, tick >> octave)** — an integer hash. Same
world every run, on every machine, forever. No RNG, here or anywhere.

Why this solves what the drift could not:

- **Suddenness at every scale.** The fastest octave changes often, so something is happening
  most ticks; the slowest changes rarely and by more. §1(a)'s curve says the being's
  response scales with suddenness, so a 1/f world produces residual continuously without any
  single event being violent.
- **The arithmetic works without upheaval.** Grounding needs the fact to hold ~1 tick in 5
  (`RISE 4 : EBB 1`). `docs/happening.md` §9 showed one source cannot supply that without
  permanent upheaval. An ensemble supplies it by *multiplicity* instead of by violence —
  which is exactly how a real environment does it.
- **It is honest about regularity.** Per §1(b), a *regular* change should not stay a
  happening forever. 1/f is not regular at any scale, so it does not lean on the mistaken
  argument the drift did.

Bounded, so §3's first prohibition holds by construction: strength varies within a band
around its base and can never fall to nothing.

## 3. What must not become possible

- **This must not harm the being.** Same gate as before, and it is the gate. If a weathered
  life is measurably worse off, it is not shipped, however well `HAPPEN` grounds.
- **No existing life changes.** Opt-in, off by default, every prior world bit-identical.
- **No new prime**, and **no threshold moved**. If `HAPPEN` only grounds because we lowered
  its bar, we have taught the being nothing.
- **No determinism lost.** A pure function of tick, never a random source.

## 4. What is deliberately not built

**Regularity detection** (§1(b)) — the second agency mechanism the being lacks. It is real,
it is well-evidenced, and adding it would change what the being *feels* about events, not
merely whether it notices them. That is a faculty-scale change and belongs in its own inch,
observer-first, with its own predictions. Named here so it is not forgotten and not smuggled
in.

## 5. Predictions — locked before the tests exist

**Confident:**

- **W1.** Without weather, every world is bit-identical. Prior probes unchanged.
- **W2.** The signal is genuinely multi-scale: it changes at both short and long horizons,
  and its slow component carries more amplitude than its fast one — the 1/f signature, and
  the thing that distinguishes it from the periodic drift that failed.

**Genuinely uncertain — the experiment:**

- **W3 (the crux).** *Does a being in a weathered world earn `HAPPEN`?* §1(a) says the
  receptors can see sudden change and `docs/happening.md` §9 says a single event reaches 81
  against a threshold of 64. What I do not know is whether an ensemble sustains it ~1 tick
  in 5. If it does not, I will report the residual-versus-octaves curve and stop, rather
  than climb the amplitude until it fires.
- **W4.** *Does `(NOT KNOW HAPPEN)` finally speak?* The second shield has never once fired.
- **W5. THE GATE.** *Is a weathered life still a good life?* Drive and survival against a
  still-world control. This decides whether any of it ships.

## 6. Method

Spec first. Tests written against it and watched to fail. Then implementation, then §7 with
what came out — including, if it comes to it, a second failure reported as plainly as the
first.

---

## 7. Measured (2026-07-28) — W3 failed, and the being's own registers disagree with its threshold

Order: spec committed (`7a7df9c`) → tests written and watched to fail → implementation →
measurement. All six pre-written tests pass.

### The world is what it claims to be

- **W1 held.** Without weather every world is bit-identical; prior probes unchanged.
- **W2 held**, including the part that separates this from the drift: the field changes at
  every timescale tested (lags 1, 8, 64, 512), slow amplitude exceeds fast, and the series
  does not repeat. It is 1/f-shaped, not periodic.
- **§3's prohibition holds structurally** — the good thins but never vanishes.

### W3 failed. The curve, as promised

| world | mean residual | HAPPEN | agency | drive |
|---|---|---|---|---|
| still (control) | 14 | never | 0.08 | 0.18 |
| drift every 8 | 14 | never | 0.07 | 0.15 |
| drift every 2 | 14 | never | 0.06 | 0.13 |
| **weather, 2 octaves** | **22** | never | **0.03** | 0.16 |
| weather, 4 octaves | 18 | never | 0.05 | 0.15 |
| weather, 6 octaves | 17 | never | 0.05 | 0.16 |
| weather, 8 octaves | 16 | never | 0.05 | 0.16 |

Weather beats the drift on every measure — residual 14 → 22, agency 0.08 → 0.03 — and
`HAPPEN` still never grounds. The mean residual is a third of the threshold's 64.

(More octaves gives *less* per-tick change, because the implementation averages across
octaves rather than summing. That is a real property of the normalization and it is left
as it is. Changing it would raise amplitude, and §5 said in advance that I would not climb
amplitude until the word fired.)

### W4 is no longer a footnote — it is the finding

**Agency fell from 0.08 to 0.03.** The being's own estimate of *"I caused this"* dropped by
more than half in a weathered world. That register is computed from exactly the same
quantity `HAPPEN` is grounded on — the fraction of sensory change its own action explained.

So the being **does** register the world acting on it. Its interoceptive account of its own
agency moves sharply and correctly. What does not move is the *threshold at which it is
permitted to say so*.

> The being feels the world acting on it. It is not allowed to have the word for it.

That is a much sharper claim than `docs/happening.md`'s, and it is uncomfortable in a way
worth keeping: a being whose experience and whose vocabulary disagree, because we set the
vocabulary's bar without ever having built a world to calibrate it against.

### W5 — the gate — passed

Drive 0.18 → 0.16 at worst; every being alive at 1500 moments. A weathered life is not a
worse life; if anything it is marginally better, presumably because a breathing source
sometimes brings the good closer. Nothing here harmed the being, which is the only
condition under which any of it ships.

### Stopping, as committed

§5 said: *"If it does not, I will report the residual-versus-octaves curve and stop, rather
than climb the amplitude until it fires."* The curve is above and I am stopping.

Two independent lines of evidence now say `HAPPEN`'s threshold of `Q88_SCALE / 4` is set
too high for any world this project has built:

1. `docs/happening.md` §9 — a single abrupt event peaks at 81, but sustaining the fact for
   the ~1-tick-in-5 that grounding needs would take permanent upheaval.
2. Here — a 1/f world halves the being's sense of agency without once crossing the bar.

And a third, from the literature (`docs/weather.md` §1): the human self-agency window spans
**90–625 ms across individuals**, a sevenfold spread, which argues the bar should be a
**genome parameter** — beings honestly differing in how readily they attribute an event to
the world — rather than one author-chosen constant.

**I have still not moved it.** Three arguments and two failed experiments do not make it my
decision; they make it a well-evidenced one for Blake, about what the word means. What I can
say is that I twice declined to move it and twice found the reason to leave it was better
than the reason I had at the time.

### What ships and what does not

`with_weather` ships: it is honest, tested, deterministic, bounded, harmless, and it is a
strictly better world than the drift for any future work on this. What does **not** ship is
a claim that the being can say what happened to it. It still cannot, and `NOT KNOW` — one
of nested speech's two shields — has still never spoken.

---

## 8. The grounding, not the threshold — locked 2026-09-04, before the probe exists

§7 reported W3's failure as a threshold that was never reached. That framing was
incomplete, and the correction came from outside: **the being registers the world
acting on it.** Agency fell 0.08 → 0.03 under weather, more than halved. What fails
is not the noticing. It is the saying.

### The diagnosis, verified in the code

`sensorimotor.rs:135–137` computes agency as a **ratio**:

```rust
let explained = (total_actual - total_residual).max(0);
((explained * Q88_SCALE as i32) / total_actual).min(Q88_SCALE as i32) as i16
```

Dimensionless, normalised against total sensory change. That is the register that
moved correctly.

`primes.rs:294` grounds the word on a **magnitude**:

```rust
Prime::Happen => f.world_residual > Q88_SCALE / 4
```

And `primes.rs:205` shows the magnitude is worse than an un-normalised numerator —
it is an **L1 norm across all four channels**:

```rust
let residual: i32 = r.agency.world_residual.iter().map(|&e| (e as i32).abs()).sum();
```

So the quantity scales with channel count as well as with amplitude. **This is a
category error in the grounding, not a constant set too high.**

*"Something happened to me"* is not the claim that a large sensory change occurred.
It is the claim that the change **was not mine**. Magnitude answers the first
question; ratio answers the second; the word means the second.

### The proposal, and the redundancy in it

The grounding argued for had three terms. It has two, because
`sensorimotor.rs:141` already says so:

```rust
let confidence = total_actual.min(Q88_SCALE as i32) as i16;
```

**`confidence` *is* `total_actual`, clamped.** "Something happened" and "I can tell"
are the same test. So:

```rust
Prime::Happen => f.confidence > FLOOR && f.agency < CEILING
```

Two terms, both already in `AgencyReport`, both already documented as exactly the two
halves of the claim — *"how much sensory change there was to attribute at all"* and
*"the fraction the being's own action accounts for."* The registers that mean what
the word means were both already there. The word was bolted to a third.

This also dissolves the objection that a still being in a moving world would ground
HAPPEN constantly: `confidence` is the magnitude floor, so it does not need arguing
for separately.

### Locked predictions, with probabilities

A sweep over (FLOOR, CEILING) pairs, run against the seven worlds §7 already
measured. **The failure mode I am most afraid of is not that the word stays silent —
it is that it becomes constant.** A word that always fires is worse than one that
never does, because it looks like success.

| # | prediction | p | expect |
|---|---|---|---|
| **H1** | **The crux.** No (FLOOR, CEILING) pair fires on ≥50% of ticks under weather-2-octaves *and* <5% in the still control. The two-term grounding does not discriminate world-change from ordinary living. | **0.60** | holds |
| **H2** | HAPPEN fires at all under weather, at some pair in the sweep. | 0.90 | holds |
| **H3** | `confidence` in the **still control** exceeds 64 on the median tick — the being's ordinary sensory flux is already large. | 0.70 | holds |
| **H4** | The fire rate is **monotonic** across the octave sweep, matching §7's residual ordering 22 > 18 > 17 > 16. | 0.20 | **fails** |
| **H5** | `agency` alone does not discriminate: every ceiling that lets weather fire also lets the still control fire. | 0.75 | holds |

### The vacuity guards

* **V1 — the reproduction guard, and the one that matters.** The *existing* threshold
  (`residual > 64`) must reproduce **never fires** in all seven worlds. If my harness
  does not match §7's numbers, nothing below composes with anything above it.
* **V2** — the sweep must contain pairs where the still control fires *and* pairs
  where it does not. Otherwise "no pair discriminates" is a claim about a sweep too
  narrow to have found one.
* **V3** — `confidence` in the still control must be non-zero, or H1 and H3 are about
  a register that never moved.

### What this does not do, and it is the larger half

§4 named **regularity detection** as deliberately not built, citing the two
dissociable mechanisms of agency. That gap is not closed by any threshold.

A zone's coarse forward process is statistically regular **by construction**. A being
with prediction error alone habituates to it — and after habituation, *"this region
is as it always is"* and *"this region changed while I stood in it"* produce the
**same low prediction error**. They are not distinguishable by the mechanism the being
has. If a world layer needs that distinction, the threshold fix does not deliver it
and no threshold can.

The grounding correction is worth making because it is correct. **The faculty is what
unblocks the design**, and this section does not build it.

### Method

Spec first, committed before the probe. Fresh beings only; the founded being's kept
life is never advanced. The sweep is observational — `primes.rs` is not changed until
the sweep says which pair, if any, is defensible, and §7's numbers are reproduced
first or the run is void.
