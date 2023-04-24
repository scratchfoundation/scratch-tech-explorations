use bevy::prelude::*;

use crate::sprite::{ScratchCode, ScratchScripts};

const TIME_STEP: f32 = 1. / 30.;


pub struct ScratchStagePlugin;

#[derive(Resource)]
struct SpriteNameTimer(Timer);

impl Plugin for ScratchStagePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpriteNameTimer(Timer::from_seconds(
                2.0,
                TimerMode::Repeating,
            )))
            .insert_resource(FixedTime::new_from_secs(TIME_STEP))
            .add_system(step_thread.in_schedule(CoreSchedule::FixedUpdate))
            .add_startup_system(add_stage_startup);
    }
}

fn add_stage_startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn step_thread(mut thread_query: Query<(&mut Transform, &ScratchScripts)>) {
    let one_degree_in_radians: f32 = (1.0_f32).to_radians();

    for (mut transform, scripts) in &mut thread_query {
        for script in &scripts.0 {
            match script {
                ScratchCode::MoveOneStep => {
                    let forward = transform.right();
                    transform.translation += forward;
                },
                ScratchCode::MoveTwoSteps => {
                    let forward = transform.right();
                    transform.translation += 2. * forward;
                },
                ScratchCode::TurnClockwise => {
                    transform.rotate_z(-one_degree_in_radians);
                },
                ScratchCode::TurnCounterClockwise => {
                    transform.rotate_z(one_degree_in_radians);
                },
            }
        }
    }
}
