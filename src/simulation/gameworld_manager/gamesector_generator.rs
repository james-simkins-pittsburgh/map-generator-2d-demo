use bevy::prelude::*;

pub fn generate_base_sector_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut generate_base_sector_map: EventReader<crate::GenerateBaseSectorMap>
) {
    if !generate_base_sector_map.is_empty() {
        let tiledepth: f32 = 0.0;

        for tilex in -50..=50 {
            for tiley in -50..=50 {
                commands.spawn(super::SimTileBundle {
                    sprite_bundle: SpriteBundle {
                        texture: asset_server.load("void1.png"),
                        transform: Transform::from_xyz(
                            (tilex * 96) as f32,
                            (tiley * 96) as f32,
                            tiledepth
                        ),
                        ..default()
                    },
                    sim_tile_location: super::SimTileLocation {
                        xofsector: 0,
                        yofsector: 0,
                        localx: tilex as i16,
                        localy: tiley as i16,
                    },
                });
            }
        }
    }
    generate_base_sector_map.clear();
}
