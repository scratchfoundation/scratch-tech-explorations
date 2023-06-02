//! This module defines the structure of a ready-to-install project for the current version of the VM.
//! This is "frozen" data and must be loaded into a VM before it can run.
//! This separates installing/spawning the project from conversion of SB2, SB3, etc. into the current format.

pub mod from_sb2;

use bevy::{
    prelude::{
        AudioSource,
        Handle,
        Image,
    },
    utils::HashMap,
};

#[derive(Debug)]
pub struct Project {
    pub sprites: Vec<Sprite>,
}

#[derive(Debug)]
pub struct Sprite {
    pub name: String,

    pub scripts: Vec<TopLevelItem>,
    pub sounds: Vec<Sound>,
    pub costumes: Vec<Costume>,

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
pub struct Costume {
    pub name: String,
    pub image: Handle<Image>,
    pub bitmap_resolution: i32,
    pub rotation_center_x: f64,
    pub rotation_center_y: f64,

    pub layer_index: i32,
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
