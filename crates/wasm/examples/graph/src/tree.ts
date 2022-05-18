import { PublicDirectory, PublicNode } from "../../../pkg";
import { Nullable, BlockStore } from "./types";

export class Vertex {
  name: string;
  node: Nullable<PublicNode>;
  isDir: boolean;
  tree: Tree;
  parentVertex: Nullable<Vertex>;
  rootDir: Nullable<PublicDirectory>;
  noRender: boolean;
  id: string;

  constructor(
    name: string,
    node: Nullable<PublicNode>,
    parentVertex: Nullable<Vertex>,
    tree: Tree,
    rootDir: Nullable<PublicDirectory> = null
  ) {
    // Information about the vertex itself.
    this.name = name;
    this.node = node;
    this.isDir = node ? node.isDir() : false;
    this.tree = tree;
    this.parentVertex = parentVertex;
    this.rootDir = rootDir;
    this.noRender = false;
    this.id = node ? node.getId() : "";
  }

  getRootVertexPath(): { path: string[]; rootVertex: Vertex } {
    const path: string[] = [];

    let vertex: Vertex = this;
    while (vertex.parentVertex) {
      path.push(vertex.name);
      vertex = vertex.parentVertex;
    }

    return {
      path: path.reverse(),
      rootVertex: vertex,
    };
  }
}

export class Tree {
  rootDir: Nullable<PublicDirectory>;
  store: BlockStore;
  levels: Vertex[][][];

  constructor(
    rootDir: PublicDirectory,
    store: BlockStore,
    levels: Vertex[][][] = []
  ) {
    this.rootDir = rootDir;
    this.store = store;
    this.levels =
      levels.length > 0
        ? levels
        : [[[new Vertex("root", rootDir.asNode(), null, this, rootDir)]]];
  }

  // Breadth-first traversal.
  async traverse() {
    const levels = this.levels;

    let currentLevel = levels[0];
    while (true) {
      currentLevel = await this.getChildren(currentLevel);
      if (currentLevel.length < 1) {
        break;
      }

      levels.push(currentLevel);
    }
  }

  async getChildren(level: Vertex[][]): Promise<Vertex[][]> {
    const newLevel: Vertex[][] = [];

    // Iterate vertex in each sibling list
    for (let siblings of level) {
      for (let vertex of siblings) {
        const children = await this.getChildrenForVertex(vertex);
        if (children.length > 0) {
          newLevel.push(children);
        }
      }
    }

    return newLevel;
  }

  async getChildrenForVertex(vertex: Vertex): Promise<Vertex[]> {
    if (vertex.node && vertex.node.isDir()) {
      const dir = vertex.node.asDir();
      const { result }: { result: string[] } = await dir.ls([], this.store);

      if (result.length > 0) {
        const children: Vertex[] = [];
        for (let name of result) {
          const node: PublicNode = await dir.lookupNode(name, this.store);
          children.push(new Vertex(name, node, vertex, this, vertex.rootDir));
        }

        return children;
      }
    }

    return [];
  }

  diff(previousTree: Tree): Nullable<Tree> {
    const levels: Vertex[][][] = [];

    // If previous tree has the same root node, we return null.
    if (
      this.rootDir &&
      previousTree.rootDir &&
      previousTree.rootDir.getId() === this.rootDir.getId()
    ) {
      return null;
    }

    const previousLevels = previousTree.levels.flat(3);

    for (let level of this.levels) {
      const newLevel = this.diffLevel(level, previousLevels);
      if (newLevel.length > 0) {
        levels.push(newLevel);
      }
    }

    return new Tree(this.rootDir as PublicDirectory, this.store, levels);
  }

  diffLevel(level: Vertex[][], previousLevels: Vertex[]) {
    const newLevel: Vertex[][] = [];
    for (let siblings of level) {
      const newSiblings = this.diffSiblings(siblings, previousLevels);
      if (newSiblings.length > 0) {
        newLevel.push(newSiblings);
      }
    }

    return newLevel;
  }

  diffSiblings(siblings: Vertex[], previousLevels: Vertex[]) {
    const newSiblings: Vertex[] = [];
    for (let vertex of siblings) {
      // We look for a vertex with similar id in previous tree.
      const prevVertex = previousLevels.find((prev) => prev.id === vertex.id);

      if (prevVertex) {
        // If vertex is found, we check if they don't share a common parent.
        // This means they are divergent. We skip non-divergent vertices.
        if (prevVertex.parentVertex?.id != vertex.parentVertex?.id) {
          let newVertex = new Vertex(
            vertex.name,
            vertex.node,
            vertex.parentVertex,
            this,
            this.rootDir
          );
          // We don't want to render duplicate vertices.
          newVertex.noRender = true;
          newSiblings.push(newVertex);
        }
      } else {
        // If vertex is not found, it also means they are divergent.
        newSiblings.push(vertex);
      }
    }

    return newSiblings;
  }
}
