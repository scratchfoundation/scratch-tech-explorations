# Scratch using Tauri+Bevy

This branch is for exploring the idea of building a Scratch editor using [Tauri](https://tauri.app/) and
[Bevy](https://bevyengine.org/). The specific proposed architecture:

## Quick start

Install system prerequisites if necessary: <https://tauri.app/v1/guides/getting-started/prerequisites>

`cargo tauri dev`

## Recommended IDE Setup

Recommendation from Tauri:

[VS Code](https://code.visualstudio.com/) +
  [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) +
  [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## App architecture

* Tauri will display the Scratch UI, analogous to `scratch-gui`, in a web view
  * This will also wrap Blockly, the paint editor, and other interactive editor elements
* The web view will include an HTML canvas for the Scratch stage, similar to the current Scratch 3.0 layout
* The stage will be rendered by a small Bevy module compiled to WASM, analogous to `scratch-render`
* Another Bevy module, compiled to native code, will run everything that doesn't interact directly with the stage:
  * VM and runtime features similar to `scratch-vm`
  * Audio support similar to `scratch-audio`
  * Hardware communication for extensions
  * etc.
* Communication between the render/web side and native side will be handled through Tauri's IPC-like features

If Tauri adds support for externally rendering to a canvas (see
[here](https://github.com/tauri-apps/wry/discussions/284)), then the renderer can move to the native side. That should
be even better for performance and should simplify communication as well.

## Web architecture

On the web, all modules will be compiled to WASM and the communication between the rendering module and the rest of
the editor will be more direct. If possible, the rendering and other features should still run in separate threads.
