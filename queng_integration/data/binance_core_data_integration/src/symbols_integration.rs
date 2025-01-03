use crate::getters::HasBinanceIntegrationFields;
use crate::{ImsBinanceDataIntegration, SYMBOL_CACHE_DURATION};
use cgp::core::Async;
use cgp::prelude::HasErrorType;
use serde_json::Value;
use std::collections::HashSet;
use tokio::time::Instant;
use error_data_integration::ImsDataIntegrationError;
use trait_data_integration::{CanFetchExchangeSymbols, HasApiUrl, HasSymbolType, SymbolFetcher, SymbolValidator};

impl<Context> SymbolFetcher<Context> for ImsBinanceDataIntegration
where
    Context: HasSymbolType<Symbol = String>
        + HasErrorType<Error= ImsDataIntegrationError>
        + HasBinanceIntegrationFields
        + HasApiUrl
        + Async
        + Sync
        + Send,
{
    /// Fetches a list of valid trading symbols from the Binance API.
    ///
    /// This function will first check the cache to see if the symbols are already
    /// available. If the cache is stale or doesn't exist, it will fetch the
    /// symbols from the API and update the cache.
    ///
    /// The API endpoint is [GET /api/v3/exchangeInfo](https://binance-docs.github.io/apidocs/spot/en/#how-to-get-all-symbols-information-user-data-streams).
    ///
    /// # Errors
    ///
    /// This function will return an error if the API request fails or if
    /// the response can't be deserialized into JSON.
    ///
    async fn fetch_exchange_symbols(
        context: &Context,
    ) -> Result<HashSet<Context::Symbol>, Context::Error> {
        // Check cache first
        if let Some((symbols, timestamp)) = &*context.symbol_cache().read().await {
            if timestamp.elapsed() < SYMBOL_CACHE_DURATION {
                return Ok(symbols.clone());
            }
        }

        // Cache is stale or doesn't exist, fetch symbols from API
        let url = format!("{}/exchangeInfo", context.api_url());
        let response =
            context.http_client().get(&url).send().await.map_err(|e| {
                ImsDataIntegrationError::FailedToFetchSymbols(e.to_string()).into()
            })?;

        let data: Value = response
            .json()
            .await
            .map_err(|e| ImsDataIntegrationError::FailedToDeserializeJsonSymbols(e.to_string()).into())?;


        let symbols = data["symbols"]
            .as_array()
            .ok_or_else(|| ImsDataIntegrationError::FailedToExtractSymbolsFromResponse("".to_string()).into())?
            .iter()
            .filter_map(|s| s["symbol"].as_str().map(String::from))
            .collect::<HashSet<_>>();

        // Update cache
        *context.symbol_cache().write().await = Some((symbols.clone(), Instant::now()));

        Ok(symbols)
    }
}

impl<Context> SymbolValidator<Context> for ImsBinanceDataIntegration
where
    Context: HasSymbolType<Symbol = String>
    + HasErrorType<Error= ImsDataIntegrationError>
    + HasBinanceIntegrationFields
    + CanFetchExchangeSymbols
    + Async
    + Sync
    + Send,
{
    /// Validates a list of trading symbols against Binance's supported symbols.
    ///
    /// This method:
    /// 1. Retrieves the current list of valid symbols (using cache when possible)
    /// 2. Checks each input symbol against the valid symbol list
    /// 3. Returns an error if any symbols are invalid
    ///
    /// # Arguments
    /// * `symbols` - List of symbols to validate
    ///
    /// # Returns
    /// - `Ok(true)`: If all symbols are valid
    /// - `Err(MessageProcessingError)`: If any symbols are invalid, with error message listing invalid symbols
    ///
    async fn validate_symbols( context: &Context, symbols: &[Context::Symbol]) -> Result<bool, Context::Error>{

        let valid_symbols = match Context::fetch_exchange_symbols(context).await{
            Ok(valid_symbols) => valid_symbols,
            Err(e) => return Err(e),
        };

        let invalid_symbols: Vec<_> = symbols
            .iter()
            .filter(|s| !valid_symbols.contains(*s))
            .collect();

        if !invalid_symbols.is_empty() {
            return Err(ImsDataIntegrationError::FailedToValidateSymbols(format!(
                "The following symbols are invalid and not traded on Binance: {:?}",
                invalid_symbols
            )));
        }

        Ok(true)
    }
}
