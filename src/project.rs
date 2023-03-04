use bevy::{
    prelude::*,
    tasks::*,
};
use futures_lite::future;
use std::{fs, path::Path};

use crate::AppState;
use crate::sb2;

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
struct ProjectLoadTask(Task<sb2::load::ProjectLoadResult>);

fn project_load(mut commands: Commands) {
    info!("Starting project load");
    let thread_pool = AsyncComputeTaskPool::get();
    let load_task = thread_pool.spawn(async move {
        load_sb2("assets/Infinite ToeBeans.sb2").await
    });
    commands.insert_resource(ProjectLoadTask(load_task));
}

fn project_check_load(mut app_state: ResMut<State<AppState>>, mut project_task: Option<ResMut<ProjectLoadTask>>) {
    if let Some(project_task) = &mut project_task {
        if let Some(project_load_result) = future::block_on(future::poll_once(&mut project_task.0)) {
            match project_load_result {
                Ok(project_data) => info!("Project loaded data: {:#?}", project_data),
                Err(project_error) => error!("Project load failure: {}", project_error),
            }

            app_state.set(AppState::Running).unwrap();
        }
    }
}

async fn load_sb2(path: impl AsRef<Path>) -> sb2::load::ProjectLoadResult {
    let project_content = deserialize_sb2(path).await;

    // validate project content
    // stop the VM
    // install project into runtime
    // start the VM

    // artificial delay
    {
        info!("delaying for a bit so you can watch the pretty loading screen");
        let start_time = std::time::Instant::now();
        while start_time.elapsed() < std::time::Duration::from_secs_f32(4.2)
        {
            // spin to pretend we're loading lots of stuff
            break; // or don't
        }
    }

    project_content
}

async fn deserialize_sb2(path: impl AsRef<Path>) -> sb2::load::ProjectLoadResult {

    // TODO: would it make sense to use async_zip instead? ...but will tokio conflict with bevy?

    let file = fs::File::open(&path)?;

    let project_bundle = sb2::Project::from_reader(file)?;

    Ok(project_bundle) // return hydrated project
}
