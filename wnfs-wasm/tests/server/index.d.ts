export { };

declare global {
  interface Window {
    setup: () => Promise<{
      mock: {
        sampleCID: typeof import("../mock").sampleCID;
        MemoryBlockStore: typeof import("../mock").MemoryBlockStore;
        Rng: typeof import("../mock").Rng;
      };
      wnfs: {
        PublicDirectory: typeof import("../../pkg/index").PublicDirectory;
        PublicFile: typeof import("../../pkg/index").PublicFile;
        PublicNode: typeof import("../../pkg/index").PublicNode;
        PrivateDirectory: typeof import("../../pkg/index").PrivateDirectory;
        PrivateForest: typeof import("../../pkg/index").PrivateForest;
        PrivateFile: typeof import("../../pkg/index").PrivateFile;
        PrivateNode: typeof import("../../pkg/index").PrivateNode;
        Namefilter: typeof import("../../pkg/index").Namefilter;
      };
    }>;
  }
}
