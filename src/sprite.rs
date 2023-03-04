use bevy::prelude::*;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Costume(String);

#[derive(Debug)]
#[allow(dead_code)] // until the new thread step function
pub enum ScratchCode {
    MoveOneStep,
    MoveTwoSteps,
    TurnClockwise,
    TurnCounterClockwise,
}

#[derive(Component)]
pub struct ScratchScripts(
    pub Vec<ScratchCode>
);
