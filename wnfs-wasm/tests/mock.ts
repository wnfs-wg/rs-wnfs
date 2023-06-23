import { CID } from "multiformats/cid";
import { sha256 } from "multiformats/hashes/sha2";
import { fromString, toString } from "uint8arrays";
import {
  Name,
  NameAccumulator,
  PrivateDirectory,
  PrivateForest,
  PublicDirectory,
} from "../pkg";

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

/** A mock exchange key. */
class ExchangeKey {
  key: CryptoKey;

  constructor(key: CryptoKey) {
    this.key = key;
  }

  static async fromModulus(modulus: Uint8Array): Promise<ExchangeKey> {
    var keyData = {
      kty: "RSA",
      n: toString(modulus, "base64url"),
      e: toString(new Uint8Array([0x01, 0x00, 0x01]), "base64url"),
      alg: "RSA-OAEP-256",
      ext: true,
    };

    const key = await crypto.subtle.importKey(
      "jwk",
      keyData,
      {
        name: "RSA-OAEP",
        hash: { name: "SHA-256" },
      },
      false,
      ["encrypt"]
    );

    return new ExchangeKey(key);
  }

  async encrypt(data: Uint8Array): Promise<Uint8Array> {
    const encryptedData = await window.crypto.subtle.encrypt(
      {
        name: "RSA-OAEP",
      },
      this.key,
      data
    );

    return new Uint8Array(encryptedData);
  }

  async getPublicKeyModulus(): Promise<Uint8Array> {
    const key = await crypto.subtle.exportKey("jwk", this.key);
    return fromString(key.n as string, "base64url");
  }
}

/** A mock private key. */
class PrivateKey {
  key: CryptoKeyPair;

  constructor(key: CryptoKeyPair) {
    this.key = key;
  }

  static async generate(): Promise<PrivateKey> {
    const keyPair = await crypto.subtle.generateKey(
      {
        name: "RSA-OAEP",
        modulusLength: 2048,
        publicExponent: new Uint8Array([0x01, 0x00, 0x01]),
        hash: { name: "SHA-256" },
      },
      true,
      ["decrypt"]
    );

    return new PrivateKey(keyPair);
  }

  async decrypt(data: Uint8Array): Promise<Uint8Array> {
    const decryptedData = await window.crypto.subtle.decrypt(
      {
        name: "RSA-OAEP",
      },
      this.key.privateKey,
      data
    );

    return new Uint8Array(decryptedData);
  }

  getPublicKey(): ExchangeKey {
    return new ExchangeKey(this.key.publicKey);
  }
}

const createSharerDir = async (
  initialForest: PrivateForest,
  store: MemoryBlockStore,
  rng: Rng
): Promise<{ rootDir: PrivateDirectory; forest: PrivateForest }> => {
  var { rootDir, forest } = await PrivateDirectory.newAndStore(
    initialForest.emptyName(),
    new Date(),
    initialForest,
    store,
    rng
  );

  return await rootDir.write(
    ["text.txt"],
    true,
    new Uint8Array([1, 2, 3, 4, 5]),
    new Date(),
    forest,
    store,
    rng
  );
};

const createRecipientExchangeRoot = async (
  store: MemoryBlockStore
): Promise<[PrivateKey, PublicDirectory]> => {
  const key = await PrivateKey.generate();
  const exchangeKey = await key.getPublicKey().getPublicKeyModulus();
  const exchangeKeyCid = await store.putBlock(exchangeKey, 0x55);

  const { rootDir } = await new PublicDirectory(new Date()).write(
    ["device1", "v1.exchange_key"],
    exchangeKeyCid,
    new Date(),
    store
  );

  return [key, rootDir];
};

export {
  sampleCID,
  CID,
  MemoryBlockStore,
  Rng,
  createSharerDir,
  createRecipientExchangeRoot,
  PrivateKey,
  ExchangeKey,
};
