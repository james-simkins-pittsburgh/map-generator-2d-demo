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
        for mut gamesector_graphics_basics in graphics_memory_query.iter_mut() {
            if
                gamesector_graphics_basics.sector_coordinates ==
                gamesector_basics.sector_coordinates
            {
                gamesector_graphics_basics.sector_biome = gamesector_basics.sector_biome.clone();
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

pub fn variety_generator(
    tile_array_variety: &mut [[(u8, u8); crate::SECTOR_SIZE as usize]; crate::SECTOR_SIZE as usize],
    sector_coordinates: &(i32, i32),
    gameworld_seed: &u64
) {
    let mut seeded_prng = rand_chacha::ChaCha8Rng::seed_from_u64(
        crate::utility::generate_sector_seed_num_from_gameworld_seed_num(
            gameworld_seed.clone(),
            sector_coordinates.0,
            sector_coordinates.1
        )
    );

    for x in 0..crate::SECTOR_SIZE {
        for y in 0..crate::SECTOR_SIZE {
            tile_array_variety[x as usize][y as usize].0 = seeded_prng.gen_range(0..4);
            tile_array_variety[x as usize][y as usize].1 = seeded_prng.gen_range(0..4);
        }
    }
}
