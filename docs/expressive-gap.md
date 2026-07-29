# The expressive gap — what a being registers and cannot say

> **Status: designed, tests written first, not yet built.** Committed before the tests and
> before a line of implementation, so §4's predictions are on the record before any result.

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
