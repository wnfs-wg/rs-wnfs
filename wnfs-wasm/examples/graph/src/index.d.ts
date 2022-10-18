import { MemoryBlockStore } from "./blockstore";
import { Nullable } from "./types";

declare global {
  var store: MemoryBlockStore;
  class LeaderLine {
    constructor(
      start: Nullable<HTMLElement>,
      end: Nullable<HTMLElement>,
      options?: Options
    );
  }
}

type Options = {
  color?: string;
  size?: number;
  dash?: {
    animation: boolean;
  };
};

export {};
