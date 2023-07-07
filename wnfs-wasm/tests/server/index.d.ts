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
        Name: typeof import("../../pkg/index").Name;
        NameAccumulator: typeof import("../../pkg/index").NameAccumulator;
        AccessKey: typeof import("../../pkg/index").AccessKey;
        share: typeof import("../../pkg/index").share;
        findLatestShareCounter: typeof import("../../pkg/index").findLatestShareCounter;
        receiveShare: typeof import("../../pkg/index").receiveShare;
        createShareName: typeof import("../../pkg/index").createShareName;
      };
      setPanicHook: typeof import("../../pkg/index").setPanicHook;
    }>;
  }
}
