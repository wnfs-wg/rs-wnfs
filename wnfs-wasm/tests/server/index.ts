///<reference path="index.d.ts"/>

import { sampleCID, MemoryBlockStore, Rng, CID } from "../mock";

const setup = async () => {
  const { PublicDirectory, PublicFile, PublicNode, PrivateDirectory, PrivateForest, PrivateFile, PrivateNode, Namefilter, PrivateRef } = await import("../../pkg/index");

  const mock = { sampleCID, MemoryBlockStore, Rng, CID };
  const wnfs = { PublicDirectory, PublicFile, PublicNode, PrivateDirectory, PrivateForest, PrivateFile, PrivateNode, Namefilter, PrivateRef };

  return { mock, wnfs };
};

window.setup = setup;
