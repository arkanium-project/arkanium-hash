use thiserror::Error as ThisError;

#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v2")]
pub mod v2;
#[cfg(feature = "v3")]
pub mod v3;

pub mod scratchpad;

#[cfg(feature = "tracker")]
pub mod tracker;
const HASH_SIZE: usize = 32;
pub type Hash = [u8; HASH_SIZE];
#[derive(Debug, ThisError)]
#[error("Error while hashing")]
pub enum Error {
    #[error("Error while hashing")]
    Error,
    #[error("Error while casting: {0}")]
    CastError(bytemuck::PodCastError),
    #[error("Error on format")]
    FormatError,
}
