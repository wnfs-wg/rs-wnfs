import { MemoryBlockStore } from "./blockstore";

export type Nullable<T> = T | null;

export interface BlockStore {
  getBlock(cid: Uint8Array): Promise<Uint8Array | undefined>;
  putBlock(bytes: Uint8Array, code: number): Promise<void>;
}

export type Handler = ((e: Event) => void) | ((e: Event) => Promise<void>);

export type EventMap = Map<HTMLElement, [string, Handler]>;

export type Connection = [string, string, boolean];
