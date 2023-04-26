use bevy::asset::LoadedAsset;
use zip::ZipArchive;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::reflect::TypeUuid;
use bevy::{
    prelude::*,
};
use std::io::{Cursor, self};
use std::{fs, path::Path};

use crate::AppState;
use crate::virtual_machine::VirtualMachine;
use crate::virtual_machine::load::{VMLoadResult, VMLoadError};

pub struct ScratchDemoProjectPlugin;

impl Plugin for ScratchDemoProjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<ZipAsset>()
            .add_asset_loader(ZipAssetLoader)
            .init_asset_loader::<ZipAssetLoader>()
            .add_startup_system(project_load)
            .add_system(project_check_load
                .in_set(OnUpdate(AppState::Loading)));
    }
}

#[derive(TypeUuid)]
#[uuid = "b27daf98-015c-473e-bba7-631b00d45925"]
pub struct ZipAsset(ZipArchive<Cursor<Vec<u8>>>);

#[derive(Default)]
pub struct ZipAssetLoader;

impl AssetLoader for ZipAssetLoader {
    fn extensions(&self) -> &[&str] {
        &["sb2", "sb3", "zip"]
    }

    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            load_zip(bytes, load_context).await
        })
    }
}

async fn load_zip<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut bevy::asset::LoadContext<'b>,
) -> Result<(), bevy::asset::Error> {
    let reader = Cursor::new(bytes.to_vec());
    let zip = ZipArchive::new(reader)?;
    load_context.set_default_asset(LoadedAsset::new(ZipAsset(zip)));
    Ok(())
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
    mut app_state: ResMut<NextState<AppState>>,
 ) {
    info!("Starting project load");
    let sb2_handle = asset_server.load("Infinite ToeBeans.sb2");
    commands.insert_resource(LoadingProjectSB2(sb2_handle));

}

fn project_check_load(
    mut commands: Commands,
    mut loading_project: ResMut<LoadingProjectSB2>,
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
