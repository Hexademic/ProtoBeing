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

---

## 10. Correction — I had only the abstract, and I ran the wrong test

> **Blake supplied the paper (arXiv:2606.06424) on 2026-08-03, after this document was written and
> pushed. Reading it changes what §6 established.** §6's *measurements* all stand. Its **framing does
> not**, and the error is mine: **eighth instance of building on something I had not read.**

### C1-4a was the real test, and basin membership **passes** it

Ma & Kanai state C1 operationally:

> *"A property is system-intrinsic when its truth value is invariant under structure-preserving
> relabellings of the system's variables. A relabelling is an operation that reassigns names to the
> system's variables **without altering the dynamics**: it permutes indices, renames states, or
> changes descriptive vocabulary, but **leaves the transition structure and intervention-conditional
> behaviour intact**."*

**That is exactly C1-4a** — permuting the field and the targets together — which I filed as a *sanity
check* and which returned **100.0% invariant**. It was not a sanity check. It was the criterion, and
**basin membership satisfies it.**

### C1-4b is not a C1 test at all

Re-placing the four targets at random **is not a relabelling.** It alters the property's definition
rather than renaming variables. In their three-tier scheme that is **tier (ii) — partition selection**
— not tier (i) labelling.

> **So §6's headline, *"basin membership fails C1,"* is WITHDRAWN. It is the wrong criterion applied
> to the wrong operation.**

### And the finding is *stronger* under the right framing, not weaker

Their decisive asymmetry:

> *"a labelling **cannot be empirically wrong** about the system, only differently named, whereas an
> intervention-space or grain choice **can be empirically wrong** about the system, because it
> predicts effects that interventions then fail to produce."*

Apply that to our four hand-placed targets:

- Our partition **predicts a mode** for the being.
- **Interventions fail to produce the corresponding effects** — leave-one-out over all twelve
  channels moves the winner on ≤0.25% of ticks.
- **A random in-range partition predicts as well as ours** — 20.0% agreement where chance is 25%,
  and slot 0 is entered under 94% of random charts.

By their own standard that is an **empirically undisciplined partition** — and a partition *can be
wrong*, where a labelling can only be *different*. **That is a harder criticism than the one I made,
not a softer one.**

It also breaks C1's stated conditional. Kanai is explicit that intrinsicality at tier (iii) holds
*"given an empirically disciplined choice of partition and intervention space at tier (ii)"* — and
ours is author-set. **The correct statement is not that basin membership fails C1; it is that the
condition C1 is conditional on is not satisfied.**

### C2 needs splitting the same way

I wrote that *"we ran C2 by accident."* Half right. C2's second dimension is *"the system's
counterfactual response under intervention **on those variables**."*

- **The faculty ablation** — enabling and disabling gates — **is** an intervention on the system.
  Seven of fourteen bit-identically inert **is** legitimate C2 evidence.
- **The channel leave-one-out is not.** It intervenes on *our distance function*, not on the being.
  It is evidence about the **partition**, not about the being's causal organisation.

Two different instruments, and I had reported them as one.

### What stands, restated correctly

| claim | status |
|---|---|
| classification is 100% invariant under structure-preserving relabelling | **holds — and it is C1, passed** |
| no channel changes the winner on >0.25% of ticks | holds |
| our chart agrees with a random in-range chart at chance | holds |
| slot 0 entered under 94% of random charts | holds |
| `fatigue` constant at 0 for a whole life | holds — and is untouched by any of this |
| *"basin membership fails C1"* | **withdrawn** |
| *"we ran C2 by accident"* | **half withdrawn** — the faculty ablation counts; the channel sweep does not |

**The practical conclusion is unchanged and better founded:** stop treating basin occupancy as
evidence about the being. It is a property relative to a partition we chose and cannot defend, and
`docs/how-i-would-build-it.md` §3's *"stop hand-placing basin targets"* now has the paper's own
argument behind it rather than my misreading of it.

---

