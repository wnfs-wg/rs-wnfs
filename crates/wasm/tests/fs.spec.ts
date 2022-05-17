///<reference path="server/index.d.ts"/>

import { expect, test } from "@playwright/test";

const url = "http://localhost:8085";

test.beforeEach(async ({ page }) => {
  await page.goto(url);
});

test.describe("PublicDirectory", () => {
  test("lookupNode can fetch file added to directory", async ({ page }) => {
    const result = await page.evaluate(async () => {
      const {
        wnfs: { PublicDirectory },
        mock: { MemoryBlockStore, sampleCID },
      } = await window.setup();

      const time = new Date();
      const store = new MemoryBlockStore();
      const rootDir = new PublicDirectory(time);

      var { rootNode } = await rootDir.write(
        ["text.txt"],
        sampleCID,
        time,
        store
      );

      return await rootNode.asDir().lookupNode("text.txt", store);
    });

    expect(result).toBeDefined();
  });

  test("lookupNode cannot fetch file not added to directory", async ({
    page,
  }) => {
    const result = await page.evaluate(async () => {
      const {
        wnfs: { PublicDirectory },
        mock: { MemoryBlockStore },
      } = await window.setup();

      const time = new Date();
      const store = new MemoryBlockStore();
      const rootDir = new PublicDirectory(time);

      return await rootDir.lookupNode("Unknown", store);
    });

    expect(result).toBe(undefined);
  });

  test("mkdir can create new directory", async ({ page }) => {
    const result = await page.evaluate(async () => {
      const {
        wnfs: { PublicDirectory },
        mock: { MemoryBlockStore, sampleCID },
      } = await window.setup();

      const time = new Date();
      const store = new MemoryBlockStore();
      const rootDir = new PublicDirectory(time);

      var { rootNode } = await rootDir.mkdir(["pictures", "cats"], time, store);

      var { rootNode } = await rootNode
        .asDir()
        .write(["pictures", "cats", "tabby.png"], sampleCID, time, store);

      var { rootNode } = await rootNode
        .asDir()
        .getNode(["pictures", "cats", "tabby.png"], store);

      return rootNode;
    });

    expect(result).toBeDefined();
  });

  test("ls can list children under directory", async ({ page }) => {
    const result = await page.evaluate(async () => {
      const {
        wnfs: { PublicDirectory },
        mock: { MemoryBlockStore, sampleCID },
      } = await window.setup();

      const time = new Date();
      const store = new MemoryBlockStore();
      const rootDir = new PublicDirectory(time);

      var { rootNode } = await rootDir.mkdir(["pictures", "dogs"], time, store);

      var { rootNode } = await rootNode
        .asDir()
        .write(["pictures", "cats", "tabby.png"], sampleCID, time, store);

      var { result } = await rootNode.asDir().ls(["pictures"], store);

      return result;
    });

    expect(result.length).toBe(2);
    expect(result.includes("dogs")).toBe(true);
    expect(result.includes("cats")).toBe(true);
  });

  test("rm can remove children from directory", async ({ page }) => {
    const result = await page.evaluate(async () => {
      const {
        wnfs: { PublicDirectory },
        mock: { MemoryBlockStore, sampleCID },
      } = await window.setup();

      const time = new Date();
      const store = new MemoryBlockStore();
      const rootDir = new PublicDirectory(time);

      var { rootNode } = await rootDir.write(
        ["pictures", "dogs", "billie.jpeg"],
        sampleCID,
        time,
        store
      );
      var { rootNode } = await rootNode
        .asDir()
        .write(["pictures", "cats", "tabby.png"], sampleCID, time, store);
      var { rootNode } = await rootNode.asDir().rm(["pictures", "cats"], store);
      var { result } = await rootNode.asDir().ls(["pictures"], store);

      return result;
    });

    expect(result).toEqual(["dogs"]);
  });
});
