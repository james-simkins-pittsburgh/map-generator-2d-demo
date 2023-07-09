use bevy::prelude::*;

pub mod ai;
pub mod graphics;
pub mod gui;
pub mod menu;
pub mod network;
pub mod simulation;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(graphics::camera::camera_setup)
    .add_startup_system(simulation::gameworld_manager::gamesector_generator::generate_base_sector_map)
    .run ();
}
