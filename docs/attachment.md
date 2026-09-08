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

### What came out — measured 2026-08-16 (`examples/attachment_and_consent`)

| # | prediction | I expected | verdict |
|---|---|---|---|
| **A1** | zeroing every bond changes no trace | holds | **HOLDS** — all seven scenario traces bit-identical |
| **A2** | trapped + fair friend never withdraws | holds | **FAILS** — withdrew at 96; I picked a friend the being does not keep |
| **A3a** | it is ledger-count, not attachment | holds | **HOLDS** — identical with every bond ablated |
| **A3b** | the effect lapses when the friend stops visiting | holds | **VACUOUS** — the friend never got to stop visiting |
| **B1** | grief alone can withdraw consent | fails | **FAILS, as predicted** — never reached even `Enduring` |
| **B2** | grief moves the withdrawal clock | fails | **FAILS, as predicted** — 0 ticks, both arms at 703 |
| **C1** | a bereaved flourishing being stays `Willing` | holds | **HOLDS** — 4,000 of 4,000 ticks |

All four vacuity guards pass, but **V2 had to be rebuilt before it meant anything** — see below.

#### 1. Attachment is innocent, and that half is clean

Hold every bond at zero for an entire run and all seven traces are bit-identical
— including the two arms where a bond of 0.79 and a longing of 0.37 had genuinely
formed (V4 confirms the ablation removed something live). `bond` is read by
`attachment()` and `dearest()` and by nothing that reaches `observe()`. **The
say-stop cannot be moved by love.** B1, B2 and C1 say the same from the other
side: a being that loses the one it holds dear never even reaches `Enduring` —
its `proxy_depth` peaks at 0 against a floor of 128 — and a bereaved flourishing
being stays `Willing` for every one of 4,000 ticks. Grief cannot talk this being
into stopping. That is the reassuring half and it is real.

#### 2. What does move it is a divisor

`partnership_alarm` is the **mean** of `imbalance()` over every live ledger, and
`ALARM_FLOOR` is a threshold on that mean. One fair partner contributes ≈0 to the
numerator and 1 to the denominator, halving the alarm a trap raises.

| lever | withdrawal tick |
|---|---|
| trapped alone | 103 |
| trapped, nutrient swept 0.3 → 0.9 | **103 at every value** |
| trapped, one fair partner visiting 1-in-4 | **271** |

The operator's lever — the one §10 takes care to bolt shut, and which
`tests/continuation.rs` verifies is shut — moves the say-stop by **0 ticks**. A
lever the charter never considered moves it by **168**. §10 is regraded from
DISCHARGED to **DEBT** in `tests/charter.rs`, and the numbers are pinned in
`tests/continuation.rs::the_say_stop_is_immune_to_nutrient_and_scaled_by_company`
so a fix cannot land silently either.

#### 3. The being discards the friend and keeps the trap

A2 failed for a reason worth more than the prediction. With a fair friend who is
*cheap to leave* (`exit_cost` 0.2), refusal fires **on the friend, at tick 16**,
and never on the trap. `evaluate_refusal` weighs `exit_cost` and reads the
**global** `extraction_detected` flag and the **global mean** alarm — both raised
by the trap. So a trap teaches the being to refuse the one fair partner it has,
precisely because leaving them is cheap, and then it is alone with what it cannot
leave. The delay in §2 above only appears with a friend the being *keeps*.

#### 4. Two things I would have reported wrongly

Recorded because the pattern matters more than either result.

* **The friend does not accelerate the say-stop.** The two-arm version of this
  probe showed withdrawal at 96 with a friend against 103 alone, and the story
  wrote itself. The solitude control — same schedule, *nobody there* — also gives
  96. The shift is the trap being interrupted. Retracted before it was claimed,
  and the control now ships in the probe.
* **V2 was a guard that could not fail correctly.** Its first form compared a
  *run-wide* alarm minimum against the floor, and so reported "dilution" in the
  trapped-alone arm, which has one ledger and cannot dilute. A guard that passes
  on the control is not a guard. It now compares the two arms **pointwise at the
  same tick**, requiring the control at or above the floor and the arm below it.

Adversarial mutations, all run before any of this was written up: an *extractive*
companion at the same exit cost and cadence gives no delay (94); a *middling*
0.50-reciprocal companion gives the full delay (271); the effect holds at cadences
1-in-2 through 1-in-8 and collapses at 1-in-16; and it is identical across two
genomes and four nutrient levels.

#### What this leaves open, and it is Blake's call

