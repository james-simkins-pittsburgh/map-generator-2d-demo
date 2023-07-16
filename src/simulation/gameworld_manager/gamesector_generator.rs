use bevy::prelude::*;
use rand_core::SeedableRng;

use crate::simulation::GamesectorTileMap;

pub fn generate_sector_map(
    mut generate_sector_map: EventReader<crate::GenerateSectorMap>,
    mut sector_to_be_generated: ResMut<super::SectorToBeGenerated>,
    gameworld_seed: Res<crate::GameworldSeed>,
    mut commands: Commands
) {
    if !generate_sector_map.is_empty() {
        for new_sector_coordinates in &sector_to_be_generated.sector_to_be_generated_coordinates {
            let new_sector_data = generate_map(
                &new_sector_coordinates.0,
                &new_sector_coordinates.1,
                &gameworld_seed.gameworld_seed_array
            );
            commands.spawn(super::super::GamesectorBundle {
                gamesector_tile_map: GamesectorTileMap {
                    sector_coordinates: new_sector_coordinates.clone(),
                    tile_array: new_sector_data.0,
                    sector_biome: new_sector_data.1,
                },
            });
        }
    }
    generate_sector_map.clear();
    sector_to_be_generated.sector_to_be_generated_coordinates.clear();
}

fn generate_map(
    x_coordinate: &i32,
    y_coordinate: &i32,
    gameworld_seed_array: &[u8; 12]
) -> (
    [[crate::simulation::TileType; crate::SECTOR_SIZE as usize]; crate::SECTOR_SIZE as usize],
    crate::simulation::SectorBiome,
) {
    let gamesector_environment_array = [
        [crate::simulation::TileType::Open; crate::SECTOR_SIZE as usize];
        crate::SECTOR_SIZE as usize
    ];
    let sector_biome = crate::simulation::SectorBiome::Base;

    // Code to generate gameworld here

    (gamesector_environment_array, sector_biome)
}
