use std::io;

use bevy::prelude::*;

use zip::ZipArchive;

use crate::AppState;
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

#[derive(Resource)]
struct LoadingProjectSB2(Handle<ZipAsset>);

#[derive(Resource)]
struct LoadingProjectAssets{
    loading_assets: Vec<HandleUntyped>,
    vm: VirtualMachine,
}

fn project_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
 ) {
    info!("Starting project load");
    let sb2_handle = asset_server.load("Infinite ToeBeans.sb2#project.json");
    commands.insert_resource(LoadingProjectSB2(sb2_handle));
}

fn project_check_load(
    mut commands: Commands,
    loading_project: ResMut<LoadingProjectSB2>,
    asset_server: Res<AssetServer>,
    mut sb2_assets: ResMut<Assets<ZipAsset>>,
) {
    use bevy::asset::LoadState;
    match asset_server.get_load_state(&loading_project.0) {
        LoadState::Failed => {
            panic!("failed to load project");
        }
        LoadState::Loaded => {
            if let Some(project_sb2) = sb2_assets.get_mut(&loading_project.0) {
                commands.remove_resource::<LoadingProjectSB2>();
                commands.insert_resource(
                    load_sb2_assets(&mut (project_sb2.0), asset_server)
                        .unwrap()
                );
            }
        },
        _ => {
            // not loaded / loading / unloaded: no need to do anything
        }
    }
}

fn load_sb2_assets<R>(
    sb2_zip: &mut ZipArchive<R>,
    asset_server: Res<AssetServer>,
) -> Result<LoadingProjectAssets, VMLoadError>
    where
        R: io::Read + std::io::Seek,
{
    let new_vm = VirtualMachine::from_sb2_zip(sb2_zip)?;

    Ok(LoadingProjectAssets {
        loading_assets: todo!(),
        vm: new_vm,
    })
}
