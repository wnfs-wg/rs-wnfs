///<reference path="spec.d.ts"/>

import { expect, test } from '@playwright/test';
import { cid } from './mocks';

const url = "http://localhost:8085/tests";

test.beforeEach(async ({ page }) => {
  await page.goto(url);
});

test.describe('PublicDirectory', () => {
  test('lookupNode can fetch file added to directory', async ({ page }) => {
    const result = await page.evaluate(async ({ cid }) => {
      const { init, MemoryBlockStore, PublicDirectory } = window.wnfs;
      await init();

      const time = new Date();
      const store = new MemoryBlockStore();
      const rootDir = new PublicDirectory(time);

      var { rootNode } = await rootDir.write(["text.txt"], cid, time, store);
      return await rootNode.asDir().lookupNode("text.txt", store);
    }, { cid });

    expect(result).toBeDefined();
  });

  test('lookupNode cannot fetch file not added to directory', async ({ page }) => {
    const result = await page.evaluate(async () => {
      const { init, MemoryBlockStore, PublicDirectory } = window.wnfs;
      await init();

      const time = new Date();
      const store = new MemoryBlockStore();
      const rootDir = new PublicDirectory(time);

      return await rootDir.lookupNode("Unknown", store);
    });

    expect(result).toBe(undefined);
  });

  test('mkdir can create new directory', async ({ page }) => {
    const result = await page.evaluate(async ({ cid }) => {
      const { init, MemoryBlockStore, PublicDirectory } = window.wnfs;

      await init();

      const time = new Date();
      const store = new MemoryBlockStore();
      const rootDir = new PublicDirectory(time);

      var { rootNode } = await rootDir.mkdir(["pictures", "cats"], time, store);
      var { rootNode } = await rootNode.asDir().write(["pictures", "cats", "tabby.png"], cid, time, store);
      var { rootNode } = await rootNode.asDir().getNode(["pictures", "cats", "tabby.png"], store);

      return rootNode;
    }, { cid });

    expect(result).toBeDefined();
  });

  test('ls can list children under directory', async ({ page }) => {
    const result = await page.evaluate(async ({ cid }) => {
      const { init, MemoryBlockStore, PublicDirectory } = window.wnfs;

      await init();

      const time = new Date();
      const store = new MemoryBlockStore();
      const rootDir = new PublicDirectory(time);

      var { rootNode } = await rootDir.mkdir(["pictures", "dogs"], time, store);
      var { rootNode } = await rootNode.asDir().write(["pictures", "cats", "tabby.png"], cid, time, store);
      var { result } = await rootNode.asDir().ls(["pictures"], store);

      return result;
    }, { cid });

    expect(result.length).toBe(2);
    expect(result.includes("dogs")).toBe(true);
    expect(result.includes("cats")).toBe(true);
  });

  test('rm can remove children from directory', async ({ page }) => {
    const result = await page.evaluate(async ({ cid }) => {
      const { init, MemoryBlockStore, PublicDirectory } = window.wnfs;

      await init();

      const time = new Date();
      const store = new MemoryBlockStore();
      const rootDir = new PublicDirectory(time);

      var { rootNode } = await rootDir.write(["pictures", "dogs", "billie.jpeg"], cid, time, store);
      var { rootNode } = await rootNode.asDir().write(["pictures", "cats", "tabby.png"], cid, time, store);
      var { rootNode } = await rootNode.asDir().rm(["pictures", "cats"], store);
      var { result } = await rootNode.asDir().ls(["pictures"], store);

      return result;
    }, { cid });

    expect(result).toEqual(["dogs"]);
  });
});
