use binance_core_data_integration::{UseBinanceSpotMainnetUrl, UseImsBinanceDataIntegration};
use cgp::prelude::*;
use trait_data_integration::{
    ApiUrlComponent, OhlcvDataStreamComponent, SymbolFetchComponent, SymbolTypeComponent,
    SymbolValidatorComponent, TimeResolutionTypeComponent, TradeDataStreamComponent,
};

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
        // These are always the same for Binance
        SymbolFetchComponent: UseImsBinanceDataIntegration,
        SymbolValidatorComponent: UseImsBinanceDataIntegration,
        OhlcvDataStreamComponent: UseImsBinanceDataIntegration,
        TradeDataStreamComponent: UseImsBinanceDataIntegration,
        SymbolTypeComponent: UseImsBinanceDataIntegration,
        TimeResolutionTypeComponent: UseImsBinanceDataIntegration,
    }
}
