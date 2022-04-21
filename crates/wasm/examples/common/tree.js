// import * as arrowLine from '/arrow-line.min.js';

class Vertex {
  constructor(name, node, parent) {
    this.name = name;
    this.node = node;
    this.isDir = node.isDir();
    this.parent = parent;
    this.noRender = false;
    this.id = this.node.getId();
  }

  getRootVertexPath() {
    let path = [];
    let vertex = this;

    while (vertex.parent) {
      path.push(vertex.name);
      vertex = vertex.parent;
    }

    return {
      path: path.reverse(),
      rootVertex: vertex,
    };
  }
}

export class Tree {
  constructor(rootNode, store, levels = []) {
    this.rootNode = rootNode;
    this.store = store;
    this.levels =
      levels.length > 0 ? levels : [[[new Vertex("root", rootNode, null)]]];
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

  async getChildren(level) {
    const newLevel = [];

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

  async getChildrenForVertex(vertex) {
    if (vertex.node.isDir()) {
      const dir = vertex.node.asDir();
      const { result } = await dir.ls([], this.store);

      if (result.length > 0) {
        let children = [];
        for (let name of result) {
          const node = await dir.lookupNode(name, this.store);
          children.push(new Vertex(name, node, vertex));
        }

        return children;
      }
    }

    return [];
  }

  diff(previousTree) {
    const levels = [];
    const previousLevels = previousTree.levels.flat(3);

    for (let level of this.levels) {
      const newLevel = this.diffLevel(level, previousLevels);
      if (newLevel.length > 0) {
        levels.push(newLevel);
      }
    }

    return new Tree(this.rootNode, this.store, levels);
  }

  diffLevel(level, previousLevels) {
    const newLevel = [];
    for (let siblings of level) {
      const newSiblings = this.diffSiblings(siblings, previousLevels);
      if (newSiblings.length > 0) {
        newLevel.push(newSiblings);
      }
    }

    return newLevel;
  }

  diffSiblings(siblings, previousLevels) {
    const newSiblings = [];
    for (let vertex of siblings) {
      // We look for a vertex with similar id in previous tree.
      const previousVertex = previousLevels.find(
        (prevVertex) => prevVertex.id === vertex.id
      );

      if (previousVertex) {
        // If vertex is found, we check if they don't share a common parent.
        // This means they are divergent. We skip non-divergent vertices.
        if (previousVertex.parent?.id != vertex.parent?.id) {
          let newVertex = new Vertex(vertex.name, vertex.node, vertex.parent);
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
