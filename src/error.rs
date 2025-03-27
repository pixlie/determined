use thiserror::Error;

#[derive(Error, Debug)]
pub enum DetError {
    #[error("Invalid request")]
    InvalidRequest,
}

pub type DetResult<T> = Result<T, DetError>;
