use rayon::prelude::*;
mod u256;
#[cfg(test)]
mod tests {
    use super::*;

    use rand::Rng;
    use std::time::Instant;

    #[test]
    fn time_it() {
        let start = Instant::now();
        let base: i128 = 2;
        let RUNS = u64::pow(base as u64,33)-1;
        const ROUNDS: u8 = 3;
        let mut sum: i128 = 0;
        for _i in 0..ROUNDS {
            let partial_sum: i128 = (0..RUNS).into_par_iter().map(|_| {
                let mut rng = rand::thread_rng(); // Initialize random number generator inside the loop
                let rand_a: i128 = rng.gen_range(-i128::pow(base, 16)..=i128::pow(base, 16) - 1);
                let rand_b: i128 = rng.gen_range(-10..=10);
                let (a, b): (i128, i128) = (rand_a, rand_b);
                let res = egcd(a, b);
                res.0
            }).sum(); // Sum the results in parallel
            println!("partialSum: {:?}", partial_sum);
            sum += partial_sum as i128; // Aggregate the results
        }
        let end = Instant::now();
        let duration = start.elapsed();
        println!("Time elapsed: {:?}", duration);
        println!("Sum: {:?}", sum);
        assert!(duration > end.elapsed());
    }
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

