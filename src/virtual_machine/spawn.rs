use bevy::{
    prelude::*,
};

use crate::sb4;

use crate::virtual_machine as VM;

pub fn quat_from_direction(direction: f64) -> Quat {
    Quat::from_axis_angle(Vec3::Z, (90.0 - direction).to_radians() as f32)
}

impl VM::VirtualMachine {
    /// Install this VM state, in bulk, into the Bevy world.
    /// This probably only makes sense at the end of project load.
    pub fn spawn_from(
        sb4_project: sb4::Project,
        commands: &mut Commands,
        mut sprite_server: ResMut<Assets<VM::Sprite>>,
    ) -> VM::VirtualMachine {
        let mut sprites = Vec::new();

        info!("spawning {} sprites", sb4_project.sprites.len());

        for archetype in sb4_project.sprites {
            let sprite = VM::Sprite {
                name: archetype.name,
                scripts: archetype.scripts,
                sounds: archetype.sounds,
                costumes: archetype.costumes,
            };
            let sprite_handle = sprite_server.add(sprite);
            sprites.push(sprite_handle.clone());
            let sprite = sprite_server.get_mut(&sprite_handle).unwrap();

            let costume = &sprite.costumes[archetype.current_costume];
            let scale = (archetype.scale / (costume.bitmap_resolution as f64) / 100.0) as f32;

            let _target = commands
                .spawn((
                    VM::Target {
                        x: archetype.x,
                        y: archetype.y,
                        scale: archetype.scale,
                        direction: archetype.direction,
                        rotation_style: archetype.rotation_style,
                        is_draggable: archetype.is_draggable,
                        is_visible: archetype.is_visible,
                        variables: archetype.variables,
                        lists: archetype.lists,
                        current_costume: archetype.current_costume,
                        sprite: sprite_handle,
                    },
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(
                                archetype.x as f32,
                                archetype.y as f32,
                                sprites.len() as f32, // TODO: z-order
                            ),
                            rotation: quat_from_direction(archetype.direction),
                            scale: Vec3::new(
                                scale,
                                scale,
                                1.0
                            )
                        },
                        texture: (sprite.costumes[archetype.current_costume].image).clone(),
                        visibility: if archetype.is_visible { Visibility::Visible } else { Visibility::Hidden },
                        ..Default::default()
                    }
                ))
                .id();

            // TODO: track target entity?

            info!(
                concat!(
                    "\nTarget {} is at ({},{}) with scale {}\n",
                ),
                sprites.len() - 1,
                archetype.x,
                archetype.y,
                archetype.scale,
            );
        }

        VM::VirtualMachine { sprites }
    }
}
