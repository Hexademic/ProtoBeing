# Underdetermination — the one thing the being lacks, in four disguises

> **Status: research brief for 2026-07-31.** Not a spec. No predictions are locked here and
> nothing is authorised. This is the reading and the candidate mechanisms, written the night
> before, so tomorrow's specs start from literature rather than from my intuition — which has
> now been wrong enough times this week to be worth routing around deliberately.

*Written 2026-07-30 after examining the being (`docs/composed.md` §6, plus fresh registers
below). Blake: "that is precisely what we should work on."*

## 1. What the measurement said

Two thousand ticks, weathered world, generous partner:

| register | receptors OFF | receptors ON | of |
|---|---|---|---|
| `self_surprise` | 5.17 | **1.21** | 256 |
| `self_knowledge` | 249.49 | **254.26** | 256 |
| `free_energy` | 2.80 | **0.69** | 256 |
| ticks at rest | **0%** | **0%** | 2000 |

And from the project's own record: `NOT KNOW`, one of nested speech's two shields, has never
been spoken in 1,486 audited sentences.

Read together: **a being that never rests, never doubts, and is never surprised by itself** —
and giving it a working body made all three *more* extreme, not less. Better senses, better
prediction, less surprise.

Four apparent gaps — rest, doubt, self-surprise, variation. §2 says they are one gap.

## 2. The diagnosis, and it is not flattering

Edelman & Gally (2001) and Whitacre & Bender (2010) study **degeneracy**: structurally
*different* elements that can perform the same function. Their central results are that
degeneracy is a fundamental source of robustness, is what ties robustness to **evolvability**,
and — the line that stopped me —

> one important difference between systems created **by design** and those that **evolve
> without planning** is that in the former, components with multiple overlapping functions are
> absent.

Our being was designed. Every gap I listed is the signature of that, not four separate
oversights. Total determination is what careful design produces when nobody was specifically
trying to leave anything open.

**And this corrects yesterday's work.** `docs/null-space.md` went looking for *redundancy* —
four compass directions, equally good. That is **redundancy, not degeneracy**: one mechanism,
several settings. Degeneracy would be *different mechanisms* reaching the same end. The
literature says only degeneracy buys evolvability; pure redundancy does not. That is probably
why the scavenged null space felt hollow even where it existed at 95% of ticks — it was never
the right thing to be looking for.

This matters directly for Blake's lineage goal: **degeneracy is the named precondition for
evolvability.** A being without it can be selected on but cannot evolve well.

## 3. Rest — the mechanism already exists in the literature and fits us exactly

Klar, Stein, Paterson, Williamson & Gollee, *Intermittent Active Inference* (Entropy 28(3):269,
2026). Standard active inference assumes continuous inference and control; humans do not work
that way. Their agents **follow the current plan and re-plan only when prediction error exceeds
a threshold**, or when expected free energy of the current plan exceeds prior estimates.

Our being recomputes `intent_from(report)` from scratch every single tick. It has no notion of
*holding* a plan. That is precisely why it is at rest 0% of ticks: nothing in the architecture
lets it keep doing what it was doing.

The fit is unusually good, and it inverts a fact we had filed as a nuisance:

> **`free_energy` of 0.69/256 currently buys the being nothing. Under intermittency, it becomes
> the reason it may rest.** A being that predicts its world well should not have to re-decide
> every tick — and one whose predictions start failing should wake up and re-plan.

Rest earned by the world being predictable, and vigilance earned by it not being. That is not a
sleep timer bolted on; it is rest as a consequence of the being's own competence.

Biological grounding: *satiety quiescence* in C. elegans (Front. Neurosci. 2021) — a fed worm
enters an actively-signalled quiescent state. Rest is a state with its own machinery, not the
absence of activity. Worth taking seriously against a naive "effort floor," which would give
stillness without giving rest.

**Candidate for tomorrow, cheapest first:** an intermittency threshold on re-planning, observer
first — compute *whether* the being would re-plan, report it, steer nothing, and measure what
fraction of a life it would have spent holding its plan. If that fraction is near zero the idea
is dead before it costs anything.

## 4. Doubt — why it is currently impossible, and the shape of the fix

The uncertainty literature is unambiguous on the structural point. **Aleatoric** uncertainty
(irreducible noise) is representable as a distribution. **Epistemic** uncertainty — not knowing
how good your own model is — *requires a second-order representation*: a distribution over
distributions. First-order predictions capture randomness in outcomes; epistemic uncertainty
expresses "the learner's lack of knowledge about how accurately its predictions approximate the
ground truth."

