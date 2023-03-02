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
        PublicDirectory: typeof import("../../pkg/index").PublicDirectory;
        PublicFile: typeof import("../../pkg/index").PublicFile;
        PublicNode: typeof import("../../pkg/index").PublicNode;
        PrivateDirectory: typeof import("../../pkg/index").PrivateDirectory;
        PrivateForest: typeof import("../../pkg/index").PrivateForest;
        PrivateFile: typeof import("../../pkg/index").PrivateFile;
        PrivateNode: typeof import("../../pkg/index").PrivateNode;
        PrivateRef: typeof import("../../pkg/index").PrivateRef;
        Namefilter: typeof import("../../pkg/index").Namefilter;
        SharePayload: typeof import("../../pkg/index").SharePayload;
        share: typeof import("../../pkg/index").share;
        findLatestShareCounter: typeof import("../../pkg/index").findLatestShareCounter;
        receiveShare: typeof import("../../pkg/index").receiveShare;
        createShareLabel: typeof import("../../pkg/index").createShareLabel;
      };
      setPanicHook: typeof import("../../pkg/index").setPanicHook;
    }>;
  }
}
