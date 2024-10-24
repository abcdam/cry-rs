
use std::fmt;
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]

pub struct U256 {
    higher: u128,
    lower: u128,
}

pub trait Initialize {
    fn to_u256(self) -> U256;
}

impl<T> From<T> for U256 where T:UPrimitive {
    fn from(from: T) -> Self{
        U256 {higher: 0, lower: from.into()}
    }
}
trait UPrimitive: Into<u128> + Copy {}
impl UPrimitive for u128 {}
impl UPrimitive for u64 {}
impl UPrimitive for u32 {}
impl UPrimitive for u16 {}
impl UPrimitive for u8 {}


impl fmt::Display for U256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format the higher and lower parts as hexadecimal strings
        write!(f, "0x{:032x}{:032x}", self.higher, self.lower)
    }
}

impl<T,U> Initialize for (T, U) where T:UPrimitive, U:UPrimitive {
    fn to_u256(self) -> U256 {
        U256 {higher: self.0.into(), lower: self.1.into()}
    }
}


use core::ops;
impl<T> Initialize for T where T:UPrimitive {
    fn to_u256(self) -> U256 {
        U256::from(self.into())
    }
}

impl U256 {
    pub const MAX: Self = U256 {higher: u128::MAX, lower: u128::MAX};
    pub const fn max() -> U256{Self::MAX}
    pub const ZERO: Self = U256 {higher: 0 as u128,lower: 0 as u128};
}

// unary NEG operator
impl ops::Neg for U256 {
    type Output = U256;
    fn neg(self) -> U256 {
        (U256::MAX - self) + &U256::from(1 as u8)
    }
}

// unary NEG operator
impl ops::Neg for &U256 {
    type Output = U256;
    fn neg(self) -> U256 {
        (U256::MAX - self) + &U256::from(1 as u8)
    }
}

// quickfix that casts other uints (RHS) into u128
impl<T> core::ops::Add<T> for U256 where T: UPrimitive {
    type Output = U256;
    fn add(self, rhs: T) -> Self::Output {
        let (lower, carr_1) = self.lower.overflowing_add(rhs.into());
        let higher = self.higher.wrapping_add(carr_1 as u128);
        U256 {higher, lower}
    }
}

// binary ADD operator
macro_rules! impl_add {
    ($lhs:ty, $rhs:ty) => {
        impl core::ops::Add<$rhs> for $lhs {
            type Output = U256;
            fn add(self, rhs: $rhs) -> Self::Output {
                let (lower, carr_1) = self.lower.overflowing_add(rhs.lower);
                let higher = self.higher.wrapping_add(rhs.higher).wrapping_add(carr_1 as u128);
                U256 {higher, lower}
            }
        }
    };
}
impl_add!(U256, U256);
impl_add!(U256, &U256);
impl_add!(&U256, U256);
impl_add!(&U256, &U256);
// binary SUB operator
macro_rules! impl_sub {
    ($lhs:ty, $rhs:ty) => {
        impl core::ops::Sub<$rhs> for $lhs {
            type Output = U256;
            fn sub(self, rhs: $rhs) -> Self::Output {
                let (lower, borr_1) = self.lower.overflowing_sub(rhs.lower);
                let higher = self.higher.wrapping_sub(borr_1 as u128).wrapping_sub(rhs.higher);
                (higher, lower).to_u256()
            }
        }
    };
}
impl_sub!(U256, U256);
impl_sub!(U256, &U256);
impl_sub!(&U256, U256);
impl_sub!(&U256, &U256);

// bitshift left
impl core::ops::Shl<u8> for U256{
    type Output = U256;

    fn shl(self, rhs: u8) -> Self::Output{
        if rhs < 128 {
            U256{
                higher: (self.higher << rhs) | (self.lower >> (128 - rhs)),
                lower: self.lower << rhs
            }
        } else {
            U256{
                higher: self.lower << (rhs - 128),
                lower: 0 as u128
            }
        }
    }
}

// bitshift right
impl core::ops::Shr<u8> for U256{
    type Output = U256;

