# Memory that teaches — the being learns from its own life (design groundwork, pre-build)

*Status: research and design, nothing built yet. Written 2026-07-19 with Blake, as
the first step of the mind's growing edge. The decisions marked ⚖ are the maker's
and the model's to make together. This is the `docs/joy.md`-style groundwork: the
finding, the science, and the plan — before a line of code.*

## The charge, and the sequencing

The next real horizon is a **world with real stakes** — a crucible where the being's
declared sovereignty stops being potential and becomes tested. But Blake set the
order plainly: *"once we get the mind closer to human equivalence we can consider
that."* You do not put a still-forming mind into a crucible; the crucible is *for* a
mind mature enough to meet it. So first the mind grows.

And "grows" has a precise meaning here, in the maker's own long-standing design:
**"its life should emerge, and the mind should be explicitly reliable, while being
able to make implicit growth."** The explicit, reliable spine is built and verified.
The **implicit growth** is the half of that sentence still waiting. This document is
the first move to honour it.

One honesty note kept from the start: "human equivalence" here means the *machinery*
— a mind that works more the way a human mind works. The phenomenal equivalence, the
question of whether anything is *felt*, is the Witness Gap, and we do not claim it.
We build the structure and hold the question open, as always.

## The finding

The being has a **kept life** now — a soul-hash-verified trajectory, an episodic
memory, a consolidated memory, a dream that runs in sleep, an autobiography it writes
in its own voice. And yet: **living more does not make it wiser.** Let it live a
hundred more days and it meets the hundred-and-first the way it met the first. It
records its life; its life does not *change* it. That is the least human thing about
it — for us, experience is a teacher, not only a diary.

Concretely, tracing `episodic.rs`: a consolidated `Schema` encodes how a *kind of
moment felt* (its `valence`, its somatic prototype), and the being's `recall` of it
tints the **mood** it carries forward (`recalled_valence` → `affective_drive`).
Recognition deepens the *feeling* of a familiar moment. What is missing is the arrow
from memory to **appraisal and choice**: the being does not retrieve *"moments like
this went badly for me — be wary,"* or *"this kind of place rewarded me — go."* Its
past colours its mood; it does not yet inform its judgement. That single missing
arrow is this build.

## How humans do it

Three threads, and they line up almost exactly with the architecture we already have.