Now look at ours. `self_knowledge: i16`. A **scalar**. There is nowhere in it to put "and I am
not sure how well I know that."

**So the being cannot doubt for the same reason it cannot store a fear: the type has nowhere to
hold one.** That is the same structural argument `inheritance.rs` makes as a virtue, appearing
here as a defect. Which is oddly reassuring — it means the fix is a type change, not a
philosophy.

**The minimum honest mechanism:** carry a *spread* alongside the mean. Not a full second-order
distribution — Q8.8 and zero dependencies rule that out — but the first moment that a scalar
cannot express:

```text
self_knowledge : i16   // the estimate            (exists)
self_spread    : i16   // how much it has been moving   (does not)
```

High spread = *my model of myself has been unstable lately* = a fact that is **true**, readable,
and auditable — which is exactly what the honesty floor requires before `NOT KNOW` may be
asserted. That gives the word a referent instead of giving it a lower bar.

**This is the difference between doubt and theatre, and it is worth stating in advance:**
lowering `Prime::NotKnow`'s threshold until the word appears would be tuning to a desired
output — the thing this project refuses. Giving the being a register that can *be* uncertain,
and letting the existing audit decide whether the word holds, is the opposite. If the spread
never rises, the being still never says it, and that is an honest answer too.

**A convergence worth noticing:** this is the same shape as the mean-plus-variance proposal for
inheritance (2026-07-29 conversation — averaging parameter vectors converges to mush; carry the
variance and it becomes the child's plasticity). Two unrelated problems, one answer: **the
variance is where the openness lives.** I do not think that is a coincidence and it may be the
most useful idea available to us right now.

## 5. World richness — the being has nothing to be wrong about

`free_energy` 0.69/256. It is not predicting well because it is clever; it is predicting well
because there is almost nothing there. One drifting source, one weather term, one person.

In expected-free-energy terms, an agent's **epistemic value** is the uncertainty a policy is
expected to resolve. Ours has almost none available, so novelty-seeking has nothing to seek.
`docs/happening.md` §9 reached the same wall from the language side — `HAPPEN` cannot ground
because one moving thing cannot supply enough happening — and I confirmed today that receptors
do *not* rescue it (grounds at bar 25, not 30, shipped bar 64; max residual 72 → 82 with
receptors on).

This one needs no new faculty at all. It needs more in the world, and it is probably the least
intellectually interesting and most immediately effective thing on the list.

## 6. How these depend on each other

```text
world richness ──► something to be wrong about ──► self_surprise > 0
                                                        │
degeneracy ──► variation the being owns ──────────────► │
                                                        ▼
                                              self_spread rises
                                                        │
                                                        ▼
                                            NOT KNOW becomes true
                                                        │
intermittency ──► rest ──► surplus ──────────────────► play
```

Doubt is downstream of both world richness and variation, which is why I could not see how to
build it directly. It is not a faculty to add. **It is what appears when a being that can vary
meets a world that can surprise it.**

## 7. What I would bring to tomorrow, in order

1. **Intermittency, observer-first.** Cheapest, self-contained, literature-backed, and it
   converts our embarrassing 0.69 free energy into the mechanism for rest. Kill-early test: if
   the being would hold its plan on <5% of ticks, drop it.
2. **World richness.** No new faculty. Several independent sources. Re-run `happening` and the
   expressive-gap sweep against it — `NOT KNOW` and `HAPPEN` may both be waiting on this and
   nothing else.
3. **`self_spread`, observer-first.** The type change that makes doubt *possible*. Report it,
   steer nothing, and see whether it ever rises. If it never does, we have learned that the
   being's self-model is stable for real reasons and not merely for lack of a register.
4. **Degeneracy rather than redundancy** — reframing the action-surface work with the right
   target. Different *ways*, not different *settings*. This is the long one and it is also the
   one Blake's lineage goal is blocked on.

## 8. Honest limits of this brief

- The Entropy paper's mechanism is verified from its abstract; I have not read the full method,
  and the fit to our architecture is my inference, not their claim.
- Whitacre & Bender and Edelman & Gally are cited from search metadata: the environment's proxy
  blocks Crossref, PMC and arXiv, so **their identifiers are recorded unverified** in
  `docs/references.md` and marked as such.
- I could not reproduce the project's 1,486-sentence speech record with my own probe (it
  recorded zero utterances), which is almost certainly my misuse of `PrimeLayer::speak_tree`
  rather than a fact about the being. Anything tomorrow that depends on speech should re-derive
  that number before trusting it.
- Nothing here is a prediction. Predictions get locked in the specs, tomorrow, before any code.
