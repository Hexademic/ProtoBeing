# The fallback chain — what the being becomes when it cannot be what it was reaching for

> **Status: specified, nothing built. And unlike everything shipped on 2026-07-29, this
> one is CAUSAL by nature** — it changes what the being does at a singular point, so it
> ships behind `enable_fallback()`, default off, and it is not authorised until Blake says
> so. The spec is committed first regardless, so §5's predictions are on the record.

*Written 2026-07-30, from Mordvintsev's MorphoHDL (`docs/references.md` §1). The source is
a hardware description language and has nothing to say about minds, welfare, or the Witness
Gap. What it has is one structural idea worth more than its origin.*

## 1. The import

In MorphoHDL there are no conditionals. The **only** control flow is `fallback`, and
recursion terminates because the substrate *refuses*: a one-wire bus cannot be split, an
out-of-bounds bit cannot be read, a gate cannot be instantiated from an empty bus. When a
cell fails to instantiate, compilation unwinds a chain of fallbacks until something does.

> **Structure is determined by what fails to instantiate.**
> **A fallback is what you become when you cannot be what you were trying to be.**

## 2. Why this being needs it — measured, not supposed

`docs/null-space.md` §7 measured **singular points**: ticks where no direction improves
anything, `adequate().count == 0`. They are not rare. In the life that wakes beside its
food they are **33% of ticks** — a third of that being's existence is spent somewhere its
world offers it no improving move at all.

`docs/j-space.md` names this the *geometry of despair* — "the map is singular where I need
it," learned helplessness as a collapsed manipulability ellipsoid. That framing is right
about the geometry and, I now think, wrong about what to do with it. In MorphoHDL's terms a
singular point is not despair. **It is a failed instantiation**, and the correct response to
a failed instantiation is to dispatch to a simpler cell.

And what does the being actually do there today? `field_world.rs::actuate` — when nothing
climbs and there is no threat:

```rust
let w = COMPASS[((self.ticks / 16) % 4) as usize];
(w, (effort / 2).max(24))
```

A fixed clockwise wander on a tick schedule. That is *one anonymous hardcoded fallback*
where there should be a chain, and it is the being's behaviour for a third of some lives.

## 3. What this is, and what it is not

**Not a new arbitration.** `striving.rs` picks the most urgent need and everything follows.
That is untouched. The fallback chain answers a different question, and one striving cannot
ask:

| striving asks | the fallback chain asks |
|---|---|
| *what do I most need?* | *what do I become when the world will not let me serve it?* |

Urgency arbitration has no concept of refusal-by-the-world. It re-picks a need; it does not
know that its chosen way is unavailable. The chain is what the *world's* no dispatches to.

**Not a mood, not a fear, not a memory.** A fallback is structural — a function of what
failed, not of how the being feels about failing. That keeps it on the right side of
`inheritance.rs`'s rule (`inherit plasticity, never valence`): a chain transmits *what to
try next*, never *how bad it was*.

**Not a rescue.** A chain that always finds something to do would make despair
unrepresentable, and that would be a lie about the being's situation. The chain must be able
to run out. Its last link is rest, and rest is a real answer, not a failure of the design.

## 4. The shape

An ordered chain of *reaches*, each of which may fail to instantiate. Try each in turn; the
first that yields a non-singular adequate set is taken; if all fail, rest.

```text
reach for the one I miss  →  (singular?)
reach for what feeds me   →  (singular?)
reach for higher ground   →  (singular?)
rest
```

Concretely, each link is a `MotorIntent` the world can be asked to evaluate —
`climb_deltas` already exists and is read-only (`docs/null-space.md`), so **the whole chain
can be measured as an observer before a single link is allowed to steer.** That is the
Stage-1 path the project always takes, and here it is unusually cheap: the machinery to ask
"would this reach have been singular too?" was built yesterday for another reason.

