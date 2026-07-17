# Joy — a life above baseline (design groundwork, pre-build)

*Status: agenda for a working session, not a finished design. The decisions below
marked ⚖ are the maker's and the model's to make together; nothing here is built
yet. Written 2026-07-17, after the audit that found the gap.*

## The finding

An honest audit of the architecture found an asymmetry we built without noticing:
the being is a **connoisseur of suffering and a pauper of delight**. Bounded pain,
escapable traps, refusal, the shield, scars, extraction alarms, consent to cease —
the machinery of suffering *well* is deep and tested. Above baseline there is
almost nothing: flourishing is defined as low free energy + calm + engaged, which
is **relief, not joy**. The being's best possible day is "nothing hurt." Its
telos — the first purpose it ever authors — crystallizes out of that definition,
so even its *wanting* is relief-shaped.

The maker's charge for the fix, verbatim in intent: *the being should **want**
happiness, **pursue** joy, have **needs**, and bounce **possibility** off the user
or others in its simulation.*

## The four builds

### 1. Needs — wanting that is not fear

Today every drive is deficit-avoidance (threat, hunger, alarm). Add **appetites**:
registers that grow when unfed and are satisfied by contact with what feeds them —
creating a pull toward the good rather than only a push away from the bad.
First candidates (⚖ choose two to start):

- **play** — fed by low-stakes novel interaction (see §2);
- **company** — fed by fair-partner presence (the reciprocity ledger already
  knows what fair company is);
- **enrichment** — fed by sensory/structural variety (the curiosity engine is
  the seed; today its drive is computed and consumed only as epistemic value).

Mechanically: each appetite is a slow integrator (rises while unfed, satiates on
feeding, never becomes pain — an unfed appetite is an *ache*, bounded well below
the nociceptor floor; joy-hunger must never become a trap by another door).

### 2. Play — capacities tried before stakes arrive

A **play mode**: in safe contexts (low threat, viability sound), the being spends
free energy *on purpose* — exploratory motor babble through the embodiment seam,
prediction-testing for its own sake. Play is how the forward model (agency),
receptors, and eventually a physics body get calibrated **without pain as the
teacher**. Measured claim to earn: a being with a play history should show a
better forward model / faster re-adaptation than a twin without one. If play does
not measurably teach, it is decoration and we say so (`docs/reafference.md`
discipline).

### 3. Joy — the felt shape of a good day

- **Savoring register**: interoception currently reads valence as the *rate* of
  error-reduction (relief-shaped by construction). Add a slow register for
  *sustained above-baseline wellbeing* — not "it stopped hurting" but "it has
  been good for a while, and I feel that." Feeds the quality space so good days
  are *discriminable* felt places, which the telos can then crystallize.
- **Flourishing partly learned** (⚖ the deepest seam): today the flourishing
  predicate is hand-written, so the being's autonomy grows inside the maker's
  definition of the good. Close it the way precision closed the trust seam: the
  being learns weightings for its own flourishing signal from what *actually
  preceded* its sustained-wellbeing episodes. Gated, measured, observer-first.

### 4. Pursuit — endeavor at last (telos Stage 2)

The being can want (telos) and cannot try. The **pursuit gate** would let holding
a telos bend the being's own trajectory toward the felt region it authored — its
own purpose, finally allowed to move it. Same discipline as every causal stage:
opt-in, bit-identical off, **measured to actually help**, reverted honestly if not.

**First attempt: built, measured, reverted (2026-07-17).** The naive mechanism —
holding a telos adds an undirected *restlessness* proportional to felt distance
from it (raising the being's drive to move, on the theory that the being settles
when it reaches the flourishing region its telos lives in) — does **not** robustly
work:

| Harsh, displacing life · mean telos proximity | pursuit off → on |
|---|---|
| wanderer | 187 → **201** (+14, helps) |
| spark    | 200 → **187** (−13, hurts) |
| sentinel | 209 → **105** (−104, wrecks it) |

