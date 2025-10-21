#[cfg(feature = "num-bigint-dig")]
use num_bigint_dig::{BigUint, ModInverse, RandBigInt, RandPrime, prime::probably_prime};
use num_traits::{One, Zero};
use rand_core::CryptoRngCore;
#[cfg(feature = "rug")]
use rug::{
    Integer,
    integer::{IsPrime, Order},
    rand::RandState,
};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{BitOrAssign, MulAssign, Rem, RemAssign},
    str::FromStr,
};
use wnfs_common::utils::CondSync;
#[cfg(feature = "num-bigint-dig")]
use zeroize::Zeroize;

/// Big integer math functions required to implement name accumulators.
///
/// This allows abstracting the big number library backend.
pub trait Big: Eq + Clone + Hash {
    /// The big unsigned integer for this backend
    type Num: Clone
        + Debug
        + Display
        + PartialEq
        + Eq
        + Hash
        + CondSync
        // number-related traits
        + FromStr
        + Zero
        + One
        + for<'a> MulAssign<&'a Self::Num>
        + for<'a> RemAssign<&'a Self::Num>
        + for<'a> Rem<&'a Self::Num, Output = Self::Num>
        + BitOrAssign
        + Ord;

    /// Computes the power of base to the product of some numbers all under a modulus.
    ///
    /// `modpow_product(b, exps, N) = b ^ product(exps) mod N`
    fn modpow_product<'a>(
        base: &Self::Num,
        exponents: impl Iterator<Item = &'a Self::Num>,
        modulus: &Self::Num,
    ) -> Self::Num
    where
        Self::Num: 'a;

    /// Computes `(base ^ exponent) mod modulus`
    fn modpow(base: &Self::Num, exponent: &Self::Num, modulus: &Self::Num) -> Self::Num;

    /// Computes the modulo multiplicative inverse of `base`.
    ///
    /// `(mod_inv(base, N) * base) mod N = 1`
    fn mod_inv(base: &Self::Num, modulus: &Self::Num) -> Option<Self::Num>;

    /// Computes `(base ^ 2) mod N`.
    ///
    /// A specialization of `modpow` with `exponent = 2`.
    fn squaremod(base: &Self::Num, modulus: &Self::Num) -> Self::Num;

    /// Computes the quotient and remainder of a product of numbers divided
    /// by `divisor`.
    ///
    /// `quotrem_product(fs, d).0 * d + quotrem_product(fs, d).1 = product(fs)`
    ///
    /// Returns `(quotient, remainder)`.
    fn quotrem_product<'a>(
        factors: impl Iterator<Item = &'a Self::Num>,
        divisor: &Self::Num,
    ) -> (Self::Num, Self::Num)
    where
        Self::Num: 'a;

    /// Parses a big-endian-encoded number
    ///
    /// (Big endian is the standard encoding for RSA numbers.)
    fn from_bytes_be(bytes: &[u8]) -> Self::Num;

    /// Turns a 2048-bit number into a big-endian encoded slice.
    ///
    /// (Big endian is the standard encoding for RSA numbers.)
    fn to_bytes_be<const N: usize>(n: &Self::Num) -> [u8; N];

    /// Returns whether given number is probably prime.
    /// The propability of the number being non-prime must be neglegible.
    fn is_probably_prime(candidate: &Self::Num) -> bool;

    /// Generates a random number less than the non-zero ceiling, given some randomness.
    fn rand_below(ceiling: &Self::Num, rng: &mut impl CryptoRngCore) -> Self::Num;

    /// Generate a random modulus `N = p * q` where `p` and `q` are prime.
    fn rand_rsa_modulus(rng: &mut impl CryptoRngCore) -> Self::Num;

    /// Generate a random 256-bit prime number.
    fn rand_prime_256bit(rng: &mut impl CryptoRngCore) -> Self::Num;
}

#[cfg(feature = "num-bigint-dig")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BigNumDig;

#[cfg(feature = "num-bigint-dig")]
impl Big for BigNumDig {
    type Num = BigUint;

