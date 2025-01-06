use crate::UseImsBinanceDataIntegration;
use cgp::prelude::*;
use reqwest::Client;
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::ops::Deref;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::time::Instant;

#[cgp_component {
    name: BinanceIntegrationFieldsComponent,
    provider: BinanceIntegrationFieldsProvider,
    }]
pub trait HasBinanceIntegrationFields {
    fn http_client(&self) -> &Client;
    fn symbols_active_trade(&self) -> &RwLock<Vec<String>>;
    fn symbols_active_ohlcv(&self) -> &RwLock<Vec<String>>;
    fn symbol_cache(&self) -> &RwLock<Option<(HashSet<String>, Instant)>>;
    fn trade_handlers(&self) -> &RwLock<HashMap<String, JoinHandle<()>>>;
    fn ohlcv_handlers(&self) -> &RwLock<HashMap<String, JoinHandle<()>>>;
}

impl<Context> BinanceIntegrationFieldsProvider<Context> for UseImsBinanceDataIntegration
where
    Context: Deref,
    Context: HasField<symbol!("http_client"), Value = Client>,
    Context: HasField<symbol!("symbols_active_trade"), Value =  RwLock<Vec<String>>>,
    Context: HasField<symbol!("symbols_active_ohlcv"), Value =  RwLock<Vec<String>>>,
    Context: HasField<symbol!("symbol_cache"), Value =  RwLock<Option<(HashSet<String>, Instant)>>>,
    Context: HasField<symbol!("trade_handlers"), Value =  RwLock<HashMap<String, JoinHandle<()>>>>,
    Context: HasField<symbol!("ohlcv_handlers"), Value =  RwLock<HashMap<String, JoinHandle<()>>>>,
{
    fn http_client(context: &Context) -> &Client {
        context.get_field(PhantomData::<symbol!("http_client")>)
    }

    fn symbols_active_trade(context: &Context) -> &RwLock<Vec<String>> {
        context.get_field(PhantomData::<symbol!("symbols_active_trade")>)
    }

    fn symbols_active_ohlcv(context: &Context) -> &RwLock<Vec<String>> {
        context.get_field(PhantomData::<symbol!("symbols_active_ohlcv")>)
    }

    fn symbol_cache(context: &Context) -> &RwLock<Option<(HashSet<String>, Instant)>> {
        context.get_field(PhantomData::<symbol!("symbol_cache")>)
    }

    fn trade_handlers(context: &Context) -> &RwLock<HashMap<String, JoinHandle<()>>> {
        context.get_field(PhantomData::<symbol!("trade_handlers")>)
    }

    fn ohlcv_handlers(context: &Context) -> &RwLock<HashMap<String, JoinHandle<()>>> {
        context.get_field(PhantomData::<symbol!("ohlcv_handlers")>)
    }
}
