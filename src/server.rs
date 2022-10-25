use crate::http::ParseError;
use crate::http::Request;
use crate::http::Response;
use crate::http::StatusCode;
use std::io::Error;
use std::io::Read;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

pub trait Handler: Sync + Send {
    fn handle_request(&self, request: &Request) -> Response;

    fn handle_bad_request(&self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

#[derive(Clone)]
pub struct Server {
    addr: String,
    num_threads: u16,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr,
            num_threads: 10,
        }
    }

    fn handle_listener(
        &self,
        handler: &impl Handler,
        accepted_listener: Result<(TcpStream, SocketAddr), Error>,
    ) {
        if let Some(err) = accepted_listener.as_ref().err() {
            println!("Failed to establish a connection: {err}");

            return;
        }

        let (mut stream, _) = accepted_listener.unwrap();

        let mut buffer = [0; 1024];

        let read_response = stream.read(&mut buffer);

        if let Some(err) = read_response.err() {
            println!("Failed to read stream buffer: ${err}");

            return;
        }

        let response = match Request::try_from(&buffer[..]) {
            Ok(request) => handler.handle_request(&request),
            Err(e) => handler.handle_bad_request(&e),
        };

        if let Err(e) = response.send(&mut stream) {
            println!("Failed to send response: {}", e);

            return;
        }

        println!("Successfully sent response");
    }

    fn test_things(&self) {}

    pub fn run(&'static self, handler: &'static impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        println!("Listening on... {}", self.addr);

        loop {
            let accepting_listener = listener.accept();

            // self.handle_listener(&handler, accepting_listener);

            // let created_thread = thread::spawn(move || self.handle_thread());
            thread::spawn(move || self.handle_listener(handler, accepting_listener));
        }
    }
}
