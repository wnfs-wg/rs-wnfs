import { Tree } from "./tree.js";

export class Render {
  static activeHandlers = new Map();
  static addFileElement = document.getElementById("add-file");
  static addFolderElement = document.getElementById("add-folder");
  static deleteNodeElement = document.getElementById("delete-node");
  static menuElement = document.getElementById("menu");
  static inputElement = document.getElementById("input");
  static inputBoxElement = document.getElementById("input-box");
  static inputButtonElement = document.getElementById("input-button");
  static activityBodyElement = document.getElementById("activity-panel-body");

  constructor(tree, rootElement) {
    this.tree = tree;
    this.rootElement = rootElement;
    this.rootElementBounds = rootElement.getBoundingClientRect();
  }

  render() {
    // We don't render tree if tree is null.
    if (!this.tree) {
      console.log("No change to tree");
      return;
    }

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
      vertexNameDiv.innerText = `${vertex.name}`;
      vertexDiv.appendChild(vertexNameDiv);

      const vertexIdDiv = document.createElement("div");
      vertexIdDiv.className = "vertex-id";
      vertexIdDiv.innerText = `#${vertex.id}`;
      vertexDiv.appendChild(vertexIdDiv);

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
      if (vertex.parentVertex) {
        connections.push([
          `_${vertex.parentVertex.id}`,
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

      // Factor in rootElement scroll position.
      // this.rootElement.scrollTop = this.rootElement.scrollTop + 1;

      // Update leaderLine element position.
      const leaderLineLeft = parseFloat(leaderLine.style.left.slice(0, -2));
      const leaderLineTop = parseFloat(leaderLine.style.top.slice(0, -2));

      leaderLine.style.top =
        leaderLineTop -
        this.rootElementBounds.top +
        this.rootElement.scrollTop +
        "px";

      leaderLine.style.left =
        leaderLineLeft -
        this.rootElementBounds.left +
        this.rootElement.scrollLeft +
        "px";
    });
  }

  handleContextMenu = (event, vertex) => {
    // Show input and hide all menu action items.
    Render.inputElement.classList.add("hide");
    Render.addFileElement.classList.remove("hide");
    Render.addFolderElement.classList.remove("hide");
    Render.deleteNodeElement.classList.remove("hide");

    event.preventDefault();

    // Cleanup.
    Render.cleanupHandlers();

    // Show menu.
    Render.menuElement.classList.remove("veil");

    // Update menu position.
    Render.menuElement.style.left = event.clientX + "px";
    Render.menuElement.style.top = event.clientY + "px";

    // Show `add` menu items if the vertex is a directory.
    if (vertex.isDir) {
      Render.addFileElement.classList.remove("hide");
      Render.addFolderElement.classList.remove("hide");
    } else {
      Render.addFileElement.classList.add("hide");
      Render.addFolderElement.classList.add("hide");
    }

    // Set handlers on menu action items.
    const handler = async (event) => {
      await this.handleContextMenuItemClick(event, vertex);
    };

    Render.attachHandler(Render.addFileElement, "click", handler);
    Render.attachHandler(Render.addFolderElement, "click", handler);
    Render.attachHandler(Render.deleteNodeElement, "click", handler);
  };

  handleContextMenuItemClick = async (event, vertex) => {
    const eventName = event.target.id;
    if (eventName !== "delete-node") {
      // Show input and hide all menu action items.
      Render.inputElement.classList.remove("hide");
      Render.addFileElement.classList.add("hide");
      Render.addFolderElement.classList.add("hide");
      Render.deleteNodeElement.classList.add("hide");

      // Focus on input box.
      Render.inputBoxElement.focus();

      // Handle click on input button.
      Render.attachHandler(
        Render.inputButtonElement,
        "click",
        this.handleInputButtonClick(vertex, eventName)
      );

      // Detect enter key press in input box.
      Render.attachHandler(Render.inputBoxElement, "keypress", (event) => {
        switch (event.key) {
          case "Enter":
            this.handleInputButtonClick(vertex, eventName)(event);
            break;
          case " ":
            event.preventDefault();
            return;
        }
      });
    } else {
      await this.handleInputButtonClick(vertex, eventName)(event);
    }
  };

  handleInputButtonClick = (vertex, eventName) => {
    const inner = async (event) => {
      const path_segments = Render.inputBoxElement.value
        .split("/")
        .filter((v) => v !== "");

      try {
        // Dispatch action.
        switch (eventName) {
          case "add-folder": {
            await Render.handleAddFolder(vertex, path_segments);
            break;
          }
          case "add-file": {
            await Render.handleAddFile(vertex, path_segments);
            break;
          }
          case "delete-node": {
            await Render.handleDeleteNode(vertex);
            break;
          }
        }
      } finally {
        // Show menu action items.
        Render.addFileElement.classList.remove("hide");
        Render.addFolderElement.classList.remove("hide");
        Render.deleteNodeElement.classList.remove("hide");

        // Hide input and the menu itself.
        Render.menuElement.classList.add("veil");
        Render.inputElement.classList.add("hide");

        // Reset input element value
        Render.inputBoxElement.value = "";
      }
    };

    return inner;
  };

  static async handleAddFolder(vertex, path_segments) {
    const { graphRootElement, store } = globalThis.graph;
    let { rootNode, tree } = vertex;
    let full_path_segments = [
      ...vertex.getRootVertexPath().path,
      ...path_segments,
    ];

    // Create new directory.
    ({ rootNode } = await rootNode
      .asDir()
      .mkdir(full_path_segments, new Date(), store));

    const path_string = `/${full_path_segments.join("/")}`;
    console.log(`mkdir -p ${path_string}`);

    const item = document.createElement("div");
    item.className = "item";
    item.innerText = `create folder ${path_string}`;
    Render.activityBodyElement.appendChild(item);

    // Draw new tree.
    await draw(rootNode, store, graphRootElement, tree);
  }

  static async handleAddFile(vertex, path_segments) {
    const { graphRootElement, store } = globalThis.graph;
    let { rootNode, tree } = vertex;
    let full_path_segments = [
      ...vertex.getRootVertexPath().path,
      ...path_segments,
    ];

    // Create new file.
    ({ rootNode } = await rootNode
      .asDir()
      .write(
        full_path_segments,
        "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi",
        new Date(),
        store
      ));

    const path_string = `/${full_path_segments.join("/")}`;
    console.log(`echo "" >> ${path_string}`);

    const item = document.createElement("div");
    item.className = "item";
    item.innerText = `write file ${path_string}`;
    Render.activityBodyElement.appendChild(item);

    // Draw new tree.
    await draw(rootNode, store, graphRootElement, tree);
  }

  static async handleDeleteNode(vertex) {
    const { graphRootElement, store } = globalThis.graph;
    let { rootNode, tree } = vertex;
    let { path } = vertex.getRootVertexPath();

    ({ rootNode } = await rootNode.asDir().rm(path, store));

    console.log(`rm /${path.join("/")}`);

    const item = document.createElement("div");
    item.className = "item";
    item.innerText = `remove /${path.join("/")}`;
    Render.activityBodyElement.appendChild(item);

    // Draw new tree.
    await draw(rootNode, store, graphRootElement, tree);
  }

  static attachHandler(element, eventName, handler) {
    element.addEventListener(eventName, handler);
    Render.activeHandlers.set(element, [eventName, handler]);
  }

  static cleanupHandlers() {
    for (let [
      element,
      [eventType, handler],
    ] of Render.activeHandlers.entries()) {
      element.removeEventListener(eventType, handler);
    }
  }
}

// Handles click on root element.
const handleRootClick = () => {
  const menuElement = document.getElementById("menu");
  const inputElement = document.getElementById("input");
  inputElement.classList.add("hide");
  menuElement.classList.add("veil");
};

export const draw = async (
  rootNode,
  store,
  rootElement,
  previousTree = null
) => {
  // Create tree based on rootNode.
  let tree = new Tree(rootNode, store);
  await tree.traverse();

  // Keep track of the tree.
  const fullTree = tree;

  // If previous tree is supplied, we get the diff tree.
  if (previousTree) {
    tree = tree.diff(previousTree);
  }

  // Render tree.
  new Render(tree, rootElement).render();

  // Add click listener to root element.
  rootElement.addEventListener("click", handleRootClick);

  return fullTree;
};
