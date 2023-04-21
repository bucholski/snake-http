// #[macro_use]
// extern crate crossterm;

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use rand::Error;
use reqwest::{self};
pub use std::io::stdout;
use std::io::Stdout;

pub fn setup(stdout: &mut Stdout) {
    enable_raw_mode().unwrap();
    crossterm::execute!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        Print(
            r#"
        WASD to steer the snake
        space to get the the current state of the game
        ctrl + c to exit 
        Make sure CapsLock is off
        "#
        )
    )
    .unwrap();
}

pub async fn input_loop(stdout: &mut Stdout) {
    let mut handles = vec![];
    loop {
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Char('w'),
                kind: KeyEventKind::Press,
                ..
            }) => {
                execute!(stdout, Clear(ClearType::All), Print("up")).unwrap();
                handles.push(tokio::spawn(async move {
                    post_request("up").await;
                }))
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(' '),
                kind: KeyEventKind::Press,
                ..
            }) => {
                execute!(
                    stdout,
                    Clear(ClearType::All),
                    // Print("You will see the state in a moment")
                )
                .unwrap();
                handles.push(tokio::spawn(async move {
                    get_state().await;
                }))
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('a'),
                kind: KeyEventKind::Press,
                ..
            }) => {
                execute!(stdout, Clear(ClearType::All), Print("left")).unwrap();
                handles.push(tokio::spawn(async move {
                    post_request("left").await;
                }))
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('s'),
                kind: KeyEventKind::Press,
                ..
            }) => {
                execute!(stdout, Clear(ClearType::All), Print("down")).unwrap();
                handles.push(tokio::spawn(async move {
                    post_request("down").await;
                }))
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('d'),
                kind: KeyEventKind::Press,
                ..
            }) => {
                execute!(stdout, Clear(ClearType::All), Print("right")).unwrap();
                handles.push(tokio::spawn(async move {
                    post_request("right").await;
                }))
            }
            _ => (),
        }
    }
}

pub fn wrapup() {
    disable_raw_mode().unwrap();
}
pub async fn post_request(direction: &str) {
    let client = reqwest::Client::new(); //TODO - Arc/Mutex solution for client in main
    let _res = client
        .post(format!("http://127.0.0.1:7878/snake/{}", direction))
        // .body("the exact body that is sent")
        .send()
        .await;
}

pub async fn get_state() {
    let client = reqwest::Client::new();
    let res = client
        .get("http://127.0.0.1:7878/snake")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{}", res);
}
