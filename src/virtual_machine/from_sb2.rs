use std::io::Cursor;
use std::io::Read;
use std::io::Seek;

use bevy::asset::LoadedAsset;
use bevy::prelude::*;
use bevy::render::texture::CompressedImageFormats;
use bevy::render::texture::ImageType;
use bevy::utils::Entry;
use bevy::utils::HashMap;
use bevy::utils::HashSet;
use zip::ZipArchive;

use crate::assets::scratch_project_plugin::ScratchProject;
use crate::sb2;

use crate::virtual_machine as VM;

use super::TopLevelItem;
use super::load::VMLoadError;

pub async fn load_sb2_zip_project<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut bevy::asset::LoadContext<'b>,
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
    let project: sb2::Project = serde_json::from_reader(sb2_json)?;

    let stage = VM::Sprite::from_sb2_stage(project.stage, &mut asset_helper)?;

    let sprites_iter = project.children.into_iter()
        .filter_map(|child| match child {
            sb2::StageChild::Sprite(sprite) => Some(sprite),
            _ => None
        })
        .map(|sprite| VM::Sprite::from_sb2_sprite(sprite, &mut asset_helper));

    let sprites = Some(Ok(stage)).into_iter().chain(sprites_iter).collect::<Result<_,_>>()?;

    load_context.set_default_asset(LoadedAsset::new(ScratchProject {
        vm: VM::VirtualMachine {
            sprites
        },
        loading_assets
    }));

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
    fn get_image(&mut self, md5: &str, ext: &str, id: i32) -> Result<Handle<Image>, VMLoadError>
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

    fn get_sound(&mut self, md5: &str, ext: &str, id: i32) -> Result<Handle<AudioSource>, VMLoadError>
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
    costumes: Vec<sb2::Costume>,
    asset_helper: &mut AssetHelper<'_, '_, R>,
) -> Result<Vec<VM::Costume>, VMLoadError>
where
    R: Read + Seek
{
    let mut loaded_costumes = vec![];
    for costume in costumes {
        let image = match costume.base_layer_md5.split_once('.') {
            Some((md5, ext)) => {
                asset_helper.get_image(md5, ext, costume.base_layer_id)?
            },
            None => todo!("couldn't understand costume designation: {}", costume.base_layer_md5),
        };
        loaded_costumes.push(VM::Costume {
            name: costume.costume_name,
            image,
            bitmap_resolution: costume.bitmap_resolution,
            rotation_center_x: costume.rotation_center_x,
            rotation_center_y: costume.rotation_center_y,
            layer_index: costume.base_layer_id,
        });
    }
    Ok(loaded_costumes)
}

fn load_sounds<R>(
    sounds: Vec<sb2::Sound>,
    asset_helper: &mut AssetHelper<'_, '_, R>,
) -> Result<Vec<VM::Sound>, VMLoadError>
where
    R: Read + Seek
{
    let mut loaded_sounds = vec![];
    for sound in sounds {
        let audio_source = match sound.md5.split_once('.') {
            Some((md5, ext)) => asset_helper.get_sound(md5, ext, sound.sound_id)?,
            None => todo!("couldn't understand sound designation: {}", sound.md5),
        };
        loaded_sounds.push(VM::Sound {
            name: sound.sound_name,
            audio_source,
            format: sound.format,
            sample_rate: sound.rate,
            sample_count: sound.sample_count,
            sound_index: sound.sound_id,
        })
    }
    Ok(loaded_sounds)
}

impl VM::Sprite {
    fn from_sb2_stage<R>(
        stage: sb2::Stage,
        asset_helper: &mut AssetHelper<'_, '_, R>
    ) -> Result<VM::Sprite, VMLoadError>
    where
        R: Read + Seek
    {
        let costumes = load_costumes(stage.target.costumes, asset_helper)?;
        let sounds = load_sounds(stage.target.sounds, asset_helper)?;

        Ok(VM::Sprite {
            name: stage.target.name,
            scripts: stage.target.scripts.into_iter().map(|script| script.into()).collect(),
            sounds,
            costumes,
            targets: vec![
                VM::Target {
                    x: 0.0,
                    y: 0.0,
                    scale: 100.0,
                    direction: 90.0,
                    rotation_style: VM::RotationStyle::None,
                    is_draggable: false,
                    is_visible: true,
                    variables: stage.target.variables.into_iter().map(|script| script.into()).collect(),
                    lists: stage.target.lists.into_iter().map(|script| script.into()).collect(),
                    current_costume: stage.target.current_costume_index,
                }
            ],
        })
    }

