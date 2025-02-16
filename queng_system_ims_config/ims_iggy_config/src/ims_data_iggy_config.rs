use common_ims::ExchangeID;
use iggy::stream_builder::IggyStreamConfig;
use iggy::utils::duration::IggyDuration;
use std::str::FromStr;

pub fn ims_data_iggy_config(client_id: u16, exchange_id: ExchangeID) -> IggyStreamConfig {
    IggyStreamConfig::from_stream_topic(
        format!("{}-data-client-{}", exchange_id, client_id).as_str(),
        format!("{}-data-topic", exchange_id).as_str(),
        100,
        IggyDuration::from_str("1ms").unwrap(),
        IggyDuration::from_str("1ms").unwrap(),
    )
    .expect("Failed to create iggy config")
}
