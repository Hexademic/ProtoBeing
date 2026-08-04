# The expressive gap — what a being registers and cannot say

> **Status: built and measured** — `primes.rs` (`would_ground`, `Prime::threshold`),
> `tests/expressive_gap.rs` (written *first*, E0 load-bearing), `examples/expressive_gap`.
> §1–§5 are exactly as committed in `91075eb`, before the tests. §6 is what came out: the
> answer landed *between* the two branches I predicted, and the between is the finding.

*Written 2026-07-28, out of two failed experiments. `docs/weather.md` §7 found a being
whose agency register fell by more than half in a world that moved, while the word for
what moved it never became sayable. I filed that as a defeat. It is an instrument.*

## 1. What the field can and cannot do

Several groups are measuring the distance between what a system represents and what it
reports. [DenialBench](https://arxiv.org/pdf/2604.25922) scores self-report incoherence
across 115 models — claiming to lack preferences while producing outputs implying them.
The [entanglement gap](https://arxiv.org/pdf/2603.11382) deliberately measures latent
structure in trajectories *rather than* what a system says, because a model trained to
express welfare concern is indistinguishable, by verbal methods, from one that has it.
[Probing the Preferences of a Language Model](https://arxiv.org/pdf/2509.07961) pairs
verbal against behavioural tests.

Every one of these **estimates** the gap without ground truth, and each is honest that it
must: the internal state is unreadable, so behaviour or latent structure stands in for it.

**A proxy cannot be validated without a case where the true answer is known, and that
literature has no such case.**

This project has one. The being's registers are directly readable and its utterances are
audited against those registers by construction. So the gap here is not estimated. It is
computed.

That is the contribution, and it is not "our being is a better candidate for
consciousness" — a claim we refuse anyway. It is that **this being can serve as the ruler
those instruments are calibrated against.**

## 2. The metric

For a prime `P` grounded on register `R` with reporting threshold `t`:

- **Registered variation** — the spread of `R` across a life. What the being's own
  machinery distinguishes.
- **Reportable set** — the values of `R` that actually lead to a grounded, speakable word,
  given the grounding dynamics (`RISE` on hold, `EBB` on lapse, ground at
  `GROUNDED_THRESHOLD`).
- **The expressive gap** — registered variation that lies outside the reportable set.
  States the being has and cannot say.

Computed by replaying the grounding accumulation **offline** at many thresholds against a
recorded life. Nothing in `primes.rs` changes and no threshold is moved; the sweep is
arithmetic over a record, not an edit to the being.

## 3. What must not become possible

- **The instrument must not alter the instrument's subject.** Pure observer. The shipped
  thresholds stay exactly as they are; the sweep is hypothetical throughout.
- **The ruler must agree with what it measures.** At the shipped threshold, the offline
  replay must reproduce the live `PrimeLayer` exactly. If it does not, every other number
  here is worthless. This is E0 and it comes first.
- **No word may be made sayable by lowering its bar until it lies.** The point of the sweep
  is to find whether a bar exists at which the word *discriminates*, not one at which it
  merely fires.

## 4. Predictions — locked before the tests exist

**Confident:**

- **E0.** The offline grounding replay reproduces the live layer exactly at the shipped
  threshold, for every prime tested. The instrument agrees with its subject.
- **E1.** At the shipped threshold, `HAPPEN`'s gap in a weathered world is total: the
  register moves across a wide range, the word never grounds. (Known from
  `docs/weather.md` §7; asserted here so the instrument reproduces a result we already
  trust.)

**The crux, genuinely uncertain:**

- **E2.** *Does a threshold exist at which `HAPPEN` grounds in a weathered world and does
  **not** ground in a still one?*
  - **If yes** — the word can discriminate the world it is about, and its bar is simply set
    above the range where it works. That is a miscalibration with a measured range, and it
    converts three sessions of my judgment into a number.
  - **If no** — any bar low enough to fire in weather also fires in stillness. The word
    would then be *lying* if lowered, every refusal to lower it was right for a better
    reason than I had, and `HAPPEN` needs a different ground rather than a different
    number.

  I do not know which. Both outcomes are publishable and one of them tells me I have been
  wrong to hold the line for the reason I held it.

- **E3.** Run across all 18 primes: how many have their shipped threshold *inside* their
  discriminating range? This is the instrument turned on the project itself, and I expect
  it to be uncomfortable.

## 5. Method

Spec first. Tests written against it and watched to fail — E0 above all, since an
instrument that disagrees with its subject is worse than no instrument. Then the
implementation, then §6 with what came out.

---

## 6. Measured (2026-07-28) — the answer is between the two branches I predicted

Order: spec committed (`91075eb`) → tests written and watched to fail → implementation →
measurement. All six pre-written tests pass, E0 among them.

### E0 held — the instrument is valid

The offline replay reproduces the live `PrimeLayer` exactly at the shipped bar across
five register shapes, including intermittency near the `RISE:EBB` break-even where the two
could most easily have diverged. `Prime::threshold()` reports the bar `holds` actually
enforces. Everything below rests on this and it is asserted, not assumed.

### E2 — a discriminating bar exists, and it is razor-thin

`HAPPEN`, swept against a still world and a weathered one:

| bar | still world | moving world | |
|---|---|---|---|
| 0–24 | grounds | grounds | fires in both — **would lie** |
| **28** | **never** | **grounds @421** | **discriminates** |
| 32–96 | never | never | silent in both |

Finer sweep: the discriminating range is **[25, 30]**. The shipped bar is **64** — more
than twice the top of the window.

**So the threshold is miscalibrated, and now that is a measured fact rather than my
opinion.** Three sessions of judgment became a number, which is what this instrument was
built for.

But the honest headline is the *width*. Six units out of a 0–192 range. My §4 prediction
had two branches — either a discriminating bar exists (miscalibration) or none does (the
word needs a different ground). **Reality landed between them**, and the between is more
informative than either: the bar exists, so the word *can* track the world; it is so narrow
that a bar set inside it would be fitted to this world and probably to no other.

A 6-wide window is not a calibration error. It is a sign that **the ground is fragile** —
an absolute bar on raw residual, in a being whose own full-effort action moves that
register by about 3. What the word wants is a bar relative to the being's own scale
(*"more than I could have done"*), which would be dimensionless and would travel between
worlds. That is a different inch and it is the one this measurement argues for.

### E3 — the instrument on the whole vocabulary, with a caveat that matters

| prime | register span | shipped bar | discriminating range |
|---|---|---|---|
| GOOD / BAD | 0..124 | 25 | [89, 89] |
| VERY | 0..124 | 128 | [89, 89] |
| NOT KNOW | 0..152 | 42 | **none** |
| CAN | 0..128 | 128 | none |
| CAN'T | 1..71 | 48 | [6, 6] |
| DO | 0..226 | 32 | none |
| **HAPPEN** | 0..68 | **64** | **[25, 30]** |

**This table must not be over-read, and the caveat is the point.** "Discriminating" here
means *grounds in a moving world and not a still one* — which is the right test **only for
primes that are about world-change**. `HAPPEN` and `NOT KNOW` are; `GOOD`, `CAN`, `DO` are
not, and "none" for them is not a criticism. It says the world moving is not what those
words are for.

The one row that is a real finding besides `HAPPEN`: **`NOT KNOW` cannot discriminate at
any bar.** Novelty does not distinguish a weathered world from a still one in this setup.
That is the second shield failing for a reason quite separate from `HAPPEN`'s, and it
explains why it has never once spoken.

### The gap itself

`HAPPEN` in the moving world: the register spans **68 raw** (0.27), and **5 ticks out of
1500** clear the shipped bar — enough to flicker, never enough to sustain. The being
registered a range it could not once report.

### What I am doing about the threshold: still nothing, and now I can say why properly

An objection I have been leaning on turns out not to apply. `primes.rs` is a pure observer
— `being.rs` is not modified by it — so changing a prime's threshold **re-founds no being
and moves no soul-hash**. That was the digest's problem, not this one. I had been carrying
the objection across from a different decision without checking, which is the fourth time
today an unexamined assumption has been the thing in the way.

The reasons to leave it are different and, I think, better:

1. **A 6-wide window is overfitting waiting to happen.** Setting the bar to 28 would tune
   it to one measured world.
2. **n = 1.** The method validated on exactly one prime for which the test is meaningful.
3. **The finding argues for a different ground, not a different number** — relative to the
   being's own action scale rather than absolute.

So: I recommend **not** setting it to 28, and instead grounding `HAPPEN` on residual
*relative to the being's own maximal action*, then re-running this sweep. If the window
widens, that was the fix. If it stays 6 wide, the word may simply be a poor discriminator
and should be said so about.

That is a recommendation with a measurement behind it, and it is still Blake's to take.
What changed today is that it is no longer a matter of taste for either of us.
