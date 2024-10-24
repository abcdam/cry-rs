#[cfg(any(feature = "U256", feature = "integer_snips"))]
pub mod structs;
#[cfg(any(feature = "U256"))]
use structs::U256::*;

#[cfg(feature = "bigU256")]
pub mod bigUint;

#[cfg(feature = "bigU256")]
use bigUint::U256::*;

#[cfg(any(feature = "U256", feature = "bigU256"))]
mod tests {
    
    //use rand::Rng;
    use  std::time::Instant;
    // use rayon::prelude::*;
    use super::*;
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
        let u_max = u128::MAX;
        let zero_256 = U256::from(u_zero);
        let one_256 = U256::from(u_one);
        let max_rhs_256 = U256::from(u_max);
        let max_lhs_256=(u_max, u_zero).to_u256();
        
        // sanity check
        assert_ne!(zero_256, one_256);
        assert_ne!(max_rhs_256, max_lhs_256);
        assert_eq!(&max_lhs_256 + &max_rhs_256, U256::max());
        assert_eq!(&one_256 + &one_256, U256::from(2 as u8));

        // some checks for
        //      abelian group props: associative, id, inverse, commutative
        assert_eq!(one_256, &one_256 + &zero_256);
        assert_eq!(one_256, &zero_256 + &one_256);
        assert_eq!((&one_256 + &max_rhs_256) + &max_lhs_256, &one_256 + (&max_rhs_256 + &max_lhs_256));
        assert_eq!(one_256, &U256::from(2*u_one) + (&max_rhs_256 + &max_lhs_256));

        // modularity/edgecase/closure check
        assert_eq!((u_one,u_zero).to_u256(), &one_256 + &max_rhs_256); // all rhs bits flip to zero and LSB in lhs flips to one
        assert_eq!((u_max, u_max).to_u256(), &max_rhs_256 + &max_lhs_256);
        assert_eq!((u_max, u_one).to_u256(), &max_lhs_256 + &one_256);
        assert_eq!(zero_256, (max_lhs_256 + max_rhs_256) + one_256); // 1 added to MAX_VAL of type u256 wraps around to 0 
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
        assert_eq!(one_256, &one_256 - &zero_256);
        assert_eq!(zero_256, &zero_256 - &zero_256);

        // modularity/edgecase/closure check
        assert_eq!(&max_lhs_256+ &max_rhs_256, &zero_256 - &one_256);
        assert_eq!(max_rhs_256, (u_one,u_zero).to_u256() - &one_256);
        assert_eq!(one_256, &zero_256 - (&max_lhs_256 + &max_rhs_256));
        assert_eq!(U256::max(),-U256::from(1 as u8));
        assert_eq!(zero_256,-U256::from(1 as u8) + &one_256);
        assert_eq!(zero_256, -&max_rhs_256 + max_rhs_256);
    }

    #[test]
    fn test_u256_MUL(){

        let zero_256 = U256::from(0u8);
        let one_256 = U256::from(1u8);

        // sanity checks
        assert_ne!(one_256 * one_256, zero_256 * zero_256);
        assert_eq!(zero_256, one_256 * zero_256);
        assert_eq!(zero_256, zero_256 * zero_256);
        // assoc
        assert_eq!(
            (U256::from(2u8) * U256::from(3u8)) * U256::from(4u8), 
            U256::from(2u8) * (U256::from(3u8) * U256::from(4u8))
        );

        // id
        let t_1 = (123456789u32,987654321u32).to_u256();
        assert_eq!(t_1, t_1 * one_256);
        assert_eq!(t_1, one_256 * t_1);

        //assert_eq!(U256::MAX * U256::MAX, (0u8,u128::MAX.wrapping_mul(u128::MAX)).to_u256());

        //assert_eq!((1u8,1u8).to_u256() * (1u8,1u8).to_u256(), (2u8,1u8).to_u256());

    
    }


    //#[test]
    fn time_it() {
        let start = Instant::now();
        let base: i128 = 2;
        let runs = u64::pow(base as u64,28)-1;
        let mut res = U256::ZERO;
        let mut x = U256::from(u128::MAX);
        //let mut x = U256::from(4 as u8);
        let y = U256::from( 100000 as u32);
        for _i in 0..3 {
            for _k in 0..runs {
                x = x + &y;
                //println!("{:?}", x);
                //if _k % 100000000 == 0 {println!("val: {}",x)}
            }
            //println!("Round num. {_i}");
            res = res + &x;
            println!("res: {:?}", res); // force use of calc. var to avoid optimization
        }

        
        let _end = Instant::now();
        let duration = start.elapsed();
        println!("Time elapsed: {:?}", duration);
    }
}

#[cfg(feature = "integer_snips")]
mod tests {
    #[test]
    fn test_karatsuba_u8(){
        println!("{:<9}{:<9}{:<9}{:<9}{:<9}{:<9}{:<9}{:<9}{:<9}{:<9}{:<9}{:<9}{:<9}{:<9}{:<9}{:<9}{:<9}",
        "x_in", "y_in", "x_lo", "x_hi", 
        "y_lo", "y_hi", "xy_lo", "xy_hi", 
        "x_c", "y_c", "xy_c_hi", "xy_c_lo", "cross", 
        "HI", "LO", "RES", "REAL"
        );
        use super::structs::U256::mul_karatsuba_u8;
        assert_eq!(mul_karatsuba_u8(1u8,1u8), 1u16);
        assert_eq!(mul_karatsuba_u8(1u8,0u8), 0u16);
        assert_eq!(mul_karatsuba_u8(0u8,1u8), 0u16);

        assert_eq!(mul_karatsuba_u8(u8::MAX,1u8), (u8::MAX as u16));
        assert_eq!(mul_karatsuba_u8(u8::MAX,2u8), (u8::MAX as u16)*2);
        assert_eq!(mul_karatsuba_u8(u8::MAX,4u8), (u8::MAX as u16)*4);
        assert_eq!(mul_karatsuba_u8(u8::MAX,127u8), (u8::MAX as u16)*127);
        assert_eq!(mul_karatsuba_u8(u8::MAX,128u8), (u8::MAX as u16)*128);
        assert_eq!(mul_karatsuba_u8(u8::MAX,u8::MAX), (u8::MAX as u16)*(u8::MAX as u16));
    }
    

}