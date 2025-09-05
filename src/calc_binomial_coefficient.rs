use crate::factorial::getfactorial;
use num_bigint::BigUint;

pub fn get_binomial_coefficient(n: u64, r: u64) -> BigUint {
    let mut res = BigUint::from(1_u64);

    for i in (r + 1)..=n {
        res *= BigUint::from(i);
    }

    return res / getfactorial(n - r);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_binomial_coefficient() {
        assert_eq!(get_binomial_coefficient(9, 3), BigUint::from(84_u64));
        assert_eq!(get_binomial_coefficient(9, 4), BigUint::from(126_u64));
        assert_eq!(get_binomial_coefficient(9, 5), BigUint::from(126_u64));
        assert_eq!(get_binomial_coefficient(9, 0), BigUint::from(1_u64));
        assert_eq!(get_binomial_coefficient(9, 9), BigUint::from(1_u64));
    }
}
