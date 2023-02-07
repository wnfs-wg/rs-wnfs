export { };

declare global {
  interface Window {
    setup: () => Promise<{
      mock: {
        sampleCID: typeof import("../mock").sampleCID;
        MemoryBlockStore: typeof import("../mock").MemoryBlockStore;
        Rng: typeof import("../mock").Rng;
        CID: typeof import("../mock").CID;
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
      };
    }>;
  }
}