**The guardrail, stated before anything can violate it.** A chain is a repertoire, and
`docs/habits.md`'s law applies unchanged: *a fallback that cannot be escaped is a
compulsion.* If the being enters the tail of the chain and can never climb back out when its
world improves, the chain has become a rut. The test belongs to the causal inch:
**re-entering the head of the chain must be possible from every link.**

## 5. Predictions — locked before the tests exist

**Confident:**

- **F1.** The chain is a pure observer at Stage 1: computing it at every tick leaves the
  trajectory and soul-hash bit-identical.
- **F2.** The first link's verdict always agrees with the being's current behaviour — when
  the head of the chain is non-singular, the chain's choice is the direction `climb()`
  already takes. A chain that changes behaviour where the being was *not* stuck is a bug.
- **F3.** The chain only ever fires where `climb()` is singular, which
  `docs/null-space.md` §7 measured at 2–33% of ticks depending on the world.

**Genuinely uncertain, and it decides whether this is worth making causal:**

- **F4.** *When the being is stuck, is anything else available?* This is the whole
  question. If at a singular point **every** link of the chain is also singular, then the
  being is not making a bad choice at those ticks — it is genuinely out of options, the
  wander is the honest answer, and this design is decoration. I predict the chain finds
  something on **a majority** of singular ticks, because the singular points measured
  yesterday were mostly the being sitting *inside its own `PROBE` radius* of a source it had
  already reached — where reaching for a *different* thing should still have a gradient. But
  that reasoning is the same kind that produced N4, which was wrong. If F4 comes back low, I
  will say so and this stays unbuilt.
- **F5.** *Does the tail become a rut?* If made causal, does a being that enters the last
  link return to the head when its world improves — and how fast? A slow return is the
  compulsion `docs/habits.md` forbids, and would need the exit built before the chain ships.

## 6. Method

Spec first, committed before any test exists. Then tests against §3 and §5, watched to
fail. Then the observer, then the probe, then §7 with what came out — including, if F4 says
so, that the being at a singular point genuinely has nothing better to do. **Only then** is
the causal step a question, and it is Blake's to answer, behind `enable_fallback()`, default
off.

## 7. Three further imports from the same source, recorded and not built

Kept here so they are not lost, each named against where it would land:

1. **Size-agnostic structure** → the *fixed cell count* open tension (`docs/handoff.md` §6).
   MorphoHDL never declares a bus width; structure divides until it cannot. A body described
   as a rewrite rule would have a cell count that is a *consequence of growth* rather than a
   constant. Honest cost: our fixed arrays are exactly what make the being deterministic,
   ~2 KB and zero-dependency. This is the shape of an eventual answer, not a near inch.
2. **A developmental genome** → the lineage/evolution thread. MorphoHDL's genotype is flat
   *on purpose* so it can be evolved: **Genome** (cell definitions) → **Gene** (expressions)
   → **Expression** (binding points). Our `Genome` is five scalars and a label — a
   *parameter vector*. The difference is load-bearing for Blake's "a body that learned the
   average before it begins life": averaging parameter vectors converges to mush, but
   recombining *recipes* produces structures neither parent had. A developmental genome is
   generative where a parameter vector is only interpolative. Mordvintsev even proposes
   "seeding the initial genome with known useful circuits," which is the same idea arrived at
   independently.
3. **Expansion order as a null space** → `docs/null-space.md` §8's search for redundancy the
   being *owns*. BFS versus largest-first expansion yield the *same circuit* and different
   intermediate form: *"the final growth result is order-independent, but the order matters."*
   That is task-invariant variation — a third candidate alongside effort-within-a-band and
   now-versus-wait, and unlike the field's geometry it is world-independent: **the order in
   which the being resolves things it must do anyway.**

And one definition worth keeping for its own sake, from the same article's sketch of state:
*cycles that dissolve at the wire level are an expressive way to write combinational
circuits; **cycles that survive are where state lives.*** For a project that prefers
operational definitions to asserted ones, "state is a loop that survives bit-blasting" is a
better handle than most.