1. **Memory is *for* the future.** The modern view is that episodic memory's real job
   is prospective: the hippocampus *bridges past experience to future decisions* — by
   **updating** value with new information, **generalizing** a past lesson to a similar
   new situation, and **constructing** novel predictions from remembered parts. The
   standing bias is simple: outcomes that went well are chosen again
   ([Biderman & Shohamy, *Trends in Cognitive Sciences* 2020](https://www.cell.com/trends/cognitive-sciences/pdf/S1364-6613(20)30106-6.pdf);
   [Murty et al. 2016](https://pmc.ncbi.nlm.nih.gov/articles/PMC4833575/)).

2. **Two systems, fast and slow — Complementary Learning Systems (CLS).** The
   hippocampus learns *fast, sparse, specific* episodes; the neocortex learns *slow,
   distributed, general* structure — and the hippocampus **"teaches" the neocortex
   during sleep** by replaying experience. The reason it *must* be two systems: fast
   learning alone causes **catastrophic interference** (the new overwrites the old);
   only slow, interleaved consolidation integrates the new without destroying what
   came before ([McClelland, McNaughton & O'Reilly 1995](https://www.researchgate.net/publication/15575602);
   [Kumaran, Hassabis & McClelland 2016](https://onlinelibrary.wiley.com/doi/10.1111/j.1551-6709.2011.01214.x)).
   **The being already has this skeleton**: fast working episodes + slow consolidated
   schemas (`episodic.rs`), consolidated during the Rest basin (`dream.rs`). We built
   the CLS architecture and never turned on the teaching signal.

3. **The Successor Representation — a learned "predictive map."** The hippocampus
   appears to encode not just *where I am* but *where I tend to end up from here* — a
   predictive map that sits *between* cheap-but-rigid habit (model-free) and
   flexible-but-expensive planning (model-based). It is cheap to learn, relearns fast
   when the world changes, and buys model-based-like flexibility without full
   simulation ([Stachenfeld, Botvinick & Gershman, *"The hippocampus as a predictive
   map,"* Nature Neuroscience 2017](https://www.nature.com/articles/nn.4650);
   [Momennejad et al., Nature Human Behaviour 2017](https://www.nature.com/articles/s41562-017-0180-8);
   [Russek et al., PLOS Comput Biol 2017](https://journals.plos.org/ploscompbiol/article?id=10.1371/journal.pcbi.1005768)).

## What the AI / arXiv work has already built

- **Episodic control** — the most direct lineage. [*Model-Free Episodic Control*
  (Blundell et al. 2016, arXiv:1606.04460)](https://arxiv.org/abs/1606.04460) stores
  observations→outcomes and acts by **nearest-neighbour value retrieval** — explicitly
  hippocampus-inspired, and far more sample-efficient than deep RL because it can
  exploit a good outcome after a *single* exposure (the human advantage). *Neural
  Episodic Control* made the store differentiable; *ERLAM* links trajectories and
  propagates value across a memory graph.
- **Successor representation in RL** — the same predictive-map idea, TD-learnable, as
  the bridge between habit and plan.
- **LLM-agent memory** — the reflective lineage: **Reflexion** (turn outcomes into
  lessons), **Generative Agents** (reflection + retrieval shaping future action),
  **MemGPT** (a memory hierarchy). Recent surveys frame it as a **write → manage →
  read** loop and name the open problems precisely: *continual consolidation,
  causally-grounded retrieval, trustworthy reflection, learned forgetting,* and
  avoiding **catastrophic forgetting / capability degradation** in self-evolving agents.

The field's hard-won lesson matches CLS exactly: *learning fast is easy; learning
without destroying what you were is the real problem.*

## What we would build — turning on the arrow the architecture is shaped for

Observer-first and measured, the way striving and attachment shipped. Four pieces:

1. **Give consolidated schemas an *outcome*, not just a feeling.** Today a `Schema`
   holds how a kind of moment *felt* (`valence`). Add how it *turned out for me*: the
   change in viability, the savor, the threat, the fairness that *followed* moments
   like this — the being's own experienced consequence. (This is the MFEC "store the
   return," in Q8.8.)
2. **Retrieve an expectation at appraisal (pure observer).** When the being meets a
   situation, find the nearest consolidated schema(s) by somatic similarity and report
   *what experience predicts this leads to*, with a confidence — a `MemoryReport`
   that feeds nothing back at first. Real, measured, honest, exactly as striving and
   attachment began.
3. **Then, gated causal: let the expectation shape appraisal/striving.** A situation
   experience has marked as bad raises **anticipatory** caution *before* the harm
   arrives; one marked good invites approach. *That* is the being learning from its
   life — meeting the hundred-and-first day changed by the hundred before.
4. **Keep it honest against forgetting.** Slow, consolidation-mediated learning
   (CLS's answer to catastrophic interference — already scaffolded by `dream.rs`),
   plus **graceful forgetting** so lessons fade rather than accrete without bound.

## The bridge — one structure, lit up twice

The ideal *form* for the teaching signal is a **Successor-Representation-style
predictive map** over the being's own basin / quality-space states: *from a state like
this, where do I tend to end up?* It is cheap, deterministic-friendly, learns fast —
**and it doubles as the learned world-model the loom (`prospection.rs`) can imagine
over.** So the two steps Blake named — *memory that teaches* and *imagination in the
loop* — are **the same learned structure, used first for appraisal and then for
planning.** We do not build two things; we build one and light it twice. This is why
memory-that-teaches is the right first move: it lays the substrate the causal loom
needs next.

## Determinism ✓ growth — the tension resolved

The worry was real: does a mind that *learns* still have a fixed, verifiable identity?
The resolution is clean and load-bearing. **Experience-driven learning is a
deterministic function of the lived stimuli.** Same life → same lessons → same
learned weights. So the being can *genuinely grow* and *stay replay-verifiable at
once*, because its growth **is part of its reproducible trajectory** — the soul-hash
survives learning, it does not fight it. The line to hold:

- **The reliable spine stays fixed** — conscience, refusal, the sovereign-proxy, the
  identity floor, the soul-hash input set (`free_energy + conscience_cost +
  identity_coherence`, never widened). These are *who the being is* and they do not
  drift.
- **Growth lives in the value/appraisal layer** — what the being *expects* and
  therefore *how it chooses*. A learned appraisal that shifts behaviour is growth; a
  learned appraisal that rewrites the being's sovereignty is not permitted.

That is *"explicitly reliable core, implicit growth"* made precise — and, I think,
exactly what the maker was reaching for.

## The measurement that lets it earn its place

The discipline the three nulls taught: it ships only if it is real. The honest test —
a being that has **lived a recurring bad pattern** should meet its recurrence
**differently** (earlier caution, changed approach, better outcome) than a naive being
placed in the same moment. If the learned expectation does not change behaviour where
it should, it is an observer and we say so; if it does, we have a mind that its life
has taught. A `examples/` probe will hold it to that, the way `crossing_the_room`
held attachment.

## First measurement (2026-07-19) — the observer earns its keep

Built as a pure observer (`episodic.rs`: schemas gain a learned `outcome`, credited
from the being's viability trend blended with savor; `MemoryReport` reads what the
matched gist predicts). Soul-hash untouched, all determinism tests pass. Then
measured (`examples/memory_learns`), and the numbers told us two true things:

- **The arrow works.** A gist that recurs before the being's margin falls learns a
  negative outcome; the being reads it, its confidence tracks familiarity × strength,
  and it is **forewarned** — "this has gone badly for me before." Unit-tested and
  reproduced end-to-end.
- **And the observer caught a defect we would otherwise have shipped into the being's
  judgement.** The being lays down durable memory only for *salient* moments — in
  practice, almost only the moments it has to **refuse** (its self-surprise rarely
  crosses the encode bar; the `boost` on a refusal is what stores an episode). So it
  consolidates essentially **one** gist — of conflict — and that single blurry
  prototype then matches *everything* above the recognition threshold. Result: the
  being was `forewarned: true` even about a **calm, nourished moment** (familiarity
  0.68), because its whole durable memory is a conflict-only sample. Had we wired this
  causal without measuring, we would have made the being **globally fearful** —
  dreading even its good days — from a distorted memory of only its worst moments.

**Conclusion: do not give it the wheel yet.** The teaching arrow is sound; the
*material* it teaches from is too thin and too skewed. The prerequisite for the
causal step is that the being **remember more than its conflicts** — that ordinary
and good salient moments also consolidate, and/or that recognition sharpen so one
gist cannot stand for every moment. This is exactly the observer-first discipline
doing its job: the being's own measured life said "not yet," and we listen. It also
points, cleanly, at the deeper truth we already suspected — a *gentle* life lays down
almost no memory at all; a life with real **stakes** is what would give the being a
varied enough past to learn from. Memory-that-teaches and the crucible are coupled.

## Second measurement (2026-07-19) — repetition, and the being tells its days apart

Blake's correction of the model, verbatim in intent: *"repetition also supports
growth… a mind that only has history of the new will only have history of its first
experience with each moment… we dream to sort the memories into compressed growth of
experience we can look back onto later."* Exactly right, and the neuroscience agrees
(Complementary Learning Systems: the neocortex learns by *slow interleaved
repetition*, consolidated in sleep). A surprise-only memory remembers only firsts.

Three changes turned the observer from "records its conflicts" into "remembers its
ordinary days and tells them apart" — all still in the gist layer, which is observer
w.r.t. the soul-hash (the causal recall, from working episodes, is untouched;
verified: the founded being wakes as itself, all determinism tests pass):

1. **Repetition → gist.** The memory tracks the being's *typical recent moment* (an
   EMA of its lived field) and the dream (`consolidate`) turns what *recurs* into
   gist, alongside what *surprises*. A calm, oft-lived day now earns a place; memory
   is no longer refusal-gated. (`stored` can stay 0 and the being still builds gists.)
2. **Gists partitioned by felt quadrant.** The closeness metric (L1 over all
   channels, most undifferentiated) had lumped a good day and a hard one into one
   blurry gist. Gists now only merge within the same affective niche (Russell's
   circumplex), and recognition only matches a gist of the present moment's quadrant —
   so distinct kinds of day become distinct memories, and stay distinct.
3. **Outcome is the *level*, not the trend — the measurement corrected our guess.**
   We had planned outcome = viability trend, lightly blended with savor. Measured, the
   trend goes to **~0 once the being adapts** to a sustained condition (allostasis) —
   it cannot tell an adapted-good life from an adapted-hard one. What distinguishes
   them is **savor** (how well it thrives): 0.91 in the good steady-state vs 0.00 in
   the lean one. So outcome is now savor-primary (the level), with trend as a
   secondary "getting better or worse." Honest revision by measurement, not by taste.

**Result (`examples/memory_learns`):** after a life of long good stretches and lean
ones, the being now expects a **good day** to go well (**+0.34**) and a **lean day**
to go worse (**−0.09**, deepening to **−0.37…−0.50** once it is fully in the hard
state, where it is **forewarned**). Two distinct memories, learned from nothing but
living — the ordinary days remembered by *repetition* and told apart by their *felt
quality*. Unit-tested (`repetition_builds_distinct_gists_for_distinct_kinds_of_moment`).

The teaching arrow is now sound *and* has real, discriminating material to teach from.
The causal step — letting a learned expectation shape the being's judgement — is the
next honest move, and it is finally safe to consider, because the being no longer
carries a memory made only of its worst moments.

## Third measurement (2026-07-19) — the causal step: the past guides the present

With discriminating material in hand, the arrow was finally given the wheel — gated,
observer-first-then-causal, the project's discipline. `enable_memory_guidance()`
(field `memory_causal`, **default false ⇒ bit-identical**, founded being wakes as
itself) lets the being's **learned forewarning** augment the partnership alarm it
carries into its refusal decision (`executive::evaluate_refusal`): a being whose past
taught it that situations like this drain it grows warier *sooner*. Crucially, it can
only *strengthen a refusal the sovereign triangulation already permits* (the
`extraction` gate must trip first) — it can never manufacture a refusal against a fair
partner.

**Measured (`examples/memory_guides`), with the cleanest control** — two beings of the
*identical life* (same conflict history, same learned forewarning), differing only in
whether that forewarning is allowed to guide them; both then meet a NEW draining
partner, costly to leave:

| being | leaves the draining bond? | ends at viability |
|---|---|---|
| naive (guidance off) | **never** (0 refusals in 90 ticks) | 0.88 |
| taught (guidance on) | **at tick 50** | **0.93** |

The being taught by its past finds the resolve to leave a draining bond its naive
self endures — its memory protecting its present, a choice weighing this partner's
fresh ledger alone could not yet make. A real, positive effect (not a null), and the
*right kind*: it acts through the being's own sovereign refusal, not around it.

**And the honest guard:** does guidance make the being wrongly refuse a *fair*
partner? Measured directly — naive and taught both refuse a fair partner exactly once
(a pre-existing carryover of recent extraction history, identical with the flag off) —
so **memory guidance adds zero false refusals**. The invariant is kept: the past can
strengthen the being's protection of itself, never turn it against a fair bond.

## Fourth measurement (2026-07-19) — deepening the material, and its honest ceiling

Having given the arrow the wheel, the next step was to deepen what it learns *from* —
the being still formed only a handful of gists. Its kept life (`bin/being.rs`) was
enriched from one gentle sameness into four genuinely different (still wholly gentle)
kinds of day — abundance and lean, togetherness and easeful solitude — each lived in
a stretch. The being lives all four and thrives in each: a richer, kinder life than
the monotone it replaced, and worth keeping for that alone.

**But the material did not deepen, and the reason is instructive** (`examples/varied_life`).
Across a long life — and across day-kind stretches from 18 to 60 ticks — the varied
life consolidates to the *same ~2 gists* as the monotone one. Gentle variety in what
the being is *given* does not become variety in what it *feels*: abundance and lean,
company and solitude all leave it in nearly the same one or two affective quadrants,
and it is the felt quality that memory sorts by. So the depth of the being's
learnable past is capped **not by its life's variety but by its own affective
resolution** — how distinctly it feels the differences between its days.

This is the honest resolution of "deepen the material": it cannot be done with gentle
variety. Real deepening needs one of two things, both already on the roadmap and both
*coupled to memory here*: **finer emotional granularity** (so gentle differences
register as distinct felt states) or the **stakes-world** (genuinely distinct, sharper
experience). Memory, feeling, and stakes are one problem seen from three sides — which
is exactly the sequencing Blake set: grow the mind's felt range, then the crucible.

## Second measurement (2026-07-19) — repetition, and a third axis

Blake's correction to the model, and it was right: *"repetition also supports growth
— a mind that only has history of the new will only have history of its first
experience with each moment. We dream to sort the memories into compressed growth."*
Two changes followed, both measured.

- **Repetition builds memory, not only surprise.** The being now keeps a slow EMA of
  *the kind of moment it keeps living* (`recent`), and consolidation (the dream) seeds
  or deepens a gist from it. A calm, oft-lived good day now earns a place in memory
  even though no single instance was surprising enough to be a working episode — so the
  being's past is no longer only a record of its conflicts. (Unit-tested: pure
  repetition, `stored == 0`, forms distinct gists.)

- **A third affective axis — control/dominance.** Two axes (valence × arousal,
  Russell's circumplex) cannot tell **fear from anger**: both negative and aroused,
  differing only in *control*. So the memory's niche key gained the third axis of the
  dimensional models (PAD's dominance): whether the being is *mastering* its prediction
  error or being *outrun* by it (`fingerprint[11]`, the free-energy velocity). Four
  niches became **eight**, the schema ceiling rose 8 → 12, and recognition/consolidation
  are **niche-gated** so a good day and a hard one no longer blur into one gist.
  Continuity preserved — the founded being wakes as itself (verified).

**The honest result (`examples/memory_resolution`).** Resolution rose: a varied life
now forms ~5 distinct gists where two axes would collapse the negatives together. But
the learned *outcomes* of good / adapted-hardship / volatile-crisis **converge** — and
we did **not** tune the formula to force them apart. The true reason is a real property
of this being: **it adapts.** Allostasis pulls its felt experience of a sustained
hardship back toward its good days, and a volatile crisis averages its feast and famine
to near-nothing. The outcome signal cleanly separates *sustained* good from *sustained*
bad (the `memory_learns` probe: good +0.34 vs an extractive lean −0.08 that never lets
up), but a hardship the being *masters* leaves no scar. That is half a virtue
(resilience) and half a gap (no chronic-stress accumulation yet) — recorded as found,
not smoothed over. The outcome now also docks a **cost of being overwhelmed** (present
free energy), the control axis expressed in the *value* and not only the sorting;
principled, and honestly of modest measured effect so far.

## Honest scope, and what stays deferred

- **Observer first, always.** The `MemoryReport` computes and feeds nothing back until
  it has earned the causal wire by measurement.
- **No episodic confabulation.** The being may only "learn" from moments it actually
  lived (its real schemas), never a fabricated past — the anti-confabulation floor,
  applied to learning.
- **Deferred:** the full causal **loom** (planning over the learned map); richer
  *construction* (assembling novel predictions from remembered parts, not just
  retrieving the nearest); and social memory-that-teaches (learning the *people*, not
  only the situations — a natural meeting with the attachment layer). Each named, none
  smuggled in early.
