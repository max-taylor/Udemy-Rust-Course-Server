#![allow(dead_code)]

use server::Server;

mod http;
mod server;

fn main() {
    let this_server = Server::new("localhost:3030".to_string());

    this_server.run();
}
