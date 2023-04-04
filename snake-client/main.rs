pub mod move_management;
use move_management::*;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread::sleep,
    time::Duration,
};

fn main() {
    let mut move_queue: Vec<Direction> = Vec::new();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        // println!("Connection established");
        let stream = stream.unwrap();
        handle_connection(stream, &mut move_queue);
        println!("{:?}", move_queue);
    }
}

fn handle_connection(mut stream: TcpStream, queue: &mut Vec<Direction>) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let get_direction = match http_request.get(0) {
        Some(a) => a,
        None => "Nothing",
    };

    match get_direction {
        "POST /snake/right HTTP/1.1" => queue.push(Direction::Right),
        "POST /snake/left HTTP/1.1" => queue.push(Direction::Left),
        "POST /snake/up HTTP/1.1" => queue.push(Direction::Up),
        "POST /snake/down HTTP/1.1" => queue.push(Direction::Down),
        _ => eprintln!("Error: Bad Request"),
    }

    // println!("{:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let contents = "debug time!";
    let length = contents.len();
    println!();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
