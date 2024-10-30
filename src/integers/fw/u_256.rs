use crate::integers::HexString;
use crate::integers::UPrimitive;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct U256 {
    higher: u128,
    lower: u128,
}

pub trait FixedU256init {
    fn to_u256(self) -> U256;
}

impl<T, U> FixedU256init for (T, U)
where
    T: UPrimitive,
    U: UPrimitive,
{
    fn to_u256(self) -> U256 {
        U256 {
            higher: self.0.into(),
            lower: self.1.into(),
        }
    }
}

impl<T> FixedU256init for T
where
    T: UPrimitive,
{
    fn to_u256(self) -> U256 {
        U256::from(self.into())
    }
}

impl<T> From<T> for U256
where
    T: UPrimitive,
{
    fn from(from: T) -> Self {
        U256 {
            higher: 0,
            lower: from.into(),
        }
    }
}

impl U256 {
    pub const MAX: Self = U256 {
        higher: u128::MAX,
        lower: u128::MAX,
    };
    pub const fn max() -> U256 {
        Self::MAX
    }
    pub const ZERO: Self = U256 {
        higher: 0u128,
        lower: 0u128,
    };
}

use core::ops;
macro_rules! impl_neg {
    ($lhs:ty) => {
        impl ops::Neg for $lhs {
            type Output = U256;
            fn neg(self) -> Self::Output {
                (U256::MAX - self) + &U256::from(1 as u8)
            }
        }
    };
}
impl_neg!(U256);
impl_neg!(&U256);

// binary ADD operator
macro_rules! impl_add {
    ($lhs:ty, $rhs:ty) => {
        impl core::ops::Add<$rhs> for $lhs {
            type Output = U256;
            fn add(self, rhs: $rhs) -> Self::Output {
                let (lower, carr_1) = self.lower.overflowing_add(rhs.lower);
                let higher = self.higher + rhs.higher + carr_1 as u128;
                U256 { lower, higher }
            }
        }
    };
    ($lhs:ty) => {
        impl<T> core::ops::Add<T> for $lhs
        where
            T: UPrimitive,
        {
            type Output = U256;

            fn add(self, rhs: T) -> Self::Output {
                let (lower, carr_1) = self.lower.overflowing_add(rhs.into());
                let higher = self.higher.wrapping_add(carr_1 as u128);
                U256 { lower, higher }
            }
        }
    };
}
impl_add!(U256, U256);
impl_add!(U256, &U256);
impl_add!(&U256, U256);
impl_add!(&U256, &U256);
impl_add!(U256);
impl_add!(&U256);

// binary SUB operator
macro_rules! impl_sub {
    ($lhs:ty, $rhs:ty) => {
        impl core::ops::Sub<$rhs> for $lhs {
            type Output = U256;
            fn sub(self, rhs: $rhs) -> Self::Output {
                let (lower, borr_1) = self.lower.overflowing_sub(rhs.lower);
                let higher = self.higher - borr_1 as u128 - rhs.higher;
                U256 { lower, higher }
            }
        }
    };
}
impl_sub!(U256, U256);
impl_sub!(U256, &U256);
impl_sub!(&U256, U256);
impl_sub!(&U256, &U256);

// bitshift left
impl core::ops::Shl<u8> for U256 {
    type Output = U256;

    fn shl(self, rhs: u8) -> Self::Output {
        if rhs < 128 {
            U256 {
                higher: (self.higher << rhs) | (self.lower >> (128 - rhs)),
                lower: self.lower << rhs,
            }
        } else {
            U256 {
                higher: self.lower << (rhs - 128),
                lower: 0u128,
            }
        }
    }
}

// bitshift right
impl core::ops::Shr<u8> for U256 {
    type Output = U256;

    fn shr(self, rhs: u8) -> Self::Output {
        if rhs < 128 {
            U256 {
                higher: self.higher >> rhs,
                lower: (self.higher << (128 - rhs)) | (self.lower >> rhs),
            }
        } else {
            U256 {
                higher: 0u128,
                lower: self.higher >> (rhs - 128),
            }
        }
    }
}
macro_rules! impl_mul {
    ($lhs:ty, $rhs:ty) => {
        impl core::ops::Mul<$rhs> for $lhs {
            type Output = U256;
            #[inline(always)]
            fn mul(self, rhs: $rhs) -> Self::Output {
                let mut result = U256::ZERO;
                mul_long_u128(self.lower, rhs.lower, &mut result);
                result.higher +=
                    self.lower.wrapping_mul(rhs.higher) + self.higher.wrapping_mul(rhs.lower);
                result
            }
        }
    };
}
impl_mul!(U256, U256);
impl_mul!(U256, &U256);
impl_mul!(&U256, U256);
impl_mul!(&U256, &U256);
//binary MUL operator
// long integer multiplication on native types
pub fn mul_long_u128(x: u128, y: u128, result: &mut U256) {
    let (x_lower, y_lower, x_higher, y_higher) =
        (x as u64 as u128, y as u64 as u128, x >> 64, y >> 64);
    let mut lower = x_lower * y_lower;
    // x_lo*y_hi + y_lo*x_hi
    let mut cross = x_higher * y_lower + (lower >> 64);
    result.higher = cross >> 64;
    lower = (lower as u64 as u128) + (cross << 64);
    cross = x_lower * y_higher;
    result.higher += cross >> 64;
    result.lower = (cross << 64) + lower;
    result.higher += x_higher * y_higher + (result.lower < lower) as u128;
}

// use std::fmt::Write;
// use std::mem::size_of;
// fn fhstr<T: std::fmt::LowerHex + Copy>(input: T) -> String {
//     let num_hex_chars = size_of::<T>() * 2;
//     let mut buf = String::with_capacity(num_hex_chars);
//     // Format the number as a hexadecimal string and pad with zeroes
//     write!(&mut buf, "{:01$x}", input, num_hex_chars).unwrap();
//     buf
// }

impl fmt::Display for U256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:032x}{:032x}", self.higher, self.lower)
    }
}

impl HexString for U256 {
    fn to_hex_str(&self) -> String {
        format!("0x{:032x}{:032x}", self.higher, self.lower)
    }
}
