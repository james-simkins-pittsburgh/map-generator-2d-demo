use bevy::prelude::*;

// This module holds the logic for gamesector generation.
pub(super) mod gamesector_generator;

pub struct HiveboticaGameworldManagerPlugin;

impl Plugin for HiveboticaGameworldManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            gamesector_generator::generate_sector,
        ).init_resource::<SectorToBeGenerated>();
    }
}

pub enum InitializationType {

    Empty,
    Existing,
    Player,
    Guardian,

}

impl Default for InitializationType {
    fn default() -> Self { InitializationType::Empty }
}

#[derive(Resource, Default)]
pub struct SectorToBeGenerated {
    pub sector_to_be_generated_list: Vec<(i32, i32, InitializationType)>,
}
