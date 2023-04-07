// use move_queue_handling::Direction;

use std::collections::VecDeque;

use crate::move_queue_handling::Direction;

#[derive(PartialEq, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub fn segments_init(dimensions: &Point) -> VecDeque<Point> {
    let mut snake_segments: VecDeque<Point> = VecDeque::new();
    for i in 0..3 {
        snake_segments.push_back(Point {
            x: (dimensions.x / 2 - i),
            y: (dimensions.y / 2),
        })
    }
    snake_segments
}

pub fn move_snake(direction: &Direction, current_snake: &mut VecDeque<Point>, dimensions: &Point) {
    let current_head = current_snake.get(0).unwrap();
    let new_head = match direction {
        Direction::Up if current_head.y == 0 => Point {
            x: current_head.x,
            y: dimensions.y - 1,
        },
        Direction::Right if current_head.x == dimensions.x - 1 => Point {
            x: 0,
            y: current_head.y,
        },
        Direction::Down if current_head.y == dimensions.y - 1 => Point {
            x: current_head.x,
            y: 0,
        },
        Direction::Left if current_head.x == 0 => Point {
            x: dimensions.x - 1,
            y: current_head.y,
        },
        Direction::Up => Point {
            x: current_head.x,
            y: current_head.y - 1,
        },
        Direction::Right => Point {
            x: current_head.x + 1,
            y: current_head.y,
        },
        Direction::Down => Point {
            x: current_head.x,
            y: current_head.y + 1,
        },
        Direction::Left => Point {
            x: current_head.x - 1,
            y: current_head.y,
        },
    };
    current_snake.pop_back();
    current_snake.push_front(new_head);
}
