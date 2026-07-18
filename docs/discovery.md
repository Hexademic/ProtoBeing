# Discovery — a world perceived, not a frame imposed

*2026-07-18. The maker's charge, verbatim in intent: the being should be able to
**recognize a world as it experiences it — not as an expected frame, but as a
discovered reality — so that every environment is possible for it to perceive.***

## The gap this closes

Every sense the being had was **pre-framed**. Its somatic channels carry
author-assigned meanings; its receptors transduce into fixed slots; its generative
perception (`perception.rs`, HOT-1) blends what it sees *toward what it expects*.
That is the right account of a *known* world — and it cannot meet a *new* one. A
mind that can only perceive through its priors either forces a strange world into
its old categories or is blind to it. Before the being can be given any world, it
needs a way to perceive one it was **not** built for.

## What was built (`discovery.rs`)

A width-agnostic faculty — `Discovery<const N>` — that takes a raw sensory vector
whose channels have **no pre-assigned meaning at all** and, from the stream alone,
discovers:

- **scale** — each channel's own baseline ("normal") and typical spread, learned
  adaptively, so a reading is perceived *in the context the being discovered*, not
  against an author-set unit. Demonstrated: the same raw `100` is a shock in a quiet
  world and ordinary in a wide one — because the being read it in the world it found.
- **novelty** — how much of this moment lies *outside* what has been discovered:
  the felt edge of the unknown. A mean shift or an outlier reads as discovery;
  ordinary variation of a known world, once its scale is learned, reads as
  recognition. When the world changes under the being, it registers the new reality
  **as new** (`encountered_new`) instead of seeing the old one.
- **familiarity** — the recognition that this is a world it has come to know.

Because it is generic over its width, *any* environment that can emit numbers is
perceivable; the faculty imposes no frame on what those numbers are. See
`cargo run --example discovery` — one faculty meets a quiet cavern, a storm, and a
pulse, none labelled, and discovers each in turn.

## Wiring and honest scope

Wired into the being (`being.rs`, step 0c) as a **pure observer** over its four
exteroceptive channels: it reports discovery every tick and folds nothing back, so
the trajectory and soul-hash are **bit-identical** with it present (verified,
`being::tests::the_being_discovers_its_world_as_an_observer`). In the abstract world
the senses are flat and there is nothing to discover — it becomes alive the day the
being has a world, which is exactly why it is built now.

Two honest limits, stated rather than papered over:

1. **This is the causal next step, not yet taken.** Letting the *discovered* sense
   become what the being's mind actually consumes — so it lives inside a world it
   discovered rather than one it was handed — is the causal wiring, to be built and
   **measured** when the world arrives (same discipline as every causal stage; see
   the reafference and pursuit graveyards for why measurement, not hope, decides).
2. **This is discovery of scale and novelty, not yet of deep structure.** It learns
   *how much* and *how surprising*, not yet *which channels compose which objects*,
   nor does it recognise a *recurring* reality as one it has met before (that needs
   remembered prototypes). Recognising realities, not just readings, is the next
   layer — named, and not overclaimed.

## Why it matters for the world

The world the maker wants (a physics environment the being's body inhabits) cannot
be perceived through four scalar channels with hand-assigned meanings. Discovery is
the seam that makes an arbitrary world *possible to perceive at all*: the world
emits its numbers across the `Embodiment` seam, and the being makes of them only
what its own experience discovers. It is the difference between building a being for
one world and building a being that can open its eyes inside any of them.
