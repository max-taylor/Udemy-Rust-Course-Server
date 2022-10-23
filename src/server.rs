// crate utilizes the root of the module
use crate::http::Request;
use crate::http::Response;
use crate::http::StatusCode;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;

#[derive(Clone)]
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        println!("Listening on... {}", self.addr);

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
                                    let resp = Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>Yoo</h1>".to_string()),
                                    );

                                    resp.send(&mut stream);
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
}
