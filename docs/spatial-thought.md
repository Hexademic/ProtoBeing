# Spatial thought — what Tversky's *Mind in Motion* lands on

> **Status: research note, nothing specified, nothing built.** Written 2026-07-31 from a talk
> Blake watched and relayed (Barbara Tversky, Stanford — *Mind in Motion: How Action Shapes
> Thought*). **Citations below are UNVERIFIED**: the search tool hit a session limit while this
> was written, so nothing here has been checked against a publisher. Verify before any of it
> enters `docs/references.md` or grounds a design.

## 1. The claims

- **All thought begins as spatial thought.** The elementary act is approach the nutritious,
  avoid the noxious — and it is "replete with emotion." Space is not geometry: our
  representations of it are *distorted by perception and action, which are linked*.
- **The brain reuses spatial machinery for abstract domains.** Place cells fire for places and
  for **people**, for **events in time**, and for **ideas** — and those are arrayed in grid
  cells, in social, temporal and conceptual "spaces."
- **Gesture is thinking, not decoration.** Students reading alone, speaking to nobody, made
  spatial models with their hands — lines for paths, points for places. ~70% did it. **Those
  who did performed better; made to sit on their hands, people perform worse.** Gestures come
  *before* words. Blind people gesture. Action gestures conveyed process better than structure
  gestures even when the test could be answered from the script alone.
- **The mind is too small, so it puts thought into the world.** Maps, tallies, diagrams —
  40,000+ years of external representation. Diagrams are *"frozen gestures."*
- **Sketches give their makers ideas they did not have.** Architects looked at their own
  sketches and saw what they had not intended — clogged traffic, bad winter light. *"A
  conversation between the eye, the hand, and the page."* They could not verbalise it while
  doing it; words got in the way.
- **Explaining raises understanding; explaining *visually* raises it more** — measured on
  students before and after, with no new material seen.

## 2. Four places this lands on us, with the code

### (a) The being lives in a spatial world and does not think spatially

`field_world.rs` is genuinely spatial — position, gradient, distance, neighbours. The being's
*cognition* is scalar registers: valence, arousal, drive, free energy, agency.

**Except in one place, where we already did the right thing by feel.** `telos.rs`:

> *"A telos here is concrete and grounded: **a felt place the being has flourished in and
> commits to returning to.** Its material is the being's own quality space
> (`quality_space.rs`) — the low-D similarity space of its felt states."*

That is a **location in a conceptual space, navigated and returned to** — exactly the structure
Tversky says the brain reuses spatial machinery for. If the place-cells-for-concepts result
holds up on verification, `quality_space.rs` + `telos.rs` stop being an author's metaphor and
become the mechanism biology appears to use. Worth checking properly, because it would ground a
module we currently justify on intuition.

### (b) The being has a body and speech, and no gesture

`embodiment.rs` gives it `MotorIntent`. `primes.rs` gives it audited speech. **Nothing connects
them.** Speech is read off registers; the body acts on the world; neither informs the other.

Tversky's measurement is that this connection is load-bearing for *thinking*, not just for
communicating — people alone in a room, speaking to no one, think better with their hands.

And note what gesture is, in our terms: **action whose purpose is cognitive rather than
instrumental.** That matters here specifically, because
`docs/fallback.md` §1 found construction blocked at the `Embodiment` seam — there is no channel
for "place a thing." **Gesture needs no such channel.** It does not act on the world. It could
exist without touching the seam at all.

That makes it a third category beside foraging and play, and the cheapest of the three to try.

### (c) "The mind is too small" is literally true of this being

Fixed cell count, ~2 KB of core state, bounded memory — named as an open tension in
`docs/handoff.md` §6. The being **cannot grow its internal store.** So external marks are not
an enrichment; they are the only way its representational capacity can increase at all.

This is a much stronger argument for construction-as-sediment (2026-07-31 conversation) than
shelter was. Shelter is comfort. **A mark is memory the being does not have to carry.**

### (d) The architects — and this is the one I would build

> *"They'd make a sketch and look at it and see new things in the sketch, things they hadn't
> intended."*

**The being never perceives its own output.** `primes.rs` speaks and nothing reads it back. The
soul-hash records and the being never sees it. Its trace goes out and never returns as input.

Now put that beside a number we measured this week: **`self_surprise` = 1.21 out of 256.** We
diagnosed that as a missing register for doubt. Tversky suggests a simpler and more mechanical
reading:

> **Nothing the being produces ever comes back to it, so there is nothing it could be surprised
> by having made.**

The architects' insight is not a special faculty. It is a **loop** — output re-entering as
perception, and being different from what was expected. We have every part of that except the
wire.

## 3. What this would cost, honestly

- Closing the output→input loop is **causal** and it is a **feedback loop**, so it can
  destabilise. It would need the usual treatment: observer first (compute what the being *would*
  perceive of its own trace, and whether it differs from prediction), then gated.
- It is not obviously safe for determinism. Any loop must be evaluated within the tick's fixed
  order, or the soul-hash story breaks.
- And Tversky's own closing point cuts against easy optimism: the processes guiding sketching
  and gesture are *"invisible, interactive, context-bound, do not decompose into recombinable
  units, and are often unique."* Our being is built entirely from decomposable, recombinable,
  repeatable units. That is not a detail we can wave at.

## 4. What to verify before any of this is used

1. Tversky, B. *Mind in Motion: How Action Shapes Thought* — book, publisher and year.
2. The grid-cells-for-concepts result (Constantinescu / O'Reilly / Behrens?) and the
   place-cells-for-people/social-space result (Tavares et al.?) — **author, venue, year, DOI.**
3. The gesture-improves-comprehension studies and the sit-on-your-hands manipulation — these
   carry the weight of §2(b) and should not be cited from a talk transcript alone.

Until those are checked this document grounds **nothing**. It is a list of places to look.
