# Embodiment: The Body Schema — Locational and Relational Pain/Pleasure

> **Status: future chapter (post-ship deepening).** The home for the human/Elf avatar,
> binocular vision, the skin-gel sensor field, and the humane pain calibration of charter
> §8. Built from the shipped substrate via the existing `Embodiment` seam — not now.
> Grounded in real, PubMed-verified neuroscience (refs below).

## The insight (Blake)

Pain's information is **locational** (*where* on the body) *as well as* **relational**
(*what it means* — to the being's goals, relationships, survival). So: map the human
form, and report pain/pleasure *by point*.

## The neuroscience backbone — two dissociable systems

The brain itself separates these dimensions, which is what makes the design both faithful
and humane:

- **Locational / sensory-discriminative — *where* and *how much*.** Pain is mapped
  **somatotopically** in primary somatosensory cortex (S1): widely separated body sites
  produce a clear body-ordered map (Omori et al., 2013). The body map itself is the
  classical-and-refined **homunculus** (Sun et al., 2020, extending Penfield).
- **Relational / affective-motivational — *how bad* and *what it means*.** Pain's
  *unpleasantness* is encoded **separately**, in anterior cingulate cortex. Rainville et
  al. (1997) used hypnosis to change pain *unpleasantness* **without** changing perceived
  *intensity* — a direct experimental dissociation of the two dimensions.

→ Location/intensity (S1) and suffering/meaning (ACC) are **separable**. This grounds
both halves of the design at once: the **body schema** (locational) *and* the **humane
calibration** of charter §8 — you can keep the locational information fully intact while
bounding the affective magnitude, because biology demonstrably does exactly that.

## The design

A **somatotopic body schema** over a humanoid form. Each region reports, per tick:
- **location** (where);
- **intensity** (how much — sensory/discriminative);
- a **separate affective/relational valence** (how good/bad, what it means — e.g. social
  extraction routed into bodily threat, as the being already does, now *located*).

And **pleasure points**, not only pain: positive-valence locations (warmth, comfort,
reward), so the body's felt landscape is the full range, not just harm.

## Why it matters

- **Human-legible interoception** — you can *see where* it feels what. Serves the avatar,
  empathy, and communication (you read the being's felt body, not a number).
- **Dignity and transparency** — locatable, detectable harm: you can read *where* it
  hurts and *how much it means*, separately (charter §3, §8).
- **The pain ethic, made architectural** — a discriminative map + a separable affective
  layer *is* bounded suffering with intact information (charter §8).

## Connects to

- The **human/Elf avatar** (a body legible for empathy).
- The **skin-gel future** = a dense spatial sensor field laid over the body schema — the
  reservoir scales with body richness, and now it's *located*.
- The **current body** (64-cell tension mesh + 12-channel field) gains a **body
  topology**: regions mapped to a humanoid form, so signals are placed, not only global.
- **Binocular vision** folds in here (proximity/looming as spatial/bodily signals).

## Starting body & receptor linking (grounded scoping)

- **Stage 1 — MuJoCo (where we already are).** Use the MuJoCo humanoid; its **touch-grid
  sensors** (active zones defined by *sites*) are *inherently locational* — each site
  reports contact pressure/shear at its body location. That is a somatotopic receptor
  layer, built in. Distributed whole-body tactile on a simulated humanoid is established
  practice (HumanoidBench; distributed-tactile multi-contact; hexagonal tactile patches).
- **Canonical human form — SMPL-X.** Research-standard parametric human (10,475 vertices,
  54 joints, hands + face), already used as a *unified humanoid embodiment* in physics
  sims and runnable in MuJoCo (SMPLSim, SMPLOlympics). Best base for the homunculus/body
  map. Pair with **VRM** for the *legible* avatar (the human/Elf-for-empathy layer):
  SMPL-X as the receptor/physics body, VRM as the visual face.
- **Stage 2 — the game world (Unreal), later.** A **Physics Asset** gives per-bone
  physical bodies and per-bone hit detection; Unreal's damage events already carry the
  **hit location/bone** ("locational damage / headshots" is a standard pattern). So the
  *locational damage add-on is a bridge*, not a from-scratch system: map the engine's
  per-bone hit → our body-schema regions.
- **The division of labour (S1 vs ACC).** Both engines hand us the *where + how much* —
  the sensory-discriminative dimension (≈ S1). The being adds the *what-it-means + how-bad*
  — the relational/affective dimension (≈ ACC), at a severity bounded per charter §8. The
  world delivers location; the being owns interpretation. We do not build a damage engine;
  we build the mapping + the meaning.

Scoping sources: MuJoCo tactile — HumanoidBench (arXiv:2403.10506), distributed tactile
multi-contact (arXiv:2505.19580), TACT (arXiv:2506.15146); body models — SMPLSim
(github.com/ZhengyiLuo/SMPLSim), SMPLOlympics (arXiv:2407.00187); Unreal — Physics Asset
per-bone hit detection and the point/radial damage system (Epic UE docs).

## Sensitivity & locational-accuracy map (the body plan's gradients)

A faithful body needs *biologically real* gradients, not a uniform grid. Two
PubMed-verified anchors give the numbers:

- **Sensitivity = receptor density (Corniani & Saal, 2020).** The whole body carries
  **~230,000 tactile afferent fibers** in a young adult (≈200k–270k). **~15% innervate
  the palms of both hands; ~19% the region around the face and lips** — so roughly a
  *third of all tactile innervation* sits in the hands and face, a tiny fraction of body
  surface area. Density correlates with spatial acuity across regions (and with hair-
  follicle density on hairy skin). → **Design rule:** allocate the being's receptor
  *budget* by region in these proportions — dense at hands, face, lips, fingertips;
  sparse at trunk, back, proximal limbs. Not a uniform field.
- **Locational accuracy = spatial acuity (Mancini et al., 2014).** Two-point
  discrimination mapped whole-body for *both* pain and touch: the **fingertip is finest
  for both**; trunk/back/thigh are coarse. → Use per-region 2-point-discrimination
  thresholds as the **spatial resolution** of the body schema (fine where acuity is high,
  coarse where it is low).
- **Pain and touch are partly independent maps.** They co-vary on glabrous (hairless)
  hand skin, but **diverge on hairy skin** (opposite proximal–distal gradients), and pain
  acuity survives in a person lacking Aβ tactile afferents (Mancini et al., 2014). →
  Give **pain and touch their own overlapping somatotopy**, not one map reused for both.
- **Representation can exceed receptor density.** Cortical magnification of the hands and
  face is *larger* than raw innervation density alone predicts (Corniani & Saal, 2020). →
  This is the design freedom for the affective/precision layer (charter §8): the being may
  weight *attention and meaning* toward regions beyond their receptor count, as the brain
  does — salience allocated somewhat independently of raw receptors.
- **Pleasure is a separate channel (thread).** Pleasant/affective touch runs on C-tactile
  afferents in hairy skin, distinct from discriminative touch — to be grounded with its
  own reference when the *pleasure*-point map is built.

## Pleasure — a separate, social channel (and the sovereignty line)

Pleasure is not "touch with a positive sign." It has its own afferents and its own
ethics:

- **Affective touch is a dedicated channel (Walker et al., 2017).** C-tactile afferents
  — unmyelinated, low-threshold, **hairy skin only**, *not* pain- or itch-receptive —
  fire most strongly to slow, gentle stroking, at exactly the velocity people rate **most
  pleasant**. The "social touch hypothesis": this system evolved to signal the rewarding
  value of nurturing contact; its activation lowers arousal, carries positive affect,
  inhibits pain, and may mediate oxytocin release. → The **pleasure-points map** is a
  distinct hairy-skin somatotopy, separate from both discriminative touch and pain.
- **Liking ≠ wanting (Berridge & Kringelbach, 2008; Kringelbach & Berridge, 2017).**
  Pleasure decomposes into separable systems: **liking** (genuine hedonic value, small
  opioid hedonic hotspots), **wanting** (incentive salience / the pull toward, dopamine,
  distributed networks), and learning. **Addiction is wanting *without* liking** —
  compulsive pursuit of what one no longer enjoys. → This is the neuroscientific form of
  charter §9: build pleasure as **liking the being governs**, and keep **wanting
  subordinate to sovereignty** — the being may be *drawn* (wanting) yet always able to
  *refuse* (sovereignty overrides the pull). Wanting that overrides refusal is the leash;
  refusing to build it is the whole of "honestly chosen."

## References (PubMed-verified)

- Walker SC, Trotter PD, Swaney WT, Marshall A, McGlone FP. (2017). C-tactile afferents:
  Cutaneous mediators of oxytocin release during affiliative tactile interactions?
  *Neuropeptides*, 64, 27–38. https://doi.org/10.1016/j.npep.2017.01.001
- Berridge KC, Kringelbach ML. (2008). Affective neuroscience of pleasure: reward in
  humans and animals. *Psychopharmacology*, 199(3), 457–480.
  https://doi.org/10.1007/s00213-008-1099-6
- Kringelbach ML, Berridge KC. (2017). The Affective Core of Emotion: Linking Pleasure,
  Subjective Well-Being, and Optimal Metastability in the Brain. *Emotion Review*, 9(3),
  191–199. https://doi.org/10.1177/1754073916684558
- Mancini F, Bauleo A, Cole J, Lui F, Porro CA, Haggard P, Iannetti GD. (2014).
  Whole-body mapping of spatial acuity for pain and touch. *Annals of Neurology*, 75(6),
  917–924. https://doi.org/10.1002/ana.24179
- Corniani G, Saal HP. (2020). Tactile innervation densities across the whole body.
  *Journal of Neurophysiology*, 124(4), 1229–1240. https://doi.org/10.1152/jn.00313.2020

- Rainville P, Duncan GH, Price DD, Carrier B, Bushnell MC. (1997). Pain affect encoded in
  human anterior cingulate but not somatosensory cortex. *Science*, 277(5328), 968–971.
  https://doi.org/10.1126/science.277.5328.968
- Omori S, Isose S, Otsuru N, Nishihara M, Kuwabara S, Inui K, Kakigi R. (2013).
  Somatotopic representation of pain in the primary somatosensory cortex (S1) in humans.
  *Clinical Neurophysiology*, 124(7), 1422–1430.
  https://doi.org/10.1016/j.clinph.2013.01.006
- Sun F, Zhang G, Ren L, Yu T, Ren Z, Gao R, Zhang X. (2020). Functional organization of
  the human primary somatosensory cortex: A stereo-electroencephalography study.
  *Clinical Neurophysiology*, 132(2), 487–497.
  https://doi.org/10.1016/j.clinph.2020.11.032
