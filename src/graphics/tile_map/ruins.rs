use bevy::prelude::*;
use crate::SECTOR_SIZE;
use crate::simulation::TileType;
use bevy::render::view::visibility::Visibility::Hidden;
use bevy::render::view::visibility::Visibility::Inherited;

pub fn spawn_ruins(env_texture_handle: Handle<TextureAtlas>, commands: &mut Commands) {
    let mut x_location: i32;
    let mut y_location: i32;

    for column_x in 0..12 {
        for row_y in 0..53 {
            match column_x {
                0 => {
                    x_location = -30;
                }
                1 => {
                    x_location = -29;
                }
                2 => {
                    x_location = -25;
                }
                3 => {
                    x_location = -24;
                }
                4 => {
                    x_location = -21;
                }
                5 => {
                    x_location = -20;
                }
                6 => {
                    x_location = 20;
                }
                7 => {
                    x_location = 21;
                }
                8 => {
                    x_location = 24;
                }
                9 => {
                    x_location = 25;
                }
                10 => {
                    x_location = 29;
                }
                _ => {
                    x_location = 30;
                }
            }

            match row_y {
                0..=13 => {
                    y_location = -30 + row_y;
                }
                14..=36 => {
                    y_location = -30 + row_y + 4;
                }
                _ => {
                    y_location = -30 + row_y + 8;
                }
            }

            if
                (((SECTOR_SIZE - 1) / 2) as i32) + x_location >= 0 &&
                (((SECTOR_SIZE - 1) / 2) as i32) + y_location >= 0
            {
                spawn_ruins_helper(
                    env_texture_handle.clone(),
                    commands,
                    ((((SECTOR_SIZE - 1) / 2) as i32) + x_location) as u16,
                    ((((SECTOR_SIZE - 1) / 2) as i32) + y_location) as u16
                );
            }
        }
    }

    // This is where I left off. Still need to spawn horizontal ruins without intersections. 
    
}

fn spawn_ruins_helper(
    env_texture_handle: Handle<TextureAtlas>,
    commands: &mut Commands,
    x_index: u16,
    y_index: u16
) {
    let sprite_transform = Transform::from_xyz(
        (((x_index as f32) - (((SECTOR_SIZE - 1) / 2) as f32)) * 96.0) as f32,
        (((y_index as f32) - (((SECTOR_SIZE - 1) / 2) as f32)) * 96.0) as f32,
        0.01
    );

    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: env_texture_handle.clone(),
            transform: sprite_transform,
            visibility: Hidden,
            ..default()
        },
        super::TileIndex { x: x_index, y: y_index },
    ));
}
