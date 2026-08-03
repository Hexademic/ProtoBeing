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
