use bevy::prelude::*;
pub struct HiveboticaGameworldManagerPlugin;

impl Plugin for HiveboticaGameworldManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, gamesector_generator::generate_base_sector_map);
    }
}

// This module holds the logic for gamesector generation.
pub(super) mod gamesector_generator;
