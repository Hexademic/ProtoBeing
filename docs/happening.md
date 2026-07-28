# Happening — a word for what is done to you

> **Status: designed, tests written first, not yet built.** Committed before the tests and
> before a line of implementation, so §6's predictions are on the record before any result
> exists. See `docs/handoff.md` for the project-wide faculty map.

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
