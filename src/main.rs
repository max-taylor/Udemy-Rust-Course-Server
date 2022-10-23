#![allow(dead_code)]

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    // This macro reads environment variables at compile time
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    dbg!(&public_path);

    let this_server = Server::new("localhost:3030".to_string());

    this_server.run(WebsiteHandler::new(public_path));
}

// Next Steps:
// Headers are currently being ignored
// Multithread the server: std::thread and std::sync for mutex's, etc
// After multithreading look into async-await in rust (https://tokio.rs/)
