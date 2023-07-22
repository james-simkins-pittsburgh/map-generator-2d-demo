use bevy::prelude::*;
use rand::prelude::*;

// These use declaration are just to make the code more readable.

use crate::SECTOR_SIZE;
use crate::simulation::TileType;
use crate::simulation::SectorBiome;
use crate::simulation::SectorBaseType;
use crate::simulation::GamesectorBasics;
use crate::simulation::GamesectorUnits;
use crate::simulation::UnitAttributes;
use super::InitializationType;

// This function generates a new sector and populates it with a procedurally generated environmental map and the other components it needs.

pub fn generate_sector(
    mut generate_new_sector_event: EventReader<crate::GenerateNewSector>,
    mut sector_to_be_generated: ResMut<super::SectorToBeGenerated>,
    gameworld_seed: Res<crate::GameworldSeed>,
    mut commands: Commands,
    mut make_tiles_now: ResMut<crate::graphics::testing_mode_tile_map::MakeTilesNow>
) {
    // If there are events recorded in generate sector then the rest of the function runs.

    if !generate_new_sector_event.is_empty() {
        // This iterates over the vector of tuples of coordinates of the gamesectors to generated.

        for new_sector_list in &sector_to_be_generated.sector_to_be_generated_list {
            // This returns into a tuple the data for a new gamesector.

            let new_sector_basics = generate_map(
                //  A gamesector is generated from its coordinates and a gameworld worldseed. Everything else is procedural.

                new_sector_list.0.clone(),
                new_sector_list.1.clone(),
                &gameworld_seed.gameworld_seed_num
            );

            let new_sector_units = generate_units(
                // Units are generated or loaded based on the initialization type.
                // Sector coordinates are provided in case the sector already exists and units must be loaded.

                new_sector_list.0.clone(),
                new_sector_list.1.clone(),
                &new_sector_list.2
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
                    
                    // This stores the sector's base type in the gameworld.
                    sector_base_type: SectorBaseType::Wild,
                },

                // This copies over the units for the new sector.
                gamesector_units: GamesectorUnits { unit_array: new_sector_units },
            });

            // This is test code saying the graphics can make its world now.
            make_tiles_now.ready_now.0 = true;
        }

        // This clears the event so sectors aren't generated more than once.
        generate_new_sector_event.clear();
        sector_to_be_generated.sector_to_be_generated_list.clear();
    }
}

// This function does the actual procedural generation of the gameworld.

fn generate_map(
    x_coordinate: i32,
    y_coordinate: i32,
    gameworld_seed_num: &u64
) -> ([[TileType; SECTOR_SIZE as usize]; SECTOR_SIZE as usize], SectorBiome, (i32, i32)) {
    // This initially sets the array to open.
    let mut gamesector_environment_array: [[TileType; 101]; 101] = [
        [TileType::Open; SECTOR_SIZE as usize];
        SECTOR_SIZE as usize
    ];

    // This is just placeholder code.
    let sector_biome = SectorBiome::Arid;

    // This block of code deterministically gets a sector seed from the gameworld seed.
    // First we need to convert our coordinates to u64.

    let x_coordinate_to_u64: u64 = if x_coordinate < 0 {
        u64::MAX - (x_coordinate.abs() as u64)
    } else {
        x_coordinate as u64
    };

    let y_coordinate_to_u64: u64 = if y_coordinate < 0 {
        u64::MAX - (y_coordinate.abs() as u64)
    } else {
        y_coordinate as u64
    };

    // Now we add a big number based on the coordinated to the gameworld seed number.
    // If this makes it larger than u64 that's fine since we just wrapping_add it.
    // 412 is for Pittsburgh pride! The other numbers are just large primes.

    let sector_seed_num: u64 = gameworld_seed_num.wrapping_add(
        412 + 3943 * x_coordinate_to_u64 + 4211 * y_coordinate_to_u64
    );

    // This code seeds the random number generator.

    let mut seeded_prng = rand_chacha::ChaCha8Rng::seed_from_u64(sector_seed_num);

    // This draws the patches of tall grass.

    generate_patches(60, 25, TileType::Vegetated, &mut gamesector_environment_array, &mut seeded_prng);

    // This draws the rocky areas.

    generate_patches(12, 250, TileType::Elevated, &mut gamesector_environment_array, &mut seeded_prng);

    (gamesector_environment_array, sector_biome, (x_coordinate, y_coordinate))

}

    // This draws the patches of vegetated and elevated themselves\.

fn generate_patches<R: Rng>(
    number_of_patches: i32,
    size_of_patches: i32,
    tile_type: TileType,
    gamesector_environment_array: &mut[[TileType; 101]; 101],
    seeded_prng: &mut R
) {
    let half_sector_size_minus_five: i32 = ((SECTOR_SIZE - 1) / 2 - 5) as i32;
    let mut x = 0;
    let mut y = 0;
    let mut r = 0;

    for _p in 0..number_of_patches {
      
        x = seeded_prng.gen_range(-half_sector_size_minus_five..half_sector_size_minus_five + 1);
        y = seeded_prng.gen_range(-half_sector_size_minus_five..half_sector_size_minus_five + 1);

        for  _s in 0.. size_of_patches {

            r = seeded_prng.gen_range(0..4);

            match r {

            0 => if x < half_sector_size_minus_five {x=x+1;}
            1 => if x > -half_sector_size_minus_five {x=x-1;}
            2 => if y < half_sector_size_minus_five {y=y+1;}
            _ => if y > -half_sector_size_minus_five {y=y-1;}

            }
        
            gamesector_environment_array [(x + ((SECTOR_SIZE as i32 -1)/2))as usize][(y + ((SECTOR_SIZE as i32-1)/2)) as usize] = tile_type;

        }
    }
}

fn generate_units(
    _x_coordinate: i32,
    _y_coordinate: i32,
    _initialization_type: &InitializationType
) -> Vec<UnitAttributes> {
    let unit_list: Vec<UnitAttributes> = Vec::new();

    unit_list
}
