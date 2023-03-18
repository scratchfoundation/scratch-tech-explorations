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
    pub script: Vec<Block>,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Block {
    DefineProcedure(DefineProcedure),
    EBlock(EBlock),
    CBlock(CBlock),
    BasicBlock(BasicBlock),
}

#[derive(Debug)]
pub struct BasicBlock {
    pub opcode: String,
    pub args: Vec<BlockArgument>,
}

#[derive(Debug)]
pub struct CBlock {
    pub opcode: String,
    pub args: Vec<BlockArgument>,
    pub branch: Vec<Block>,
}

#[derive(Debug)]
pub struct EBlock {
    pub opcode: String,
    pub args: Vec<BlockArgument>,
    pub branch0: Vec<Block>,
    pub branch1: Vec<Block>,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct DefineProcedure {
    pub opcode: String, // always "procDef"
    pub spec: String,
    pub parameter_names: Vec<String>,
    pub default_arg_values: Vec<LiteralValue>,
    pub run_without_screen_refresh: bool,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum BlockArgument {
    Boolean(bool),
    Number(f64),
    String(String),
    Reporter(BasicBlock),
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum LiteralValue {
    Boolean(bool),
    Number(f64),
    String(String),
}

impl From<&str> for BlockArgument {
    fn from(value: &str) -> Self {
        BlockArgument::String(value.to_string())
    }
}

impl<'de> Deserialize<'de> for EBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>,
    {
        struct EBlockVisitor;

        impl<'de> Visitor<'de> for EBlockVisitor {
            type Value = EBlock;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a script block with two branches (an 'E block')")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where A: serde::de::SeqAccess<'de>,
            {
                let opcode = seq.next_element()?.ok_or_else(||
                    de::Error::custom("could not read opcode for E block")
                )?;

                let mut args = vec![];
                let mut branches: [serde_json::Value; 2] = [
                    seq.next_element()?.ok_or_else(||
                        de::Error::custom("could not find first branch for E block")
                    )?,
                    seq.next_element()?.ok_or_else(||
                        de::Error::custom("could not find second branch for E block")
                    )?,
                ];
                while let Some(next_element) = seq.next_element::<serde_json::Value>()? {
                    let arg = serde_json::from_value::<BlockArgument>(branches[0].take()).map_err(|_|
                        de::Error::custom("could not interpret argument for E block")
                    )?;
                    args.push(arg);
                    branches[0] = branches[1].take();
                    branches[1] = next_element;
                }
                Ok(EBlock {
                    opcode,
                    args,
                    branch0: serde_json::from_value(branches[0].take()).map_err(|_|
                        de::Error::custom("could not interpret first branch for E block")
                    )?,
                    branch1: serde_json::from_value(branches[1].take()).map_err(|_|
                        de::Error::custom("could not interpret second branch for E block")
                    )?,
                })
            }
        }

        deserializer.deserialize_seq(EBlockVisitor)
    }
}

impl Serialize for EBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        let mut state = serializer.serialize_seq(Some(3 + self.args.len()))?;
        state.serialize_element(&self.opcode)?;
        for arg in &self.args {
            state.serialize_element(arg)?;
        }
        state.serialize_element(&self.branch0)?;
        state.serialize_element(&self.branch1)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for CBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>,
    {
        struct CBlockVisitor;

        impl<'de> Visitor<'de> for CBlockVisitor {
            type Value = CBlock;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a script block with one branch (a 'C block')")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where A: serde::de::SeqAccess<'de>,
            {
                let opcode = seq.next_element()?.ok_or_else(||
                    de::Error::custom("could not read opcode for C block")
                )?;

                let mut args = vec![];
                let mut branch = seq.next_element()?.ok_or_else(||
                        de::Error::custom("could not find branch for C block")
                    )?;
                while let Some(next_element) = seq.next_element::<serde_json::Value>()? {
                    let arg = serde_json::from_value::<BlockArgument>(branch).map_err(|_|
                        de::Error::custom("could not interpret argument for C block")
                    )?;
                    args.push(arg);
                    branch = next_element;
                }
                Ok(CBlock {
                    opcode,
                    args,
                    branch: serde_json::from_value(branch).map_err(|_|
                        de::Error::custom("could not interpret first branch for C block")
                    )?,
                })
            }
        }

        deserializer.deserialize_seq(CBlockVisitor)
    }
}

impl Serialize for CBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        let mut state = serializer.serialize_seq(Some(2 + self.args.len()))?;
        state.serialize_element(&self.opcode)?;
        for arg in &self.args {
            state.serialize_element(arg)?;
        }
        state.serialize_element(&self.branch)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for BasicBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>,
    {
        struct BasicBlockVisitor;

        impl<'de> Visitor<'de> for BasicBlockVisitor {
            type Value = BasicBlock;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a script block with no branches")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where A: serde::de::SeqAccess<'de>,
            {
                let opcode = seq.next_element()?.ok_or_else(||
                    de::Error::custom("could not read opcode for block")
                )?;

                let mut args = vec![];
                while let Some(next_argument) = seq.next_element()? {
                    args.push(next_argument);
                }
                Ok(BasicBlock {
                    opcode,
                    args,
                })
            }
        }

        deserializer.deserialize_seq(BasicBlockVisitor)
    }
}

impl Serialize for BasicBlock {
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
