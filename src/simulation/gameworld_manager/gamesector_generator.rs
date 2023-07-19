use bevy::prelude::*;

use rand_chacha::rand_core::SeedableRng;
use rand_chacha;

// These use declaration are just to make the code more readable.

use crate::SECTOR_SIZE;
use crate::simulation::TileType;
use crate::simulation::SectorBiome;
use crate::simulation::GamesectorBasics;
use crate::simulation::GamesectorUnits;
use crate::simulation::UnitAttributes;
use super::InitializationType;

// This function generates a new sector and populates it with a procedurally generated environmenal map and the other components it needs.

pub fn generate_sector(
    mut generate_new_sector_event: EventReader<crate::GenerateNewSector>,
    mut sector_to_be_generated: ResMut<super::SectorToBeGenerated>,
    gameworld_seed: Res<crate::GameworldSeed>,
    mut commands: Commands,
    mut make_tiles_now: ResMut<crate::graphics::testing_mode_tile_map::MakeTilesNow>,
) {
    // If there are events recorded in generate sector then the rest of the function runs.

    if !generate_new_sector_event.is_empty() {
        // This iterates over the vector of tuples of coordinates of the gamesectors to generated.

        for new_sector_list in &sector_to_be_generated.sector_to_be_generated_list {
            // This returns into a tuple the data for a new gamesector.

            let new_sector_basics = generate_map(
                //  A gamesector is generated from its coordinates and a gamewold worldseed. Everything else is procedural.

                new_sector_list.0.clone(),
                new_sector_list.1.clone(),
                &gameworld_seed.gameworld_seed_num
            );

            let new_sector_units= generate_units(
                // Units are generated or loaded based on the initialization type. 
                // Sector coordinates are provided in case the sector already exists and units must be loaded.

                new_sector_list.0.clone(),
                new_sector_list.1.clone(),
                &new_sector_list.2,
            );

            // This creates a new gamesector entity in the ECS and pushes the data from the tuple to it.
            commands.spawn(super::super::GamesectorBundle {
                gamesector_basics: GamesectorBasics {
                    // This stores the data of what tile goes where in the new sector's tile map component.
                    tile_array: new_sector_basics.0,

                    // This stores the sector's biome in the tile map component.
                    sector_biome: new_sector_basics.1,

                    // This stores the sector's coordinates in the gameworld.
                    sector_coordinates: new_sector_basics.2,
                },
                gamesector_units: GamesectorUnits { unit_array: new_sector_units},
            });

            
        make_tiles_now.ready_now.0 = true;
        // This clears the event so sectors aren't generated more than once.

        }

        generate_new_sector_event.clear();
        // This clears the coordinates of the sector to be generated.
        sector_to_be_generated.sector_to_be_generated_list.clear();

    }


}

// This function does the actual procedural generation of the gameworld.

fn generate_map(
    x_coordinate: i32,
    y_coordinate: i32,
    gameworld_seed_num: &u64
) -> ([[(TileType, u8); SECTOR_SIZE as usize]; SECTOR_SIZE as usize], SectorBiome, (i32, i32)) {
    // This initially sets the array to open.
    let gamesector_environment_array = [
        [(TileType::Open, 0); SECTOR_SIZE as usize];
        SECTOR_SIZE as usize
    ];

    // This is just placeholder code
    let sector_biome = SectorBiome::Base;

    // Code to generate gameworld will go here.

    let mut _seeded_prng = rand_chacha::ChaCha8Rng::seed_from_u64(gameworld_seed_num.clone());

    (gamesector_environment_array, sector_biome, (x_coordinate, y_coordinate))
}

fn generate_units (
    _x_coordinate: i32,
    _y_coordinate: i32,
    _initiatization_type: &InitializationType) -> Vec<UnitAttributes> {
    
    let unit_list: Vec<UnitAttributes>  = Vec::new();

    unit_list

    }