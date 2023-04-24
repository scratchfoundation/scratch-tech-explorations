use bevy::{
    prelude::*,
    tasks::*,
};
use futures_lite::future;
use std::{fs, path::Path};

use crate::AppState;
use crate::virtual_machine::VirtualMachine;
use crate::virtual_machine::load::VMLoadResult;

pub struct ScratchDemoProjectPlugin;

impl Plugin for ScratchDemoProjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(project_load)
            .add_system(project_check_load
                .in_set(OnUpdate(AppState::Loading)));
    }
}

#[derive(Resource)]
struct ProjectLoadTask(Task<VMLoadResult>);

fn project_load(mut commands: Commands) {
    info!("Starting project load");
    let thread_pool = AsyncComputeTaskPool::get();
    let load_task = thread_pool.spawn(async move {
        load_sb2("assets/Infinite ToeBeans.sb2").await
    });
    commands.insert_resource(ProjectLoadTask(load_task));
}

fn project_check_load(mut app_state: ResMut<NextState<AppState>>, mut project_task: Option<ResMut<ProjectLoadTask>>) {
    if let Some(project_task) = &mut project_task {
        if let Some(project_load_result) = future::block_on(future::poll_once(&mut project_task.0)) {
            match project_load_result {
                Ok(vm) => info!("Project loaded data: {:#?}", vm),
                Err(project_error) => error!("Project load failure: {}", project_error),
            }

            app_state.set(AppState::Running);
        }
    }
}

async fn load_sb2(path: impl AsRef<Path>) -> VMLoadResult {
    let new_vm = deserialize_sb2(path).await?;

    // stop the old VM
    // replace it with the new VM
    // start the VM (but not the project)

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

    Ok(new_vm)
}

async fn deserialize_sb2(path: impl AsRef<Path>) -> VMLoadResult {

    // TODO: would it make sense to use async_zip instead? ...but will tokio conflict with bevy?

    let file = fs::File::open(&path)?;

    let new_vm = VirtualMachine::from_sb2_reader(file)?;

    Ok(new_vm) // return hydrated project
}
