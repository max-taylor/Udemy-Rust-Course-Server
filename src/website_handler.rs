use super::server::Handler;
use crate::http::{Method, Request, Response, StatusCode};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>Home Page</h1>".to_string())),
                "/secret" => {
                    Response::new(StatusCode::Ok, Some("<h1>Secret page</h1>".to_string()))
                }
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}