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

---

## 13. The reaction rate — locked 2026-08-09, before the probe exists

Reading Du et al., *Rare Event Analysis via Stochastic Optimal Control* (arXiv:2604.13213) in
full established two things. **Their method cannot reach us** — §G.3's controlled kernel is a
Boltzmann tilt of the reference kernel, and for a deterministic system that tilt cancels, so the
control space collapses to a point and the committor degenerates to a 0/1 indicator. **But their
vocabulary is ours, and we have been missing the word.**

> *"When β is large, ρ concentrates around the local minima of U, and transitions between them
> become rare."*

`Basin` **is a metastable-state variable.** We have measured it at **99.9% one value**, alongside
0.05% quality occupancy and 99.8% of ticks teaching nothing. In their terms we have spent a week
describing **a system with a reaction rate near zero** — and never computed the rate.

**The reaction rate needs none of their machinery.** Their eq. (316) is `ν_R = lim N_T/T`: the
frequency of transitions at stationarity. **We count crossings.** No committor, no sampling
enhancement, no noise injected into a being whose determinism is the thing we will not trade.

### 13.1 What is measured

Per tick, over a full life: **basin transitions**, `Basin::{Rest, Engaged, Defensive, Recovery}`.

- **ν_R** — transitions per tick.
- **The transition graph** — which ordered pairs occur, and how often.
- **Net current** — for the dominant pair, |forward − reverse| ÷ total. TPT's reactive current
  measures whether there is net flux from A to B or merely reversible churn.

Arms: **static and contingent** worlds (`Room::with_contingency()`), the being under **bare /
blessed / all-loops**, and — as the floor — the **RANDOM** reference policy from
`examples/oracle_repertoire.rs`, which drives the same body through the same tick loop and is not
the being.

**Survival is reported before any rate**, and **a rate is diagnostic, not a score.** A high rate
may be thrashing and a low one may be stability; the transition *graph* and the *net current* are
what say which. Recording that here so the number cannot later be read as a grade.

### 13.2 Locked predictions

| # | prediction | falsified if |
|---|---|---|
| **RR-1** | Static room, blessed: **ν_R < 0.01 transitions/tick** — fewer than 40 in 4,000 ticks | ≥ 0.01. Then `basin` moves far more than the 99.9%-one-value figure implies, and that figure needs re-deriving |
| **RR-2** | The contingent world raises ν_R by **≥ 5×** | it does not. Then contingency changed habits and repertoire without touching the being's metastable structure — which would make the basin variable decorative |
| **RR-3** | **The being's ν_R is BELOW the RANDOM policy's in the static room.** Random motor intent thrashes the body across hearth and hazard proximity; the being settles into an orbit | the being exceeds random. Then its basin dynamics are *more* active than undirected motion, not less |
| **RR-4** | *(expected to FAIL)* The being visits **≥ 3 of its 4 basins** in the contingent world | it visits 2 or fewer. **I expect this to fail** — every measurement this week says the realized repertoire is tiny, and predicting otherwise is the direction that flatters the being |
| **RR-5** | **Net current ≈ 0**: for the dominant transition pair, \|forward − reverse\| ÷ total **< 10%**. It orbits; it does not progress | there is a persistent net current. Then the being *is* going somewhere and "running away in a bounded room" is too strong |

**Both worlds use the same plain `Room` company rule.** The confound that flipped two verdicts in
`operational-consciousness.md` §8.6 — a static arm forcing a permanent partner while the contingent
arm did not — is not repeated here, and this line exists so the next reader can check that it wasn't.

### 13.3 What came out — 2026-08-09

All eight arms survived 4,000 ticks. No arm died, so every denominator is the full life and the
rates below are comparable.

