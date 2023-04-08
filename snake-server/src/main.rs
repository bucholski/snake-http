pub mod drawing;
pub mod game_logic;
pub mod http_handling;
pub mod move_queue_handling;
use game_logic::Point;
use http_handling::*;
use move_queue_handling::Direction;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let mut current_direction: Direction = Direction::Down;
    let move_queue: DirQueue = Arc::new(Mutex::new(Vec::new()));
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    let mut interval = tokio::time::interval(Duration::from_millis(300));
    let dimensions = Point { x: 10, y: 10 };
    let mut snake = game_logic::segments_init(&dimensions);
    let mut cookie: Point = Point { x: 0, y: 0 };

    loop {
        tokio::select! {
          _server_tick = interval.tick() => {
            current_direction = move_queue_handling::select_move(&move_queue, current_direction);
            move_queue.lock().unwrap().clear();
            game_logic::move_snake(&current_direction, &mut snake, &dimensions);
            drawing::draw_map(&dimensions, &cookie, &snake);
          },
          incoming_request = listener.accept() =>{
            let (stream, _) = incoming_request.unwrap();
            let handle_http = move_queue.clone();
              handle_connection(stream, handle_http).await;
          }
        }
    }
}
