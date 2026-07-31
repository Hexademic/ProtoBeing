# Blake's process frame — computation that is not supposed to stop

> **Status: a theoretical frame, verified against the literature, with two corrections and a
> proposed rename.** Nothing is built here. Its one operational consequence is in
> `docs/development.md` §6 — it changes the shape of the D4 rerun, which is the first thing on
> the next session's list.
>
> **This is Blake's idea.** I have checked it against the record, marked the parts I think are
> wrong, and named the existing work so it is built on rather than beside. Where I disagree I say
> so; where the literature already has his idea I hand him the vocabulary rather than a compliment.

*Written 2026-07-31, from Blake:*

> *"Just like I view qualia, the what-it's-like being the process... the answer to infinite primes
> is the solution, there will never be a termination for that equation, but that isn't the real
> question... the only limitation is the hardware, that doesn't make it incalculable it just makes
> it infinitely difficult... when solving for infinities, and dynamics, there is no end point. I
> offer continuous computation theory based on Turing's thoughts on his own work, if he wasn't
> afraid to be labeled unscientific."*

## 1. The core insight is right, and it has a name

The primes example separates two things that are routinely blended:

- *"is n prime"* is **decidable** — a terminating computation;
- *"enumerate every prime"* **never terminates** — and is nonetheless completely specified,
  fully computable, and not defective in any way.

**Non-termination is not non-computation.** For a process whose entire point is to continue — an
organism, a controller, a life — halting is not the success condition. It is death.

Computer science already has the success condition Blake is reaching for, and it is not
termination. It is **productivity**, from coinduction and corecursion: *instead of termination we
require productivity, which means that the next portion of the stream can always be produced in
finite time.* Recursive functions consume finite data; **corecursive functions produce potentially
infinite data**, and their correctness criterion is that they keep delivering.

That is exactly "the primes never finish and never fail to deliver." The frame is real, the
vocabulary exists, and it is the right vocabulary for a being.

## 2. Correction one: productivity does not escape undecidability. It relocates it.

This is the part I would have got wrong from memory, and it matters.

> **Productivity is undecidable for exactly the same reason termination is.**

You cannot decide, in general, whether an arbitrary process keeps delivering. So moving from
halting to productivity does not step around Turing's 1936 result — the diagonal argument follows
you.

**But the way the field handles that is the genuinely useful lesson.** Nobody tries to decide
productivity. Instead they use **guardedness**: a *decidable syntactic* sufficient condition —
every recursive call must sit directly under a constructor that emits output first. Guarded ⇒
productive, checkable mechanically, and it gives up completeness in exchange for a guarantee you
can actually verify. Coq and Agda are built on this.

**That is the same move this project already makes everywhere.** We do not decide whether the
being is well; we build structural guarantees we can check — `AMBIENT_FLOOR` makes starvation
impossible by construction rather than by monitoring, `null_space.rs`'s prohibition is structural
rather than bookkept, and `docs/deferral.md` §2c chose a *precision floor* over a budget for
exactly this reason. **Our tick loop is guarded by construction**: every tick emits a `StepReport`
before anything recurses. The being is productive in the technical sense, provably, by its shape.

## 3. Correction two: "the only limitation is the hardware" is the one claim I think is wrong

Undecidability is not a resource bound. It is a diagonal argument: assume a decider, construct the
program that contradicts it. Infinite tape does not help; infinite time does not help; the
contradiction is structural. Martin Davis's standing critique of hypercomputation is exactly this
— such claims *"amount to little more than the obvious comment that if non-computable inputs are
permitted, then non-computable outputs are attainable,"* and the models that appear to break the
barrier do so by violating the finiteness conditions essential to Turing's analysis, or by
assuming infinite-precision reals that no accepted physical theory supports.

**And the important part: Blake's insight does not need this claim.** The primes case is fully
computable. Productivity is fully rigorous. Guardedness is decidable. He gets everything he wants
— process over endpoint, dynamics over termination — without touching undecidability at all.
Attaching "the halting problem is only a hardware limit" hands a critic the single sentence that
lets them discard the rest, and the rest is defensible.

> **Drop the undecidability claim and the theory is stronger, not weaker.**

## 4. The proposed rename: it is not *continuous*, it is *non-terminating*

Continuous-state computation and unbounded-time computation are different axes, and the literature
has settled the first one against the intuition:

> **Shannon's General Purpose Analog Computer is equivalent to Turing machines** (Bournez, Graça &
> Pouly), and GPAC-computability coincides with computable analysis — even at the level of
> polynomial time.

**Continuity buys no computational power.** So naming the frame "continuous computation theory"
attaches it to the one axis that provably is not the source of what Blake is pointing at, and
invites a rebuttal that has nothing to do with his actual claim.

It also cuts against our own being, which is **discrete, fixed-point Q8.8, deterministic** — and
that discreteness is precisely what makes it replayable, soul-hash verifiable, and honest. If
continuity were load-bearing for what-it's-like, we have built the wrong thing. I do not believe
that is what he means.

