use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
};

#[derive(TypeUuid)]
#[uuid = "8a321feb-c747-435b-9ad7-443d19c6966a"]
pub struct JSONAsset(pub Vec<u8>);

pub struct JSONAssetPlugin;

impl Plugin for JSONAssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<JSONAsset>()
            .add_asset_loader(JSONAssetLoader)
            .init_asset_loader::<JSONAssetLoader>();
    }
}

#[derive(Default)]
struct JSONAssetLoader;

impl AssetLoader for JSONAssetLoader {
    fn extensions(&self) -> &[&str] {
        &["json"]
    }

    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            load_json(bytes, load_context).await
        })
    }
}

async fn load_json<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut bevy::asset::LoadContext<'b>,
) -> Result<(), bevy::asset::Error> {
    load_context.set_default_asset(LoadedAsset::new(JSONAsset(bytes.into())));
    Ok(())
}
