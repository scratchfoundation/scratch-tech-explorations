use bevy::prelude::*;

use crate::AppState;

pub struct ScratchLoadingScreenPlugin;

#[derive(Component)]
struct LoadingScreen;

impl Plugin for ScratchLoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(AppState::Loading)
            .add_system_set(
                SystemSet::on_enter(AppState::Loading)
                    .with_system(ScratchLoadingScreenPlugin::start_loading_screen)
            )
            .add_system_set(
                SystemSet::on_update(AppState::Loading)
                    .with_system(ScratchLoadingScreenPlugin::update_loading_screen)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Loading)
                    .with_system(ScratchLoadingScreenPlugin::stop_loading_screen)
            );
    }
}

impl ScratchLoadingScreenPlugin {

    fn start_loading_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            Text2dBundle {
                text: Text::from_section("Loading...",
                    TextStyle {
                        font: asset_server.load("fonts/Scratch.ttf"),
                        font_size: 60.0,
                        color: Color::ORANGE,
                    }
                ).with_alignment(TextAlignment::CENTER),
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
        for loading_screen in &mut loading_screen_query {
            commands.entity(loading_screen).despawn();
        }

    }
}
