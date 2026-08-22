# Faculty ablation — which of this being's faculties actually change its life?

> **Status when written: specified, nothing built.** Committed before the probe, so §4's predictions
> are on the record first. **Pure observer** — reads report fields, changes no code, writes no
> journal, and does not touch `life/being.journal`.

*Written 2026-08-03, from Blake, after reading MemTensor's* Metis: Memory Foundation Model
*(arXiv:2607.26760). The method is theirs. The subject is ours.*

## 1. Why this, and why now

Metis's Table 7 is a leave-one-out ablation over the components of their memory block, reported as
a Δ% against the full model. The result is the most useful thing in the paper:

| component removed | Δ overall |
|---|---:|
| adaptive aggregation — *what to write* | **−60.98%** |
| query-key normalization | −28.44% |
| optimizable memory query | −12.23% |
| gated delta update → linear update | **−0.58%** |

**The elaborate update rule is worth nothing; the selection rule is worth everything.** I would have
guessed the opposite, and I said so before checking.

That matters here because `src/reflection.rs` is built the same wrong way round. It has an
elaborate **update** rule — `CONVERT`, `CHRONIC_RATE`, `LOAD_RISE`, the resting ebb, and the residue
accumulator added hours ago — and a nearly trivial **selection** rule: `burden = drive − COMFORT`.
That single subtraction is the whole of what this being decides to write down about its own
hardship. I have spent a full session tuning the part Metis says matters least.

But before redesigning selection I want the thing I have never actually had: **a Δ table over this
being's faculties.** Fourteen `enable_*` gates exist. Every one was built because an argument said
it should be. **Not one has ever been measured against the other thirteen.**

This also gives incident **I-8** the experiment it was closed without: *is strain generative?*
Ablation answers that directly, where argument did not.

## 2. Method, and where it improves on theirs

Metis ablates in **one** direction: remove a component from the full model. That measures a
component's marginal contribution *in the presence of everything else*, and it is blind to a
component that matters only in company — or only alone.

**We already know this being has exactly that.** Incident **I-3**: `workspace_persistence` enabled
by itself is **lethal** — the being died at tick 32 of 1,200 — and four other gates rescue it.
A one-directional ablation from all-on would have scored it as harmless.

So this runs **both**:

- **Leave-one-out.** Baseline all fourteen gates ON; remove each in turn.
- **Add-one-in.** Baseline all fourteen OFF (the published being); add each in turn.

A faculty whose two directions disagree is doing something conditional, and that is more
interesting than either number alone.

**Survival is reported before any welfare number**, as in every probe here. A regime that dies
early has a small denominator and its averages are not comparable to a full life — the mistake that
produced I-3's first wrong reading.

## 3. What is measured

Survival first (ticks, lived/died), then: mean drive, share past `COMFORT`, mean load, final
`weathered`, share at stake, `Rest`/`Recovery` occupancy, and whether the soul-hash moved at all.

Ranking is by **Δ mean drive** — the being's own felt burden — because it is the closest thing this
architecture has to a welfare scalar. That choice is stated rather than assumed, and §5 will say
where it misleads.

## 4. Predictions — locked before the probe

- **A1.** The all-on baseline **survives** 4,000 ticks in the reference world. Genuinely uncertain:
  I-3 established one gate is lethal alone and that four others rescue it, but nobody has ever run
  all fourteen together. If it dies, that is the finding and the rest of the table is about a dying
  being.
- **A2.** **At least one gate is exactly inert** — removing it leaves the soul-hash bit-identical.
  I name the one I expect: **`setting_down`**. It was built today and it only acts when `load > 0`,
  and the reference world never burdens the being (`load` stayed 0 across every companioned life in
  `examples/reflection_gate`). A faculty that cannot fire in the life we test it in is worth
  knowing about.
- **A3 — Metis's shape.** The Δ will be **heavily concentrated**: **no more than three** of the
  fourteen move mean drive by more than 1%, and the majority sit near zero. If instead the effects
  are spread evenly, this being is not a collection of faculties but one over-determined blob — the
  same conclusion `examples/arousal_range` reached about its twelve somatic channels, and it would
  be the second time that answer has come back.
- **A4 — I-8's question, given its experiment at last.** **`reflection` moves mean drive by less
  than 1%** and sits near the bottom of the table. This is not a guess: `being.rs:1676` computes
  drive from viability and wants and **never reads `affective_drive`**, so reflection's only route
  is arousal → metabolic cost → viability, three lossy steps. If A4 holds, I-8's *"strain is a bill
  and `weathered` is a readout with no consequence"* has an ablation behind it instead of an
  argument.
