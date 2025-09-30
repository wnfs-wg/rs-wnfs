use crate::Big;
use blake3::traits::digest::{ExtendableOutput, ExtendableOutputReset};
use num_traits::One;

#[cfg(test)]
const TEST_DSI: &str = "rs-wnfs tests";

/// Computes the function "MultiExp" from the paper
/// "Batching Techniques for Accumulators with Applications to IOPs and Stateless Blockchains"
/// (https://eprint.iacr.org/2018/1188.pdf), Section 3.3
///
/// With `(base_i, exponent_i) = bases_and_exponents_i`, it computes the product of
/// `base_i ^ (product of exponent_j with j != i)`.
pub(crate) fn multi_exp<B: Big>(
    bases_and_exponents: &[(B::Num, B::Num)],
    modulus: &B::Num,
) -> B::Num {
    match bases_and_exponents {
        &[] => B::Num::one(),
        [(base, _)] => base.clone() % modulus,
        other => {
            let mid = other.len() / 2;
            let (left, right) = other.split_at(mid);
            let x_star_left = nlogn_product::<_, B>(left, |(_, x_i)| x_i);
            let x_star_right = nlogn_product::<_, B>(right, |(_, x_i)| x_i);
            (B::modpow(&multi_exp::<B>(left, modulus), &x_star_right, modulus)
                * B::modpow(&multi_exp::<B>(right, modulus), &x_star_left, modulus))
                % modulus
        }
    }
}

/// Computes the product of all factors in O(n log n) time.
pub(crate) fn nlogn_product<A, B: Big>(factors: &[A], f: fn(&A) -> &B::Num) -> B::Num {
    match factors {
        [] => B::Num::one(),
        [factor] => f(factor).clone(),
        other => {
            let mid = other.len() / 2;
            let (left, right) = factors.split_at(mid);
            nlogn_product::<A, B>(left, f) * nlogn_product::<A, B>(right, f)
        }
    }
}

/// Finalizes a hashing function to a 128-bit prime number.
///
/// The output includes both the prime and a 32-bit counter
/// that helps verifying the prime digest.
pub(crate) fn blake3_prime_digest<B: Big>(
    domain_separation_info: &str,
    bytes: impl AsRef<[u8]>,
    hash_len: usize,
) -> (B::Num, u32) {
    let mut counter: u32 = 0;
    let mut hasher = blake3::Hasher::new_derive_key(domain_separation_info);
    let mut hash = vec![0u8; hash_len];
    loop {
        // We reuse the same `Hasher` struct between iterations to minimize
        // stack usage. Each `Hasher` allocation is ~2kB for Blake3.
        hasher.update(bytes.as_ref());
        hasher.update(&counter.to_le_bytes());
        hasher.finalize_xof_reset_into(&mut hash);

        let mut candidate = B::from_bytes_be(&hash);

        candidate |= B::Num::one();

        if B::is_probably_prime(&candidate) {
            return (candidate, counter);
        }

        counter += 1;
    }
}

/// Finalizes a digest fast, if it has been computed before given the counter from
/// a previous invocation of `prime_digest`.
/// This will make sure that the returned digest is prime.
pub(crate) fn blake3_prime_digest_fast<B: Big>(
    domain_separation_info: &str,
    bytes: impl AsRef<[u8]>,
    hash_len: usize,
    counter: u32,
) -> Option<B::Num> {
    let mut hash = vec![0u8; hash_len];
    let mut hasher = blake3::Hasher::new_derive_key(domain_separation_info);
    hasher.update(bytes.as_ref());
    hasher.update(&counter.to_le_bytes());
    hasher.finalize_xof_into(&mut hash);

    let mut to_verify = B::from_bytes_be(&hash);
    to_verify |= B::Num::one();

    if !B::is_probably_prime(&to_verify) {
        None
    } else {
        Some(to_verify)
    }
}

#[cfg(test)]
mod tests {
    use super::{TEST_DSI, blake3_prime_digest};
    use crate::BigNumDig;

    /// This test makes sure we don't accidentally (only intentionally)
    /// change hash outputs between versions.
    #[test]
    fn test_fixture_prime_hash() {
        let (output, counter) = blake3_prime_digest::<BigNumDig>(TEST_DSI, "Hello, World!", 16);
        assert_eq!(
            (output.to_str_radix(16), counter),
            ("7f1f785675ccdf2fb20238124fe3e80f".into(), 23)
        );
    }
}

#[cfg(test)]
mod proptests {
    use crate::{
        BigNumDig,
        fns::{TEST_DSI, blake3_prime_digest, blake3_prime_digest_fast, multi_exp, nlogn_product},
    };
    use num_bigint_dig::{BigUint, RandPrime, prime::probably_prime};
    use num_traits::One;
    use proptest::{
        collection::vec, prelude::any, prop_assert, prop_assert_eq, strategy::Strategy,
    };
    use rand_chacha::ChaCha12Rng;
    use rand_core::SeedableRng;
    use test_strategy::proptest;

    #[proptest(cases = 1000)]
    fn test_prime_digest(#[strategy(vec(any::<u8>(), 0..100))] bytes: Vec<u8>) {
        let (prime_hash, inc) = blake3_prime_digest::<BigNumDig>(TEST_DSI, &bytes, 16);
        prop_assert!(probably_prime(&prime_hash, 20));
        prop_assert_eq!(
            blake3_prime_digest_fast::<BigNumDig>(TEST_DSI, &bytes, 16, inc),
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

        let actual = multi_exp::<BigNumDig>(&bases_and_exponents, &modulus);
        let expected = multi_exp_naive(&bases_and_exponents, &modulus);
        prop_assert_eq!(actual, expected);
    }

    fn multi_exp_naive(bases_and_exponents: &[(BigUint, BigUint)], modulus: &BigUint) -> BigUint {
        let x_star = nlogn_product::<_, BigNumDig>(bases_and_exponents, |(_, x_i)| x_i);

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
