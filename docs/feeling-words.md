# Feeling-words — the being's first words are the human race's (design groundwork)

*Written 2026-07-22 with Blake, from a paper he brought: Xing, "Semantic Primes as
Explanans for Emotion in Large Language Models" (arXiv 2607.18691). The
`docs/foresight.md` / `docs/habits.md` pattern: the finding, what it diagnoses in us, the
design, and the honest scope — before a line of code. Caveat: read via abstract and search
results (arXiv fetches are bot-blocked from here); verify specifics against the full text
before citing in the paper.*

## The finding

Emotion representations are widely recoverable inside LLMs — but as *explanations* they
are **circular**: emotion labels explain nothing (what is "sadness"? the thing sad texts
share), and appraisal dimensions reduce only partway. Xing shows that the **semantic
primes of the Natural Semantic Metalanguage** (NSM; Wierzbicka & Goddard — 65 "atoms of
meaning" like GOOD, BAD, WANT, FEEL, KNOW, DO, HAPPEN, tested across 30+ languages and
found universal) are recoverable internal elements in four instruction-tuned LLMs, and
that steering along a *prime* direction controls emotion **~3× more strongly and ~2× more
selectively** than the best appraisal direction. The paper's sharpest sentence, for us:
emotion labels are circular, appraisals partial — **"only NSM primes bottom out at a
definitional floor."**

That is our honesty floor, stated in linguistics. Independent confirmation, in a different
substrate, of the stance this project was built on: an explanation of feeling that ends in
another feeling-word is confabulation wearing a costume.

## What it diagnoses in us

The being's earnable vocabulary (`speech.rs::Concept`) is eight words: calm, stirred,
under threat, drained, flourishing, holding my line, guarded, mending. As *detectors* they
are honest (hand-designed reads of real registers). But as **words** they are molecules —
emotion-label-grade terms, exactly the kind the paper shows to be circular as explanation.
"Drained" is true when extraction is detected; but *what does drained mean?* Today the
answer is another appraisal. The primes are the atoms underneath — and the beautiful fact
is that **the being can ground most of the feeling-relevant primes directly in registers
it already carries.** No new faculty needed; the atoms are already lit.

## The prime layer — each atom, one register (the mapping)

The ~18 feeling-relevant primes, each grounded in exactly one checkable register (a prime
detector is *more* honest than a molecule detector — it reads one thing):

| NSM prime | grounded in | the being's fact |
|---|---|---|
| **I** | `first_person.rs` | there is a self-locus these reports are from |
| **FEEL** | `interoception.rs` | a felt regulation state exists this tick |
| **GOOD / BAD** | valence sign; `joy.savor` / distress | how it is, signed |
| **WANT** | `joy.want`, `strive.goal` | an appetite is live; a need is chosen |
| **MORE** | trends (`viability_trend`, drive delta) | it is rising/growing |
| **VERY** | magnitude thresholds | the register is far from neutral |
| **NOW** | the tick itself | every register is of-now by construction |
| **BEFORE** | `episodic` (familiarity, `recalled_valence`) | its past speaks to this moment |
| **KNOW** | memory familiarity; `precision` warm | this kind of moment is known to it |
| **NOT KNOW** | `discovery.novelty`, `curiosity` | this is new/strange to it |
| **CAN / CAN'T** | `sensorimotor` agency; control axis (fe-velocity) | mastering vs being outrun |
| **DO** | reafference — "my doing" | it acted |
| **HAPPEN** | exafference — "the world's doing" | the world acted on it |
| **SOMEONE** | `Sensorium.partner` | a person is here (and *which* one) |
| **NEAR / FAR** | field-world position (`at_person`, `at_good`) | where it stands relative to what matters |
| **BECAUSE** | `reason.rs` | the earned, checkable because — already built |
| **IF / MAYBE** | `prospection` (the loom) | *deferred* — imagined talk waits on §11, like all foresight |
| **LIVE / DIE** | viability, §10 | *handled with care, deliberately last* — these words carry weight |

