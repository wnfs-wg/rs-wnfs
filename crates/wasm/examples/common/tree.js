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

  diff() {
    // TODO(appcypher): Implement diffing
  }
}

export class Render {
  constructor(tree, rootElement) {
    this.tree = tree;
    this.rootElement = rootElement;
    this.rootElement.addEventListener("click", this.handleRootClick);
  }

  render() {
    const graphTree = document.createElement("div");
    graphTree.className = "graph-tree";

    let connections = [];
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

          const vertexNameDiv = document.createElement("div");
          vertexNameDiv.className = "vertex-name";
          vertexNameDiv.innerText = `${vertex.name} #${vertex.id}`;
          vertexDiv.appendChild(vertexNameDiv);

          // Add connection to parent.
          if (vertex.parent) {
            connections.push([`#_${vertex.parent.id}`, `#_${vertex.id}`]);
          }

          // Events
          vertexDiv.addEventListener("contextmenu", this.handleContextMenu);

          // Add vertex to sibling div
          siblingDiv.appendChild(vertexDiv);
        }

        levelDiv.appendChild(siblingDiv);
      }

      graphTree.appendChild(levelDiv);
    }

    this.rootElement.appendChild(graphTree);
    this.connectVertices(connections);

    return graphTree;
  }

  connectVertices(connections) {
    connections.forEach(([parentId, childId]) => {
      arrowLine(parentId, childId, { thickness: 1, color: "#C39BD3" });
    });
  }

  handleContextMenu(event) {
    event.preventDefault();
    const menuElement = document.getElementById("menu");
    menuElement.classList.remove("hide");
    menuElement.style.left = event.clientX + "px";
    menuElement.style.top = event.clientY + "px";

    const addNodeElement = document.getElementById("add-node");
    const deleteNodeElement = document.getElementById("delete-node");

    // function addNodeHandler() {
    //   console.log("Add node clicked from: ", this);
    // }

    // function deleteNodeHandler() {
    //   console.log("Delete node clicked from: ", this);
    // }

    // addNodeElement.removeEventListener("click", addNodeHandler);
    // deleteNodeElement.removeEventListener("click", deleteNodeHandler);
    // addNodeElement.addEventListener("click", addNodeHandler, { once: true });
    // deleteNodeElement.addEventListener("click", deleteNodeHandler, {
    //   once: true,
    // });
  }

  handleRootClick() {
    let menuElement = document.getElementById("menu");
    menuElement.classList.add("hide");
  }
}

export const draw = async(rootNode, store, rootElement) => {
  let tree = new Tree(rootNode, store);

  await tree.traverse();

  console.log("tree levels", tree.levels);

  new Render(tree, rootElement).render();
};
