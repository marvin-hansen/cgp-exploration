use binance_core_data_integration::{UseBinanceSpotMainnetUrl, UseImsBinanceDataIntegration};
use cgp::core::error::*;
use cgp::prelude::*;
use deep_causality_macros::Constructor;
use error_data_integration::UseImsDataIntegrationError;
use trait_data_integration::*;

#[derive(Constructor, Default, Copy, Clone)]
pub struct ImsBinanceSpotDataIntegration {}

pub struct ImsBinanceSpotDataIntegrationComponents;

impl HasComponents for ImsBinanceSpotDataIntegration {
    type Components = ImsBinanceSpotDataIntegrationComponents;
}

delegate_components! {
    ImsBinanceSpotDataIntegrationComponents {
        ApiUrlComponent: UseBinanceSpotMainnetUrl,
        ErrorTypeComponent: UseImsDataIntegrationError,
        [
            // These are always the same for all Binance integrations
            BinanceIntegrationFieldsComponent:
            SymbolFetchComponent,
            SymbolValidatorComponent,
            OhlcvDataStreamComponent,
            TradeDataStreamComponent,
            SymbolTypeComponent,
            TimeResolutionTypeComponent,
        ]: UseImsBinanceDataIntegration,
    }
}
