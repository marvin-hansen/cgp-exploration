use trait_data_integration::ApiUrlProvider;

// SPOT MARKET
const SPOT_API_BASE_URL: &str = "https://api.binance.com/api/v3";
const SPOT_API_WSS_URL: &str = "wss://stream.binance.com:9443/ws";
// TESTNET API https://www.binance.com/en/support/faq/how-to-test-my-functions-on-binance-testnet-ab78f9a1b8824cf0a106b4229c76496d
const SPOT_TESTNET_API_BASE_URL: &str = "https://testnet.binance.vision/api/v3";
const SPOT_TESTNET_API_WSS_URL: &str = "wss://testnet.binance.vision/ws";
// USD FUTURES Binance USD-M Futures API endpoints https://binance-docs.github.io/apidocs/futures/en/#general-info
const USD_FUTURES_API_BASE_URL: &str = "https://fapi.binance.com/fapi/v1";
const USD_FUTURES_API_WSS_URL: &str = "wss://fstream.binance.com/ws";
// TESTNET API https://developers.binance.com/docs/derivatives/usds-margined-futures/general-info
const USD_FUTURES_TESTNET_API_BASE_URL: &str = "https://testnet.binancefuture.com/api/v3";
const USD_FUTURES_TESTNET_API_WSS_URL: &str = "wss://stream.binancefuture.com";
// COIN FUTURES Binance Coin-M Futures API endpoints https://binance-docs.github.io/apidocs/delivery/en/#basis
const COIN_FUTURES_API_BASE_URL: &str = "https://dapi.binance.com/dapi/v1";
const COIN_FUTURES_API_WSS_URL: &str = "wss://dstream.binance.com/ws";
// TESTNET API https://developers.binance.com/docs/derivatives/coin-margined-futures/general-info
const COIN_FUTURES_TESTNET_API_BASE_URL: &str = "https://testnet.binancefuture.com/api/v3";
const COIN_FUTURES_TESTNET_API_WSS_URL: &str = "wss://dstream.binancefuture.com";

/// Use Binance Spot Market mainnet API endpoints.
///
/// This struct implements the `ApiUrlProvider` trait for the Binance Spot Market mainnet API endpoints.
/// It provides the base URL for the API and the WebSocket URL for the Spot Market API.
///
/// # Notes
/// - The API endpoints are subject to rate limits. Consult the
///   [Binance API documentation](https://binance-docs.github.io/apidocs/spot/en/#how-to-get-started)
///   for more information.
pub struct UseBinanceSpotMainnetUrl;
impl<Context> ApiUrlProvider<Context> for UseBinanceSpotMainnetUrl {
    fn api_url(_context: &Context) -> &str {
        SPOT_API_BASE_URL
    }
    fn api_wss_url(_context: &Context) -> &str {
        SPOT_API_WSS_URL
    }
}

/// Use Binance Spot Market testnet API endpoints.
///
/// This struct implements the `ApiUrlProvider` trait for the Binance Spot Market testnet API endpoints.
/// It provides the base URL for the API and the WebSocket URL for the Spot Market API.
///
/// # Notes
/// - The API endpoints are subject to rate limits. Consult the
///   [Binance API documentation](https://binance-docs.github.io/apidocs/spot/en/#how-to-get-started)
///   for more information.
pub struct UseBinanceSpotTestnetUrl;
impl<Context> ApiUrlProvider<Context> for UseBinanceSpotTestnetUrl {
    fn api_url(_context: &Context) -> &str {
        SPOT_TESTNET_API_BASE_URL
    }
    fn api_wss_url(_context: &Context) -> &str {
        SPOT_TESTNET_API_WSS_URL
    }
}

/// Use Binance USD-M Futures mainnet API endpoints.
///
/// This struct implements the `ApiUrlProvider` trait for the Binance USD-M Futures mainnet API endpoints.
/// It provides the base URL for the API and the WebSocket URL for the USD-M Futures API.
///
/// # Notes
/// - The API endpoints are subject to rate limits. Consult the
///   [Binance API documentation](https://binance-docs.github.io/apidocs/futures/en/#how-to-get-started)
///   for more information.
pub struct UseBinanceUsdFuturesMainnetUrl;
impl<Context> ApiUrlProvider<Context> for UseBinanceUsdFuturesMainnetUrl {
    fn api_url(_context: &Context) -> &str {
        USD_FUTURES_API_BASE_URL
    }
    fn api_wss_url(_context: &Context) -> &str {
        USD_FUTURES_API_WSS_URL
    }
}
/// Use Binance USD-M Futures testnet API endpoints.
///
/// This struct implements the `ApiUrlProvider` trait for the Binance USD-M Futures testnet API endpoints.
/// It provides the base URL for the API and the WebSocket URL for the USD-M Futures API.
///
/// # Notes
/// - The API endpoints are subject to rate limits. Consult the
///   [Binance API documentation](https://binance-docs.github.io/apidocs/futures/en/#how-to-get-started)
///   for more information.
pub struct UseBinanceUsdFuturesTestnetUrl;
impl<Context> ApiUrlProvider<Context> for UseBinanceUsdFuturesTestnetUrl {
    fn api_url(_context: &Context) -> &str {
        USD_FUTURES_TESTNET_API_BASE_URL
    }
    fn api_wss_url(_context: &Context) -> &str {
        USD_FUTURES_TESTNET_API_WSS_URL
    }
}

/// Use Binance Coin-M Futures mainnet API endpoints.
///
/// This struct implements the `ApiUrlProvider` trait for the Binance Coin-M Futures mainnet API endpoints.
/// It provides the base URL for the API and the WebSocket URL for the Coin-M Futures API.
///
/// # Notes
/// - The API endpoints are subject to rate limits. Consult the
///   [Binance API documentation](https://binance-docs.github.io/apidocs/delivery/en/#basis)
///   for more information.
pub struct UseBinanceCoinFuturesMainnetUrl;
impl<Context> ApiUrlProvider<Context> for UseBinanceCoinFuturesMainnetUrl {
    fn api_url(_context: &Context) -> &str {
        COIN_FUTURES_API_BASE_URL
    }
    fn api_wss_url(_context: &Context) -> &str {
        COIN_FUTURES_API_WSS_URL
    }
}

/// Use Binance Coin-M Futures testnet API endpoints.
///
/// This struct implements the `ApiUrlProvider` trait for the Binance Coin-M Futures testnet API endpoints.
/// It provides the base URL for the API and the WebSocket URL for the Coin-M Futures API.
///
/// # Notes
/// - The API endpoints are subject to rate limits. Consult the
///   [Binance API documentation](https://binance-docs.github.io/apidocs/delivery/en/#basis)
///   for more information.
pub struct UseBinanceCoinFuturesTestnetUrl;
impl<Context> ApiUrlProvider<Context> for UseBinanceCoinFuturesTestnetUrl {
    fn api_url(_context: &Context) -> &str {
        COIN_FUTURES_TESTNET_API_BASE_URL
    }
    fn api_wss_url(_context: &Context) -> &str {
        COIN_FUTURES_TESTNET_API_WSS_URL
    }
}
