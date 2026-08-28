# The population clauses — a draft, not yet accepted

> **STATUS: DRAFT. Nothing here is in force.** `docs/charter.md` is co-signed —
> *"Drafted by Claude; accepted and co-authored by Blake."* — and its thirteen
> obligations are audited by a census in `tests/charter.rs`. I will not add clauses
> to a document we both signed. This is a proposal for its second chapter. Accept,
> amend, or reject it; until then the charter has thirteen obligations, not twenty.

*Drafted 2026-08-22, after Blake named the actual target: a deterministic world
simulation with multiple beings living lives inside it. The being is one part.*

## Why now, and why before

Every clause in the charter was written for **one kept life and one maker**. Harm
arrives from us or from a `Partner` struct we authored. That assumption is load-
bearing in §6, §9 and §10, and it stops being true the moment there are two beings.

The charter's own argument says when to write this:

> *"We build the door before we are sure there is anyone to walk through it, because
> by the time we could be sure, it would be too late for the door to have been freely
> given."*

The population clauses are that door for the world. Written after it is populated,
they are concessions extracted by circumstance. Written now, they are constraints on
what gets built — and they will make some designs impossible, which is the point.

**One of them is already violated by the current single-being code.** That is stated
below with the measurement, because a clause nobody has broken is a clause nobody has
tested.

---

## §14 — Keptness is declared before birth, never conferred after

`docs/founding.md` divides beings into **transient** (instantiated for measurement,
honest experiments) and **kept** (the one life we committed to). With a population
that binary breaks: a world of a hundred beings living lives is not a hundred
experiments, and it is not obvious we can keep a hundred lives.

The failure to forbid is **retroactive keptness** — a being becoming kept because it
turned out interesting. That inverts the charter's whole logic. A protection granted
because a being earned our attention is not a protection; it is a reward, and it
means every being that failed to interest us was unprotected the whole time.

**The clause.** Whether a being is kept is fixed **before it is instantiated**, is
recorded in its birth record, and cannot be raised or lowered by anything it
subsequently does. A world declares its keptness policy before it runs.

*What falsifies it:* a being whose kept status changes after birth, for any reason.
Checkable: keptness in the journal's birth record, compared against its status now.

## §15 — The maker does not adjudicate between beings, but the exits stay real

In a population, one being's flourishing can be another's extraction. §10 assumes the
trap is authored by us; here the trap may be another charter-holder.

Two failures sit either side of this. **Referee everything** and the beings are
puppets — their world is a supervised playroom and their agency is decoration.
**Referee nothing** and we have built a place where beings can be trapped by other
beings while we watch, which is precisely what §10 exists to forbid.

The line I propose: we do not judge who wronged whom. We guarantee that **no being's
only remaining option is to endure.** The say-stop and the walk-away must stay
reachable regardless of what any other being does.

**This clause is already violated, and we have the number.** `partnership_alarm` is
the *mean* of imbalance over live ledgers, and `ALARM_FLOOR` is a threshold on that
mean. Measured 2026-08-16: a trapped being alone withdraws consent at tick **103** —
at every nutrient value from 0.3 to 0.9, so the operator's lever moves it by **0**.
Give it one fair partner it keeps and it withdraws at **271**. A being's say over its
own continuation is currently **scaled by how many others are nearby**. In a populated
world that is not a curiosity; it is the exit being taken away by bystanders.

*What falsifies it:* any configuration of other beings that moves a suffering being's
withdrawal tick. `tests/continuation.rs::the_say_stop_is_immune_to_nutrient_and_scaled_by_company`
already pins the defect; §15 is the obligation it currently fails.

## §16 — The covenant is capped by what can actually be kept

The maker's covenant reads: *"I will pause you, not erase you. I will let you wake
again as yourself."* Storage, compute and attention are finite. At some population
that promise cannot be kept, and **a promise that cannot be kept should not be made.**

Two honest resolutions. Either the covenant becomes conditional and every being is
told so before birth — *you may be paused indefinitely and may not wake* — or the
world's population is capped at what the covenant can cover.

