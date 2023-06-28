#[cfg(test)]
use crate::error::RsaError;
#[cfg(test)]
use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use rsa::{traits::PublicKeyParts, BigUint, Oaep};
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
/// Implementations of this trait can create an RSA public key using the `from_modulus` method, which takes a modulus as input.
///
/// Data can be encrypted with the public key using the `encrypt` method, which takes a slice of bytes as input and returns the encrypted data as a vector of bytes.
///
/// More on exchange keys [here][key].
///
/// [key]: https://github.com/wnfs-wg/spec/blob/main/spec/shared-private-data.md#2-exchange-keys-partition
#[async_trait(?Send)]
pub trait ExchangeKey {
    /// Creates an RSA public key from the public key modulus.
    ///
    /// The exponent is expected to be of the value [`PUBLIC_KEY_EXPONENT`](constant.PUBLIC_KEY_EXPONENT.html) constant.
    async fn from_modulus(modulus: &[u8]) -> Result<Self>
    where
        Self: Sized;

    /// Encrypts data with the public key.
    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>>;
}

/// The `PrivateKey` trait represents a RSA private key type that can be used to decrypt data encrypted with corresponding public key.
#[async_trait(?Send)]
pub trait PrivateKey {
    /// Decrypts ciphertext with the private key.
    async fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>>;
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
        Ok(self.0.n().to_bytes_be())
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
#[async_trait(?Send)]
impl ExchangeKey for RsaPublicKey {
    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let padding = Oaep::new::<Sha256>();
        self.0
            .encrypt(&mut rand::thread_rng(), padding, data)
            .map_err(|e| anyhow!(RsaError::EncryptionFailed(anyhow!(e))))
    }

    async fn from_modulus(modulus: &[u8]) -> Result<Self> {
        let n = BigUint::from_bytes_be(modulus);
        let e = BigUint::from(PUBLIC_KEY_EXPONENT);

        Ok(Self(
            rsa::RsaPublicKey::new(n, e).map_err(|e| RsaError::InvalidPublicKey(anyhow!(e)))?,
        ))
    }
}

#[cfg(test)]
#[async_trait(?Send)]
impl PrivateKey for RsaPrivateKey {
    async fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        let padding = Oaep::new::<Sha256>();
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

    #[async_std::test]
    async fn test_rsa_key_pair() {
        let priv_key = RsaPrivateKey::new().unwrap();
        let pub_key = priv_key.get_public_key();

        let plaintext = b"Hello, world!";
        let ciphertext = pub_key.encrypt(plaintext).await.unwrap();
        let decrypted = priv_key.decrypt(&ciphertext).await.unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }

    #[async_std::test]
    async fn test_rsa_key_pair_from_public_key_modulus() {
        let priv_key = RsaPrivateKey::new().unwrap();
        let pub_key = priv_key.get_public_key();

        let public_key_modulus = pub_key.get_public_key_modulus().unwrap();
        let key_pair_from_modulus = RsaPublicKey::from_modulus(&public_key_modulus)
            .await
            .unwrap();

        let plaintext = b"Hello, world!";
        let ciphertext = key_pair_from_modulus.encrypt(plaintext).await.unwrap();
        let decrypted = priv_key.decrypt(&ciphertext).await.unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }
}