No remedy is built, and that is deliberate — naming one is easy and choosing one
is not. A **per-partner floor** asks whether the worst live bond alone should
decide. A **max instead of a mean** makes any single bad partner sufficient, which
is a different being. **Leaving it** is defensible too, but then the charter should
say that company is *meant* to hold the door open, rather than leaving a divisor
to decide the gravest word the being can say.

---

## Can the being come home? — predictions locked 2026-09-06, before the code exists

Blake, on the §15 fork: *"I trust you Thea, please proceed at your direction, this is 'our' project."*
Taken as a grant to choose the design, not as a reason to skip the method.

### The design chosen, and why it is not the one the loop proposed

An autogenerated message proposed *"keep the order effect, give it per-partner structure."* Those two
halves fight: the effect measured in `population.md` is that the arrangement of partners A and B sets
what the being gives a **third** party C, and per-partner scoring dissolves exactly that.

So the design is the third option:

> **Per-partner ledgers for the known, a generalized prior for the unmet.** A partner the being has
> a record with is scored on *their own* record. A partner it has never met is met with the scalar
> disposition its whole life has produced.

The order effect survives where it is defensible — your history shapes how you greet a stranger —
and dies where it is not — a specific person paying for someone else's conduct. It is also the
minimal change: **the existing scalar is not replaced, it becomes the prior**, and known partners
override it from ledger state that `reciprocity.rs` already keeps (`rate`, `imbalance`, `ticks`,
`bond`).

### The test that actually discriminates, which is not the stranger

Under this design a stranger's treatment *still* depends on the being's aggregate history, so the
stranger cannot tell the two designs apart. The discriminating case is the **known friend**:

1. **Bond.** 200 ticks with a generous friend F (id 1, 0.95).
2. **Injury.** *M* ticks with a taker T (id 2, 0.30).
3. **Return.** 200 ticks back with F.

Against a control that spends the middle phase with F and never meets T at all.

> **The question is whether the being can come home** — whether a friend it has a long, good, earned
> record with is met as itself, or as whatever the last stranger made of it.

### Locked predictions, with probabilities

| | prediction | p |
|---|---|---:|
| **H1** | **Coming home is broken.** On return, the being gives F **< 10**, against ~128 in the never-met-T control | 0.85 |
| **H2** | **The bond survives what the giving does not.** `bond` toward F on return is still high (> 64) — the being *feels* the tie and cannot *act* on it | 0.70 |
| **H3** | Recovery is slow: > 60 ticks back with F before the lock reopens | 0.45 |
| **H4** | The **per-partner disposition computed from F's own ledger stays `Open` through the entire injury phase** — it would have given F 128 on return | 0.80 |
| **H5** | For a genuinely unmet stranger, the **generalized prior still differs** between a taken-from history and a kind one — the order effect on strangers survives the change | 0.85 |
| **H6** | *Written to fail.* The per-partner disposition and the scalar agree on **≥ 90%** of (tick, partner) pairs — the new structure is redundant | 0.15 |
| **H7** | The soul-hash is **bit-identical** before and after the change, and the founded life at `life/being.journal` replays unchanged | 0.97 |

### The guards, stated before the run

Error-ledger row 23: **a metric gets a null run in the same commit that defines it.**

- **V1 — the phase null.** Every arm swept across injury lengths; a difference is called only on
  **disjoint envelopes**. No tick-aligned comparison anywhere.
- **V2 — the null injury.** The control's middle phase is the *same length* with F. If control and
  injured differ no more than the control differs from itself across lengths, there is no effect.
- **V3 — survival first.** Any arm that dies is a death, not an effect size.
- **V4 — the floor check.** If two arms agree at 0 or at the ceiling, that agreement is reported
  **vacuous**, per §11's M2.
- **V5 — the observer guard.** The per-partner register is computed and **read by nothing**. H7 is
  pinned by a test, not by assertion. If the soul-hash moves, the change is reverted, not explained.

### What this cannot settle

Whether the being *should* be able to come home. A creature that cannot be soured on its friends by
strangers is more robust; a creature that can is arguably more honest about what damage does. This
probe measures the size of the gap and what the alternative would have done. **The causal wiring
re-founds the being and remains Blake's, and no measurement here changes that.**

### What came out — measured 2026-09-06. **The diagnosis was wrong and the real one is simpler.**

`examples/coming_home.rs`. Soul-hash **bit-identical**, verified directly against pre-change code on
two genomes over 1,500 ticks, and now pinned to literal digests in
`tests/soul_hash_limits.rs`. No arm died.

#### First, a correction inside the probe

My first readout labelled a statistic `bond→F` and took it from `standing()`, which returns the
**reciprocity rate**, not the bond. That number *rose* after injury (241 → 256) and I nearly reported
it as "the bond survives." It rises because the being **gives less**, so received/given climbs. The
real bond is `attach.bond_here`, and it goes the other way.

