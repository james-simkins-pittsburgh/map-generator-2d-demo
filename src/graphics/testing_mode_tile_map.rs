use bevy::prelude::*;
use crate::simulation::TileType;

#[derive(Resource, Default)]
pub struct EnvironmentalTextureHandle {
    handle: Handle<Image>
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
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    env_texture_handle: Res<EnvironmentalTextureHandle>,
) {
    if make_tiles_now.ready_now.0 && make_tiles_now.ready_now.1 {

        let env_texture_atlas = TextureAtlas::from_grid(env_texture_handle.handle.clone(), Vec2::new(96.0, 96.0), 5, 5,Some (Vec2::new(4.0,4.0)), Some(Vec2::new(2.0,2.0)));

        let env_texture_atlas_handle = texture_atlases.add(env_texture_atlas);
        

        for graphics_sector_memory in graphics_memory_sector_query.iter() {
            if graphics_sector_memory.sector_coordinates == (0, 0) {
                #[allow(unused_assignments)]
                let mut tile_graphics_index = 0;

                for index_one in 0..crate::SECTOR_SIZE as usize {
                    for index_two in 0..crate::SECTOR_SIZE as usize {
                        match graphics_sector_memory.tile_array[index_one][index_two] {
                            TileType::Open => {
                                match
                                    graphics_sector_memory.tile_array_variety[index_one][index_two]
                                {
                                    0 => {
                                        tile_graphics_index = 0;
                                    }
                                    1 => {
                                        tile_graphics_index = 1;
                                    }
                                    2 => {
                                        tile_graphics_index = 2;
                                    }
                                    _ => {
                                        tile_graphics_index = 3;
                                    }
                                }
                            }

                            TileType::Elevated => {
                                match
                                    graphics_sector_memory.tile_array_variety[index_one][index_two]
                                {
                                    0 => {
                                        tile_graphics_index = 4;
                                    }
                                    1 => {
                                        tile_graphics_index = 5;
                                    }
                                    2 => {
                                        tile_graphics_index = 6;
                                    }
                                    _ => {
                                        tile_graphics_index = 7;
                                    }
                                }
                            }

                            TileType::Vegetated => {
                                match
                                    graphics_sector_memory.tile_array_variety[index_one][index_two]
                                {
                                    0 => {
                                        tile_graphics_index = 8;
                                    }
                                    1 => {
                                        tile_graphics_index = 9;
                                    }
                                    2 => {
                                        tile_graphics_index = 10;
                                    }
                                    _ => {
                                        tile_graphics_index = 11;
                                    }
                                }
                            }

                            _ => {
                                tile_graphics_index = 0;
                            }
                        }

                        commands.spawn(SpriteSheetBundle {
                            sprite: TextureAtlasSprite::new(tile_graphics_index),
                            texture_atlas: env_texture_atlas_handle.clone(),

                            transform: Transform::from_xyz(
                                (((index_one as f32) - 50.0) * 96.0) as f32,
                                (((index_two as f32) - 50.0) * 96.0) as f32,
                                0.0
                            ),
                            ..default()
                        });
                    }
                }
            }
        }

        make_tiles_now.ready_now.0 = false;
        make_tiles_now.ready_now.1 = false;
    }
}

#[derive(Resource, Default)]
pub struct MakeTilesNow {
    pub ready_now: (bool, bool),
}
