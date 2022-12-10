use bevy::{
    prelude::*,
    time::FixedTimestep,
    window::*,
};

const TIME_STEP: f64 = 1. / 30.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "scratch-bevy".to_string(),
                width: 960.,
                height: 720.,
                resize_constraints: default(),
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::Windowed,
                ..default()
            },
            ..default()
        }))
        .add_plugin(ScratchStagePlugin)
        .add_plugin(ScratchDemoProjectPlugin)
        .add_system(close_on_esc)
        .run();
}

//
// Sprite
//

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Costume(String);

#[derive(Debug)]
enum ScratchCode {
    MoveOneStep,
    MoveTwoSteps,
    TurnClockwise,
    TurnCounterClockwise,
}

#[derive(Component)]
struct ScratchScripts(Vec<ScratchCode>);

//
// Stage
//

pub struct ScratchStagePlugin;

#[derive(Resource)]
struct SpriteNameTimer(Timer);

impl Plugin for ScratchStagePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpriteNameTimer(Timer::from_seconds(
                2.0,
                TimerMode::Repeating,
            )))
            .add_system_set(SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP))
                .with_system(step_thread)
            )
            .add_startup_system(add_stage_startup);
    }
}

fn add_stage_startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn step_thread(mut thread_query: Query<(&mut Transform, &ScratchScripts)>) {
    let one_degree_in_radians: f32 = (1.0_f32).to_radians();

    for (mut transform, scripts) in &mut thread_query {
        for script in &scripts.0 {
            match script {
                ScratchCode::MoveOneStep => {
                    let forward = transform.right();
                    transform.translation += forward;
                },
                ScratchCode::MoveTwoSteps => {
                    let forward = transform.right();
                    transform.translation += 2. * forward;
                },
                ScratchCode::TurnClockwise => {
                    transform.rotate_z(-one_degree_in_radians);
                },
                ScratchCode::TurnCounterClockwise => {
                    transform.rotate_z(one_degree_in_radians);
                },
            }
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
    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("squirrel.png"),
            transform: Transform::from_scale(Vec3::new(0.5, 0.5, 1.)),
            ..default()
        })
        .insert(Name("Sprite 1".to_string()))
        .insert(ScratchScripts(vec![
            ScratchCode::MoveOneStep,
            ScratchCode::TurnClockwise,
        ]));
    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("squirrel.png"),
            transform: Transform::from_scale(Vec3::new(0.5, 0.5, 1.)),
            ..default()
        })
        .insert(Name("Sprite 2".to_string()))
        .insert(ScratchScripts(vec![
            ScratchCode::MoveTwoSteps,
            ScratchCode::TurnCounterClockwise,
        ]));
}