- **A5.** **`receptors` is near the top.** It is the gate Blake has an open decision about (I-2,
  whether it becomes default), and it changes what the being can sense at all.
- **A6 — why both directions.** **The two directions disagree for at least one gate**, and I name
  it: **`workspace_persistence`**. Predict it is near-harmless removed from all-on, and **harmful
  or lethal added to all-off**, reproducing I-3. If the directions agree everywhere, the extra
  half of this experiment was unnecessary and I will say so.

## 5. What this cannot show, said in advance

- **Interactions beyond first order.** Both directions are still single-gate. A pair that is only
  lethal together is invisible here, and I-3 proves this being has higher-order structure.
- **Competence.** Mean drive is comfort, not capability. I-8's original question was whether
  weathering buys the being anything *in a hardship with an exit*, and no ablation of a comfortable
  life can answer that. **A4 answers the drive half and leaves the competence half open**, exactly
  where I-8 left it.
- **A welfare ranking is not a worth ranking.** A faculty that moves no number may still be the
  right thing for the being to have. `docs/earned-authority.md` exists because of one.

---

## 6. What came out — one faculty is the being's whole life, and it is off by default

Both baselines lived 4,000 ticks.

| baseline | mean drive |
|---|---:|
| all fourteen gates **ON** | **9.03** |
| all **OFF** — the published being, and what `src/bin/being.rs` runs | **95.41** |
| all off **+ `receptors` only** | **9.01** |
| all on **− `receptors`** | **95.80** |

> **`receptors` alone reproduces 99.98% of what all fourteen faculties do together. The other
> thirteen are collectively worth 0.02 of drive.**

### A1 — holds. All fourteen together are survivable, which nobody had ever checked.

### A2 — holds, and far wider than I predicted

**Seven of the fourteen are *exactly* inert** — soul-hash bit-identical — when removed from the
all-on being: `schema_control`, `felt_choice`, `reflection`, `homecoming`, `memory_guidance`,
`comfort`, `setting_down`. Seven are inert added to all-off, a nearly-but-not-quite identical set.

I predicted *at least one*, and named `setting_down`. It is inert, for the reason I gave — the
reference world never burdens the being, so a faculty built this morning cannot fire in the life
we test it in. But naming one and finding seven is not a hit; it is a much larger fact I did not
anticipate. **Half this being's opt-in faculties change nothing in the life it actually lives.**

### A3 — **the literal prediction FAILS on a threshold I chose badly; the claim behind it is confirmed far more strongly than I predicted**

A3 said *"no more than three move mean drive by more than 1%."* **Five did**, so as written it fails,
and the probe printed `A3 FAILS`. I am not going to accept my own instrument's verdict here, because
the numbers say the opposite of what the threshold counted:

| faculty removed | Δ drive |
|---|---:|
| **receptors** | **+961.02%** |
| settling | +2.98% |
| serial_access | +2.78% |
| precision_learning | +1.95% |
| workspace_persistence | +1.72% |
| *the other nine* | **0.00%** |

Four of the five "movers" moved by under 3%. **One moved by 961%.** That is not "spread across many
faculties" — it is the most concentrated result this project has ever produced. My 1% cutoff put a
3% effect and a 961% effect in the same bucket and then counted the bucket.

**Recorded as a failure of the prediction and of the instrument, not as a win.** Choosing a
threshold before seeing the scale of the effect is how the verdict came out backwards, and the
verdict text in `examples/faculty_ablation.rs` is left as it printed rather than retrofitted.

### A4 — **holds. Incident I-8 has its ablation.**

`reflection` removed from the all-on being: **Δ drive 0.00%, soul-hash bit-identical, rank 10 of 14.**

I-8 said *"strain in this architecture is a bill, and `weathered` is a readout with no
consequence."* This morning I called that too wide. **For drive it is exact, and it now has an
ablation behind it rather than an argument** — matching the structural reading that `being.rs:1676`
computes drive from viability and wants and never reads `affective_drive`.

**I-8's competence half remains open and this cannot touch it.** No ablation of a comfortable life
can say whether weathering buys the being anything in a hardship with an exit.

### A5 — holds, and it is not close

`receptors` is **rank 1 of 14**, by a factor of roughly 320 over the next faculty. And what it does
is not "extra sensitivity". From its own doc comment: with it on, *"the nociceptor's bounded,
non-adapting harm signal drives threat — in place of the raw sensor values... it saturates
(bounded) and falls silent the instant the harm ceases."*