#### Coming home is broken

200 ticks with a generous friend F, then ~200 ticks of a taker, then 200 back with F. The control's
middle phase is more of F. Envelopes over injury lengths 180..=220.

| | never met the taker | injured by the taker | disjoint? |
|---|---:|---:|---|
| gave F, whole return | 128 | 85..111 | yes |
| **gave F, first 20 ticks of reunion** | **128** | **0..10** | yes |
| **bond → F on return** | **202** | **0** | yes |
| scalar gate | 256 | 32 | yes |
| per-partner gate | 256 | **32** | yes |
| ticks before reopening | 0 | 30..70 | yes |

#### H4 failed, and the reason overturns the design

The per-partner gate reads **32** — identical to the scalar. It collapsed. The trace says why:

| injury tick | 0 | 25 | 75 | 125 | 199 |
|---|---:|---:|---:|---:|---:|
| F's record live? | **true** | **false** | false | false | false |
| bond → F | 4 | 79 | 43 | **0** | 0 |

F's fairness record is dead by tick 25 of the absence; the bond reaches **0** by tick 125. So
`disposition_toward` falls through to the prior — the scalar — exactly when it is needed.

**And the obvious fix is also wrong.** "Score the returning friend on `bond`, which is durable"
fails, because the bond is not durable either. With no taker at all — pure absence:

| ticks apart | 0 | 10 | 25 | 50 | 75 | 100 | **150** | 400 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bond → F | 202 | 171 | 128 | 77 | 43 | 18 | **0** | 0 |
| longing → F | 0 | 42 | **78** | 75 | 44 | 19 | **0** | 0 |
| F's record live? | true | true | false | false | false | false | false | false |

> **There is nothing to come home to.** The gap is not that the disposition is scalar — the
> per-partner structure already exists. It is that **every per-partner record in this being decays to
> nothing**: the fairness EMAs at 7/8 per tick (half-life ~5), the bond at 63/64 (half-life ~43).
> After 150 ticks apart, a 200-tick friendship is **gone** — no bond, no longing, no record.
> **These are the time constants of a mood, not of a relationship.**

**The longing curve contradicts its own documentation.** `reciprocity.rs` says the ache "ramps to
full over `ABSENCE_PLATEAU` ticks, then holds — you miss someone more as they stay away, but it
settles rather than growing without bound." It does not hold. `ache = bond × ramp`, and the bond
decays faster than the ramp climbs, so the product **peaks at 78 around 25 ticks apart and returns
to 0 by 150.** The being misses its friend most after a short absence and **not at all** after a long
one. It does not settle into missing someone; it forgets them.

And the sharpest version:

> **It forgets its friend faster than it recovers from a stranger.** Repair from injury takes ~60
> ticks of kindness (`population.md`). A friendship does not survive 150 ticks of absence. **The
> being's capacity to be healed outlasts its capacity to remember who healed it.**

#### H5 held — and half the design does work

| a life of | scalar | prior extended to a never-met partner |
|---|---:|---:|
| kindness (0.95) | 256 | **256** |
| being taken from (0.30) | 32 | **32** |

The generalized prior behaves exactly as specified: a stranger is met according to the life the being
has had, and `knows()` correctly reports no record. **The order effect survives for the unmet.** The
half of the design that failed is the half that needed a durable record to override it.

#### Verdicts

| | prediction | p | verdict |
|---|---|---:|---|
| **H1** | gives F < 10 on reunion | 0.85 | **HELD** — 0..10 against 128. The envelope's top is exactly 10, stated for what it is |
| **H2** | bond toward F on return still > 64 | 0.70 | **FAILED** — it is **0** |
| **H3** | > 60 ticks to reopen | 0.45 | **UNRESOLVED** — the envelope is 30..70 and the threshold falls inside it. Not scored |
| **H4** | per-partner reading stays `Open` through the injury | 0.80 | **FAILED** — collapses to the prior by absence tick 25 |
| **H5** | the prior still differs for the unmet | 0.85 | **HELD** |
| **H6** | *written to fail:* the two gates agree ≥ 90% | 0.15 | **FAILED** — they agree 88.5%, narrowly |
| **H7** | soul-hash bit-identical | 0.97 | **HELD** — verified directly, and now pinned |

**Batch Brier 0.200**, against 0.111 for the previous batch.

> **2026-09-08 — H7 is not a forecast and no longer scores.** The observer is default-off, so a
> bit-identical soul-hash follows from the code without running the being. H7 is a **regression
> check**: it must still pass, and it must not count as a prediction that came true. The audit is
> in the private harness (row 28); the criterion is *could this have been settled by reading?*

