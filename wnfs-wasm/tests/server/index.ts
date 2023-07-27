///<reference path="index.d.ts"/>

import {
  sampleCID,
  CID,
  MemoryBlockStore,
  Rng,
  createSharerDir,
  createRecipientExchangeRoot,
  PrivateKey,
  ExchangeKey,
} from "../mock";

const setup = async () => {
  const {
    PublicDirectory,
    PublicFile,
    PublicNode,
    PrivateDirectory,
    PrivateForest,
    PrivateFile,
    PrivateNode,
    Name,
    NameAccumulator,
    AccessKey,
    setPanicHook,
    share,
    createShareName,
    receiveShare,
    findLatestShareCounter,
  } = await import("../../dist/bundler/wnfs_wasm");

  const mock = {
    sampleCID,
    CID,
    MemoryBlockStore,
    Rng,
    createSharerDir,
    createRecipientExchangeRoot,
    PrivateKey,
    ExchangeKey,
  };

  const wnfs = {
    PublicDirectory,
    PublicFile,
    PublicNode,
    PrivateDirectory,
    PrivateForest,
    PrivateFile,
    PrivateNode,
    Name,
    NameAccumulator,
    AccessKey,
    share,
    createShareName,
    receiveShare,
    findLatestShareCounter,
  };

  return { mock, wnfs, setPanicHook };
};

window.setup = setup;
