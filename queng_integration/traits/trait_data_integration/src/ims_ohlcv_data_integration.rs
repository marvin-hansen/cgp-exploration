use crate::{HasSymbolType, HasTimeResolutionType};
use cgp::prelude::*;

#[async_trait]
#[cgp_component {
    name: OhlcvDataStarterComponent,
    provider: OhlcvDataStarter,}]
pub trait CanStartOhlcvData: HasSymbolType + HasTimeResolutionType + HasErrorType {
    async fn start_ohlcv_data(
        &self,
        symbols: &[Self::Symbol],
        time_resolution: &Self::TimeResolution,
    ) -> Result<(), Self::Error>;
}

#[async_trait]
#[cgp_component {
    name: OhlcvDataStopperComponent,
    provider: OhlcvDataStopper,}]
pub trait CanStopOhlcvData: HasSymbolType + HasErrorType {
    async fn stop_ohlcv_data(&self, symbols: &[Self::Symbol]) -> Result<(), Self::Error>;
}
