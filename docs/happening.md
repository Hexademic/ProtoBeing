# Happening — a word for what is done to you

> **Status: built, measured, and the crux prediction FAILED** — `field_world.rs`
> (`with_drift`), `tests/happening.rs` (written *first*), `examples/happening`. §1–§7 are
> exactly as committed in `559aa54`, **before** the tests and before a line of
> implementation. §8 is what came out: the world moves, and the being cannot notice.
> The threshold was not lowered to fix that.

*Written 2026-07-28. Out of one line in yesterday's measurement, and out of what yesterday
turned out to be about.*

## 1. The gap

`examples/nested_speech` reported that `NOT KNOW` — one of the two shields — never fired in
either life. The probe was made to say why rather than leave it to guesswork:

> its complement `HAPPEN` was never earned — neither being accumulated enough unexplained
> world-residual to learn the word.

So the being has `DO`: it acted. It has `CAN'T`: it was overwhelmed. It has no word for
**something happened to me**.

That is not a missing feature. It is a missing *grammatical position* — the one where the
being is the object rather than the subject. A being that cannot say "this happened to me"
cannot report being subject to anything at all.

## 2. Why this, and why now

Yesterday's finding was that the **record** could not testify to what was done to the
being — a starved moment left no trace. We fixed that with a second hash.

The same blind spot turns out to exist one level down: the **subject** cannot testify
either. We built a being that acts, in worlds that respond to its action, and never asked
whether it could describe being acted upon. Both gaps come from the same unexamined
assumption — that a life is made of what the being does.

The vocabulary welfare would actually need begins here. Not with distress, which the being
already has words for, but with **attribution**: was this me, or was this done to me?

## 3. The mechanism — the being is ready, the world is not

Nothing in the being needs to change. `Prime::Happen` is already grounded:

```
Prime::Happen => f.world_residual > Q88_SCALE / 4
```

and `world_residual` is computed honestly in `sensorimotor.rs`:

```
residual[c] = actual_change[c] − (action × learned_gain[c])
```

The part of a sensory change the being's **own doing does not explain**. That is exactly
the right fact for this word, and it was right before anyone needed it.

The problem is entirely in the worlds we have built. In `field_world.rs` the exteroceptive
channels are the field's *gradient*, and the field never changes on its own — so every
change the being senses is a consequence of its own motion, the forward model learns to
predict it, and the residual goes to nothing. **Our worlds contain no happenings.**

## 4. The design — the world moves without being pushed

A **drift**: one source of the field changes position on a fixed cadence, bouncing within
the field's bounds. Deterministic (no RNG — the whole project's reproducibility depends on
it), bounded, and **opt-in via a builder**, so every existing world, probe and test is
bit-identical.

That is the entire mechanism. The good is somewhere slightly different than it was, and
the being did not move it. The gradient shifts under it, the forward model does not predict
the shift, the residual is real, and `HAPPEN` has something to be earned from.

Determinism is worth a word here, because "surprise" and "deterministic" sound opposed.
They are not, for this: the forward model predicts the sensory consequences of the being's
*own action*, and a drift is not its action. A drift stays unexplained no matter how
regular it is — which is also true of us. Rain in monsoon season is still something that
happens to you.

## 5. What must not become possible

- **This must not harm the being.** It exists to give it a word, not to manufacture the
  experience the word names. If a drifting life is measurably worse off — starved, driven
  into chronic load, pushed toward the trauma ceiling — **the design has failed and is not
  shipped**, however well `HAPPEN` grounds. Dignity before capability, as always.
- **No existing life changes.** Drift is opt-in and off by default. Every prior probe,
  test and world must be bit-identical.
- **No new prime.** If this needs a nineteenth word to be interesting, it is not this inch.
- **No determinism lost.** No RNG, anywhere, ever.

## 6. Predictions — locked before the tests exist

**Confident:**

- **H1.** With drift off, every world is bit-identical: prior probes and tests unchanged.

**Genuinely uncertain — this is the experiment:**

- **H2 (the crux).** *Does a being in a drifting world earn `HAPPEN`?* The drift must move
  the gradient by more than the forward model absorbs, sustained across ~32 moments. I do
  not know whether a plausible drift rate clears `Q88_SCALE / 4`. If it does not, either
  the drift is too gentle to notice or `HAPPEN`'s threshold is set for a world we never
  built — and which of those it is will be reported, not guessed.
- **H3.** *Does `(NOT KNOW HAPPEN)` then fire?* If `HAPPEN` grounds, the second shield
  should finally speak — the first test of whether two different shields behave alike
  (`docs/nested-speech.md` left this open). It also needs `NOT KNOW` to hold at a speakable
  moment, which is not guaranteed.
