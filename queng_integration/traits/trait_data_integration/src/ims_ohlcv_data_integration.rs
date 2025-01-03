use crate::{HasSymbolType, HasTimeResolutionType};
use cgp::prelude::*;

#[cgp_component {provider: OhlcvDataStreamProvider,}]
#[async_trait]
pub trait CanStreamOhlcvData: HasSymbolType + HasTimeResolutionType + HasErrorType {
    async fn start_ohlcv_data(
        &self,
        symbols: &[Self::Symbol],
        time_resolution: &Self::TimeResolution,
    ) -> Result<(), Self::Error>;

    async fn stop_ohlcv_data(&self, symbols: &[Self::Symbol]) -> Result<(), Self::Error>;

    async fn stop_all_ohlcv_data(&self) -> Result<(), Self::Error>;
}
