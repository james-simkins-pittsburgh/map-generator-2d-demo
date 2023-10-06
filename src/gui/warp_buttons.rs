use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use super::GUITextureHandle;
use crate::{WarpButtonControl, SECTOR_SIZE};
pub struct HiveboticaWarpButtonPlugin;

impl Plugin for HiveboticaWarpButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_warp_buttons);
        app.add_systems(Update, (check_warp_buttons, update_warp_buttons).chain());
        app.init_resource::<WarpInfo>();
    }
}

// This resource contains the information needed by the camera to warp and by the buttons to get bigger.

#[derive(Resource, Default)]
pub struct WarpInfo {
    pub warping_now: bool,
    pub warp_direction: (WarpSign, WarpSign),
    pub warp_timer: u8,
    pub cursor_over: [bool; 8],
    pub button_pressed: [bool; 8],
}

// This is the direction of warp.

#[derive(Copy, Clone, PartialEq)]
pub enum WarpSign {
    Positive,
    Negative,
    Neutral,
}

impl Default for WarpSign {
    fn default() -> Self {
        WarpSign::Neutral
    }
}

// This allows the 8 warp buttons to be numbered. 

#[derive(Component)]
pub struct WarpButton {
    button_number: u8,
}
// This spawns the warp button entities. 

