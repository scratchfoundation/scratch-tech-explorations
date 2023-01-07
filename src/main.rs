use bevy::{
    prelude::*,
    tasks::*,
    time::FixedTimestep,
    window::*,
};
use futures_lite::future;

const TIME_STEP: f64 = 1. / 30.;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]

enum AppState {
    Loading,
    Running,
}

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
        .add_plugin(ScratchLoadingScreenPlugin)
        .add_plugin(ScratchStagePlugin)
        .add_plugin(ScratchDemoProjectPlugin)
        .add_system(close_on_esc)
        .run();
}

//
// Loading screen
//

pub struct ScratchLoadingScreenPlugin;

#[derive(Component)]
struct LoadingScreen;

impl Plugin for ScratchLoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(AppState::Loading)
            .add_system_set(
                SystemSet::on_enter(AppState::Loading)
                    .with_system(ScratchLoadingScreenPlugin::start_loading_screen)
            )
            .add_system_set(
                SystemSet::on_update(AppState::Loading)
                    .with_system(ScratchLoadingScreenPlugin::update_loading_screen)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Loading)
                    .with_system(ScratchLoadingScreenPlugin::stop_loading_screen)
            );
    }
}

impl ScratchLoadingScreenPlugin {

    fn start_loading_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            Text2dBundle {
                text: Text::from_section("Loading...",
                    TextStyle {
                        font: asset_server.load("fonts/Scratch.ttf"),
                        font_size: 60.0,
                        color: Color::ORANGE,
                    }
                ).with_alignment(TextAlignment::CENTER),
                ..default()
            },
            LoadingScreen
        ));
    }

    fn update_loading_screen(time: Res<Time>, mut loading_screen_query: Query<&mut Transform, With<LoadingScreen>>) {
        for mut transform in &mut loading_screen_query {
            transform.rotation = Quat::from_rotation_z((5. * time.elapsed_seconds()).cos());
        }
    }

    fn stop_loading_screen(mut commands: Commands, mut loading_screen_query: Query<Entity, With<LoadingScreen>>) {
        for loading_screen in &mut loading_screen_query {
            commands.entity(loading_screen).despawn();
        }

    }
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
            .add_startup_system(project_load)
            .add_system_set(
                SystemSet::on_update(AppState::Loading)
                    .with_system(project_check_load)
            );
    }
}

#[derive(Resource)]
struct ProjectZip(HandleUntyped);

#[derive(Resource)]
struct ProjectLoadTask(Task<i32>);

fn project_load(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Starting project load");
    let project_handle = asset_server.load_untyped("Infinite Toebeans.sb3");
    commands.insert_resource(ProjectZip(project_handle))
}

fn project_check_load(mut commands: Commands, mut app_state: ResMut<State<AppState>>, project_zip: Option<Res<ProjectZip>>, mut project_task: Option<ResMut<ProjectLoadTask>>) {
    if let Some(project_zip) = project_zip {
        if project_zip.is_added() {
            info!("Project archive is loaded. Time to unpack...");
            let thread_pool = AsyncComputeTaskPool::get();
            let load_task = thread_pool.spawn(async move {
                let start_time = std::time::Instant::now();
                while start_time.elapsed() < std::time::Duration::from_secs_f32(4.2)
                {
                    // spin
                }
                42 // return hydrated project
            });
            commands.insert_resource(ProjectLoadTask(load_task));
        }
        else if let Some(project_task) = &mut project_task {
            if let Some(project_data) = future::block_on(future::poll_once(&mut project_task.0)) {
                info!("Project data is: {}", project_data);
                app_state.set(AppState::Running).unwrap();
            }
        }
    }
}
