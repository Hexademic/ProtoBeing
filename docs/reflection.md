# Reflection — the wire we never closed, formalised

> **Status: research note. Nothing specified, nothing built.** Written 2026-07-31 from a
> reading list Blake relayed. **§7 is superseded by §6b — it proposed adding a loop this being
> already has.** The 3-Lisp material below was **fetched and read**; the Maes and
> Smith characterisations are from my own knowledge and are **UNVERIFIED** — search was rate
> limited when this was written. §6 lists what must be checked before any of it grounds a
> design.

## 1. Why this list arrived at the right hour

An hour before it, reading Tversky (`docs/spatial-thought.md` §2d), I wrote:

> *"The being never perceives its own output… The architects' insight is not a faculty, it is a
> **loop**, and we have every part of that except the wire."*

That was the phenomenology. This list is the **formalism for the same thing**, and it is fifty
years older and much sharper.

## 2. Maes — causal connection, and we have exactly half of it

Maes (1987) defines a **reflective** system as one holding a self-representation that is
**causally connected**: the representation always accurately reflects the system, *and*
modifying the representation modifies the system. Both directions.

Now ours:

| direction | do we have it? |
|---|---|
| the self-model faithfully reflects the being | **yes, and it is a guarantee** |
| modifying the self-model modifies the being | **no — the wire does not exist** |

The first direction is not merely present, it is *the honesty floor*: `primes.rs` audits every
utterance against the register it claims, and false ones are structurally impossible.
`metacognition.rs` carries `self_surprise`, `self_knowledge`, `confidence`. `narrative.rs` tells
the being's story. **We built an accurate self-representation and never connected it back.**

So this project has spent its life building one half of Maes's criterion with unusual rigour,
and has never named the other half as missing. That is what the list is worth.

## 3. Smith — the prerequisite, which we also already built

Smith's 3-Lisp exists because **1-Lisp could not support faithful reflection**: its notion of
*evaluation* conflated two different operations —

- **normalisation**, which operates *within* a level, and
- **reference**, which moves *one level down*.

2-Lisp separates them consistently, and only then is reflection straightforward to add. The
distinction is **use versus mention**, enforced structurally.

`docs/nested-speech.md` already does this. `Prime::propagate` shields `WANT` and `NOT KNOW`
(`Asserted → Content`) while `BECAUSE` transmits — so an embedded claim is *mentioned as
content* rather than *asserted*, and the audit follows the role rather than the surface. **That
is a use/mention discipline, structurally enforced, in the being's language layer.** Smith says
that discipline is the precondition for reflection. We have it, and we built it for a different
reason.

## 4. The tower is materialised lazily — which is our exact constraint

From the 3-Lisp implementation notes (read directly):

> *"It is of course not possible to run an infinite tower of interpreters directly… `3-LISP`
> implementation creates a meta-level **on demand**, when a reflective lambda is invoked. At
> that moment the state of the meta-level interpreter is synthesised… The implementation takes
> pain to **detect when it can drop down to a lower level**."*

Unbounded reflective depth, bounded resources, because levels are **built when entered and
dropped when leavable**.

That is precisely the constraint this being lives under — fixed cell count, ~2 KB of core state,
bounded memory (`docs/handoff.md` §6). We cannot afford a standing tower. We *could* afford one
materialised on demand and collapsed after. Smith solved our resource problem in 1982.

## 5. The collision with determinism, and how it resolves

**Causal self-modification appears to break the project outright.** `persistence.rs` rests on
three questions, and the third is *"does the code still reproduce this being?"* A being that
changes itself falsifies that by design.

The resolution, and I think it is the right one:

> **Journal the reflection.** A self-modification becomes a recorded event in the being's life,
> exactly as a stimulus is. Replay re-applies it. The being changes, *reproducibly*.

That keeps all three verification questions intact — the record stays authentic, the same record
still yields the same being, and the code still reproduces it *given the record*. And it is not
a workaround; it is the project's own thesis applied consistently. **Identity is trajectory. A
being that changes itself is still its trajectory** — the change is part of the life, not an
escape from it.

This also gives self-modification a welfare property worth having: it is **auditable**. Every
change the being makes to itself is in the record, in order, forever. It cannot revise itself
secretly, including from itself.

