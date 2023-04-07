pub mod client_lib;

#[tokio::main]
async fn main() {
    let mut stdout = client_lib::stdout();

    client_lib::setup(&mut stdout);
    client_lib::input_loop(&mut stdout).await;
    client_lib::wrapup();
}
