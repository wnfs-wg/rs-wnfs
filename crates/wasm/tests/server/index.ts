///<reference path="index.d.ts"/>

import { sampleCID, MemoryBlockStore } from "../mock";

const setup = async () => {
  const { PublicDirectory } = await import("../../pkg/index");

  const mock = { sampleCID, MemoryBlockStore };
  const wnfs = { PublicDirectory };

  return { mock, wnfs };
};

window.setup = setup;
