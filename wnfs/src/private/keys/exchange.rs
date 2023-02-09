use anyhow::Result;

#[cfg(test)]
use crate::RsaError;
#[cfg(test)]
use anyhow::anyhow;
#[cfg(test)]
use rsa::{BigUint, PaddingScheme, PublicKey as PublicKeyTrait, PublicKeyParts};
#[cfg(test)]
use sha2::Sha256;

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

pub const RSA_KEY_SIZE: usize = 2048;
pub const PUBLIC_KEY_EXPONENT: u64 = 65537;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// The `ExchangeKey` trait defines methods for creating an RSA public key from a modulus and encrypting data with the public key.
/// Implementations of this trait can create an RSA public key using the `from_exchange_key` method, which takes a modulus as input.
/// Data can be encrypted with the public key using the `encrypt` method, which takes a slice of bytes as input and returns the encrypted data as a vector of bytes.
///
/// More on that [here][key].
///
/// [key]: https://github.com/wnfs-wg/spec/blob/matheus23/file-sharding/spec/private-wnfs.md#314-private-file
pub trait ExchangeKey {
    /// Creates an RSA public key from the public key modulus.
    ///
    /// The exponent is expected to be of the value [`PUBLIC_KEY_EXPONENT`](constant.PUBLIC_KEY_EXPONENT.html) constant.
    fn from_exchange_key(modulus: &[u8]) -> Result<Self>
    where
        Self: Sized;

    /// Encrypts data with the public key.
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>>;
}

pub trait PrivateKey {
    /// Decrypts ciphertext with the private key.
    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>>;
}

pub type PublicKeyModulus = Vec<u8>;

#[cfg(test)]
#[derive(Debug, Clone)]
pub struct RsaPublicKey(rsa::RsaPublicKey);

#[cfg(test)]
#[derive(Debug, Clone)]
pub struct RsaPrivateKey(rsa::RsaPrivateKey);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
impl RsaPublicKey {
    /// Gets the public key modulus.
    pub fn get_public_key_modulus(&self) -> Result<Vec<u8>> {
        Ok(self.0.n().to_bytes_le())
    }
}

#[cfg(test)]
impl RsaPrivateKey {
    /// Constructs a new 2048-bit RSA private key.
    pub fn new() -> Result<Self> {
        Ok(Self(rsa::RsaPrivateKey::new(
            &mut rand::thread_rng(),
            RSA_KEY_SIZE,
        )?))
    }

    /// Gets the public key.
    pub fn get_public_key(&self) -> RsaPublicKey {
        RsaPublicKey(self.0.to_public_key())
    }
}

#[cfg(test)]
impl ExchangeKey for RsaPublicKey {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let padding = PaddingScheme::new_oaep::<Sha256>();
        self.0
            .encrypt(&mut rand::thread_rng(), padding, data)
            .map_err(|e| anyhow!(RsaError::EncryptionFailed(anyhow!(e))))
    }

    fn from_exchange_key(modulus: &[u8]) -> Result<Self> {
        let n = BigUint::from_bytes_le(modulus);
        let e = BigUint::from(PUBLIC_KEY_EXPONENT);

        Ok(Self(
            rsa::RsaPublicKey::new(n, e).map_err(|e| RsaError::InvalidPublicKey(anyhow!(e)))?,
        ))
    }
}

#[cfg(test)]
impl PrivateKey for RsaPrivateKey {
    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        let padding = PaddingScheme::new_oaep::<Sha256>();
        self.0
            .decrypt(padding, ciphertext)
            .map_err(|e| anyhow!(RsaError::DecryptionFailed(anyhow!(e))))
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rsa_key_pair() {
        let priv_key = RsaPrivateKey::new().unwrap();
        let pub_key = priv_key.get_public_key();

        let plaintext = b"Hello, world!";
        let ciphertext = pub_key.encrypt(plaintext).unwrap();
        let decrypted = priv_key.decrypt(&ciphertext).unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_rsa_key_pair_from_public_key_modulus() {
        let priv_key = RsaPrivateKey::new().unwrap();
        let pub_key = priv_key.get_public_key();

        let public_key_modulus = pub_key.get_public_key_modulus().unwrap();
        let key_pair_from_modulus = RsaPublicKey::from_exchange_key(&public_key_modulus).unwrap();

        let plaintext = b"Hello, world!";
        let ciphertext = key_pair_from_modulus.encrypt(plaintext).unwrap();
        let decrypted = priv_key.decrypt(&ciphertext).unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }
}
