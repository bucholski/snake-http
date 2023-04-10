// use move_queue_handling::Direction;

use std::collections::VecDeque;

use crate::move_queue_handling::Direction;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

pub fn game_handler(
    direction: &Direction,
    current_snake: &mut VecDeque<Point>,
    dimensions: &Point,
    cookie: &mut Point,
    free_points: &mut Vec<Point>,
) {
    let (mut ate_cookie, mut hit_tail) = (false, false);
    move_snake(direction, current_snake, dimensions);
    //TODO
    check_collisions(&current_snake, &mut hit_tail, &mut ate_cookie, &cookie);
    if ate_cookie {
        add_segment(current_snake);
        replace_cookie(cookie, free_points);
    }
    if hit_tail {
        game_over();
    }
}

fn check_collisions(
    current_snake: &VecDeque<Point>,
    hit_tail: &mut bool,
    ate_cookie: &mut bool,
    cookie: &Point,
) {
    let head = current_snake.get(0).unwrap();
    let headless: VecDeque<&Point> = current_snake.range(1..).collect();
    if head == cookie {
        *ate_cookie = true;
    }
    if headless.contains(&head) {
        println!("GAME OVER");
        *hit_tail = true;
    }
}
pub fn replace_cookie(cookie: &mut Point, free_points: &mut Vec<Point>) {
    //TODO
    //remove snake from free points
    //get one random point
    //place cookie on that point
    //gotta use that free points array + binary_search
}
fn add_segment(current_snake: &mut VecDeque<Point>) {
    let last_segment: Point = current_snake.get(&current_snake.len() - 1).unwrap().clone();
    current_snake.push_back(last_segment)
}
fn game_over() {
    //TODO
    //reset the thing
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

fn move_snake(direction: &Direction, current_snake: &mut VecDeque<Point>, dimensions: &Point) {
    let mut head = current_snake.get(0).unwrap().clone();
    match direction {
        Direction::Up => head.y -= 1,
        Direction::Right => head.x += 1,
        Direction::Down => head.y += 1,
        Direction::Left => head.x -= 1,
    };
    if head.x < 0 {
        head.x = dimensions.x - 1
    };
    if head.x >= dimensions.x {
        head.x = 0
    };
    if head.y < 0 {
        head.y = dimensions.y - 1
    };
    if head.y >= dimensions.y {
        head.y = 0
    };
    current_snake.pop_back();
    current_snake.push_front(head);
}

// pub fn move_snake(direction: &Direction, current_snake: &mut VecDeque<Point>, dimensions: &Point) {
//     let current_head = current_snake.get(0).unwrap();
//     let new_head = match direction {
//         Direction::Up => Point {
//             x: current_head.x,
//             y: if current_head.y == 0 {
//                 dimensions.y - 1
//             } else {
//                 current_head.y - 1
//             },
//         },
//         Direction::Right => Point {
//             x: if current_head.x == dimensions.x - 1 {
//                 0
//             } else {
//                 current_head.x + 1
//             },
//             y: current_head.y,
//         },
//         Direction::Down => Point {
//             x: current_head.x,
//             y: if current_head.y == dimensions.y - 1 {
//                 0
//             } else {
//                 current_head.y + 1
//             },
//         },
//         Direction::Left => Point {
//             x: if current_head.x == 0 {
//                 dimensions.x - 1
//             } else {
//                 current_head.x - 1
//             },
//             y: current_head.y,
//         },
//     };
//     current_snake.pop_back();
//     current_snake.push_front(new_head);
// }
