use binance_spot_data_integration::*;
use std::fmt::Error;
use trait_data_integration::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let integration = ImsBinanceSpotDataIntegration::new();

    // This works
    let api_url = integration.api_url();
    println!("api_url: {}", api_url);
    let api_wss_url = integration.api_wss_url();
    println!("api_wss_url: {}", api_wss_url);

    // This doesn't work and throws an error
    // let res =binance_spot_data_integration::ImsBinanceSpotDataIntegration::fetch_exchange_symbols().await;
    let res = integration.fetch_exchange_symbols().await;
    if res.is_ok() {
        let symbols = res.unwrap();
        println!("symbols: {:#?}", symbols);
    } else {
        println!("Error: {}", res.unwrap_err());
    }

    Ok(())
}
