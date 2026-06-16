use thiserror::Error;

#[derive(Error, Debug)]
pub enum DnfDeskError {
    #[error("DNF command failed")]
    DnfError,
}