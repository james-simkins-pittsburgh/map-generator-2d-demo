use bevy::prelude::*;

// This module holds the logic for gamesector generation.
pub mod gamesector_generator;

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
