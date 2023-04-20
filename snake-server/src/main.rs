pub mod drawing;
pub mod game_logic;
pub mod http_handling;
pub mod move_queue_handling;
use game_logic::Point;
use http_handling::*;
use move_queue_handling::Direction;
use std::collections::{BTreeSet, VecDeque};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let dimensions = Point { x: 0, y: 0 };
    let mut free_points = BTreeSet::new();
    let mut cookie: Point = Point { x: 0, y: 0 };
    let mut snake: VecDeque<Point> = VecDeque::new();
    game_logic::snake_init(&dimensions, &mut snake);
    game_logic::free_points_init(&dimensions, &mut free_points, &snake);
    game_logic::replace_cookie(&mut cookie, &mut free_points);

    let mut current_direction: Direction = Direction::Right;
    let move_queue: DirQueue = Arc::new(Mutex::new(Vec::new()));
    let listener = match TcpListener::bind("127.0.0.1:7878").await {
        Ok(x) => x,
        Err(e) => panic!("TCP error: {e}"),
    };
    let mut interval = tokio::time::interval(Duration::from_millis(200));

    loop {
        tokio::select! {
          _server_tick = interval.tick() => {
            current_direction = move_queue_handling::select_move(&move_queue, current_direction);
            match move_queue.lock() {
              Ok(mut x) => x.clear(),
              Err(e) => panic!("Mutex error: {e}"),
            };

            game_logic::game_handler(&mut current_direction, &mut snake, &dimensions, &mut cookie, &mut free_points);
            drawing::draw_map(&dimensions, &cookie, &snake);
          },
          incoming_request = listener.accept() =>{
            let (stream, _) = match incoming_request {
              Ok(x) => x,
              Err(e) => panic!("Incoming Request Error: {e}"),
            };
            let handle_http = move_queue.clone();
              handle_connection(stream, handle_http).await;
          }
        }
    }
}
