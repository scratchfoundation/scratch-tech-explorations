use std::time::Duration;

use bevy::{prelude::*, render::camera::ScalingMode, time::common_conditions::on_timer, utils::Instant};

use crate::{virtual_machine as VM, AppState};

/// The time between each frame of the VM, when running at full speed.
const TIME_STEP: Duration = Duration::new(0, (Duration::from_secs(1).as_nanos() / 30) as u32);

/// When the work time expires, the VM will stop running code until next tick.
/// This is 3/4 of TIME_STEP
const WORK_TIME: Duration = Duration::new(0, (Duration::from_secs(1).as_nanos() / 40) as u32);

pub struct RuntimePlugin;

#[derive(Resource)]
struct WorkTimer(Instant);

impl Plugin for RuntimePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_stage_startup);

        app.insert_resource(WorkTimer(Instant::now()));

        app.configure_set(OnUpdate(AppState::Running).run_if(on_timer(TIME_STEP)));
        app.add_systems((
            reset_work_timer,
            start_hats,
            step_threads,
        ).in_set(OnUpdate(AppState::Running)));
    }
}

fn add_stage_startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: 480.0,
                min_height: 360.0,
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

fn reset_work_timer(
    mut commands: Commands,
) {
    commands.insert_resource(WorkTimer(Instant::now()));
}

fn start_hats(
    _vm: ResMut<VM::VirtualMachine>,
) {
}

/// Run one "tick" of the VM, doing as much work as possible before the work timer expires
fn step_threads(
    work_timer: Res<WorkTimer>,
    mut thread_query: Query<(&mut Transform, &VM::Target)>,
) {
    let one_degree_in_radians: f32 = (1.0_f32).to_radians();
    loop {
        for (mut transform, _target) in &mut thread_query {
            transform.rotate_z(one_degree_in_radians);
        }
        if work_timer.0.elapsed() >= WORK_TIME {
            break;
        }
    }
}