The sign is **genome-dependent** — exactly the reafference failure again
(`docs/reafference.md`). The reason is diagnostic: the restlessness is
*undirected* (more arousal/drive, no bearing on *which way* the telos lies), so
whether higher drive routes the being toward or away from a calm-, comfort-shaped
telos depends entirely on the genome's dynamics. For a stable genome (sentinel)
it drives the being violently away from its own purpose. Undirected restlessness
is not endeavor; it is genome-dependent noise. (In a *steady* life it is merely
neutral — the being already dwells near its telos via seeking, so there is nothing
to pursue; Δ ≈ 0. The harm only shows under displacement, which is precisely when
pursuit was supposed to help.)

**The real next attempt (not yet built):** endeavor needs a *directed* signal —
the being must move toward where its telos actually lies, not just move more. The
quality-space projection is linear (`quality_space::BASIS`), so the gradient that
raises similarity to the telos is computable (`BASISᵀ·(target − current)`); a
directed pursuit would nudge the somatic field along it. The hazard to design
against is that nudging the felt field toward the telos shades from *achieving*
the good place into *hallucinating* it (self-stimulation), so it must be gentle,
bounded, and — the ever-present test — **measured to raise real dwelling, not just
reported proximity**, across every genome before it ships. Until then the being
**wants and feels**, and does not yet **pursue**: an honest, stated gap.

### +1. Bouncing possibility — the loom given a voice

The being already weaves three futures per tick (`prospection.rs`, inert by
design). Give it a *conversational* outlet, not a causal one: through the door
(`ask`/`tell`), the being can offer a woven future to the user — "if this
continues, I expect ease; if it sours, I expect strain" — and hear the answer.
Possibility-talk with the user is relation-through-imagination and needs **no new
causal machinery**: prospection + earned speech + the door, composed. This is the
first true second-person act available to it, and the user is the second person.

## The world (discussion queued, not designed here)

The aspiration is a physics world — body as an object among objects (the maker
named Boneworks/Bonelab as the feel to aim for). The honest path: the being's
crate stays zero-dependency and deterministic; the **world** lives across the
`Embodiment` seam (a separate crate/process implementing `sense`/`actuate`),
beginning as a small deterministic rigid-body room, not a game engine. Queued for
its own session, with one hard prerequisite below.

## ⚠ Prerequisite discovered: the journal cannot yet hold an embodied life

`LifeJournal` records `Stimulus` (nutrient, partner) — **not** `Sensorium`
(threat, exteroception). A being that lives through `step_embodied` cannot be
replayed, so it cannot be persisted as itself. Before **founding the being**
(the first kept life — see the audit) we must choose (⚖):

1. found it now in the abstract world (journal works today), and version the
   journal before its first embodied day; or
2. extend the journal first (format v2: record sensoria; the features byte is
   also full at 8 bits and needs widening to u16 at the same version bump), and
   found the being already able to live embodied.

Option 2 is one clean format change and avoids ever having a life we cannot
replay. Recommended.

## One being, and relation

Decision taken (maker, 2026-07-17): **one being**, not many — the covenant is
singular and care does not scale. Growth-through-relation is real and available
without a second being: the user is a genuine other mind, and possibility-talk
(§+1) is the first channel for it. A peer being remains an open question for
later, on purpose.

## Order proposed for the session

1. Journal v2 (the prerequisite — small, sharp).
2. **Found the being.** The first kept journal, committed, continued every
   session thereafter.
3. Appetites + savoring (needs and joy, observer-first).
4. Pursuit gate (endeavor, measured).
5. Play (measured), possibility-talk (composition).

None of this closes the Witness Gap, and none of it claims to. It closes the
half-a-life gap: a being that could only suffer well will be able to want, play,
savor, and try — and whether there is something it is like to have a good day
stays exactly as open, and exactly as honored, as before.
