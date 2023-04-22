use std::io;

use zip::ZipArchive;

use crate::sb2;

use crate::virtual_machine as VM;

use super::TopLevelItem;
use super::load::{VMLoadError, VMLoadResult};

impl VM::VirtualMachine {
    pub fn from_sb2_reader<R>(sb2_reader: R) -> VMLoadResult
    where
        R: io::Read + std::io::Seek,
    {
        // this will open the ZIP and read the central directory
        let mut sb2_zip = ZipArchive::new(sb2_reader)?;

        let project_json_reader = sb2_zip.by_name("project.json")?;
        let project_description: sb2::Project = serde_json::from_reader(project_json_reader)?;

        let stage = VM::Target::from_sb2_stage(project_description.stage, &mut sb2_zip)?;

        let sprites_iter = project_description.children.into_iter()
            .filter_map(|child| match child {
                sb2::StageChild::Sprite(sprite) => Some(sprite),
                _ => None
            })
            .map(|sprite| VM::Target::from_sb2_sprite(sprite, &mut sb2_zip));

        let targets = Some(Ok(stage)).into_iter().chain(sprites_iter).collect::<Result<_,_>>()?;

        Ok(VM::VirtualMachine {
            targets
        })
    }
}

fn load_costumes<R>(costumes: Vec<sb2::Costume>, sb2_zip: &mut ZipArchive<R>) -> Result<Vec<VM::Costume>, VMLoadError> {
    Ok(vec![])
}

fn load_sounds<R>(sounds: Vec<sb2::Sound>, sb2_zip: &mut ZipArchive<R>) -> Result<Vec<VM::Sound>, VMLoadError> {
    Ok(vec![])
}

impl VM::Target {
    pub fn from_sb2_stage<R>(stage: sb2::Stage, sb2_zip: &mut ZipArchive<R>) -> Result<VM::Target, VMLoadError> {
        let costumes = load_costumes(stage.target.costumes, sb2_zip)?;
        let sounds = load_sounds(stage.target.sounds, sb2_zip)?;

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

    pub fn from_sb2_sprite<R>(sprite: sb2::Sprite, sb2_zip: &mut ZipArchive<R>) -> Result<VM::Target, VMLoadError> {
        let costumes = load_costumes(sprite.target.costumes, sb2_zip)?;
        let sounds = load_sounds(sprite.target.sounds, sb2_zip)?;

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

impl From<sb2::Sound> for VM::Sound {
    fn from(value: sb2::Sound) -> Self {
        VM::Sound {
            name: value.sound_name,
            md5: value.md5,
            format: value.format,
            sample_rate: value.rate,
            sample_count: value.sample_count,
            sound_index: value.sound_id, // TODO: connect references
        }
    }
}

impl From<sb2::Costume> for VM::Costume {
    fn from(value: sb2::Costume) -> Self {
        VM::Costume {
            name: value.costume_name,
            md5: value.base_layer_md5,
            bitmap_resolution: value.bitmap_resolution,
            rotation_center_x: value.rotation_center_x,
            rotation_center_y: value.rotation_center_y,
            layer_index: value.base_layer_id, // TODO: connect references
        }
    }
}
