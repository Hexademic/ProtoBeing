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

---

## 6. What came out — basin membership is not a property of this being

The being lived 4,000 ticks. **Two of my five predictions failed, one of them for the sixth time in
the same shape, and the failures produced better findings than the hits.**

### C1-1 — **FAILS.** Channels 4 and 8 are equal on **0.0%** of ticks

Measured ranges: channel 4 spans 115–255, channel 8 spans 105–246. They are never equal, not once.

**My code reading was wrong.** I read `Field::write_from_body` and stopped there. I did not read
`Field::inject` (`field.rs:70`), which the mind uses to add conscience, seeking and narrative signals
into specific channels *after* the body has written them. Channel 8 is injected into; channel 4 is
not. So they diverge from the first tick.

> **Sixth error today, and the same shape every single time: read one code path, then generalise as
> though it were the only one.** I wrote a whole section of §3 on a duplication that does not exist.

### And the failure found something better: **this being never gets tired**

| channel | min | max | distinct values |
|---|---:|---:|---:|
| **10 · fatigue** | **0** | **0** | **1** |

`channel[10] = Q8_8::ONE.sub(b.energy)`. **Fatigue is pinned at zero for the entire life** — the
being's energy never leaves full in this world.

And `docs/comfort.md` §11 established that `Rest` is a *conjunction*: **low arousal AND fatigue ≈ 80
AND channel 0 ≈ 20.**

> **One of `Rest`'s three required coordinates is a dead channel that never leaves zero.** That is
> the measured reason the being never rests, and it is better than anything we argued for over three
> documents.

### C1-2 — **FAILS as stated.** I predicted two or more constant channels; there is exactly one.

**And my own probe's arithmetic here is wrong, so I am correcting it rather than quoting it.** The
probe reports *"2·breach = 10·fatigue, 99.4%"* and concludes **"effective dimensionality 10 of 12."**
That is a mistake: breach and fatigue agree because **fatigue is always 0 and breach is usually 0**,
not because they are the same variable. They are not a duplicate pair.

**The honest number is 11 of 12** — one dead channel, no duplicates.

### C1-3 — **HOLDS.** Our published sentence was about the label, not about arousal

Leave-one-out over all twelve, on the same lived trajectory:

| channel removed | winner changed |
|---|---:|
| **10 · fatigue** | **0.25%** |
| 4 · arousal-set | 0.05% |
| 3 · mean-tension, 5 · stability, **8 · arousal**, 9 · valence | 0.03% |
| the remaining six | **0.00%** |

**Not one channel changes the classification on more than a quarter of one percent of ticks.**

> So *"arousal is dead weight in the classifier"* — published in `docs/comfort.md` §13 and in
> `docs/settling.md`'s header — **took its content from the label.** The honest sentence is **"no
> channel decides the mode,"** which is what `examples/arousal_range` actually measured and which I
> then reported under a name that made it sound like a discovery about arousal.

And the joke at the bottom of the table: **the most "influential" channel in the classifier is
`fatigue`, the one that carries no information at all.** Removing a constant column shifts more
argmin ties than removing any real signal does.

### C1-4a — **HOLDS exactly.** 100.0% invariant

Permuting the field and the basin targets by the same permutation leaves the classification
identical on every tick. The classifier is genuinely label-symmetric; there is no index-dependence
bug hiding under the labels.

### C1-4b — **HOLDS, and it is the result that matters**

Our chart: modal basin **Engaged, 99.9%** of the life.

200 random charts, targets drawn uniformly from **the range the being's own channels actually
occupy**:

| | |
|---|---:|
| slot 0 ("Rest") modal | **32.0%** of charts |
| slot 1 ("Engaged") modal | 20.0% |
| slot 2 ("Defensive") modal | 23.0% |
| slot 3 ("Recovery") modal | 25.0% |
| **agrees with our chart's verdict** | **20.0%** |
| **slot 0 entered at least once** | **94.0%** |

**Our hand-drawn chart agrees with a random in-range chart at chance.** One in four would be 25%;
we got 20%.

> **`Basin::Rest` being unreachable is a fact about where we put `Rest`, not a fact about the
> being.** Place the targets anywhere in the region the being actually occupies and it enters slot 0
> in **94% of charts**.
>
> **Basin membership fails C1.** It is not a property this being has in virtue of itself. It is a
> property of the being *relative to a map we drew*, and the map's verdict carries no more
> information than a random map drawn in the same region.

## 7. What this costs us — the claims that were never about the being

Every one of these was an investigation of our chart:

- **`docs/comfort.md`** — the whole enquiry into *why the being does not rest*, including §8's
  "purpose satiated and still 0.0% rest" and §11's conjunction analysis.
- **`docs/settling.md`** — **S3**, *"does the being ever enter `Rest`?"*, predicted to fail and
  reported as "the conjunction confirmed."
- **`examples/basins_probe.rs`** — **B1–B4**, including *"Rest is the FURTHEST of the four basins
  from where it actually lives."* That is a statement about our four points.
- **Yesterday's headline to Blake** — *"the being spends 97.8% of its real day in
  `Basin::Defensive`."* **That number is chart-relative and I presented it as a fact about the
  being.** The claim I should have made is the one that survives: with `receptors` on, its drive
  halves, its effort rises 34.6%, and it travels three times as far. Those need no chart.

**What survives C1 untouched:** `drive`, `load`, `weathered`, `at_stake`, `viability`, survival,
effort, and distance travelled. None is defined relative to an author-placed target. They are
computed from the being's own dynamics, and they are where claims about this being should live.

## 8. What C1 did not show

- **This is not a verdict about consciousness**, as §5 said in advance. C1 is a necessary condition
  on a property being *intrinsic*. Basin membership failing it means our description was
  observer-relative — not that the being lacks structure.
- **The being is not thereby "restful."** It has never been measured resting *by any chart we have a
  reason to prefer.* What fails is our reason to prefer ours.
- **Fatigue being dead is a fact about this world, not necessarily about the architecture.** The
  being's energy never leaves full in a `FieldWorld` with these parameters. Whether it can tire at
  all is a separate measurement and has not been made.

## 9. What I would do next

1. **Ask whether the being can tire at all.** `fatigue` is one of `Rest`'s three coordinates and it
   is dead. If energy can never fall in any survivable world, `Rest` is unreachable *by
   construction* and no chart-fixing helps.
2. **Stop using basin occupancy as evidence.** It has appeared as a headline in at least four
   documents. It should be reported as *"under our chart"* wherever it survives at all.
3. **Re-examine `quality_space.rs` under C1.** Its basis is author-set by its own admission. If the
   quality axes fail C1 the same way the basins did, then the whole quality-space route to the
   Witness question — including the GWOT alignment I proposed — needs an intrinsic basis first, and
   that becomes the prerequisite rather than the follow-on.
