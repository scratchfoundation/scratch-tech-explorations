//! Terminology:
//! The `VM` is the overall runtime state of the Scratch project.
//! A `VM` contains one or more `Sprites` (the stage is a `Sprite` for this purpose).
//! Each `Sprite` contains:
//! - Zero or more `Scripts`. A `Script` is a sequence of `Blocks`.
//!   In the editor, this collection of `Scripts` is presented as a single blocks workspace.
//! - Zero or more `Costumes` (a backdrop is a `Costume` for this purpose).
//! - Zero or more `Sounds`.
//! From the perspective of the VM, all of the above are fixed data.
//! From the perspective of Bevy, all of the above are Resources or are contained within Resources.
//! The runtime state of each sprite is stored in a `Target`. A clone is also a `Target`.
//! The `Sprite` corresponds to at least one `Target`, and each `Target` corresponds to exactly one `Sprite`.
//! The `Sprite` corresponding to a `Target` contains the code that the `Target` is running.
//! The execution state (threads, instruction pointers, etc.) are associated with the `Target`, not the `Sprite`.
//! Note that this includes "sprite" variables and lists, which are actually stored at the `Target` level.
//! Conceptually, a `Sprite` is an executable file (or application bundle) and a `Target` is a running process.
//! The `Target` is represented by a Bundle in the Bevy world.
//! Bevy also has a concept of a `Sprite`. Bevy's `Sprite` is part of a `Target` bundle, not a Scratch `Sprite`.

use bevy::{
    prelude::*,
    utils::HashMap,
};

pub mod from_sb2;
pub mod load;
pub mod spawn;

// This represents the virtual machine state for a Scratch project.
// Ideally, loading a new Scratch project should mean throwing this away and replacing it with a new instance.
#[derive(Debug)]
pub struct VirtualMachine {
    pub sprites: Vec<Sprite>,
}

#[derive(Debug)]
pub struct Sprite {
    pub name: String,

    pub scripts: Vec<TopLevelItem>,
    pub sounds: Vec<Sound>,
    pub costumes: Vec<Costume>,

    pub targets: Vec<Target>,
}

#[derive(Debug)]
pub struct Target {
    pub x: f64,
    pub y: f64,
    pub scale: f64, // percentage: 100=100%
    pub direction: f64,
    pub rotation_style: RotationStyle,
    pub is_draggable: bool,
    pub is_visible: bool,
    pub variables: HashMap<String, Variable>,
    pub lists: HashMap<String, List>,
    pub current_costume: usize,
}

#[derive(Debug, Default)]
pub enum RotationStyle {
    #[default]
    Normal,
    LeftRight,
    None,
}

#[derive(Debug)]
pub struct TopLevelItem {
    pub x: f64,
    pub y: f64,
    pub stack: BlockStack,
}

#[derive(Debug)]
pub enum BlockStack {
    Script(Script),
    Definition(ProcedureDefinition),
}

pub type Script = Vec<Block>;

#[derive(Debug)]
pub struct Block {
    pub opcode: String,
    pub arguments: Vec<Argument>,
    pub branches: Vec<Script>,
}

#[derive(Debug)]
pub enum Argument {
    Expression(Block),
    Literal(Value),
}

#[derive(Debug)]
pub enum Value {
    Boolean(bool),
    Number(f64),
    String(String),
}

#[derive(Debug)]
pub struct ProcedureDefinition {
    pub spec: String,
    pub body: Script,
    pub parameter_names: Vec<String>,
    pub default_arguments: Vec<Value>,
    pub run_without_screen_refresh: bool,
}

#[derive(Debug)]
pub struct Variable {
    pub value: Value,
    pub is_cloud: bool,
}

#[derive(Debug)]
pub struct List {
    pub values: Vec<Value>,
    pub is_cloud: bool,
}

#[derive(Debug)]
pub struct Sound {
    pub name: String,
    pub audio_source: Handle<AudioSource>,
    pub format: String,
    pub sample_rate: i32,
    pub sample_count: i32,

    pub sound_index: i32,
}

#[derive(Debug)]
pub struct Costume {
    pub name: String,
    pub image: Handle<Image>,
    pub bitmap_resolution: i32,
    pub rotation_center_x: f64,
    pub rotation_center_y: f64,

    pub layer_index: i32,
}
