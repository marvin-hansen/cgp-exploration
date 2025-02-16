use common_exchange::ExchangeID;
use sbe_messages_control::StopAllDataMessage;
use sbe_types::MessageType;

#[test]
fn test_new() {
    let client_id = 1;
    let exchange_id = ExchangeID::Kraken;
    let message = StopAllDataMessage::new(client_id, exchange_id);

    assert_eq!(message.message_type(), &MessageType::StopAllData);
    assert_eq!(message.client_id(), &1);
    assert_eq!(message.exchange_id(), &ExchangeID::Kraken);
}

#[test]
fn test_encode() {
    let client_id = 1;
    let exchange_id = ExchangeID::Kraken;
    let message = StopAllDataMessage::new(client_id, exchange_id);

    assert_eq!(message.message_type(), &MessageType::StopAllData);
    assert_eq!(message.client_id(), &1);
    assert_eq!(message.exchange_id(), &ExchangeID::Kraken);

    let enc = message.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 13);

    let expected: Vec<u8> = vec![5, 0, 203, 0, 1, 0, 1, 0, 203, 0, 1, 0, 1];
    let actual = buffer;

    assert_eq!(expected, actual);
}

#[test]
fn test_decode() {
    let encoded: Vec<u8> = vec![5, 0, 203, 0, 1, 0, 1, 0, 203, 0, 1, 0, 1];
    let buffer = encoded.as_slice();

    let message = StopAllDataMessage::from(buffer);
    assert_eq!(message.message_type(), &MessageType::StopAllData);
    assert_eq!(message.client_id(), &1);
    assert_eq!(message.exchange_id(), &ExchangeID::Kraken);
}

#[test]
fn test_message_type() {
    let client_id = 1;
    let exchange_id = ExchangeID::Kraken;
    let message = StopAllDataMessage::new(client_id, exchange_id);

    assert_eq!(message.message_type(), &MessageType::StopAllData);
}

#[test]
fn test_message_client_id() {
    let client_id = 1;
    let exchange_id = ExchangeID::Kraken;
    let message = StopAllDataMessage::new(client_id, exchange_id);

    assert_eq!(message.client_id(), &1);
}

#[test]
fn test_exchange_id() {
    let client_id = 1;
    let exchange_id = ExchangeID::Kraken;
    let message = StopAllDataMessage::new(client_id, exchange_id);

    assert_eq!(message.exchange_id(), &ExchangeID::Kraken);
}

#[test]
fn test_display() {
    let client_id = 1;
    let exchange_id = ExchangeID::Kraken;
    let message = StopAllDataMessage::new(client_id, exchange_id);

    let expected =
        "StopAllDataMessage[message_type: StopAllData, client_id: 1, exchange_id: kraken]";
    let actual = format!("{message}");
    assert_eq!(expected, actual);
}
