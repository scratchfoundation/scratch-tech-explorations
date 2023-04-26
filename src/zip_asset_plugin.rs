use std::io::Cursor;

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
};

use zip::ZipArchive;


#[derive(TypeUuid)]
#[uuid = "b27daf98-015c-473e-bba7-631b00d45925"]
pub struct ZipAsset(pub ZipArchive<Cursor<Vec<u8>>>);

pub struct ZipAssetPlugin;

impl Plugin for ZipAssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<ZipAsset>()
            .add_asset_loader(ZipAssetLoader)
            .init_asset_loader::<ZipAssetLoader>();
    }
}

#[derive(Default)]
struct ZipAssetLoader;

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
    info!("loading a ZIP");
    let reader = Cursor::new(bytes.to_vec());
    let zip = ZipArchive::new(reader)?;
    load_context.set_default_asset(LoadedAsset::new(ZipAsset(zip)));
    Ok(())
}
