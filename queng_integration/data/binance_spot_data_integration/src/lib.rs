use binance_core_data_integration::{UseBinanceSpotMainnetUrl, UseImsBinanceDataIntegration};
use cgp::prelude::*;
use trait_data_integration::*;

#[derive(Default, Copy, Clone)]
pub struct ImsBinanceSpotDataIntegration {}

impl ImsBinanceSpotDataIntegration {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct ImsBinanceSpotDataIntegrationComponents;

impl HasComponents for ImsBinanceSpotDataIntegration {
    type Components = ImsBinanceSpotDataIntegrationComponents;
}

delegate_components! {
    ImsBinanceSpotDataIntegrationComponents {
        ApiUrlComponent: UseBinanceSpotMainnetUrl,
        [
            // These are always the same for all Binance integrations
            SymbolFetchComponent,
            SymbolValidatorComponent,
            OhlcvDataStreamComponent,
            TradeDataStreamComponent,
            SymbolTypeComponent,
            TimeResolutionTypeComponent,
        ]: UseImsBinanceDataIntegration,
    }
}
