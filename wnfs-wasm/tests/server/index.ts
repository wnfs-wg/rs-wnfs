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
    AccessKey,
    Namefilter,
    setPanicHook,
    share,
    createShareLabel,
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
    AccessKey,
    Namefilter,
    share,
    createShareLabel,
    receiveShare,
    findLatestShareCounter,
  };

  return { mock, wnfs, setPanicHook };
};

window.setup = setup;
