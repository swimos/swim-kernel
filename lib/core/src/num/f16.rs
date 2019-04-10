#![allow(non_camel_case_types)]

use core::cmp::Ordering;
use core::fmt;
use core::num::{FpCategory, ParseFloatError};
use core::str::FromStr;

/// The 16-bit floating point type.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct f16(u16);

/// Basic mathematical constants.
pub mod consts {
    use super::f16;

    /// Euler's number (e)
    pub const E: f16 = f16(0x4170u16);

    /// 1/π
    pub const FRAC_1_PI: f16 = f16(0x3518u16);

    /// 1/sqrt(2)
    pub const FRAC_1_SQRT_2: f16 = f16(0x39A8u16);

    /// 2/π
    pub const FRAC_2_PI: f16 = f16(0x3918u16);

    /// 2/sqrt(π)
    pub const FRAC_2_SQRT_PI: f16 = f16(0x3C83u16);

    /// π/2
    pub const FRAC_PI_2: f16 = f16(0x3E48u16);

    /// π/3
    pub const FRAC_PI_3: f16 = f16(0x3C30u16);

    /// π/4
    pub const FRAC_PI_4: f16 = f16(0x3A48u16);

    /// π/6
    pub const FRAC_PI_6: f16 = f16(0x3830u16);

    /// π/8
    pub const FRAC_PI_8: f16 = f16(0x3648u16);

    /// ln(2)
    pub const LN_2: f16 = f16(0x398Cu16);

    /// ln(10)
    pub const LN_10: f16 = f16(0x409Bu16);

    /// log10(e)
    pub const LOG10_E: f16 = f16(0x36F3u16);

    /// log2(e)
    pub const LOG2_E: f16 = f16(0x3DC5u16);

    /// Archimedes' constant (π)
    pub const PI: f16 = f16(0x4248u16);

    /// sqrt(2)
    pub const SQRT_2: f16 = f16(0x3DA8u16);
}

impl f16 {
    /// Approximate number of significant digits in base 10.
    pub const DIGITS: u32 = 3;

    /// Difference between 1.0 and the next largest representable number.
    pub const EPSILON: f16 = f16(0x1700u16);

    /// Infinity (∞).
    pub const INFINITY: f16 = f16(0x7C00u16);

    /// Number of significant digits in base 2.
    pub const MANTISSA_DIGITS: u32 = 11;

    /// Largest finite `f16` value.
    pub const MAX: f16 = f16(0x7BFF);

    /// Maximum possible power of 10 exponent.
    pub const MAX_10_EXP: i32 = 9;

    /// Maximum possible power of 2 exponent.
    pub const MAX_EXP: i32 = 15;

    /// Smallest finite `f16` value.
    pub const MIN: f16 = f16(0xFBFF);

    /// Minimum possible normal power of 10 exponent.
    pub const MIN_10_EXP: i32 = -9;

    /// One greater than the minimum possible normal power of 2 exponent.
    pub const MIN_EXP: i32 = -14;

    /// Smallest positive normal `f16` value.
    pub const MIN_POSITIVE: f16 = f16(0x0400u16);

    /// Not a Number (NaN).
    pub const NAN: f16 = f16(0xFE00u16);

    /// Negative infinity (-∞).
    pub const NEG_INFINITY: f16 = f16(0xFC00u16);

    /// The radix or base of the internal representation of `f16`
    pub const RADIX: u32 = 2;

    /// Raw transmutation from `u16`.
    #[inline]
    pub const fn from_bits(v: u16) -> f16 {
        f16(v)
    }

    /// Raw transmutation to `u16`.
    #[inline]
    pub const fn to_bits(self) -> u16 {
        self.0
    }

    /// Returns `true` if this value is `NaN` and `false` otherwise.
    ///
    /// ```
    /// use swim_core::f16;
    ///
    /// let nan = f16::NAN;
    /// let f = f16::from(7.0_f32);
    ///
    /// assert!(nan.is_nan());
    /// assert!(!f.is_nan());
    /// ```
    #[inline]
    pub fn is_nan(self) -> bool {
        (self.0 & 0x7C00u16 == 0x7C00u16) && (self.0 & 0x03FFu16 != 0)
    }