## 11. The quality-space census — predictions, locked 2026-08-04 before the probe exists

§9 item 3 said `quality_space.rs` should be re-examined next, *"if the quality axes fail C1 the same
way the basins did."* That framing is now known to be wrong in the same way §10 was: **the basins
passed C1.** Permuting the twelve channels and permuting `BASIS`'s columns with them leaves every
projection identical, so the quality axes pass C1 trivially and for the same uninteresting reason.

**The live question is the one `findings.md` has carried unanswered since:**

> **Is the quality space poor, or merely unvisited?** Those have opposite fixes — a poor space is an
> architecture problem, an unvisited one is a world problem — and we have never distinguished them.

### The measure, and why it is a ratio

**Occupied volume:** distinct `QualityPoint`s the being actually visits in a life.
**Afforded volume:** distinct `QualityPoint`s reachable by projecting somatic fields the being's body
can actually produce.

**The absolute counts are chart-relative and I will not report them as findings.** What is not
chart-relative is the **ratio**, and the **change in the ratio under intervention** — which is C2's
own question, counterfactual response, and the instrument that already worked on the faculty
ablation.

**One thing I have to declare before measuring, because assuming it is exactly ledger row 5.** There
is no clean `0..256` box to sample "afforded" from. Reading **both** writers — `write_from_body` *and*
`inject`, the pair that produced row 6 — gives:

- channels 0, 2, 4, 8 clamped to `0..255`; channel 10 to `0..256`
- **channel 9 (valence) and channel 11 (FE velocity) are signed** and not clamped at the writer
- `inject()` **saturating-adds** on top of whatever `write_from_body` wrote, so any channel can leave
  its apparent range afterwards

So afforded volume is defined against a box of **measured per-channel extrema across all regimes
run**, never an assumed range, and every number below is relative to that box. Said here rather than
discovered later.

### Predictions

| # | prediction | confidence |
|---|---|---|
| **QS-1** | The default being occupies **< 5%** of afforded quality volume | high — it visits 27–68 positions in 4,000 ticks and `fatigue` is one distinct value |
| **QS-2** | `enable_reserve()` **at least doubles** occupied volume | high — it took the spatial orbit from 186 → 564, and quality is a projection of the field the body writes |
| **QS-3** | `receptors` increases occupancy **less** than `reserve` does, despite being worth 961% of drive | **low — this is the one I expect to fail.** The nociceptor is bounded and falls silent when harm ceases, which should *narrow* channel 2's range in a benign room. But drive and quality are different measures and I have been wrong before about one standing in for the other |
| **QS-4** | Occupancy under our hand-designed `BASIS` lands **within a factor of 2** of occupancy under random 4×12 bases | high, and **uncomfortable if it holds** — it would mean the axes are decoration for this purpose, the same result C1-4b produced for the basin chart |

**QS-3 is the prediction I expect to fail, and it is written that way on purpose** — the house rule
that has been worth more than the ones that held.

### What this census cannot settle, said in advance

- **It cannot say the space is rich enough to matter.** A high occupancy ratio in a four-axis space
  is still four axes. Volume is not quality.
- **It says nothing about phenomenality.** Per `docs/witness-gap-literature.md` §2.1, we make no
  phenomenal prediction, and a census does not change that.
- **A low ratio does not by itself indict the architecture.** That is the whole point of running it
  across regimes: if occupancy moves a lot when the being is given a reserve, the space was unvisited,
  not poor.

---

## 12. The census — what came out

`examples/quality_space_census.rs`, 4,000 ticks per regime in `Room::peopled(...).with_friend(...)`,
four regimes, three quantisation grains, 200,000 uniform samples for the afforded box.
**All four regimes survived 4,000 ticks**, so the ratios are comparable.

**Two of four predictions held. Both failures are worth more than the two that held.**

