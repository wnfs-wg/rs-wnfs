export {};

declare global {
    interface Window {
        wnfs: {
            init: () => Promise<void>;
            MemoryBlockStore: typeof import("../pkg/wasm_wnfs").MemoryBlockStore;
            PublicDirectory: typeof import("../pkg/wasm_wnfs").PublicDirectory;
        };
    }
}
