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
            if graphics_sector_memory.sector_coordinates == (0,0) {

                let mut tile_graphics_path= "void1";

                for index_one in 0..crate::SECTOR_SIZE as usize {
                    for index_two in 0..crate::SECTOR_SIZE as usize {
                       
                       if graphics_sector_memory.tile_array[index_one][index_two].0 == TileType::Open {

                        tile_graphics_path = "plains1"


                       } else if graphics_sector_memory.tile_array[index_one][index_two].0 == TileType::Elevated{

                        tile_graphics_path = "rock1"

                       } else {

                        tile_graphics_path = "void1"

                       }

                        commands.spawn(SpriteBundle {
                            texture: asset_server.load(tile_graphics_path),
                            transform: Transform::from_xyz(
                                ((index_one-50)* 96) as f32,
                                ((index_two-50)* 96) as f32,
                                0.0
                            ),
                            ..default()
                        });







                    }
                }
            }
        }
    }

    make_tiles_now.ready_now.0 = false;
    make_tiles_now.ready_now.1 = false;
}

#[derive(Resource, Default)]
pub struct MakeTilesNow {
    pub ready_now: (bool, bool),
}
