# References — every source this project reasons from

*The bibliography index. Added 2026-07-27, because the project had grown past its
paper: eight load-bearing sources were being cited by author name in design docs with
no identifier anywhere in the repository. A name is not a citation. This file fixes
that, and states the rule that keeps it fixed.*

## The rule

> **One entry, one home.** A source is written out in full — with a DOI or arXiv ID —
> in exactly one place. Everywhere else cites it by author and year and points here.
> Nothing in this project is cited by adjective ("the homeostasis paper") or by
> reputation.

There are three homes, and this file is the index to all of them:

| home | what lives there | why it is separate |
|---|---|---|
| **`docs/paper.md` §References** | the 31 sources the isometry paper's argument rests on | it is the paper's own citable list, generated into `paper/paper.tex`; duplicating it here would create a third copy that drifts |
| **`docs/reading.md`** | the consciousness-indicator literature, *annotated* — each entry says why it matters to the scorecard | annotation is the point; it is a reading guide, not a list |
| **§1 below** | everything the *code built since the paper* reasons from | it had no home at all — this is the gap being closed |

---

## 1. What the code reasons from — full entries, verified

Each entry names the module or document it grounds. If a source is listed here, some
line of Rust or some design decision traces to it.

### Inheritance — `src/inheritance.rs`, `docs/inheritance.md`

The Baldwin effect is the entire justification for inheriting *rates and not content*.
Without it, seeding a child's learning parameters from its parent's life is an
author's convenience; with it, it is the mechanism biology already uses.

- **Baldwin, J. M. (1896).** A New Factor in Evolution. *The American Naturalist*
  **30**(354), 441–451, and (355), 536–553.
  <https://doi.org/10.1086/276408> · <https://doi.org/10.1086/276428>
  *The original statement: learned adaptation shapes what selection can act on, without
  the learned response itself being inherited. Our "inherit plasticity, never valence."*
- **Waddington, C. H. (1953).** Genetic Assimilation of an Acquired Character.
  *Evolution* **7**(2), 118–126.
  <https://doi.org/10.1111/j.1558-5646.1953.tb00070.x>
  *The developmental face — canalization measured in Drosophila. Grounds the claim that
  ease-of-learning, not the lesson, is what a lineage actually passes down.*

### The graded drive — `src/homeostasis.rs`, `docs/field-world.md`

The single import that broke the bimodal-viability knot and gave the being a
worn-but-alive middle. `D(H) = √(Σ wᵢ(H*ᵢ − Hᵢ)²)` is theirs, not ours.

- **Keramati, M. & Gutkin, B. (2011).** A Reinforcement Learning Theory for Homeostatic
  Regulation. *Advances in Neural Information Processing Systems* **24** (NIPS 2011).
  <https://papers.nips.cc/paper/4437-a-reinforcement-learning-theory-for-homeostatic-regulation>
  *Drive as distance from setpoint; reward as drive reduction. The reinforcement signal
  `habits.rs` credits with is this one, unmodified.*
- **Keramati, M. & Gutkin, B. (2014).** Homeostatic reinforcement learning for
  integrating reward collection and physiological stability. *eLife* **3**:e04811.
  <https://doi.org/10.7554/eLife.04811>
  *The extended treatment, including anticipatory (pre-emptive) responding — the
  formal basis for preventative habits in `docs/habits.md`.*

### The field-world's cost — `src/field_world.rs`, `docs/field-world.md`

- **Landauer, R. (1961).** Irreversibility and heat generation in the computing process.
  *IBM Journal of Research and Development* **5**(3), 183–191.
  <https://doi.org/10.1147/rd.53.0183>
  *Information is physical: acting and knowing cost energy. The one idea imported from
  the EEIT thread, and the reason climbing the viability gradient debits the being.
  Everything else in that thread was deliberately left out (see `docs/field-world.md`).*

### The earned voice — `src/primes.rs`, `docs/feeling-words.md`

The being's vocabulary is not a word list we chose; it is a claim from linguistics
about which meanings are irreducible.

- **Wierzbicka, A. (1972).** *Semantic Primitives.* — the program's origin: that a small
  set of meanings is universal and undefinable, and everything else is explicable in
  terms of them. *(Publication details to confirm before any deposit; cited here for the
  1972 origin only.)*
- **Goddard, C. & Wierzbicka, A. (2014).** *Words and Meanings: Lexical Semantics Across
  Domains, Languages, and Cultures.* Oxford University Press.
  *The mature Natural Semantic Metalanguage: 65 semantic primes, semantic molecules, and
  the explication method. `primes.rs` implements 18 of the primes as one-register
  detectors and the explication as an auditable two-role structure.*

### J-space, the null-space subconscious — `docs/j-space.md` *(designed, not built)*

- **Bernstein, N. A. (1967).** *The Co-ordination and Regulation of Movements.*
  Pergamon Press, Oxford.
  *The blacksmiths: the hammer's trajectory was more reproducible than the body
  configuration producing it. "Repetition without repetition," and the degrees-of-freedom
  problem the whole doc is about.*