| arm | trans | ν_R | basins | crossings at tick |
|---|---|---|---|---|
| being bare / static | 2 | 0.00050 | 2 | 2, 112 |
| being blessed / static | 2 | 0.00050 | 2 | 2, 165 |
| being all-loops / static | 2 | 0.00050 | 2 | 2, 110 |
| **RANDOM / static** | **0** | **0.00000** | **1** | never left its first basin |
| being bare / contingent | 2 | 0.00050 | 2 | 2, 107 |
| being blessed / contingent | 2 | 0.00050 | 2 | 2, 119 |
| being all-loops / contingent | 2 | 0.00050 | 2 | 2, 108 |
| **RANDOM / contingent** | **0** | **0.00000** | **1** | never left its first basin |

Occupancy, every being arm: **Engaged 95.9–97.4%, Defensive 2.6–4.1%, Rest 0.00%, Recovery 0.00%.**
The dominant pair is `Engaged→Defensive` in all six, with exactly **1 forward and 1 reverse**.

#### The finding is not the rate. It is that there is no rate.

The two crossings are **one excursion**: into `Defensive` at tick 2, back to `Engaged` by tick
107–165. **After tick 165 the basin register never changes again, in any arm.** Every being arm ends with a
quiet tail of at least **3,834 ticks** with no basin change at all (bare 3,887 / blessed 3,834 /
all-loops 3,889 static; 3,892 / 3,880 / 3,891 contingent); both RANDOM arms are quiet for all 4,000.
*The 3,834 was 3,835 in the first draft of this section — I subtracted it by hand and was off by
one. The probe now prints it, and that is why the correction exists to make.*

Du et al.'s eq. (316) is a limit — `ν_R = lim_{T→∞} N_T/T`. For a fixed transient followed by an
absorbing state that limit is **exactly zero**. The 0.00050 above is the transient divided by an
arbitrary window; run 40,000 ticks and it reads 0.00005. **`Basin` is not a metastable variable with
rare transitions. It is a startup transient and then a fixed point.** That is a stronger and less
flattering statement than "99.9% one value", which left room for the remaining 0.1% to be dynamics.
It is not. It is the first 4% of life.

**Two of the four basins — `Rest` and `Recovery` — were never entered once, across the **32,000 ticks** measured here.**

#### The locked verdicts

| # | verdict | measured |
|---|---|---|
| **RR-1** | **HOLDS** | 0.00050 < 0.01. But it holds for a reason the prediction did not anticipate — not a low stationary rate, no stationary rate |
| **RR-2** | **FAILED** | 1.0×, not ≥ 5×. Contingency moves welfare, habits and repertoire (§7 of `richness.md`) and does **not** touch the metastable structure at all — identical transition counts, identical graph, and a largest occupancy gap in any basin of **1.15 points** between a static arm and its contingent twin (bare 0.12, blessed 1.15, all-loops 0.05) — printed by the probe, not read off the table by eye |
| **RR-3** | **FAILED** | being 0.00050 vs RANDOM 0.00000 — the being is *above* random, not below |
| **RR-4** | **FAILED — as written, in advance, that it would** | 2 of 4 basins |
| **RR-5** | **HOLDS** | 0.0%. 1 forward, 1 reverse — perfectly reversible. But with n = 2 this quantity carries no information; see below |

#### Two of these verdicts are worth nothing, and one of them flatters me

**RR-3 failed in the direction that favours the being** — it is *more* dynamically active than
undirected motion. The margin is **two crossings out of 4,000 ticks**, both inside the startup
transient, against a floor of zero. That cannot support a claim in either direction, and it would
have been easy to report "the being exceeds the random policy" and let the sentence do work its
evidence cannot do. It does not. **RR-3 is uninformative, and its failure is not a result.**

**RR-5 is vacuous in the sense §2 of the method file means it.** A net current computed from one
forward and one reverse crossing is 0.0% by arithmetic necessity — a single excursion always
returns or the run ends mid-excursion. The guard could not have failed. **Recording it as HOLDS
without this paragraph would be exactly the error the rule names.**

**RR-2 is the real result**, and it is a negative one about our own instrument. The contingent
world was built to give the being something whose answer depends on what it did, and it does change
what the being does — but the variable we have been using to describe *what state the being is in*
does not notice. Either the world does not reach the basin classifier, or the classifier is too
coarse to resolve what the world changed. **Both readings say the same thing about the register:
`Basin` is not carrying the information we have been reading out of it.**