pub fn spawn_warp_buttons(
    mut warp_button_control: ResMut<WarpButtonControl>,
    gui_texture_handle: Res<GUITextureHandle>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    if warp_button_control.warp_buttons_created == false {

        warp_button_control.warp_buttons_created = true;

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
            sprite_transform.scale.x = 0.85;
            sprite_transform.scale.y = 0.85;

            match n {
                0 => {
                    sprite_transform.translation.x = -96.0 * ((SECTOR_SIZE-1)/4) as f32;
                    sprite_transform.translation.y = ((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0 ;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z((PI / 2.0) * 0.0);
                }

                1 => {
                    sprite_transform.translation.x = 96.0 * ((SECTOR_SIZE-1)/4) as f32;
                    sprite_transform.translation.y = ((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0 ;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z((PI / 2.0) * 0.0);
                }

                2 => {
                    sprite_transform.translation.x = ((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0 ;
                    sprite_transform.translation.y = 96.0 * ((SECTOR_SIZE-1)/4) as f32;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z(-(PI / 2.0) * 1.0);
                }

                3 => {
                    sprite_transform.translation.x = ((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0 ;
                    sprite_transform.translation.y = -96.0 * ((SECTOR_SIZE-1)/4) as f32;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z(-(PI / 2.0) * 1.0);
                }

                4 => {
                    sprite_transform.translation.x = 96.0 * ((SECTOR_SIZE-1)/4) as f32;
                    sprite_transform.translation.y = -(((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0) ;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z(-(PI / 2.0) * 2.0);
                }

                5 => {
                    sprite_transform.translation.x = -96.0 * ((SECTOR_SIZE-1)/4) as f32;
                    sprite_transform.translation.y = -(((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0) ;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z(-(PI / 2.0) * 2.0);
                }

                6 => {
                    sprite_transform.translation.x = -(((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0) ;
                    sprite_transform.translation.y = -96.0 * ((SECTOR_SIZE-1)/4) as f32;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z(-(PI / 2.0) * 3.0);
                }

                _ => {
                    sprite_transform.translation.x = -(((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0) ;
                    sprite_transform.translation.y = 96.0 * ((SECTOR_SIZE-1)/4) as f32;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z(-(PI / 2.0) * 3.0);
                }
            }

            commands.spawn((
                SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(0),
                    texture_atlas: gui_texture_atlas_handle.clone(),
                    transform: sprite_transform,
                    ..default()
                },

                WarpButton { button_number: n },
            ));
        }
    }

}

// This checks to see if warp buttons have been pressed and initiates warp.

pub fn check_warp_buttons(
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    main_camera_query: Query<(&Camera, &GlobalTransform), With<super::camera::MainCamera>>,
    mut warp_information: ResMut<WarpInfo>,
    mouse_buttons: Res<Input<MouseButton>>
) {
    let (camera, camera_transform) = main_camera_query.single();

    let display_window = primary_window_query.single();

    if let Some(cursor_viewport_position) = display_window.cursor_position() {
        if
            let Some(cursor_world_position) = camera.viewport_to_world_2d(
                camera_transform,
                cursor_viewport_position
            )
        {
            let x = cursor_world_position.x;
            let y = cursor_world_position.y;

            for button_number in 0..8 {

                if check_for_cursor_over_button(x, y, button_number) {
                    warp_information.cursor_over[button_number as usize] = true;

                    if mouse_buttons.pressed(MouseButton::Left) {
                        warp_information.button_pressed[button_number as usize] = true;
                    } else {
                        warp_information.button_pressed[button_number as usize] = false;
                    }
                } else {
                    warp_information.cursor_over[button_number as usize] = false;
                    warp_information.button_pressed[button_number as usize] = false;
                }
            }
        }
    }
}

// This makes the warp buttons get slightly larger if the cursor is over them.

fn check_for_cursor_over_button(cursor_x: f32, cursor_y: f32, button_number: u8) -> bool {
    let square_bl_x: f32;
    let square_bl_y: f32;
    let triangle_bc_x: f32;
    let triangle_bc_y: f32;
    let triangle_direction: i32;

    match button_number {
        0 => {
            square_bl_x = -96.0 * ((SECTOR_SIZE-1)/4) as f32 - 96.0;
            square_bl_y = (((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0) - 192.0;
            triangle_bc_x = -96.0 * ((SECTOR_SIZE-1)/4) as f32;
            triangle_bc_y = 5184.0;
            triangle_direction = 0;
        }
        1 => {
            square_bl_x = 96.0 * ((SECTOR_SIZE-1)/4) as f32 - 96.0;
            square_bl_y = (((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0)  - 192.0;
            triangle_bc_x = 96.0 * ((SECTOR_SIZE-1)/4) as f32;
            triangle_bc_y = ((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0;
            triangle_direction = 0;
        }

        2 => {
            square_bl_x = (((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0)  - 192.0;
            square_bl_y = 96.0 * ((SECTOR_SIZE-1)/4) as f32 - 96.0;
            triangle_bc_x = ((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0 ;
            triangle_bc_y = 96.0 * ((SECTOR_SIZE-1)/4) as f32;
            triangle_direction = 1;
        }
        3 => {
            square_bl_x = (((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0)  - 192.0;
            square_bl_y = -96.0 * ((SECTOR_SIZE-1)/4) as f32 - 96.0;
            triangle_bc_x = ((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0;
            triangle_bc_y = -96.0 * ((SECTOR_SIZE-1)/4) as f32;
            triangle_direction = 1;
        }

        4 => {
            square_bl_x = 96.0 * ((SECTOR_SIZE-1)/4) as f32 - 96.0;
            square_bl_y = -(((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0) ;
            triangle_bc_x = 96.0 * ((SECTOR_SIZE-1)/4) as f32;
            triangle_bc_y = -(((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0) ;
            triangle_direction = 2;
        }

        5 => {
            square_bl_x = -96.0 * ((SECTOR_SIZE-1)/4) as f32 - 96.0;
            square_bl_y = ((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0 ;
            triangle_bc_x = -96.0 * ((SECTOR_SIZE-1)/4) as f32;
            triangle_bc_y = -(((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0) ;
            triangle_direction = 2;
        }

        6 => {
            square_bl_x = -(((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0) ;
            square_bl_y = -96.0 * ((SECTOR_SIZE-1)/4) as f32 - 96.0;
            triangle_bc_x = -(((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0) ;
            triangle_bc_y = -96.0 * ((SECTOR_SIZE-1)/4) as f32;
            triangle_direction = 3;
        }

        _ => {
            square_bl_x = -(((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0) ;
            square_bl_y = 96.0 * ((SECTOR_SIZE-1)/4) as f32 - 96.0;
            triangle_bc_x = -(((SECTOR_SIZE-1)/2) as f32 * 96.0 + 384.0) ;
            triangle_bc_y = 96.0 * ((SECTOR_SIZE-1)/4) as f32;
            triangle_direction = 3;
        }
    }

    if
        cursor_x > square_bl_x &&
        cursor_x < square_bl_x + 192.0 &&
        cursor_y > square_bl_y &&
        cursor_y < square_bl_y + 192.0
    {
        return true;
    }

    match triangle_direction {
        0 => {
            if
                crate::utility::point_in_triangle(
                    cursor_x,
                    cursor_y,
                    triangle_bc_x - 192.0,
                    triangle_bc_y,
                    triangle_bc_x + 192.0,
                    triangle_bc_y,
                    triangle_bc_x,
                    triangle_bc_y + 192.0
                )
            {
                return true;
            }
        }
        1 => {
            if
                crate::utility::point_in_triangle(
                    cursor_x,
                    cursor_y,
                    triangle_bc_x,
                    triangle_bc_y + 192.0,
                    triangle_bc_x,
                    triangle_bc_y - 192.0,
                    triangle_bc_x + 192.0,
                    triangle_bc_y
                )
            {
                return true;
            }
        }

        2 => {
            if
                crate::utility::point_in_triangle(
                    cursor_x,
                    cursor_y,
                    triangle_bc_x - 192.0,
                    triangle_bc_y,
                    triangle_bc_x + 192.0,
                    triangle_bc_y,
                    triangle_bc_x,
                    triangle_bc_y - 192.0
                )
            {
                return true;
            }
        }
        _ => {
            if
                crate::utility::point_in_triangle(
                    cursor_x,
                    cursor_y,
                    triangle_bc_x,
                    triangle_bc_y + 192.0,
                    triangle_bc_x,
                    triangle_bc_y - 192.0,
                    triangle_bc_x - 192.0,
                    triangle_bc_y
                )
            {
                return true;
            }
        }
    }

    false
}

pub fn update_warp_buttons(
    mut warp_button_query: Query<(&mut WarpButton, &mut Transform)>,
    warp_info: Res<WarpInfo>
) {
    for (warp_button, mut transform) in warp_button_query.iter_mut() {
        if warp_info.cursor_over[warp_button.button_number as usize] {
            transform.scale.x = 1.0;
            transform.scale.y = 1.0;
        } else {
            transform.scale.x = 0.85;
            transform.scale.y = 0.85;
        }
    }
}
