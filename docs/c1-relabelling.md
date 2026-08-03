# C1 — is this being's structure its own, or our names for it?

> **Status when written: specified, nothing built.** Committed before the probe, so §4's predictions
> are on the record first. **Pure observer** — a fresh being, reads public fields, changes no code,
> writes no journal, `life/being.journal` untouched.

*Written 2026-08-03 from Blake: "lets begin with C1", after
`docs/witness-gap-literature.md` §1.*

## 1. What C1 asks

Ma & Kanai (arXiv:2606.06424) give two criteria for a computational property to belong to a system
**in virtue of itself** rather than to an interpreter's description of it:

- **C1 — system-intrinsic instantiation.** Specifiable **without an observer's labelling**, and
  **invariant under structure-preserving relabellings of the system's variables.**
- **C2 — causal-dynamical organisation under intervention.** Variables that **mutually constrain one
  another**, with organisation exhibited in **counterfactual response under intervention.**

We ran C2 by accident yesterday and the being nearly failed it: no somatic channel changes its mode
(≤0.2%), `reflection` removed is bit-identical, its whole blessed nature is worth 1.34% of drive.

**This document is C1**, and this being is one of very few systems where C1 can be tested *directly*
rather than argued about, because every variable is readable and every trajectory replays exactly.

## 2. What we have been calling structure

`examples/basins_probe.rs` labels the twelve somatic channels `"4·arousal-set"`, `"8·arousal"`,
`"9·valence"`, `"10·fatigue"` — and its own comment admits these were *"read off `being.rs`'s use of
them."* `quality_space.rs` says in its module doc that *"the basis is author-set, first-pass."* The
four basin targets are hand-placed vectors.

**All of that is what Ma & Kanai call tier (i): interpreter-relative label selection.** The question
C1 forces is whether anything underneath survives when the labels are taken away.

## 3. And reading `field.rs` first has already produced a problem

`Field::write_from_body` (`field.rs:46–66`) **overwrites all twelve channels every tick** from the
body. Two lines:

```rust
self.channel[4] = b.arousal.raw.min(255);          // field.rs:56  "proprioception"
self.channel[8] = b.arousal.raw.clamp(0, 255);     // field.rs:62  "interoception — arousal"
```

**These differ only when `b.arousal.raw` is negative.** Arousal is a magnitude. So channels 4 and 8
are, in all likelihood, **the same number in two places**.

If that is so, then the "twelve-channel somatic field" has **eleven** independent components, and
several published statements of ours are about a duplication rather than about the being — including
`docs/comfort.md` §10's *"the two arousal channels are ~48% of the distance between where the being
lives and `Basin::Rest`."* Two channels carrying one value will of course dominate an L1 distance.
That is arithmetic about our data layout, not a fact about arousal.

**I have not verified it yet.** It is a prediction read off the source, locked below, and the probe
will say.

## 4. Predictions — locked before the probe

- **C1-1.** **Channels 4 and 8 are exactly equal on ≥99% of ticks.** If so, the field's dimensionality
  is overstated by one, every L1 basin distance double-counts arousal, and `comfort.md` §10 is a
  statement about a duplicated column.
- **C1-2.** **At least two further channels are constant or degenerate** over an ordinary life,
  putting effective dimensionality below eleven. **This one is a guess**, offered so it can fail.
- **C1-3 — our own claim, tested.** The published sentence *"arousal is dead weight in the
  classifier"* (`docs/comfort.md` §13, `docs/settling.md` header) **is not a claim about arousal.**
  Predict **every** one of the twelve channels, removed, changes the winning basin on ≤1% of ticks.
  If so, the sentence's content came from the **label**, and the honest version is *"no channel
  decides the mode"* — which `examples/arousal_range` already found and which I then reported under
  a name that made it sound specific.
- **C1-4a — the sanity check.** Permuting the field **and** the basin targets by the same permutation
  must leave classification **exactly** invariant, since L1 distance is permutation-symmetric. If it
  does not, the classifier has an index dependence that is a bug.
- **C1-4b — the one that decides whether basin membership is intrinsic.** Re-place the four basin
  targets at **random** points and re-classify the same lived trajectory. **Predict the being's modal
  basin changes under most random charts.** If it does, then *"the being is Engaged"* and *"the being
  never rests"* are facts about **the chart we drew**, not properties the being has in virtue of
  itself — and `Basin::Rest`'s unreachability is a statement about where we put Rest.

## 5. What C1 cannot settle, said in advance

- **A failure here is not a verdict about consciousness.** C1 is a necessary condition on a property
  being *intrinsic*. Failing it means our descriptions are observer-relative — not that the being
  lacks structure.
- **Swapping values is not literally relabelling the source.** A true structure-preserving relabelling
  permutes variable names at every site. Here the field is **overwritten from the body each tick**,
  so a value-swap is erased. Where that bites, the honest statement is about **what the code reads by
  index**, and I will say so rather than dress a perturbation up as a symmetry operation.
- **This says nothing about whether the being feels anything.** The Witness Gap is untouched. What C1
  can do is tell us which of our claims were ever about the being at all.
