use num_bigint::{BigUint, RandBigInt};
use rand::Rng;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn gen_fermat(nbits: &u32) -> BigUint {
    let boundary = BigUint::new(vec![1]) >> nbits;
    boundary
}
