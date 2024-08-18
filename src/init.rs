use thiserror::Error;

#[derive(Debug, Error)]
#[error("{message}")]
pub struct InitError {
    message: String,
}
impl InitError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

pub fn init() -> Result<(), InitError> {
    Err(InitError::new("oh no"))
}
