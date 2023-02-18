use bevy::{
    prelude::*,
    tasks::*,
};
use futures_lite::future;
use std::{fs, path::Path};
use zip::result::ZipError;

use crate::AppState;
use crate::project_bundle::ProjectBundle;
use crate::sb3::SB3;

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

type ProjectLoadResult = Result<ProjectBundle, ProjectLoadError>;

#[derive(Debug)]
pub enum ProjectLoadError {
    IoError(std::io::Error),
    ParseError(serde_json::Error),
    ZipError(ZipError),
}

impl From<std::io::Error> for ProjectLoadError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<serde_json::Error> for ProjectLoadError {
    fn from(err: serde_json::Error) -> Self {
        Self::ParseError(err)
    }
}

impl From<ZipError> for ProjectLoadError {
    fn from(err: ZipError) -> Self {
        Self::ZipError(err)
    }
}

impl std::fmt::Display for ProjectLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IoError(err) => write!(f, "{}", err),
            Self::ParseError(err) => write!(f, "{}", err),
            Self::ZipError(err) => write!(f, "{}", err),
        }
    }
}

#[derive(Resource)]
struct ProjectLoadTask(Task<ProjectLoadResult>);

fn project_load(mut commands: Commands) {
    info!("Starting project load");
    let thread_pool = AsyncComputeTaskPool::get();
    let load_task = thread_pool.spawn(async move {
        load_sb3("assets/Infinite ToeBeans.sb3").await
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

async fn load_sb3(path: impl AsRef<Path>) -> ProjectLoadResult {
    let project_content = deserialize_sb3(path).await;

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
        }
    }

    project_content
}

async fn deserialize_sb3(path: impl AsRef<Path>) -> ProjectLoadResult {

    // TODO: would it make sense to use async_zip instead? ...but will tokio conflict with bevy?

    let file = fs::File::open(&path)?;

    let project_bundle = SB3::from_reader(file)?;

    Ok(project_bundle) // return hydrated project
}
