// crate utilizes the root of the module
use crate::http::Request;
use std::io::Read;
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
        TcpListener::bind(&self.addr).unwrap();
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
