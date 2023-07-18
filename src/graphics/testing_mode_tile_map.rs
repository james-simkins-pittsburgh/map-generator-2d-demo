use bevy::prelude::*;

pub fn testing_mode_tile_map(

    mut make_tiles_now: ResMut<MakeTilesNow>,

) {

    if make_tiles_now.ready_now.0 && make_tiles_now.ready_now.1 {

        
        // Code here to iterate over two dimensional array spawning tiles.

        make_tiles_now.ready_now.0 = false;
        make_tiles_now.ready_now.1 = false;

    }



}

#[derive(Resource, Default)]
pub struct MakeTilesNow {
    pub ready_now: (bool,bool)
}
