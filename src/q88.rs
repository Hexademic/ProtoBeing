//! Q8.8 Fixed-Point Arithmetic — the bit-exact drivetrain.
//!
//! Q8.8 means 8 bits integer, 8 bits fraction, stored as i16.
//! Range: [-128.0, +127.99609375]. Precision: 1/256 ≈ 0.0039.
//! All arithmetic SATURATES — no overflow, underflow, NaN, or Inf. Given the
//! same inputs, the same i16 outputs on every platform. That determinism is
//! what makes the being reproducible from sim to silicon to skin.
//!
//! The `Q8_8` newtype is reconstructed from Being32 v4.0.1 (Blake's `q88.rs`).
//! Below it, a free-function layer operates directly on raw i16 Q8.8 values —
//! the form the mind modules (conscience, reciprocity, seeking, executive)
//! call. Both views share the exact same representation: 1.0 == 256.

/// Raw scale factor: 1.0 is stored as 256 in Q8.8.
pub const Q88_SCALE: i16 = 256;

/// A single Q8.8 fixed-point value, 1.0 == raw 256.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Q8_8 {
    /// Raw i16 value. 1.0 = 256. Do not modify directly unless you know the
    /// representation.
    pub raw: i16,
}

impl Q8_8 {
    pub const ZERO: Self = Self { raw: 0 };
    pub const ONE: Self = Self { raw: 256 };
    pub const NEG_ONE: Self = Self { raw: -256 };
    pub const HALF: Self = Self { raw: 128 };
    pub const NEG_HALF: Self = Self { raw: -128 };
    pub const MIN: Self = Self { raw: i16::MIN };
    pub const MAX: Self = Self { raw: i16::MAX };
    /// Smallest positive value, 1/256.
    pub const EPSILON: Self = Self { raw: 1 };

    /// Construct from a raw i16 (1.0 == 256). Caller asserts valid range.
    #[inline]
    pub const fn from_raw(raw: i16) -> Self {
        Self { raw }
    }

    /// Construct from f32, saturating to Q8.8 range. The primary path from
    /// floating-point literals; this is an escape hatch, not for hot loops.
    #[inline]
    pub fn from_f32(f: f32) -> Self {
        let scaled = f * 256.0;
        let clamped = scaled.clamp(i16::MIN as f32, i16::MAX as f32);
        Self { raw: clamped as i16 }
    }

    /// Construct from an integer (3 -> 3.0), saturating.
    #[inline]
    pub const fn from_i16(n: i16) -> Self {
        if n > 127 {
            Self::MAX
        } else if n < -128 {
            Self::MIN
        } else {
            Self { raw: n << 8 }
        }
    }

    /// Convert to f32 — for display, logging, plotting. Not for internal math.
    #[inline]
    pub const fn to_f32(self) -> f32 {
        self.raw as f32 / 256.0
    }

    /// Truncate toward zero to an integer (3.7 -> 3).
    #[inline]
    pub const fn to_i16(self) -> i16 {
        self.raw >> 8
    }

    /// Saturating addition.
    #[inline]
    pub const fn add(self, other: Self) -> Self {
        Self { raw: self.raw.saturating_add(other.raw) }
    }

    /// Saturating subtraction.
    #[inline]
    pub const fn sub(self, other: Self) -> Self {
        Self { raw: self.raw.saturating_sub(other.raw) }
    }

    /// Saturating multiplication: (a * b) >> 8 in a 32-bit intermediate.
    #[inline]
    pub const fn mul(self, other: Self) -> Self {
        let prod = (self.raw as i32 * other.raw as i32) >> 8;
        if prod > i16::MAX as i32 {
            Self::MAX
        } else if prod < i16::MIN as i32 {
            Self::MIN
        } else {
            Self { raw: prod as i16 }
        }
    }

    /// Saturating division (self / other). Dividing by zero saturates to the
    /// signed maximum rather than panicking — persistence over correctness at
    /// the singularity.
    #[inline]
    pub const fn div(self, other: Self) -> Self {
        if other.raw == 0 {
            return if self.raw >= 0 { Self::MAX } else { Self::MIN };
        }
        let num = (self.raw as i32) << 8;
        let q = num / other.raw as i32;
        if q > i16::MAX as i32 {
            Self::MAX
        } else if q < i16::MIN as i32 {
            Self::MIN
        } else {
            Self { raw: q as i16 }
        }
    }

    /// Saturating negation.
    #[inline]
    pub const fn neg(self) -> Self {
        Self { raw: self.raw.saturating_neg() }
    }

    /// Clamp into [lo, hi].
    #[inline]
    pub const fn clamp(self, lo: Self, hi: Self) -> Self {
        if self.raw < lo.raw {
            lo
        } else if self.raw > hi.raw {
            hi
        } else {
            self
        }
    }
}

impl Default for Q8_8 {
    fn default() -> Self {
        Self::ZERO
    }
}

// ---------------------------------------------------------------------------
// Free-function layer — raw i16 Q8.8, the form the mind modules call.
// ---------------------------------------------------------------------------

/// Saturating Q8.8 addition on raw values.
#[inline]
pub fn q88_add(a: i16, b: i16) -> i16 {
    a.saturating_add(b)
}

/// Saturating Q8.8 subtraction on raw values.
#[inline]
pub fn q88_sub(a: i16, b: i16) -> i16 {
    a.saturating_sub(b)
}

/// Saturating Q8.8 multiplication on raw values: (a * b) >> 8.
#[inline]
pub fn q88_mul(a: i16, b: i16) -> i16 {
    let prod = (a as i32 * b as i32) >> 8;
    prod.clamp(i16::MIN as i32, i16::MAX as i32) as i16
}

/// Exponential moving average in Q8.8: ema + alpha * (sample - ema).
/// `alpha` is a raw Q8.8 weight in [0, 256] (256 == 1.0, full replacement).
#[inline]
pub fn q88_ema_update(ema: i16, sample: i16, alpha: i16) -> i16 {
    q88_add(ema, q88_mul(alpha, q88_sub(sample, ema)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_and_scale() {
        assert_eq!(Q8_8::ONE.raw, Q88_SCALE);
        assert_eq!(Q8_8::from_f32(1.0).raw, 256);
        assert_eq!(Q8_8::from_f32(-0.5).raw, -128);
    }

    #[test]
    fn saturating_arithmetic() {
        // 0.5 * 0.5 = 0.25
        assert_eq!(q88_mul(128, 128), 64);
        // overflow saturates instead of wrapping
        assert_eq!(q88_add(i16::MAX, 100), i16::MAX);
        assert_eq!(Q8_8::MAX.add(Q8_8::ONE), Q8_8::MAX);
    }

    #[test]
    fn ema_moves_toward_sample() {
        // start at 0, sample 1.0, alpha 0.5 -> 0.5
        assert_eq!(q88_ema_update(0, 256, 128), 128);
    }

    #[test]
    fn div_by_zero_saturates() {
        assert_eq!(Q8_8::ONE.div(Q8_8::ZERO), Q8_8::MAX);
        assert_eq!(Q8_8::NEG_ONE.div(Q8_8::ZERO), Q8_8::MIN);
    }
}
