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
        wnfs: { PrivateForest, share, createShareName, receiveShare },
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

      const rng = new Rng();
      var forest = new PrivateForest(rng);
      const sharerRootDid = "did:key:z6MkqZjY";
      const store = new MemoryBlockStore();

      var { rootDir: sharerDir, forest: forest } = await createSharerDir(
        forest,
        store,
        rng
      );

      const [recipientKey, recipientExchRoot] =
        await createRecipientExchangeRoot(store);

      const recipientExchRootCid = await recipientExchRoot.store(store);

      var [accessKey, forest2] = await sharerDir
        .asNode()
        .store(forest, store, rng);

      var forest2 = await share(
        accessKey,
        0,
        sharerRootDid,
        recipientExchRootCid,
        forest2,
        store
      );

      const modulus = await recipientKey.getPublicKey().getPublicKeyModulus();
      const shareLabel = createShareName(0, sharerRootDid, modulus, forest2);

      const sharedNode = await receiveShare(
        shareLabel,
        recipientKey,
        forest2,
        store
      );

      console.log("sharedNode", sharedNode);

      return sharedNode;
    });

    expect(result).toBeDefined();
  });
});
