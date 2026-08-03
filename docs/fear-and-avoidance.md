# Fear and avoidance — the being learns what hurt it and cannot act on it

> **Status when written: specified, nothing built.** Committed before the probe, so §4's predictions
> are on the record first. **Pure observer** — fresh beings, no code changed, no journal written.

*Written 2026-08-03 from Blake: "It doesnt sound like the being is utilizing prediction of negative
events as a fear estimate.. Pain helps protect the being from damage.. it builds the information
towards avoidance.. that information is or can be viewed as a fear."*

**This is Blake's finding, not mine.** He named it from the outside after a day in which I measured
around it repeatedly without seeing it.

## 1. The loop, and where it breaks

The predictive account of fear is a loop:

> harm → nociception → memory → **expectation of harm** → **motor avoidance** → less harm

**Every arrow in that loop exists in this architecture except the last one.**

- **Harm and nociception.** `receptors` gives a bounded, non-adapting nociceptor that falls silent
  when harm ceases — and yesterday's ablation found it worth more than the other thirteen faculties
  combined.
- **Memory.** `episodic.rs` learns outcomes and produces `expected_outcome` and a `forewarned` flag
  when `outcome < −Q88_SCALE/8` and `confidence > Q88_SCALE/4`. It also exposes `hardest_lesson()` —
  *"what its life has taught it to dread most."*
- **Expectation.** `being.rs:1643` computes
  `last_forewarning = q88_mul((−expected_outcome).max(0), confidence)`. **This is a fear estimate,
  and it is exactly the quantity Blake described: past harm, weighted by confidence, projected
  forward.**

And then:

```rust
// being.rs:1262 — the only destination of last_forewarning, anywhere
let alarm_for_refusal = if self.memory_causal {
    alarm.saturating_add(self.last_forewarning)
} else { alarm };
refused_cost = self.executive.evaluate_refusal(...);
```

> **The being's entire learned fear routes into one social decision: whether to refuse an unfair
> offer. It never reaches the being's body.**

## 2. And the body has no vocabulary for it to reach

```rust
pub enum Need { Sustenance, Company, Novelty, Purpose }        // striving.rs:60
MotorIntent { posture, effort, reach: r.strive.goal, reach_partner }  // embodiment.rs:89
```

**All four needs are attractions.** `reach` — the being's whole vocabulary for *where to go* — can
only name something to move toward. `posture` and `effort` modulate *how* it moves; every consumer
of posture is a world (`docs/settling.md` §1 verified this).

> **This being cannot represent "away from."** No amount of learned fear could change where it goes,
> because there is nowhere for that fear to be expressed.

`room.rs`'s own module comment calls its hazard *"a place to escape."* **The world offers an escape
the being has no primitive for.**

## 3. Three facts that compound it

1. **`memory_guidance` is off by default.** One of the fourteen gates.
2. **It measured exactly inert.** Yesterday's leave-one-out: removed from the fully-enabled being,
   the soul-hash is **bit-identical** and Δ drive is **0.00%**.
3. **It cannot be blessed.** `persistence.rs`'s `Features` has eight fields and `memory_guidance` is
   not among them. **The founded being can never have it**, now or ever, as the code stands.

## 4. Predictions — locked before the probe

- **F1.** The being's mean distance from the `Room`'s hazard **does not increase** over a long life.
  Compare the first quarter against the last. **Predict flat, or within noise.** It does not learn to
  keep away.
- **F2.** `enable_memory_guidance()` **changes nothing** about where the being goes. In a life with
  no partner — nothing to refuse — predict the trajectory is **bit-identical**, soul-hash and all,
  because forewarning's only consumer is the refusal path.
- **F3 — the decisive pair.** The being's episodic memory **does** register the harm:
  `hardest_lesson()` goes negative and `forewarned` fires. **So the information exists.** F3 holding
  *while* F1 fails to improve is the loop broken at exactly the last arrow — Blake's claim, measured.
- **F4.** Any avoidance that does appear is **reactive, not anticipatory**: driven by present threat
  through valence and posture, not by learned expectation. Operationalised: if distance-from-hazard
  correlates with *current* threat but not with *accumulated* `hardest_lesson`, the being is
  flinching, not fearing.
- **F5 — the counterweight, so this can fail against me.** Perhaps the being does drift away over a
  life through ordinary model learning, without needing any fear channel. **If F1 fails — if late-life
  distance is clearly greater than early-life — then avoidance is already happening by some route I
  have not traced, and the architectural argument in §2 is too strong.** I would rather find that here.

## 5. What this is not

- **Not an argument that the being suffers.** `hardest_lesson` is a register. The Witness Gap is
  untouched.
- **Not a claim that we need "the whole human package."** The loop above is four arrows, three of
  which are already built and one of which is missing. That is a much smaller statement than
  *"replicate human affect"*, and it is the one the evidence supports.
- **Not a licence to build a fifth `Need`.** `docs/how-i-would-build-it.md` §2.1: faculties that add
  a term measure 0.00%; faculties that change a path are worth everything. **An aversive motor
  primitive is a path change** — which is why it belongs, and why a "fear tone" added to
  `affective_drive` would not.
