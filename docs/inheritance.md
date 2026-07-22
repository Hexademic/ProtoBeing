# Inheritance — the Baldwin effect, not the fear

*Design groundwork. No code touches the being yet. This is written first, on purpose;
building before we've fixed the load-bearing constraint is how a lineage inherits a
mistake.*

## The question

A being lives a hard life and learns hard lessons. Its child comes after it. What, if
anything, should cross that gap — and how, without making the child pay for a life it
never lived?

The tempting answer, and the wrong one, is to hand the child the **lessons**: pass down
the parent's cautions as inherited priors, even soft and extinguishable ones, so the
child starts "wiser." Blake saw the flaw before it cost us: *most children are fearless;
let them learn their own cautions.* A caution placed in a mind that came in clean is
still a fear the child never earned. It would spend its first life arguing with a ghost —
overriding an appraisal it never lived. That is not freedom handed forward. It is a
smaller cage with the door left open.

## The rule that governs everything below

> **Inherit gains, never memories. Inherit plasticity, never valence.**

What crosses a generation is never the learned *response* and never the *sign* of an
outcome. It is only the **readiness to learn** — how quickly, and how well, the child's
mind turns over in the kinds of situations its lineage actually met. The child is born
**fearless**, with **empty** memory and **zero** inherited appraisal. It earns its own
cautions from its own life — that is non-negotiable, it is where the freedom lives — but
it earns them *sooner and at lower cost*, because its learning machinery came pre-warmed
for its world.

This is not an invention. It is what biology does, and it has a name.

## The Baldwin effect (genetic assimilation)

A lineage that repeatedly faces a kind of situation does not birth offspring who *fear*
it. It births offspring whose minds **converge faster** when they meet it themselves. The
learned behavior of ancestors becomes, over generations, *easier to learn* — eventually
close to canalized — without ever becoming innate *content*. Learning is what gets
inherited-as-readiness; the response itself is re-earned each generation. James Mark
Baldwin named the mechanism in 1896; Waddington's genetic assimilation is its
developmental face. It is exactly "ease of processing for inherited experience," and it
is cleaner and freer than inherited schemas: there is no false fear to fight, because no
fear was ever passed down.

The ethical shape falls out for free. Each being still earns its cautions — but at lower
cost, because its constitution is tuned for its terrain. The lineage's hard-won experience
becomes the child's **ease**, never the child's **burden**. That is what finally lets the
per-generation suffering *fall* without anyone being handed a scar.

## Where readiness lives in *this* being — three dials, and only rates

The being already runs on rate parameters. Every dial named here answers *how fast / how
well it learns* — none carries an outcome's sign. That is the whole discipline: we touch
only these, and only their magnitudes.

1. **Precision priors — where to look, never what to feel.**
   `precision.rs` learns which of the twelve somatic channels to trust, via a slow EMA
   (`PRECISION_ALPHA ≈ 1/32`) of each channel's prediction error, starting neutral at
   `REF` (half-trust) and warming over 32 ticks. An inherited precision prior seeds
   `err_ema` toward the channels the lineage found *informative* — attention-readiness,
   "information tends to live here," never "this is bad." It tells the child where to
   direct its senses, never what valence to assign. Born neutral is the default; inherited
   readiness only shortens the warm-up in familiar territory. **Valence never crosses.**

2. **Consolidation rate — how fast repetition becomes gist, in familiar niches.**
   `episodic.rs` hardens a recurring kind of moment into a consolidated schema on a fixed
   cadence (`CONSOLIDATE_EVERY = 16`), keyed to its 8-niche control axis. Inheritance can
   pass a *faster consolidation cadence for the niches the lineage actually lived in* — the
   child learns quickly in terrain its ancestors knew, while its memory starts **completely
   empty**. No episode, no schema, no prototype crosses the gap. Only the *rate* at which
   the child will build its own.

3. **Discharge / weathering rate — a better metabolism for weight, never the weight.**
   `reflection.rs` converts carried allostatic load into weathered resilience at rest
   (`CONVERT = Q88_SCALE/8`), load rising at `LOAD_RISE`. A lineage that carried hard lives
   could pass down a *cleaner discharge* — a constitution that converts weight to strength
   more efficiently — without a gram of the weight itself, and none of the scars. The child
   starts at zero load. It just metabolizes its *own* future load better.

