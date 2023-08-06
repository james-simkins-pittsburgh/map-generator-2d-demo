use std::f32::consts::PI;

use bevy::prelude::*;
use super::GUITextureHandle;
use crate::graphics::testing_mode_tile_map::MakeTilesNow;

pub struct HiveboticaWarpButtonPlugin;

impl Plugin for HiveboticaWarpButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display_warp_buttons);
    }
}

pub fn display_warp_buttons(
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
            Some(Vec2::new(2.0, 2.0)),
        );

        let gui_texture_atlas_handle = texture_atlases.add(gui_texture_atlas);

        let mut sprite_transform = Transform::from_xyz(0.0, 0.0, 0.0);

        for n in 1..9 {
            match n {
                1 => {
                    sprite_transform.translation.x = -2400.0;
                    sprite_transform.translation.y = 10000.0;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z((PI / 2.0) * 0.0);
                }

                2 => {
                    sprite_transform.translation.x = 2400.0;
                    sprite_transform.translation.y = 10000.0;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z((PI / 2.0) * 0.0);
                }

                3 => {
                    sprite_transform.translation.x = 10000.0;
                    sprite_transform.translation.y = 2400.0;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z((PI / 2.0) * 1.0);
                }

                4 => {
                    sprite_transform.translation.x = 10000.0;
                    sprite_transform.translation.y = -2400.0;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z((PI / 2.0) * 1.0);
                }

                5 => {
                    sprite_transform.translation.x = 2400.0;
                    sprite_transform.translation.y = -10000.0;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z((PI / 2.0) * 2.0);
                }

                6 => {
                    sprite_transform.translation.x = -2400.0;
                    sprite_transform.translation.y = -10000.0;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z((PI / 2.0) * 2.0);
                }

                7 => {
                    sprite_transform.translation.x = -10000.0;
                    sprite_transform.translation.y = 2400.0;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z((PI / 2.0) * 3.0);
                }

                _ => {
                    sprite_transform.translation.x = -10000.0;
                    sprite_transform.translation.y = -2400.0;
                    sprite_transform.translation.z = 0.0;
                    sprite_transform.rotate_z((PI / 2.0) * 3.0);
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
