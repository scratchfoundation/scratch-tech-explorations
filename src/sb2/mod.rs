pub mod load;

use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, Hash, PartialEq)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct BlockId(String);

#[derive(Debug, Eq, Hash, PartialEq)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ListId(String);

#[derive(Debug, Eq, Hash, PartialEq)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct VariableId(String);

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct Project {
    #[serde(flatten)]
    pub stage: Stage,

    pub children: Vec<StageChild>,

    pub info: HashMap<String, serde_json::Value>,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Stage {
    #[serde(default, rename="penLayerMD5")]
    pub pen_layer_md5: String,

    #[serde(default, rename="penLayerID")]
    pub pen_layer_id: i32,

    #[serde(default, rename="tempoBPM")]
    pub tempo_bpm: f64,

    #[serde(default)]
    pub video_alpha: f64,

    #[serde(flatten)]
    pub target: Target,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum StageChild {
    Sprite(Sprite),
    Monitor(Monitor),
    List(List),
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Sprite {
    #[serde(rename="scratchX")]
    pub x: f64,

    #[serde(rename="scratchY")]
    pub y: f64,

    pub scale: f64,
    pub direction: f64,

    pub rotation_style: RotationStyle,

    pub is_draggable: bool,

    pub index_in_library: i32,

    #[serde(rename="visible")]
    pub is_visible: bool,

    pub sprite_info: serde_json::Value,

    #[serde(flatten)]
    pub target: Target,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Target {
    #[serde(rename="objName")]
    pub name: String,

    #[serde(default)]
    pub scripts: Vec<serde_json::Value>, // TODO

    #[serde(default)]
    pub variables: Vec<Variable>, // TODO: HashMap

    #[serde(default)]
    pub lists: Vec<List>, // TODO: HashMap

    #[serde(default)]
    pub sounds: Vec<serde_json::Value>, // TODO

    #[serde(default)]
    pub costumes: Vec<serde_json::Value>, // TODO

    pub current_costume_index: i32,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Monitor {
    pub target: String,
    pub cmd: String,
    pub param: String,
    pub color: i32,
    pub label: String,
    pub mode: i32,
    pub slider_min: f64,
    pub slider_max: f64,
    pub is_discrete: bool,
    pub x: f64,
    pub y: f64,
    pub visible: bool,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Variable {
    pub name: String,
    pub value: serde_json::Value,
    pub is_persistent: bool,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct List {
    #[serde(rename="listName")]
    pub name: String,
    pub contents: Vec<serde_json::Value>,
    pub is_persistent: bool,
}

#[derive(Debug, Default)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub enum RotationStyle {
    #[default]
    Normal,
    LeftRight,
    None,
}
