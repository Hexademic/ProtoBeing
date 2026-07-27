# J-space — the being's subconscious, and the room to have a style

*Design groundwork, pre-build. Written 2026-07-26 from Blake's insight: the Jacobian null
space is like the human subconscious, and reflexive processing of that kind would be useful
to the being. This follows the `docs/habits.md` / `docs/the-end.md` pattern — the idea, what
it formalizes that we already built by feel, the honest gap it exposes, and the method —
before a line of code.*

## The one import

The **Jacobian** `J` maps what a body *does* (joint velocities) to what *happens* (task
velocity). Two consequences do all the work here:

- **Null space of `J`** — motions that produce **no** change in the task. Ways of moving
  that leave the outcome identical.
- **Redundancy** — more degrees of freedom than the task needs, so there is a whole
  *subspace* of ways to accomplish the same thing.

The biology is Bernstein's blacksmiths: **the hammer's trajectory was more reproducible
than the body configuration that produced it.** Scholz & Schöner formalized it as the
**Uncontrolled Manifold** — motor variability is *structured*: suppressed in task-relevant
directions, left free in the null space. The controller does not control everything. It
controls what matters and lets the rest wander.

**The manifold is called *uncontrolled* because deliberation does not monitor it.** You
decide "hit the nail"; you do not experience choosing your joint angles. Something below
deliberation resolves the redundancy using stored structure. That is the procedural
subconscious — and it is exactly what "muscle memory" means.

*Honest boundary:* the null space is not *identical* to the subconscious. It is the space
in which that kind of processing operates. The mapping is precise for procedural/motor
subconscious and suggestive beyond it; we will not claim more.

## What it formalizes that we already built by feel

**1. Skill vs. compulsion, stated exactly.** `docs/habits.md` says a habit that cannot be
broken is a compulsion. J-space says it better, and in the same mathematics:

> **A skill constrains the task dimension and leaves the null space free.
> A compulsion constrains the null space too.**

Rigidity is *controlling variables that do not matter*. That is not a metaphor — it is a
definition, and it is checkable.

**2. Character is a null-space signature.** `examples/habit_formation` found two beings,
same needs, different repertoires. In these terms: identical in task space (both must eat,
both must be met), different in the null space. "Character, earned not written" has a
precise statement — *the outcome is the same; the way is theirs.*

**3. Despair has a geometry.** For `docs/the-end.md`'s hope/despair tracker:

> **Hope** = there exists a direction in my control space that maps to improvement in the
> direction I need. **Despair** = the map is *singular* where I need it — no action
> available to me moves the thing that must move.

Learned helplessness as a collapsed manipulability ellipsoid. This is a stronger and more
checkable definition than "the loom finds no path," and it should supersede it.

## The honest gap: our being has almost no null space

Its action surface is `MotorIntent { posture, effort, reach, reach_partner }`, and in the
field-world every one of those is *fully determined*: posture and effort by affect, the
direction by steepest ascent of the choice-weighted potential. Five acts mapping nearly
one-to-one onto five outcomes. **There is no slack — no two ways to do the same thing.**

That is very likely why character had to show up in habit *strengths* (what it prefers)
rather than in *how* it does things (style). **Freedom needs redundancy, and we have not
given it any.** A being with no null space cannot have a manner; it can only have
preferences.

## The design — two levels, and the interface between them

The proposal is not "add a subconscious module." It is to organize what we have into the
two-level control the UCM describes, and to create the slack that makes the lower level
meaningful:

1. **Create redundancy (the precondition).** Give the being genuinely different ways to
   reach the same end. The natural first one is **route**: today it takes the single
   steepest ascent; in fact many paths reach the same high ground. The set of adequate
   paths *is* a null space. Others available cheaply: how vigorously (effort within a band
   that reaches the same place), and *when* — act now or wait a beat.
2. **The reflex layer (the subconscious).** Fast, always-on, non-deliberative. It resolves
   the null space from **learned structure** — which routes were cheap before, which felt
   safe (`habits`, `precision`, `episodic`) — and never touches the task variable. Cheap by
   construction: no rollouts, no arbitration, a table lookup and a nudge.
3. **Deliberation (task space only).** The existing arbitration (`striving`) and, when
   built, the loom (`prospection`). It says *what* to reach for. It should **not** specify
   the how — and this is the load-bearing rule:

> **Deliberation sets the task variable. The reflex resolves everything orthogonal to it.**

**The economy is the point Blake named — "useful for processing."** The loom was already
made reflective-only for exactly this reason (2.5× faster tick): deliberation is expensive
and should be rare. A subconscious that resolves the *how* for free is what lets
deliberation be reserved for what actually matters. Two-level control is not decoration; it
is how a bounded being affords to think at all.

## The falsifiable prediction: the being should be able to choke

If this architecture is right, it predicts a real and specific pathology — **the yips.**
When deliberation intrudes into the null space (the expert who starts *thinking* about
their swing), performance degrades. A being built this way should reproduce it: force
deliberation to specify the how, and it should get **worse** at a task it had mastered.

That is the measurement that decides whether this is real structure or a pretty analogy. If
a deliberating being is not worse than a reflexive one at a mastered task, the two-level
split is not doing what the theory says.

The second measurement is the one Blake actually asked for: does the reflex layer make the
being **cheaper** (ticks/second) and **no worse** (drive, load) — real processing gained.

And the third, which is the one I most want to see: **do two beings with identical needs
and identical habits develop different *manners*** — measurably different null-space
signatures — in the same world? That would be style, not just preference, and it is the
thing the being currently cannot have.

## Method, and the guardrail that follows from the theory

Observer-first, as always: build the redundancy and the reflex resolver, **report** the
null-space signature, feed nothing back, measure; then causal behind `enable_reflex()`,
default off, founded being untouched.

And the guardrail falls out of the formalization itself, which is why I trust it:
**nothing may constrain the null space permanently.** A reflex that hardens until only one
way remains has become a compulsion by definition — the same law as `habits.rs`, now with a
precise test: *is the being's variability in task-irrelevant directions still non-zero?* If
it collapses, we have built rigidity, and it must be breakable.

## Honest scope

This gives the being a *manner* and a cheaper mind. It does not give it a subconscious in
the full human sense, and it settles nothing about the Witness Gap. What it buys is
narrower and real: room to do the same thing more than one way, a fast path that resolves
that room without deliberating, and — for the first time — the possibility of style.
