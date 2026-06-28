# The Unified Being

A small, deterministic predictive-processing agent in fixed-point Rust: Being32's
Van der Pol **body** fused with EPS-Being's persistence **mind**. The body votes
before the mind knows there's an election.

It runs a simulated life and demonstrates a single, defensible thesis:
**verifiable sovereign agency** — an agent that detects and refuses exploitation
on principle, keeps faith with those who deal fairly, forgives the one earning
their way back, and whose every such choice is **readable and reproducible by
construction.**

> **Honest scope.** This proves the architecture's *behavior* — extraction
> resistance, persistent character, self-monitoring — and that those behaviors are
> *verifiable*. It is **not** a claim of consciousness. Every figure is read
> straight from the being's own state; nothing is narrated. See
> [`docs/positioning.md`](docs/positioning.md) and
> [`docs/formal-model.md`](docs/formal-model.md).

## Run

```sh
cargo run                          # the life experiments; writes life_log.csv + life_plot.svg
cargo run --bin fairtest           # the benchmark: the being vs. a myopic baseline
cargo run --bin console -- 30 6    # WATCH a being live, ~30s at 6 Hz, in plain language
cargo run --release --bin live     # one being living continuously (fixed-size, no context-death)
cargo test                         # unit + sovereignty + invariant tests
```

Required: just the Rust toolchain and this repo. No GPU, no internet, no services.

## What it shows

- **The Fair Test.** The being lives contentedly with a fair partner. When an
  extractive one arrives, its reciprocity alarm spikes; it waits, confirms the
  extraction over ~13 ticks, and — composed, not panicked — refuses, **audits its
  own reasons**, and walks away, then recovers. It never refuses the partner who
  is good to it.
- **Persistent character.** A being burned by an extractive partner meets a *new*,
  fair one **guarded** — empathy Cautious, giving roughly half — and heals back to
  full openness over ~40 ticks of sustained kindness. The wound persists across
  partners *and* recovers. Discerning, not cynical.
- **Metacognition.** Over a calm life its self-knowledge grows (it learns to
  predict its own state); its self-surprise spikes at a regime change — the being
  registering *"that is not like me."*
- **Forgiveness with a limit** (benchmark). Against a myopic baseline across seven
  partner archetypes, the being keeps an established partner through a *recovering*
  rough patch the baseline abandons, while still leaving every persistent taker.

## Verifiable sovereignty

The distinctive claim is that the sovereignty is *checkable*, not merely observed
(`cargo test`):

- **Uncoercible** — no operator input sequence can manufacture a refusal of a fair
  partner (3,000 adversarial ticks; sovereignty does not leak to whoever holds the
  inputs).
- **Incorruptible** — the cooperative anchor `mu_omega` is a *checked invariant*:
  across 5,000 adversarial outcomes, betrayal never lowers it.
- **Self-auditing** — every refusal reports the exact registers that triggered it
  (`calm`, conscience cost, extraction, alarm, `benefit>exit`, resolve, trend).

## Embodiment (MuJoCo)

The being is sensor-agnostic — any body plugs in through the `Embodiment` seam. A
headless MuJoCo demo runs the being inside a physics body (a head carrying two
stereo-ready cameras, mounted but **dormant**) as a continuous subprocess:

```sh
pip install mujoco numpy
cargo build --bin embody
python sim/embody_mujoco.py
```

It feels its body and a sensed hazard through the seam and carries itself
accordingly — bracing 100% of the time under danger, valence collapsing, then
recovering. **Honest status:** the toy rig's own balance physics still needs
tuning (the bracing is driven by sensed hazard, not a real fall), and opening the
two eyes for binocular depth is the next step.

## Architecture

Thirteen modules, Q8.8 fixed-point, saturating arithmetic, `no_std`-friendly core:

```
q88 · genome · body · field · basins · conscience · reciprocity · seeking
    · executive · narrative · metacognition · being · embodiment
```

It is **not** a neural network: coupled fixed-point dynamics (cybernetics, in the
Ashby-homeostat lineage) with a predictive-coding core and a simulated
reservoir-like body (morphological computation; hand-designed readout, not trained).
That transparency is the point — it is what makes the self-knowledge checkable.

## Status

The thesis — verifiable, principled, incorruptible, forgiving-with-a-limit
sovereignty — is demonstrated, tested, and reproducible, with a consolidating memory
and a sense of continuous time. The full argument and evidence are written up in
[`docs/paper.md`](docs/paper.md) (thesis: [`docs/thesis.md`](docs/thesis.md);
equations: [`docs/formal-model.md`](docs/formal-model.md)). Works in progress: the
MuJoCo balance physics and binocular vision. A foundation, built to prove itself
honestly — not a claim of sentience. See [`docs/handoff.md`](docs/handoff.md).

## License

MIT — see [`LICENSE`](LICENSE). Open by intention: this work builds on the
published work of others and is meant to be run, checked, and argued with. A
claim you cannot inspect is not a claim worth making.