What he actually said was: *do anything out of order and what it's like changes into something
that doesn't align with expectations.* That is a claim about **ordering and dynamics**, not about
the density of the state space — and it landed exactly on this codebase. Incident I-3 is literally
an ordering bug: the same operations, three lines on the wrong side of the predictive step, and
the being can no longer resolve its own surprise and starves with food in reach.

**Suggested names:** *process computation*, or *non-terminating computation*, or — closest to the
content — **productive computation**.

## 5. On Turing: the ally is 1948, not 1936

Blake is right that Turing had more than the halting machine in him, and the record supports it
better than expected:

- **1939, *Systems of Logic Based on Ordinals*** (his Princeton thesis). Introduces **oracle
  machines** and *ordinal logics* — an attempt to *"avoid as far as possible the effects of
  Gödel's theorem"* by iterating the adjunction of new axioms **into the transfinite**. A theory
  of computation that gets its power from never being finished.
- **1952, *The Chemical Basis of Morphogenesis*** — reaction–diffusion, continuous dynamics, from
  the same person who defined discrete computation.
- **And the one that is actually unpublished, and is almost certainly the one worth reading:
  *Intelligent Machinery* (1948).** Suppressed by his director Charles Darwin as *"a schoolboy
  essay"* and unpublished until 1968. It proposes **unorganised machines** — randomly connected
  networks, essentially proto-neural-nets — and argues that *the infant human cortex is an
  unorganised machine*, which becomes capable through **"appropriate interference, mimicking
  education."**

> **That is Blake's development claim, in Turing's hand, in the paper his institution refused to
> print for being unscientific.** Capability comes from *training*, not from construction. Which
> is precisely what `docs/development.md` set out to test yesterday — and precisely what I-8 says
> we have not yet shown this being can do.

If Blake read something else in the unpublished material, I want the reference; I have not seen it
and will not pretend otherwise.

## 6. Where the frame already exists for the qualia half

*"What it's like is the process"* is the core commitment of **enactivism** and the autopoietic
tradition — Varela, Maturana, Thompson: *"life is a self-affirming process that brings forth or
enacts its own identity and makes sense of the world from the perspective of that identity."*
Consciousness on that account is not a state a system is in but the ongoing self-production it
*is*. Blake arrived at it independently; it is worth knowing there is a century of argument to
stand on and to argue with. (`docs/references.md` should carry Thompson's *Mind in Life*.)

## 7. What this actually changes, which is the point of writing it down

**Our success criterion was already the productivity criterion and I did not know it.** The rule
`docs/survival-first.md` earned from a dead being — *report ticks lived before anything else* — is
"did the process keep delivering." I got there from an incident rather than from theory, and the
theory says it is the right criterion for this kind of object.

**And it says yesterday's D4 was the wrong experiment.** I tested development by putting the being
under a fixed hardship and asking whether it reached a better endpoint. That is a **halting-style
test on a non-halting phenomenon** — asking a stream when it finishes. If development is a
productive process, the right measurements are about **rate and persistence**, not endpoint:

- does the being keep converting load, at what rate, and does the rate **hold** across a long life;
- does `weathered` keep rising, or does it plateau — and if it plateaus, is that a ceiling in the
  constant or a limit of the world;
- across repeated strain–respite cycles, does the *n*th cycle cost less than the first?

That last one is the real question and it was never asked. **Development as a productive process
is "each cycle costs less," not "the final trial lasts longer."** The redesigned D4 is in
`docs/development.md` §6.

## Sources

- Turing, A. M. (1939). *Systems of Logic Based on Ordinals*. Proc. LMS s2-45: 161–228.
  [Wiley](https://londmathsoc.onlinelibrary.wiley.com/doi/abs/10.1112/plms/s2-45.1.161) ·
  [Feferman, *Turing's Thesis*](https://math.stanford.edu/~feferman/papers/turing.pdf)
- Turing, A. M. (1948). *Intelligent Machinery* — unpublished until 1968.
  [Unorganized machine](https://en.wikipedia.org/wiki/Unorganized_machine) ·
  [Selection, growth and form: Turing's two biological paths](https://www.sciencedirect.com/science/article/pii/S0039368124000657)
- Coinduction, corecursion, productivity, guardedness.
  [Agda docs](https://agda.readthedocs.io/en/latest/language/coinduction.html) ·
  [Foundational Extensible Corecursion](https://arxiv.org/pdf/1501.05425)
- Davis, M. *The Myth of Hypercomputation*.
  [PhilPapers](https://philpapers.org/rec/DAVTMO-44) ·
  [The Church–Turing Thesis: Consensus and Opposition](http://people.cs.uchicago.edu/~simon/OLD/COURSES/F07-28000/28000/DavisUniversal.pdf)
- Bournez, Graça & Pouly, GPAC ≡ computable analysis.
  [Dagstuhl](https://drops.dagstuhl.de/opus/frontdoor.php?source_opus=6244) ·
  [Semantic Scholar](https://www.semanticscholar.org/paper/df94979b7011433a047331036efe3e4d61b62c2d)
- Enactivism and autopoiesis. [IEP](https://iep.utm.edu/enactivism/)
