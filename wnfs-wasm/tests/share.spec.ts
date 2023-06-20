///<reference path="server/index.d.ts"/>

import { expect, test } from "@playwright/test";

const url = "http://localhost:8085";

test.beforeEach(async ({ page }) => {
  await page.goto(url);
  await page.waitForFunction(() => window.setup != null);
});

test.describe("Share", () => {
  test("share and recieve share", async ({ page }) => {
    const result = await page.evaluate(async () => {
      const {
        wnfs: {
          PrivateForest,
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
      const store = new MemoryBlockStore();
      const sharerRootDid = "did:key:z6MkqZjY";
      const rng = new Rng();

      var { rootDir: sharerDir, forest: sharerForest } = await createSharerDir(
        sharerForest,
        store,
        rng
      );

      const [recipientKey, recipientExchRoot] =
        await createRecipientExchangeRoot(store);

      const recipientExchRootCid = await recipientExchRoot.store(store);

      var [accessKey, sharerForest2] = await sharerDir
        .asNode()
        .store(sharerForest, store, rng);

      var sharerForest2 = await share(
        accessKey,
        0,
        sharerRootDid,
        sharerForest2,
        recipientExchRootCid,
        store
      );

      const modulus = await recipientKey.getPublicKey().getPublicKeyModulus();
      const shareLabel = createShareLabel(0, sharerRootDid, modulus);

      const sharedNode = await receiveShare(
        shareLabel,
        recipientKey,
        sharerForest2,
        store
      );

      console.log("sharedNode", sharedNode);

      return sharedNode;
    });

    expect(result).toBeDefined();
  });
});
