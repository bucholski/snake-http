use std::collections::VecDeque;

use super::*;

pub fn draw_map(dimensions: &Point, cookie: &Point, snake_segments: &VecDeque<Point>) -> String {
    let map_tile = "··";
    let snake_tile = "██";
    let cookie_tile = "()";
    let mut map = String::new();
    for y in 0..dimensions.y {
        for x in 0..dimensions.x {
            let current_point = Point { x: x, y: y };
            let snake_is_here: &Point = match snake_segments
                .iter()
                .filter(|segment| segment.x == current_point.x && segment.y == current_point.y)
                .next()
            {
                Some(x) => x,
                None => &Point {
                    //just move it outta the dimensions
                    x: 9999,
                    y: 9999,
                },
            };
            match current_point {
                x if x == *snake_is_here => map.push_str(snake_tile),
                x if x == *cookie => map.push_str(cookie_tile),
                _ => map.push_str(map_tile),
            }
        }
        map.push_str("\n")
    }
    // print!("{map}");
    map
}
