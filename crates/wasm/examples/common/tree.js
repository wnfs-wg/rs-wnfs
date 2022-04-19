// import * as arrowLine from '/arrow-line.min.js';

class Vertex {
  constructor(name, node, parent) {
    this.name = name;
    this.node = node;
    this.parent = parent;
    this.id = this.node.getId();
  }
}

export class Tree {
  constructor(rootNode, store) {
    this.rootNode = rootNode;
    this.store = store;
    this.levels = [[[new Vertex("root", rootNode, null)]]];
  }

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

    return [];
  }

  diff() {
    // TODO(appcypher): Implement diffing
  }
}

export class Render {
  constructor(tree) {
    this.tree = tree;
  }

  render() {
    const graphTree = document.createElement("div");
    graphTree.className = "graph-tree";

    for (let level of this.tree.levels) {
      const levelDiv = document.createElement("div");
      levelDiv.className = "level";

      for (let siblings of level) {
        const siblingDiv = document.createElement("div");
        siblingDiv.className = "sibling";

        for (let vertex of siblings) {
          const vertexDiv = document.createElement("div");
          vertexDiv.className = "vertex";
          vertexDiv.id = `_${vertex.id}`;
          vertexDiv.innerText = `${vertex.name} (${vertex.id})`;
          vertexDiv.addEventListener("mouseover", this.handleVertexHover);
          siblingDiv.appendChild(vertexDiv);
        }

        levelDiv.appendChild(siblingDiv);
      }

      graphTree.appendChild(levelDiv);
    }

    this.connectVertices();

    return graphTree;
  }

  connectVertices() {
    // arrowLine("#_0x13ba88", "#_0x13a1d0");
  }

  handleVertexHover() {}
}
