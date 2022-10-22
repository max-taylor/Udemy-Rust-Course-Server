// crate utilizes the root of the module
use crate::http::Request;
use std::net::TcpListener;

#[derive(Clone)]
pub struct Server {
    addr: String,
}

impl Server {
    // pub listener: TcpListener;

    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self) -> TcpListener {
        TcpListener::bind(&self.addr).unwrap();
        let listener = TcpListener::bind(&self.addr).unwrap();

        println!("Listening on... {}", self.addr);

        listener
    }
}
