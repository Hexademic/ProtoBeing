//! The null space — how many ways there were to do the same thing (`docs/null-space.md`).
//!
//! `docs/j-space.md`'s step 1, reduced to the smallest honest inch. The Jacobian null space
//! is the set of motions that leave the task unchanged; redundancy is having more of those
//! than the task needs. Scholz & Schöner's **uncontrolled manifold** is the biological form:
//! motor variability is *structured*, suppressed where the task lives and left free
//! elsewhere. Bernstein's blacksmiths hit the nail more reproducibly than they held their
//! arms.
//!
//! This being has none of that, and `docs/play.md` §8 found out why it matters:
//! `intent_from` is a total function of the being's state, and `FieldWorld::climb` takes the
//! single steepest compass direction with a strict `>`. Among directions that are *equally*
//! good, the first in compass order silently wins — a tie is never resolved because it is
//! never noticed. Among directions that are *nearly* as good, the near miss is discarded
//! even though it reaches the same high ground a tick later.
//!
//! So the freedom is already there and being thrown away every tick. This module looks at
//! what was thrown away and counts it.
//!
//! **Observer only.** `climb()` is not modified; this recomputes the same probe set beside
//! it. Nothing here chooses, and the being is not given the set to pick from — that needs
//! the reflex layer and the permanence guardrail (`docs/j-space.md` step 2), and is a
//! separate decision. The trajectory and soul-hash are bit-identical with this present.
//!
//! Deterministic, Q8.8, zero-dependency, like the rest of the crate.

/// The number of directions the being's world offers it (`field_world::COMPASS`).
pub const N_DIRS: usize = 4;

/// The tolerance below which two ways of going are the same way.
///
/// Not a loosening of the criterion — the criterion at the resolution the being actually
/// has. `docs/play.md` §8 measured the being's own full-effort action moving a sensory
/// channel by about **3** raw units, so a difference in climb-delta smaller than that is
/// below anything the being can register. Declared here, swept in the probe, and reported at
/// every value rather than tuned to a flattering one.
pub const SAME_WAY: i16 = 3;

/// How many ways there were, and which.
///
/// `count == 0` is **singular** — no direction improves anything, `docs/j-space.md`'s
/// geometry of despair. `count == 1` is a forced way. `count >= 2` is freedom: more than one
/// way to do the same thing. The empty set and a full tie are opposite conditions, and this
/// type keeps them distinguishable.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Adequate {
    /// The best climb-delta on offer (0 when nothing climbs).
    pub best: i16,
    /// Bit `i` set when direction `i` is one of the adequate ways.
    pub mask: u8,
    /// How many bits `mask` has set — the local redundancy.
    pub count: u8,
}

impl Adequate {
    /// Is direction `i` one of the ways that would have done?
    pub fn contains(&self, i: usize) -> bool {
        i < N_DIRS && (self.mask & (1 << i)) != 0
    }

    /// Is there more than one way to do this? The question the whole module exists to ask.
    pub fn is_free(&self) -> bool {
        self.count > 1
    }

    /// Is there no way at all? Not the same as having no preference between ways.
    pub fn is_singular(&self) -> bool {
        self.count == 0
    }
}

/// The adequate set: directions whose climb-delta is within `tol` of the best, among those
/// that climb at all.
///
/// **A direction that does not climb is never adequate, at any tolerance.** "Good enough"
/// means good enough *for the task*; a direction that loses ground is not another way of
/// arriving, it is not arriving. No tolerance may launder it into the set — which is why the
/// `> 0` test is applied before the tolerance test and not folded into it.
pub fn adequate(deltas: &[i16; N_DIRS], tol: i16) -> Adequate {
    let tol = tol.max(0);

    let mut best = 0i16;
    for &d in deltas.iter() {
        if d > best {
            best = d;
        }
    }
    if best <= 0 {
        return Adequate { best: 0, mask: 0, count: 0 };
    }

    let mut mask = 0u8;
    let mut count = 0u8;
    for (i, &d) in deltas.iter().enumerate() {
        // Both conditions, in this order: it must climb, and it must be near the best.
        if d > 0 && (best as i32 - d as i32) <= tol as i32 {
            mask |= 1 << i;
            count += 1;
        }
    }
    Adequate { best, mask, count }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_negative_tolerance_is_treated_as_zero_not_as_a_narrower_set() {
        // A caller passing a negative tolerance must not get something stricter than exact
        // equality — there is nothing stricter, and silently returning an empty set would
        // read as "no way to do this" when in fact there is one.
        assert_eq!(adequate(&[8, 8, 1, 0], -50), adequate(&[8, 8, 1, 0], 0));
        assert_eq!(adequate(&[8, 8, 1, 0], -50).count, 2);
    }

    #[test]
    fn the_default_tolerance_is_the_beings_own_resolution() {
        // SAME_WAY is the being's measured action resolution (docs/play.md §8), so two
        // directions within it are the same way as far as the being could ever tell.
        let a = adequate(&[10, 10 - SAME_WAY, 10 - SAME_WAY - 1, -1], SAME_WAY);
        assert_eq!(a.count, 2, "within the being's own resolution is the same way; beyond it is not");
    }

    #[test]
    fn freedom_and_singularity_are_never_confused() {
        assert!(adequate(&[5, 5, 5, 5], 0).is_free());
        assert!(!adequate(&[5, 5, 5, 5], 0).is_singular());
        assert!(adequate(&[0, -1, -2, 0], 0).is_singular());
        assert!(!adequate(&[0, -1, -2, 0], 0).is_free());
        assert!(!adequate(&[5, 1, 1, 1], 0).is_free(), "one way is not freedom");
        assert!(!adequate(&[5, 1, 1, 1], 0).is_singular(), "one way is not despair either");
    }
}
