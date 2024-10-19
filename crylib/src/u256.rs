use core::ops;


pub struct U256 {
    higher: u128,
    lower: u128,
}

pub trait Initialize {
    fn to_u256(self) -> U256;
}
impl Initialize for (u128, u128) {
    fn to_u256(self) -> U256 {
        U256 {higher: self.0, lower: self.1}
    }
}
impl Initialize for u128 {
    fn to_u256(self) -> U256 {
        U256 {higher: 0, lower: self}
    }
}
// overloaded modular ADD operator
// todo: testing
impl ops::Add<U256> for U256 {
    type Output = U256;

    fn add(self, rhs: U256) -> U256 {
        let (lower, carr) = self.lower.overflowing_add(rhs.lower);
        let higher = self.higher + rhs.higher + carr as u128;
        (higher, lower).to_u256()
    }
}
// overloaded modular SUB operator
// todo: testing
impl ops::Sub<U256> for U256 {
    type Output = U256;
    fn sub(self, rhs: U256) -> U256 {
        let (lower, borr) = self.lower.overflowing_sub(rhs.lower);
        let higher = self.higher - rhs.higher - borr as u128;
        (higher, lower).to_u256()
    }
}

// overloaded modular MUL operator
impl ops::Mul<U256> for U256 {
    type Output = U256;
    fn mul(self, rhs: U256) -> U256 {

        
        let lower_lo = mul_u128(self.lower, rhs.lower);
        let lower_hi= mul_u128(self.lower,rhs.higher);
        let higher_lo = mul_u128(self.higher, rhs.lower);
        let higher_hi= mul_u128(self.higher,rhs.higher);


        let lower_total = 
        let higher_total = 

        (higher_total,lower_total).to_u256()
    }
}

// overflow safe u128 mul function
// todo: testing + simplification
fn mul_u128(x: u128, y: u128) -> U256 {
    let x_lower = x as u64;
    let x_higher = (x >> 64) as u64;
    let y_lower = y as u64;
    let y_higher = (y >> 64) as u64;

    let lower_l = (x_lower as u128) * (y_lower as u128);
    let lower_h = (x_lower as u128) * (y_higher as u128);
    let higher_l = (x_higher as u128) * (y_lower as u128);
    let higher_h = (x_higher as u128) * (y_higher as u128);

    let lower = lower_l;
    let mid_lower = (lower_l >> 64) + (lower_h & 0xFFFFFFFFFFFFFFFF) + (higher_l & 0xFFFFFFFFFFFFFFFF);
    let mid_higher = (mid_lower >> 64) + (higher_l >> 64) + (lower_h >> 64);
    let higher = higher_h + mid_higher;
    (higher, lower).to_u256()
}