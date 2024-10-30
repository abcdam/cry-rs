pub mod fw;
pub mod dynamic;
pub trait HexString {
    #[allow(dead_code)]
    fn to_hex_str(&self) -> String;
}
trait UPrimitive: Into<u128> + Copy {}
impl UPrimitive for u128 {}
impl UPrimitive for u64 {}
impl UPrimitive for u32 {}
impl UPrimitive for u16 {}
impl UPrimitive for u8 {}


#[cfg(test)]
mod tests {
    use super::fw::u_256::*;
    use super::dynamic::u_256::*;
    use super::HexString;
    use num_bigint::BigUint;
    use std::time::Instant;

    impl HexString for BigUint {
        fn to_hex_str(&self) -> String {
            format!("0x{:064x}", self)
        }
    }

    #[test]
    fn test_u256_init() {
        let u_zero: u8 = 0;
        assert_eq!((u_zero as u128).to_u256(), (u_zero as u64).to_u256());
        assert_eq!((u_zero as u64).to_u256(), (u_zero as u32).to_u256());
        assert_eq!((u_zero as u32).to_u256(), (u_zero as u16).to_u256());
        assert_eq!((u_zero as u16).to_u256(), (u_zero).to_u256());

        assert_eq!(U256::from(u_zero), U256::from(u_zero as u16));
        assert_eq!(U256::from(u_zero as u16), U256::from(u_zero as u32));
        assert_eq!(U256::from(u_zero as u32), U256::from(u_zero as u64));
        assert_eq!(U256::from(u_zero as u64), U256::from(u_zero as u128));
    }
    #[test]
    fn test_u256_add() {
        let u_max = u128::MAX;
        let zero_256 = U256::from(0u8);
        let one_256 = U256::from(1u8);
        let max_rhs_256 = U256::from(u_max);
        let max_lhs_256 = (u_max, 0u8).to_u256();

        // sanity check
        assert_ne!(zero_256, one_256);
        assert_ne!(max_rhs_256, max_lhs_256);
        assert_eq!(&max_lhs_256 + &max_rhs_256, U256::max());
        assert_eq!(&one_256 + &one_256, U256::from(2 as u8));

        // some checks for
        //      abelian group props: associative, id, inverse, commutative
        assert_eq!(one_256, &one_256 + &zero_256);
        assert_eq!(one_256, &zero_256 + &one_256);
        assert_eq!(one_256, &zero_256 + 1u8);
        assert_eq!(
            (&one_256 + &max_rhs_256) + &max_lhs_256,
            &one_256 + (&max_rhs_256 + &max_lhs_256)
        );
        assert_eq!(
            one_256,
            &U256::from(2 * 1u8) + (&max_rhs_256 + &max_lhs_256)
        );

        // modularity/edgecase/closure check
        assert_eq!((1u8, 0u8).to_u256(), &one_256 + &max_rhs_256); // all rhs bits flip to zero and LSB in lhs flips to one
        assert_eq!((u_max, u_max).to_u256(), &max_rhs_256 + &max_lhs_256);
        assert_eq!((u_max, 1u8).to_u256(), &max_lhs_256 + &one_256);
        assert_eq!(zero_256, (max_lhs_256 + max_rhs_256) + one_256); // 1 added to MAX_VAL of type u256 wraps around to 0
        assert_eq!(
            U256::from(20 as u8),
            (u_max, u_max).to_u256() + U256::from(21 as u8)
        );
    }
    #[test]
    fn test_u256_sub() {
        let u_zero: u8 = 0;
        let u_one: u8 = 1;
        let zero_256 = U256::from(u_zero);
        let one_256 = U256::from(u_one);
        let max_rhs_256 = U256::from(u128::MAX);
        let max_lhs_256 = (u128::MAX, u_zero).to_u256();

        // rhs neutral element
        assert_eq!(one_256, &one_256 - &zero_256);
        assert_eq!(zero_256, &zero_256 - &zero_256);

        // modularity/edgecase/closure check
        assert_eq!(&max_lhs_256 + &max_rhs_256, &zero_256 - &one_256);
        assert_eq!(max_rhs_256, (u_one, u_zero).to_u256() - &one_256);
        assert_eq!(one_256, &zero_256 - (&max_lhs_256 + &max_rhs_256));
        assert_eq!(U256::max(), -U256::from(1 as u8));
        assert_eq!(zero_256, -U256::from(1 as u8) + &one_256);
        assert_eq!(zero_256, -&max_rhs_256 + max_rhs_256);
    }

