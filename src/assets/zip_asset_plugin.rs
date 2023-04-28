use std::io::{Cursor, Read};

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    render::texture::CompressedImageFormats,
};

use zip::ZipArchive;

use crate::assets::json_asset_plugin::JSONAsset;


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
    info!("loading a ZIP ({} bytes)", bytes.len());
    let reader = Cursor::new(bytes.to_vec());
    let mut zip = ZipArchive::new(reader)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;

        let label = file.name().to_owned();

        let mut bytes = vec![];
        file.read_to_end(&mut bytes)?;

        add_labeled_asset(load_context, &label, &bytes)?;
    }
    load_context.set_default_asset(LoadedAsset::new(ZipAsset(zip)));
    Ok(())
}

fn add_labeled_asset(load_context: &mut LoadContext, label: &str, bytes: &[u8]) -> Result<(), bevy::asset::Error> {
    let lower_label = label.to_lowercase();

    // pretend that loading takes a long time
    std::thread::sleep(std::time::Duration::from_millis(250));

    if lower_label.ends_with(".png") {
        info!("registering PNG: {}", label);
        let image = Image::from_buffer(
            bytes,
            bevy::render::texture::ImageType::Extension("png"),
            CompressedImageFormats::NONE,
            true
        )?;
        load_context.set_labeled_asset(label, LoadedAsset::new(image));

        return Ok(());
    }

    if lower_label.ends_with(".wav") || lower_label.ends_with(".mp3") {
        info!("registering audio: {}", label);
        let audio = AudioSource {
            bytes: bytes.into()
        };
        load_context.set_labeled_asset(label, LoadedAsset::new(audio));

        return Ok(());
    }

    if lower_label.ends_with(".json") {
        info!("registering JSON: {}", label);
        let asset = JSONAsset(bytes.into());
        load_context.set_labeled_asset(label, LoadedAsset::new(asset));

        return Ok(());
    }

    info!("ignoring unrecognized file: {}", label);
    Ok(())
}
