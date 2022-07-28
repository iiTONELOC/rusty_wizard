use crate::game_data::constants::{DIRECTIONS, NUMBERS};
use crate::game_data::models::Movement;

pub fn available_moves() -> [Vec<Movement>; 13] {
    [
        // Room 1
        vec![
            Movement {
                direction: DIRECTIONS.north,
                move_to: NUMBERS[12],
            },
            Movement {
                direction: DIRECTIONS.south,
                move_to: NUMBERS[5],
            },
            Movement {
                direction: DIRECTIONS.east,
                move_to: NUMBERS[7],
            },
            Movement {
                direction: DIRECTIONS.west,
                move_to: NUMBERS[1],
            },
        ],
        // Room 2
        vec![
            Movement {
                direction: DIRECTIONS.south,
                move_to: NUMBERS[4],
            },
            Movement {
                direction: DIRECTIONS.west,
                move_to: NUMBERS[2],
            },
            Movement {
                direction: DIRECTIONS.east,
                move_to: NUMBERS[0],
            },
        ],
        // Room 3
        vec![
            Movement {
                direction: DIRECTIONS.south,
                move_to: NUMBERS[3],
            },
            Movement {
                direction: DIRECTIONS.east,
                move_to: NUMBERS[1],
            },
        ],
        // Room 4
        vec![
            Movement {
                direction: DIRECTIONS.north,
                move_to: NUMBERS[2],
            },
            Movement {
                direction: DIRECTIONS.east,
                move_to: NUMBERS[4],
            },
        ],
        // Room 5
        vec![
            Movement {
                direction: DIRECTIONS.north,
                move_to: NUMBERS[1],
            },
            Movement {
                direction: DIRECTIONS.east,
                move_to: NUMBERS[5],
            },
            Movement {
                direction: DIRECTIONS.west,
                move_to: NUMBERS[3],
            },
        ],
        // Room 6
        vec![
            Movement {
                direction: DIRECTIONS.north,
                move_to: NUMBERS[0],
            },
            Movement {
                direction: DIRECTIONS.east,
                move_to: NUMBERS[6],
            },
            Movement {
                direction: DIRECTIONS.west,
                move_to: NUMBERS[4],
            },
        ],
        // Room 7
        vec![
            Movement {
                direction: DIRECTIONS.north,
                move_to: NUMBERS[7],
            },
            Movement {
                direction: DIRECTIONS.west,
                move_to: NUMBERS[5],
            },
        ],
        // Room 8
        vec![
            Movement {
                direction: DIRECTIONS.north,
                move_to: NUMBERS[8],
            },
            Movement {
                direction: DIRECTIONS.south,
                move_to: NUMBERS[6],
            },
            Movement {
                direction: DIRECTIONS.west,
                move_to: NUMBERS[0],
            },
        ],
        // Room 9
        vec![
            Movement {
                direction: DIRECTIONS.north,
                move_to: NUMBERS[10],
            },
            Movement {
                direction: DIRECTIONS.south,
                move_to: NUMBERS[7],
            },
            Movement {
                direction: DIRECTIONS.east,
                move_to: NUMBERS[9],
            },
        ],
        // Room 10
        vec![Movement {
            direction: DIRECTIONS.west,
            move_to: NUMBERS[8],
        }],
        // Room 11
        vec![
            Movement {
                direction: DIRECTIONS.south,
                move_to: NUMBERS[8],
            },
            Movement {
                direction: DIRECTIONS.west,
                move_to: NUMBERS[12],
            },
            Movement {
                direction: DIRECTIONS.east,
                move_to: NUMBERS[11],
            },
        ],
        // Room 12
        vec![Movement {
            direction: DIRECTIONS.west,
            move_to: NUMBERS[10],
        }],
        // Room 13
        vec![
            Movement {
                direction: DIRECTIONS.south,
                move_to: NUMBERS[0],
            },
            Movement {
                direction: DIRECTIONS.east,
                move_to: NUMBERS[10],
            },
        ],
    ]
}
