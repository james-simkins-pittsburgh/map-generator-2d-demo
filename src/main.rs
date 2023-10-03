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

// Sector size must be an odd number greater than 101 and smaller than 32,768.
const SECTOR_SIZE: u16 = 201;

// If sector size is adjusted smaller than 101 then pan speed and zoom out max has to be changed at the same time.
// Otherwise new graphical sectors might not be loaded before they become visible.
const PAN_TOP_SPEED: f32 = 48.0;
const ZOOM_OUT_MAX: f32 = 8.0;
const WARP_LENGTH: u8 = 20;

// Zoom speed is just a matter of preference
const ZOOM_SPEED: f32 = 0.1;


// This is the Bevy app that organizes and runs everything from the game engine to the logic of Hivebotica.
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.build().add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
        )
        .add_plugins(gui::HiveboticaGUIPluginGroup)
        .add_plugins(simulation::HiveboticaSimulationPluginGroup)
        .add_systems(
            Startup,
            (graphics::tile_map::tile_texture_loader, graphics::tile_map::spawn_tile_map).chain()
        )
        .add_systems(Update, graphics::tile_map::update_tile_map)
        .init_resource::<graphics::tile_map::EnvironmentalTextureHandle>()
        .init_resource::<graphics::tile_map::TileControlForSectorSwitch>()
        .init_resource::<GameworldSeed>()
        .init_resource::<WarpButtonControl>()
        .init_resource::<gui::GUITextureHandle>()
        // v Everything between these comments are test code v
        .add_systems(Startup, testing_mode_startup)
        .add_systems(
            Update,
            simulation::testing_mode_simtographics_copier::testing_mode_simtographics_processor_copier
        )

        .add_event::<GenerateNewSector>()
        // ^ Everything between these comments are test code ^
        
        .run();
}

// This is the seed number for the gameworld. Eventually, it will represent a 6 letter word 
// with a simple A = 1, B = 2, etc cipher. An additional 3 digits will be added for the millisecond the 
// world was created for tournament worlds. 

#[derive(Resource, Default)]
pub struct GameworldSeed {
    pub gameworld_seed_num: u64,
}

#[derive(Resource, Default)]
pub struct WarpButtonControl {
    pub warp_buttons_created: bool,
}

// The rest of this module is test code to start the game before I add the menu.
// It will eventually be deleted when a proper menu and loading system are written.

#[derive(Event, Default)]
pub struct GenerateNewSector;

fn testing_mode_startup(
    mut gameworld_seed: ResMut<GameworldSeed>,
    mut game_control: ResMut<WarpButtonControl>,
    mut sector_to_be_generated: ResMut<simulation::gameworld_manager::SectorToBeGenerated>,
    mut writer: EventWriter<GenerateNewSector>,
    mut commands: Commands,
    mut tile_control: ResMut<graphics::tile_map::TileControlForSectorSwitch>
) {
    tile_control.gamesector_generated = false;
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
