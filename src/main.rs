use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(graphics_setup)
        .add_startup_system(generate_base_sector_map)
        .run();
}

fn graphics_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn generate_base_sector_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tiledepth: f32 = 0.0;

    for tilex in -50..=50 {
        for tiley in -50..=50 {
            commands.spawn(SimTileBundle {
                sprite_bundle: SpriteBundle {
                    texture: asset_server.load("void1.png"),
                    transform: Transform::from_xyz((tilex * 96) as f32, (tiley * 96) as f32, tiledepth),
                    ..default()
                },
                sim_tile_location: SimTileLocation { 
                    xofsector: (0), 
                    yofsector: (0), 
                    localx: (tilex as i16), 
                    localy: (tiley as i16), 
                }
            });
        }
    }
}


#[derive(Component)]
pub struct SimTileLocation {
    pub xofsector: i32,
    pub yofsector: i32,
    pub localx: i16,
    pub localy: i16,
}

#[derive(Bundle)]
pub struct SimTileBundle {
    sim_tile_location: SimTileLocation,
    sprite_bundle: SpriteBundle,
}
