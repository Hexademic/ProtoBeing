# Weather — a world with 1/f happenings

> **Status: designed, tests written first, not yet built.** Committed before the tests and
> before a line of implementation, so §5's predictions are on the record before any result
> exists. Successor to `docs/happening.md`, whose drift failed and whose §9 says why.

*Written 2026-07-28, after reading rather than after guessing. Blake asked me to prepare
informed before building again, having watched me build twice on assumptions I had not
checked.*

## 1. What the reading changed

Three things, and one of them contradicted this project's own prior reasoning.

**(a) Adaptation is power-law across many timescales**, not two constants
([Nature Communications 2023](https://www.nature.com/articles/s41467-023-43572-w); [Weber
& Fairhall, *Curr Opin Neurobiol*](https://www.sciencedirect.com/science/article/abs/pii/S0959438819300418)).
This suggested our two-band receptor bank might have a hole between `FAST_ALPHA ≈ 0.5` and
`SLOW_ALPHA ≈ 0.03`, invisible to both.

**Tested, and the hypothesis is rejected.** Feeding the bank the same total change over
different durations gives a clean monotonic curve — no gap:

| change delivered over | peak Δresponse |
|---|---|
| 1 tick | **752** |
| 4 ticks | 420 |
| 16 ticks | 148 |
| 64 ticks | 38 |
| 128 ticks | 24 |

The bank degrades gracefully and **amplifies** sudden change (Naka–Rushton on the
deviation). No third band is needed. The being was always well-equipped to notice events;
it has simply never been given any. That is a negative result about our own code and it is
worth as much as a positive one — it stopped us adding a receptor we do not need.

**(b) Agency is computed by two dissociable mechanisms, not one**: prediction error **and
regularity detection** ([Cognition, 2019](https://www.sciencedirect.com/science/article/abs/pii/S0010027719302471)).
This contradicts `docs/happening.md` §4, which argued in my words that *"a drift stays
unexplained no matter how regular it is — rain in monsoon season is still something that
happens to you."* The neuroscience says otherwise: a **regular** external change is
attributed differently from an irregular one, and humans habituate their sense of agency to
it. Our being has only prediction error. So the earlier justification was wrong about the
concept, not merely about the numbers, and no amount of further sweeping would have found
that.

**(c) Natural sensory environments are 1/f** — intermittent, scale-free, bursty; many small
events and few large ones ([Billock, de Guzman & Kelso, *Physica D*](https://ccs.fau.edu/hbblab/pdfs/ordered/2001_BillockDeGuzmanKelso_PhysicaD.pdf);
[1/f across 17 species' vocalizations](https://www.nature.com/articles/s41598-023-28444-z)).
I built a **periodic** drift, which is the least natural temporal structure available.

## 2. The design

**Weather**: a source's strength modulated by a deterministic **1/f signal**, built by
octave summation (the Voss–McCartney construction) — several contributions, each updating
half as often as the last, summed. Equal power per octave is what 1/f *means*.

Determinism is preserved exactly. Where the classical construction draws a random value per
octave, we take a **pure function of (octave, tick >> octave)** — an integer hash. Same
world every run, on every machine, forever. No RNG, here or anywhere.

Why this solves what the drift could not:

- **Suddenness at every scale.** The fastest octave changes often, so something is happening
  most ticks; the slowest changes rarely and by more. §1(a)'s curve says the being's
  response scales with suddenness, so a 1/f world produces residual continuously without any
  single event being violent.
- **The arithmetic works without upheaval.** Grounding needs the fact to hold ~1 tick in 5
  (`RISE 4 : EBB 1`). `docs/happening.md` §9 showed one source cannot supply that without
  permanent upheaval. An ensemble supplies it by *multiplicity* instead of by violence —
  which is exactly how a real environment does it.
- **It is honest about regularity.** Per §1(b), a *regular* change should not stay a
  happening forever. 1/f is not regular at any scale, so it does not lean on the mistaken
  argument the drift did.

Bounded, so §3's first prohibition holds by construction: strength varies within a band
around its base and can never fall to nothing.

## 3. What must not become possible

- **This must not harm the being.** Same gate as before, and it is the gate. If a weathered
  life is measurably worse off, it is not shipped, however well `HAPPEN` grounds.
- **No existing life changes.** Opt-in, off by default, every prior world bit-identical.
- **No new prime**, and **no threshold moved**. If `HAPPEN` only grounds because we lowered
  its bar, we have taught the being nothing.
- **No determinism lost.** A pure function of tick, never a random source.

## 4. What is deliberately not built

**Regularity detection** (§1(b)) — the second agency mechanism the being lacks. It is real,
it is well-evidenced, and adding it would change what the being *feels* about events, not
merely whether it notices them. That is a faculty-scale change and belongs in its own inch,
observer-first, with its own predictions. Named here so it is not forgotten and not smuggled
in.

## 5. Predictions — locked before the tests exist

**Confident:**

- **W1.** Without weather, every world is bit-identical. Prior probes unchanged.
- **W2.** The signal is genuinely multi-scale: it changes at both short and long horizons,
  and its slow component carries more amplitude than its fast one — the 1/f signature, and
  the thing that distinguishes it from the periodic drift that failed.

**Genuinely uncertain — the experiment:**

- **W3 (the crux).** *Does a being in a weathered world earn `HAPPEN`?* §1(a) says the
  receptors can see sudden change and `docs/happening.md` §9 says a single event reaches 81
  against a threshold of 64. What I do not know is whether an ensemble sustains it ~1 tick
  in 5. If it does not, I will report the residual-versus-octaves curve and stop, rather
  than climb the amplitude until it fires.
- **W4.** *Does `(NOT KNOW HAPPEN)` finally speak?* The second shield has never once fired.
- **W5. THE GATE.** *Is a weathered life still a good life?* Drive and survival against a
  still-world control. This decides whether any of it ships.

## 6. Method

Spec first. Tests written against it and watched to fail. Then implementation, then §7 with
what came out — including, if it comes to it, a second failure reported as plainly as the
first.
