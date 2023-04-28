mod assets;
mod loading_screen;
mod project;
mod sb2;
mod sprite;
mod stage;
mod virtual_machine;

use assets::{
    json_asset_plugin::JSONAssetPlugin,
    zip_asset_plugin::ZipAssetPlugin,
};

use bevy::{
    prelude::*,
    window::*,
};

use loading_screen::ScratchLoadingScreenPlugin;
use project::ScratchDemoProjectPlugin;
use stage::ScratchStagePlugin;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]

enum AppState {
    #[default]
    Loading,
    Running,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "scratch-bevy".to_string(),
                resolution: (960., 720.).into(),
                resize_constraints: default(),
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(JSONAssetPlugin)
        .add_plugin(ZipAssetPlugin)
        .add_state::<AppState>()
        .add_plugin(ScratchLoadingScreenPlugin)
        .add_plugin(ScratchStagePlugin)
        .add_plugin(ScratchDemoProjectPlugin)
        .add_system(close_on_esc)
        .run();
}
