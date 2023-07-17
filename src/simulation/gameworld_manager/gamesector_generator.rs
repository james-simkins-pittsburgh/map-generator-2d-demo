use bevy::prelude::*;

use rand_chacha::rand_core::SeedableRng;
use rand_core::RngCore;
use rand_chacha;


// These use declaration are just to make the code more readable.

use crate::SECTOR_SIZE;
use crate::simulation::TileType;
use crate::simulation::SectorBiome;
use crate::simulation::GamesectorTileMap;

// This function generates a new sector and populates it with a procedurally generated environmenal map and the other components it needs.

pub fn generate_sector(
    mut generate_new_sector_event: EventReader<crate::GenerateNewSector>,
    mut sector_to_be_generated: ResMut<super::SectorToBeGenerated>,
    gameworld_seed: Res<crate::GameworldSeed>,
    mut commands: Commands
) {
    // If there are events recorded in generate sector then the rest of the function runs.

    if !generate_new_sector_event.is_empty() {
        // This iterates over the vector of tuples of coordinates of the gamesectors to generated.

        for new_sector_coordinates in &sector_to_be_generated.sector_to_be_generated_coordinates {
            // This returns into a tuple the data for a new gamesector.

            let new_sector_data = generate_map(
                //  A gamesector is generated from its coordinates and a gamewold worldseed. Everything else is procedural.

                new_sector_coordinates.0.clone(),
                new_sector_coordinates.1.clone(),
                &gameworld_seed.gameworld_seed_num
            );

            // This creates a new gamesector entity in the ECS and pushes the data from the tuple to it.
            commands.spawn(super::super::GamesectorBundle {
                gamesector_tile_map: GamesectorTileMap {
                    // This stores the data of what tile goes where in the new sector's tile map component.
                    tile_array: new_sector_data.0,

                    // This stores the sector's biome in the tile map component.
                    sector_biome: new_sector_data.1,

                    // This stores the sector's coordinates in the gameworld.
                    sector_coordinates: new_sector_data.2,
                },
            });
        }
    }

    // This clears the event so sectors aren't generated more than once.
    generate_new_sector_event.clear();
    // This clears the coordinates of the sector to be generated.
    sector_to_be_generated.sector_to_be_generated_coordinates.clear();
}

// This function does the actual procedural generation of the gameworld.

fn generate_map(
    x_coordinate: i32,
    y_coordinate: i32,
    gameworld_seed_num: &u64
) -> ([[TileType; SECTOR_SIZE as usize]; SECTOR_SIZE as usize], SectorBiome, (i32, i32)) {
    
    // This initially sets the array to open. 
    let gamesector_environment_array = [
        [TileType::Open; SECTOR_SIZE as usize];
        SECTOR_SIZE as usize
    ];

    // This is just placeholder code
    let sector_biome = SectorBiome::Base;

    // Code to generate gameworld will go here.

    let mut _seeded_prng = rand_chacha::ChaCha8Rng::seed_from_u64(gameworld_seed_num.clone());

    (gamesector_environment_array, sector_biome, (x_coordinate, y_coordinate))
}
