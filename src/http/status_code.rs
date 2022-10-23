use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::NotFound => "Not Found",
            Self::BadRequest => "Bad Request",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Dereference the immutable reference to self and get the actual value
        write!(f, "{}", *self as u16)
    }
}
