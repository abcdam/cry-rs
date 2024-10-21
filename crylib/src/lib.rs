
//mod u256;
#[cfg(test)]
mod tests {

    use super::*;
    //use rand::Rng;
    use std::{time::Instant, u128};
    // use rayon::prelude::*;

    #[test]
    fn test_u256_init(){
        let u_zero: u8 = 0;
        assert_eq!((u_zero as u128).to_u256(),(u_zero as u64).to_u256());
        assert_eq!((u_zero as u64).to_u256(),(u_zero as u32).to_u256());
        assert_eq!((u_zero as u32).to_u256(),(u_zero as u16).to_u256());
        assert_eq!((u_zero as u16).to_u256(),(u_zero).to_u256());

        assert_eq!(U256::from(u_zero), U256::from(u_zero as u16));
        assert_eq!(U256::from(u_zero as u16), U256::from(u_zero as u32));
        assert_eq!(U256::from(u_zero as u32), U256::from(u_zero as u64));
        assert_eq!(U256::from(u_zero as u64), U256::from(u_zero as u128));

    }
    #[test]
    fn test_u256_add(){
        let u_zero: u8 = 0;
        let u_one: u8 = 1;
        let zero_256 = U256::from(u_zero);
        let one_256 = U256::from(u_one);
        let u_max = u128::MAX;
        let max_rhs_256 = U256::from(u_max);
        let max_lhs_256=(u_max, u_zero).to_u256();
        
        // sanity check
        assert_ne!(zero_256,one_256);
        assert_ne!(max_rhs_256,max_lhs_256);
        assert_eq!(U256::from(2 as u8), one_256 + one_256);

        // some checks for
        //      abelian group props: associative, id, inverse, commutative
        assert_eq!(one_256, one_256 + zero_256);
        assert_eq!(one_256, zero_256 + one_256);
        assert_eq!((one_256 + max_rhs_256) + max_lhs_256, one_256 + (max_rhs_256 + max_lhs_256));
        assert_eq!(one_256, U256::from(2*u_one) + max_rhs_256 + max_lhs_256);

        // modularity/edgecase/closure check
        assert_eq!((u_one,u_zero).to_u256(), one_256 + max_rhs_256); // all rhs bits flip to zero and LSB in lhs flips to one
        assert_eq!((u_max, u_max).to_u256(), max_rhs_256 + max_lhs_256);
        assert_eq!((u_max, u_one).to_u256(), max_lhs_256 + one_256);
        assert_eq!(zero_256, (max_lhs_256+max_rhs_256) + one_256); // 1 added to MAX_VAL of type u256 wraps around to 0 
        assert_eq!(U256::from(20 as u8), (u_max,u_max).to_u256() + U256::from(21 as u8));
    }
    #[test]
    fn test_u256_sub(){
        let u_zero: u8 = 0;
        let u_one: u8 = 1;
        let zero_256 = U256::from(u_zero);
        let one_256 = U256::from(u_one);
        let max_rhs_256 = U256::from(u128::MAX);
        let max_lhs_256=(u128::MAX, u_zero).to_u256();

        // rhs neutral element
        assert_eq!(one_256, one_256 - zero_256);
        assert_eq!(zero_256, zero_256 - zero_256);

        // modularity/edgecase/closure check
        assert_eq!(max_lhs_256+max_rhs_256, zero_256 - one_256);
        assert_eq!(max_rhs_256, (u_one,u_zero).to_u256() - one_256);
        assert_eq!(one_256, zero_256 - (max_lhs_256+max_rhs_256));
        assert_eq!((max_lhs_256+ max_rhs_256),-U256::from(1 as u8));
        assert_eq!(zero_256,-U256::from(1 as u8) + one_256);
        assert_eq!(zero_256, -max_rhs_256 + max_rhs_256);
    }

