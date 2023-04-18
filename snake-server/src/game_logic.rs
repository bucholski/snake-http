// use move_queue_handling::Direction;

use crate::move_queue_handling::Direction;
use rand::{seq::IteratorRandom, thread_rng};
use std::collections::{BTreeSet, VecDeque};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

pub fn free_points_init(
    dimensions: &Point,
    free_points: &mut BTreeSet<Point>,
    snake: &VecDeque<Point>,
) {
    for x in 0..dimensions.x {
        for y in 0..dimensions.y {
            free_points.insert(Point { x: x, y: y });
        }
    }
    for segment in snake {
        free_points.remove(&segment);
    }
}

pub fn game_handler(
    direction: &mut Direction,
    snake: &mut VecDeque<Point>,
    dimensions: &Point,
    cookie: &mut Point,
    free_points: &mut BTreeSet<Point>,
) {
    let (mut ate_cookie, mut hit_tail) = (false, false);
    move_snake(direction, snake, dimensions);
    update_free_points(free_points, snake);
    check_collisions(&snake, &mut hit_tail, &mut ate_cookie, &cookie);
    if ate_cookie {
        add_segment(snake);
        replace_cookie(cookie, free_points);
    }
    if hit_tail {
        restart_game(dimensions, snake, cookie, free_points, direction);
    }
}

fn check_collisions(
    snake: &VecDeque<Point>,
    hit_tail: &mut bool,
    ate_cookie: &mut bool,
    cookie: &Point,
) {
    let head = snake.get(0).unwrap();
    let headless: VecDeque<&Point> = snake.range(1..).collect();
    if head == cookie {
        *ate_cookie = true;
    }
    if headless.contains(&head) {
        // println!("GAME OVER");
        *hit_tail = true;
    }
}

fn update_free_points(free_points: &mut BTreeSet<Point>, snake: &VecDeque<Point>) {
    free_points.remove(snake.get(0).unwrap());
    free_points.insert(snake.get(snake.len() - 1).unwrap().clone());
}

pub fn replace_cookie(cookie: &mut Point, free_points: &BTreeSet<Point>) {
    let mut rng = thread_rng();
    *cookie = free_points.iter().choose(&mut rng).unwrap().clone();
    println!("NEW COOKIE");
    //TODO On Option -> NONE WIN THE GAME
}
fn add_segment(snake: &mut VecDeque<Point>) {
    let last_segment: Point = snake.get(&snake.len() - 1).unwrap().clone();
    snake.push_back(last_segment)
}
fn restart_game(
    dimensions: &Point,
    snake: &mut VecDeque<Point>,
    cookie: &mut Point,
    free_points: &mut BTreeSet<Point>,
    current_direction: &mut Direction,
) {
    snake_init(dimensions, snake);
    free_points_init(dimensions, free_points, snake);
    replace_cookie(cookie, free_points);
    println!("GAME OVER");
    *current_direction = Direction::Right;
}
pub fn snake_init(dimensions: &Point, snake: &mut VecDeque<Point>) {
    let mut snake_segments: VecDeque<Point> = VecDeque::new();
    for i in 0..3 {
        snake_segments.push_back(Point {
            x: (dimensions.x / 2 - i),
            y: (dimensions.y / 2),
        })
    }
    *snake = snake_segments.clone();
}

fn move_snake(direction: &Direction, snake: &mut VecDeque<Point>, dimensions: &Point) {
    let mut head = snake.get(0).unwrap().clone();
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
    snake.pop_back();
    snake.push_front(head);
}

// pub fn move_snake(direction: &Direction, snake: &mut VecDeque<Point>, dimensions: &Point) {
//     let current_head = snake.get(0).unwrap();
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
//     snake.pop_back();
//     snake.push_front(new_head);
// }
// #[cfg(test)]
// use super::*;
// #[test]
// fn test_cookie_spawn() {
//     let dimensions = Point { x: 5, y: 5 };
//     let mut cookie: Point = Point { x: 0, y: 0 };
//     let mut snake = game_logic::snake_init(&dimensions, snake);
//     let mut free_points = BTreeSet::new();
//     game_logic::free_points_init(&dimensions, &mut free_points, &snake);
//     for _ in 0..10000 {
//         replace_cookie(&mut cookie, &free_points);
//         let check: Vec<_> = snake.iter().filter(|x| **x == cookie).collect();
//         if check.len() > 0 {
//             assert!(false);
//         }
//     }
// }
