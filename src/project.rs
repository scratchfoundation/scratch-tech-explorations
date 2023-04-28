use std::io;

use bevy::asset::LoadState;
use bevy::prelude::*;

use bevy::prelude::system_adapter::new;
use bevy::utils::HashSet;
use zip::ZipArchive;

use crate::AppState;
use crate::assets::json_asset_plugin::JSONAsset;
use crate::assets::zip_asset_plugin::ZipAsset;
use crate::virtual_machine::VirtualMachine;
use crate::virtual_machine::load::VMLoadError;

pub struct ScratchDemoProjectPlugin;

impl Plugin for ScratchDemoProjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(project_load)
            .add_system(project_check_load
                .in_set(OnUpdate(AppState::Loading)));
    }
}

enum LoadPhaseSB2 {
    ProjectJSON(Handle<JSONAsset>),
    Assets(VirtualMachine, HashSet<HandleUntyped>),
}

#[derive(Resource)]
struct LoadingProjectSB2 {
    phase: LoadPhaseSB2,
    sb2_path: Option<String>,
}

fn project_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
 ) {
    info!("Starting project load");
    let sb2_path = "Infinite ToeBeans.sb2";
    let sb2_json = asset_server.load(sb2_path.to_string() + "#project.json");
    commands.insert_resource(LoadingProjectSB2 {
        phase: LoadPhaseSB2::ProjectJSON(sb2_json),
        sb2_path: Some(sb2_path.to_string()),
    });
}

fn project_check_load(
    mut commands: Commands,
    mut loading_project: ResMut<LoadingProjectSB2>,
    asset_server: Res<AssetServer>,
    assets_json: Res<Assets<JSONAsset>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    match &mut loading_project.phase {
        LoadPhaseSB2::ProjectJSON(ref sb2_json) => {
            match asset_server.get_load_state(sb2_json) {
                LoadState::Failed => {
                    panic!("failed to load project");
                }
                LoadState::Loaded => {
                    info!("loaded");
                    if let Some(sb2_json) = assets_json.get(sb2_json) {
                        let (vm, loading_assets) = VirtualMachine::from_sb2_json(&sb2_json.0, &loading_project.sb2_path, asset_server).unwrap();
                        loading_project.phase = LoadPhaseSB2::Assets(vm, loading_assets);
                    }
                }
                _ => {
                    // not loaded / loading / unloaded: no need to do anything
                }
            }
        }
        LoadPhaseSB2::Assets(vm, ref mut loading_assets) => {
            info!("checking {} assets", loading_assets.len());
            loading_assets
                .drain_filter(|handle|
                    asset_server.get_load_state(handle) == LoadState::Loaded
                );
            info!("remaining assets: {}", loading_assets.len());
            if loading_assets.is_empty() {
                app_state.set(AppState::Running);
            }
        },
    }
}
