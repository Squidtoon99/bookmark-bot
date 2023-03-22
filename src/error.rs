// use twilight_http::response::DeserializeBodyError;

use crate::verification::VerificationError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Environment variable '{0}' not found.")]
    EnvironmentVariableNotFound(String),

    #[error("Header '{0}' not found.")]
    HeaderNotFound(String),

    #[error("Failed to deserialize from or serialize to JSON.")]
    JsonFailed(#[from] serde_json::Error),

    #[error("Invalid payload provided: {0}.")]
    InvalidPayload(String),

    #[error("Verification failed.")]
    VerificationFailed(VerificationError),

    #[error("Interaction failed.")]
    InteractionFailed(InteractionError)
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum InteractionError {

    #[allow(dead_code)]
    #[error("Error communicating with {0}")]
    UpstreamError(String),

    #[error("Command not found {0}")]
    UnknownCommand(String),

    #[error("Something went wrong")]
    GenericError(),

    #[error("Cloudflare worker error: {0}")]
    WorkerError(String),

    #[error("Failed to get env variable ")]
    EnvVarError(#[from] std::env::VarError),

    #[error("Message serialization failed")]
    MessageSerializationError(#[from] twilight_validate::message::MessageValidationError),

    #[error("Message (de)serialization failed")]
    SerdeError(#[from] serde_json::Error),

    #[error("HTTP error")]
    HttpError(#[from] reqwest::Error),
}

impl From<worker::Error> for InteractionError {
    fn from(error: worker::Error) -> InteractionError {
        InteractionError::WorkerError(format!("{}", error))
    }
}