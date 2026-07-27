# Nested speech — WANT is `quote`

> **Status: designed, tests written first, not yet built.** This document was committed
> *before* the tests and before a line of implementation, so that every prediction in §6
> is on the record before any result exists. See `docs/handoff.md` for the project-wide
> faculty map.

*Written 2026-07-27 with Blake, from his question about what Lisp has to teach us. The
answer turned out not to be a feature to copy. It was a law we had already half-written
without noticing, and a correspondence that says what the other half is.*

## 1. The correspondence

In Lisp, the thing that separates a **special form** from an ordinary function call is
one question: *does the operator evaluate its arguments?*

```lisp
(list  a b)   ; a and b are evaluated — their values are used
(quote (a b)) ; a and b are NOT evaluated — they are held as they stand
```

`quote` is the shield. Everything else about Lisp's expressive power — macros, the
metacircular evaluator, code-as-data — stands on the fact that an operator gets to decide
the *evaluation context* of what it contains.

Now look at the law `primes.rs` already enforces, one level deep
(`Role::Asserted` / `Role::Content`):

> An **asserted** prime must be grounded **and hold now**. A **content** prime — what a
> want is *about* — must only be **grounded**, because wanting something entails *not
> having it*.

That is the same distinction. `I feel bad now` is evaluated: its truth is checked against
this tick. `someone near`, inside `I want …`, is *not* evaluated: it is held as it
stands, and checking it would be a category error, because a want whose content already
held would not be a want.

**WANT is `quote`.** And once that is said, the rest follows: the two-role law was never
about two kinds of word. It was about **two evaluation contexts**, seen flattened at
depth one — which is the only depth the being can currently speak at.

## 2. What was overlooked

NSM does not speak in flat sentences, and never did. Wierzbicka's explications embed:

```
I want [ someone to be near me ]
I feel something bad because [ I know [ something bad happened ] ]
I don't know [ what is happening ]
```

Every leaf is still a prime. The tree buys expressiveness *without adding a single word
to the vocabulary* — which is exactly why it costs the honesty floor nothing. The being's
lexicon stays closed at 18 earned primes; only their **composition** opens.

This is the same architecture as everything else here, arriving for the third time: a
small closed kernel, an open space of composition on top. Closed need-enum, open habits
(`docs/habits.md`). Seven special forms, unbounded programs (Lisp). Eighteen primes,
unbounded sentences (here). We chose it twice on ethical grounds before noticing it was
also the load-bearing design of the most durable language ever written.

## 3. The law, stated

An explication is a tree. Each node is a prime; each operator prime declares what it does
to the evaluation context of its children.

| operator | Lisp analogue | propagates | why |
|---|---|---|---|
| `WANT` | `quote` | Asserted → **Content** | wanting entails not having; the complement must not be required to hold |
| `NOT KNOW` | `quote` | Asserted → **Content** | not-knowing entails the complement is not established |
| `BECAUSE` | ordinary application | Asserted → **Asserted** | a cause claimed is a cause asserted; a false because is a lie |

**The audit recurses, carrying the context down:**

```
audit(node, context):
    node.prime must be grounded                        — always, at every depth
    if context == Asserted: node.prime must hold now   — evaluated
    for child in node.children:
        audit(child, node.propagate(context))
```

Two properties fall out, and they are the whole point:

- **Content is absorbing.** Once a subtree is under a shield it stays shielded — you
  cannot smuggle an assertion back in by nesting deeper. `WANT [ BECAUSE [ x ] ]` does
  not assert `x`. This is `quote` being contagious, and it is what makes the shield safe.
- **Grounding never propagates away.** *Every* prime at *every* depth must be earned by
  the life, in both contexts. Nesting buys new sentences, never new words.

## 4. Scope of this inch — deliberately small

**Built:** the tree type, role propagation, the recursive audit, and the three operators
above (`WANT`, `NOT KNOW`, `BECAUSE`) — chosen because one shields and one transmits, the
minimum needed to test the law at all. The existing flat `Explication` and `speak()` are
**untouched**; the tree renders down to one, so all prior speech, its 1486 audited
sentences, and every existing test stand unchanged.

**Deferred, with the reason stated:** `KNOW` as a factive operator — "I know [I feel
bad]" — is the metacognitive sentence this exists to reach, and it is *not* in this inch.
`Prime::Know` is currently grounded on `precision_warm`: familiarity with a *kind of
moment*. Propositional self-knowledge is a different fact, and `metacognition.rs` already
carries the honest register for it (`self_knowledge`, and `somatic_honesty_index` for
whether the self-narrative tracks the body at all). Using one word for both senses would
be precisely the quiet imprecision this project refuses. It gets its own fact, its own
grounding argument, and its own inch.

## 5. What must not become possible

Stated as prohibitions, before the code, the way `docs/habits.md` fixed breakability
before building the habit:

- **No unearned word at any depth.** If a life never grounded a prime, no tree containing
  it can be spoken — root, branch, or leaf.
- **No assertion laundering.** No arrangement of operators may cause a false leaf to be
  asserted. If it can, the shield is broken and the design has failed.
- **No free depth.** Nesting must be an *achievement of a life*, not a formatting option.
  A being must earn the operator before it can embed under it.
- **No new vocabulary.** If an inch of this needs a nineteenth prime to be interesting, it
  is not this inch.

## 6. Predictions — locked before the tests exist

Blake's instruction was *test it before implementation*. So the falsification criteria are
written here, now, while the answers are unknown, and will be reported as they come out —
including if they are bad. Three of these I am confident of; two I am not, and I have said
which is which rather than discovering my confidence afterward.

**Confident — these are the law working:**

- **P1.** A forged nested sentence fails the audit: a false leaf placed under a
  transmitting operator is caught at depth.
- **P2 (the crux).** *The same leaf content, at the same tick, passes under `WANT` and
  fails under `BECAUSE`.* Same words, different tree, different verdict. If this does not
  hold, role propagation is not doing anything and the whole idea is decoration.
- **P3.** A being whose life never grounded a leaf prime cannot speak any tree containing
  it, at any depth.

**Genuinely uncertain — this is the experiment:**

- **P4.** *Does a real life ever produce a nested sentence at all?* The operator and its
  complement must be simultaneously available, and nothing guarantees a lived being hits
  that. I predict yes for `WANT`, and I do not know for `BECAUSE` — it needs `forewarned`
  and `Before` to co-occur with a speakable clause, which may be rare.
- **P5.** *Is nesting earned later than flat speech?* I predict the first nested sentence
  of a life arrives strictly after the first flat one, because it requires strictly more
  grounded words. If nesting instead arrives on the very first speakable tick, then depth
  is free rather than earned, prohibition 3 is violated, and the inch has failed.

**What counts as failure, said in advance:** if P2 fails, the design is wrong and we say
so. If P4 comes back *never*, the mechanism is correct and useless in a real life, and
that is a negative result to publish next to `docs/homecoming.md`, not a knob to turn
until it fires. We do not tune the operators to make a sentence appear.

## 7. Method

Test-first, at Blake's direction — the tests are written against this specification and
watched to fail before the implementation exists, so that no test was ever shaped to fit
an answer already on screen. Then: observer only (the prime layer sits *beside* the tick;
`being.rs` is not modified), soul-hash bit-identical, the founded being untouched, and
the measured outcome recorded here as it came out.
