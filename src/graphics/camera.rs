use bevy::prelude::*;
pub struct HiveboticaCameraPlugin;

impl Plugin for HiveboticaCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup);
    }
}
// This spawns the camera using the Bevy default 2D settings.

pub fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
