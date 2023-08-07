use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::PrimaryWindow;
use super::GUITextureHandle;
use crate::graphics::testing_mode_tile_map::MakeTilesNow;

pub struct HiveboticaWarpButtonPlugin;

impl Plugin for HiveboticaWarpButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_warp_buttons);
        app.init_resource::<WarpMode>();
    }
}

#[derive(Resource, Default)]
pub struct WarpInfo {

    pub warping_now: bool,
    pub current_warp_direction: (WarpSign, WarpSign),
    pub warp_timer: u8,
    pub warped_once: bool,
    pub first_warp_direction: (WarpSign, WarpSign),
    pub warped_twice: bool,
    pub warp_button: (bool,bool,bool,bool,bool,bool,bool,bool),
    
}

pub enum WarpSign {

    Positive,
    Negative 

}

impl Default for WarpSign {

    fn default() -> Self {
        WarpSign::Negative
    }
}
    


pub fn spawn_warp_buttons(
    make_tiles_now: Res<MakeTilesNow>,
    gui_texture_handle: Res<GUITextureHandle>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    if make_tiles_now.ready_now.0 && make_tiles_now.ready_now.1 {
        let gui_texture_atlas = TextureAtlas::from_grid(
            gui_texture_handle.handle.clone(),
            Vec2::new(384.0, 384.0),
            1,
            1,
            Some(Vec2::new(4.0, 4.0)),
            Some(Vec2::new(2.0, 2.0))
        );

        let gui_texture_atlas_handle = texture_atlases.add(gui_texture_atlas);

        let mut sprite_transform;

        for n in 0..8 {
            sprite_transform = Transform::from_xyz(0.0, 0.0, 0.0);

            match n {
                0 => {
                    sprite_transform.translation.x = -2400.0;
                    sprite_transform.translation.y = 5184.0;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z((PI / 2.0) * 0.0);
                }

                1 => {
                    sprite_transform.translation.x = 2400.0;
                    sprite_transform.translation.y = 5184.0;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z((PI / 2.0) * 0.0);
                }

                2 => {
                    sprite_transform.translation.x = 5184.0;
                    sprite_transform.translation.y = 2400.0;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z(-(PI / 2.0) * 1.0);
                }

                3 => {
                    sprite_transform.translation.x = 5184.0;
                    sprite_transform.translation.y = -2400.0;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z(-(PI / 2.0) * 1.0);
                }

                4 => {
                    sprite_transform.translation.x = 2400.0;
                    sprite_transform.translation.y = -5184.0;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z(-(PI / 2.0) * 2.0);
                }

                5 => {
                    sprite_transform.translation.x = -2400.0;
                    sprite_transform.translation.y = -5184.0;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z(-(PI / 2.0) * 2.0);
                }

                6 => {
                    sprite_transform.translation.x = -5184.0;
                    sprite_transform.translation.y = 2400.0;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z(-(PI / 2.0) * 3.0);
                }

                7 => {
                    sprite_transform.translation.x = -5184.0;
                    sprite_transform.translation.y = -2400.0;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z(-(PI / 2.0) * 3.0);
                }
            }

            commands.spawn(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(0),
                texture_atlas: gui_texture_atlas_handle.clone(),
                transform: sprite_transform,
                ..default()
            });
        }
    }
}

pub fn check_warp_buttons(
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    main_camera_query: Query<(&Camera, &GlobalTransform), With<super::camera::MainCamera>>
) {
    let (camera, camera_transform) = main_camera_query.single();

    let display_window = primary_window_query.single();

    if let Some(cursor_viewport_position) = display_window.cursor_position() {
        if
            let Some(cursor_world_position) = camera.viewport_to_world_2d (
                camera_transform,
                cursor_viewport_position
            )
        {

            let x = cursor_world_position.x;
            let y = cursor_world_position.y;





        }
    }
}

fn check_for_button_contact (cursor_x:f32, cursor_y: f32, button_number: u8) -> bool {

true 

}