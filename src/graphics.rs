use bevy::prelude::*;
pub(super) mod tile_map;

#[derive(Copy, Clone, PartialEq)]
pub enum DirectionFromCamera {
    Center,
    Left,
    Right,
    Above,
    Below,
}

#[derive(Component)]
pub struct GamesectorGraphicsBasicsMemory {
    pub sector_coordinates: (i32, i32),
    pub sector_biome: crate::simulation::SectorBiome,
    pub sector_base_type: crate::simulation::SectorBaseType,
    pub tile_array: [
        [crate::simulation::TileType; crate::SECTOR_SIZE as usize];
        crate::SECTOR_SIZE as usize
    ],
    pub tile_array_variety: [[(u8, u8); crate::SECTOR_SIZE as usize]; crate::SECTOR_SIZE as usize],
    pub ruin_rotation: [[u8; crate::SECTOR_SIZE as usize]; crate::SECTOR_SIZE as usize],
    pub orientation_to_camera: DirectionFromCamera,
}

#[derive(Bundle)]
pub struct GamesectorGraphicsMemoryBundle {
    pub gamesector_graphics_basic_memory: GamesectorGraphicsBasicsMemory,
}
