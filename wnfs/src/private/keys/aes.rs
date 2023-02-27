use serde::{Deserialize, Serialize};
use std::fmt::Debug;

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

pub(crate) const NONCE_SIZE: usize = 12;
pub(crate) const AUTHENTICATION_TAG_SIZE: usize = 16;
pub const KEY_BYTE_SIZE: usize = 32;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A symmetric private key used for encryption and decryption of content in the filesystem.
/// This is a wrapper around a 32 byte AES key.
///
/// # Examples
///
/// ```
/// use wnfs::{private::AesKey, common::utils};
/// use rand::thread_rng;
///
/// let rng = &mut thread_rng();
/// let key = AesKey::new(utils::get_random_bytes(rng));
///
/// println!("AesKey: {:?}", key);
/// ```
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AesKey(
    #[serde(serialize_with = "crate::utils::serialize_byte_slice32")]
    #[serde(deserialize_with = "crate::utils::deserialize_byte_slice32")]
    pub(super) [u8; KEY_BYTE_SIZE],
);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl AesKey {
    /// Creates a new key from [u8; KEY_SIZE].
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{private::AesKey, common::utils};
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let key = AesKey::new(utils::get_random_bytes(rng));
    ///
    /// println!("AesKey: {:?}", key);
    /// ```
    pub fn new(bytes: [u8; KEY_BYTE_SIZE]) -> Self {
        Self(bytes)
    }

    /// Grabs the bytes of the key.
    pub fn bytes(self) -> [u8; KEY_BYTE_SIZE] {
        self.0
    }

    /// Gets the bytes of the key.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Debug for AesKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x")?;
        for (i, byte) in self.0.iter().enumerate() {
            if i > 6 {
                write!(f, "..")?;
                break;
            } else {
                write!(f, "{byte:02X}")?;
            }
        }

        Ok(())
    }
}
