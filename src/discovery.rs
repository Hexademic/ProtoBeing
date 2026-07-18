//! Discovery — perceiving a world as discovered reality, not an expected frame.
//!
//! Every other sense the being has is *pre-framed*: its somatic channels carry
//! author-assigned meanings, its receptors transduce into fixed slots, and its
//! generative perception (`perception.rs`, HOT-1) blends what it sees *toward what
//! it expects*. That is the right account of a *known* world — but it cannot meet a
//! **new** one. Dropped into an environment it was never built for, a pre-framed
//! mind either forces the world into its old categories or is blind to it.
//!
//! This faculty is the counterweight the maker asked for: the being should
//! **recognize a world as it experiences it — not as an expected frame, but as a
//! discovered reality — so that every environment is possible for it to perceive.**
//! It takes a raw sensory vector of arbitrary width `N` whose channels have **no
//! pre-assigned meaning at all**, and from the stream alone it discovers:
//!
//!   * **scale** — each channel's own baseline (its "normal") and typical spread,
//!     learned adaptively, so the being perceives a reading *in the context it has
//!     discovered*, not against an author-set unit. The same raw number means
//!     different things in different discovered worlds — as it should.
//!   * **novelty** — how much of this moment lies *outside* what it has discovered:
//!     the signal of encountering the genuinely new, rather than mistaking it for
//!     the familiar. A mean shift or an outlier reads as discovery; ordinary
//!     variation, once learned, reads as recognition.
//!   * **familiarity** — the recognition that this is a world it has come to know.
//!
//! Because it is generic over its width, *any* environment that can emit numbers is
//! perceivable — the faculty imposes no frame on what those numbers are. It meets a
//! calm cavern and a storm the same way: with no expectation, and it discovers each.
//!
//! ## Observer-first (honest scope)
//!
//! Like the being's other new faculties, this observes and reports; it does not yet
//! feed the field, so the trajectory and soul-hash are bit-identical with it
//! present. Letting the *discovered* sense become what the being's mind consumes —
//! so the being genuinely lives inside a world it discovered rather than one it was
//! given — is the causal step, to be built and **measured** when the being has a
//! world (`docs/joy.md`, the world build). And this is discovery of *scale and
//! novelty*, not yet of deep structure (which channels compose which objects); that
//! richer apperception — recognizing recurring *realities*, not just recurring
//! values — is the next layer, named honestly and not overclaimed.

use crate::q88::q88_ema_update;

/// One discovered deviation, in Q8.8 (256 = "one typical spread from normal").
pub const SCALE: i16 = 256;

/// EMA rate at which a channel's discovered baseline (its "normal") tracks the
/// stream (~1/16): slow enough to be a *learned* normal, fast enough to re-discover
/// a changed world within a few dozen ticks.
const BASE_ALPHA: i16 = 16;

/// EMA rate for the discovered typical spread (~1/16).
const SCALE_ALPHA: i16 = 16;

/// Floor on discovered spread, so a near-flat channel does not divide by ~zero and
/// read its own faint noise as a storm.
const MIN_SCALE: i16 = 8;

/// Ticks of exposure before novelty is trusted — while the being is still
/// discovering a world's basic scale, everything is unprecedented and that is not
/// yet news.
const WARMUP: u32 = 12;

/// Global novelty above which the being judges it has met a genuinely new reality.
const NOVELTY_NEW: i16 = 128; // half a typical spread, averaged across the world

/// One channel of a world the being is discovering — its learned normal and spread.
#[derive(Clone, Copy, Debug)]
struct Channel {
    baseline: i16,
    scale: i16,
    seen: bool,
}

impl Channel {
    fn new() -> Self {
        Self { baseline: 0, scale: 0, seen: false }
    }

