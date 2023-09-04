use rand::prelude::*;

use crate::SECTOR_SIZE;
use crate::simulation::TileType;

pub fn generate_ruins<R: Rng>(
    gamesector_environment_array: &mut [[TileType; SECTOR_SIZE as usize]; SECTOR_SIZE as usize],
    seeded_prng: &mut R
) {
    let mut column_x: i32;
    let mut row_y: i32;
    let mut x_start: i32;
    let mut y_start: i32;

    for _n in 0..30 {
        column_x = seeded_prng.gen_range(0..6);
        row_y = seeded_prng.gen_range(0..49);

        match column_x {
            0..=2 => {
                x_start = -30 + column_x * 4;
            }

            _ => {
                x_start = 21 + column_x * 4;
            }
        }

        match row_y {
            0..=13 => {
                y_start = -30 + row_y;
            }
            14..=32 => {
                y_start = -30 + row_y + 6;
            }
            _ => {
                y_start = -30 + row_y + 12;
            }
        }

        draw_ruin_if_open(x_start, y_start, 3, 2, gamesector_environment_array);
    }

    for _n in 0..30 {
        row_y = seeded_prng.gen_range(0..6);
        column_x = seeded_prng.gen_range(0..49);

        match row_y {
            0..=2 => {
                y_start = -30 + row_y * 4;
            }

            _ => {
                y_start = 21 + row_y * 4;
            }
        }

        match column_x {
            0..=13 => {
                x_start = -30 + column_x;
            }
            14..=32 => {
                x_start = -30 + column_x + 6;
            }
            _ => {
                x_start = -30 + column_x + 12;
            }
        }

        draw_ruin_if_open(x_start, y_start, 2, 3, gamesector_environment_array);
    }
}

fn draw_ruin_if_open(
    x: i32,
    y: i32,
    ruin_y_length: i32,
    ruin_x_width: i32,
    gamesector_environment_array: &mut [[TileType; SECTOR_SIZE as usize]; SECTOR_SIZE as usize]
) {}
