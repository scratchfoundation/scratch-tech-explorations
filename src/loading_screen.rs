use bevy::prelude::*;

use crate::AppState;

pub struct ScratchLoadingScreenPlugin;

#[derive(Component)]
struct LoadingScreen;

impl Plugin for ScratchLoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(ScratchLoadingScreenPlugin::start_loading_screen
                .in_schedule(OnEnter(AppState::Loading)))
            .add_system(ScratchLoadingScreenPlugin::update_loading_screen
                .in_set(OnUpdate(AppState::Loading)))
            .add_system(ScratchLoadingScreenPlugin::stop_loading_screen
                .in_schedule(OnExit(AppState::Loading)));
    }
}

impl ScratchLoadingScreenPlugin {

    fn start_loading_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
        info!("start_loading_screen");
        commands.spawn((
            Text2dBundle {
                text: Text::from_section("Loading...",
                    TextStyle {
                        font: asset_server.load("fonts/Scratch.ttf"),
                        font_size: 60.0,
                        color: Color::ORANGE,
                    }
                ).with_alignment(TextAlignment::Center),
                ..default()
            },
            LoadingScreen
        ));
    }

    fn update_loading_screen(time: Res<Time>, mut loading_screen_query: Query<&mut Transform, With<LoadingScreen>>) {
        for mut transform in &mut loading_screen_query {
            transform.rotation = Quat::from_rotation_z((5. * time.elapsed_seconds()).cos());
        }
    }

    fn stop_loading_screen(mut commands: Commands, mut loading_screen_query: Query<Entity, With<LoadingScreen>>) {
        info!("stop_loading_screen");
        for loading_screen in &mut loading_screen_query {
            commands.entity(loading_screen).despawn();
        }

    }
}