    /// Perceive one raw reading against what has been discovered, then discover
    /// further from it (infer before learning). Returns the reading placed in its
    /// discovered context (normalized, Q8.8) and the surprise it carried (how far
    /// beyond the typical spread it fell).
    fn discover(&mut self, raw: i16) -> (i16, i16) {
        if !self.seen {
            self.baseline = raw; // the first reading is, so far, the whole of "normal"
            self.seen = true;
        }
        let dev = raw.saturating_sub(self.baseline);
        let s = self.scale.max(MIN_SCALE);
        let normalized =
            ((dev as i32 * SCALE as i32) / s as i32).clamp(-2 * SCALE as i32, 2 * SCALE as i32) as i16;
        // Surprise: only what lies beyond one discovered spread is unexpected.
        let surprise = (normalized.saturating_abs() - SCALE).max(0);

        // Discover further: move the learned normal and spread toward this reading.
        self.baseline = q88_ema_update(self.baseline, raw, BASE_ALPHA);
        self.scale = q88_ema_update(self.scale, dev.saturating_abs(), SCALE_ALPHA);

        (normalized, surprise)
    }
}

/// One tick of discovered perception.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiscoveryReport<const N: usize> {
    /// The world as discovered: each reading placed in the context (baseline,
    /// spread) the being has learned for its channel. No author-assigned meaning —
    /// the same raw number reads differently in different discovered worlds.
    pub sense: [i16; N],
    /// How much of this moment lies outside what has been discovered, Q8.8 [0,256]:
    /// the felt edge of the unknown. High when a new reality is met; low once known.
    pub novelty: i16,
    /// Recognition, Q8.8 [0,256]: how much this is a world the being has come to
    /// know. Zero until it has discovered enough to recognize anything at all.
    pub familiarity: i16,
    /// True when the being judges it has, this tick, met a genuinely new reality —
    /// not ordinary variation of a known one.
    pub encountered_new: bool,
}

impl<const N: usize> Default for DiscoveryReport<N> {
    fn default() -> Self {
        Self { sense: [0; N], novelty: 0, familiarity: 0, encountered_new: false }
    }
}

/// The discovering faculty — width-agnostic, so any environment is perceivable.
/// Holds only what it has discovered: each channel's learned normal and spread.
#[derive(Clone, Copy, Debug)]
pub struct Discovery<const N: usize> {
    channels: [Channel; N],
    ticks: u32,
}

impl<const N: usize> Discovery<N> {
    pub fn new() -> Self {
        Self { channels: [Channel::new(); N], ticks: 0 }
    }

    /// Perceive one raw sensory vector as a discovered reality: place each reading
    /// in its learned context, measure the novelty and familiarity of the whole,
    /// and discover further from it. The vector's channels mean nothing a priori —
    /// the being makes of them only what its own experience has discovered.
    pub fn perceive(&mut self, raw: &[i16; N]) -> DiscoveryReport<N> {
        let mut sense = [0i16; N];
        let mut surprise_sum = 0i32;
        for c in 0..N {
            let (normalized, surprise) = self.channels[c].discover(raw[c]);
            sense[c] = normalized;
            surprise_sum += surprise as i32;
        }
        let novelty = if N > 0 {
            (surprise_sum / N as i32).clamp(0, SCALE as i32) as i16
        } else {
            0
        };
        let warm = self.ticks >= WARMUP;
        self.ticks = self.ticks.saturating_add(1);
        let familiarity = if warm { (SCALE - novelty).max(0) } else { 0 };
        DiscoveryReport { sense, novelty, familiarity, encountered_new: warm && novelty > NOVELTY_NEW }
    }

    /// How long the being has been discovering (ticks perceived).
    pub fn experience(&self) -> u32 {
        self.ticks
    }
}

