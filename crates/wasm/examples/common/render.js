import { Tree } from "./tree.js";

export class Render {
  constructor(tree, rootElement) {
    this.tree = tree;
    this.rootElement = rootElement;
    this.rootElementBounds = rootElement.getBoundingClientRect();
    this.rootElement.addEventListener("click", this.handleRootClick);
  }

  render() {
    const graphTree = document.createElement("div");
    graphTree.className = "graph-tree";

    const connections = [];
    for (let level of this.tree.levels) {
      const [levelDiv, newConnections] = this.addLevelSiblings(level);
      graphTree.appendChild(levelDiv);
      connections.push(...newConnections);
    }

    this.rootElement.appendChild(graphTree);
    this.connectVertices(connections);

    return graphTree;
  }

  addLevelSiblings(level) {
    const levelDiv = document.createElement("div");
    levelDiv.className = "level";

    const connections = [];
    for (let siblings of level) {
      const [siblingDiv, newConnections] = this.addSiblingVertices(siblings);
      levelDiv.appendChild(siblingDiv);
      connections.push(...newConnections);
    }

    return [levelDiv, connections];
  }

  addSiblingVertices(siblings) {
    const siblingDiv = document.createElement("div");
    siblingDiv.className = "sibling";

    const connections = [];

    for (let vertex of siblings) {
      const vertexDiv = document.createElement("div");
      vertexDiv.className = "vertex" + (!vertex.isDir ? " file" : "");
      vertexDiv.id = `_${vertex.id}`;

      const vertexNameDiv = document.createElement("div");
      vertexNameDiv.className = "vertex-name";
      vertexNameDiv.innerText = `${vertex.name} #${vertex.id}`;
      vertexDiv.appendChild(vertexNameDiv);

      // Add context menu handler.
      vertexDiv.addEventListener("contextmenu", (event) =>
        this.handleContextMenu(event, vertex)
      );

      // Add vertex to sibling div if it can be rendered.
      let longConnection = true;
      if (!vertex.noRender) {
        siblingDiv.appendChild(vertexDiv);
        longConnection = false;
      }

      // Add connection to parent.
      if (vertex.parent) {
        connections.push([
          `_${vertex.parent.id}`,
          `_${vertex.id}`,
          longConnection,
        ]);
      }
    }

    return [siblingDiv, connections];
  }

  connectVertices(connections) {
    connections.forEach(([parentId, childId, longConnection]) => {
      new LeaderLine(
        document.getElementById(parentId),
        document.getElementById(childId),
        {
          color: longConnection ? "#76D7C4" : "#C39BD3",
          size: 2,
          ...(longConnection && { dash: { animation: true } }),
        }
      );

      // Move line element from body to rootElment.
      const leaderLine = document.querySelector("body > .leader-line");
      this.rootElement.appendChild(leaderLine);

      // Update leaderLine element position.
      const leaderLineLeft = parseFloat(leaderLine.style.left.slice(0, -2));
      const leaderLineTop = parseFloat(leaderLine.style.top.slice(0, -2));

      leaderLine.style.top = leaderLineTop - this.rootElementBounds.top + "px";
      leaderLine.style.left =
        leaderLineLeft - this.rootElementBounds.left + "px";
    });
  }

  handleContextMenu(event, vertex) {
    event.preventDefault();

    // Show menu.
    const menuElement = document.getElementById("menu");
    menuElement.classList.remove("veil");

    // Update menu position.
    menuElement.style.left = event.clientX + "px";
    menuElement.style.top = event.clientY + "px";

    // Get menu items.
    const addFileElement = document.getElementById("add-file");
    const addFolderElement = document.getElementById("add-folder");
    const deleteNodeElement = document.getElementById("delete-node");

    // Show `add` menu items if the vertex is a directory.
    if (vertex.isDir) {
      addFileElement.classList.remove("hide");
      addFolderElement.classList.remove("hide");
    } else {
      addFileElement.classList.add("hide");
      addFolderElement.classList.add("hide");
    }

    // Create a menuItemHandler.
    const handler = menuItemHandler(vertex, menuElement, [
      addFolderElement,
      addFileElement,
      deleteNodeElement,
    ]);

    addFileElement.addEventListener("click", handler);
    addFolderElement.addEventListener("click", handler);
    deleteNodeElement.addEventListener("click", handler);
  }

  handleRootClick() {
    let menuElement = document.getElementById("menu");
    menuElement.classList.add("veil");
  }
}

const menuItemHandler = (vertex, menuElement, menuItemElements) => {
  const inner = async (event) => {
    const { graphRootElement, store } = globalThis.scope;
    let { rootNode, tree } = globalThis.scope;
    
    // Dispatch action.
    // TODO(appcypher): Move into separate functions
    switch (event.target.id) {
      case "add-folder": {
        let { path, rootVertex } = vertex.getRootVertexPath();
        // const newFolderName = prompt("Enter new folder name:"); // TODO(appcypher): Use custom element.
        const newFolderName = "TestFolder";

        if (newFolderName) {
          path = [...path, newFolderName];
          ({ rootNode } = await rootVertex.node
            .asDir()
            .mkdir(path, new Date(), store));
        }

        break;
      }
      case "add-file": {
        let { path, rootVertex } = vertex.getRootVertexPath();
        // const newFileName = prompt("Enter new file name:"); // TODO(appcypher): Use custom element.
        const newFileName = "TestFile";

        if (newFileName) {
          path = [...path, newFileName];
          ({ rootNode } = await rootVertex.node
            .asDir()
            .write(
              path,
              "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi",
              new Date(),
              store
            ));
        }

        break;
      }
      case "delete-node": {
        const { path, rootVertex } = vertex.getRootVertexPath();

        ({ rootNode } = await rootVertex.node
          .asDir()
          .rm(["pictures", "cats", "kitten.png"], store));

        break;
      }
    }

    // Draw new tree.
    tree = await draw(rootNode, store, graphRootElement, tree);

    // Update global scope.
    globalThis.scope.tree = tree;
    globalThis.scope.rootNode = rootNode;

    // Hide menu.
    menuElement.classList.add("veil");

    // Clean up.
    menuItemElements.forEach((menuItemElement) => {
      menuItemElement.removeEventListener("click", inner);
    });
  };

  return inner;
};

export const draw = async (
  rootNode,
  store,
  rootElement,
  previousTree = null
) => {
  let tree = new Tree(rootNode, store);
  let fullTree = tree;

  await tree.traverse();

  console.log("Previous tree...", previousTree);

  // If previous tree is supplied, we get the diff tree.
  if (previousTree) {
    tree = tree.diff(previousTree);
  }

  new Render(tree, rootElement).render();

  return fullTree;
};
