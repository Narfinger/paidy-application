use axum::Router;
use types::{new_app_state, AppState};

mod types;

// we validate against this secret key. Not perfect security but better than nothing.
static SECRET_KEY: &str =
    "QXlj0uzlyckcmhVvvRHfSKzXZZE0K/k7+dyQx2k5Le2HwTdpInoh3VtDiLEV4eJLTX3aUcG+7mVO";

#[tokio::main]
async fn main() {
    let state: AppState = new_app_state();

    let app = Router::new().with_state(state);

    println!("Listening on port 127.0.0.1:3000");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Cannot listen on port 3000");
    axum::serve(listener, app).await.unwrap();
}
