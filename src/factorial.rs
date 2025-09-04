use num_bigint::BigUint;

pub fn getfactorial(n: u64) -> BigUint {
    // TODO: 並列化して高速化する
    if n == 0 {
        return BigUint::from(1_u64);
    }

    let mut result = BigUint::from(1_u64);

    for i in 2..=n {
        result *= BigUint::from(i);
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(getfactorial(5), BigUint::from(120_u64));
        assert_eq!(getfactorial(0), BigUint::from(1_u64));
    }
}
