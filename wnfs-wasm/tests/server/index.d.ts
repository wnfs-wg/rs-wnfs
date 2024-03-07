export {};

declare global {
  interface Window {
    setup: () => Promise<{
      mock: {
        sampleCID: typeof import("../mock").sampleCID;
        CID: typeof import("../mock").CID,
        MemoryBlockStore: typeof import("../mock").MemoryBlockStore;
        Sha256BlockStore: typeof import("../mock").Sha256BlockStore;
        Rng: typeof import("../mock").Rng;
        ExchangeKey: typeof import("../mock").ExchangeKey;
        PrivateKey: typeof import("../mock").PrivateKey;
        createSharerDir: typeof import("../mock").createSharerDir;
        createRecipientExchangeRoot: typeof import("../mock").createRecipientExchangeRoot;
      };
      wnfs: {
        PublicDirectory: typeof import("../../dist/bundler/wnfs_wasm").PublicDirectory;
        PublicFile: typeof import("../../dist/bundler/wnfs_wasm").PublicFile;
        PublicNode: typeof import("../../dist/bundler/wnfs_wasm").PublicNode;
        PrivateDirectory: typeof import("../../dist/bundler/wnfs_wasm").PrivateDirectory;
        PrivateForest: typeof import("../../dist/bundler/wnfs_wasm").PrivateForest;
        PrivateFile: typeof import("../../dist/bundler/wnfs_wasm").PrivateFile;
        PrivateNode: typeof import("../../dist/bundler/wnfs_wasm").PrivateNode;
        Name: typeof import("../../dist/bundler/wnfs_wasm").Name;
        NameAccumulator: typeof import("../../dist/bundler/wnfs_wasm").NameAccumulator;
        AccessKey: typeof import("../../dist/bundler/wnfs_wasm").AccessKey;
        share: typeof import("../../dist/bundler/wnfs_wasm").share;
        findLatestShareCounter: typeof import("../../dist/bundler/wnfs_wasm").findLatestShareCounter;
        receiveShare: typeof import("../../dist/bundler/wnfs_wasm").receiveShare;
        createShareName: typeof import("../../dist/bundler/wnfs_wasm").createShareName;
      };
      setPanicHook: typeof import("../../dist/bundler/wnfs_wasm").setPanicHook;
    }>;
  }
}
