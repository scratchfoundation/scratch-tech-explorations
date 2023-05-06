use bevy::asset::LoadState;
use bevy::prelude::*;

use crate::AppState;
use crate::assets::scratch_project_plugin::ScratchProject;

pub struct ScratchDemoProjectPlugin;

impl Plugin for ScratchDemoProjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(project_load)
            .add_system(project_check_load
                .in_set(OnUpdate(AppState::Loading)))
            .add_system(spawn_project
                .in_schedule(OnEnter(AppState::Running)));
    }
}

#[derive(Resource)]
struct LoadingScratchProject {
    project: Handle<ScratchProject>,
}

fn project_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
 ) {
    info!("Starting project load");
    let sb2_path = "Infinite ToeBeans.sb2";
    let project = asset_server.load::<ScratchProject, _>(sb2_path);
    commands.insert_resource(LoadingScratchProject {
        project
    });
}

fn project_check_load(
    loading_project: Res<LoadingScratchProject>,
    asset_server: Res<AssetServer>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    match asset_server.get_load_state(&loading_project.project) {
        LoadState::Failed => {
            panic!("failed to load project");
        }
        LoadState::Loaded => {
            info!("loaded");
            app_state.set(AppState::Running);
        }
        _ => {
            // not loaded / loading / unloaded: no need to do anything
        }
    }
}

fn spawn_project(
    mut commands: Commands,
    loading_project: Res<LoadingScratchProject>,
    project_assets: Res<Assets<ScratchProject>>,
) {
    let project = project_assets.get(&loading_project.project)
        .expect("Tried to spawn a project that doesn't exist!");
    commands.remove_resource::<LoadingScratchProject>();
    project.vm.spawn(commands);
}
