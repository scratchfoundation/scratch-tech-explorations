mod loading_screen;
mod project;
mod project_bundle;
mod sb3;
mod sprite;
mod stage;

use bevy::{
    prelude::*,
    window::*,
};

use loading_screen::ScratchLoadingScreenPlugin;
use project::ScratchDemoProjectPlugin;
use stage::ScratchStagePlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]

enum AppState {
    Loading,
    Running,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "scratch-bevy".to_string(),
                width: 960.,
                height: 720.,
                resize_constraints: default(),
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::Windowed,
                ..default()
            },
            ..default()
        }))
        .add_plugin(ScratchLoadingScreenPlugin)
        .add_plugin(ScratchStagePlugin)
        .add_plugin(ScratchDemoProjectPlugin)
        .add_system(close_on_esc)
        .run();
}
