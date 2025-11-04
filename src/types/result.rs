use crate::types::{error::Error, method::Method};

pub type Result<T> = std::result::Result<T, Error>;

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "GET" => Method::Get,
            _ => Method::Unknown,
        }
    }
}
