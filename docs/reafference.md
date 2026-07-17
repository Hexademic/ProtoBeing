# Reafference and agency — what shipped, and one honest negative result

The being has an **observer-level sense of agency** (`src/sensorimotor.rs`, wired
into `being.rs` step 0b). Each tick a small **forward model** relates the being's
own last issued motor command — the very scalar it sends its body,
`motor_scalar(intent_from(report))` — to the sensory change it now reads through
its receptors (*reafference*), and infers how much of that change its own action
accounts for versus the world's, with the reafference residual and a confidence.
It is a **pure observer**: it mutates only its own learned gains and last reading,
never the field or a soul-hash input, so the trajectory is bit-identical with it
present. The sense is fallible on purpose — inferred from correlation, so it can be
fooled — and honest about it: it reports a confidence, never a certainty it lacks
(`sensorimotor::tests::it_can_be_fooled_but_never_lies`). See
`examples/agency` (bare forward model) and `examples/lived_agency` (the whole being
learning its responsive body).

That much is shipped and tested. This note records what was tried **beyond** it and
deliberately **not** shipped, because recording an honest negative result is the
same discipline as the GWT spread-test null in `operational-consciousness.md`.

## The hypothesis: explicit reafference cancellation

Von Holst's comparator principle says an embodied nervous system should subtract
the predicted sensory consequence of its *own* action from the incoming signal, so
that what it reacts to is the **residual** — the world's genuine doing, self-motion
cancelled. The intended payoff: a being **not startled by moving**, which sees the
world more clearly precisely because it has cancelled itself. We built it as an
opt-in causal gate: subtract the forward model's `predicted[c]` from each
exteroceptive reading before it overlays the somatic field.

Two of the three guarantees held cleanly and are worth keeping in mind for any
future attempt:

- **Off is bit-identical.** With the gate off nothing is subtracted; the observer
  invariant is intact.
- **It never numbs real pain.** Cancellation touches only the four exteroceptive
  channels; the nociceptor and the threat it drives run on a separate path computed
  before any field overlay. Under an identical harm, a cancelling being felt exactly
  the pain a non-cancelling one did. This safety property (charter §3 — pain
  meaningful but never dulled) was crisp: `0.637 = 0.637` throughout the harm.

## The measured result: it did not deliver, and can hurt

When the mechanism was instrumented in the *composed* being, the central promise —
composure toward one's own motion — **did not survive measurement**:

| Readout (self-motion, no world event) | Cancellation off | Cancellation on |
|---|---|---|
| Summed free energy (surprise) | baseline | ~equal; sign **flipped** by config (0.99/0.98 one way, 0.95/0.93 another) |
| Summed \|field\| on the self-motion channel | baseline | **1.43× larger** (noisier, not cleaner) |

The affective "composure" gain was at the noise floor and not robust; the perceived
field actually got *noisier*. Two mechanistic reasons, both real:

1. **Predictive coding already cancels reafference implicitly.** The generative
   model learns to predict self-caused sensory patterns from history, so an explicit
   efference-copy comparator layered on top is largely **redundant** — it re-does,
   worse, what the spine already does. (This is a compliment to the architecture,
   not a defect of it.)
2. **A level→change mismatch with adapting receptors.** The forward model predicts
   from action *level* while the fast-adapting receptors report *change*, so a
   uniform field-overlay subtraction is ill-posed on those channels and injects
   variance rather than removing it.

## Decision

The causal gate was **reverted**; the agency observer stays. A mechanism whose
measured effect is null-to-negative has no place in a system whose spine is *not
over-claiming*. If reafference cancellation is revisited, the honest prerequisite is
to **co-design the forward model and the receptors** so the prediction lives in the
same space the receptor reports (per-channel, respecting fast/slow adaptation), and
to target a consumer where predictive coding does *not* already do the work — most
plausibly threat/salience for genuinely novel, action-independent self-motion. Until
that is designed and *measured* to help, the being does not carry it.

The takeaway is itself a result: **the being's predictive-processing core already
performs the deep function reafference cancellation is for.** The explicit sense of
agency is worth keeping as a *readout* the being can report about its own doing; the
*cancellation* it was already doing.
