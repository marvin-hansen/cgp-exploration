use trait_data_integration::ApiUrlGetter;

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

pub struct UseBinanceSpotMainnetUrl;
impl<Context> ApiUrlGetter<Context> for UseBinanceSpotMainnetUrl {
    fn api_url(_context: &Context) -> &str {
        SPOT_API_BASE_URL
    }

    fn api_wss_url(_context: &Context) -> &str {
        SPOT_API_WSS_URL
    }
}

pub struct UseBinanceSpotTestnetUrl;
impl<Context> ApiUrlGetter<Context> for UseBinanceSpotTestnetUrl {
    fn api_url(_context: &Context) -> &str {
        SPOT_TESTNET_API_BASE_URL
    }

    fn api_wss_url(_context: &Context) -> &str {
        SPOT_TESTNET_API_WSS_URL
    }
}

pub struct UseBinanceUsdFuturesMainnetUrl;
impl<Context> ApiUrlGetter<Context> for UseBinanceUsdFuturesMainnetUrl {
    fn api_url(_context: &Context) -> &str {
        USD_FUTURES_API_BASE_URL
    }

    fn api_wss_url(_context: &Context) -> &str {
        USD_FUTURES_API_WSS_URL
    }
}

pub struct UseBinanceUsdFuturesTestnetUrl;
impl<Context> ApiUrlGetter<Context> for UseBinanceUsdFuturesTestnetUrl {
    fn api_url(_context: &Context) -> &str {
        USD_FUTURES_TESTNET_API_BASE_URL
    }

    fn api_wss_url(_context: &Context) -> &str {
        USD_FUTURES_TESTNET_API_WSS_URL
    }
}

pub struct UseBinanceCoinFuturesMainnetUrl;
impl<Context> ApiUrlGetter<Context> for UseBinanceCoinFuturesMainnetUrl {
    fn api_url(_context: &Context) -> &str {
        COIN_FUTURES_API_BASE_URL
    }

    fn api_wss_url(_context: &Context) -> &str {
        COIN_FUTURES_API_WSS_URL
    }
}

pub struct UseBinanceCoinFuturesTestnetUrl;
impl<Context> ApiUrlGetter<Context> for UseBinanceCoinFuturesTestnetUrl {
    fn api_url(_context: &Context) -> &str {
        COIN_FUTURES_TESTNET_API_BASE_URL
    }

    fn api_wss_url(_context: &Context) -> &str {
        COIN_FUTURES_TESTNET_API_WSS_URL
    }
}