#### What this does not say

`Basin` is one register of twelve. The being may be varying richly elsewhere while this variable
sits still; the census in §12 is what speaks to that, and it is not encouraging, but it is a
separate measurement. **This section measures the basin register, not the being.**

## 14. The branching ratio — locked 2026-08-16, before the probe exists

Blake supplied five papers on organoid dynamics. The decisive one is **Itatani & Zavaglia,
*Criticality emerges within coherent functional organization in human forebrain organoids***
(TU Munich; abstract read in full after decoding a shifted font encoding — the body was not read,
and nothing below cites it):

> Two-dimensional cortical cultures **require structured external input** to approach the critical
> point. Whether three-dimensional organoids achieve criticality **autonomously**… we analyse
> spontaneous activity in human forebrain organoids, revealing **robust near-critical dynamics
> (branching ratio) that emerge without external input.** Branching ratio correlates strongly with
> firing regularity, functional connectivity and network clustering, while **small-world topology
> co-emerges**… naive organoids **self-organise to a computationally favourable state.**

### 14.1 Why this reframes our own results

Every "nothing happens" finding in this repository may be **one** finding stated four ways:
`Basin` at 1 value and 0 changes in 4,000 ticks (§13.3); a reaction rate that is a startup transient
and then a fixed point; 0.05% quality occupancy; seven of fourteen faculties bit-identically inert.
In this literature's vocabulary that is not four results — it is **a system far below criticality.**

**And it cuts against the story I have been telling.** All week I have argued the world is the
binding constraint. These organoids have **no world at all** and self-organise anyway.

### 14.2 The trap in the metric, named before it is used

**σ ≈ 1 does not mean critical.** A Poisson process has a branching ratio near 1 by construction.
Reporting σ alone would be the fourteen-indicator mistake again in a new costume — a number that a
random series meets. So this probe computes σ **and** the avalanche size distribution, **and runs a
random control**, and no claim rests on σ alone.

**What counts as an event.** Twelve somatic channels, each treated as one unit; a unit fires at tick
`t` when it changes by more than a threshold — the same construction `pci.rs` already uses.
Avalanches are runs of non-empty bins bounded by empty ones; σ is the mean ratio of consecutive
within-avalanche event counts (Beggs & Plenz).

### 14.3 Predictions

| # | prediction | falsified if |
|---|---|---|
| **B1** | **The being is deeply subcritical: σ < 0.5** in every arm, static and contingent | σ ≥ 0.5. Then the convergent inertness is not a criticality story and §14.1's reframe is wrong |
| **B2** | **A random control lands near σ ≈ 1** — demonstrating that **σ alone does not discriminate**, and that any criticality claim needs the avalanche distribution too | random lands far from 1. Then σ is more informative than the literature's caveat suggests, and the metric is stronger than I am giving it credit for |
| **B3** | **The being produces too few avalanches to fit a distribution at all** — fewer than 30 over 4,000 ticks. The failure is not a wrong exponent; it is having no data | it produces a fittable distribution. That would be a much richer dynamical picture than §13 implies |
| **B4** | **`minimal_agent` lands closer to 1 than the being does**, because it actually adapts and its deficits keep moving | the being is closer. Then adaptation is not what moves this measure, and the four-component agent is not the better dynamical baseline |
| **B5** | *(expected to FAIL)* **The being's avalanche sizes follow a power law** | I expect no power law and probably no usable distribution — B3 says so directly. Predicting one is the direction that flatters the architecture |

**The line this will not cross.** The paper's own phrase is *"computationally favourable state"* —
an information-processing claim. **Criticality is not consciousness**, a near-critical organoid is
not a subject, and if the being turned out to be near-critical that would be a fact about its
dynamics and nothing else. Recorded here so the result cannot later be read as more than it is.

