///<reference path="index.d.ts"/>

import { sampleCID, MemoryBlockStore, Rng } from "../mock";

const setup = async () => {
  const { PublicDirectory, PublicFile, PublicNode, PrivateDirectory, PrivateForest, PrivateFile, PrivateNode, Namefilter } = await import("../../pkg/index");

  const mock = { sampleCID, MemoryBlockStore, Rng };
  const wnfs = { PublicDirectory, PublicFile, PublicNode, PrivateDirectory, PrivateForest, PrivateFile, PrivateNode, Namefilter };

  return { mock, wnfs };
};

window.setup = setup;
