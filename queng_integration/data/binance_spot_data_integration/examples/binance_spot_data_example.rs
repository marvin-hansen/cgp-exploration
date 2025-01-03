use std::fmt::Error;
use binance_spot_data_integration::*;
use trait_data_integration::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Example");

    let integration = ImsBinanceSpotDataIntegration::new();

    let api_url = integration.api_url();

    println!("api_url: {}", api_url);



    Ok(())
}
