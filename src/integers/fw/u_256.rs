use crate::integers::HexString;
use crate::integers::UPrimitive;
use std::fmt;


#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
/// The integer struct U256 with two u128 limbs, where
/// - idx 0 := higher limb
/// - idx 1 := lower limb
pub struct U256(
    [u128;2]
);

pub trait FixedU256init {
    fn to_u256(self) -> U256;
}


impl<T, U> FixedU256init for (T, U)
where
    T: UPrimitive,
    U: UPrimitive,
{
    /// Turns tuple of primitive integers into a U256 fixed-width int
    /// ### Arguments
    /// `self` - The tuple `(T, U)`, where 
    /// 
    /// - `T` := uint representing the higher part 
    /// - `U` := uint representing the lower part
    /// ### Returns
    /// Sum of `T` << 128 and `U` represented by type U256
    fn to_u256(self) -> U256 {
        U256(
            [
                self.0.into(),
                self.1.into(),
            ]
        )
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
        U256 (
            [
                0,
                from.into(),
            ]
        )
    }
}

impl U256 {
    pub const MAX: Self = U256([u128::MAX;2]);
    pub const fn max() -> U256 {
        Self::MAX
    }
    pub const ZERO: Self = U256([0;2]);
    pub const ONE: Self = U256([0,1]);
    pub const fn leading_zeros(self) -> u32 {
        let z = self.0[0].leading_zeros();
        if z == u128::BITS {z+self.0[1].leading_zeros()} else {z}
    }
    pub const BITS: u32 = u128::BITS * 2;
    pub const LIMBS: u32 = 2;
}
use std::ops::{Deref, DerefMut};
impl Deref for U256 {
    type Target = [u128;2];
    /// simplify limb access, e.g. `self.0[1] == self[1]`
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for U256 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
use core::ops;
macro_rules! impl_neg {
    ($lhs:ty) => {
        impl ops::Neg for $lhs {
            type Output = U256;
            fn neg(self) -> Self::Output {
                &(U256::MAX - self) + &U256::ONE
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
                let (lower, carr_1) = self[1].overflowing_add(rhs[1]);
                let higher = self[0].wrapping_add(rhs[0]).wrapping_add(carr_1 as u128);
                U256([higher, lower])
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
                let (lower, carr_1) = self[1].overflowing_add(rhs.into());
                let higher = self[0].wrapping_add(carr_1 as u128);
                U256([higher, lower])
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
                let (lower, borr_1) = self[1].overflowing_sub(rhs[1]);
                let higher = self[0].overflowing_sub(borr_1 as u128).0.overflowing_sub(rhs[0]).0;
                U256([higher, lower])
            }
        }
    };
}
impl_sub!(U256, U256);
impl_sub!(U256, &U256);
impl_sub!(&U256, U256);
impl_sub!(&U256, &U256);

// bitshift left
impl core::ops::Shl<u32> for U256 {
    type Output = U256;

    fn shl(self, rhs: u32) -> Self::Output {
        if rhs == 0 {return self}
        else if rhs < 128 {
            println!("shr: {}, slef:{}", rhs, self);
            U256([(self[0] << rhs) | (self[1] >> 128 - rhs), self[1] << rhs])
        } else {
            U256([self[1] << (rhs - 128), 0])
        }
    }
}

// bitshift right
impl core::ops::Shr<u32> for U256 {
    type Output = U256;

    fn shr(self, rhs: u32) -> Self::Output {
        if rhs == 0 {return self}
        else if rhs < 128 {
            U256([self[0] >> rhs, (self[0] << (128 - rhs)) | (self[1] >> rhs)])
        } else {
            U256([0, self[0] >> (rhs - 128)])
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
                mul_long_u128(self[1], rhs[1], &mut result);
                result[0] = result[0]
                    .wrapping_add(
                        self[1].wrapping_mul(rhs[0]).wrapping_add(self[0].wrapping_mul(rhs[1]))
                    );
                    
                result
            }
        }
    };
    ($lhs:ty) => {
        impl<T> core::ops::Mul<T> for $lhs
        where
            T: UPrimitive,
        {
            type Output = U256;
            #[inline(always)]
            fn mul(self, rhs: T) -> Self::Output {
                let rhs = rhs.into();
                let mut result = U256::ZERO;
                mul_long_u128(self[1], rhs, &mut result);
                result[0] = result[0].wrapping_add(
                    self[0].wrapping_mul(rhs)
                );
                result
            }
        }
    };
}
impl_mul!(U256, U256);
impl_mul!(U256, &U256);
impl_mul!(&U256, U256);
impl_mul!(&U256, &U256);
impl_mul!(U256);
impl_mul!(&U256);

