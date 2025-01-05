use cgp::core::error::ProvideErrorType;
use std::error::Error;

pub struct UseImsDataIntegrationError;

impl<Context> ProvideErrorType<Context> for UseImsDataIntegrationError {
    type Error = ImsDataIntegrationError;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ImsDataIntegrationError {
    FailedToFetchSymbols(String),
    FailedToDeserializeJsonSymbols(String),
    FailedToExtractSymbolsFromResponse(String),
    FailedToValidateSymbols(String),
    SymbolNotFound(String),
}

impl Error for ImsDataIntegrationError {}

impl std::fmt::Display for ImsDataIntegrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImsDataIntegrationError::FailedToFetchSymbols(msg) => write!(
                f,
                "[ImsDataIntegrationError]: Failed to fetch symbols: {}",
                msg
            ),
            ImsDataIntegrationError::FailedToDeserializeJsonSymbols(msg) => write!(
                f,
                "[ImsDataIntegrationError]: Failed to deserialize json symbols: {}",
                msg
            ),
            ImsDataIntegrationError::FailedToExtractSymbolsFromResponse(msg) => write!(
                f,
                "[ImsDataIntegrationError]: Failed to extract symbols from response: {}",
                msg
            ),
            ImsDataIntegrationError::FailedToValidateSymbols(msg) => write!(
                f,
                "[ImsDataIntegrationError]: Failed to validate symbols: {}",
                msg
            ),
            ImsDataIntegrationError::SymbolNotFound(msg) => {
                write!(f, "[ImsDataIntegrationError]: Symbol not found: {}", msg)
            }
        }
    }
}