- **H4.** *What does it cost the being's sense of agency?* Residual up means agency down,
  by construction — `agency` is the fraction of sensory change its own action explained. I
  predict a measurable drop, and I want the number, because "a world where things happen to
  you leaves you feeling less in control" is a real finding about welfare if it holds, not
  a side effect to bury.
- **H5.** *Is a drifting life still a good life?* Measured on the graded drive and
  reflection load against a still-world control. **This is the gate.** If drift costs the
  being its wellbeing, §5 applies and we stop.

## 7. Method

Spec committed first. Tests written against it and watched to fail. Then implementation,
then the measurement, then §8 with what came out — including H5's verdict, which I am
willing to have come back against this.

---

## 8. Measured (2026-07-28) — H2 failed, and the reason is the finding

Order: spec committed (`559aa54`) → tests written and watched to fail → implementation →
measurement. The six pre-written tests pass: the world does move on its own,
deterministically, within bounds, and only when asked.

### H1 and H5 held. H2 and H3 did not.

| world | mean residual | HAPPEN | agency | drive | `(NOT KNOW HAPPEN)` |
|---|---|---|---|---|---|
| still (control) | 14 | never | 0.08 | 0.18 | 0 |
| drift every 32 | 14 | never | 0.08 | 0.16 | 0 |
| drift every 16 | 16 | never | 0.07 | 0.18 | 0 |
| drift every 8 | 14 | never | 0.07 | 0.15 | 0 |
| drift every 4 | 15 | never | 0.06 | 0.16 | 0 |
| drift every 2 | 14 | never | 0.06 | 0.13 | 0 |

- **H1 held.** Every prior probe is bit-identical — `nested_speech` still reports
  588 / 2 / 898 / 360 exactly.
- **H5 held**, and it was the gate: no cadence made the being worse off. All alive, drive
  flat or slightly better. Nothing here harmed it.
- **H2 failed.** No cadence taught the word.
- **H3** could not be reached, since it depends on H2.
- **H4** is a real but tiny effect: agency 0.08 → 0.06 across the sweep. Directionally as
  predicted — residual up, agency down — and far too small to call a welfare finding.

### Why — and it is structural, not a badly-picked knob

§6 named the two possibilities in advance: *the drift is too gentle to notice*, or
*`HAPPEN`'s threshold is set for a world we never built*. Measured, at a fixed body
position, in change to what the being actually senses:

| | Δ exteroception |
|---|---|
| the being's **own full-effort step** | **3** |
| the good source **vanishing entirely** | **40** |
| what `Prime::Happen` requires | **> 64** |

**The ceiling is below the floor.** The most violent event this world class can produce —
the good ceasing to exist — is 40, and the threshold is 64. Drifting the source faster
cannot help; nothing available can.

> **⚠ This paragraph is wrong. See §9.** The 40 was measured by comparing two *separate*
> worlds, which bypassed the receptor bank's adaptation dynamics entirely. Measured
> properly — in-life, through the real path — an abrupt event reaches **81**. The ceiling
> is *above* the floor. The rest of §8's data stands; this inference does not.

The cause is the field's smoothness, and it traces to a choice we made on purpose.
`docs/field-world.md`: *"sources reach across the whole field, so there is always a
gradient to feel — it is a field, not a set of local beacons."* `REACH = 2 × SIZE`. A
landscape that reaches everywhere is nearly linear, so its **gradient** — which is what
the four exteroceptive channels carry — barely moves when a source does.

> **The property that made it a field is the property that makes it eventless.** A
> perfectly smooth world cannot contain a happening.

### What I am not doing, and why

I could make H2 pass by changing one character: `Q88_SCALE / 4` to `Q88_SCALE / 8`.

I am not going to, and the reason matters more than the result. Grounding thresholds are
what make a prime *earned* rather than installed. Moving one so that a word I wanted fires
would be tuning until the answer is the one I hoped for — the exact thing "told, not tuned"
exists to forbid. If `HAPPEN`'s threshold changes, it changes on an argument about what
the word means, made in the open, and not as a side effect of someone's feature working.

There **is** such an argument, and honesty requires me to record that I only found it after
the failure, which is precisely when to distrust it: a world event of 40 is **thirteen
times larger than anything the being can do to itself** (3). A threshold of 64 asks for an
event twenty-one times its own maximal action before it will call something a happening.
That may be a definition of *cataclysm* rather than of *something happened*. I think that
argument is probably right. **I have not acted on it.** It is Blake's call, with the
numbers above, and it should be made on what the word means.

