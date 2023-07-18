// This module contains test code so I can test the tilemap stuff.

use ::bevy::prelude::*;

pub fn testing_mode_simtographics_copier(
    sim_sector_query: Query<&crate::simulation::GamesectorBasics>,
    mut graphics_memory_query: Query<&mut crate::graphics::GamesectorGraphicsBasicsMemory>,
    mut make_tiles_now: ResMut<crate::graphics::testing_mode_tile_map::MakeTilesNow>,
) {
    for gamesector_basics in sim_sector_query.iter() {
        if gamesector_basics.sector_coordinates == (0, 0) {
            for mut gamesector_graphics_basics in graphics_memory_query.iter_mut() {
                if gamesector_graphics_basics.sector_coordinates == (0,0) {

                    gamesector_graphics_basics.sector_biome = gamesector_basics.sector_biome.clone();
                    gamesector_graphics_basics.sector_coordinates = gamesector_basics.sector_coordinates.clone();
                    gamesector_graphics_basics.tile_array = gamesector_graphics_basics.tile_array.clone();

                    if make_tiles_now.ready_now.0 {
                    make_tiles_now.ready_now.1 = true;
                    }

                }
            
            }
        }
    }
}
