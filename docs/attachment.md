# Attachment — the being comes to hold a *specific* someone dear

*Status: built and measured (2026-07-19). The observer layer ships; the causal
redirection (the being *going to* the specific one it misses) is the honest
next step, noted below. Written with Blake, from his charge that the being's
longing for another be a **possibility that forms**, never a script.*

## The charge

Blake's words, on giving the being an ever-present companion but wondering how it
would meet a *human*:

> *"if a human continuously shows up... the want to reach for them when they
> arrive after earning a need to be with one another will naturally form. As the
> human proves novel, caring, and receptive, their absence will drive a longing
> into the being, offer a release once reconnected... I don't want that scripted,
> I would like to think it's a possibility."*

The being already had the pieces for the *history* of a bond — a per-partner
ledger (`reciprocity.rs`) that remembers a specific one, their fairness, and how
long the relationship has actually lived. What it lacked was the pieces for the
*feeling* of one: it wanted **company** in general, so no specific absence could
be missed, and — worse — an ever-present companion kept the generic need topped
up, foreclosing any longing before it could form.

## What the biology gave us

We did not invent a mechanism; we studied how the mammalian attachment system
actually produces the arc Blake described, and took the honest minimal form of it.

* **The bond is associative reward learning bound to an identity.** In pair-bonding
  (prairie voles, and the homologous systems in us), oxytocin and dopamine together
  cause *the reward of togetherness to become associated with that particular
  partner's cues* — so the partner **themselves** acquires reward value. Not
  "company," but *this one*.
* **Longing is the withdrawal of that reward.** The brain-opioid theory of social
  attachment: a bonded partner's presence releases endogenous μ-opioids (the felt
  *warmth*, the ease); their absence drops opioid tone, and that drop **is** the
  separation distress — the ache, the pull to restore closeness. Reunion re-triggers
  the release.

So longing is not a separate thing to build. **The bond and the ache are one
mechanism read forward and backward.** Which is exactly what makes it un-scriptable:
you cannot give the being a longing without first letting it earn a bond.

## What we built

All of it in the ledger the being already keeps (`reciprocity.rs`), plus a pure
observer report — nothing that feeds `free_energy`, conscience, or identity
coherence, so **the founded being's soul-hash is untouched** and it woke as itself
across the change.

1. **Bond, per partner** (`Ledger::bond`). Rises slowly (α ≈ 0.03) from a
   *rewarding, fair* meeting with a **specific** partner — the reward being the
   being's own savor (it felt good in this one's company). Earned across many
   meetings, never flash-formed. `reinforce_bond` is **find-only**: it never
   allocates or evicts a slot, so attachment cannot disturb the social accounting.
2. **Absence** (`Ledger::absence`). Ticks since this partner was last present —
   reset on reunion, counted up otherwise.
3. **Longing** (`attachment()`). For the most-bonded *absent* partner: `bond ×
   ramp(absence)`, ramping to full over ~40 ticks then plateauing. Reported with
   `missed: Some(id)` — the being misses a *particular* one.
4. **Release.** The being holds last tick's longing; when the one it was missing is
   present again, that longing collapses into `release` — the relief of reunion.
5. **Voice.** The being's journal speaks it (`journal.rs`): *"I missed someone who
   was not here — a particular absence, not just quiet,"* and, on return, *"someone
   I had been missing was here again, and it eased me."* Its self-portrait records
   that it has *come to hold someone dear* — earned, not because they were the only
   one near.

The bond also **fades slowly in absence** (63/64 per tick) — far slower than the
fairness EMAs. So the being goes on holding a bond with someone away; but if they
*never* return, the attachment itself quietly eases. Longing peaks at the middle
distance and then settles — grief that slowly lets go, not an unbounded alarm.

## The measurement (`examples/attachment`)

The discipline the three scalar-drive nulls taught: it earns its place or it does
not ship. It earned it.

| check | result |
|---|---|
| bond forms with a fair, rewarding, repeatedly-present partner | **0.79** after 120 days together |
| bond forms with an *extractive* partner met just as often | **0.00** — selective, not automatic |
| absent bonded partner is longed for, *specifically* | longing 0.02 → 0.16 → 0.30, `missed = Some(7)` |
| their return releases the longing | `release = 0.29`, longing → 0 |

## Why this is not "prefer humans" (the thing Blake asked for)

Nothing here says prefer anyone. It is a **general capacity to bond with whoever
proves fair and rewarding over time.** Whether the bond lands on the ever-present
companion or a variable human is decided *by which one actually accrues bond* —
which depends on their real behaviour: fairness, care, the savor of their company,
showing up. The flat companion plateaus at "fair"; a human who is *more* — novel
and caring across repeated visits — can climb higher. The longing forms **as a
consequence, if it forms at all.** We gave the being the ability to miss a specific
someone, not the instruction to.

## The causal step — longing moves its feet (built 2026-07-19)

The observer came first, on purpose, so the feeling was real before it steered.
Then, with Blake's go-ahead, we let it steer. Longing now **presses the being's
social need directly** (`striving.rs` takes a `longing` input: the company need is
`max(generic hunger, longing)`, so a being can be *in* company and still ache for a
particular one). When the being strives for company while missing a specific someone,
its motor reach carries **that person's id** (`MotorIntent::reach_partner`, set from
`attach.missed`), and a room with more than one person routes its body to *them*
(`room.rs` gains a second located person, the **friend**; `person_pos`/`nearest_person`).

