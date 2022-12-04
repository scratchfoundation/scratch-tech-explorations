import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div id="gui" style={{position: "absolute", top: 0, right: 0, bottom: 0, left: 0}}>
      <div id="menu-bar" style={{display: "flex", flexDirection:"row"}}>
        <button>File</button>
        <button>Edit</button>
        <button>Things</button>
        <button>Stuff</button>
      </div>
      <div id="body" style={{display: "flex", flexDirection: "row", height: "100%"}}>
        <div id="editor-wrapper" style={{flexGrow: "1"}}>
          <div id="tabs" style={{ display: "flex", flexDirection: "column", height: "100%" }}>
            <ul id="tablist" style={{ display: "flex", flexDirection: "row" }}>
              <button>Code</button>
              <button>Costumes</button>
              <button>Sounds</button>
            </ul>
            <div id="code-tab" style={{ height: "100%", border: "1px solid black" }}>
              Blockly goes here
            </div>
          </div>
        </div>
        <div id="stage-and-targets">
          <div id="stage-controls" style={{ display: "flex", flexDirection: "row" }}>
            <button style={{color: "green"}}>⚑</button>
          </div>
          <div id="stage-wrapper" style={{border: "1px solid black"}}>
            <canvas width={480} height={360}></canvas>
          </div>
          <div id="targets-wrapper" style={{ height: "100%", border: "1px solid black" }}>
            Sprites and the Stage
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
