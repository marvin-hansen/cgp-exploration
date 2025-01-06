mod api_url;
mod context;
mod getters;
mod ohlcv_data_integration;
mod symbols_integration;
mod trade_data_integration;
mod types;
mod utils;
mod utils_connect;

use cgp::prelude::*;
use std::time::Duration;

pub use crate::api_url::*;
pub use crate::context::*;
pub use crate::getters::*;

/// Duration between symbol cache refreshes.
///
/// This value determines how often the symbol cache is refreshed from the Binance API.
/// The cache is used to validate symbols and map them to their corresponding OHLCV and trade
/// data streams.
pub(crate) const SYMBOL_CACHE_DURATION: Duration = Duration::from_secs(7200); // 120 minutes

/// Maximum time to wait between reconnect attempts.
///
/// When the WebSocket connection is lost, the integration will attempt to reconnect to the Binance
/// API. If the reconnection fails, it will wait for the specified duration before trying again.
pub(crate) const RECONNECT_INTERVAL: Duration = Duration::from_secs(12 * 3600); // 12 hours

/// Maximum number of reconnect attempts.
///
/// When the WebSocket connection is lost and the reconnection fails, the integration will attempt to
/// reconnect up to the specified number of times. If the maximum number of attempts is reached, the
/// integration will stop.
pub(crate) const MAX_RECONNECT_ATTEMPTS: u32 = 5;

/// Time to wait between reconnect attempts.
///
/// When the WebSocket connection is lost, the integration will wait for the specified duration
/// before attempting to reconnect.
pub(crate) const RECONNECT_DELAY: Duration = Duration::from_secs(5);

/// A Binance data integration implementation that provides real-time trade and OHLCV data streams.
///
/// This struct implements the `ImsDataIntegration` trait for the Binance cryptocurrency exchange.
/// It manages WebSocket connections for trade and OHLCV data streams, handles symbol validation,
/// and provides efficient caching of exchange information.
///
/// # Features
/// - Real-time trade data streaming via WebSocket
/// - Real-time OHLCV (candlestick) data streaming
/// - Symbol validation with caching
/// - Thread-safe connection management
/// - Automatic cleanup of terminated connections
///
#[derive(Clone)]
pub struct UseImsBinanceDataIntegration {}

pub struct ImsBinanceDataContextComponents;


delegate_components! {
    ImsBinanceDataContextComponents {
        BinanceIntegrationFieldsComponent: UseImsBinanceDataIntegration,
    }
}