    fn modpow_product<'a>(
        base: &Self::Num,
        exponents: impl Iterator<Item = &'a Self::Num>,
        modulus: &Self::Num,
    ) -> Self::Num {
        let mut product = Self::Num::one();
        for exponent in exponents {
            product *= exponent;
        }

        base.modpow(&product, modulus)
    }

    fn modpow(base: &Self::Num, exponent: &Self::Num, modulus: &Self::Num) -> Self::Num {
        base.modpow(exponent, modulus)
    }

    fn mod_inv(base: &Self::Num, modulus: &Self::Num) -> Option<Self::Num> {
        base.mod_inverse(modulus)?.to_biguint()
    }

    fn squaremod(base: &Self::Num, modulus: &Self::Num) -> Self::Num {
        base.modpow(&BigUint::from(2u8), modulus)
    }

    fn quotrem_product<'a>(
        factors: impl Iterator<Item = &'a Self::Num>,
        divisor: &Self::Num,
    ) -> (Self::Num, Self::Num) {
        let mut product = BigUint::one();
        for factor in factors {
            product *= factor;
        }

        use num_integer::Integer;
        product.div_mod_floor(divisor)
    }

    fn from_bytes_be(bytes: &[u8]) -> Self::Num {
        BigUint::from_bytes_be(bytes)
    }

    fn to_bytes_be<const N: usize>(n: &Self::Num) -> [u8; N] {
        let vec = n.to_bytes_be();
        let mut bytes = [0u8; N];
        let zero_bytes = N - vec.len();
        bytes[zero_bytes..].copy_from_slice(&vec);
        bytes
    }

    fn is_probably_prime(candidate: &Self::Num) -> bool {
        probably_prime(candidate, 20)
    }

    fn rand_below(ceiling: &Self::Num, rng: &mut impl CryptoRngCore) -> Self::Num {
        rng.gen_biguint_below(ceiling)
    }

    fn rand_rsa_modulus(rng: &mut impl CryptoRngCore) -> Self::Num {
        // This is a trusted setup.
        // The prime factors are "toxic waste", they need to be
        // disposed immediately.
        let mut p = rng.gen_prime(1024);
        let mut q = rng.gen_prime(1024);
        let modulus = &p * &q;
        // Make sure to delete toxic waste
        p.zeroize();
        q.zeroize();
        modulus
    }

    fn rand_prime_256bit(rng: &mut impl CryptoRngCore) -> Self::Num {
        rng.gen_prime(256)
    }
}

#[cfg(feature = "rug")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BigNumRug;

#[cfg(feature = "rug")]
impl Big for BigNumRug {
    type Num = Integer;

    fn modpow_product<'a>(
        base: &Self::Num,
        exponents: impl Iterator<Item = &'a Self::Num>,
        modulus: &Self::Num,
    ) -> Self::Num {
        let mut product = Self::Num::one();
        for exponent in exponents {
            product *= exponent;
        }

        base.clone().secure_pow_mod(&product, modulus)
    }

    fn modpow(base: &Self::Num, exponent: &Self::Num, modulus: &Self::Num) -> Self::Num {
        base.clone().secure_pow_mod(exponent, modulus)
    }

    fn mod_inv(base: &Self::Num, modulus: &Self::Num) -> Option<Self::Num> {
        base.clone().invert(modulus).ok()
    }

    fn squaremod(base: &Self::Num, modulus: &Self::Num) -> Self::Num {
        base.clone()
            .pow_mod(&Integer::from_digits(&[2u8], Order::LsfLe), modulus)
            .unwrap()
    }

    fn quotrem_product<'a>(
        factors: impl Iterator<Item = &'a Self::Num>,
        divisor: &Self::Num,
    ) -> (Self::Num, Self::Num) {
        let mut product = Self::Num::one();
        for factor in factors {
            product *= factor;
        }

        product.div_rem_floor(divisor.clone())
    }

    fn from_bytes_be(bytes: &[u8]) -> Self::Num {
        Integer::from_digits(bytes, Order::MsfLe)
    }

    fn to_bytes_be<const N: usize>(n: &Self::Num) -> [u8; N] {
        let mut bytes = [0u8; N];
        n.write_digits(&mut bytes, Order::MsfLe);
        bytes
    }

    fn is_probably_prime(candidate: &Self::Num) -> bool {
        match candidate.is_probably_prime(20) {
            IsPrime::No => false,
            IsPrime::Probably => true,
            IsPrime::Yes => true,
        }
    }

    fn rand_below(ceiling: &Self::Num, rng: &mut impl CryptoRngCore) -> Self::Num {
        let mut rng = Self::setup_rand_state(rng);
        Integer::random_below(ceiling.clone(), &mut rng)
    }

    fn rand_rsa_modulus(rng: &mut impl CryptoRngCore) -> Self::Num {
        let mut rng = Self::setup_rand_state(rng);
        let p_pre: Integer = Integer::random_bits(1024, &mut rng).into();
        let q_pre: Integer = Integer::random_bits(1024, &mut rng).into();
        let p = p_pre.next_prime();
        let q = q_pre.next_prime();
        p * q
    }

    fn rand_prime_256bit(rng: &mut impl CryptoRngCore) -> Self::Num {
        let i: Integer = Integer::random_bits(256, &mut Self::setup_rand_state(rng)).into();
        let prime = i.next_prime();
        debug_assert!(prime.is_positive());
        prime
    }
}

