use bevy::utils::HashMap;

mod from_sb2;

// This represents the virtual machine state for a Scratch project.
// Ideally, loading a new Scratch project should mean throwing this away and replacing it with a new instance.
#[derive(Debug)]
pub struct VirtualMachine {
    pub targets: Vec<Target>,
}

#[derive(Debug)]
pub struct Target {
    pub name: String,

    pub x: f64,
    pub y: f64,
    pub scale: f64,
    pub direction: f64,
    pub rotation_style: RotationStyle,
    pub is_draggable: bool,
    pub is_visible: bool,

    pub scripts: Vec<TopLevelItem>,
    pub variables: HashMap<String, Variable>,
    pub lists: HashMap<String, List>,
    pub sounds: Vec<Sound>,
    pub costumes: Vec<Costume>,

    pub current_costume: i32,
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
    pub md5: String,
    pub format: String,
    pub sample_rate: i32,
    pub sample_count: i32,

    pub sound_index: i32,
}

#[derive(Debug)]
pub struct Costume {
    pub name: String,
    pub md5: String,
    pub bitmap_resolution: i32,
    pub rotation_center_x: f64,
    pub rotation_center_y: f64,

    pub layer_index: i32,
}