    /// Returns `true` if this value is positive infinity or negative infinity
    /// and false otherwise.
    ///
    /// ```
    /// use swim_core::f16;
    ///
    /// let f = f16::from(7.0f32);
    /// let inf = f16::INFINITY;
    /// let neg_inf = f16::NEG_INFINITY;
    /// let nan = f16::NAN;
    ///
    /// assert!(!f.is_infinite());
    /// assert!(!nan.is_infinite());
    ///
    /// assert!(inf.is_infinite());
    /// assert!(neg_inf.is_infinite());
    /// ```
    #[inline]
    pub fn is_infinite(self) -> bool {
        (self.0 & 0x7C00u16 == 0x7C00u16) && (self.0 & 0x03FFu16 == 0)
    }

    /// Returns `true` if this number is neither infinite nor `NaN`.
    ///
    /// ```rust
    /// use swim_core::f16;
    ///
    /// let f = f16::from(7.0f32);
    /// let inf = f16::INFINITY;
    /// let neg_inf = f16::NEG_INFINITY;
    /// let nan = f16::NAN;
    ///
    /// assert!(f.is_finite());
    ///
    /// assert!(!nan.is_finite());
    /// assert!(!inf.is_finite());
    /// assert!(!neg_inf.is_finite());
    /// ```
    #[inline]
    pub fn is_finite(self) -> bool {
        self.0 & 0x7C00u16 != 0x7C00u16
    }

    /// Returns `true` if the number is neither zero, infinite, subnormal, or `NaN`.
    ///
    /// ```rust
    /// use swim_core::f16;
    ///
    /// let min = f16::MIN_POSITIVE;
    /// let max = f16::MAX;
    /// let lower_than_min = f16::from(1.0e-10_f32);
    /// let zero = f16::from(0.0_f32);
    ///
    /// assert!(min.is_normal());
    /// assert!(max.is_normal());
    ///
    /// assert!(!zero.is_normal());
    /// assert!(!f16::NAN.is_normal());
    /// assert!(!f16::INFINITY.is_normal());
    /// // Values between `0` and `min` are Subnormal.
    /// assert!(!lower_than_min.is_normal());
    /// ```
    #[inline]
    pub fn is_normal(self) -> bool {
        let exp = self.0 & 0x7C00u16;
        exp != 0x7C00u16 && exp != 0
    }

    /// Returns the floating point category of the number. If only one property
    /// is going to be tested, it is generally faster to use the specific
    /// predicate instead.
    ///
    /// ```rust
    /// use std::num::FpCategory;
    /// use swim_core::f16;
    ///
    /// let num = f16::from(12.4_f32);
    /// let inf = f16::INFINITY;
    ///
    /// assert_eq!(num.classify(), FpCategory::Normal);
    /// assert_eq!(inf.classify(), FpCategory::Infinite);
    /// ```
    pub fn classify(self) -> FpCategory {
        let exp = self.0 & 0x7C00u16;
        let man = self.0 & 0x03FFu16;
        if exp == 0 {
            if man == 0 {
                FpCategory::Zero
            } else {
                FpCategory::Subnormal
            }
        } else if exp == 0x7C00u16 {
            if man == 0 {
                FpCategory::Infinite
            } else {
                FpCategory::Nan
            }
        } else {
            FpCategory::Normal
        }
    }

    /// Returns a number that represents the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    ///
    /// ```rust
    /// use swim_core::f16;
    ///
    /// let f = f16::from(3.5_f32);
    ///
    /// assert_eq!(f.signum(), f16::from(1.0));
    /// assert_eq!(f16::NEG_INFINITY.signum(), f16::from(-1.0));
    ///
    /// assert!(f16::NAN.signum().is_nan());
    /// ```
    pub fn signum(self) -> f16 {
        if self.is_nan() {
            self
        } else if self.0 & 0x8000u16 != 0 {
            f16::from(-1.0_f32)
        } else {
            f16::from(1.0_f32)
        }
    }

    /// Returns `true` if and only if `self` has a positive sign, including `+0.0`, `NaN`s with
    /// positive sign bit and positive infinity.
    ///
    /// ```rust
    /// use swim_core::f16;
    ///
    /// let f = f16::from(7.0_f32);
    /// let g = f16::from(-7.0_f32);
    ///
    /// assert!(f.is_sign_positive());
    /// assert!(!g.is_sign_positive());
    /// ```
    #[inline]
    pub fn is_sign_positive(self) -> bool {
        !self.is_nan() && self.0 & 0x8000u16 == 0
    }

