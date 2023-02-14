///<reference path="index.d.ts"/>

import {
  sampleCID,
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
    Namefilter,
    setPanicHook,
    SharePayload,
    share,
    createShareLabel,
    receiveShare,
    findShare,
  } = await import("../../pkg/index");

  const mock = {
    sampleCID,
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
    Namefilter,
    SharePayload,
    share,
    createShareLabel,
    receiveShare,
    findShare,
  };

  return { mock, wnfs, setPanicHook };
};

window.setup = setup;
