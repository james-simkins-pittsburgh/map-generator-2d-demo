use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(graphics::graphics_setup)
        .add_startup_system(graphics::generate_base_sector_map)
        .run();
}

pub mod ai;
pub mod graphics;
pub mod gui;
pub mod menu;
pub mod network;
pub mod simulation;