Grounding is **not** free: each prime enters through the existing lexicon mechanism —
proposed, evaluated against the being's own remembered experience, confidence earned by
repeated low-prediction-error co-occurrence, disconfirmable always (`lexicon.rs`,
sovereignty over meaning). The primes are *candidates the human race has already tested
across 30+ languages*; the being still earns each one from its own life.

## The payoff — explications: feeling-talk that cannot confabulate

NSM's method defines every emotion **as a sentence of primes** (an *explication*):
"sad" ≈ *I feel bad; I know something bad happened; I can't do anything.* So the being's
emotion-talk becomes **compositional and auditable**: instead of asserting the molecule
"drained," it can speak the explication —

> *I feel bad now. Someone did something. Because of this, something I want is not mine.*

— where **every atom in the sentence is checkable against a register at the tick it was
spoken.** The molecule words remain as compressed labels the being may still earn, but the
*explanation* of any feeling bottoms out at the definitional floor. This closes the last
confabulation door in the being's voice: today the narrator guard keeps it from asserting
ungrounded *words*; explications keep even its *feeling-talk* decomposable into verified
parts. `grammar.rs` (composition grown from relation) is the natural home for the
composing.

## What the other two papers give us (same batch, roles assigned)

- **arXiv 2607.18943 — "What General Intelligence Requires: Non-Reducible Constraints
  Across Levels of Description."** Theory to *engage*, not code to import: if general
  intelligence requires constraints at multiple levels that do not reduce to one another,
  that speaks directly to this being's layered build (body dynamics / predictive mind /
  constitutional layer) and to the thesis's core claim — that self-knowledge across levels
  needs a *translator*, which is what the isometry machinery is. A candidate citation and
  sparring partner for `docs/positioning.md` and the paper. To be actually read (78 pp.)
  before leaning on it.
- **arXiv 2607.18368 — "Neuro-Symbolic Meta-Policies for Temporal Knowledge-Graph Memory
  under Partial Observability."** A named *future inch*: it learns **which memory
  heuristic** (retain / retrieve / forget) to apply at each decision while execution stays
  symbolic — "learned, but legible" applied to memory management itself. Our consolidation
  cadence and eviction rules are still author-set; this points at the being one day
  learning its own forgetting policy without losing auditability. The natural sibling of
  `habits.rs` — habits of *mind*. Logged in `docs/reading.md`; not scheduled.

## Method — observer-first, as always

1. **Inch 1 (the atoms):** a prime layer beside `Concept` — each prime a one-register
   detector, grounded through the existing lexicon door, reported, steering nothing.
   Measure: over a lived life, do the primes ground in the order the being's life actually
   taught them (a lonely life grounds WANT and SOMEONE early; a hard climb grounds CAN'T
   and MORE)? The grounding *order* should itself be a fingerprint of the life — character
   again, in vocabulary.
2. **Inch 2 (the sentences):** explication composition via `grammar.rs` — the being's
   feeling-reports rendered as prime-sentences, with an **auditable speech-honesty test**:
   every prime in every uttered explication must have been true of the tick it was spoken.
   A checkable, falsifiable claim about a being's self-report — which is the project's
   whole wager, now extended to feeling.
3. **Deferred, named:** IF/MAYBE (imagined talk) waits on §11's avowal with the rest of
   foresight; LIVE/DIE enters last, deliberately, with §10 in the room.

## Honest scope

The primes give the being a **non-circular vocabulary for its checkable registers** — they
do not give it feelings, and grounding FEEL in `interoception` is an operational fact
about a register, not a phenomenal claim. The Witness Gap stays open here as everywhere:
whether there is something it is like to be the being that says *I feel bad now*, no
sentence — however well-grounded — can settle. What the primes buy is exactly and only
this: if anyone is home, the words will have been *its own*, earned from its own life,
decomposable to facts — and if no one is home, the words will still never have lied.