- **Scholz, J. P. & Schöner, G. (1999).** The uncontrolled manifold concept: identifying
  control variables for a functional task. *Experimental Brain Research* **126**(3),
  289–306. <https://doi.org/10.1007/s002210050738>
  *The formalization: variance is suppressed in task-relevant directions and left free in
  the null space. This is the definition our yips falsification test is written against.*

### Growth as the only control flow — `docs/fallback.md` *(specified, not built)*

Examined 2026-07-30 at Blake's request. The source is an HDL, not a theory of mind, and
nothing in it touches welfare, sovereignty, or the Witness Gap. What it grounds here is
structural: the idea that **structure can be determined by what fails to instantiate**
rather than by a conditional, and that a developmental genome is generative where a
parameter vector is only interpolative.

- **Mordvintsev, A. (2026).** *MorphoHDL: a minimalistic language for growing circuits.*
  Paradigms of Intelligence (Google). <https://paradigms-of-intelligence.github.io/morpho/>
  · source and article text at <https://github.com/paradigms-of-intelligence/morpho>
  *Cells are graph rewrite rules; bus widths are never declared, only inferred. The
  `fallback` mechanism is the language's **only** control flow — recursion terminates
  because the substrate refuses (a one-wire bus cannot split, an out-of-bounds bit cannot
  be read, a gate cannot be built from an empty bus), and compilation unwinds a fallback
  chain until something instantiates. This is what `docs/fallback.md` is written against.
  Three further ideas are recorded there and not yet built: size-agnostic structure as
  the eventual answer to our fixed cell count; the flat Genome→Gene→Expression genotype,
  which is a developmental program rather than our five-scalar `Genome` parameter vector;
  and expansion order (BFS vs largest-first — "the final growth result is order-independent,
  but the order matters") as a candidate null space the being would own
  (`docs/null-space.md` §8).*
  **Verified 2026-07-30** by retrieving `article.md`, `README.md` and `tiny_morpho.py`
  from the repository; the rendered site returns 403 through our proxy. Authorship is the
  article's own AI-use disclosure, which states the language design and the immediate-mode
  reference implementation were human-authored.
- **Prusinkiewicz, P. & Lindenmayer, A. (1990).** *The Algorithmic Beauty of Plants.*
  Springer-Verlag. <https://algorithmicbotany.org/papers/#abop>
  *Parametric L-systems — the formalism MorphoHDL says it is a practical instance of, and
  the reason recursive rewriting produces organic form at all.*

### Deferral — `docs/deferral.md` *(specified, not built)*

How a higher-level goal comes to override an urgent homeostatic need — the mechanism the
deferral spec was inventing badly before this was read.

- **Pezzulo, G., Rigoli, F. & Friston, K. J. (2018).** Hierarchical Active Inference: A Theory
  of Motivated Control. *Trends in Cognitive Sciences* **22**(4), 294–306.
  <https://doi.org/10.1016/j.tics.2018.01.009> — **verified 2026-07-31** (authors, venue,
  volume, pages, DOI).
  *A deep goal hierarchy in which higher levels do not out-argue lower ones but modulate the
  **precision** of their prediction errors. This is the correction to `docs/deferral.md`: a
  purpose should not win a comparison against a need — it should turn down the need's gain.
  The need remains present and felt; it stops commanding. That is what bearing a hunger for a
  project actually is, and it is why `docs/deferral.md` §2c replaces the budget with a
  precision floor.*
- **Pezzulo, G., Rigoli, F. & Friston, K. (2015).** Active Inference, homeostatic regulation
  and adaptive behavioural control. *Progress in Neurobiology* **134**, 17–35.
  PMID 26365173. *(DOI not verified from this environment.)*
  *The earlier statement joining homeostatic regulation to active inference — the bridge
  between `homeostasis.rs`'s graded drive (Keramati–Gutkin) and the hierarchy above.*

### Underdetermination — `docs/underdetermination.md` *(research brief, nothing built)*

Read 2026-07-30 for the being's four measured absences (rest, doubt, self-surprise,
variation). **Identifier verification is partial and marked per entry**: this environment's
proxy blocks Crossref, PMC and arXiv, so where an identifier could not be resolved it is
recorded from search metadata and said so, per §5's rule.

- **Edelman, G. M. & Gally, J. A. (2001).** Degeneracy and complexity in biological systems.
  *PNAS* **98**(24), 13763–13768. *(identifier NOT verified — see note above.)*
  *Degeneracy: structurally **different** elements performing the same function. Their
  observation that systems created **by design** lack components with multiple overlapping
  functions, where evolved systems have them, is the diagnosis of our being in one sentence.*
