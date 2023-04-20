use crate::move_queue_handling;
use move_queue_handling::*;

use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub type DirQueue = Arc<Mutex<Vec<Direction>>>;

pub async fn handle_connection(mut stream: TcpStream, queue: DirQueue) {
    let mut buf = [0; 128];
    let http_request: String = match stream.read(&mut buf).await {
        Ok(n) if n == 0 => return,
        Ok(_) => {
            let buf_reader = String::from_utf8(buf.to_vec()).unwrap(); //Think about this unwrap when it's not 10 PM
            buf_reader
        }
        Err(e) => {
            eprintln!("failed to read from socket; err = {:?}", e);
            return;
        }
    };

    let get_direction = match http_request
        .lines()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .get(0)
    {
        Some(a) => a,
        None => "Nothing",
    };

    match get_direction {
        "POST /snake/right HTTP/1.1" => queue.lock().unwrap().push(Direction::Right),
        "POST /snake/left HTTP/1.1" => queue.lock().unwrap().push(Direction::Left),
        "POST /snake/up HTTP/1.1" => queue.lock().unwrap().push(Direction::Up),
        "POST /snake/down HTTP/1.1" => queue.lock().unwrap().push(Direction::Down),
        _ => eprintln!("Error: Bad Request"),
    }

    let status_line = "HTTP/1.1 200 OK";
    let contents = "debug";
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write(response.as_bytes()).await.unwrap();
}
