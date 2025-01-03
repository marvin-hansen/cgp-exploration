use crate::{HasSymbolType, HasTimeResolutionType};
use cgp::prelude::*;

#[async_trait]
#[cgp_component {provider: TradeDataStreamProvider}]
pub trait CanStreamTradeData: HasSymbolType + HasTimeResolutionType + HasErrorType {
    async fn start_trade_data(
        &self,
        symbols: &[Self::Symbol],
    ) -> Result<(), Self::Error>;

    async fn stop_trade_data(&self, symbols: &[Self::Symbol]) -> Result<(), Self::Error>;

    async fn stop_all_trade_data(&self) -> Result<(), Self::Error>;
}
