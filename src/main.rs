#![allow(dead_code)]

use server::Server;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let this_server = Server::new("localhost:3030".to_string());

    this_server.run(WebsiteHandler);
}