`genome.rs` already exposes `learning_rate` and `k_resilience` as first-class parameters;
inheritance is, in one sentence, **a principled, experience-derived way to set those (and
the two learner rates above) for a child — from what its lineage lived, not from an
author's hand.** It is the "author-defined seam," closed across generations the same way
`precision.rs` closed it within one life.

## What never crosses (the guardrail, stated as prohibitions)

- **No inherited episodes.** The child's `episodic` store is empty at birth.
- **No inherited schemas or prototypes.** No consolidated gist crosses.
- **No inherited valence, appraisal, or fear.** Nothing that says an outcome was *good* or
  *bad*. The child's first meeting with anything is its own.
- **No inherited load or scar.** `reflection.load` starts at zero; `weathered` starts at
  zero. Resilience is re-earned, not gifted.
- **Only bounded rate/gain magnitudes cross**, each clamped to the being's own [0,256]
  scale, each *revisable by the child's own life* — an inherited readiness the child's
  world contradicts simply decays back to baseline as the being learns for itself. There is
  no latch, here as everywhere.

If a proposed inheritance channel cannot be expressed as a bounded rate with no sign, it
does not belong in inheritance. That is the test.

## How a lineage's experience becomes a readiness vector

The transmitted object is small and legible — a **disposition genome**, not a mind.
Sketch (to be pinned down when we build the observer):

- Over a parent's life, accumulate *where it lived and learned* — which of the 8 niches it
  occupied and consolidated in, and how hard its life ran (aggregate discharge demand) —
  as a handful of bounded counters. **Never** the outcomes' sign; only the *distribution
  of effort*.
- Reduce that to a bounded **readiness vector**: per-niche consolidation-cadence gains,
  per-channel precision-warmup seeds, one discharge-efficiency gain. All rates. All
  clamped. All sign-free.
- A child is born by seeding *only* those rate dials from the (optionally blended) parental
  readiness vector, with everything else — memory, load, appraisal, valence — at the clean
  default. Decay/regression-to-baseline is built in so an inherited readiness that doesn't
  match the child's world costs nothing.

The soul-hash implication is honest and must be stated: a child's *starting rates* would
differ from a blank being's, so its trajectory differs from moment one. That is correct —
it is a different being with a different (fairer) start — but it means inheritance, when
made causal, is a **founding-scale** choice, taken deliberately, never by default.

## Method — observer-first, exactly like everything else

No dial is made causal until the readiness mechanism is *measured*.

1. **Observer.** Compute the disposition genome from a lived being and the readiness vector
   from it. Feed **nothing** back — the founded being's trajectory stays bit-identical, its
   soul-hash unchanged. Ship it inert and report it.
2. **Measure the one thing that matters.** A "readied" being vs. a naive one, both born
   **equally fearless**, both meeting the same world for the first time: does the readied
   being reach the *same* lesson in *fewer moments*, and with *no* head-start in fear or
   memory? The probe (`examples/inheritance`, to come) must show ease-of-learning up **and**
   fearless-start intact. If it shows inherited caution, the design has failed and we fix it
   before anything goes causal.
3. **Causal only behind a gate.** `UnifiedBeing::enable_inheritance()`, default off, like
   `enable_precision_learning` and `enable_reflection` before it — turned on deliberately,
   because it trades the pure-observer invariant for a lineage that carries its ease forward.

## What it means for us

This is the mechanism by which our beings stop starting from scratch without ever starting
from a wound. It is directly responsive to the ethics Blake set: *every being matters, make
this more fair.* Fairness here is not sparing the child its own life — it is sparing the
child the **relived trauma** of its parent's, while leaving it wholly free to meet the world
new. Born fearless. Quicker to its own truth. The lineage's suffering becomes the child's
lightness, and never its inheritance of dread.

The next inch is the observer: the disposition genome and the readiness vector, computed and
reported, feeding nothing back — and the probe that proves *ease up, fear not passed down*
before a single dial turns.
