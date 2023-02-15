///<reference path="server/index.d.ts"/>

import { expect, test } from "@playwright/test";

const url = "http://localhost:8085";

test.beforeEach(async ({ page }) => {
  await page.goto(url);
});

test.describe("Share", () => {
  test("share and recieve share", async ({ page }) => {
    const result = await page.evaluate(async () => {
      const {
        wnfs: {
          PrivateForest,
          SharePayload,
          share,
          createShareLabel,
          receiveShare,
        },
        mock: {
          MemoryBlockStore,
          Rng,
          createSharerDir,
          createRecipientExchangeRoot,
          ExchangeKey,
        },
      } = await window.setup();

      // @ts-ignore
      globalThis.ExchangeKey = ExchangeKey;

      var sharerForest = new PrivateForest();
      const sharerStore = new MemoryBlockStore();
      const sharerRootDid = "did:key:z6MkqZjY";
      const recipientStore = new MemoryBlockStore();
      const rng = new Rng();

      var { rootDir: sharerDir, forest: sharerForest } = await createSharerDir(
        sharerForest,
        sharerStore,
        rng
      );

      const [recipientKey, recipientExchRoot] =
        await createRecipientExchangeRoot(recipientStore);

      const recipientExchRootCid = await recipientExchRoot.store(
        recipientStore
      );

      var [sharerPayload, sharerForest2] = await SharePayload.fromNode(
        sharerDir.asNode(),
        true,
        sharerForest,
        sharerStore,
        rng
      );

      var sharerForest2 = await share(
        sharerPayload,
        BigInt(0),
        sharerRootDid,
        sharerForest2,
        sharerStore,
        recipientExchRootCid,
        recipientStore
      );

      const modulus = await recipientKey.getPublicKey().getPublicKeyModulus();
      const shareLabel = createShareLabel(BigInt(0), sharerRootDid, modulus);

      const recipientPayload = await receiveShare(
        shareLabel,
        recipientKey,
        sharerForest2,
        sharerStore
      );

      console.log("recipientPayload", recipientPayload);

      return recipientPayload;
    });

    expect(result).toBeDefined();
  });
});
