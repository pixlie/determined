use thiserror::Error;

#[derive(Error, Debug)]
pub enum DetError {
    #[error("Invalid request")]
    InvalidRequest,

    #[error("JSON response from LLM could not be parsed: {0}")]
    InvalidJSONFromLLM(String),

    #[error("Missing item in response from LLM: {0}")]
    MissingItemInResponseFromLLM(String),
}

pub type DetResult<T> = Result<T, DetError>;
