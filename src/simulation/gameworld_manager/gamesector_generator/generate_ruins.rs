use rand::prelude::*;

use crate::SECTOR_SIZE;
use crate::simulation::TileType;

#[derive(Copy, Clone, PartialEq)]
pub enum RuinBlockDirection {
    Vertical,
    Horizontal,
}

pub fn generate_ruins<R: Rng>(
    gamesector_environment_array: &mut [[TileType; SECTOR_SIZE as usize]; SECTOR_SIZE as usize],
    seeded_prng: &mut R
) {
    let mut column_x: i32;
    let mut row_y: i32;
    let mut x_start: i32;
    let mut y_start: i32;
    
    // This part draws the vertical columns of 2x3 buildings.

    for _n in 0..40 {
        column_x = seeded_prng.gen_range(0..6);
        row_y = seeded_prng.gen_range(0..49);

        match column_x {
            0..=2 => {
                x_start = -30 + column_x * 4;
            }

            _ => {
                x_start = 21 + (column_x - 3) * 4;
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

        draw_ruin_if_open(
            x_start,
            y_start,
            3,
            2,
            gamesector_environment_array,
            RuinBlockDirection::Vertical
        );
    }
    
    // This part draws the horizontal columns of 2x3 buildings.

    for _n in 0..40 {
        row_y = seeded_prng.gen_range(0..6);
        column_x = seeded_prng.gen_range(0..49);

        match row_y {
            0..=2 => {
                y_start = -30 + row_y * 4;
            }

            _ => {
                y_start = 21 + (row_y - 3) * 4;
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

        draw_ruin_if_open(
            x_start,
            y_start,
            2,
            3,
            gamesector_environment_array,
            RuinBlockDirection::Horizontal
        );
    }

    // This part draws the vertical columns of 2x2 buildings.

    for _n in 0..50 {
        column_x = seeded_prng.gen_range(0..6);
        row_y = seeded_prng.gen_range(0..49);

        match column_x {
            0..=2 => {
                x_start = -30 + column_x * 4;
            }

            _ => {
                x_start = 21 + (column_x - 3) * 4;
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

        draw_ruin_if_open(
            x_start,
            y_start,
            2,
            2,
            gamesector_environment_array,
            RuinBlockDirection::Vertical
        );
    }

    
    // This part draws the horizontal columns of 2x2 buildings.

    for _n in 0..50 {
        row_y = seeded_prng.gen_range(0..6);
        column_x = seeded_prng.gen_range(0..49);

        match row_y {
            0..=2 => {
                y_start = -30 + row_y * 4;
            }

            _ => {
                y_start = 21 + (row_y - 3) * 4;
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

        draw_ruin_if_open(
            x_start,
            y_start,
            2,
            2,
            gamesector_environment_array,
            RuinBlockDirection::Horizontal
        );
    }

    // This part draws the vertical columns of 1x1 buildings.

    for _n in 0..60 {
        column_x = seeded_prng.gen_range(0..6);
        row_y = seeded_prng.gen_range(0..49);

        match column_x {
            0..=2 => {
                x_start = -30 + column_x * 4;
            }

            _ => {
                x_start = 21 + (column_x - 3) * 4;
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

        draw_ruin_if_open(
            x_start,
            y_start,
            1,
            1,
            gamesector_environment_array,
            RuinBlockDirection::Vertical
        );
    }

    
    // This part draws the horizontal columns of 1x1 buildings.

    for _n in 0..60 {
        row_y = seeded_prng.gen_range(0..6);
        column_x = seeded_prng.gen_range(0..49);

        match row_y {
            0..=2 => {
                y_start = -30 + row_y * 4;
            }

            _ => {
                y_start = 21 + (row_y - 3) * 4;
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

        draw_ruin_if_open(
            x_start,
            y_start,
            1,
            1,
            gamesector_environment_array,
            RuinBlockDirection::Horizontal
        );
    }
}

fn draw_ruin_if_open(
    x_start: i32,
    y_start: i32,
    ruin_y_length: i32,
    ruin_x_width: i32,
    gamesector_environment_array: &mut [[TileType; SECTOR_SIZE as usize]; SECTOR_SIZE as usize],
    block_direction: RuinBlockDirection
) {
    // This part checks if the space is clear.

    let x_start_index = x_start + (((SECTOR_SIZE - 1) / 2) as i32);
    let y_start_index = y_start + (((SECTOR_SIZE - 1) / 2) as i32);

    let mut space_clear: bool = true;

    for delta_x in 0..=ruin_x_width {
        for delta_y in 0..=ruin_y_length {
            if
                gamesector_environment_array[(x_start_index + delta_x) as usize as usize]
                    [(y_start_index + delta_y) as usize as usize] != TileType::Open
            {
                space_clear = false;
            }
            if
                block_direction == RuinBlockDirection::Vertical &&
                (y_start + delta_y).abs() > (x_start + delta_x).abs()
            {
                space_clear = false;
            } else if
                block_direction == RuinBlockDirection::Horizontal &&
                (x_start + delta_x).abs() > (y_start + delta_y).abs()
            {
                space_clear = false;
            }
        }
    }

    // If the space is clear, then this puts the building there. If not, nothing happens.

    if space_clear == true {
        for delta_x in 0..ruin_x_width {
            for delta_y in 0..ruin_y_length {
                if ruin_x_width == 1 && ruin_y_length == 1 {
                    gamesector_environment_array[(x_start_index + delta_x) as usize as usize][
                        (y_start_index + delta_y) as usize as usize
                    ] = TileType::Ruin1x1;
                }

                if ruin_x_width == 2 && ruin_y_length == 2 {
                    if delta_x == 0 && delta_y == 0 {
                        gamesector_environment_array[(x_start_index + delta_x) as usize][
                            (y_start_index + delta_y) as usize
                        ] = TileType::RuinBottomLeft;
                    } else if delta_x == 1 && delta_y == 0 {
                        gamesector_environment_array[(x_start_index + delta_x) as usize][
                            (y_start_index + delta_y) as usize
                        ] = TileType::RuinBottomRight;
                    } else if delta_x == 0 && delta_y == 1 {
                        gamesector_environment_array[(x_start_index + delta_x) as usize][
                            (y_start_index + delta_y) as usize
                        ] = TileType::RuinTopLeft;
                    } else if delta_x == 1 && delta_y == 1 {
                        gamesector_environment_array[(x_start_index + delta_x) as usize][
                            (y_start_index + delta_y) as usize
                        ] = TileType::RuinTopRight;
                    }
                }

                if ruin_x_width == 3 && ruin_y_length == 2 {
                    if delta_x == 0 && delta_y == 0 {
                        gamesector_environment_array[(x_start_index + delta_x) as usize][
                            (y_start_index + delta_y) as usize
                        ] = TileType::RuinBottomLeft;
                    } else if delta_x == 1 && delta_y == 0 {
                        gamesector_environment_array[(x_start_index + delta_x) as usize][
                            (y_start_index + delta_y) as usize
                        ] = TileType::RuinBottomSide;
                    } else if delta_x == 2 && delta_y == 0 {
                        gamesector_environment_array[(x_start_index + delta_x) as usize][
                            (y_start_index + delta_y) as usize
                        ] = TileType::RuinBottomRight;
                    } else if delta_x == 0 && delta_y == 1 {
                        gamesector_environment_array[(x_start_index + delta_x) as usize][
                            (y_start_index + delta_y) as usize
                        ] = TileType::RuinTopLeft;
                    } else if delta_x == 1 && delta_y == 1 {
                        gamesector_environment_array[(x_start_index + delta_x) as usize][
                            (y_start_index + delta_y) as usize
                        ] = TileType::RuinTopSide;
                    } else if delta_x == 2 && delta_y == 1 {
                        gamesector_environment_array[(x_start_index + delta_x) as usize][
                            (y_start_index + delta_y) as usize
                        ] = TileType::RuinTopRight;
                    }
                }

                if ruin_x_width == 2 && ruin_y_length == 3 {
                    if delta_x == 0 && delta_y == 0 {
                        gamesector_environment_array[(x_start_index + delta_x) as usize][
                            (y_start_index + delta_y) as usize
                        ] = TileType::RuinBottomLeft;
                    } else if delta_x == 1 && delta_y == 0 {
                        gamesector_environment_array[(x_start_index + delta_x) as usize][
                            (y_start_index + delta_y) as usize
                        ] = TileType::RuinBottomRight;
                    } else if delta_x == 0 && delta_y == 1 {
                        gamesector_environment_array[(x_start_index + delta_x) as usize][
                            (y_start_index + delta_y) as usize
                        ] = TileType::RuinLeftSide;
                    } else if delta_x == 1 && delta_y == 1 {
                        gamesector_environment_array[(x_start_index + delta_x) as usize][
                            (y_start_index + delta_y) as usize
                        ] = TileType::RuinRightSide;
                    } else if delta_x == 0 && delta_y == 2 {
                        gamesector_environment_array[(x_start_index + delta_x) as usize][
                            (y_start_index + delta_y) as usize
                        ] = TileType::RuinTopLeft;
                    } else if delta_x == 1 && delta_y == 2 {
                        gamesector_environment_array[(x_start_index + delta_x) as usize][
                            (y_start_index + delta_y) as usize
                        ] = TileType::RuinTopRight;
                    }
                }
            }
        }
    }
}