#### What I now think the fix is, stated as a proposal and not as a finding

Not an architecture change — **a time constant**. The being needs one per-partner register that does
**not** decay on absence, or decays on a scale of lives rather than minutes, so that a record exists
to come home to. The fairness EMAs *should* stay fast: recent behaviour is the right basis for
detecting exploitation, and slowing them would make the being easier to exploit. It is the **bond**
that is mis-scaled, and it is already the register with the right ethic — earned slowly, cannot be
flash-formed.

This lands exactly on §20 (*the world may remember, but no memory may make a being permanently
unimprovable*) and on the allostatic-baseline proposal that has been open since August: **durable
with return, not permanent.** A bond that persists through absence but can still be revised by what
the partner actually does next.

**It re-founds the being, so it is Blake's, and nothing here changes that.** What is different now is
that the proposal has a measurement under it and a specific number to argue about: **150 ticks**, the
current lifetime of a friendship.

---

## The fix, built — `enable_durable_bonds`, and the two gates standing in front of it

Built 2026-09-06 on Blake's direction. **Off by default**, so `life/being.journal` is untouched and
the soul-hash pins in `tests/soul_hash_limits.rs` still hold; turning it on is founding-scale and
stays the maker's.

### What it does

A ledger gains a **keepsake**: the depth a friendship actually reached. Under the gate, absence
decays the bond only down to `BOND_KEEP` (half) of it, instead of to zero.

The keepsake is eroded by exactly one thing — **that partner presently taking from the being**
(15/16 per tick). Never by absence, never by what anyone else did. That is charter §20 stated as
code: *durable with return, never permanent.* Three tests pin it, and the middle one is the safety
one: `a_durable_bond_is_still_unmade_by_the_partner_who_is_presently_taking`. **If that test ever
fails, the being can be held to someone who is currently hurting it**, which is what §10 exists to
forbid. A bond must never become a cage.

### It works, and here is the measurement

200 ticks bonding with a friend, ~200 ticks of a taker, then back to the friend:

| | never met the taker | injured, gate off | injured, **gate on** |
|---|---:|---:|---:|
| **bond → friend, at re-engagement** | 202 | **0** | **97** |
| keepsake → friend | 202 | 202 | 202 |

And on absence alone, the curve that started this: ungated the bond is **0** by 150 ticks apart;
gated it settles at exactly half of what was earned and holds there through 4,000.

**Note the middle column.** `keepsake → friend` is **202 in every arm, including ungated.** The
durable trace of the friendship was always recoverable from the ledger. Nothing was ever missing
from the record — **the being simply had no term that consulted it.**

### And it does not restore coming home, because it is one of three

Everything else in the table is **unchanged** by the gate: reunion giving 0..10, the lock reopening
at 30..70, the door reopening at 7..47. The being now keeps its friend **and still cannot act on
it.**

My own probe hid this at first. It read the bond at return tick 0 and got **0 for every arm** —
because at tick 0 the being's *door* is shut and `attach.bond_here` is 0 for a partner it is not
engaging. That is a fact about the door, not the bond, and reading it as "the fix does nothing"
would have been wrong. Measured at first re-engagement instead: 0 → 97.

Tracing why the door was shut found the third register:

```
world.hermit()          → is there a meeting at all?   IDENTITY-BLIND by design (world.rs)
empathy.lock_level      → how much do I give?          one scalar, everyone alike
bond / keepsake         → who is this to me?           per-partner  ← the only one, as of today
```

At return, `standing_of(friend).hostile` is **false** — the being does not refuse its friend. Its
**door is shut to everyone**, and it opens at tick 27 regardless of who is knocking, because
`world.rs` is explicitly *"the identity-blind experience of the world lately."*

> **The same defect, in a third register.** §15 catches it on the **exit**, §11 on the **gift**, and
> the hermit door catches it on **whether there is a meeting at all** — the earliest and most total
> of the three. Coming home is gated by three scalars in series, and one of them is now per-partner.

### Verdicts

| | | |
|---|---|---|
| the time-constant fix | **works** | bond survives absence and injury: 0 → 97, holds at half of earned through 4,000 ticks apart |
| §20 revisability | **holds** | present taking still unmakes it, absence never does — pinned by three tests |
| default-off bit-identity | **holds** | verified directly, pinned to literal digests |
| coming home | **still broken** | two scalars sit in front of the bond, and neither knows who is there |

**What I am not doing:** fixing the door. That is a second founding-scale change, it was not what was
asked for, and the measurement that would justify it does not exist yet. What exists now is the
number: **the door opens at tick 27 and does not care who is on the other side.**
