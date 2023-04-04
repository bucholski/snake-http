pub mod client_lib;
// use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let mut stdout = client_lib::stdout();
    // let client = reqwest::Client::new();

    client_lib::setup(&mut stdout);
    client_lib::input_loop(&mut stdout).await;
    client_lib::wrapup();
}
