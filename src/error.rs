#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Not found")]
    NotFound,

    #[error("Invalid data")]
    InvalidData,
}