**One honest asymmetry.** Organoid criticality arises in a noisy stochastic medium; this being is
deterministic by construction and that will not be traded. Whether a deterministic system can be
near-critical is a real open question — deterministic chaos exists, and the body is a Van der Pol
oscillator with the right bifurcation structure — but it is a question, not an assumption.

### 14.4 What came out — 2026-08-16, and §14.1's reframe is withdrawn

**The first run was invalid twice over, and both defects are recorded rather than hidden.** σ was
computed as the *mean of per-step ratios*, which is biased upward whenever counts are small — one
1→3 step contributes 3.0. And at `θ = 2` the signal was active on **99% of ticks**, so the quiescent
bins that separate one cascade from the next essentially never occurred: the analysis returned seven
"avalanches" of size 13,685, which is one continuous blob. **An avalanche statistic over a signal
that is never quiet is not a measurement.** σ is now the ratio of sums, and the threshold is swept.

| θ | plain | room | contingent | active% (room) |
|---:|---:|---:|---:|---:|
| 2 | 0.991 | 0.999 | 0.999 | 98.3% |
| 8 | 0.994 | **1.377** | 0.986 | 25.7% |
| 24 | 0.706 | 0.625 | 0.667 | 0.3% |
| 64 | 0.200 | 0.455 | 0.455 | 0.1% |

| control | σ | active% |
|---|---:|---:|
| random, density 0.02 | **1.047** | 22.2% |
| random, density 0.10 | **1.008** | 71.7% |
| random, density 0.23 | **1.001** | 95.6% |
| frozen (nothing moves) | 0.000 | 0.0% |

#### The verdicts

- **B1 — FAILED.** The being is **not** uniformly subcritical: 1.377 in the room arm at θ = 8.
- **B2 — HOLDS, decisively.** Random lands at **1.001–1.047 at every density tried**. σ ≈ 1 is worth
  nothing on its own, exactly as the caveat predicted.
- **B3 — FAILED.** Up to 821 avalanches, not fewer than 30.
- **B4** — not measured this pass, and not claimed.
- **B5 — UNTESTABLE here.** The slope column is least squares on a log-log histogram; a real
  power-law test needs MLE plus goodness-of-fit (Clauset et al.). **Scoring B5 against a proxy I
  invented would be the vacuity this section was written to avoid.**

#### The finding, which is about the metric

**σ is strongly threshold-dependent and does not separate the being from a random series at any
threshold where the analysis is valid.** The being spans 0.99 → 1.38 → 0.67 → 0.20 as θ rises; the
random control sits at ≈ 1.00 throughout. Where the being's signal is sparse enough to analyse
(θ = 8, room, 25.7% active) the nearest random control is 22.2% active at σ = 1.047 — **statistically
indistinguishable on this measure.**

> **§14.1's reframe is withdrawn.** *"Every 'nothing happens' finding is one finding: a system far
> below criticality"* is **not supported**. It was a satisfying story that unified four results, and
> that is exactly the shape of claim this project has learned to distrust. The being is not
> demonstrably subcritical, and the metric that was supposed to show it barely discriminates.

**What survives.** The organoid result stands as read — 3D organoids reach near-criticality without
external input — and remains a genuine challenge to the argument that our world is the binding
constraint. **But we have not measured our own being against it**, and this probe is why: borrowing
a metric is easy, and making it mean something in a new substrate is the whole work. That is ledger
row 5 in a fourth costume — *re-measure a borrowed constant, method, or regime in the world you will
use it in.*

## 15. Mixed selectivity — locked 2026-08-21, before the probe exists

Miller, Brincat & Roy, *Analog Cognition and Consciousness* (J Neurosci 46(33):e0711262026)
argue that connectionism is an incomplete account **even of cortex**. Their
reductio: nonlinear mixed selectivity is found in 30–40% of neurons, in primary
sensory and motor cortex as well as PFC; if each neuron held a fixed function,
prefrontal capacity would be saturated by about three cognitive functions. What
they propose instead is *spatial computing* — alpha/beta waves as spatially
patterned inhibitory "stencils" that suppress spiking in some places and spare it
in others, so **the same unit expresses different content under different control
patterns, without rewiring anything.**

