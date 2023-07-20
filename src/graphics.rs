use bevy::prelude::*;
pub(super) mod testing_mode_tile_map;

pub enum DirectionFromCamera {
    LessOrEqual,
    GreaterThan,
}

#[derive(Component)]
pub struct GamesectorGraphicsBasicsMemory {
    pub sector_coordinates: (i32, i32),
    pub sector_biome: crate::simulation::SectorBiome,
    pub tile_array: [
        [(crate::simulation::TileType, u8); crate::SECTOR_SIZE as usize];
        crate::SECTOR_SIZE as usize
    ],
    pub direction_from_camera_x: DirectionFromCamera,
    pub direction_from_camera_y: DirectionFromCamera,
}

#[derive(Bundle)]
pub struct GamesectorGraphicsMemoryBundle {
    pub gamesector_graphics_basic_memory: GamesectorGraphicsBasicsMemory,
}
