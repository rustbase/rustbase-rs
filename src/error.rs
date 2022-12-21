use crate::client::Status;

#[derive(Debug)]
pub struct Error {
    pub message: Option<String>,
    pub status: Status,
}

pub type Result<T> = std::result::Result<T, Error>;
