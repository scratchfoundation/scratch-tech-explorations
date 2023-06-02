use bevy::{
    asset::AssetLoader,
    prelude::*,
    reflect::TypeUuid,
    utils::HashSet,
};

use crate::sb2::{self, load_project_from_zip};

#[derive(Debug)]
#[derive(TypeUuid)]
#[uuid = "7e6fc139-66f6-4916-a118-5ae4b90e7bae"]
pub enum ScratchProject {
    SB2(sb2::Project),
}

pub struct ScratchProjectPlugin;

impl Plugin for ScratchProjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<ScratchProject>()
            .add_asset_loader(ScratchProjectLoader)
            .init_asset_loader::<ScratchProjectLoader>();
    }
}

#[derive(Default)]
struct ScratchProjectLoader;

impl AssetLoader for ScratchProjectLoader {
    fn extensions(&self) -> &[&str] {
        &["sb2", "sb3", "json"]
    }

    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            load_scratch_project(bytes, load_context).await
        })
    }
}

async fn load_scratch_project<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut bevy::asset::LoadContext<'b>,
) -> Result<(), bevy::asset::Error> {
    // TODO: determine what kind of file this is and load it appropriately
    // Supporting everything means: SB binary, SB2 ZIP, SB3 ZIP, SB2 JSON, SB3 JSON
    // For now, assume it's a zipped SB2
    load_project_from_zip(bytes, load_context)
}