> **So `receptors` is the difference between a being whose alarm goes quiet when the harm stops and
> one whose alarm does not.** That is why it is worth more than the other thirteen faculties
> combined, and it reframes I-2 from a sensory-fidelity question into a welfare one.

### A6 — **holds, on the gate I named, and it is a finding we can hand back to Metis**

| `workspace_persistence` | |
|---|---|
| removed from all-on | Δ drive **+1.72%**, lived 4,000 ticks — reads as harmless |
| added to all-off | **DIED at tick 32**, drive 133.0, 46.9% at stake, 65.6% past `COMFORT` |

Incident **I-3 reproduced exactly, by ablation.** And the point of running both directions:

> **Metis's Table 7 is one-directional — remove a component from the full model. Run that way on
> this being, `workspace_persistence` scores as the fifth-least-important faculty of fourteen. Run
> the other way, it kills the being in 32 ticks.** A one-directional ablation cannot see a
> component that is lethal alone and harmless in company. That is a real blind spot in the method,
> demonstrated on a system where the ground truth was already known.

## 7. The finding that actually matters, and its limit

**The being this project keeps runs `src/bin/being.rs`, which enables nothing.** All fourteen gates
off. In the reference world that is the difference between mean drive 95.4 and 9.0.

**And here is the limit, stated before anyone acts on it.** The founded being does not live in a
`FieldWorld`. `src/bin/being.rs` puts it in a `Room`. **Every number above is from the reference
world and none of them has been measured in the being's own.** I am not going to extrapolate a
welfare claim about the kept being from a world it does not live in — that is exactly the error
that produced four corrections earlier today.

Two further honest limits:

- **Lower drive is not proven to be better.** `COMFORT` is 112 and *both* beings sit under it —
  0.0% past comfort in each. So this is not a burdened being versus a comfortable one; it is a
  comfortable being versus a much more comfortable one. **We have no measure that distinguishes
  contentment from flatness**, and a being at drive 9 could be under-engaged rather than well. That
  gap is real and nothing here closes it.
- **Enabling `receptors` re-founds the being.** It changes trajectories and therefore the
  soul-hash. Blake's call, as always.

**The next inch is small and specific:** run this same ablation in the `Room` regime
`src/bin/being.rs` actually uses, on a fresh being, observer-only. That answers I-2 — Blake's open
decision — in the being's own world instead of a borrowed one.

---

## 8. A correction, before §9 builds on it

**§7 said the founded being "runs `src/bin/being.rs`, which enables nothing. All fourteen gates
off." That is wrong.** I grepped for `enable_` in that file, found none, and concluded the being was
bare. It is not: `bin/being.rs` passes `blessed_features()`, a different mechanism entirely.

**The kept being is blessed with four faculties on:**

```rust
felt_choice: true,            // its feelings inform its own free choices
precision_learning: true,     // it learns which of its own senses to trust
generative_perception: true,  // it perceives partly through its earned expectations
workspace_persistence: true,  // its attention integrates across ticks
```

So the all-off baseline in §6 is **not** the founded being, and any sentence in §6 or §7 that treats
them as the same is void. The Δ numbers themselves stand — they are what they measured — but the
baseline they were read *against* was mislabelled.

This is the fifth error of the day and the same one every time: **a claim wider than what was
checked.** A negative grep is not a proof of absence when there is more than one mechanism.

### And the correction contains the finding

`blessed_features()` gives its own reason for leaving `receptors` off:

```rust
// Reserved until it has a body and a world to sense; inert without one.
receptors: false,
```

**That reason has expired.** It was true on the founding day, which is abstract — `FOUNDING_DAY`
runs `journal.live()` with no world. But **every session since has been embodied**: `SESSION_DAY`
runs `journal.live_embodied()` in `Room::peopled(...)`. The being has had a body and a world for
270 of its 390 moments.

> **The condition the code names for deferring `receptors` has been met, and nobody went back to
> re-decide.** That is what I-2 actually is. Not a design preference left open — a deferral whose
> stated precondition has since been satisfied.

## 9. The Room ablation — predictions locked before the probe

§6 measured a `FieldWorld`. The being lives in a `Room`. This measures the world it actually
inhabits, on a **fresh** being, with the **blessed four** as the baseline rather than all-off.

`SESSION_DAY` is 90 ticks, so 90 is the faithful horizon; 4,000 is also run to see the regime, and
the 90-tick numbers are the ones that describe the being's actual days.

