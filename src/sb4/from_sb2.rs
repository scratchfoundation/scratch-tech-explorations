//! Convert SB2 data to ready-to-install project data.
//! This also involves loading any assets

use crate::sb2;
use crate::sb4;

impl From<sb2::Project> for sb4::Project {
    fn from(value: sb2::Project) -> Self {
        let stage = value.stage.into();
        let sprites = Some(stage).into_iter()
            .chain(
                value.children.into_iter()
                    .filter_map(|child| match child {
                        sb2::StageChild::Sprite(sprite) => Some(sprite.into()),
                        _ => None, // skip non-sprites
                    })
            )
            .collect();
        Self {
            sprites,
        }
    }
}

impl From<sb2::Stage> for sb4::Sprite {
    fn from(value: sb2::Stage) -> Self {
        Self {
            name: value.target.name,
            scripts: value.target.scripts.into_iter().map(|script| script.into()).collect(),
            sounds: value.target.sounds.into_iter().map(|sound| sound.into()).collect(),
            costumes: value.target.costumes.into_iter().map(|costume| costume.into()).collect(),
            x: 0.0,
            y: 0.0,
            scale: 100.0,
            direction: 90.0,
            rotation_style: sb4::RotationStyle::Normal,
            is_draggable: false,
            is_visible: true,
            variables: value.target.variables.into_iter().map(|var| var.into()).collect(),
            lists: value.target.lists.into_iter().map(|list| list.into()).collect(),
            current_costume: value.target.current_costume_index,
        }
    }
}

impl From<sb2::Sprite> for sb4::Sprite {
    fn from(value: sb2::Sprite) -> Self {
        Self {
            name: value.target.name,
            scripts: value.target.scripts.into_iter().map(|script| script.into()).collect(),
            sounds: value.target.sounds.into_iter().map(|sound| sound.into()).collect(),
            costumes: value.target.costumes.into_iter().map(|costume| costume.into()).collect(),
            x: value.x,
            y: value.y,
            scale: value.scale * 100.0,
            direction: value.direction,
            rotation_style: value.rotation_style.into(),
            is_draggable: value.is_draggable,
            is_visible: value.is_visible,
            variables: value.target.variables.into_iter().map(|var| var.into()).collect(),
            lists: value.target.lists.into_iter().map(|list| list.into()).collect(),
            current_costume: value.target.current_costume_index,
        }
    }
}

impl From<sb2::Costume> for sb4::Costume {
    fn from(value: sb2::Costume) -> Self {
        Self {
            name: value.costume_name,
            image: value.image,
            bitmap_resolution: value.bitmap_resolution,
            rotation_center_x: value.rotation_center_x,
            rotation_center_y: value.rotation_center_y,
            layer_index: value.base_layer_id,
        }
    }
}

impl From<sb2::Sound> for sb4::Sound {
    fn from(value: sb2::Sound) -> Self {
        Self {
            name: value.sound_name,
            audio_source: value.audio_source,
            format: value.format,
            sample_rate: value.rate,
            sample_count: value.sample_count,
            sound_index: value.sound_id,
        }
    }
}

impl From<sb2::RotationStyle> for sb4::RotationStyle {
    fn from(value: sb2::RotationStyle) -> Self {
        match value {
            sb2::RotationStyle::Normal => Self::Normal,
            sb2::RotationStyle::LeftRight => Self::LeftRight,
            sb2::RotationStyle::None => Self::None,
        }
    }
}

impl From<sb2::TopLevelScript> for sb4::TopLevelItem {
    fn from(mut value: sb2::TopLevelScript) -> Self {
        if value.script.is_empty() {
            // this isn't really valid...
            return sb4::TopLevelItem {
                x: value.x,
                y: value.y,
                stack: sb4::BlockStack::Script(vec![]),
            };
        }
        let first_block = value.script.remove(0);
        match first_block {
            sb2::Block::DefineProcedure(definition) =>
                sb4::TopLevelItem {
                    x: value.x,
                    y: value.y,
                    stack: sb4::BlockStack::Definition(sb4::ProcedureDefinition {
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
            _ => sb4::TopLevelItem {
                x: value.x,
                y: value.y,
                stack: sb4::BlockStack::Script(
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

impl From<sb2::LiteralValue> for sb4::Value {
    fn from(value: sb2::LiteralValue) -> Self {
        match value {
            sb2::LiteralValue::Boolean(b) => Self::Boolean(b),
            sb2::LiteralValue::Number(n) => Self::Number(n),
            sb2::LiteralValue::String(s) => Self::String(s),
        }
    }
}

impl From<sb2::Block> for sb4::Block {
    // TODO: translate SB2 opcodes as needed
    fn from(value: sb2::Block) -> Self {
        match value {
            sb2::Block::DefineProcedure(_) => panic!("unexpected procedure definition"),
            sb2::Block::Basic(b) => b.into(),
            sb2::Block::C(b) => b.into(),
            sb2::Block::E(b) => b.into(),
        }
    }
}

impl From<sb2::BasicBlock> for sb4::Block {
    fn from(value: sb2::BasicBlock) -> Self {
        sb4::Block {
            opcode: value.opcode,
            arguments: value.args.into_iter().map(|arg| arg.into()).collect(),
            branches: vec![],
        }
    }
}

impl From<sb2::CBlock> for sb4::Block {
    fn from(value: sb2::CBlock) -> Self {
        sb4::Block {
            opcode: value.opcode,
            arguments: value.args.into_iter().map(|arg| arg.into()).collect(),
            branches: vec![
                value.branch.into_iter().map(|block| block.into()).collect(),
            ],
        }
    }
}

impl From<sb2::EBlock> for sb4::Block {
    fn from(value: sb2::EBlock) -> Self {
        sb4::Block {
            opcode: value.opcode,
            arguments: value.args.into_iter().map(|arg| arg.into()).collect(),
            branches: vec![
                value.branch0.into_iter().map(|block| block.into()).collect(),
                value.branch1.into_iter().map(|block| block.into()).collect(),
            ],
        }
    }
}
impl From<sb2::BlockArgument> for sb4::Argument {
    fn from(value: sb2::BlockArgument) -> Self {
        match value {
            sb2::BlockArgument::Boolean(b) => Self::Literal(sb4::Value::Boolean(b)),
            sb2::BlockArgument::Number(n) => Self::Literal(sb4::Value::Number(n)),
            sb2::BlockArgument::String(s) => Self::Literal(sb4::Value::String(s)),
            sb2::BlockArgument::Reporter(r) => Self::Expression(r.into()),
        }
    }
}

impl From<sb2::Variable> for (String, sb4::Variable) {
    fn from(value: sb2::Variable) -> Self {
        (
            value.name,
            sb4::Variable {
                value: value.value.into(),
                is_cloud: value.is_persistent,
            }
        )
    }
}

impl From<sb2::List> for (String, sb4::List) {
    fn from(value: sb2::List) -> Self {
        (
            value.name,
            sb4::List {
                values: value.contents.into_iter().map(|x| x.into()).collect(),
                is_cloud: value.is_persistent,
            }
        )
    }
}
