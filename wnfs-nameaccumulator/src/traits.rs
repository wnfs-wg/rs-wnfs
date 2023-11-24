use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{BitOr, BitOrAssign, Mul, MulAssign, Rem, RemAssign},
    str::FromStr,
};

use num_bigint_dig::{prime::probably_prime, BigUint, ModInverse, RandBigInt, RandPrime};
use num_integer::Integer;
use num_traits::One;
use rand_core::CryptoRngCore;
use zeroize::Zeroize;

use crate::uint256_serde_be::to_bytes_helper;

/// Necessary implementations for the nameaccumulator backend that implements big integer math
pub trait Big: Eq + Clone {
    /// The big unsigned integer for this backend
    type Num: Clone
        + Debug
        + Display
        + PartialEq
        + Eq
        + Hash
        + Ord
        + Mul
        + for<'a> MulAssign<&'a Self::Num>
        + for<'a> RemAssign<&'a Self::Num>
        + for<'a> Rem<&'a Self::Num, Output = Self::Num>
        + One
        + BitOr
        + BitOrAssign
        + From<u8>
        + FromStr;

    fn modpow_product<'a>(
        base: &Self::Num,
        exponents: impl Iterator<Item = Self::Num>,
        modulus: &Self::Num,
    ) -> Self::Num;

    fn modpow(base: &Self::Num, exponent: &Self::Num, modulus: &Self::Num) -> Self::Num;

    fn mod_inv(base: &Self::Num, modulus: &Self::Num) -> Option<Self::Num>;

    fn squaremod(base: &Self::Num, modulus: &Self::Num) -> Self::Num;

    fn quotrem_product(
        factors: impl Iterator<Item = Self::Num>,
        divisor: &Self::Num,
    ) -> (Self::Num, Self::Num);

    fn from_bytes_le(bytes: &[u8]) -> Self::Num;

    fn to_bytes_le<const N: usize>(n: &Self::Num) -> [u8; N];

    fn from_bytes_be(bytes: &[u8]) -> Self::Num;

    fn to_256_bytes_be(n: &Self::Num) -> [u8; 256];

    fn is_probably_prime(candidate: &Self::Num) -> bool;

    fn rand_below(ceiling: &Self::Num, rng: &mut impl CryptoRngCore) -> Self::Num;

    fn rand_rsa_modulus(rng: &mut impl CryptoRngCore) -> Self::Num;

    fn rand_prime_256bit(rng: &mut impl CryptoRngCore) -> Self::Num;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigNumDig;

impl Big for BigNumDig {
    type Num = BigUint;

    fn modpow_product<'a>(
        base: &Self::Num,
        exponents: impl Iterator<Item = Self::Num>,
        modulus: &Self::Num,
    ) -> Self::Num {
        let mut product = BigUint::one();
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

    fn quotrem_product(
        factors: impl Iterator<Item = Self::Num>,
        divisor: &Self::Num,
    ) -> (Self::Num, Self::Num) {
        let mut product = BigUint::one();
        for factor in factors {
            product *= factor;
        }

        product.div_mod_floor(divisor)
    }

    fn from_bytes_le(bytes: &[u8]) -> Self::Num {
        BigUint::from_bytes_le(bytes)
    }

    fn to_bytes_le<const N: usize>(n: &Self::Num) -> [u8; N] {
        to_bytes_helper::<N>(n)
    }

    fn from_bytes_be(bytes: &[u8]) -> Self::Num {
        BigUint::from_bytes_be(bytes)
    }

    fn to_256_bytes_be(n: &Self::Num) -> [u8; 256] {
        to_bytes_helper(n)
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
