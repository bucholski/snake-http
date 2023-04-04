// use move_queue_handling::Direction;

#[derive(PartialEq, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub fn segments_init(dimensions: &Point) -> Vec<Point> {
    let mut snake_segments: Vec<Point> = Vec::new();
    for i in 0..3 {
        snake_segments.push(Point {
            x: (dimensions.x / 2 - i),
            y: (dimensions.y / 2),
        })
    }
    snake_segments
}
