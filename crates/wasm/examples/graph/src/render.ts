///<reference path="index.d.ts"/>

import { CID } from "multiformats/cid";
import { PublicDirectory, PublicNode } from "../../../pkg/index";
import { Tree, Vertex } from "./tree";
import { Nullable, BlockStore, EventMap, Connection, Handler } from "./types";

//------------------------------------------------------------------------------
// Handlers
//------------------------------------------------------------------------------

const handleRootClick = () => {
  const menuElement = document.getElementById("menu") as HTMLElement;
  const inputElement = document.getElementById("input") as HTMLInputElement;

  inputElement.classList.add("hide");
  menuElement.classList.add("veil");
};

const handleActivityHeadClick = () => {
  const activityBodyElement = document.getElementById(
    "activity-panel-body"
  ) as HTMLElement;
  activityBodyElement.classList.toggle("hide-sm");
};

//------------------------------------------------------------------------------
// Init
//------------------------------------------------------------------------------

const graphRootElement = document.getElementById("graph-canvas") as HTMLElement;
const activityHeadElement = document.getElementById(
  "activity-panel-head"
) as HTMLElement;

graphRootElement.addEventListener("click", handleRootClick);
activityHeadElement.addEventListener("click", handleActivityHeadClick);

//------------------------------------------------------------------------------
// Render Class
//------------------------------------------------------------------------------

export class Render {
  tree: Nullable<Tree>;
  rootElement: HTMLElement;
  rootElementBounds: DOMRect;

  static activeHandlers: EventMap = new Map();
  static addFileElement = document.getElementById("add-file") as HTMLElement;
  static addFolderElement = document.getElementById(
    "add-folder"
  ) as HTMLElement;
  static deleteNodeElement = document.getElementById(
    "delete-node"
  ) as HTMLElement;
  static menuElement = document.getElementById("menu") as HTMLElement;
  static inputElement = document.getElementById("input") as HTMLElement;
  static inputBoxElement = document.getElementById(
    "input-box"
  ) as HTMLInputElement;
  static inputButtonElement = document.getElementById(
    "input-button"
  ) as HTMLElement;
  static activityBodyElement = document.getElementById(
    "activity-panel-body"
  ) as HTMLElement;

  constructor(tree: Nullable<Tree>, rootElement: HTMLElement) {
    this.tree = tree;
    this.rootElement = rootElement;
    this.rootElementBounds = rootElement.getBoundingClientRect();
  }

  render(): Nullable<HTMLElement> {
    // We don't render tree if tree is null.
    if (!this.tree) {
      console.log("No change to tree");
      return null;
    }

    const graphTree = document.createElement("div");
    graphTree.className = "graph-tree";

    const connections: Connection[] = [];
    for (let level of this.tree.levels) {
      const [levelDiv, newConnections] = this.addLevelSiblings(level);

      graphTree.appendChild(levelDiv);
      connections.push(...newConnections);
    }

    this.rootElement.appendChild(graphTree);
    this.connectVertices(connections);

    return graphTree;
  }

  addLevelSiblings(level: Vertex[][]): [HTMLElement, Connection[]] {
    const levelDiv = document.createElement("div");
    levelDiv.className = "level";

    const connections: Connection[] = [];
    for (let siblings of level) {
      const [siblingDiv, newConnections] = this.addSiblingVertices(siblings);

      levelDiv.appendChild(siblingDiv);
      connections.push(...newConnections);
    }

    return [levelDiv, connections];
  }

