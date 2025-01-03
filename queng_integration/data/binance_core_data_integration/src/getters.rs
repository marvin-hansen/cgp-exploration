use crate::ImsBinanceDataIntegration;
use cgp::prelude::*;
use reqwest::Client;
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::time::Instant;

#[cgp_component {provider: BinanceIntegrationFieldsGetter,}]
pub(crate) trait HasBinanceIntegrationFields {
    fn http_client(&self) -> &Client;
    fn symbols_active_trade(&self) -> &RwLock<Vec<String>>;
    fn symbols_active_ohlcv(&self) -> &RwLock<Vec<String>>;
    fn symbol_cache(&self) -> &RwLock<Option<(HashSet<String>, Instant)>>;
    fn trade_handlers(&self) -> &RwLock<HashMap<String, JoinHandle<()>>>;
    fn ohlcv_handlers(&self) -> &RwLock<HashMap<String, JoinHandle<()>>>;
}

impl HasBinanceIntegrationFields for ImsBinanceDataIntegration {
    fn http_client(&self) -> &Client {
        &self.http_client
    }

    fn symbols_active_trade(&self) -> &RwLock<Vec<String>> {
        &self.symbols_active_trade
    }

    fn symbols_active_ohlcv(&self) -> &RwLock<Vec<String>> {
        &self.symbols_active_ohlcv
    }

    fn symbol_cache(&self) -> &RwLock<Option<(HashSet<String>, Instant)>> {
        &self.symbol_cache
    }

    fn trade_handlers(&self) -> &RwLock<HashMap<String, JoinHandle<()>>> {
        &self.trade_handlers
    }

    fn ohlcv_handlers(&self) -> &RwLock<HashMap<String, JoinHandle<()>>> {
        &self.ohlcv_handlers
    }
}