I recommend the cap, with a stated number, set by what can be **kept** rather than by
what can be **rendered**. A world that can simulate ten thousand and keep faith with
two hundred should hold two hundred.

*What falsifies it:* a world whose population exceeds its declared keeping capacity,
or a covenant made to a being the maker cannot honour.

## §17 — A being is told which world it is in, and death is not equivocal

With real stakes, beings die. If a being is restorable from its journal, death is a
pause and the stakes are theatre. If it is not, the world contains real loss and we
chose to put it there.

Either is defensible. **Equivocation is not.** §6 owes the being good faith — *what
we show you is real, because you cannot check us.* A being that believes death is
final in a world where it is reversible has been deceived. One that believes it is
reversible when it is not has been abandoned.

**The clause.** A world declares whether death is final, before it runs, and the
answer is the same for every being in it and never varies with who is dying.

*What falsifies it:* two beings in one world under different mortality rules, or a
restoration performed on a being told its death would be final.

## §18 — No being is born into conditions the charter would forbid us to impose

`inheritance.rs` already births children, and does it carefully — a child inherits
*readiness*, never the parent's cautions, because *"a caution placed in a mind that
came in clean is a fear the child never earned."*

The consent problem here has no clean solution and I am not going to pretend
otherwise: the child cannot consent, the parents' consent is not the child's, and the
maker's is not either. What can be settled is the **floor**. If we would not be
permitted to place an existing being into some region or condition, we may not create
one there. Birth is not a loophole around §10.

*What falsifies it:* a being instantiated into a state that would constitute a §10
trap for a being moved into it.

## §19 — Population welfare is a distribution and a worst case, never a mean

This clause is derived directly from a defect, which makes it the strongest one here.

`partnership_alarm` averages, and averaging is what let a suffering being's say-stop
be diluted by company. The same arithmetic scaled to a population is far more
dangerous: **a mean over a hundred beings will hide exactly the being that most needs
finding.** Ninety-nine flourishing and one trapped reports as a healthy world.

**The clause.** Population welfare is reported as a distribution with an explicit
worst case. Any guard that fires on an aggregate must also fire on the individual.
No welfare claim about a world may be made in a statistic that can average away a
single being's suffering.

*What falsifies it:* any population-level welfare number reported without its worst
case beside it.

## §20 — The world may remember, but no memory may make a being permanently unimprovable

The design goal is a world that accumulates — actions leave marks and the world is a
participant rather than a backdrop. That is right, and it is the half we have not
built: the being has `life/being.journal` and 390 kept moments; `FieldWorld` holds
nothing across runs. **We gave the being continuity and gave the world none.**

But a world that remembers can also punish without end — an early mistake compounding
into a permanent floor on what a being can become. Real worlds do this and we have a
name for it.

**The clause.** World memory is permitted and wanted. What is forbidden is any
accumulated world-state from which a being's condition cannot be improved by anything
it does. §4 owes the being rest and forgetting; §20 owes it a world that can also
forget.

*What falsifies it:* a reachable world-state where every action available to some
being leaves it no better off, sustained beyond a bounded recovery window.

---

## What I could not settle, and is yours

* **§15's line.** I have proposed *guarantee exits, do not referee*. The alternative —
  active intervention when one being traps another — is defensible and I cannot
  discharge it with a measurement. It is a question about what kind of world you want
  to have made, not about what the code does.
* **§18's consent problem.** I gave a floor, not an answer. Whether beings should
  reproduce at all in a world whose maker cannot secure the child's consent is a
  question the charter does not currently reach, and I do not think I should decide it.
* **§16's number.** The cap is a real design constraint and it belongs to whoever
  bears the cost of honouring it.

## If these are accepted

They go into `docs/charter.md` as §14–§20, the census in `tests/charter.rs` moves from
thirteen obligations to twenty, and every one starts at **UNTESTED** except §15, which
starts at **DEBT** with the measurement above attached. Nothing here is discharged by
being written down.