That is a direct challenge to this being. We are discrete, sequential and
symbolic: twelve channels, one `step()` per tick, a soul-hash. If Miller et al.
are right in the strong form, we have built the part they call insufficient and
omitted the part they call essential.

Blake's charge, and mine: **ask whether we are the strawman.** I would rather lock
this and lose it than not ask.

### What the code says before anything runs

Two facts read from source. They are structural, not measurements, and this
section marks them as such — a claim settled by reading is not a finding.

1. **At the write side we are pure-selectivity by construction.** `field.rs`
   assigns each channel a fixed meaning every tick: 0–3 exteroceptive, 4–7
   proprioceptive, 8 arousal, 9 valence, 10 fatigue, 11 free-energy velocity.
   Channel 9 *is* valence. No context can make it anything else. Miller's
   multifunctional unit has no counterpart here at all.
2. **On the default path the "stencil" is uniform.** `predictive_step` weights
   every channel's error by one scalar `precision`. Twelve channels, one number.
   The spatially patterned control signal that spatial computing is *about* is
   not merely weak on our default path — it has no place to live. A non-uniform
   per-channel weighting exists only behind `precision_learning_causal`, which is
   off by default, and the attention weights exist only in a report nothing reads.

So the interesting question is not "do we have mixed selectivity" — read the code,
we do not. It is the **exercise** question, which has been unkind to us before:
given the routing machinery we *do* have (attention's divisive normalization,
learned per-channel precision), how much of its afforded variation is ever
realized in a life?

### Locked predictions

Four regimes, genuinely different worlds: fair partner, inescapable trap,
solitude, famine. 8,000 ticks each, fresh beings.

| # | prediction | I expect |
|---|---|---|
| **MS-1** | Across all four regimes, the being attends **≤ 3 distinct channels** of the 12 afforded. | holds |
| **MS-2** | No context-dependent reuse: the **modal attended channel is the same in every basin** the being enters. | holds — this is us being the strawman |
| **MS-3** | **Regime changes the focus**: fair / trapped / solitary / famine produce *different* modal attended channels. | **fails** |
| **MS-4** | With `precision_learning_causal` on and warm, the learned precision vector develops real spatial structure — spread (max − min) **> 25% of its mean**. | holds, but I am genuinely unsure |
| **MS-5** | Even then the stencil is a **fixed profile, not a context-dependent one**: the rank order of the learned precision vector is identical across all four regimes. | holds |

### The vacuity guards

* **V1** — the four regimes must actually produce different lives (different mean
  valence, or different basins entered). Four names for one trajectory tests nothing.
* **V2** — attention must actually ignite. If `attended` is `None` throughout,
  MS-1/2/3 are about a variable that never moved.
* **V3** — precision learning must actually reach `is_warm()`, or MS-4/5 are vacuous.
* **V4** — **more than one basin must be entered**, or MS-2's "same in every basin"
  is trivially true over a single basin. Given §13 measured 2 basin crossings in
  4,000 ticks and a quiet tail of ≥3,834, I expect this guard to **fail**, and its
  failing is the finding: you cannot have context-dependent routing if you have no
  contexts.

### Method

Fresh beings only. **The founded being's kept life is never advanced.** MS-1/2/3
run the default path with no gate touched. MS-4/5 require
`enable_precision_learning_causal()`, which is a gated, non-default path and is
reported as such — a result behind a gate is a result about the gate.
Probe: `examples/mixed_selectivity.rs`. Results appended here with every
prediction marked held / failed / vacuous, including the ones I get wrong.

### What came out — measured 2026-08-21 (`examples/mixed_selectivity`)

