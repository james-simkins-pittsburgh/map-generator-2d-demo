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
enum TileType {
    Standard,
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
pub struct GamesectorTileMap {
    pub sector_coordinates: (u16, u16),
    pub sector_biome: SectorBiome,
    pub tile_array: [[TileType; SECTOR_SIZE as usize]; SECTOR_SIZE as usize],
}
