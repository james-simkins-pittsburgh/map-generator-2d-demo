use bevy::prelude::*;
pub struct HiveboticaGameworldManagerPlugin;

impl Plugin for HiveboticaGameworldManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            gamesector_generator::generate_sector,
        ).init_resource::<SectorToBeGenerated>();
    }
}

#[derive(Resource, Default)]
pub struct SectorToBeGenerated {
    pub sector_to_be_generated_coordinates: Vec<(i32, i32)>,
}
// This module holds the logic for gamesector generation.
pub(super) mod gamesector_generator;
