use crate::{HasSymbolType, HasTimeResolutionType};
use cgp::prelude::*;

#[async_trait]
#[cgp_component {
    name: TradeDataStarterComponent,
    provider: TradeDataStarter}]
pub trait CanStartTradeData: HasSymbolType + HasTimeResolutionType + HasErrorType {
    async fn start_trade_data(
        &self,
        symbols: &[Self::Symbol],
        time_resolution: &Self::TimeResolution,
    ) -> Result<(), Self::Error>;
}

#[async_trait]
#[cgp_component {
    name: TradeDataStopperComponent,
    provider: TradeDataStopper}]
pub trait CanStopTradeData: HasSymbolType + HasErrorType {
    async fn stop_trade_data(&self, symbols: &[Self::Symbol]) -> Result<(), Self::Error>;
}
