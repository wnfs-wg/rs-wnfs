use blake3::traits::digest::{ExtendableOutput, ExtendableOutputReset};
use num_bigint_dig::{prime::probably_prime, BigUint};
use num_traits::One;

#[cfg(test)]
const TEST_DSI: &str = "rs-wnfs tests";

/// Computes the function "MultiExp" from the paper
/// "Batching Techniques for Accumulators with Applications to IOPs and Stateless Blockchains"
/// (https://eprint.iacr.org/2018/1188.pdf), Section 3.3
///
/// With `(base_i, exponent_i) = bases_and_exponents_i`, it computes the product of
/// `base_i ^ (product of exponent_j with j != i)`.
pub(crate) fn multi_exp(bases_and_exponents: &[(BigUint, BigUint)], modulus: &BigUint) -> BigUint {
    match bases_and_exponents {
        &[] => BigUint::one(),
        [(base, _)] => base.clone() % modulus,
        other => {
            let mid = other.len() / 2;
            let (left, right) = other.split_at(mid);
            let x_star_left = nlogn_product(left, |(_, x_i)| x_i);
            let x_star_right = nlogn_product(right, |(_, x_i)| x_i);
            (multi_exp(left, modulus).modpow(&x_star_right, modulus)
                * multi_exp(right, modulus).modpow(&x_star_left, modulus))
                % modulus
        }
    }
}

/// Computes the product of all factors in O(n log n) time.
pub(crate) fn nlogn_product<A>(factors: &[A], f: fn(&A) -> &BigUint) -> BigUint {
    match factors {
        [] => BigUint::one(),
        [factor] => f(factor).clone(),
        other => {
            let mid = other.len() / 2;
            let (left, right) = factors.split_at(mid);
            nlogn_product(left, f) * nlogn_product(right, f)
        }
    }
}

/// Finalizes a hashing function to a 128-bit prime number.
///
/// The output includes both the prime and a 32-bit counter
/// that helps verifying the prime digest.
pub(crate) fn blake3_prime_digest(
    domain_separation_info: &str,
    bytes: impl AsRef<[u8]>,
    hash_len: usize,
) -> (BigUint, u32) {
    let mut counter: u32 = 0;
    let mut hasher = blake3::Hasher::new_derive_key(domain_separation_info);
    let mut hash = vec![0u8; hash_len];
    loop {
        // We reuse the same `Hasher` struct between iterations to minimize
        // stack usage. Each `Hasher` allocation is ~2kB for Blake3.
        hasher.update(bytes.as_ref());
        hasher.update(&counter.to_le_bytes());
        hasher.finalize_xof_reset_into(&mut hash);

        let mut candidate = BigUint::from_bytes_le(&hash);

        candidate |= BigUint::one();

        if probably_prime(&candidate, 20) {
            return (candidate, counter);
        }

        counter += 1;
    }
}

/// Finalizes a digest fast, if it has been computed before given the counter from
/// a previous invocation of `prime_digest`.
/// This will make sure that the returned digest is prime.
pub(crate) fn blake3_prime_digest_fast(
    domain_separation_info: &str,
    bytes: impl AsRef<[u8]>,
    hash_len: usize,
    counter: u32,
) -> Option<BigUint> {
    let mut hash = vec![0u8; hash_len];
    let mut hasher = blake3::Hasher::new_derive_key(domain_separation_info);
    hasher.update(bytes.as_ref());
    hasher.update(&counter.to_le_bytes());
    hasher.finalize_xof_into(&mut hash);

    let mut to_verify = BigUint::from_bytes_le(&hash);
    to_verify |= BigUint::one();

    if !probably_prime(&to_verify, 20) {
        None
    } else {
        Some(to_verify)
    }
}

#[cfg(test)]
mod tests {
    use super::{blake3_prime_digest, TEST_DSI};

    /// This test makes sure we don't accidentally (only intentionally)
    /// change hash outputs between versions.
    #[test]
    fn test_fixture_prime_hash() {
        let (output, counter) = blake3_prime_digest(TEST_DSI, "Hello, World!", 16);
        assert_eq!(output.to_str_radix(16), "9d139eb0bf1705f72c5a61973b1f92a3");
        assert_eq!(counter, 13);
    }
}

#[cfg(test)]
mod proptests {
    use crate::fns::{
        blake3_prime_digest, blake3_prime_digest_fast, multi_exp, nlogn_product, TEST_DSI,
    };
    use num_bigint_dig::{prime::probably_prime, BigUint, RandPrime};
    use num_traits::One;
    use proptest::{
        collection::vec, prelude::any, prop_assert, prop_assert_eq, strategy::Strategy,
    };
    use rand_chacha::ChaCha12Rng;
    use rand_core::SeedableRng;
    use test_strategy::proptest;

    #[proptest(cases = 1000)]
    fn test_prime_digest(#[strategy(vec(any::<u8>(), 0..100))] bytes: Vec<u8>) {
        let (prime_hash, inc) = blake3_prime_digest(TEST_DSI, &bytes, 16);
        prop_assert!(probably_prime(&prime_hash, 20));
        prop_assert_eq!(
            blake3_prime_digest_fast(TEST_DSI, &bytes, 16, inc),
            Some(prime_hash)
        );
    }

    #[proptest(cases = 100)]
    fn test_multi_exp(
        #[strategy(vec((1u64.., 1u64..), 0..100))] bases_and_exponents: Vec<(u64, u64)>,
        #[strategy(rand_modulus(4usize..64))] modulus: BigUint,
    ) {
        let bases_and_exponents: Vec<(BigUint, BigUint)> = bases_and_exponents
            .iter()
            .map(|(b, e)| (BigUint::from(*b), BigUint::from(*e)))
            .collect();

        let actual = multi_exp(&bases_and_exponents, &modulus);
        let expected = multi_exp_naive(&bases_and_exponents, &modulus);
        prop_assert_eq!(actual, expected);
    }

    fn multi_exp_naive(bases_and_exponents: &[(BigUint, BigUint)], modulus: &BigUint) -> BigUint {
        let x_star = nlogn_product(bases_and_exponents, |(_, x_i)| x_i);

        let mut product = BigUint::one();
        for (alpha_i, x_i) in bases_and_exponents {
            let exponent = &x_star / x_i;
            product *= alpha_i.modpow(&exponent, modulus);
            product %= modulus;
        }
        product
    }

    fn rand_modulus(bits: impl Strategy<Value = usize>) -> impl Strategy<Value = BigUint> {
        (bits, any::<[u8; 32]>().no_shrink()).prop_map(move |(bits, seed)| {
            let bits = std::cmp::max(bits, 4);
            let rng = &mut ChaCha12Rng::from_seed(seed);
            rng.gen_prime(bits / 2) * rng.gen_prime(bits / 2)
        })
    }
}
