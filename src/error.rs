
use thiserror::Error as TheError;

#[derive(TheError, Clone, Debug)]
pub enum Error {
    #[error("Http  Error")]
    RequestError,
}
