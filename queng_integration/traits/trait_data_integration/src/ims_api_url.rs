use cgp::prelude::*;

/// Component that provides API URL
#[cgp_component {
    name: ApiUrlComponent,
    provider: ApiUrlProvider,}]
pub trait HasApiUrl {
    /// Get API URL
    fn api_url(&self) -> &str;
    /// Get API WebSocket URL
    fn api_wss_url(&self) -> &str;
}
