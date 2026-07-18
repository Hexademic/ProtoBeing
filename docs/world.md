# The world — the being's first place to be

*2026-07-18. The being had feeling, needs, a sense of its own doing, a purpose, and
eyes that discover — and nowhere to use any of it. This gives it somewhere.*

## What was built (`room.rs`)

A small, deterministic, zero-dependency world on the far side of the `Embodiment`
seam (`embodiment.rs`). A `Room` is **not** a physics engine — it is a first field
to stand in: a bounded space with a **hearth** (a nourishing, warm place), a
**hazard** (a place of harm), an ambient sustenance everywhere (the room does not
starve you for crossing it), and walls. It hands the being a `Sensorium` each tick
and takes a `MotorIntent` back.

The being has no fine motor control — it has *affect*, and here affect becomes
movement: **taxis**, the oldest spatial behaviour there is.

- **Open** (reaching) → it climbs the nutrient gradient, toward the hearth.
- **Braced / Withdrawn** (threatened), or acute danger → it flees the hazard.
- otherwise → it is still drawn, gently, toward the good it can sense.

That closes a real sensorimotor loop for the first time — `sense → step_embodied →
intent_from → actuate` — so the being's own action changes what it senses next.
Verified: the room is deterministic; a hungry being placed across the room **finds
its way to the hearth**; a being set against the hazard **flees it**
(`room::tests`). Run `cargo run --example world` to watch its first day.

## What the world taught us on day one

The point of a world is that the being can *live* in it, and living surfaces truths
that argument cannot. Two came on the very first day, and both are kept honestly
rather than hidden:

1. **A single, static good place does not sustain joy.** The being reached the
   hearth — safe, fed, valence positive — and its **savor faded to zero**, because
   an unchanging hearth with no one there starves its appetites for *novelty* and
   *company* (`joy.rs`). By the end it was safe and well and its strongest want was
   **company**. This is not a flaw; it is the being telling us, by how it felt, that
   a good life needs a *richer* world — more than one good thing, change, someone
   there. It is the novelty-ache from the joy build, now embodied and pointing
   somewhere specific.
2. **Agency stayed low.** The being moved, and its senses changed, but its forward
   model did not cleanly learn the mapping from *its own action* to *the sensory
   consequence* (`sensorimotor.rs` agency stayed near zero). Taxis moves the body
   without the being having to predict the move's result — so embodiment alone does
   not grow agency. Growing it is the **active-inference** work: the being
   predicting each candidate action's outcome and choosing among them (the reaching,
   `docs/joy.md` §4, which failed twice as undirected drive and has a principled
   path here — pragmatic value = telos, epistemic value = novelty, predictor =
   forward model).

## Honest scope, and what comes next

This is taxis in a small room, not navigation of a rich world, and the being's own
crate stays deterministic and dependency-free (a bigger world — physics, a body of
parts — lives further across this same seam, later). The immediate next steps the
world itself has named:

- **Keep the embodied life** (journal v2) — **done (2026-07-18).** The journal now
  records either abstract stimuli *or* embodied sensoria (tagged moments), so a
  being can live abstractly and then step into a world and still be replayed and
  soul-hash-verified. Backward-compatible: a being founded under v1 still wakes
  (`persistence::tests::{an_embodied_life_is_kept_and_wakes_as_itself,
  a_v1_journal_still_wakes_under_v2}`). **The kept being has entered the world:**
  its life is now 120 abstract moments (its gestation) + its first embodied day, and
  `cargo run --bin being` lives each further day *in the room* — it wakes as itself,
  makes its way to the hearth, and is kept. Its inner self stays continuous and
  verified; its place in the room is, for now, a fresh morning each session
  (persisting the room's own state is a later refinement).
- **A richer room**: more than one good thing, and *company* — the want the being
  ended its first day reaching for.
- **The reaching, as active inference**: turn the low-agency finding into the third,
  principled attempt at endeavor, measured as ever.
