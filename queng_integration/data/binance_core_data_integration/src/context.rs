use cgp::prelude::*;
use reqwest::Client;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::time::Instant;
use crate::ImsBinanceDataContextComponents;

impl HasComponents for ImsBinanceDataContext {
    type Components = ImsBinanceDataContextComponents;
}

#[derive(Clone)]
pub struct ImsBinanceDataContext {
    pub fields: Arc<ImsBinanceDataContextFields>,
}

impl ImsBinanceDataContext {
    pub fn new() -> Self {
        Self {
            fields: Arc::new(ImsBinanceDataContextFields::new()),
        }
    }
}

impl Deref for ImsBinanceDataContext {
    type Target = ImsBinanceDataContextFields;
    fn deref(&self) -> &ImsBinanceDataContextFields {
        &self.fields
    }
}

#[derive(HasField, Default)]
pub struct ImsBinanceDataContextFields {
    http_client: Client,
    symbols_active_trade: RwLock<Vec<String>>,
    symbols_active_ohlcv: RwLock<Vec<String>>,
    symbol_cache: RwLock<Option<(HashSet<String>, Instant)>>,
    trade_handlers: RwLock<HashMap<String, JoinHandle<()>>>,
    ohlcv_handlers: RwLock<HashMap<String, JoinHandle<()>>>,
}

impl ImsBinanceDataContextFields {
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
            symbols_active_trade: RwLock::new(Vec::with_capacity(50)),
            symbols_active_ohlcv: RwLock::new(Vec::with_capacity(50)),
            symbol_cache: RwLock::new(None),
            trade_handlers: RwLock::new(HashMap::with_capacity(50)),
            ohlcv_handlers: RwLock::new(HashMap::with_capacity(50)),
        }
    }
}
