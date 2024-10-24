
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
impl core::ops::Mul<U256> for U256 {
    type Output = U256;
    fn mul(self, rhs: U256) -> U256 {
        println!("MUL of u128 is: {}",mul_karatsuba_u128(u128::MAX, u128::MAX));
        let lower = mul_karatsuba_u128(self.lower, rhs.lower);
        let higher = mul_karatsuba_u128(self.higher, rhs.higher);
        let cross = mul_karatsuba_u128(
            self.higher + self.lower, 
            rhs.higher+rhs.lower
        ) - (higher + lower);

        let lower_tot = lower + (cross << 128);
        let higher_tot = higher + (cross >> 128) + (lower_tot < lower) as u8;

        higher_tot+lower_tot
    }
}

//overflow safe u128 mul function through bit extension
//todo: testing
pub fn mul_karatsuba_u128(x: u128, y: u128) -> U256 {
    print!("{}",format!("{:<9}", fhstr(x)));
    print!("{}",format!("{:<9}", fhstr(y)));

    let x_lower = x & 0xFFFFFFFFFFFFFFFF;
    let x_higher = x >> 64;
    print!("{}",format!("{:<9}", fhstr(x_lower)));
    print!("{}",format!("{:<9}", fhstr(x_higher)));

    let y_lower = y & 0xFFFFFFFFFFFFFFFF;
    let y_higher = y >> 64;
    print!("{}",format!("{:<9}", fhstr(y_lower)));
    print!("{}",format!("{:<9}", fhstr(y_higher)));

    let lower = x_lower * y_lower;
    print!("{}",format!("{:<9}", fhstr(lower)));
    let higher = x_higher * y_higher;
    print!("{}",format!("{:<9}", fhstr(higher)));
    let x_comb = x_higher + x_lower;
    print!("{}",format!("{:<9}", fhstr(x_comb)));
    let y_comb = y_higher + y_lower;
    print!("{}",format!("{:<9}", fhstr(y_comb)));
    let mut xy_comb = mul_safe_karatsuba_u128(x_comb, y_comb);
    print!("{}",format!("{:<9}", fhstr(xy_comb.higher)));
    print!("{}",format!("{:<9}", fhstr(xy_comb.lower)));
    let (cross, carr_1) = xy_comb.lower.overflowing_sub(lower);
    let (cross, carr_2) = cross.overflowing_sub(higher);
    xy_comb.higher -= (carr_1 as u128 + carr_2 as u128);

    println!("");
    println!("lo:{:x} hi:{:x}", lower, higher);
    println!("");
    println!("xc:{:x} yc:{:x} xyc_l:{:x} xyc_h:{:x}", x_comb, y_comb, xy_comb.lower, xy_comb.higher);
    println!("");
    println!("cross:{:x}", cross);

    let (lower, _carr) = lower.overflowing_add(cross<<64);
    println!("{}",format!("{:<9}", fhstr(lower)));
    let higher = higher + (cross >> 64) + (xy_comb.higher << 64) + _carr as u128;
    println!("{}",format!("{:<9}", fhstr(higher)));
    let result = U256 {higher: higher, lower: lower};
    println!("RESULT: {}", result);
    println!("");
    println!("");
    result
}

fn mul_safe_karatsuba_u128(x: u128, y: u128) -> U256 {
    let x_lower = x & 0xFFFFFFFFFFFFFFFF;
    let x_higher = x >> 64;
    let y_lower = y & 0xFFFFFFFFFFFFFFFF;
    let y_higher = y >> 64;

    let higher = x_higher * y_higher;
    let lower = x_lower * y_lower;
    let x_comb = x_higher + x_lower;
    let y_comb = y_higher + y_lower;

    let cross = x_comb * y_comb - higher - lower;
    let (lower, carr_1) = lower.overflowing_add(cross << 64);
    let higher = higher + (cross >> 64) + carr_1 as u128;
    //println!("n_cross: {}, n_lower: {}, n_higher: {}", cross, lower, higher);
    U256{higher, lower}
}

pub fn mul_karatsuba_u8(x: u8, y: u8) -> u16 {
    let mut result = 0u16;
    print!("{}",format!("{:<9}", fhstr(x)));
    print!("{}",format!("{:<9}", fhstr(y)));
    let x_lower = x & 0xF;
    let x_higher = x >> 4;
    print!("{}",format!("{:<9}", fhstr(x_lower)));
    print!("{}",format!("{:<9}", fhstr(x_higher)));
    let y_lower = y & 0xF;
    let y_higher = y >> 4;
    print!("{}",format!("{:<9}", fhstr(y_lower)));
    print!("{}",format!("{:<9}", fhstr(y_higher)));

    let lower = x_lower * y_lower;
    print!("{}",format!("{:<9}", fhstr(lower)));
    let higher = x_higher * y_higher;
    print!("{}",format!("{:<9}", fhstr(higher)));
    let x_comb = x_higher + x_lower;
    print!("{}",format!("{:<9}", fhstr(x_comb)));
    let y_comb = y_higher + y_lower;
    print!("{}",format!("{:<9}", fhstr(y_comb)));
    let (xy_c_hi, xy_c_lo) = nibble_mul(x_comb, y_comb);
    print!("{}",format!("{:<9}", fhstr(xy_c_hi)));
    print!("{}",format!("{:<9}", fhstr(xy_c_lo)));
    let (cross, carr_1) = xy_c_lo.overflowing_sub(lower);
    let (cross, carr_2) = cross.overflowing_sub(higher);
    let xy_c_hi = xy_c_hi -(carr_1 as u8 + carr_2 as u8);
    print!("{}",format!("{:<9}", fhstr(cross)));

    
    let (lower, _carr) = lower.overflowing_add(cross<<4);
    print!("{}",format!("{:<9}", fhstr(lower)));
    let higher = higher + (cross >> 4) + (xy_c_hi << 4) + _carr as u8;
    print!("{}",format!("{:<9}", fhstr(higher)));
    let result: u16 = ((higher as u16) << 8) | lower as u16;
    print!("{}",format!("{:<9}", fhstr(result)));
    println!("{}",format!("{:<9}", fhstr((x as u16)*(y as u16))));
    println!("{}",format!("{:-<9}-", ""));
    result
}

fn nibble_mul(x: u8, y: u8) -> (u8,u8) {
    //print!("x_n: {}, y_n: {} ", x, y);
    let x_lower = x & 0xF;
    let x_higher = x >> 4;
    let y_lower = y & 0xF;
    let y_higher = y >> 4;

    let higher = x_higher * y_higher;
    let lower = x_lower * y_lower;
    let x_comb = x_higher + x_lower;
    let y_comb = y_higher + y_lower;

    let cross = x_comb * y_comb - higher - lower;
    let (lower, carr_1) = lower.overflowing_add(cross << 4);
    let higher = higher + (cross >> 4) + carr_1 as u8;
    //println!("n_cross: {}, n_lower: {}, n_higher: {}", cross, lower, higher);
    (higher, lower)
}

use std::fmt::Write;
use std::mem::size_of;
fn fhstr<T: std::fmt::LowerHex + Copy>(input: T) -> String {
    let num_hex_chars = size_of::<T>() * 2;
    let mut buf = String::with_capacity(num_hex_chars);
    // Format the number as a hexadecimal string and pad with zeroes
    write!(&mut buf, "{:01$x}", input, num_hex_chars).unwrap();
    buf
}