use crate::ImsDataClient;
use crate::client_trait::ImsDataClientTrait;
use crate::error::ImsClientError;
use async_trait::async_trait;
use common_data_bar::TimeResolution;
use sbe_types::DataType;

#[async_trait]
impl ImsDataClientTrait for ImsDataClient {
    /// Login to the IMS Data client
    ///
    /// This function will perform a login to the ImsDataClient.
    /// The username and password are taken from the configuration.
    ///
    /// Errors:
    /// * `ImsClientError`: A generic error type for theImsDataClient
    ///
    async fn login(&self) -> Result<(), ImsClientError> {
        self.client_login().await
    }
    /// Logout from the IMS Data client
    ///
    /// This function will perform a logout from the  IMS Data client
    ///
    /// Errors:
    /// * `ImsClientError`: A generic error type for the  IMS Data client
    ///
    async fn logout(&self) -> Result<(), ImsClientError> {
        self.client_logout().await
    }
    /// Start receiving trade data for the given symbol
    ///
    /// Parameters:
    /// * `symbol_id`: The id of the symbol for which to start receiving trade data
    ///
    /// Errors:
    /// * `ImsClientError`: A generic error type for the  IMS Data client
    ///
    async fn start_trade_data(&self, symbol_id: String) -> Result<(), ImsClientError> {
        self.client_start_trade_data(symbol_id).await
    }
    /// Start receiving OHLCV data for the given symbol
    ///
    /// Parameters:
    /// * `symbol_id`: The id of the symbol for which to start receiving OHLCV data
    /// * `time_resolution`: The time resolution of the OHLCV data
    ///
    /// Errors:
    /// * `ImsClientError`: A generic error type for the  IMS Data client
    ///
    async fn start_ohlcv_data(
        &self,
        symbol_id: String,
        time_resolution: TimeResolution,
    ) -> Result<(), ImsClientError> {
        self.client_start_ohlcv_data(symbol_id, time_resolution)
            .await
    }
    /// Stop receiving trade data for the given symbol
    ///
    /// Parameters:
    /// * `symbol_id`: The id of the symbol for which to stop receiving trade data
    ///
    /// Errors:
    /// * `ImsClientError`: A generic error type for the  IMS Data client
    ///
    async fn stop_trade_data(&self, symbol_id: String) -> Result<(), ImsClientError> {
        self.client_stop_data(symbol_id, DataType::TradeData).await
    }
    /// Stop receiving OHLCV data for the given symbol
    ///
    /// Parameters:
    /// * `symbol_id`: The id of the symbol for which to stop receiving OHLCV data
    ///
    /// Errors:
    /// * `ImsClientError`: A generic error type for the  IMS Data client
    ///
    async fn stop_ohlcv_data(&self, symbol_id: String) -> Result<(), ImsClientError> {
        self.client_stop_data(symbol_id, DataType::OHLCVData).await
    }
    /// Stop all data streams
    ///
    /// Errors:
    /// * `ImsClientError`: A generic error type for the  IMS Data client
    ///
    async fn stop_all_data(&self) -> Result<(), ImsClientError> {
        self.client_stop_all_data().await
    }

    /// Shutdown the IMS data client
    ///
    /// This will stop all data streams and shutdown the underlying Iggy client.
    ///
    /// Errors:
    /// * `ImsClientError`: A generic error type for the  IMS Data client
    ///
    async fn shutdown(&self) -> Result<(), ImsClientError> {
        self.client_shutdown().await
    }
}
