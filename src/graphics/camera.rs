use bevy::prelude::*;

// This spawns the camera using the Bevy default 2D settings.

pub fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}