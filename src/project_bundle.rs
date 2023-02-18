use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct BlockId(String);

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct ListId(String);

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct VariableId(String);

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectBundle {
    pub targets: Vec<Target>,
    pub monitors: Vec<Monitor>,
    pub extensions: Vec<String>,
    pub meta: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Target {
    pub is_stage: bool,
    pub name: String,
    pub variables: HashMap<VariableId, VariableNameAndValue>,
    pub lists: HashMap<ListId, ListNameAndValues>,
    pub broadcasts: serde_json::Value, // TODO
    pub blocks: HashMap<BlockId, Block>,
    pub comments: serde_json::Value, // TODO
    pub current_costume: i32,
    pub costumes: serde_json::Value, // TODO
    pub sounds: serde_json::Value, // TODO
    pub volume: f64,
    pub layer_order: i32,

    #[serde(default)]
    pub tempo: f64,

    #[serde(default)]
    pub video_transparency: f64,

    #[serde(default)]
    pub video_state: serde_json::Value, // TODO

    #[serde(default)]
    pub text_to_speech_language: serde_json::Value, // TODO

    #[serde(default)]
    pub visible: bool,

    #[serde(default)]
    pub x: f64,

    #[serde(default)]
    pub y: f64,

    #[serde(default)]
    pub size: f64,

    #[serde(default)]
    pub direction: f64,

    #[serde(default)]
    pub draggable: bool,

    #[serde(default)]
    pub rotation_style: RotationStyle,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VariableNameAndValue {
    pub name: String,
    pub value: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListNameAndValues {
    pub name: String,
    pub values: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub opcode: String,
    pub next: Option<BlockId>,
    pub parent: Option<BlockId>,
    pub inputs: HashMap<String, serde_json::Value>, // TODO
    pub fields: serde_json::Value, // TODO
    pub shadow: bool,
    #[serde(default)]
    pub top_level: bool,
}

/*
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub struct Input {
    // see deserializeInputs in sb3.js
}
*/

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum RotationStyle {
    #[default]
    #[serde(rename = "all around")]
    AllAround,

    #[serde(rename = "left-right")]
    LeftRight,

    #[serde(rename = "don't rotate")]
    DoNotRotate,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Monitor {
    pub id: String,
    pub mode: MonitorMode,
    pub opcode: String,
    pub params: serde_json::Value, // TODO
    pub sprite_name: Option<String>,
    pub value: serde_json::Value,
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
    pub visible: bool,

    #[serde(default)]
    pub slider_min: f64,

    #[serde(default)]
    pub slider_max: f64,

    #[serde(default)]
    pub is_discrete: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub enum MonitorMode {
    Default,
    List,
}

impl Default for Target {
    fn default() -> Self {
        Self {
            is_stage: false,
            name: Default::default(),
            variables: Default::default(),
            lists: Default::default(),
            broadcasts: serde_json::Value::Null,
            blocks: Default::default(),
            comments: serde_json::Value::Null,
            current_costume: 0,
            costumes: serde_json::Value::Null,
            sounds: serde_json::Value::Null,
            volume: 100.0,
            layer_order: 0,
            tempo: 60.0,
            video_transparency: 50.0,
            video_state: serde_json::Value::from("on"),
            text_to_speech_language: serde_json::Value::Null,
            visible: true,
            x: 0.0,
            y: 0.0,
            size: 100.0,
            direction: 0.0,
            draggable: true,
            rotation_style: RotationStyle::AllAround,
        }
    }
}

impl Default for Monitor {
    fn default() -> Self {
        Self {
            id: String::new(),
            mode: MonitorMode::Default,
            opcode: "data_variable".to_string(),
            params: serde_json::Value::Null,
            sprite_name: None,
            value: serde_json::Value::from(0),
            width: 0.0,
            height: 0.0,
            x: 0.0,
            y: 0.0,
            visible: true,
            slider_min: 0.0,
            slider_max: 100.0,
            is_discrete: true,
        }
    }
}
