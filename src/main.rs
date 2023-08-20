// This hides the command prompt
#![windows_subsystem = "windows"]

use bevy::prelude::*;
use rand::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use rand_chacha::ChaCha8Rng;

pub mod ai;
pub mod graphics;
pub mod gui;
pub mod menu;
pub mod network;
pub mod simulation;
pub mod utility;

// Sector size must be an odd number smaller than 32,768.
const SECTOR_SIZE: u16 = 101;

// If sector size is adjusted smaller than 101 then pan speed and zoom out max has to be changed at the same time.
// Otherwise new graphical sectors might not be loaded before they become visible.
const PAN_TOP_SPEED: f32 = 48.0;
const ZOOM_OUT_MAX: f32 = 8.0;

// Zoom speed is just a matter of preference
const ZOOM_SPEED: f32 = 0.1;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.build().add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
        )
        .add_plugins(gui::HiveboticaGUIPluginGroup)
        .add_plugins(simulation::HiveboticaSimulationPluginGroup)
        // Everything below this in this expression is test code
        .add_systems(Startup, testing_mode_startup)
        .add_systems(Startup, (graphics::testing_mode_tile_map::tile_texture_loader, graphics::testing_mode_tile_map::testing_mode_spawn_tile_map).chain())
        .add_systems(
            Update,
            simulation::testing_mode_simtographics_copier::testing_mode_simtographics_processor_copier
        )
        .add_systems(Update, graphics::testing_mode_tile_map::testing_mode_update_tile_map)
        .add_event::<GenerateNewSector>()
        .init_resource::<graphics::testing_mode_tile_map::EnvironmentalTextureHandle>()
        .init_resource::<graphics::testing_mode_tile_map::TileControlForSectorSwitch>()
        .init_resource::<GameworldSeed>()
        .init_resource::<GameControl>()
        .init_resource::<gui::GUITextureHandle>()

        .run();
}

// This is test code to start the game before I add the menu.
// It will eventually be deleted when a proper menu and loading system are written.

#[derive(Event, Default)]
pub struct GenerateNewSector;

fn testing_mode_startup(
    mut gameworld_seed: ResMut<GameworldSeed>,
    mut game_control: ResMut<GameControl>,
    mut sector_to_be_generated: ResMut<simulation::gameworld_manager::SectorToBeGenerated>,
    mut writer: EventWriter<GenerateNewSector>,
    mut commands: Commands,
    mut tile_control: ResMut<graphics::testing_mode_tile_map::TileControlForSectorSwitch>
) {
    
    tile_control.gamesector_generated= false;
    tile_control.gamesector_copied = false;
    tile_control.gamesector_drawn = false;

    game_control.warp_buttons_created = false;
    let mut seedless_rng = ChaCha8Rng::from_entropy();
    gameworld_seed.gameworld_seed_num = seedless_rng.gen_range(0..u32::MAX) as u64;

    sector_to_be_generated.sector_to_be_generated_list.push((
        0,
        0,
        simulation::gameworld_manager::InitializationType::Player,
    ));

    writer.send(GenerateNewSector);

    commands.spawn(graphics::GamesectorGraphicsMemoryBundle {
        gamesector_graphics_basic_memory: graphics::GamesectorGraphicsBasicsMemory {
            sector_coordinates: (0, 0),
            sector_biome: simulation::SectorBiome::Plains,
            sector_base_type: simulation::SectorBaseType::Wild,
            tile_array: [
                [simulation::TileType::Open; crate::SECTOR_SIZE as usize];
                crate::SECTOR_SIZE as usize
            ],
            tile_array_variety: [
                [(0, 0); crate::SECTOR_SIZE as usize];
                crate::SECTOR_SIZE as usize
            ],
            orientation_to_camera: graphics::DirectionFromCamera::Center,
        },
    });
}

#[derive(Resource, Default)]
pub struct GameworldSeed {
    pub gameworld_seed_num: u64,
}

#[derive(Resource, Default)]
pub struct GameControl {
    pub warp_buttons_created: bool,
}
