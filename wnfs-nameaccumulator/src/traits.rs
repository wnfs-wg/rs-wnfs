#[cfg(feature = "num-bigint-dig")]
use num_bigint_dig::{prime::probably_prime, BigUint, ModInverse, RandBigInt, RandPrime};
#[cfg(feature = "num-bigint-dig")]
use num_traits::{One, Zero};
use rand_core::CryptoRngCore;
#[cfg(feature = "rug")]
use rug::{
    integer::{IsPrime, Order},
    rand::RandState,
    Integer,
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

    /// Parses a little-endian-encoded number
    fn from_bytes_le(bytes: &[u8]) -> Self::Num;

    /// Turns a number into a little-encoded byte slice
    fn to_bytes_le<const N: usize>(n: &Self::Num) -> [u8; N];

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

    fn from_bytes_le(bytes: &[u8]) -> Self::Num {
        BigUint::from_bytes_le(bytes)
    }

    fn to_bytes_le<const N: usize>(n: &Self::Num) -> [u8; N] {
        let vec = n.to_bytes_le();
        let mut bytes = [0u8; N];
        bytes.copy_from_slice(&vec);
        bytes
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

    fn from_bytes_le(bytes: &[u8]) -> Self::Num {
        Integer::from_digits(bytes, Order::LsfLe)
    }

    fn to_bytes_le<const N: usize>(n: &Self::Num) -> [u8; N] {
        let vec = n.to_digits(Order::LsfLe);
        let mut bytes = [0u8; N];
        bytes.copy_from_slice(&vec);
        bytes
    }

    fn from_bytes_be(bytes: &[u8]) -> Self::Num {
        Integer::from_digits(bytes, Order::MsfBe)
    }

    fn to_bytes_be<const N: usize>(n: &Self::Num) -> [u8; N] {
        let vec = n.to_digits(Order::MsfBe);
        let mut bytes = [0u8; N];
        let zero_bytes = N - vec.len();
        bytes[zero_bytes..].copy_from_slice(&vec);
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
        let p: Integer = Integer::random_bits(1024, &mut rng).into();
        let q: Integer = Integer::random_bits(1024, &mut rng).into();
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
    fn setup_rand_state(rng: &mut impl CryptoRngCore) -> RandState {
        let mut seed = [0u8; 32];
        rng.fill_bytes(&mut seed);
        let seed = Integer::from_digits(&seed, Order::LsfLe);
        let mut rng = RandState::new();
        rng.seed(&seed);
        rng
    }
}
