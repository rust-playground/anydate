use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid Date")]
    InvalidDate,

    #[error("Invalid DateTime")]
    InvalidDateTime,
}
