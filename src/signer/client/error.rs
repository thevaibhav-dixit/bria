use thiserror::Error;

#[derive(Error, Debug)]
pub enum SigningClientError {
    #[error("SigningClientError - CouldNotConnect: {0}")]
    CouldNotConnect(String),
    #[error("SigningClientError - RemoteCallFailure: {0}")]
    RemoteCallFailure(String),
    #[error("SigningClientError - Encode: {0}")]
    Encode(#[from] bitcoin::consensus::encode::Error),
    #[error("SigningClientError - Decode: {0}")]
    Decode(#[from] base64::DecodeError),
    #[error("SigningClientError - IO: {0}")]
    IO(#[from] std::io::Error),
}