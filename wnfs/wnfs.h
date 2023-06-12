#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>


static const size_t KEY_BYTE_SIZE = 32;

/// The maximum block size is 2 ^ 18 but the first 12 bytes are reserved for the cipher text's initialization vector.
/// The ciphertext then also contains a 16 byte authentication tag.
/// This leaves a maximum of (2 ^ 18) - 12 - 16 = 262,116 bytes for the actual data.
///
/// More on that [here][priv-file].
///
/// [priv-file]: https://github.com/wnfs-wg/spec/blob/matheus23/file-sharding/spec/private-wnfs.md#314-private-file
static const size_t MAX_BLOCK_CONTENT_SIZE = ((MAX_BLOCK_SIZE - NONCE_SIZE) - AUTHENTICATION_TAG_SIZE);

static const uint64_t PUBLIC_KEY_EXPONENT = 65537;

static const size_t RSA_KEY_SIZE = 2048;
