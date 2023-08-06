use bevy::prelude::*;
use crate::SECTOR_SIZE;
use crate::simulation::TileType;
use crate::simulation::SectorBiome;

#[derive(Resource, Default)]
pub struct EnvironmentalTextureHandle {
    handle: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct MakeTilesNow {
    pub ready_now: (bool, bool),
}


pub fn tile_texture_loader(
    asset_server: Res<AssetServer>,
    mut env_texture_handle: ResMut<EnvironmentalTextureHandle>
) {
    env_texture_handle.handle = asset_server.load("environment.png");
}

pub fn testing_mode_tile_map(
    mut make_tiles_now: ResMut<MakeTilesNow>,
    mut commands: Commands,
    graphics_memory_sector_query: Query<&crate::graphics::GamesectorGraphicsBasicsMemory>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    env_texture_handle: Res<EnvironmentalTextureHandle>
) {
    if make_tiles_now.ready_now.0 && make_tiles_now.ready_now.1 {
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

        for graphics_sector_memory in graphics_memory_sector_query.iter() {
            #[allow(unused_assignments)]
            let mut tile_graphics_index = 0;

            for index_one in 0..crate::SECTOR_SIZE as usize {
                for index_two in 0..crate::SECTOR_SIZE as usize {
                    match graphics_sector_memory.sector_biome {
                        SectorBiome::Plains => {tile_graphics_index = 0;},
                        SectorBiome::Desert => {tile_graphics_index = 12;},
                        SectorBiome::Tundra => {tile_graphics_index = 24;},
                        SectorBiome::Alpine => {tile_graphics_index = 36;},
                    }

                    match graphics_sector_memory.tile_array [index_one] [index_two] {
                        
                        TileType::Vegetated => {tile_graphics_index += 4;},
                        TileType::Elevated => {tile_graphics_index += 8;},
                        _ => {},

                    }

                    
                    match graphics_sector_memory.tile_array_variety [index_one] [index_two].0 {
                        
                        1 => {tile_graphics_index += 1;},
                        2 => {tile_graphics_index += 2;},
                        3 => {tile_graphics_index += 3;},
                        _ => {},

                    }

        
                    sprite_transform = Transform::from_xyz(
                        ((((index_one as f32) - 50.0) * 96.0) as f32) + (1.5*96.0*(SECTOR_SIZE as f32) * (graphics_sector_memory.sector_coordinates.0 as f32)),
                        ((((index_two as f32) - 50.0) * 96.0) as f32) + (1.5*96.0*(SECTOR_SIZE as f32) * (graphics_sector_memory.sector_coordinates.1 as f32)),
                        0.0
                    );

                    sprite_transform.rotate_z(
                        1.5708 *
                            (
                                graphics_sector_memory.tile_array_variety[index_one]
                                    [index_two].1 as f32
                            )
                    );

                    commands.spawn(SpriteSheetBundle {
                        sprite: TextureAtlasSprite::new(tile_graphics_index),
                        texture_atlas: env_texture_atlas_handle.clone(),
                        transform: sprite_transform,
                        ..default()
                    });
                }
            }
        }

        make_tiles_now.ready_now.0 = false;
        make_tiles_now.ready_now.1 = false;
    }
}
