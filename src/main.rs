use core::num;
use std::io::Read;

use http::Request;
use server::Server;

mod http;
mod server;

fn main() {
    let this_server = Server::new("localhost:3030".to_string());

    let listener = this_server.run();

    loop {
        match listener.accept() {
            Ok((mut stream, _)) => {
                let mut buffer = [0; 1024];

                match stream.read(&mut buffer) {
                    Ok(_) => {
                        // println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                        match Request::try_from(&buffer[..]) {
                            Ok(request) => {
                                dbg!(request);
                            }
                            Err(e) => println!("Failed to parse a request: {}", e),
                        }
                    }
                    Err(err) => {
                        println!("Failed to read stream buffer: ${err}");
                    }
                }
            }
            Err(err) => {
                println!("Failed to establish a connection: {err}");
            }
        }
    }
}