#[cfg(feature = "rug")]
impl BigNumRug {
    fn setup_rand_state(rng: &mut impl CryptoRngCore) -> RandState<'_> {
        let mut seed = [0u8; 32];
        rng.fill_bytes(&mut seed);
        let seed = Integer::from_digits(&seed, Order::LsfLe);
        let mut rng = RandState::new();
        rng.seed(&seed);
        rng
    }
}

#[cfg(feature = "rug")]
#[cfg(test)]
mod rug_tests {
    use crate::{Big, BigNumRug};
    use rand_chacha::ChaCha12Rng;
    use rand_core::SeedableRng;
    use rug::Integer;
    use std::str::FromStr;

    /// We need this property for snapshot testing
    #[test]
    fn rand_prime_is_deterministic() {
        let run_one = BigNumRug::rand_prime_256bit(&mut ChaCha12Rng::seed_from_u64(0));
        let run_two = BigNumRug::rand_prime_256bit(&mut ChaCha12Rng::seed_from_u64(0));
        assert_eq!(run_one, run_two);
        let run_three = BigNumRug::rand_prime_256bit(&mut ChaCha12Rng::seed_from_u64(1));
        assert_ne!(run_one, run_three);
    }

    /// We need this property for snapshot testing
    #[test]
    fn rand_below_is_deterministic() {
        let ceiling = BigNumRug::rand_prime_256bit(&mut ChaCha12Rng::seed_from_u64(0));
        let run_one = BigNumRug::rand_below(&ceiling, &mut ChaCha12Rng::seed_from_u64(0));
        let run_two = BigNumRug::rand_below(&ceiling, &mut ChaCha12Rng::seed_from_u64(0));
        assert_eq!(run_one, run_two);
        let run_three = BigNumRug::rand_below(&ceiling, &mut ChaCha12Rng::seed_from_u64(1));
        assert_ne!(run_one, run_three);
    }

    #[test]
    fn test_to_bytes_be_snapshot() {
        let modulus = Integer::from_str(
            "25195908475657893494027183240048398571429282126204032027777137836043662020707595556264018525880784406918290641249515082189298559149176184502808489120072844992687392807287776735971418347270261896375014971824691165077613379859095700097330459748808428401797429100642458691817195118746121515172654632282216869987549182422433637259085141865462043576798423387184774447920739934236584823824281198163815010674810451660377306056201619676256133844143603833904414952634432190114657544454178424020924616515723350778707749817125772467962926386356373289912154831438167899885040445364023527381951378636564391212010397122822120720357",
        ).expect("Can parse integer");

        let modulus_hex = "c7970ceedcc3b0754490201a7aa613cd73911081c790f5f1a8726f463550bb5b7ff0db8e1ea1189ec72f93d1650011bd721aeeacc2acde32a04107f0648c2813a31f5b0b7765ff8b44b4b6ffc93384b646eb09c7cf5e8592d40ea33c80039f35b4f14a04b51f7bfd781be4d1673164ba8eb991c2c4d730bbbe35f592bdef524af7e8daefd26c66fc02c479af89d64d373f442709439de66ceb955f3ea37d5159f6135809f85334b5cb1813addc80cd05609f10ac6a95ad65872c909525bdad32bc729592642920f24c61dc5b3c3b7923e56b16a4d9d373d8721f24a3fc0f1b3131f55615172866bccc30f95054c824e733a5eb6817f7bc16399d48c6361cc7e5";

        let bytes = BigNumRug::to_bytes_be::<256>(&modulus);
        let hex_encoded = hex::encode(bytes);

        assert_eq!(hex_encoded, modulus_hex);
    }
}