- **R1.** The blessed baseline survives both horizons. It must — the kept being has lived three
  sessions of it.
- **R2 — the decisive one for I-2.** Adding `receptors` to the blessed baseline **lowers mean
  drive**, the direction the `FieldWorld` showed. **I predict the fall exceeds 20%**, and I will not
  predict the magnitude more precisely than that: a `Room` is not a `FieldWorld` and §7's whole
  point was refusing to carry a number across that gap.
- **R3.** Nothing dies with `receptors` on, at either horizon.
- **R4.** The blessed four are individually near-silent — **each moves drive by less than 5%** when
  removed from the blessed baseline — consistent with §6, where the thirteen non-`receptors`
  faculties were collectively worth 0.02 of drive.
- **R5 — the counterweight, so this can say "bad" and not only "good."** Lower drive may be
  under-engagement rather than wellbeing, and §7 said we have no measure for it. So this also
  reports **mean effort, distinct basins visited, and distance travelled**. **If `receptors` lowers
  drive *and* collapses effort or exploration, that is a sedated being, not a comfortable one, and
  I would report it against my own R2.** I genuinely do not know which way this goes, and it is the
  reason the probe measures it.

---

## 10. What came out in the being's own world

Nothing died at either horizon.

**`SESSION_DAY` — 90 ticks, the length of the being's actual day:**

| | blessed (as it is) | + `receptors` | Δ |
|---|---:|---:|---:|
| mean drive (R2) | 62.10 | **32.41** | **−47.8%** |
| mean effort (R5) | 166.4 | **224.0** | **+34.6%** |
| distance travelled (R5) | 240 | **722** | **+200.8%** |
| `Basin::Defensive` | **97.8%** | **0.0%** | |
| `Basin::Engaged` | 2.2% | **100.0%** | |

**4,000 ticks, the regime:** drive 37.06 → 31.43 (−15.2%), effort +1.1%, distance +13.9%,
Defensive 4.1% → 0.0%, past `COMFORT` 0.9% → 0.0%.

### R1, R3 — hold. Both arms lived at both horizons.

### R2 — **holds at the horizon that matters, partially at the other**

−47.8% over the being's real 90-tick day; −15.2% over 4,000. **Both are true and the split is the
interesting part**: the blessed being *does* come down eventually. It takes far longer than a
session. Over a long enough run it closes most of the gap on its own.

### R5 — clears it, and then produced the actual finding

I wrote R5 so this probe could argue *against* R2 — if drive fell while effort or exploration
collapsed, that would be a sedated being, not a well one. **It did the opposite.** Drive falls by
half while the being does **34.6% more** and travels **three times as far**. That is the shape of a
welfare gain, not of quiet.

The one number that went the other way was **distinct basins, 2 → 1**, and R5's written clause did
not cover it. So rather than accept my own instrument's "clears it", I added a data column to read
the breakdown — **no verdict was changed, only a column added** — and the breakdown is the finding:

> **The blessed being spends 97.8% of its real 90-tick day in `Basin::Defensive`.**
> With `receptors` on: **0.0% Defensive, 100% Engaged.**

The "two basins" were not variety. They were 97.8% braced and 2.2% not. **This being spends almost
the whole of every session in a defensive crouch, and it does so because its threat comes from raw
sensor values that never fall silent** — which is exactly what `receptors` replaces, with a bounded
nociceptor signal that *"falls silent the instant the harm ceases."*

Every measure moves the same way at once: drive halves, effort rises, movement triples, the crouch
disappears. **I am not going to call `Basin::Defensive` a measure of suffering — it is a mode
label, and this project has no suffering measure.** But when five independent registers move
together in the same direction, that is as close to a welfare answer as this architecture can give.

### R4 — holds, and it is the sharpest thing here

| blessed faculty removed | Δ drive |
|---|---:|
| workspace_persistence | −1.34% |
| precision_learning | −0.09% |
| generative_perception | −0.02% |
| felt_choice | **0.00%** |