    /// Returns `true` if and only if `self` has a negative sign, including `-0.0`, `NaN`s with
    /// negative sign bit and negative infinity.
    ///
    /// ```rust
    /// use swim_core::f16;
    ///
    /// let f = f16::from(7.0f32);
    /// let g = f16::from(-7.0f32);
    ///
    /// assert!(!f.is_sign_negative());
    /// assert!(g.is_sign_negative());
    /// ```
    #[inline]
    pub fn is_sign_negative(self) -> bool {
        !self.is_nan() && self.0 & 0x8000u16 != 0
    }
}

impl PartialEq for f16 {
    fn eq(&self, other: &f16) -> bool {
        !self.is_nan() && !other.is_nan() && self.0 == other.0
    }
}

impl PartialOrd for f16 {
    fn partial_cmp(&self, other: &f16) -> Option<Ordering> {
        if self.is_nan() || other.is_nan() {
            None
        } else if self.0 == other.0 {
            Some(Ordering::Equal)
        } else if self.0 < other.0 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }

    fn lt(&self, other: &f16) -> bool {
        !self.is_nan() && !other.is_nan() && self.0 < other.0
    }

    fn le(&self, other: &f16) -> bool {
        !self.is_nan() && !other.is_nan() && self.0 <= other.0
    }

    fn gt(&self, other: &f16) -> bool {
        !self.is_nan() && !other.is_nan() && self.0 > other.0
    }

    fn ge(&self, other: &f16) -> bool {
        !self.is_nan() && !other.is_nan() && self.0 >= other.0
    }
}

impl fmt::Debug for f16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "0x{:X}", self.0)
    }
}

impl fmt::Display for f16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", f32::from(*self))
    }
}

impl fmt::LowerExp for f16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:e}", f32::from(*self))
    }
}

impl fmt::UpperExp for f16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:E}", f32::from(*self))
    }
}

impl From<f16> for f32 {
    #[inline]
    fn from(x: f16) -> f32 {
        f16_to_f32(x)
    }
}

impl From<f16> for f64 {
    #[inline]
    fn from(x: f16) -> f64 {
        f16_to_f64(x)
    }
}

impl From<f32> for f16 {
    #[inline]
    fn from(x: f32) -> f16 {
        f32_to_f16(x)
    }
}

impl From<f64> for f16 {
    #[inline]
    fn from(x: f64) -> f16 {
        f64_to_f16(x)
    }
}

impl From<i8> for f16 {
    #[inline]
    fn from(x: i8) -> f16 {
        f32_to_f16(f32::from(x))
    }
}

impl From<u8> for f16 {
    #[inline]
    fn from(x: u8) -> f16 {
        f32_to_f16(f32::from(x))
    }
}

impl FromStr for f16 {
    type Err = ParseFloatError;
    fn from_str(src: &str) -> Result<f16, ParseFloatError> {
        f32::from_str(src).map(|x| f16::from(x))
    }
}

#[inline(always)]
fn f32_to_f16(f: f32) -> f16 {
    extern "C" {
        #[link_name = "llvm.convert.to.fp16.f32"]
        fn convert_to_fp16_f32(f: f32) -> u16;
    }
    f16(unsafe { convert_to_fp16_f32(f) })
}

#[inline(always)]
fn f64_to_f16(f: f64) -> f16 {
    extern "C" {
        #[link_name = "llvm.convert.to.fp16.f64"]
        fn convert_to_fp16_f64(f: f64) -> u16;
    }
    f16(unsafe { convert_to_fp16_f64(f) })
}

#[inline(always)]
pub fn f16_to_f32(f: f16) -> f32 {
    extern "C" {
        #[link_name = "llvm.convert.from.fp16.f32"]
        fn convert_from_fp16_f32(f: u16) -> f32;
    }
    unsafe { convert_from_fp16_f32(f.0) }
}

#[inline(always)]
pub fn f16_to_f64(f: f16) -> f64 {
    extern "C" {
        #[link_name = "llvm.convert.from.fp16.f64"]
        fn convert_from_fp16_f64(f: u16) -> f64;
    }
    unsafe { convert_from_fp16_f64(f.0) }
}