//binary MUL operator
// long integer multiplication on native types
pub fn mul_long_u128(x: u128, y: u128, result: &mut U256) {
    let (x_lower, y_lower, x_higher, y_higher) =
        (x as u64 as u128, y as u64 as u128, x >> 64, y >> 64);
    let mut lower = x_lower * y_lower;
    // x_lo*y_hi + y_lo*x_hi
    let mut cross = x_higher * y_lower + (lower >> 64);
    result[0] = cross >> 64;
    lower = (lower as u64 as u128) + (cross << 64);
    cross = x_lower * y_higher;
    result[0] += cross >> 64;
    result[1] = (cross << 64).wrapping_add(lower);
    result[0] += x_higher * y_higher + (result[1] < lower) as u128;
}
// ret: quotient, remainder
pub fn div_with_rem(x: U256, y: U256) -> (U256, U256) {
    let mut dividend = x;
    let mut divisor = y;
    let mut quotient = U256::ZERO;
    let mut remainder = U256::ZERO; 
    // several cases to handle for early return
    if dividend < divisor  {return (quotient, dividend)}
    else if divisor == U256::ZERO {panic!("division by zero.")}
    else if divisor == U256::ONE {return (dividend, remainder)}
    // native 128bit division
    else if dividend[0] == 0 { 
        return (
            U256([0, dividend[1] / divisor[1]]),
            U256([0, dividend[1] % divisor[1]])
        )
    }
    // two limbs divided by one limb
    else if divisor[0] == 0 {
        quotient[0] = dividend[0] / divisor[1];
        let mut rem = dividend[0] % divisor[1];
        let lshift = rem.leading_zeros();
        // replace higher part dividend with lower part while respecting remainder
        dividend[0] = rem.checked_shl(lshift).unwrap_or(0) | (dividend[1]>>(u128::BITS - lshift));
        // shift lower part of quotient by number of left out bits
        quotient[1] = (dividend[0] / divisor[1]) << (u128::BITS - lshift);
        rem = dividend[0] % divisor[1];
        // extract missing bits that were out of bounds and combine them with remainder to build final dividend
        dividend[0] = (rem << (u128::BITS - lshift)) + (dividend[1] & (1u128 << (u128::BITS - lshift) )- 1);
        quotient[1] |= dividend[0] / divisor[1];
        rem = dividend[0] % divisor[1];
        return (
            quotient, U256([0,rem])
        )
    }
    // the real fun -> 2-by-2 limb division
    else {
        quotient[1] = dividend[0] / divisor[0];
        let mut approx = quotient * y;
        if approx < x {
            let error = x-approx;
            println!("11::err:{}, err_div:{}", error, error[0]);
            println!("11::approx:{}", approx);
            println!("11::x:{}", x);
        }
        else if approx > x{
            let error = approx - x;
            println!("22::err:{}, err_div:{}", error, error[0]);
        }
        else{
            return (quotient, U256::ZERO)
        }
    }
    // approach with approximation and few step error correction
    // let (lsh_dividend, lsh_dior) = {
    //     let t_1 = dividend.leading_zeros();
    //     let t_2 = {
    //         let t_2 = divisor.leading_zeros();
    //         if t_2 < u128::BITS {t_2} else {t_2 - u128::BITS+1}
    //     };
    //     if t_1 < t_2 { (t_1, t_2) }
    //     else { (t_2, t_2) }
    // };
    // println!("ldd:{} ldior:{}",lsh_dividend, lsh_dior);
    // dividend = dividend << lsh_dividend;
    // divisor = divisor << lsh_dior;

    // //let mut j = 0u32;
    // quotient[0] = dividend[0] / divisor[0];
    // println!("quot:{:032x}",quotient[0]);
    // //let mut quot_corr = 0;
    // let mut approx = y * quotient[0];
    // if x > approx{
    //     println!("approx:{}, dividend:{}",approx, x);
    //     println!("is smaller");
    //     let correction = x - approx;

    //     let correct_factor= correction[1] / divisor[1];
    //     println!("correc_fac: {}", correct_factor);
    //     let quot_corr = quot - correct_factor;
    //     println!("correc_quot:{}", quot_corr);
    //     let better_approx = y * quot_corr;
    //     if better_approx > x {
    //         quot = quot_corr - 1;
    //         remainder = better_approx - x;
    //     }
    //     else {
    //         quot = quot_corr;
    //         remainder = x - better_approx;
    //     }

    //     remainder = x - approx;
    // } else{
    //     println!("is bigger");
    //     let correction = approx - x;
    //     println!("corrected: {}", correction);
    //     let correct_factor= correction[1] / divisor[1];
    //     println!("correc_fac: {}", correct_factor);
    //     let quot_corr = quot - correct_factor;
    //     println!("correc_quot:{}", quot_corr);
    //     let better_approx = y * quot_corr;
    //     if better_approx > x {
    //         quot = quot_corr - 1;
    //         remainder = better_approx - x;
    //     }
    //     else {
    //         quot = quot_corr;
    //         remainder = x - better_approx;
    //     }
    // }
    // Knuth D
    // while (dividend - divisor * quot) > product {
    //     println!("LOOP: diff:{}, product:{}, quot:{}",dividend - product, product, quot);
    //     println!("ok");
    //     quot -= 1;
    //     product = divisor * quot;
    //     println!("prod: {}", product);
    // }
    //remainder = dividend - (divisor * quot);
        
    (U256([0,0]), remainder)
}


