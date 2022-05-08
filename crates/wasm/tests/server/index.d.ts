export {};

declare global {
  interface Window {
    setup: () => Promise<{
      mock: {
        sampleCID: typeof import("../mock").sampleCID;
        MemoryBlockStore: typeof import("../mock").MemoryBlockStore;
      };
      wnfs: {
        PublicDirectory: typeof import("../../pkg/index").PublicDirectory;
      };
    }>;
  }
}
