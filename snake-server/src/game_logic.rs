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
    let head = snake.get(0).expect("Snake MUST have a head");
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
    free_points.remove(snake.get(0).expect("Snake MUST have a head"));
    free_points.insert(
        snake
            .get(snake.len() - 1)
            .expect("Snake is never shorter than 3 segments")
            .clone(),
    );
}

pub fn replace_cookie(cookie: &mut Point, free_points: &BTreeSet<Point>) {
    let mut rng = thread_rng();
    match free_points.iter().choose(&mut rng) {
        Some(x) => *cookie = x.clone(),
        None => game_won(),
    };
}
fn add_segment(snake: &mut VecDeque<Point>) {
    let last_segment: Point = snake
        .get(&snake.len() - 1)
        .expect("Snake is never shorter than 3 segments")
        .clone();
    snake.push_back(last_segment)
}

fn game_won() {
    loop {
        println!("You won!"); //shrug
    }
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
    let mut head = snake.get(0).expect("Snake MUST have a head").clone();
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
