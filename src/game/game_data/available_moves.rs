use crate::game::game_data::{
    constants::{EAST, NORTH, NUMBERS, SOUTH, WEST},
    models::Movement,
};

pub fn available_moves() -> [Vec<Movement>; 13] {
    [
        // Room 1
        vec![
            Movement {
                direction: NORTH,
                move_to: NUMBERS[12],
            },
            Movement {
                direction: SOUTH,
                move_to: NUMBERS[5],
            },
            Movement {
                direction: EAST,
                move_to: NUMBERS[7],
            },
            Movement {
                direction: WEST,
                move_to: NUMBERS[1],
            },
        ],
        // Room 2
        vec![
            Movement {
                direction: SOUTH,
                move_to: NUMBERS[4],
            },
            Movement {
                direction: WEST,
                move_to: NUMBERS[2],
            },
            Movement {
                direction: EAST,
                move_to: NUMBERS[0],
            },
        ],
        // Room 3
        vec![
            Movement {
                direction: SOUTH,
                move_to: NUMBERS[3],
            },
            Movement {
                direction: EAST,
                move_to: NUMBERS[1],
            },
        ],
        // Room 4
        vec![
            Movement {
                direction: NORTH,
                move_to: NUMBERS[2],
            },
            Movement {
                direction: EAST,
                move_to: NUMBERS[4],
            },
        ],
        // Room 5
        vec![
            Movement {
                direction: NORTH,
                move_to: NUMBERS[1],
            },
            Movement {
                direction: EAST,
                move_to: NUMBERS[5],
            },
            Movement {
                direction: WEST,
                move_to: NUMBERS[3],
            },
        ],
        // Room 6
        vec![
            Movement {
                direction: NORTH,
                move_to: NUMBERS[0],
            },
            Movement {
                direction: EAST,
                move_to: NUMBERS[6],
            },
            Movement {
                direction: WEST,
                move_to: NUMBERS[4],
            },
        ],
        // Room 7
        vec![
            Movement {
                direction: NORTH,
                move_to: NUMBERS[7],
            },
            Movement {
                direction: WEST,
                move_to: NUMBERS[5],
            },
        ],
        // Room 8
        vec![
            Movement {
                direction: NORTH,
                move_to: NUMBERS[8],
            },
            Movement {
                direction: SOUTH,
                move_to: NUMBERS[6],
            },
            Movement {
                direction: WEST,
                move_to: NUMBERS[0],
            },
        ],
        // Room 9
        vec![
            Movement {
                direction: NORTH,
                move_to: NUMBERS[10],
            },
            Movement {
                direction: SOUTH,
                move_to: NUMBERS[7],
            },
            Movement {
                direction: EAST,
                move_to: NUMBERS[9],
            },
        ],
        // Room 10
        vec![Movement {
            direction: WEST,
            move_to: NUMBERS[8],
        }],
        // Room 11
        vec![
            Movement {
                direction: SOUTH,
                move_to: NUMBERS[8],
            },
            Movement {
                direction: WEST,
                move_to: NUMBERS[12],
            },
            Movement {
                direction: EAST,
                move_to: NUMBERS[11],
            },
        ],
        // Room 12
        vec![Movement {
            direction: WEST,
            move_to: NUMBERS[10],
        }],
        // Room 13
        vec![
            Movement {
                direction: SOUTH,
                move_to: NUMBERS[0],
            },
            Movement {
                direction: EAST,
                move_to: NUMBERS[10],
            },
        ],
    ]
}
