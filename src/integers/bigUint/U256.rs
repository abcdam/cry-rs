use num_bigint::BigUint;
use std::fmt;

trait UPrimitive: Into<u128> + Copy {}
impl UPrimitive for u128 {}
impl UPrimitive for u64 {}
impl UPrimitive for u32 {}
impl UPrimitive for u16 {}
impl UPrimitive for u8 {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct U256(BigUint);
fn __modulo() -> BigUint {BigUint::from(1 as u8) << 256}

pub trait Initialize {
    fn to_u256(self) -> U256;
}
impl<T> From<T> for U256 where T:UPrimitive {
    fn from(from: T) -> Self{
        U256(BigUint::from(from.into()))
    }
}
impl From<BigUint> for U256 {
    fn from(from: BigUint) -> Self{
        U256(from % &__modulo())
    }
}



impl<T,U> Initialize for (T, U) where T:UPrimitive, U:UPrimitive {
    fn to_u256(self) -> U256 {
        let mut bytes = Vec::with_capacity(32);
        bytes.extend_from_slice(&self.0.into().to_be_bytes()); // hi
        bytes.extend_from_slice(&self.1.into().to_be_bytes()); // lo
        //println!("{:?}", bytes);
        U256(BigUint::from_bytes_be(&bytes))
    }
}
impl<T> Initialize for T where T:UPrimitive {
    fn to_u256(self) -> U256 {
        U256::from(self)
    }
}

impl U256 {
    pub fn max() -> U256{U256(__modulo()-1 as u8)}
    pub const ZERO: Self = U256(BigUint::ZERO);
}

impl fmt::Display for U256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format the higher and lower parts as hexadecimal strings
        write!(f, "0x{:x}", self.0)
    }
}

use core::ops;
macro_rules! impl_add {
    ($lhs:ty, $rhs:ty) => {
        impl core::ops::Add<$rhs> for $lhs {
            type Output = U256;
            fn add(self, rhs: $rhs) -> Self::Output {
                U256::from(&self.0 + &rhs.0)
            }
        }
    };
}
impl_add!(U256, U256);
impl_add!(U256, &U256);
impl_add!(&U256, U256);
impl_add!(&U256, &U256);
macro_rules! impl_sub {
    ($lhs:ty, $rhs:ty) => {
        impl core::ops::Sub<$rhs> for $lhs {
            type Output = U256;
            fn sub(self, rhs: $rhs) -> Self::Output {
                U256::from(__modulo() + &self.0 - &rhs.0)
            }
        }
    };
}

impl_sub!(U256, U256);
impl_sub!(U256, &U256);
impl_sub!(&U256, U256);
impl_sub!(&U256, &U256);
// unary NEG operator
impl ops::Neg for &U256 {
    type Output = U256;
    fn neg(self) -> U256 {
        U256(__modulo() - &self.0)
    }
}
// unary NEG operator
impl ops::Neg for U256 {
    type Output = U256;
    fn neg(self) -> U256 {
        U256(__modulo() - &self.0)
    }
}