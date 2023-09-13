use bevy::prelude::*;
use crate::SECTOR_SIZE;
use crate::simulation::TileType;
use bevy::render::view::visibility::Visibility::Hidden;
use bevy::render::view::visibility::Visibility::Inherited;

#[derive(Component)]
pub struct RuinTile {}

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
                    x_location = -26;
                }
                3 => {
                    x_location = -25;
                }
                4 => {
                    x_location = -22;
                }
                5 => {
                    x_location = -21;
                }
                6 => {
                    x_location = 21;
                }
                7 => {
                    x_location = 22;
                }
                8 => {
                    x_location = 25;
                }
                9 => {
                    x_location = 26;
                }
                10 => {
                    x_location = 29;
                }
                _ => {
                    x_location = 30;
                }
            }

            match row_y {
                0..=15 => {
                    y_location = -30 + row_y;
                }
                16..=36 => {
                    y_location = -30 + row_y + 4;
                }
                _ => {
                    y_location = -30 + row_y + 8;
                }
            }

            if
                (((SECTOR_SIZE - 1) / 2) as i32) + x_location >= 0 &&
                (((SECTOR_SIZE - 1) / 2) as i32) + y_location >= 0 &&
                y_location.abs() <= x_location.abs()
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

    for row_y in 0..12 {
        for column_x in 0..53 {
            match row_y {
                0 => {
                    y_location = -30;
                }
                1 => {
                    y_location = -29;
                }
                2 => {
                    y_location = -26;
                }
                3 => {
                    y_location = -25;
                }
                4 => {
                    y_location = -22;
                }
                5 => {
                    y_location = -21;
                }
                6 => {
                    y_location = 21;
                }
                7 => {
                    y_location = 22;
                }
                8 => {
                    y_location = 25;
                }
                9 => {
                    y_location = 26;
                }
                10 => {
                    y_location = 29;
                }
                _ => {
                    y_location = 30;
                }
            }

            match column_x {
                0..=15 => {
                    x_location = -30 + column_x;
                }
                16..=36 => {
                    x_location = -30 + column_x + 4;
                }
                _ => {
                    x_location = -30 + column_x + 8;
                }
            }

            if
                (((SECTOR_SIZE - 1) / 2) as i32) + x_location >= 0 &&
                (((SECTOR_SIZE - 1) / 2) as i32) + y_location >= 0 &&
                x_location.abs() < y_location.abs()
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
        RuinTile {},
    ));
}

pub fn update_ruins(
    graphics_sector_memory: &crate::graphics::GamesectorGraphicsBasicsMemory,
    ruin_query: &mut Query <(&mut super::TileIndex, &mut TextureAtlasSprite, &mut Transform, &mut Visibility, &RuinTile), Without<super::EnvironmentalTile>>
    
) {
    let mut tile_graphics_index: u16;

    for mut ruin_properties in ruin_query.iter_mut() {
        tile_graphics_index = 49;

        match
            graphics_sector_memory.tile_array_variety[ruin_properties.0.x as usize]
                [ruin_properties.0.y as usize].0
        {
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

        match
            graphics_sector_memory.tile_array[ruin_properties.0.x as usize]
                [ruin_properties.0.y as usize]
        {
            TileType::Ruin1x1 => {}

            TileType::RuinBottomLeft => {
                tile_graphics_index += 4;
            }
            TileType::RuinBottomRight => {
                tile_graphics_index += 4;
            }
            TileType::RuinTopLeft => {
                tile_graphics_index += 4;
            }
            TileType::RuinTopRight => {
                tile_graphics_index += 4;
            }

            TileType::RuinBottomSide => {
                tile_graphics_index += 8;
            }
            TileType::RuinTopSide => {
                tile_graphics_index += 8;
            }
            TileType::RuinLeftSide => {
                tile_graphics_index += 8;
            }
            TileType::RuinRightSide => {
                tile_graphics_index += 8;
            }

            _ => {
                tile_graphics_index = 0;
            }
        }

        ruin_properties.1.index = tile_graphics_index as usize;

        if tile_graphics_index == 0 {
            *ruin_properties.3 = Hidden;
        } else {
            *ruin_properties.3 = Inherited;
        }
    }
}
