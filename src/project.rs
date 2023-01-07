use bevy::{
    prelude::*,
    tasks::*,
};
use futures_lite::future;

use crate::AppState;

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