    fn from_sb2_sprite<R>(
        sprite: sb2::Sprite,
        asset_helper: &mut AssetHelper<'_, '_, R>
    ) -> Result<VM::Sprite, VMLoadError>
    where
        R: Read + Seek
    {
        let costumes = load_costumes(sprite.target.costumes, asset_helper)?;
        let sounds = load_sounds(sprite.target.sounds, asset_helper)?;

        Ok(VM::Sprite {
            name: sprite.target.name,
            scripts: sprite.target.scripts.into_iter().map(|script| script.into()).collect(),
            sounds,
            costumes,
            targets: vec![
                VM::Target {
                    x: sprite.x,
                    y: sprite.y,
                    scale: sprite.scale * 100.0,
                    direction: sprite.direction,
                    rotation_style: sprite.rotation_style.into(),
                    is_draggable: sprite.is_draggable,
                    is_visible: sprite.is_visible,
                    variables: sprite.target.variables.into_iter().map(|script| script.into()).collect(),
                    lists: sprite.target.lists.into_iter().map(|script| script.into()).collect(),
                    current_costume: sprite.target.current_costume_index,
                }
            ]
        })
    }
}

impl From<sb2::RotationStyle> for VM::RotationStyle {
    fn from(value: sb2::RotationStyle) -> Self {
        match value {
            sb2::RotationStyle::Normal => Self::Normal,
            sb2::RotationStyle::LeftRight => Self::LeftRight,
            sb2::RotationStyle::None => Self::None,
        }
    }
}

impl From<sb2::TopLevelScript> for VM::TopLevelItem {
    fn from(mut value: sb2::TopLevelScript) -> Self {
        if value.script.is_empty() {
            // this isn't really valid...
            return TopLevelItem {
                x: value.x,
                y: value.y,
                stack: VM::BlockStack::Script(vec![]),
            };
        }
        let first_block = value.script.remove(0);
        match first_block {
            sb2::Block::DefineProcedure(definition) =>
                TopLevelItem {
                    x: value.x,
                    y: value.y,
                    stack: VM::BlockStack::Definition(VM::ProcedureDefinition {
                        spec: definition.spec.clone(),
                        body: value.script
                            .into_iter()
                            .map(|block| block.into())
                            .collect(),
                        parameter_names: definition.parameter_names,
                        default_arguments: definition.default_arg_values
                            .into_iter()
                            .map(|value| value.into())
                            .collect(),
                        run_without_screen_refresh: definition.run_without_screen_refresh
                    }),
                },
            _ => TopLevelItem {
                x: value.x,
                y: value.y,
                stack: VM::BlockStack::Script(
                    Some(first_block).into_iter()
                    .chain(
                        value.script.into_iter()
                    )
                    .map(|block| block.into())
                    .collect()
                ),
            }
        }
    }
}

impl From<sb2::LiteralValue> for VM::Value {
    fn from(value: sb2::LiteralValue) -> Self {
        match value {
            sb2::LiteralValue::Boolean(b) => Self::Boolean(b),
            sb2::LiteralValue::Number(n) => Self::Number(n),
            sb2::LiteralValue::String(s) => Self::String(s),
        }
    }
}

impl From<sb2::Block> for VM::Block {
    fn from(value: sb2::Block) -> Self {
        match value {
            sb2::Block::DefineProcedure(_) => panic!("unexpected procedure definition"),
            sb2::Block::Basic(b) => b.into(),
            sb2::Block::C(b) => b.into(),
            sb2::Block::E(b) => b.into(),
        }
    }
}

impl From<sb2::BasicBlock> for VM::Block {
    fn from(value: sb2::BasicBlock) -> Self {
        VM::Block {
            opcode: value.opcode,
            arguments: value.args.into_iter().map(|arg| arg.into()).collect(),
            branches: vec![],
        }
    }
}

impl From<sb2::CBlock> for VM::Block {
    fn from(value: sb2::CBlock) -> Self {
        VM::Block {
            opcode: value.opcode,
            arguments: value.args.into_iter().map(|arg| arg.into()).collect(),
            branches: vec![
                value.branch.into_iter().map(|block| block.into()).collect(),
            ],
        }
    }
}

impl From<sb2::EBlock> for VM::Block {
    fn from(value: sb2::EBlock) -> Self {
        VM::Block {
            opcode: value.opcode,
            arguments: value.args.into_iter().map(|arg| arg.into()).collect(),
            branches: vec![
                value.branch0.into_iter().map(|block| block.into()).collect(),
                value.branch1.into_iter().map(|block| block.into()).collect(),
            ],
        }
    }
}
impl From<sb2::BlockArgument> for VM::Argument {
    fn from(value: sb2::BlockArgument) -> Self {
        match value {
            sb2::BlockArgument::Boolean(b) => Self::Literal(VM::Value::Boolean(b)),
            sb2::BlockArgument::Number(n) => Self::Literal(VM::Value::Number(n)),
            sb2::BlockArgument::String(s) => Self::Literal(VM::Value::String(s)),
            sb2::BlockArgument::Reporter(r) => Self::Expression(r.into()),
        }
    }
}

impl From<sb2::Variable> for (String, VM::Variable) {
    fn from(value: sb2::Variable) -> Self {
        (
            value.name,
            VM::Variable {
                value: value.value.into(),
                is_cloud: value.is_persistent,
            }
        )
    }
}

impl From<sb2::List> for (String, VM::List) {
    fn from(value: sb2::List) -> Self {
        (
            value.name,
            VM::List {
                values: value.contents.into_iter().map(|x| x.into()).collect(),
                is_cloud: value.is_persistent,
            }
        )
    }
}
