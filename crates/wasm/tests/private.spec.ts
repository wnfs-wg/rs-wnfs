///<reference path="server/index.d.ts"/>

import { expect, test } from "@playwright/test";

const url = "http://localhost:8085";

test.beforeEach(async ({ page }) => {
  await page.goto(url);
});

test.describe("PrivateDirectory", () => {
  test("lookupNode can fetch file added to directory", async ({ page }) => {
    const result = await page.evaluate(async () => {
      const {
        wnfs: { PrivateDirectory, PrivateForest, Namefilter },
        mock: { MemoryBlockStore, Rng },
      } = await window.setup();

      const initialHamt = new PrivateForest();
      const rng = new Rng();
      const store = new MemoryBlockStore();
      const root = new PrivateDirectory(
        new Namefilter(),
        rng.randomBytes(32),
        rng.randomBytes(32),
        new Date()
      );

      var { rootDir, hamt } = await root.write(["text.txt"], new Uint8Array([1, 2, 3, 4, 5]), new Date(), initialHamt, store, rng);

      return await rootDir.lookupNode("text.txt", hamt, store);
    });

    expect(result).toBeDefined();
  });

  test("lookupNode cannot fetch file not added to directory", async ({
    page,
  }) => {
    const result = await page.evaluate(async () => {
      const {
        wnfs: { PrivateDirectory, PrivateForest, Namefilter },
        mock: { MemoryBlockStore, Rng },
      } = await window.setup();

      const initialHamt = new PrivateForest();
      const rng = new Rng();
      const store = new MemoryBlockStore();
      const root = new PrivateDirectory(
        new Namefilter(),
        rng.randomBytes(32),
        rng.randomBytes(32),
        new Date()
      );

      return await root.lookupNode("Unknown", initialHamt, store);
    });

    expect(result).toBe(undefined);
  });

  test("mkdir can create new directory", async ({ page }) => {
    const result = await page.evaluate(async () => {
      const {
        wnfs: { PrivateDirectory, PrivateForest, Namefilter },
        mock: { MemoryBlockStore, Rng },
      } = await window.setup();

      const initialHamt = new PrivateForest();
      const rng = new Rng();
      const store = new MemoryBlockStore();
      const root = new PrivateDirectory(
        new Namefilter(),
        rng.randomBytes(32),
        rng.randomBytes(32),
        new Date()
      );

      var { rootDir, hamt } = await root.mkdir(["pictures", "cats"], new Date(), initialHamt, store, rng);

      var { rootDir, hamt } = await rootDir.write(
        ["pictures", "cats", "tabby.png"],
        new Uint8Array([1, 2, 3, 4, 5]),
        new Date(),
        hamt,
        store,
        rng,
      );

      var { rootDir } = await rootDir.getNode(
        ["pictures", "cats", "tabby.png"],
        hamt,
        store
      );

      return rootDir;
    });

    expect(result).toBeDefined();
  });

  test("rm can remove children from directory", async ({ page }) => {
    const result = await page.evaluate(async () => {
      const {
        wnfs: { PrivateDirectory, PrivateForest, Namefilter },
        mock: { MemoryBlockStore, Rng },
      } = await window.setup();

      const initialHamt = new PrivateForest();
      const rng = new Rng();
      const store = new MemoryBlockStore();
      const root = new PrivateDirectory(
        new Namefilter(),
        rng.randomBytes(32),
        rng.randomBytes(32),
        new Date()
      );

      var { rootDir, hamt } = await root.write(
        ["pictures", "dogs", "billie.jpeg"],
        new Uint8Array([1, 2, 3, 4, 5]),
        new Date(),
        initialHamt,
        store,
        rng,
      );

      var { rootDir, hamt } = await rootDir.write(
        ["pictures", "cats", "tabby.png"],
        new Uint8Array([1, 2, 3, 4, 5]),
        new Date(),
        hamt,
        store,
        rng,
      );
      var { rootDir, hamt } = await rootDir.rm(["pictures", "cats"], hamt, store, rng);

      return await root.lookupNode("cats", hamt, store);
    });

    expect(result).toBe(undefined);
  });
});
