pub mod load;

use std::fmt::Debug;

use serde::{Deserialize, Serialize, de::{Visitor, self}, ser::SerializeSeq};

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

    pub info: Info,
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
    pub variables: Vec<Variable>,

    #[serde(default)]
    pub lists: Vec<List>,

    #[serde(default)]
    pub sounds: Vec<Sound>,

    #[serde(default)]
    pub costumes: Vec<Costume>,

    pub current_costume_index: i32,

    #[serde(default)]
    pub scripts: Vec<TopLevelScript>,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Monitor {
    pub target: String,
    pub cmd: String,
    pub param: String,
    pub color: u32,
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

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Costume {
    pub costume_name: String,

    #[serde(rename = "baseLayerID")]
    pub base_layer_id: i32,

    #[serde(rename = "baseLayerMD5")]
    pub base_layer_md5: String,

    pub bitmap_resolution: i32,

    pub rotation_center_x: f64,
    pub rotation_center_y: f64,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Sound {
    pub sound_name: String,

    #[serde(rename = "soundID")]
    pub sound_id: i32,

    pub md5: String,
    pub sample_count: i32,
    pub rate: i32,
    pub format: String,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Info {
    pub user_agent: String,
    pub flash_version: String,
    pub sprite_count: i32,
    pub video_on: bool,
    pub script_count: i32,
    pub swf_version: String,

    // unknown fields will be preserved here
    #[serde(flatten)]
    pub other_info: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct TopLevelScript {
    pub x: f64,
    pub y: f64,
    pub script: Script,
}

type Script = Vec<Block>;

#[derive(Debug)]
pub struct Block {
    pub opcode: String,
    pub args: Vec<BlockArgument>,
}
type BlockArgument = serde_json::Value;

/*
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum BlockArgument {
    Literal(serde_json::Value),
}
*/

impl<'de> Deserialize<'de> for Block {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>,
    {
        struct BlockVisitor;

        impl<'de> Visitor<'de> for BlockVisitor {
            type Value = Block;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an array containing an opcode optionally followed by inputs")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where A: serde::de::SeqAccess<'de>,
            {
                let opcode = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom("could not read opcode for script block")
            })?;

                let mut args = Vec::<BlockArgument>::with_capacity(seq.size_hint().unwrap_or(0));
                while let Some(arg) = seq.next_element::<serde_json::Value>()? {
                    args.push(arg);
                }

                Ok(Block { opcode, args })
            }
        }

        deserializer.deserialize_seq(BlockVisitor)
    }
}

impl Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        let mut state = serializer.serialize_seq(Some(1 + self.args.len()))?;
        state.serialize_element(&self.opcode)?;
        for arg in &self.args {
            state.serialize_element(arg)?;
        }
        state.end()
    }
}
