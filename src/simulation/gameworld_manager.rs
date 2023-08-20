use bevy::prelude::*;

// This module holds the logic for gamesector generation.
pub(super) mod gamesector_generator;
pub(super) mod testing_mode_new_sector;


pub struct HiveboticaGameworldManagerPlugin;

impl Plugin for HiveboticaGameworldManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (testing_mode_new_sector::add_sector_if_needed, gamesector_generator::generate_sector).chain())
        .init_resource::<SectorToBeGenerated>();
    }
}

pub enum InitializationType {

    Empty,
    _Existing,
    Player,
    _Guardian,

}

impl Default for InitializationType {
    fn default() -> Self { InitializationType::Empty }
}

#[derive(Resource, Default)]
pub struct SectorToBeGenerated {
    pub sector_to_be_generated_list: Vec<(i32, i32, InitializationType)>,
}
