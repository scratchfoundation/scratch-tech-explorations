use bevy::asset::Asset;
use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::sb2;

use crate::virtual_machine as VM;

use super::TopLevelItem;
use super::load::{VMLoadError, VMLoadResult};

impl VM::VirtualMachine {
    pub fn from_sb2_json(
        sb2_json: &[u8],
        sb2_path: &Option<String>,
        asset_server: Res<AssetServer>,
    ) -> Result<(Self, HashSet<HandleUntyped>), VMLoadError> {
        let project_description: sb2::Project = serde_json::from_slice(sb2_json)?;

        let mut asset_helper = AssetHelper {
            asset_server,
            sb2_path,
            all_assets: HashSet::new()
        };

        let stage = VM::Target::from_sb2_stage(project_description.stage, &mut asset_helper)?;

        let sprites_iter = project_description.children.into_iter()
            .filter_map(|child| match child {
                sb2::StageChild::Sprite(sprite) => Some(sprite),
                _ => None
            })
            .map(|sprite| VM::Target::from_sb2_sprite(sprite, &mut asset_helper));

        let targets = Some(Ok(stage)).into_iter().chain(sprites_iter).collect::<Result<_,_>>()?;

        Ok((VM::VirtualMachine {
            targets
        }, asset_helper.all_assets))
    }
}

struct AssetHelper<'a> {
    asset_server: Res<'a, AssetServer>,
    sb2_path: &'a Option<String>,
    all_assets: HashSet<HandleUntyped>,
}

impl AssetHelper<'_> {
    fn get_asset<T>(&mut self, md5: &str, ext: &str, id: i32) -> Handle<T>
    where
        T: Asset
    {
        let handle = match self.sb2_path {
            Some(sb2_path) => {
                self.asset_server.load(sb2_path.to_string() + "#" + &id.to_string() + "." + ext)
            }
            None => {
                todo!("add support for downloading assets")
            }
        };
        self.all_assets.insert(handle.clone_weak_untyped());
        handle
    }
}

fn load_costumes(costumes: Vec<sb2::Costume>, asset_helper: &mut AssetHelper) -> Vec<VM::Costume> {
    costumes
        .into_iter()
        .map(|costume| {
            let image = match costume.base_layer_md5.split_once(".") {
                Some((md5, ext)) => asset_helper.get_asset(md5, ext, costume.base_layer_id),
                None => todo!("couldn't understand costume designation: {}", costume.base_layer_md5),
            };
            VM::Costume {
                name: costume.costume_name,
                image,
                bitmap_resolution: costume.bitmap_resolution,
                rotation_center_x: costume.rotation_center_x,
                rotation_center_y: costume.rotation_center_y,
                layer_index: costume.base_layer_id,
            }
        })
        .collect()
}

fn load_sounds(sounds: Vec<sb2::Sound>, asset_helper: &mut AssetHelper) -> Vec<VM::Sound> {
    sounds
        .into_iter()
        .map(|sound| {
            let audio_source = match sound.md5.split_once(".") {
                Some((md5, ext)) => asset_helper.get_asset(md5, ext, sound.sound_id),
                None => todo!("couldn't understand sound designation: {}", sound.md5),
            };
            VM::Sound {
                name: sound.sound_name,
                audio_source,
                format: sound.format,
                sample_rate: sound.rate,
                sample_count: sound.sample_count,
                sound_index: sound.sound_id,
            }
        })
        .collect()
}

impl VM::Target {
    fn from_sb2_stage(stage: sb2::Stage, asset_helper: &mut AssetHelper) -> Result<VM::Target, VMLoadError> {
        let costumes = load_costumes(stage.target.costumes, asset_helper);
        let sounds = load_sounds(stage.target.sounds, asset_helper);

        Ok(VM::Target {
            name: stage.target.name,
            x: 0.0,
            y: 0.0,
            scale: 100.0,
            direction: 90.0,
            rotation_style: VM::RotationStyle::None,
            is_draggable: false,
            is_visible: true,
            scripts: stage.target.scripts.into_iter().map(|script| script.into()).collect(),
            variables: stage.target.variables.into_iter().map(|script| script.into()).collect(),
            lists: stage.target.lists.into_iter().map(|script| script.into()).collect(),
            sounds,
            costumes,
            current_costume: stage.target.current_costume_index,
        })
    }

    fn from_sb2_sprite(sprite: sb2::Sprite, asset_helper: &mut AssetHelper) -> Result<VM::Target, VMLoadError> {
        let costumes = load_costumes(sprite.target.costumes, asset_helper);
        let sounds = load_sounds(sprite.target.sounds, asset_helper);

        Ok(VM::Target {
            name: sprite.target.name,
            x: sprite.x,
            y: sprite.y,
            scale: sprite.scale,
            direction: sprite.direction,
            rotation_style: sprite.rotation_style.into(),
            is_draggable: sprite.is_draggable,
            is_visible: sprite.is_visible,
            scripts: sprite.target.scripts.into_iter().map(|script| script.into()).collect(),
            variables: sprite.target.variables.into_iter().map(|script| script.into()).collect(),
            lists: sprite.target.lists.into_iter().map(|script| script.into()).collect(),
            sounds,
            costumes,
            current_costume: sprite.target.current_costume_index,
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
