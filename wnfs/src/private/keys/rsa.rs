use crate::RsaError;
use anyhow::{anyhow, Result};
use rand_core::{CryptoRng, RngCore};
use rsa::{
    pkcs1::DecodeRsaPrivateKey, BigUint, PaddingScheme, PublicKey, PublicKeyParts, RsaPrivateKey,
    RsaPublicKey,
};
use sha2::Sha256;

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const RSA_KEY_SIZE: usize = 2048;
const PUBLIC_KEY_EXPONENT: u64 = 65537;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub trait RsaKeyPair {
    /// Encrypts data with the public key.
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>>;

    /// Decrypts ciphertext with the private key.
    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>>;

    /// Creates an RSA public key from the public key modulus.
    fn from_public_key_modulus(modulus: &[u8]) -> Result<Self>
    where
        Self: Sized;
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
pub struct RsaKeys {
    pub(crate) public_key: RsaPublicKey,
    pub(crate) private_key: Option<RsaPrivateKey>,
}

pub type PublicKeyModulus = Vec<u8>;

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[cfg(not(target_arch = "wasm32"))]
impl RsaKeys {
    /// Constructs a new 2048-bit RSA key pair.
    pub fn new(rng: &mut (impl RngCore + CryptoRng)) -> Result<Self> {
        let private_key = RsaPrivateKey::new(rng, RSA_KEY_SIZE)?;

        Ok(Self {
            public_key: private_key.to_public_key(),
            private_key: Some(private_key),
        })
    }

    /// Creates an RSA key pair from PKCS#1 DER encoded private key.
    pub fn from_private_key_der(bytes: &[u8]) -> Result<Self> {
        let private_key = RsaPrivateKey::from_pkcs1_der(bytes)
            .map_err(|e| RsaError::InvalidPrivateKey(anyhow!(e)))?;

        Ok(Self {
            public_key: private_key.to_public_key(),
            private_key: Some(private_key),
        })
    }

    /// Gets the public key.
    pub fn get_public_key(&self) -> &RsaPublicKey {
        &self.public_key
    }

    /// Gets the public key modulus.
    pub fn get_public_key_modulus(&self) -> Result<Vec<u8>> {
        Ok(self.public_key.n().to_bytes_le())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl RsaKeyPair for RsaKeys {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let padding = PaddingScheme::new_oaep::<Sha256>();
        self.public_key
            .encrypt(&mut rand::thread_rng(), padding, data)
            .map_err(|e| anyhow!(RsaError::EncryptionFailed(anyhow!(e))))
    }

    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        let padding = PaddingScheme::new_oaep::<Sha256>();
        self.private_key
            .as_ref()
            .ok_or(RsaError::NoPrivateKey)?
            .decrypt(padding, ciphertext)
            .map_err(|e| anyhow!(RsaError::DecryptionFailed(anyhow!(e))))
    }

    fn from_public_key_modulus(modulus: &[u8]) -> Result<Self> {
        let n = BigUint::from_bytes_le(modulus);
        let e = BigUint::from(PUBLIC_KEY_EXPONENT);

        let public_key =
            RsaPublicKey::new(n, e).map_err(|e| RsaError::InvalidPublicKey(anyhow!(e)))?;

        Ok(Self {
            public_key,
            private_key: None,
        })
    }
}
