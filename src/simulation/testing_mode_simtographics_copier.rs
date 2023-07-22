// This module contains test code so I can test the tilemap stuff.

// What it does is copy the sector information from the simulation to the graphics memory.

use ::bevy::prelude::*;
use rand::prelude::*;


pub fn testing_mode_simtographics_processor_copier(
    sim_sector_query: Query<&crate::simulation::GamesectorBasics>,
    mut graphics_memory_query: Query<&mut crate::graphics::GamesectorGraphicsBasicsMemory>,
    mut make_tiles_now: ResMut<crate::graphics::testing_mode_tile_map::MakeTilesNow>,
    gameworld_seed: Res<crate::GameworldSeed>
) {
    for gamesector_basics in sim_sector_query.iter() {
        if gamesector_basics.sector_coordinates == (0, 0) {
            for mut gamesector_graphics_basics in graphics_memory_query.iter_mut() {
                if gamesector_graphics_basics.sector_coordinates == (0, 0) {
                    gamesector_graphics_basics.sector_biome =
                        gamesector_basics.sector_biome.clone();
                    gamesector_graphics_basics.sector_base_type =
                        gamesector_basics.sector_base_type.clone();
                    gamesector_graphics_basics.sector_coordinates =
                        gamesector_basics.sector_coordinates.clone();
                    gamesector_graphics_basics.tile_array = gamesector_basics.tile_array.clone();
                    variety_generator(
                        &mut gamesector_graphics_basics.tile_array_variety,
                        &gamesector_basics.sector_coordinates,
                        &gameworld_seed.gameworld_seed_num
                    );

                    if make_tiles_now.ready_now.0 {
                        make_tiles_now.ready_now.1 = true;
                    }
                }
            }
        }
    }
}

pub fn variety_generator(
    tile_array_variety: &mut [[u8; crate::SECTOR_SIZE as usize]; crate::SECTOR_SIZE as usize],
    sector_coordinates: &(i32, i32),
    gameworld_seed: &u64
) {
    let x_coordinate_to_u64: u64 = if sector_coordinates.0 < 0 {
        u64::MAX - (sector_coordinates.0.abs() as u64)
    } else {
        sector_coordinates.0 as u64
    };

    let y_coordinate_to_u64: u64 = if sector_coordinates.1 < 0 {
        u64::MAX - (sector_coordinates.1.abs() as u64)
    } else {
        sector_coordinates.1 as u64
    };

    let sector_seed_num: u64 = gameworld_seed.wrapping_add(
        412 + 3943 * x_coordinate_to_u64 + 4211 * y_coordinate_to_u64
    );

    let mut seeded_prng = rand_chacha::ChaCha8Rng::seed_from_u64(sector_seed_num);

    for x in 0..crate::SECTOR_SIZE {
        for y in 0..crate::SECTOR_SIZE {

            tile_array_variety [x as usize][y as usize] = seeded_prng.gen_range (0..4);

        }
    }
}