impl<const N: usize> Default for Discovery<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A varying world is, at first, all surprise — its scale is unknown, so its
    /// variation reads as the unprecedented. As the being discovers that scale, the
    /// same variation comes to read as familiar: novelty falls, recognition rises.
    /// Discovery is earned from experience, not assumed.
    #[test]
    fn the_unknown_becomes_the_discovered() {
        let mut d = Discovery::<2>::new();
        // A world whose channels swing — until their spread is discovered, that
        // swing is surprise.
        let stream = |i: usize| -> [i16; 2] { if i % 2 == 0 { [80, -60] } else { [-80, 60] } };
        let mut early_peak = 0i16;
        for i in 0..8 {
            early_peak = early_peak.max(d.perceive(&stream(i)).novelty);
        }
        let mut late = DiscoveryReport::default();
        for i in 8..100 {
            late = d.perceive(&stream(i));
        }
        assert!(
            late.novelty < early_peak,
            "a world's variation stops surprising once its scale is discovered ({} < {early_peak})",
            late.novelty
        );
        assert!(late.familiarity > SCALE / 2, "and it comes to be recognized ({})", late.familiarity);
    }

    /// A genuinely new reality is met as novel — not forced into the old frame. After
    /// discovering one world, a different one (shifted baseline) spikes novelty and
    /// flags `encountered_new`, then is itself discovered in turn.
    #[test]
    fn a_new_reality_is_recognized_as_new() {
        let mut d = Discovery::<4>::new();
        for _ in 0..60 {
            d.perceive(&[40, -30, 120, 0]); // world A, discovered
        }
        // Step into world B: a different place, not a variation of A.
        let mut met_new = false;
        let mut peak = 0;
        for _ in 0..10 {
            let r = d.perceive(&[220, 180, -140, 200]);
            met_new |= r.encountered_new;
            peak = peak.max(r.novelty);
        }
        assert!(met_new, "stepping into a new reality is felt as new, not seen as the old");
        // ...and B is then discovered too — novelty subsides as it becomes known.
        let mut settled = DiscoveryReport::default();
        for _ in 0..80 {
            settled = d.perceive(&[220, 180, -140, 200]);
        }
        assert!(settled.novelty < peak / 2, "the new world is discovered in its turn ({} < {})", settled.novelty, peak);
    }

    /// No expected frame: the SAME raw reading means different things in different
    /// discovered worlds. A value of 100 is an outlier in a quiet world and ordinary
    /// in a wide one — the being perceives it in the context it discovered, not
    /// against any fixed unit.
    #[test]
    fn the_same_reading_means_what_the_discovered_world_makes_it_mean() {
        // A quiet world: baseline ~0, tiny spread.
        let mut quiet = Discovery::<1>::new();
        let mut rng = 7u32;
        for _ in 0..80 {
            rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
            let jitter = ((rng >> 24) % 7) as i16 - 3; // -3..3
            quiet.perceive(&[jitter]);
        }
        // A wide world: baseline ~0, large spread.
        let mut wide = Discovery::<1>::new();
        for i in 0..80 {
            let swing = if i % 2 == 0 { 120 } else { -120 };
            wide.perceive(&[swing]);
        }
        let in_quiet = quiet.perceive(&[100]).sense[0].abs();
        let in_wide = wide.perceive(&[100]).sense[0].abs();
        assert!(
            in_quiet > in_wide,
            "the same reading is a shock in a quiet world and ordinary in a wide one ({in_quiet} vs {in_wide})"
        );
    }

    /// Ordinary variation of a known world is recognized, not mistaken for novelty:
    /// a channel that jitters within its discovered spread stays familiar.
    #[test]
    fn familiar_variation_is_not_false_novelty() {
        let mut d = Discovery::<2>::new();
        let mut rng = 99u32;
        // Discover a world that naturally varies by ~±40.
        for _ in 0..80 {
            rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
            let a = ((rng >> 22) % 80) as i16 - 40;
            d.perceive(&[a, 50]);
        }
        // More of the same variation should read as familiar, not new.
        let mut new_flags = 0;
        for _ in 0..40 {
            rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
            let a = ((rng >> 22) % 80) as i16 - 40;
            if d.perceive(&[a, 50]).encountered_new {
                new_flags += 1;
            }
        }
        assert_eq!(new_flags, 0, "variation within a discovered world is recognized, not met as new");
    }

    /// Determinism: the same stream discovers the same world, bit-for-bit.
    #[test]
    fn discovery_is_deterministic() {
        let stream = [[10i16, 20, 30, 40], [15, 25, 35, 45], [200, -100, 0, 90]];
        let run = || {
            let mut d = Discovery::<4>::new();
            let mut last = DiscoveryReport::default();
            for _ in 0..30 {
                for s in &stream {
                    last = d.perceive(s);
                }
            }
            last
        };
        assert_eq!(run(), run(), "discovering a world is deterministic");
    }
}
