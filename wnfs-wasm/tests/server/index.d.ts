export {};

declare global {
  interface Window {
    setup: () => Promise<{
      mock: {
        sampleCID: typeof import("../mock").sampleCID;
        CID: typeof import("../mock").CID,
        MemoryBlockStore: typeof import("../mock").MemoryBlockStore;
        Rng: typeof import("../mock").Rng;
        ExchangeKey: typeof import("../mock").ExchangeKey;
        PrivateKey: typeof import("../mock").PrivateKey;
        createSharerDir: typeof import("../mock").createSharerDir;
        createRecipientExchangeRoot: typeof import("../mock").createRecipientExchangeRoot;
      };
      wnfs: {
        PublicDirectory: typeof import("../../pkg/wnfs_wasm.d").PublicDirectory;
        PublicFile: typeof import("../../pkg/wnfs_wasm.d").PublicFile;
        PublicNode: typeof import("../../pkg/wnfs_wasm.d").PublicNode;
        PrivateDirectory: typeof import("../../pkg/wnfs_wasm.d").PrivateDirectory;
        PrivateForest: typeof import("../../pkg/wnfs_wasm.d").PrivateForest;
        PrivateFile: typeof import("../../pkg/wnfs_wasm.d").PrivateFile;
        PrivateNode: typeof import("../../pkg/wnfs_wasm.d").PrivateNode;
        PrivateRef: typeof import("../../pkg/wnfs_wasm.d").PrivateRef;
        Namefilter: typeof import("../../pkg/wnfs_wasm.d").Namefilter;
        SharePayload: typeof import("../../pkg/wnfs_wasm.d").SharePayload;
        share: typeof import("../../pkg/wnfs_wasm.d").share;
        findShare: typeof import("../../pkg/wnfs_wasm.d").findShare;
        receiveShare: typeof import("../../pkg/wnfs_wasm.d").receiveShare;
        createShareLabel: typeof import("../../pkg/wnfs_wasm.d").createShareLabel;
      };
      setPanicHook: typeof import("../../pkg/wnfs_wasm.d").setPanicHook;
    }>;
  }
}