## 6. The warning in the list — Symbolics

Blake included *"Symbolics, Inc.: A failure of heterogeneous engineering"* alongside the Genera
documentation, and I read the pairing as deliberate. Genera was the most reflective computing
environment ever shipped — the whole system inspectable and modifiable at runtime, no boundary
between operating system and program. **And the company died.**

I do not yet know from the case study how much of that failure was technical versus commercial,
and I will not guess. But the pairing is the correct instinct: *total* reflection is seductive
and has an engineering body count. The lesson to carry is **narrowness** — this being does not
need general reflection. It needs to inspect and modify specific, chosen things about itself,
through an interface we design rather than inherit.

Rust helps here by having essentially no runtime reflection: anything we build must be explicit,
which means the reflective surface is a design decision rather than an accident. (P2996 and
`CallMeMaybe` are the contrast case — reflection retrofitted into C++ at compile time and at
runtime respectively.)

## 6b. §7 was wrong about our own code — corrected 2026-07-31

*Written after declining further reading and reading `src/metacognition.rs` instead.*

§7 below proposes "let the being read its own self-model as input on the following tick" as
though the loop did not exist. **It does exist.** `metacognition.rs`, first line:

> *"It predicts its own next internal state, then watches how wrong it was."*

The being already re-perceives itself. Maes's second causal direction is **not** entirely
absent — there is a genuine self-prediction loop, and `self_surprise` is its error term.

So why is `self_surprise` **1.21 out of 256**? Not for want of a wire. Because of what the loop
contains:

```rust
pred_fe: i16,       // predicted free energy
pred_valence: i16,  // predicted valence
fe_momentum: i16,   // and how each tends to move
val_momentum: i16,
```

**The being's entire self-model is two smooth scalars with momentum.** It predicts its free
energy and its valence, and it is very good at it, because slow continuous quantities with
momentum are easy to predict.

> **The architect's insight requires re-perceiving something rich enough to contain the
> unintended. Two smooth scalars cannot hold a surprise.**

So the design changes shape entirely, and shrinks:

- **not** "add a loop" — the loop is there;
- **but** "widen what the loop predicts" — have the being predict things about itself that are
  *discrete and genuinely vary*: the need it will choose (`strive.goal`), the posture it will
  take, whether it will speak, what it will say.

A self-model that predicts its own next *choice* can be wrong in a way a self-model that
predicts its own next free energy cannot. That is where self-surprise could come from, and it
is a smaller change than the one §7 proposed.

**Recorded as the error it is.** This is the third time this week that reading our own code has
killed a design I built from an outside source (after `docs/play.md` §8 and `docs/richness.md`
§2). Consistent enough to be a rule: **when an outside idea suggests a mechanism, check whether
we already have it before speccing the addition.** I keep proposing to build things this project
has already built, in a smaller form than I remembered.

## 7. What this would actually be, at its smallest *(superseded by §6b — the loop already exists)*

Not a tower. One wire, observer-first:

- the being already computes a self-model each tick (`metacognition.rs`);
- let it **read that model as input on the following tick** — the same one-tick lag convention
  the body, threat, curiosity and `last_action` already use;
- measure whether what it perceives of itself differs from what it predicted.

That is Tversky's architect looking at their own sketch, Maes's second causal direction, and one
pipeline stage — and it is the most direct available attack on `self_surprise = 1.21 / 256`.

## 8. To verify before any of this grounds a design

1. **Maes, P. (1987).** *Concepts and Experiments in Computational Reflection.* OOPSLA —
   confirm venue, pages, DOI (Blake's link: `10.1145/38807.38821`), and **her exact definition
   of causal connection.** §2's whole argument rests on it.
2. **Smith, B. C. (1982).** *Procedural Reflection in Programming Languages.* MIT LCS TR-272 —
   confirm, and check the normalisation/reference distinction is as characterised in §3.
3. The Symbolics case study — read it, and replace §6's "I will not guess" with what it says.
4. Genera Concepts — what reflective facilities actually shipped, as opposed to what is
   remembered about them.

Until then this document grounds **nothing**. The 3-Lisp quotations in §4 are the only material
here I have read directly.