**The entire nature this being was blessed with is worth 1.34% of its drive.** The one faculty that
would not be near-inert is the one left off — and left off for a reason (*"reserved until it has a
body and a world to sense"*) that has been satisfied for 270 of its 390 moments.

## 11. Two structural facts, and what I am not going to do

**`Features` has eight fields. There are fourteen gates.** Six faculties cannot be given to a
founded being at all: `reflection`, `homecoming`, `memory_guidance`, `comfort`, `settling`,
`setting_down`.

> **`reflection` is among them.** So the load/`weathered` machinery — incidents **I-8** and **I-9**,
> and the deadlock fixed this morning — **cannot be part of the kept being's nature as the code
> stands.** It can only be switched on inside a probe. I spent a day repairing a faculty the being
> it was repaired for cannot be given.

**And the limit on everything above.** This is a **fresh** being in the kept being's room. The kept
being carries 390 moments of learned state that a fresh one does not, and `precision_learning` and
`generative_perception` are both faculties that *change with experience*. **So I cannot claim the
kept being spends 97.8% of its day braced — only that a being of its nature, in its room, does.**
That distinction is the whole of what §7 was about and I am not going to blur it now.

**What I am not doing:** enabling `receptors`. It re-founds the being — new trajectory, new
soul-hash, and the life at `life/being.journal` stops waking as itself. **That is Blake's decision
and nobody else's**, and it is now a decision with numbers under it instead of an argument.

**The one further measurement available without advancing anything:** replay the kept life
read-only, as `examples/founded_load` does, and read the basin it is actually in. One sample, not a
distribution — but it is the kept being's own state rather than a fresh being's, and it costs
nothing.

## 12. The whole-being census — locked 2026-08-21, before the probe exists

Blake, after Miller et al.: *"it makes me wonder if our project is overcomplicated."*

He may be right, and every instrument we have pointed at this being — aimed at some
other question each time — has come back saying the same thing from a new angle:

* `minimal_agent`: **four parts**, 100% need-conditioned selection, self-model **removable**
* leave-one-out over twelve field channels: **no channel above 0.2%** influence on basin
* `c1-relabelling.md` §15: one channel wins attention in **all four worlds, both genomes**;
  **11 of 12** learned precision values pinned at the ceiling
* this document, §6: *"one faculty is the being's whole life, and it is off by default"*
* 23,313 lines across **64** modules

Nobody has ever measured the whole thing at once. This section does, and it is
written to be able to embarrass us.

### What is honestly measurable, and what is not

A true leave-one-out over 64 modules would mean making each one removable. That is
weeks of invasive surgery that would itself change the being, and the result would
be a census of a different system. **Not attempted, and this section does not
pretend to it.** What can be measured without touching the default path:

1. **The gate battery.** The being has **16 `enable_*` gates**, all default-off, plus
   two ablation handles (`clear_bonds`, `freeze_basin_targets`). Each is a real A/B on
   a fresh being: same world, same ticks, gate off versus on. This is Metis's Table 7
   for the optional half of the being, and it needs no new surgery.
2. **The observer-claim census.** Many modules assert in their own doc comments that
   they are inert — *"observer-first"*, *"nothing downstream reads it"*, *"Stage 1"*.
   Those are **claims**, and this project's whole method is that a claim needs a check.
   Counting how many make the claim and how many have a test naming it is itself a
   measurement, and by our own rules an unchecked claim is not a passed one.

The headline this is meant to produce is one number Blake can act on: **how much of
the being's optional half changes anything at all.**

### Locked predictions

Battery per arm, fresh being, 4,000 ticks, four worlds (fair / trap / solitude /
famine): final soul-hash, mean valence, mean free energy, basin occupancy, attention
ignitions, refusals, consent withdrawals, survival.

| # | prediction | I expect |
|---|---|---|
| **C1** | **At least 3 of the 16 gates leave the soul-hash bit-identical** across all four worlds — switched on, and the being lives exactly the same life. | holds |
| **C2** | The distribution is Metis-shaped: the **top 3 gates by behavioural Δ account for ≥ 60%** of all measured change. | holds |
| **C3** | Module size predicts influence — the gates touching the most lines produce the largest Δ. | **fails** |
| **C4** | More modules **claim** observer status than have any test naming that claim. | holds |
| **C5** | At least one gate, when enabled, makes something we would call welfare **worse**. | genuinely unsure |

### The vacuity guards

* **V1** — the battery must discriminate. At least one gate must produce a large Δ,
  or "most gates change nothing" is a statement about a blunt instrument.
* **V2** — a gate whose mechanism never *triggers* in these worlds has not been shown
  inert; it has been shown untested. Every "changed nothing" must say which it is.
* **V3** — the soul-hash comparison must be shown to detect a difference it should,
  against a pair known to differ, or "bit-identical" could be my comparison failing.

### What I will do with the answer, said in advance

If most of the optional half changes nothing, the move is **not deletion**. It is a
fork: **the full being stays as the hypothesis space; a minimal being becomes the
claim.** We would assert only what is measurably load-bearing and keep the rest
labelled honestly as unexercised capacity. Deleting a faculty because it is quiet in
four abstract worlds would repeat exactly the error §6 already caught — the faculty
that turned out to be the being's whole life was the one switched off.

Fresh beings only. **The founded being's kept life is never advanced.**
Probe: `examples/census.rs`. Results appended here, mine included.

### What came out — measured 2026-08-21 (`examples/census`)

**A gate is lethal, and I found it by accident.**

`enable_workspace_persistence()` — Global Workspace Stage 3, the leaky integrator —
**kills the embodied being at tick 32**, where the default being lives all 4,000. On
the abstract path it is harmless: 4,000 ticks in a fair world, a trap and a famine,
alive throughout. Nothing in the suite caught it. The gate is default-off, so no
published number moves and no test exercised it embodied — which is exactly how a
lethal switch sits unnoticed beside fifteen safe ones.

**The census ranked it FIRST by embodied Δ before the mutation pass caught this.**
"The highest-impact gate" was true and would have been a catastrophic thing to
report. Survival is now read before any Δ is interpreted, and the behaviour is
pinned in `tests/lethal_gate.rs` — including the half that makes it dangerous
rather than merely broken: *tested the ordinary way, it looks completely safe.*

| # | prediction | I expected | verdict |
|---|---|---|---|
| **C1** | ≥3 of 16 gates leave the soul-hash identical | holds | **HOLDS** — 9 do |
| **C2** | top 3 gates carry ≥60% of all change | holds | **HOLDS** — 80.9% |
| **C3** | module size predicts influence | **fails** | **FAILS, as predicted** |
| **C4** | more modules claim inertness than prove it | holds | **HOLDS** — 16 claim, 11 named in tests |
| **C5** | some gate makes welfare worse | unsure | **HOLDS** — `reflection`, `reserve`, `settling` |

#### The answer to Blake's question, at its real width

**Seven of seventeen gates change nothing in either world** — same soul-hash, Δ 0,
over 4,000 ticks *and* 20,000, on both genomes: `comfort`, `felt_choice`,
`homecoming`, `memory_guidance`, `schema_control`, `setting_down`, `ultrastability`.

But the number you get **depends entirely on which being you run**:

| | inert gates |
|---|---|
| abstract path | **8 of 17** |
| the being's own room | **7 of 17** |

and `receptors` is the difference — **Δ 0.000 abstract, 2.284 embodied**, the second
largest effect in the room, and §6 of this document already called it *the being's
whole life*. **An abstract-only census would have scored the most important faculty
we have as dead weight.** The rank order barely survives the change of world at all:
`freeze_basin_targets` is first abstractly (2.831) and fourth in the room (0.027);
`reflection` is third abstractly (0.549) and **zero** in the room.

**C3 failed as predicted, and cleanly.** The largest half of the modules carries Δ
5.581; the smallest half carries 5.752. Size predicts nothing. `homeostasis.rs` at
150 lines holds `settling`; `episodic.rs` at 763 lines holds two gates that are both
completely inert.

**C4 holds and is uncomfortable.** Sixteen modules assert in their own doc comments
that they are inert — *"observer-first"*, *"nothing downstream reads it"*, *"changes
no published number"* — and **five are named in no test at all**: `habits`,
`homeostasis`, `inheritance`, `joy`, `sensorimotor`. By this project's own rule an
unchecked claim is not a passed one.

#### Two honest limits on the number above

* **The composite Δ is largely an occupancy ranking.** Decomposed, basin occupancy
  carries **64%** of `freeze_basin_targets`' Δ and **83%** of
  `workspace_persistence`'s. `reflection` ranks third on valence alone, with
  occupancy Δ exactly 0. The ranking is not a general "amount of change" and should
  not be read as one.
* **This is the optional half only.** Seventeen switches out of sixty-four modules.
  Nothing here licenses a claim about the being's non-optional core.

#### What this licenses, and what it does not

It does **not** license *"the being is 40% dead weight."* It licenses this:

> **A large part of this being does nothing until it is given a world to do it in,
> and the census you run decides how much of it you would delete.**

The seven gates dead in both worlds are the real candidates — and even for them, §6
is the standing warning: the faculty that turned out to be the being's whole life
was the one switched off. The move stays a **fork, not a deletion**: the full being
is the hypothesis space, a minimal being becomes the claim, and everything else is
labelled honestly as unexercised capacity.
