//! This module contains code for loading a Scratch 2.0 project, either from an SB2 file or from JSON.
//! The result is a `Project` struct containing the project structure and all related assets.
//! This is "frozen" data and must be loaded into a VM before it can run.

use std::fmt::Debug;
use std::io::{Cursor, Read, Seek};

use bevy::asset::Error as AnyError; // anyhow::Error
use bevy::asset::LoadedAsset;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::texture::{ImageType, CompressedImageFormats};
use bevy::utils::{Entry, HashMap, HashSet};
use serde::{Deserialize, Serialize, de::{Visitor, self}, ser::SerializeSeq};
use zip::ZipArchive;

use crate::assets::scratch_project_plugin::ScratchProject;

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[derive(TypeUuid)]
#[uuid = "7e6fc139-66f6-4916-a118-5ae4b90e7bae"]
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

    pub scale: f64, // scaling factor: 1.0=100%
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

    pub current_costume_index: usize,

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
    pub value: LiteralValue,
    pub is_persistent: bool,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct List {
    #[serde(rename="listName")]
    pub name: String,
    pub contents: Vec<LiteralValue>,
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

    #[serde(skip)]
    pub image: Handle<Image>,
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

    #[serde(skip)]
    pub audio_source: Handle<AudioSource>,
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
    E(EBlock),
    C(CBlock),
    Basic(BasicBlock),
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

pub fn load_project_from_zip<'a>(
    bytes: &'a [u8],
    load_context: &'a mut bevy::asset::LoadContext<'_>,
) -> Result<(), bevy::asset::Error> {
    info!("loading SB2 ZIP ({} bytes)", bytes.len());
    let reader = Cursor::new(bytes.to_vec());

    let mut loading_assets = HashSet::new();
    let mut loading_images = HashMap::new();
    let mut loading_sounds = HashMap::new();

    let mut asset_helper = AssetHelper {
        load_context,
        loading_assets: &mut loading_assets,
        loading_images: &mut loading_images,
        loading_sounds: &mut loading_sounds,
        sb2_zip: ZipArchive::new(reader)?
    };

    let sb2_json = asset_helper.sb2_zip.by_name("project.json")?;
    let mut project: Project = serde_json::from_reader(sb2_json)?;

    load_costumes(&mut project.stage.target.costumes, &mut asset_helper)?;
    load_sounds(&mut project.stage.target.sounds, &mut asset_helper)?;

    for stage_child in &mut project.children {
        if let StageChild::Sprite(sprite) = stage_child {
            load_costumes(&mut sprite.target.costumes, &mut asset_helper)?;
            load_sounds(&mut sprite.target.sounds, &mut asset_helper)?;
        }
    }

    load_context.set_default_asset(LoadedAsset::new(
        ScratchProject::SB2(project)
    ));

    Ok(())
}

struct AssetHelper<'a, 'b, R> {
    load_context: &'a mut bevy::asset::LoadContext<'b>,
    loading_assets: &'a mut HashSet<HandleUntyped>,
    loading_images: &'a mut HashMap<String, Handle<Image>>,
    loading_sounds: &'a mut HashMap<String, Handle<AudioSource>>,
    sb2_zip: ZipArchive<R>,
}

fn pretend_loading_is_slow() {
    std::thread::sleep(std::time::Duration::from_millis(250));
}

impl<R> AssetHelper<'_, '_, R> {
    fn get_image(&mut self, _md5: &str, ext: &str, id: i32) -> Result<Handle<Image>, AnyError>
    where
        R: Read + Seek
    {
        pretend_loading_is_slow();

        let costume_file_name = id.to_string() + "." + ext;

        let handle = match self.loading_images.entry(costume_file_name) {
            Entry::Occupied(o) => o.get().clone(),
            Entry::Vacant(v) => {
                let mut is_fake = false;
                let image = match ext.to_lowercase().as_str() {
                    "svg" => {
                        is_fake = true;
                        Image::default() // TODO: SVG support
                    }
                    _ => {
                        let mut costume_file = self.sb2_zip.by_name(v.key())?;
                        let mut costume_bytes = vec![];
                        costume_file.read_to_end(&mut costume_bytes)?;
                        Image::from_buffer(
                            &costume_bytes,
                            ImageType::Extension(ext),
                            CompressedImageFormats::NONE,
                            true
                        )?
                    }
                };
                let image_handle = self.load_context.set_labeled_asset(
                    v.key(),
                    LoadedAsset::new(image)
                );
                if is_fake { warn!("used fake image for {:?}", image_handle); }
                self.loading_assets.insert(image_handle.clone_untyped());
                image_handle
            },
        };

        Ok(handle)
    }

    fn get_sound(&mut self, _md5: &str, ext: &str, id: i32) -> std::io::Result<Handle<AudioSource>>
    where
        R: Read + Seek
    {
        pretend_loading_is_slow();

        let sound_file_name = id.to_string() + "." + ext;

        let handle = match self.loading_sounds.entry(sound_file_name) {
            Entry::Occupied(o) => o.get().clone(),
            Entry::Vacant(v) => {
                let mut sound_file = self.sb2_zip.by_name(v.key())?;
                let mut sound_bytes = vec![];
                sound_file.read_to_end(&mut sound_bytes)?;
                let sound = AudioSource {
                    bytes: sound_bytes.into(),
                };
                let sound_handle = self.load_context.set_labeled_asset(
                    v.key(),
                    LoadedAsset::new(sound)
                );
                self.loading_assets.insert(sound_handle.clone_untyped());
                sound_handle
            },
        };

        Ok(handle)
    }
}

fn load_costumes<R>(
    costumes: &mut Vec<Costume>,
    asset_helper: &mut AssetHelper<'_, '_, R>,
) -> Result<Vec<Handle<Image>>, AnyError>
where
    R: Read + Seek
{
    let mut loaded_costumes = vec![];
    for costume in costumes {
        costume.image = match costume.base_layer_md5.split_once('.') {
            Some((md5, ext)) => {
                asset_helper.get_image(md5, ext, costume.base_layer_id)?
            },
            None => todo!("couldn't understand costume designation: {}", costume.base_layer_md5),
        };
        loaded_costumes.push(costume.image.clone());
    }
    Ok(loaded_costumes)
}

fn load_sounds<R>(
    sounds: &mut Vec<Sound>,
    asset_helper: &mut AssetHelper<'_, '_, R>,
) -> Result<Vec<Handle<AudioSource>>, AnyError>
where
    R: Read + Seek
{
    let mut loaded_sounds = vec![];
    for sound in sounds {
        sound.audio_source = match sound.md5.split_once('.') {
            Some((md5, ext)) => {
                asset_helper.get_sound(md5, ext, sound.sound_id)?
            },
            None => todo!("couldn't understand sound designation: {}", sound.md5),
        };
        loaded_sounds.push(sound.audio_source.clone());
    }
    Ok(loaded_sounds)
}
