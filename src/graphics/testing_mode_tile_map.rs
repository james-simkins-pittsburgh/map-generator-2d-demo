use bevy::prelude::*;
use crate::simulation::TileType;

pub fn testing_mode_tile_map(
    mut make_tiles_now: ResMut<MakeTilesNow>,
    mut commands: Commands,
    graphics_memory_sector_query: Query<&crate::graphics::GamesectorGraphicsBasicsMemory>,
    asset_server: Res<AssetServer>
) {
    if make_tiles_now.ready_now.0 && make_tiles_now.ready_now.1 {
        for graphics_sector_memory in graphics_memory_sector_query.iter() {
            if graphics_sector_memory.sector_coordinates == (0, 0) {
                let mut tile_graphics_path = "void1.png";

                for index_one in 0..crate::SECTOR_SIZE as usize {
                    for index_two in 0..crate::SECTOR_SIZE as usize {
                        match graphics_sector_memory.tile_array[index_one][index_two] {
                            TileType::Open => {
                                match
                                    graphics_sector_memory.tile_array_variety[index_one][index_two]
                                {
                                    0 => {
                                        tile_graphics_path = "plains0.png";
                                    }
                                    1 => {
                                        tile_graphics_path = "plains1.png";
                                    }
                                    2 => {
                                        tile_graphics_path = "plains2.png";
                                    }
                                    _ => {
                                        tile_graphics_path = "plains3.png";
                                    }
                                }
                            }

                            TileType::Elevated => {
                                match
                                    graphics_sector_memory.tile_array_variety[index_one][index_two]
                                {
                                    0 => {
                                        tile_graphics_path = "rock0.png";
                                    }
                                    1 => {
                                        tile_graphics_path = "rock1.png";
                                    }
                                    2 => {
                                        tile_graphics_path = "rock2.png";
                                    }
                                    _ => {
                                        tile_graphics_path = "rock3.png";
                                    }
                                }
                            }

                            TileType::Vegetated => {
                                match
                                    graphics_sector_memory.tile_array_variety[index_one][index_two]
                                {
                                    0 => {
                                        tile_graphics_path = "tallgrass0.png";
                                    }
                                    1 => {
                                        tile_graphics_path = "tallgrass1.png";
                                    }
                                    2 => {
                                        tile_graphics_path = "tallgrass2.png";
                                    }
                                    _ => {
                                        tile_graphics_path = "tallgrass3.png";
                                    }
                                }
                            }

                            _ => {
                                tile_graphics_path = "void0.png";
                            }
                        }

                        commands.spawn(SpriteBundle {
                            texture: asset_server.load(tile_graphics_path),
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
