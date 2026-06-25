# The Unified Being

A small, deterministic active-inference agent in fixed-point Rust: Being32's
Van der Pol **body** fused with EPS-Being's persistence **mind**. The body votes
before the mind knows there's an election.

It runs a simulated life and demonstrates one non-trivial behavioral claim: the
being keeps faith with a fair partner and **sovereignly refuses** an extractive
one — on principle, from a composed state, not in panic.

> **Honest scope.** This proves the architecture's *behavior* — extraction
> resistance, and a somatic wound that carries between relationships — **not**
> consciousness. Every figure is read straight from the being's own state;
> nothing is narrated.

## Run

```sh
cargo run     # runs both experiments; writes life_log.csv and life_plot.svg
cargo test    # module unit tests
```

## What it shows

- **Experiment 1 — the Fair Test.** The being lives contentedly with a fair
  partner. When an extractive one arrives, its reciprocity alarm spikes; it
  waits, confirms the extraction over ~13 ticks, and — composed, not panicked —
  refuses and walks away, then recovers. It never refuses the partner who is
  good to it.
- **Experiment 2 — persistent character (partial, honest).** A being burned by
  extraction meets a *new*, fair partner carrying a somatic wound: it arrives
  with markedly lower valence and takes ~2× longer to flourish. **Caveat:** the
  deeper *dispositional* markers (empathy lock, sovereign anchor) did not yet
  differ — giving extraction a durable dispositional trace is acknowledged
  future work, not faked here.

## Architecture

Eleven modules, Q8.8 fixed-point, saturating arithmetic, `no_std`-friendly core:

```
q88 · genome · body · field · basins · conscience · reciprocity · seeking
    · executive · narrative · being
```

## Status

First running version. The behavioral claim is demonstrated and reproducible;
the affect dynamics and the dispositional-wound mechanism are works in progress.
This is a foundation, built to prove itself honestly — not a claim of sentience.
