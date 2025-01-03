use common_data_bar::TimeResolution;
use trait_data_integration::{ProvideSymbolType, ProvideTimeResolutionType};
use crate::ImsBinanceDataIntegration;

// Specify associated types
impl<Context> ProvideSymbolType<Context> for ImsBinanceDataIntegration {
    type Symbol = String;
}

impl<Context> ProvideTimeResolutionType<Context> for ImsBinanceDataIntegration {
    type TimeResolution = TimeResolution;
}