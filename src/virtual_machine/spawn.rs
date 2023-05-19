use bevy::{
    prelude::*,
};

use super::VirtualMachine;

pub fn quat_from_direction(direction: f64) -> Quat {
    Quat::from_axis_angle(Vec3::Z, (90.0 - direction).to_radians() as f32)
}

impl VirtualMachine {
    /// Install this VM state, in bulk, into the Bevy world.
    /// This probably only makes sense at the end of project load.
    pub fn spawn(
        &self,
        mut commands: Commands,
    ) {
        for sprite_index in 0..self.sprites.len() {
            let sprite = &self.sprites[sprite_index];
            let target = &sprite.targets[0];
            let costume = sprite.costumes.get(target.current_costume).unwrap_or(sprite.costumes.first().expect("TODO: support sprites w/ zero costumes"));
            let scale = target.scale / (costume.bitmap_resolution as f64) / 100.0;
            let sprite_bundle = SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        target.x as f32,
                        target.y as f32,
                        sprite_index as f32 // TODO
                    ),
                    rotation: quat_from_direction(target.direction),
                    scale: Vec3::new(
                        scale as f32,
                        scale as f32,
                        1.0
                    ),
                },
                texture: costume.image.clone(),
                visibility: if target.is_visible { Visibility::Visible } else { Visibility::Hidden },
                ..Default::default()
            };
            info!(
                concat!(
                    "\nTarget {} is at ({},{}) wearing {:?}\n",
                    "  Bitmap resolution: {}\n",
                    "  User scale: {}\n",
                    "  Calculated scale: {}\n",
                ),
                sprite_index,
                sprite_bundle.transform.translation.x,
                sprite_bundle.transform.translation.y,
                sprite_bundle.texture,
                costume.bitmap_resolution,
                target.scale,
                scale
            );
            commands.spawn(sprite_bundle);
        }
    }
}
