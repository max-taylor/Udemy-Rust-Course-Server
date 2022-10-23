use crate::http::ParseError;
// crate utilizes the root of the module
use crate::http::Request;
use crate::http::Response;
use crate::http::StatusCode;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

#[derive(Clone)]
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        println!("Listening on... {}", self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            // println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(&request);

                                    handler.handle_request(&request)
                                    // Response::new(StatusCode::Ok, Some("<h1>Yoo</h1>".to_string()))
                                }
                                Err(e) => {
                                    println!("Failed to parse a request: {}", e);

                                    handler.handle_bad_request(&e)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
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
