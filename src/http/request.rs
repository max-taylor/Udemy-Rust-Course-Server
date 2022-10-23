use super::{
    method::{Method, MethodError},
    QueryString,
};
use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    str,
    str::Utf8Error,
};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // Doing type conversions in this matter is more idiomatic rust and how rust was designed
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;

        // The question mark operator unwraps the result if ok, otherwise it will use the provided error. It essentially does a match statement with Some and Err.
        // Also re-using the request name doesn't override the above request variable, it recreates the variable from scratch using the same name and the old one no longer exists
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        // This removes an empty match arm by allowing the if statement to only execute if the Option value returns something
        if let Some(i) = path.find("?") {
            query_string = Some(QueryString::from(&path[i + 1..]));
            // This variable is marked as mutable which allows it to be reassigned to
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            // ! NOTE: doing i + 1 here is typically unsafe. This is because strings have to be UTF-8 encoded and adding 1 adds one byte to the range, not one character. So this could return a number of characters and then an incomplete character at the end which would crash the program. Because we know the value is a space, this code is actually safe.
            return Some((&request[..i], &request[(i + 1)..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
