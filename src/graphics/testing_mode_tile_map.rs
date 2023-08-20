use bevy::prelude::*;
use crate::SECTOR_SIZE;
use crate::simulation::TileType;
use crate::simulation::SectorBiome;

#[derive(Resource, Default)]
pub struct EnvironmentalTextureHandle {
    handle: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct TileControlForSectorSwitch {
    pub gamesector_generated: bool,
    pub gamesector_copied: bool,
    pub gamesector_drawn: bool,
}

#[derive(Component)]
pub struct TileIndex {
    pub x: u16,
    pub y: u16,
}

pub fn tile_texture_loader(
    asset_server: Res<AssetServer>,
    mut env_texture_handle: ResMut<EnvironmentalTextureHandle>
) {
    env_texture_handle.handle = asset_server.load("environment.png");
}

pub fn testing_mode_spawn_tile_map(
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    env_texture_handle: Res<EnvironmentalTextureHandle>,
    mut commands: Commands
) {
    let env_texture_atlas = TextureAtlas::from_grid(
        env_texture_handle.handle.clone(),
        Vec2::new(96.0, 96.0),
        10,
        10,
        Some(Vec2::new(6.0, 6.0)),
        Some(Vec2::new(3.0, 3.0))
    );

    let env_texture_atlas_handle = texture_atlases.add(env_texture_atlas);

    let mut sprite_transform: Transform;

    for x_index in 0..SECTOR_SIZE {
        for y_index in 0..SECTOR_SIZE {
            sprite_transform = Transform::from_xyz(
                (((x_index as f32) - 50.0) * 96.0) as f32,
                (((y_index as f32) - 50.0) * 96.0) as f32,
                0.0
            );

            commands.spawn((
                SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(0),
                    texture_atlas: env_texture_atlas_handle.clone(),
                    transform: sprite_transform,
                    ..default()
                },
                TileIndex { x: x_index, y: y_index },
            ));
        }
    }
}

pub fn testing_mode_update_tile_map(
    mut tile_control: ResMut<TileControlForSectorSwitch>,
    graphics_memory_sector_query: Query<&crate::graphics::GamesectorGraphicsBasicsMemory>,
    mut tile_query: Query <(&mut TileIndex, &mut TextureAtlasSprite, &mut Transform)>
) {
    if
        tile_control.gamesector_generated &&
        tile_control.gamesector_copied &&
        !tile_control.gamesector_drawn
    {
        let mut sprite_transform: Transform;
        let mut tile_graphics_index: u16;

        for graphics_sector_memory in graphics_memory_sector_query.iter() {
            if
                graphics_sector_memory.orientation_to_camera ==
                crate::graphics::DirectionFromCamera::Center
            {
                for mut tile_properties in tile_query.iter_mut() {

                    match graphics_sector_memory.sector_biome {
                        SectorBiome::Plains => {
                            tile_graphics_index = 0;
                        }
                        SectorBiome::Desert => {
                            tile_graphics_index = 12;
                        }
                        SectorBiome::Tundra => {
                            tile_graphics_index = 24;
                        }
                        SectorBiome::Alpine => {
                            tile_graphics_index = 36;
                        }
                    }

                    match graphics_sector_memory.tile_array[tile_properties.0.x as usize][tile_properties.0.y as usize] {
                        TileType::Vegetated => {
                            tile_graphics_index += 4;
                        }
                        TileType::Elevated => {
                            tile_graphics_index += 8;
                        }
                        _ => {}
                    }

                    match graphics_sector_memory.tile_array_variety[tile_properties.0.x as usize][tile_properties.0.y as usize].0 {

                        1 => {
                            tile_graphics_index += 1;
                        }
                        2 => {
                            tile_graphics_index += 2;
                        }
                        3 => {
                            tile_graphics_index += 3;
                        }
                        _ => {}

                    }

                    tile_properties.1.index = tile_graphics_index as usize;
                    
                    
                    tile_properties.2.rotate_z(
                        1.5708 *
                            (
                                graphics_sector_memory.tile_array_variety[tile_properties.0.x as usize]
                                    [tile_properties.0.y as usize].1 as f32
                            )
                    );

                }
            }

            tile_control.gamesector_drawn = true;
        }
    }
}
