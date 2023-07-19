use bevy::prelude::*;

pub mod ai;
pub mod graphics;
pub mod gui;
pub mod menu;
pub mod network;
pub mod simulation;

// Sector size must be an odd number larger than 100 and smaller than 32,768.
const SECTOR_SIZE: u16 = 101;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(graphics::HiveboticaGraphicsPluginGroup)
        .add_plugins(simulation::HiveboticaSimulationPluginGroup)
        // Everything below this in this expression is test code
        .add_systems(Startup, testing_mode_startup)
        .add_systems(Update, simulation::testing_mode_simtographics_copier::testing_mode_simtographics_copier)
        .add_systems(Update, graphics::testing_mode_tile_map::testing_mode_tile_map)
        .add_event::<GenerateNewSector>()
        .init_resource::<graphics::testing_mode_tile_map::MakeTilesNow>()
        .init_resource::<GameworldSeed>()
        .run();
}

// This is test code to start the game before I add the menu. It will eventually be deleted.

#[derive(Event, Default)]
pub struct GenerateNewSector;

fn testing_mode_startup(
    mut gameworld_seed: ResMut<GameworldSeed>,
    mut sector_to_be_generated: ResMut<simulation::gameworld_manager::SectorToBeGenerated>,
    mut writer: EventWriter<GenerateNewSector>,
    mut commands: Commands,
    mut make_tiles_now: ResMut<graphics::testing_mode_tile_map::MakeTilesNow>,

    
) {
    make_tiles_now.ready_now = (false, false);
    gameworld_seed.gameworld_seed_num = 0 as u64;
    sector_to_be_generated.sector_to_be_generated_list.push((0, 0, simulation::gameworld_manager::InitializationType::Player));
    writer.send(GenerateNewSector);
    commands.spawn(graphics::GamesectorGraphicsMemoryBundle {

        gamesector_graphics_basic_memory: graphics::GamesectorGraphicsBasicsMemory {

            sector_coordinates: (0,0),
            sector_biome: simulation::SectorBiome::Plains,
            tile_array: [[(simulation::TileType::Vegetated, 0); crate::SECTOR_SIZE as usize]; crate::SECTOR_SIZE as usize],
            direction_from_camera_x: graphics::DirectionFromCamera::LessOrEqual,
            direction_from_camera_y: graphics::DirectionFromCamera::LessOrEqual

        }
    });
}

#[derive(Resource, Default)]
pub struct GameworldSeed {
    pub gameworld_seed_num: u64,
}