    #[test]
    fn time_it() {
        let start = Instant::now();
        let base: i128 = 2;
        let runs = u64::pow(base as u64,34)-1;
        let res = U256::from(0 as u8);
        let mut x = U256::from(4 as u8);
        // for _i in 0..2 {
        //     for _k in 0..128 {
        //         x = x + x;
        //         println!("{:?}", x);
        //     }
        //     println!("Round num. {_i}");
        //     let res = res + x;
        //     println!("res: {:?}", res); // force use of calc. var to avoid optimization
        // }

        
        let _end = Instant::now();
        let duration = start.elapsed();
        println!("Time elapsed: {:?}", duration);
    }
    //#[test]
    // fn time_it() {
    //     let start = Instant::now();
//
    //     let base: i128 = 2;
    //     let runs = u64::pow(base as u64,33)-1;
    //     const ROUNDS: u8 = 3;
    //     let mut sum: i128 = 0;
    //     for _i in 0..ROUNDS {
    //         let partial_sum: i128 = (0..runs).into_par_iter().map(|_| {
    //             let mut rng = rand::thread_rng(); // Initialize random number generator inside the loop
    //             let rand_a: i128 = rng.gen_range(-i128::pow(base, 16)..=i128::pow(base, 16) - 1);
    //             let rand_b: i128 = rng.gen_range(-10..=10);
    //             let (a, b): (i128, i128) = (rand_a, rand_b);
    //             let res = egcd128b(a, b);
    //             res.0
    //         }).sum(); // Sum the results in parallel
    //         println!("partialSum: {:?}", partial_sum);
    //         sum += partial_sum as i128; // Aggregate the results
    //     }
    //     let end = Instant::now();
    //     let duration = start.elapsed();
    //     println!("Time elapsed: {:?}", duration);
    //     println!("Sum: {:?}", sum);
    //     assert!(duration > end.elapsed());
    // }
}

pub fn egcd128b(mut a: i128, mut b: i128) -> (i128, i128, i128) {
    let (mut x1, mut x2, mut y1, mut y2):(i128,i128,i128,i128) = (1, 0, 0, 1);
    while b != 0 {
        let q: i128 = a / b;
        (a, x1, y1, b, x2, y2)=(b , x2 , y2 , a-q*b , x1-q*x2 , y1-q*y2);
    }
    if a < 0 {
        (a,x1,y1) = (-a,-x1,-y1);
    } 
    (a as i128,x1 as i128,y1 as i128)

}

// U256 type 
use core::ops;

trait UPrimitive: Into<u128> + Copy {}
impl UPrimitive for u128 {}
impl UPrimitive for u64 {}
impl UPrimitive for u32 {}
impl UPrimitive for u16 {}
impl UPrimitive for u8 {}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct U256 {
    higher: u128,
    lower: u128,

}

impl<T> From<T> for U256 where T:UPrimitive {
    fn from(from: T) -> Self{
        U256 {higher: 0, lower: from.into()}
    }
}

pub trait Initialize {
    fn to_u256(self) -> U256;
}
impl<T,U> Initialize for (T, U) where T:UPrimitive, U:UPrimitive {
    fn to_u256(self) -> U256 {
        U256 {higher: self.0.into(), lower: self.1.into()}
    }
}
impl<T> Initialize for T where T:UPrimitive {
    fn to_u256(self) -> U256 {
        U256 {higher: 0, lower: self.into()}
    }
}

impl U256 {
    const MAX: Self = U256 {higher: u128::MAX, lower: u128::MAX};
}

// unary NEG operator
impl ops::Neg for U256 {
    type Output = U256;
    fn neg(self) -> U256 {
        U256::MAX - self + U256::from(1 as u8)
    }
}

// binary ADD operator
// todo: testing
impl ops::Add<U256> for U256 {
    type Output = U256;

    fn add(self, rhs: U256) -> U256 {
        let (lower, carr_1) = self.lower.overflowing_add(rhs.lower);
        let higher = self.higher.wrapping_add(rhs.higher).wrapping_add(carr_1 as u128);
        U256 {higher, lower}
        
    }
}
// binary SUB operator
// todo: testing
impl ops::Sub<U256> for U256 {
    type Output = U256;
    fn sub(self, rhs: U256) -> U256 {
        let (lower, borr_1) = self.lower.overflowing_sub(rhs.lower);
        let self_higher = self.higher.wrapping_sub(borr_1 as u128);
        let higher = self_higher.wrapping_sub(rhs.higher);
        (higher, lower).to_u256()
    }
}

// binary MUL operator
// impl ops::Mul<U256> for U256 {
//     type Output = U256;
//     fn mul(self, rhs: U256) -> U256 {

        
//         let lower = mul_karatsuba_u128(self.lower, rhs.lower);
//         let higher = mul_karatsuba_u128(self.higher,rhs.higher);
//         let cross = 


//         let lower_total = 
//         let higher_total = 

//         (higher_total,lower_total).to_u256()
//     }
// }

// overflow safe u128 mul function through bit extension
// todo: testing
// fn mul_karatsuba_u128(x: u128, y: u128) -> U256 {
//     let x_lower = x as u64 as u128;
//     let x_higher = x >> 64;
//     let y_lower = y as u64 as u128;
//     let y_higher = y >> 64;

//     let lower = x_lower * y_lower;
//     let higher = x_higher * y_higher;
//     let cross = (x_higher + x_lower) * (y_higher + y_lower) 
//                 - (higher + lower);

//     let (lower,carr) = lower.overflowing_add(cross << 64);
//     let higher = higher + (cross >> 64) + carr as u128; 
//     (higher, lower).to_u256()
// }