use bevy::prelude::*;
use bevy::app::PluginGroupBuilder;

use crate::SECTOR_SIZE;

pub(super) mod gameworld_manager;
pub(super) mod gamesector_simulator;
pub(super) mod travelator_simulator;
pub struct HiveboticaSimulationPluginGroup;

impl PluginGroup for HiveboticaSimulationPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(gameworld_manager::HiveboticaGameworldManagerPlugin)
    }
}

#[derive(Copy, Clone)]
pub enum TileType {
    Open,
    Vegetated,
    Elevated,
}

enum SectorBiome {
    Base,
    Plains,
    Desert,
    Frozen,
    Ruins,
}

#[derive(Component)]
    struct GamesectorTileMap {
    sector_coordinates: (i32, i32),
    sector_biome: SectorBiome,
    tile_array: [[TileType; SECTOR_SIZE as usize]; SECTOR_SIZE as usize],
}
#[derive(Bundle)]
pub struct GamesectorBundle {
    gamesector_tile_map: GamesectorTileMap,
}