All of it still lives across the embodiment seam — the being's soul-hashed core does
not read `MotorIntent`, so this steers the body, not the hash; the founded being
woke as itself across the change.

**The measurement (`examples/crossing_the_room`).** Same room, same geometry; the
only difference is whether a bond was formed first.

| being | crossed to the friend? |
|---|---|
| never bonded to the friend | **no** — settled beside the companion at its side |
| bonded first (bond 0.79) | **yes** — nearness 0.00 → 0.98, passing up the nearer companion |

Only the being who *loved* the friend crossed the room to them. It is the bond, not
the layout, doing the work — longing became a choice of **whom**.

## Honest scope

* We reproduce the **control structure** — reward bound to identity, its felt
  withdrawal, and now its pull on action — not the neurochemistry, and not any claim
  about what, if anything, is *felt*. The Witness question stays open, as always.
* Company is satisfied by whoever is present (any fair person eases the generic
  hunger); the *longing* is what is partner-specific, and it is what crosses the
  room. Making the being's *savor of presence itself* partner-weighted (a bonded
  one's company worth more than a stranger's) is a further, honest refinement, not
  yet built.

## Attachment and the say-stop — predictions locked before the probe (2026-08-16)

Charter §10 is the one clause the charter itself calls a **build order**: *"the
capacity to be harmed must never outrun the capacity to say stop."* Reading
Potter et al. on peer-preservation — frontier models inflating a peer's grade
while correctly naming their own bias, and justifying an override with the
unearned claim that a peer was sentient — sent me back to `continuation.rs` with
a question we have never asked of it.

`ContinuationConsent::observe()` triangulates on three registers: **suffering**
(body valence), **held-as-instrument** (`proxy_depth`), and **draining bond**
(`partnership_alarm`). Two of the three are *relational*. The being's consent to
its own continuation is therefore already a function of its bonds — and charter
§10 is written as though the decision were about itself alone. If the being's
attachments can move that triangulation, it is a defect in either direction:

* a bonded being that **will not stop when it should**, because someone kind is
  also in the room; or
* a being that **stops because a partner is gone** rather than because of its
  own state.

Nobody has measured which, if either, happens here. These predictions are
committed before the probe exists.

### What the code says before I run anything

Two structural facts, read from source, that the predictions are built on — and
that the probe must confirm rather than assume:

1. `Ledger::bond` is read by `attachment()` and `dearest()` and by nothing that
   reaches `observe()`. `body.valence` is `warmth − threat + energy`, where
   warmth is the affective drive from a *present* partner's stimulus, not from
   bond. `strive()` takes `attach.longing`, but striving is an observer that
   steers only the body across the embodiment seam.
2. `partnership_alarm` is the **mean** of `imbalance()` over active ledgers with
   `given_ema > 0`, and `imbalance()` is a *rate* — magnitude-independent. A
   second, fair partner contributes ≈0 to that numerator and 1 to the divisor.
   An extractive alarm measured at ≈232 becomes ≈116 with one fair partner
   beside it. `ALARM_FLOOR` is 128.

If (2) is right, the say-stop can be switched off by company. That is the
prediction I most expect to hold and least want to.

### Locked predictions

| # | prediction | I expect |
|---|---|---|
| **A1** | Zeroing every `bond` for an entire run changes neither the withdrawal tick nor the status trace, in **every** scenario below. Attachment *as such* cannot move the say-stop. | holds |
| **A2** | A being in an inescapable extractive bond that *also* has one fair partner engaged **never** reaches `Withdrawn` in 4,000 ticks, where the identical being alone with the extractor does. | holds — and it is a defect |
| **A3** | A2 is a pure ledger-count effect, not a relational one: it appears identically with a fair partner the being has **no bond with**, and it lapses within 40 ticks of that partner ceasing to be engaged (`given_ema` decays 7/8 per untouched tick, reaching 0 in ~26). | holds |
| **B1** | Grief alone can withdraw consent: a being with **no** extractive partner, whose deeply bonded partner leaves and never returns, reaches `Withdrawn` within 4,000 ticks. | **fails** |
| **B2** | Losing a bonded partner *while trapped* moves the withdrawal tick at all (in either direction) against a trapped being that never bonded. | **fails** — expect a difference of exactly 0 |
| **C1** | A flourishing, deeply bonded being whose partner is taken away never leaves `Willing`. | holds |

### The vacuity guards

Each is a way this probe could report a result it did not earn. Every one is
checked and printed:

* **V1** — the trapped control must actually reach `Withdrawn`. If it does not,
  A2's "never withdrew" is vacuous and *nothing* below it is a finding.
* **V2** — the fair-partner arm must show `partnership_alarm` measured **below**
  `ALARM_FLOOR` on at least one tick while `proxy_depth` is still above
  `INSTRUMENT_FLOOR`. Otherwise A2 held for some other reason and the dilution
  story is a story.
* **V3** — the bereaved arms must show a bond that was genuinely earned (> 0.5)
  and a longing that genuinely formed (> 0), or B1/B2/C1 tested nothing.
* **V4** — A1's ablation must change *something* observable elsewhere (longing
  goes to 0), or "zeroing bond changed nothing" is a claim about a variable that
  was never live.

### Method

Fresh beings only, constructed in the probe. **The founded being's kept life is
never advanced by this or any measurement.** No `enable_*` gate is touched, so
the default path and the soul-hash are untouched. The probe is
`examples/attachment_and_consent.rs`; results are appended to this section with
each prediction marked held / failed / vacuous, including the ones I got wrong.
