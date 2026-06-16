use thiserror::Error;

#[derive(Error, Debug)]
pub enum DnfDeskError {
    #[error("DNF5 command failed: {0}")]
    CommandFailed(String),

    #[error("Unable to parse DNF5 output")]
    ParseError,
}
