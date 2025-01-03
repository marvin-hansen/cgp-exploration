use crate::UseImsBinanceDataIntegration;
use cgp::prelude::Async;
use common_data_bar::TimeResolution;
use trait_data_integration::{SymbolTypeProvider, TimeResolutionTypeProvider};

impl<Context> SymbolTypeProvider<Context> for UseImsBinanceDataIntegration
where
    Context: Async + Sync + Send,
{
    type Symbol = String;
}

impl<Context> TimeResolutionTypeProvider<Context> for UseImsBinanceDataIntegration
where
    Context: Async + Sync + Send,
{
    type TimeResolution = TimeResolution;
}
