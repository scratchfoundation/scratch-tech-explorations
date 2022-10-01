use bevy::prelude::*;
use bevy::window::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "scratch-bevy".to_string(),
            width: 960.,
            height: 720.,
            position: WindowPosition::Automatic,
            resize_constraints: default(),
            scale_factor_override: None,
            present_mode: PresentMode::AutoVsync,
            resizable: false,
            decorations: true,
            cursor_visible: true,
            cursor_locked: false,
            mode: WindowMode::Windowed,
            transparent: false,
            canvas: None,
            fit_canvas_to_parent: false,
        })
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

fn add_spawn_sprite_command<'a, P>(commands: &mut Commands, asset_server: &AssetServer, name: String, costume: P)
where P: Into<bevy::asset::AssetPath<'a>>
{
    let mut entity = commands.spawn();
    let sprite = entity.insert(ScratchSprite);
    sprite.insert(Name(name));
    sprite.insert_bundle(SpriteBundle {
        texture: asset_server.load(costume),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
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

fn add_demo_project_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    add_spawn_sprite_command(&mut commands, &asset_server, "Sprite 1".to_string(), "scratch-cat.svg");
    add_spawn_sprite_command(&mut commands, &asset_server, "Sprite 2".to_string(), "squirrel.png");
}
