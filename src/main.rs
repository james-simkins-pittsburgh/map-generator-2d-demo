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
        .add_event::<GenerateSectorMap>()
        .run();
}

// This is test code to start the game before I add the menu. It will eventually be deleted.

#[derive(Event, Default)]
pub struct GenerateSectorMap;

fn testing_mode_startup(
    mut gameworld_seed: ResMut<GameworldSeed>,
    mut sector_to_be_generated: ResMut<simulation::gameworld_manager::SectorToBeGenerated>,
    mut writer: EventWriter<GenerateSectorMap>,
    
) {
    gameworld_seed.gameworld_seed_array = [0; 12];
    sector_to_be_generated.sector_to_be_generated_coordinates.push((0, 0));
    writer.send(GenerateSectorMap);
}

#[derive(Resource, Default)]
pub struct GameworldSeed {
    pub gameworld_seed_array: [u8; 12],
}