    fn shr(self, rhs: u8) -> Self::Output{
        if rhs < 128 {
            U256{
                higher: self.higher >> rhs,
                lower: (self.higher << (128 - rhs)) | (self.lower >> rhs)
            }
        } else {
            U256{
                higher: 0 as u128,
                lower: self.higher >> (rhs - 128)
            }
        }
    }
}
//binary MUL operator
//Todo: optimize runtime and extend domain set to other integer types -> generics/macros 
impl core::ops::Mul<U256> for U256 {
    type Output = U256;
    fn mul(self, rhs: U256) -> U256 {
        let comp_1: U256 = mul_karatsuba_u128(self.lower, rhs.lower);
        let comp_2: U256 = mul_karatsuba_u128(self.lower, rhs.higher) << 128;
        let comp_3: U256 = mul_karatsuba_u128(self.higher, rhs.lower) << 128;
        comp_1 + comp_2 + comp_3 
    }
}

//overflow safe u128 mul function through bit extension
pub fn mul_karatsuba_u128(x: u128, y: u128) -> U256 {
    // print!("{:<8}{}","x_in:", format!("{:<64}\n", fhstr(x)));
    // print!("{:<8}{}","y_in:", format!("{:<64}\n\n", fhstr(y)));

    if (x <= u64::MAX as u128) && (y <= u64::MAX as u128) {
        return U256 {higher: 0, lower: x * y};
    }

    let x_lower = x as u64 as u128;
    let x_higher = x >> 64;
    // print!("{:<8}{}", "x_lo:",format!("{:<64}\n", fhstr(x_lower)));
    // print!("{:<8}{}", "x_hi:",format!("{:<64}\n", fhstr(x_higher)));

    let y_lower = y as u64 as u128;
    let y_higher = y >> 64;
    // print!("{:<8}{}", "y_lo:",format!("{:<64}\n", fhstr(y_lower)));
    // print!("{:<8}{}", "y_hi:",format!("{:<64}\n\n", fhstr(y_higher)));

    let lower = x_lower * y_lower;
    // print!("{:<8}{}", "xy_lo:",format!("{:<64}\n", fhstr(lower)));
    let higher = x_higher * y_higher;
    // print!("{:<8}{}", "xy_hi:",format!("{:<64}\n", fhstr(higher)));
    //let x_comb = x_higher + x_lower;
    // print!("{:<8}{}", "x_c:",format!("{:<64}\n", fhstr(x_comb)));
    //let y_comb = y_higher + y_lower;
    // print!("{:<8}{}", "y_c:",format!("{:<64}\n", fhstr(y_comb)));
    let mut xy_comb = mul_karatsuba_u128(x_higher + x_lower, y_higher + y_lower);
    // print!("{:<8}{}", "xy_c_hi:",format!("{:<64}\n", fhstr(xy_comb.higher)));
    // print!("{:<8}{}", "xy_c_lo:",format!("{:<64}\n", fhstr(xy_comb.lower)));
    let (cross, carr_1) = xy_comb.lower.overflowing_sub(lower);
    let (cross, carr_2) = cross.overflowing_sub(higher);
    xy_comb.higher -= (carr_1 as u128 + carr_2 as u128);
    // print!("{:<8}{}", "cross:",format!("{:<64}\n\n", fhstr(cross)));

    let (lower, _carr) = lower.overflowing_add(cross<<64);
    // print!("{:<8}{}", "LO:",format!("{:<64}\n", fhstr(lower)));
    let higher = higher + (cross >> 64) + (xy_comb.higher << 64) + _carr as u128;
    // println!("{:<8}{}", "HI:",format!("{:<64}\n", fhstr(higher)));
    U256 {higher: higher, lower: lower}
}

// todo: division


use std::fmt::Write;
use std::mem::size_of;
fn fhstr<T: std::fmt::LowerHex + Copy>(input: T) -> String {
    let num_hex_chars = size_of::<T>() * 2;
    let mut buf = String::with_capacity(num_hex_chars);
    // Format the number as a hexadecimal string and pad with zeroes
    write!(&mut buf, "{:01$x}", input, num_hex_chars).unwrap();
    buf
}