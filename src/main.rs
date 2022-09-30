use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

#[derive(Component)]
struct Sprite;

#[derive(Component)]
struct Name(String);

fn add_default_sprites(mut commands: Commands) {
    commands.spawn().insert(Sprite).insert(Name("Sprite 1".to_string()));
    commands.spawn().insert(Sprite).insert(Name("Sprite 2".to_string()));
}

fn print_sprite_names(time: Res<Time>, mut timer: ResMut<SpriteNameTimer>, query: Query<&Name, With<Sprite>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("found a sprite named {}", name.0);
        }
    }
}

pub struct HelloPlugin;

struct SpriteNameTimer(Timer);

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpriteNameTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_default_sprites)
            .add_system(print_sprite_names);
    }
}
