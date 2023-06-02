//! Terminology:
//! The `VM` is the overall runtime state of the Scratch project.
//! A `VM` contains one or more `Sprites` (the stage is a `Sprite` for this purpose).
//! Each `Sprite` contains:
//! - Zero or more `Scripts`. A `Script` is a sequence of `Blocks`.
//!   In the editor, this collection of `Scripts` is presented as a single blocks workspace.
//! - Zero or more `Costumes` (a backdrop is a `Costume` for this purpose).
//! - Zero or more `Sounds`.
//! From the perspective of the VM, all of the above are fixed data.
//! From the perspective of Bevy, all of the above are Assets or are contained within the VM Resource.
//! The runtime state of each sprite is stored in a `Target`. A clone is also a `Target`.
//! The `Sprite` corresponds to at least one `Target`, and each `Target` corresponds to exactly one `Sprite`.
//! The `Sprite` corresponding to a `Target` contains the code that the `Target` is running.
//! The execution state (threads, instruction pointers, etc.) are associated with the `Target`, not the `Sprite`.
//! Note that this includes "sprite" variables and lists, which are actually stored at the `Target` level.
//! Conceptually, a `Sprite` is an executable file (or application bundle) and a `Target` is a running process.
//! The `Target` is represented by an `Entity` in the Bevy world. Its components refer to the assets and resources.
//! Bevy also has a concept of a `Sprite`. Bevy's `Sprite` is part of a Scratch `Target`, not a Scratch `Sprite`.

use bevy::{
    prelude::*,
    utils::HashMap, reflect::TypeUuid,
};

use crate::sb4;

pub mod load;
pub mod spawn;

// This represents the virtual machine state for a Scratch project.
// Ideally, loading a new Scratch project should mean throwing this away and replacing it with a new instance.
#[derive(Debug)]
pub struct VirtualMachine {
    pub sprites: Vec<Handle<Sprite>>,
}

#[derive(Debug)]
#[derive(TypeUuid)]
#[uuid = "7f554a39-0c3c-46c4-8c7a-e1dbf47d2ce5"]
pub struct Sprite {
    pub name: String,

    pub scripts: Vec<sb4::TopLevelItem>,
    pub sounds: Vec<sb4::Sound>,
    pub costumes: Vec<sb4::Costume>,
}

#[derive(Debug)]
#[derive(Component)]
pub struct Target {
    pub x: f64,
    pub y: f64,
    pub scale: f64, // percentage: 100=100%
    pub direction: f64,
    pub rotation_style: sb4::RotationStyle,
    pub is_draggable: bool,
    pub is_visible: bool,
    pub variables: HashMap<String, sb4::Variable>,
    pub lists: HashMap<String, sb4::List>,
    pub current_costume: usize,
    pub sprite: Handle<Sprite>,
}