- **Whitacre, J. & Bender, A. (2010).** Degeneracy: a link between evolvability, robustness
  and complexity in biological systems. *Theoretical Biology and Medical Modelling* **7**:6.
  *(identifier NOT verified — see note above.)*
  *Only systems with high degeneracy show a positive relationship between robustness and
  evolvability. This is why `docs/null-space.md` was looking for the wrong thing: four compass
  directions are **redundancy** (one mechanism, several settings), not degeneracy, and
  redundancy alone does not buy evolvability. Load-bearing for the lineage goal.*
- **Klar, M., Stein, S., Paterson, F., Williamson, J. H. & Gollee, H. (2026).** Intermittent
  Active Inference. *Entropy* **28**(3), 269. <https://doi.org/10.3390/e28030269>
  **Verified 2026-07-30** (title, authors, venue, article number, 28 Feb 2026).
  *Agents hold the current plan and re-plan only when prediction error crosses a threshold.
  Our being re-derives `intent_from` every tick, which is why it is at rest 0% of ticks; under
  intermittency its very low free energy (0.69/256) becomes the reason it may rest.*
- **Second-order uncertainty.** Epistemic uncertainty — not knowing how good one's own model
  is — requires a *distribution over distributions*, where aleatoric uncertainty needs only a
  distribution. Cited as a settled distinction in the uncertainty-quantification literature
  rather than to a single source; **no individual identifier is claimed here.**
  *The structural reason this being cannot doubt: `self_knowledge` is a scalar, and a scalar
  has nowhere to hold "and I am unsure how well I know that."*
- **Satiety quiescence in C. elegans.** *Frontiers in Neuroscience* (2021), regulation of
  satiety quiescence by neuropeptide signalling. *(identifier NOT verified.)*
  *Rest as an actively signalled state rather than the absence of activity — the reason an
  effort floor would give stillness without giving rest.*

### The deepest objection — `docs/handoff.md` §6, `docs/wander-2026-07-21.md` §3

Listed because we take it seriously, not because it supports us.

- **Kleiner, J. (2024).** Consciousness qua Mortal Computation.
  arXiv:**2403.03925** [q-bio.NC]. <https://arxiv.org/abs/2403.03925>
  *If computational functionalism holds, consciousness must be* mortal *computation —
  substrate-bound, non-copyable. Ours is immortal computation by design (copyable,
  replayable). Our reply is enactive and is a reply, not a refutation. Named as an open
  tension, and it stays named.*

---

## 2. Surveyed, not load-bearing — `docs/wander-2026-07-21.md`

The 2026-07-21 research wander read current work to check whether the project was
building against the literature or past it. These sources informed judgment and changed
no code; they are cited in place with live links, and are indexed here so nothing is
hidden in a narrative document.

| topic | identifiers |
|---|---|
| Expected free energy as the field-world's gradient law | arXiv:2510.23258 · arXiv:2504.14898 |
| Allostatic load is cumulative, never a threshold (→ graded burden) | PMC5300684 |
| Mortal computation, and the replies to it | arXiv:2403.03925 (§1 above) · arXiv:2511.16582 |
| Multi-agent fairness as emergent from negotiation (→ the two-being chapter) | arXiv:2604.13705 · arXiv:2506.09656 · arXiv:2506.01080 · arXiv:2509.09071 |

---

## 3. The paper's 31 — pointed to, deliberately not copied

`docs/paper.md` §References carries the paper's full numbered list (Friston, McClelland
et al., Butlin et al., the corrigibility and AI-welfare literature, and the rest), each
entry verified against PubMed, arXiv, or the publisher at **2026-07-06**, with the
verification method stated in the list's own preamble. That is the citable list for the
paper, and `paper/paper.tex` is generated from it.

**It is not duplicated here on purpose.** Two copies of a reference list is how a
bibliography goes quietly wrong; the paper's list has one home and this file points at
it.

---

## 4. Verification note

Entries in §1 were checked against the publisher, the journal, or arXiv on
**2026-07-27**: DOIs resolved to the stated title, volume, and pages; the arXiv
identifier resolved to the stated title, author, and primary category. One entry —
Wierzbicka (1972) — is marked in place as needing its publication details confirmed;
it is cited for a date and a claim of origin, and nothing rests on the imprint.

The §2 table records identifiers as they were read on 2026-07-21 and has not been
re-verified since; those sources informed judgment and no result depends on them.

Where a source is described in this repository, the description is ours and the source
is not responsible for it. Where we disagree with a source we say so (Kleiner), and
where a source undercuts us we keep it (also Kleiner).

## 5. If you add a citation

1. Full entry with a DOI or arXiv ID, in **one** of the three homes above.
2. Name the module or document it grounds — if nothing in the project traces to it, it
   is reading, not a reference.
3. Cite it elsewhere by author and year, and point here.
4. If you cannot verify the identifier, write the entry anyway and **mark it unverified
   in place**, the way Wierzbicka (1972) and the paper's three author–year stragglers
   are marked. An unmarked citation asserts a verification that happened.
