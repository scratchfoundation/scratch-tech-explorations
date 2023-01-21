use bevy::{
    prelude::*,
    tasks::*,
};
use futures_lite::future;
use std::{fs, path::Path};
use zip::ZipArchive;

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
struct ProjectLoadTask(Task<usize>);

fn project_load(mut commands: Commands) {
    info!("Starting project load");
    let thread_pool = AsyncComputeTaskPool::get();
    let load_task = thread_pool.spawn(async move {
        deserialize_sb3("assets/Infinite ToeBeans.sb3").await
    });
    commands.insert_resource(ProjectLoadTask(load_task));
}

fn project_check_load(mut app_state: ResMut<State<AppState>>, mut project_task: Option<ResMut<ProjectLoadTask>>) {
    if let Some(project_task) = &mut project_task {
        if let Some(project_data) = future::block_on(future::poll_once(&mut project_task.0)) {
            info!("Project data is: {}", project_data);
            app_state.set(AppState::Running).unwrap();
        }
    }
}

async fn deserialize_sb3(path: impl AsRef<Path>) -> usize {

    // TODO: would it make sense to use async_zip instead? ...but will tokio conflict with bevy?

    let file = fs::File::open(&path).unwrap();
    let sb3_zip = ZipArchive::new(file).unwrap();

    let start_time = std::time::Instant::now();
    while start_time.elapsed() < std::time::Duration::from_secs_f32(4.2)
    {
        // spin to pretend we're loading lots of stuff
    }

    sb3_zip.len() // return hydrated project
}