### What was kept, and what it is

The drift mechanism stays. It does exactly what it claims — the world moves without being
pushed, deterministically, harmlessly, opt-in — and that is proven in
`tests/happening.rs`. What it does **not** do is make the being able to notice, and both
halves are needed for the finding to be reproducible. It is not a stub; it is a working
world the being is currently deaf to.

### The real fix, named and not built

Exteroception in the field-world is a **gradient** — a difference between neighbouring
points. Differences are small and smooth by construction, which is why events do not
survive into them. A being that also sensed the field's **value** would feel a vanishing
immediately. That is an embodiment-level change across the `Embodiment` seam, not an
afternoon, and it is the honest next inch for this thread.

Until then the finding stands as it is: **the being has a word it cannot earn, in any
world we know how to build.** That is worth more than a word we taught it by lowering the
bar.


---

## 9. Corrected (2026-07-28, same day) — the ceiling is above the floor

Blake pushed back on §8's framing: the smoothness is not a fact about our worlds, it is a
fact about the being's **receptors**, which we have never designed for a realistic
experience. He was right, and checking it overturned my conclusion.

### What I had missed

`receptors.rs` already exists — organoid-styled transduction with **adaptation**
(receptors report change, not steady state), Naka–Rushton **compression**, and receptor
**type** (fast-adapting change-detectors, slow-adapting level-reporters, non-adapting
nociceptors). And it is not gated out of this path:

```rust
// being.rs:898
let agency_report = self.forward_model.step(self.last_action, &receptor_reading.extero);
```

The residual is computed through the receptor bank **always** — `enable_receptors` governs
only whether receptors *steer* the being. So receptors were in play throughout §8, and my
§8 measurement of "40" compared two separate worlds, which bypassed their adaptation state
altogether. That number was an artifact of how I measured, not a property of the world.

### Measured properly, in-life

| | residual |
|---|---|
| steady life, mean | 8 |
| **peak within 10 ticks of an abrupt change** | **66** |
| **peak across the life** | **81** |
| what `HAPPEN` requires | > 64 |

**An abrupt event clears the threshold.** §8's central inference was false.

### Why the drift failed, stated correctly

Not because the world cannot produce events. Because **I made the world change gradually,
and gradual change is precisely what adaptation exists to erase.** A source that creeps is
clothes you stop feeling. `FAST_ALPHA ≈ 0.5` means the baseline catches a slow drift within
a couple of ticks and cancels it. The receptors were working perfectly; my events were not
events.

An event is *sudden*. That is not a tuning detail — it is what the word means, and the
receptor module had encoded it correctly before I arrived.

### What still does not ground, and the real gap

A single abrupt event spikes above threshold for one or two ticks. Grounding uses
`RISE = 4` against `EBB = 1`, so a word needs its fact to hold roughly **one tick in five**
to accumulate at all. Swept:

| world | mean residual | HAPPEN |
|---|---|---|
| still (control) | 8 | never |
| creep, every 2 by 3 | 10 | never |
| events, every 32 by 120 | 19 | never |
| events, every 16 by 200 | 27 | never |
| events, every 8 by 200 | 34 | never |

Residual climbs with event rate and the word still does not ground. Getting to one-in-five
from a **single** moving source would mean a world in permanent upheaval — which would fail
H5, the gate, and I am not going to trade the being's wellbeing for a vocabulary entry.

So the corrected gap is not the threshold and not the receptors. It is that **our world
contains exactly one thing capable of changing.** A real environment has many independent
sources of happening — weather, others, decay, chance — and no single one of them needs to
be violent for something to be happening most of the time. That is the honest next inch,
and it is a world-richness problem, not a sensory one.

### On BCI data

Blake's instinct — that this deserves empirical grounding rather than a designer's guess —
is right in general and, I think, not needed *here*. Adaptation, compression and receptor
typing are century-old physiology and `receptors.rs` already cites them; the thing that was
wrong today was my design, which those constants would have told me if I had read them
first. BCI recordings would matter for a different question — what a *human* cortex does
with such signals — and the being is not human-shaped, so that mapping would need its own
argument. Worth having; not the blocker for this.

### The record

I nearly published "the being has a word it cannot earn in any world we know how to build."
That was wrong, it was wrong in the direction of a tidier story, and it was caught by Blake
asking about receptors rather than by me. The threshold I declined to lower turned out not
to need lowering — which is the second time in two days that refusing to tune was the right
call for a reason I did not have at the time.