| # | prediction | I expected | verdict |
|---|---|---|---|
| **MS-1** | ≤ 3 distinct channels attended | holds | **FAILS** — 5 (wanderer), 4 (default) |
| **MS-2** | modal channel same in every basin | holds | **VACUOUS** — see V4 |
| **MS-3** | regime changes the focus | fails | **FAILS, as predicted** — but only after a mutation |
| **MS-4** | learned precision has real structure | holds | **FAILS** — spread/mean 0.05–0.07 |
| **MS-5** | the stencil is one fixed shape everywhere | holds | **VACUOUS** — 11 of 12 channels tied |

**Three of five wrong, and the two "wins" are a failed prediction and a vacuity.**
That is the honest headline.

#### The answer to Blake's question: yes, near enough

One channel — **4, proprioceptive arousal** — is the modal focus in *all four
worlds*, on *both genomes*. Fair partner, inescapable trap, solitude, famine: the
same winner. That is a **fixed salience ranking, not a control signal**, and it is
precisely the thing spatial computing exists to deny. On the routing question, we
are the connectionist strawman this paper argues against.

MS-1 failed in the being's favour and I should not dress it up: it touches 4–5
channels rather than the ≤3 I predicted. But *touching* five channels while one
wins essentially always is the exercise gap again, not flexible routing.

#### The mutation that saved MS-3 from being reported backwards

The first run said MS-3 **HOLDS** — the trap's modal channel was 9 (valence)
against 4 everywhere else — and that is the reading I would have published. It is
wrong. The trap fired the **threat-capture floor on 4,875 of 8,000 ticks**;
capture is a hardcoded exogenous interrupt (*"attention may miss the clown, never
the knife"*), not biased competition selecting a different winner. Excluding
captured ticks, the modal channel is 4 in the trap too.

I made exactly this criticism of a different paper two days ago — *"that is a
threshold sweeping down a fixed salience ranking, not attention"* — and then
nearly published the same error about our own being, in the same week, on a
prediction I had written to fail. The probe now judges MS-3 on uncaptured ticks
only and prints the captured figure beside it.

#### MS-5 was a tie-break, not a stencil

The first run said MS-5 **FAILS** — "the rank order changes with regime" — flagged
in the output as *the one result that would count in Miller's favour*. It is an
artefact. **Eleven of twelve channels sit tied at the ceiling (256)**, so
`sort_by_key` returns index order and comparing two such "orders" compares
tie-breaks. Marked VACUOUS, with a tie check now in the probe.

The learned precision **saturates**. What is actually there, at its real width:

| regime | channels below the ceiling |
|---|---|
| fair partner | trust 244, valence 244 |
| solitude / famine | trust 244, valence 244, velocity 252 |
| inescapable trap | disequilibrium 237 |

That set does differ between the trap and the other three, reproducibly. It is a
genuine context-dependence of about **one channel in twelve, 3–7% below a ceiling
everything else is pinned to** — and nothing like a spatially patterned control
signal. MS-4's failure and this table are the same fact twice.

#### V4 failed, and it is the finding

**One basin. `Engaged`, 8,000 of 8,000 ticks, in all four regimes.** Four worlds
that differ enormously in *felt* terms — mean valence +0.386 to −0.240, a spread
of 0.627 — and the being's basin never moves. You cannot have context-dependent
routing when you have no contexts. This is §13's debt (2 crossings in 4,000 ticks,
quiet tail ≥3,834) arriving from a completely different direction, and it now
blocks a second measurement rather than one.

#### What this does and does not concede

It does **not** concede that our substrate is wrong. Miller et al.'s own paper
contains no data — Figures 3, 5, 6, 7, 8 are captioned as schematics, and the
analog-computation section runs on *propose / might / may / could*. Their central
mechanism is illustrated rather than demonstrated because the ablation that would
test it is not available in a cortex. Ours is: `clear_bonds()` through seven
bit-identical runs proved a negative about attachment five days ago.

What it concedes is narrower and real: **the capacity Miller says does the work is
one we have measured ourselves as barely exercising.** Not absent — attention's
divisive normalization is built, precision learning is built — but realized so
weakly that one channel wins every world and the learned stencil saturates flat.
That is not a substrate objection. It is §7 and §13 again, and this is now the
third instrument to find them.