  addSiblingVertices(siblings: Vertex[]): [HTMLElement, Connection[]] {
    const siblingDiv = document.createElement("div");
    siblingDiv.className = "sibling";

    const connections: Connection[] = [];
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
      vertexDiv.addEventListener("contextmenu", (event: MouseEvent) =>
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

  connectVertices(connections: Connection[]) {
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
      const leaderLine = document.querySelector(
        "body > .leader-line"
      ) as HTMLElement;
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

  handleContextMenu = (event: MouseEvent, vertex: Vertex) => {
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
    const handler = async (event: Event): Promise<void> => {
      await this.handleContextMenuItemClick(event, vertex);
    };

    Render.attachHandler(Render.addFileElement, "click", handler);
    Render.attachHandler(Render.addFolderElement, "click", handler);
    Render.attachHandler(Render.deleteNodeElement, "click", handler);
  };

  handleContextMenuItemClick = async (event: Event, vertex: Vertex) => {
    const eventName = (event.target as HTMLElement).id;
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
      Render.attachHandler(
        Render.inputBoxElement,
        "keypress",
        (event: Event) => {
          switch ((event as KeyboardEvent).key) {
            case "Enter":
              this.handleInputButtonClick(vertex, eventName)();
              break;
            case " ":
              event.preventDefault();
              return;
          }
        }
      );
    } else {
      await this.handleInputButtonClick(vertex, eventName)();
    }
  };

  handleInputButtonClick = (vertex: Vertex, eventName: string) => {
    const inner = async () => {
      const path_segments = Render.inputBoxElement.value
        .split("/")
        .filter((v: string) => v !== "");

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

  static async handleAddFolder(vertex: Vertex, path_segments: string[]) {
    const { store } = globalThis;
    let { rootDir, tree } = vertex;
    let full_path_segments = [
      ...vertex.getRootVertexPath().path,
      ...path_segments,
    ];

    // Create new directory.
    ({ rootDir } = await (rootDir as PublicDirectory).mkdir(
      full_path_segments,
      new Date(),
      store
    ));

    const path_string = `/${full_path_segments.join("/")}`;
    console.log(`mkdir -p ${path_string}`);

    const item = document.createElement("div");
    item.className = "item";
    item.innerText = `create folder ${path_string}`;
    Render.activityBodyElement.appendChild(item);

    // Draw new tree.
    await draw(rootDir as PublicDirectory, store, tree);
  }

  static async handleAddFile(vertex: Vertex, path_segments: string[]) {
    const { store } = globalThis;
    let { rootDir, tree } = vertex;
    let full_path_segments = [
      ...vertex.getRootVertexPath().path,
      ...path_segments,
    ];

    // Create mock cid

    /** A mock CID. */
    const cid = CID.parse(
      "bagaaierasords4njcts6vs7qvdjfcvgnume4hqohf65zsfguprqphs3icwea"
    ).bytes;

    // Create new file.
    ({ rootDir } = await (rootDir as PublicDirectory).write(
      full_path_segments,
      cid,
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
    await draw(rootDir as PublicDirectory, store, tree);
  }

  static async handleDeleteNode(vertex: Vertex) {
    const { store } = globalThis;
    let { rootDir, tree } = vertex;
    let { path } = vertex.getRootVertexPath();

    ({ rootDir } = await (rootDir as PublicDirectory).rm(path, store));

    console.log(`rm /${path.join("/")}`);

    const item = document.createElement("div");
    item.className = "item";
    item.innerText = `remove /${path.join("/")}`;
    Render.activityBodyElement.appendChild(item);

    // Draw new tree.
    await draw(rootDir as PublicDirectory, store, tree);
  }

  static attachHandler(
    element: HTMLElement,
    eventName: string,
    handler: Handler
  ) {
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

//------------------------------------------------------------------------------
// Draw
//------------------------------------------------------------------------------

export const draw = async (
  rootDir: PublicDirectory,
  store: BlockStore,
  previousTree: Nullable<Tree> = null
): Promise<Tree> => {
  // Create tree based on rootDir.
  let tree: Nullable<Tree> = new Tree(rootDir, store);
  await tree.traverse();

  // Keep track of the tree.
  const fullTree = tree;

  // If previous tree is supplied, we get the diff tree.
  if (previousTree) {
    tree = tree.diff(previousTree);
  }

  // Render tree.
  new Render(tree, graphRootElement).render();

  return fullTree;
};
