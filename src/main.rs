use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ScratchStagePlugin)
        .add_plugin(ScratchDemoProjectPlugin)
        .run();
}

//
// Sprite
//

#[derive(Component)]
struct ScratchSprite;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Costume(String);

fn add_spawn_sprite_command(commands: &mut Commands, name: String, costume: String) {
    let mut entity = commands.spawn();
    let sprite = entity.insert(ScratchSprite);
    sprite.insert(Name(name));
    sprite.insert(Costume(costume));
}

//
// Stage
//

pub struct ScratchStagePlugin;

struct SpriteNameTimer(Timer);

impl Plugin for ScratchStagePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpriteNameTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_stage_startup)
            .add_system(step_vm);
    }
}

fn add_stage_startup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn step_vm(time: Res<Time>, mut timer: ResMut<SpriteNameTimer>, query: Query<&Name, With<ScratchSprite>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("found a sprite named {}", name.0);
        }
    }
}

//
// Project
//

pub struct ScratchDemoProjectPlugin;

impl Plugin for ScratchDemoProjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(add_demo_project_sprites);
    }
}

fn add_demo_project_sprites(mut commands: Commands) {
    add_spawn_sprite_command(&mut commands, "Sprite 1".to_string(), "Cat".to_string());
    add_spawn_sprite_command(&mut commands, "Sprite 2".to_string(), "Ball".to_string());
}
