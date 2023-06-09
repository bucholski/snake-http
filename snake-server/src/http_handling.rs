use crate::move_queue_handling;
use move_queue_handling::*;

use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub type DirQueue = Arc<Mutex<Vec<Direction>>>;

pub async fn handle_connection(mut stream: TcpStream, queue: DirQueue, state: &String) {
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

    let process_request = match http_request
        .lines()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .get(0)
    {
        Some(a) => a,
        None => "Nothing",
    };
    println!("{}", process_request);
    match process_request {
        "POST /snake/right HTTP/1.1" => queue.lock().unwrap().push(Direction::Right),
        "POST /snake/left HTTP/1.1" => queue.lock().unwrap().push(Direction::Left),
        "POST /snake/up HTTP/1.1" => queue.lock().unwrap().push(Direction::Up),
        "POST /snake/down HTTP/1.1" => queue.lock().unwrap().push(Direction::Down),
        "GET /snake HTTP/1.1" => println!("Should respond with ASCII art"),
        _ => eprintln!("Error: Bad Request"),
    }

    let status_line = "HTTP/1.1 200 OK";
    let contents = state;
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Type: text\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    // println!("RESPONSE NOW");
    // println!("{}", response);
    stream.write(response.as_bytes()).await.unwrap();
}