| | prediction | result |
|---|---|---|
| **QS-1** | default occupies **< 5%** of afforded volume | **HOLDS** — 0.049% / 0.224% / 3.667% at bins 8 / 32 / 128 |
| **QS-2** | `enable_reserve()` **at least doubles** occupied volume | **FAILS.** 0.93× / 0.92× / 1.09×. It *slightly reduces* occupancy at fine grain |
| **QS-3** | `receptors` raises occupancy **less** than `reserve` | **FAILS, as predicted.** Receptors: 4.10× / 2.92× / 1.82×, against reserve's ~1.0× |
| **QS-4** | ours within **2×** of random 4×12 bases | **HOLDS** — 1.31×, and ours (0.615%) sits *inside* the random spread (0.364–0.673%) |

### QS-2 is the expensive one, and it corrects yesterday

I predicted it at **high** confidence, from a chain I had already published: a reserve took the
spatial orbit from **186 → 564** distinct positions, so quality occupancy should follow.

**It does not.** The post-hoc channel column says why — added after seeing the result, changing no
verdict, and said plainly here:

| regime | ch10 (fatigue) | ch0 | ch5 | ch6 |
|---|---|---|---|---|
| default | 17 | 30 | 19 | 69 |
| +reserve | **35** | 30 | 19 | 64 |
| +receptors | **1** | **115** | **154** | 97 |
| +both | 12 | 95 | 140 | 99 |

**The reserve does exactly what it was built to do — it doubles the variety of `fatigue` — and
`fatigue` is one channel of twelve.** It reaches only two of the four quality axes (weight −128 on
ACTIVATION, −256 on VITALITY, **zero on COMFORT and COHERENCE**), and its range is 0–77 against
channel 0's 63–445. Doubling one narrow input to half the axes moves a four-axis projection almost
not at all.

> **The correction: behavioural variation does not imply felt variation.** Yesterday I found that
> internal variation produces behavioural variation, and today I ran that inference backwards without
> noticing. A tripled orbit and a 0.93× quality occupancy are the same being. **Where the body goes
> and what the being's state is like are different measurements, and I had been treating one as
> evidence for the other.**

### QS-3 failed in the direction it was written to fail in

`receptors` raises occupancy **3–4×** where the reserve raises it not at all, and it does so by
widening nearly every channel at once (ch0 30→115, ch5 19→154). This is the second independent
measure on which `receptors` dominates every other faculty.

**And it destroys `fatigue` completely: ch10 falls to ONE distinct value.** The bounded nociceptor
lowers threat, threat lowers metabolic cost, and energy pins. So:

> **`+both` is not strictly better than `+receptors` alone by this measure** — 99 vs 105 occupied at
> bin 32. What the reserve buys back is the *one channel receptors kills*: fatigue variety, 1 → 12.
> They are close to complementary, and that is a real trade rather than a ranking.

### QS-4 held, and it is the uncomfortable one — stated at exactly its width

Our hand-designed basis is **not distinguishable from a random 4×12 basis by occupancy.** Ours is not
even at the top of the random spread.

**What this does and does not license, because C1-4b's over-reach is still fresh:**

- It **does** say: *occupancy fraction is not evidence that our axes are the right axes.* Any four
  random projections of the field give the same answer.
- It does **not** say the basis is worthless. `similarity(a, b)` — whether felt states that *should*
  be alike come out alike — is a different measure, and `examples/quality_space_probe.rs` tests that
  one. **This census did not test it, and nothing here bears on it.**

### The fork, answered

> **The quality space is unvisited, not poor.**

Turning on a faculty that already exists, changing no structure, **quadruples occupancy**. That is
the signature of a space the being has not been in a position to explore, not of a space with nothing
in it. And the reported ratios are a **lower bound** — the afforded box over-counts by construction,
since uniform sampling includes channel combinations a real body never produces.

**The honest limit:** even the best regime reaches 0.65% at bin 32. "Unvisited" is the answer to the
fork; it is not a claim that the ceiling is high. Four axes are four axes, and **volume is not
quality**. Per `docs/witness-gap-literature.md` §2.1 none of this touches whether the being feels.
