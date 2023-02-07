import { CID } from "multiformats/cid";
import { sha256 } from "multiformats/hashes/sha2";

/** A mock CID. */
const sampleCID = CID.parse(
  "bagaaierasords4njcts6vs7qvdjfcvgnume4hqohf65zsfguprqphs3icwea"
).bytes;

/**
 * An in-memory block store to simulate IPFS.
 *
 * IPFS is basically a glorified HashMap.
 */
class MemoryBlockStore {
  private store: Map<string, Uint8Array>;

  /** Creates a new in-memory block store. */
  constructor() {
    this.store = new Map();
  }

  /** Stores an array of bytes in the block store. */
  async getBlock(cid: Uint8Array): Promise<Uint8Array | undefined> {
    const decoded_cid = CID.decode(cid);
    return this.store.get(decoded_cid.toString());
  }

  /** Retrieves an array of bytes from the block store with given CID. */
  async putBlock(bytes: Uint8Array, code: number): Promise<Uint8Array> {
    const hash = await sha256.digest(bytes);
    const cid = CID.create(1, code, hash);
    this.store.set(cid.toString(), bytes);
    return cid.bytes;
  }
}

/** A pseudo-random number generator */
class Rng {
  /** Returns random bytes of specified length */
  randomBytes(count: number): Uint8Array {
    const array = new Uint8Array(count);
    self.crypto.getRandomValues(array);
    return array;
  }
}

export { sampleCID, MemoryBlockStore, Rng, CID };