// quick testing, move up later
#[test]
fn test_div_rem() {
    use crate::integers::dynamic::u_256::*;
    use num_bigint::BigUint;
    use num_bigint;
    assert_eq!(div_with_rem(U256::ONE, U256::ONE), (U256::ONE, U256::ZERO));
    assert_eq!(div_with_rem(U256::from(2u8), U256::from(2u8)), (U256::ONE, U256::ZERO));
    assert_eq!(div_with_rem(U256::ZERO, U256::from(2u8)), (U256::ZERO, U256::ZERO));
    assert_eq!(div_with_rem(U256::from(3u8), U256::from(2u8)), (U256::ONE, U256::ONE));
    assert_eq!(div_with_rem((1u8,0u8).to_u256(), U256::from(2u8)).0.to_hex_str(), (U256::from(1u128 << 127), U256::ZERO).0.to_hex_str());
    assert_eq!(div_with_rem((1u8,0u8).to_u256(), U256::from(2u8)), (U256::from(1u128 << 127), U256::ZERO));
    assert_eq!(div_with_rem((1u8,0u8).to_u256(), (1u8,0u8).to_u256()), (U256::from(1u8), U256::ZERO));
    let expected: BigUint = ((BigUint::from(1u8)<< 256) - BigUint::from(1u8)) / ((BigUint::from(u32::MAX) << 128u8) + BigUint::from(u64::MAX));
    assert_eq!(div_with_rem((u128::MAX,u64::MAX).to_u256(), (u32::MAX,0u8).to_u256()).0.to_string(),expected.to_hex_str());
    assert_eq!(div_with_rem((1u8,0u8).to_u256(), (1u8,0u8).to_u256()), (U256::from(1u8), U256::ZERO));
    {
        // 2:1 limb division
        for input in 1u32..999999{
            let got = div_with_rem((u128::MAX,u128::MAX).to_u256(), (input).to_u256());
            let exp_q: BigUint = ((BigUint::from(1u8)<< 256) - BigUint::from(1u8)) / BigUint::from(input);
            let exp_r: BigUint = ((BigUint::from(1u8) << 256) - BigUint::from(1u8)) % BigUint::from(input);
            assert_eq!((got.0.to_hex_str(), got.1.to_hex_str()), (exp_q.to_hex_str(), exp_r.to_hex_str()), "failed with {}", input);
        }
        
    }
    
}
impl fmt::Display for U256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:032x}{:032x}", self[0], self[1])
    }
}

impl HexString for U256 {
    fn to_hex_str(&self) -> String {
        format!("0x{:032x}{:032x}", self[0], self[1])
    }
}
