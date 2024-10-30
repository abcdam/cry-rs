use crate::integers::HexString;
use crate::integers::UPrimitive;
use num_bigint::BigUint;
use std::fmt;
use std::ops::Deref;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BigU256(pub BigUint);
fn __modulo() -> BigUint {
    BigUint::from(1 as u8) << 256
}

impl BigU256 {
    pub fn max() -> BigUint {
        __modulo() - 1 as u8
    }
    pub const ZERO: Self = BigU256(BigUint::ZERO);
    pub fn to_fixed_u256(&self) -> crate::integers::fw::u_256::U256 {
        let list = self.to_u64_digits();
        let (mut lower, mut higher) = (0u128, 0u128);
        for (i, &list) in list.iter().enumerate() {
            if i < 2 {
                lower |= (list as u128) << (i * 64);
            } else {
                higher |= (list as u128) << ((i - 2) * 64);
            }
        }
        crate::integers::fw::u_256::FixedU256init::to_u256((higher, lower))
    }
}

pub trait BigU256init {
    fn to_big_u256(self) -> BigU256;
}
impl BigU256init for BigUint {
    fn to_big_u256(self) -> BigU256 {
        BigU256::from(self)
    }
}
impl<T, U> BigU256init for (T, U)
where
    T: UPrimitive,
    U: UPrimitive,
{
    fn to_big_u256(self) -> BigU256 {
        let higher: BigUint = BigUint::from(self.0.into()) << 128;
        BigU256::from(higher + BigUint::from(self.1.into()))
    }
}

impl<T> From<T> for BigU256
where
    T: UPrimitive,
{
    fn from(from: T) -> Self {
        BigU256(BigUint::from(from.into()))
    }
}
impl From<BigUint> for BigU256 {
    fn from(from: BigUint) -> Self {
        BigU256(from % &__modulo())
    }
}

// forward function calls to BigUint
impl Deref for BigU256 {
    type Target = BigUint;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use core::ops;
macro_rules! impl_add {
    ($lhs:ty, $rhs:ty) => {
        impl core::ops::Add<$rhs> for $lhs {
            type Output = BigU256;
            fn add(self, rhs: $rhs) -> Self::Output {
                BigU256::from(&self.0 + &rhs.0)
            }
        }
    };
}
impl_add!(BigU256, BigU256);
impl_add!(BigU256, &BigU256);
impl_add!(&BigU256, BigU256);
impl_add!(&BigU256, &BigU256);

macro_rules! impl_sub {
    ($lhs:ty, $rhs:ty) => {
        impl core::ops::Sub<$rhs> for $lhs {
            type Output = BigU256;
            fn sub(self, rhs: $rhs) -> Self::Output {
                BigU256::from(__modulo() + &self.0 - &rhs.0)
            }
        }
    };
}
impl_sub!(BigU256, BigU256);
impl_sub!(BigU256, &BigU256);
impl_sub!(&BigU256, BigU256);
impl_sub!(&BigU256, &BigU256);

// unary NEG operator
macro_rules! impl_neg {
    ($lhs:ty) => {
        impl ops::Neg for $lhs {
            type Output = BigU256;
            fn neg(self) -> Self::Output {
                BigU256(__modulo() - &self.0)
            }
        }
    };
}
impl_neg!(BigU256);
impl_neg!(&BigU256);

impl core::ops::Mul<BigU256> for BigU256 {
    type Output = BigU256;
    fn mul(self, rhs: BigU256) -> BigU256 {
        ((self.0 * rhs.0) % __modulo()).to_big_u256()
    }
}

impl std::fmt::Display for BigU256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format the higher and lower parts as hexadecimal strings
        write!(f, "0x{:x}", self.0)
    }
}
impl HexString for BigU256 {
    fn to_hex_str(&self) -> String {
        format!("0x{:064x}", &self.0)
    }
}
