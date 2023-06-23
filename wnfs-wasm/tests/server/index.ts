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
    PrivateRef,
    Name,
    NameAccumulator,
    setPanicHook,
    SharePayload,
    share,
    createShareName,
    receiveShare,
    findLatestShareCounter,
  } = await import("../../pkg/index");

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
    PrivateRef,
    Name,
    NameAccumulator,
    SharePayload,
    share,
    createShareName,
    receiveShare,
    findLatestShareCounter,
  };

  return { mock, wnfs, setPanicHook };
};

window.setup = setup;
