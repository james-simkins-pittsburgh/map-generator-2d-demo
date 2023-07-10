use bevy::prelude::*;
pub struct HiveboticaGameworldManagerPlugin;

impl Plugin for HiveboticaGameworldManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, gamesector_generator::generate_base_sector_map);
    }
}

// This module holds the logic for gamesector generation.
pub(super) mod gamesector_generator;

#[derive(Component)]
pub struct SimTileLocation {
    pub xofsector: i32,
    pub yofsector: i32,
    pub localx: i16,
    pub localy: i16,
}

#[derive(Bundle)]
pub struct SimTileBundle {
    sim_tile_location: SimTileLocation,
    sprite_bundle: SpriteBundle,
}