    #[test]
    fn test_u256_mul() {
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
        let (lhs, rhs) = (123456789u32, 987654321u32);
        let t_1 = (lhs, rhs).to_u256();
        let t_2: BigU256 =
            (BigUint::from(lhs as u32) << 128u8).to_big_u256() + (BigU256::from(rhs));
        assert_eq!(t_1.to_hex_str(), (t_1 * one_256).to_hex_str());
        assert_eq!(t_1, one_256 * t_1);
        assert_eq!(t_1.to_hex_str(), t_2.to_hex_str());

        assert_eq!(
            (U256::MAX * U256::MAX).to_hex_str(),
            U256::from(1u8).to_hex_str()
        );
        assert_eq!(
            (U256::MAX * U256::MAX).to_hex_str(),
            (u128::MAX, u128::MAX)
                .to_big_u256()
                .modpow(&BigUint::from(2u8), &(BigU256::max() + 1u8))
                .to_hex_str()
        );
        assert_eq!(
            U256::MAX * (u128::MAX, u128::MAX - 1).to_u256(),
            U256::from(2u8)
        );

        assert_eq!(
            (1u8, 1u8).to_u256() * (1u8, 1u8).to_u256(),
            (2u8, 1u8).to_u256()
        );
    }

    #[test]
    fn test_u256_mul_rand() {
        // test 32kkk combinations using a seed for deterministic randomness
        use rand::distributions::{Distribution, Uniform};
        use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
        use rayon::prelude::*;
        use std::sync::atomic::{AtomicUsize, Ordering};

        static SEED: u64 = 33;
        static SUBBATCH_SIZE: usize = 100_000_000; // ~ 3,2 GiB/subbatch
        static BATCHES: u64 = 32;
        static SUBBATCHES: u64 = 10;
        let interval = Uniform::new(0u128, u128::MAX);
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let start = Instant::now();
        (0..BATCHES).into_par_iter().for_each(|i| {
            let mut rng = ChaCha8Rng::seed_from_u64(SEED);
            rng.set_stream(i);
            for _ in 0..SUBBATCHES {
                // initialize in-memory list of #SUBBATCH_SIZE random numbers taken from unifrom distribution
                // -> takes a lot of RAM space but speeds up inner loop.
                let rand_values: Vec<u128> = interval
                    .sample_iter(&mut rng)
                    .take(SUBBATCH_SIZE as usize)
                    .collect();
                for i in 3..SUBBATCH_SIZE {
                    let (lhs, rhs) = (
                        (rand_values[i], rand_values[i - 1]),
                        (rand_values[i - 2], rand_values[i - 3]),
                    );
                    // to test integrity
                    let expected = lhs.to_big_u256() * rhs.to_big_u256();
                    // to test speed
                    //let expected = rhs.to_u256() * lhs.to_u256();
                    let got = lhs.to_u256() * rhs.to_u256();

                    assert_eq!(
                        expected.to_fixed_u256(),
                        got,
                        "lhs:0x{:032x}{:032x} rhs:0x{:032x}{:032x}\nexpected: {}\ngot: {}",
                        lhs.0,
                        lhs.1,
                        rhs.0,
                        rhs.1,
                        expected.to_hex_str(),
                        got.to_hex_str()
                    );
                }
                let count = COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
                println!("Progress: {}/{}", count, SUBBATCHES * BATCHES);
            }
        });
        println!("Finished deterministic random sampling U256 multiplication test.");
        let duration = start.elapsed();
        println!("Time elapsed: {:?}", duration);
    }
}
