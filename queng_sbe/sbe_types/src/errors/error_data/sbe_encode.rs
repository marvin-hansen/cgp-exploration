use crate::{DataErrorMessage, SbeEncodeError};
use sbe_bindings::message_type::MessageType as SbeMessageType;
use sbe_bindings::{data_error_codec::DataErrorEncoder, message_header_codec, Encoder, WriteBuf};

impl DataErrorMessage {
    /// Encodes a `DataErrorMessage` to a byte buffer.
    ///
    /// # Arguments
    ///
    /// * `self` - `DataErrorMessage` to encode
    ///
    /// # Returns
    ///
    /// (usize, `Vec<u8>`) - Tuple containing encoded size and byte buffer
    ///
    /// # Errors
    ///
    /// Returns Err if encoding fails
    ///
    /// # Process
    ///
    /// - Create 13 byte buffer
    /// - Create default `DataErrorEncoder`
    /// - Wrap buffer in `WriteBuf`
    /// - Encode header
    /// - Encode `message_type`
    /// - Encode `client_id`
    /// - Encode `data_error_type`
    /// - Return encoded size and buffer
    ///
    pub fn encode(&self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        let mut buffer = vec![0u8; 13];

        let mut csg = DataErrorEncoder::default();

        csg = csg.wrap(
            WriteBuf::new(buffer.as_mut_slice()),
            message_header_codec::ENCODED_LENGTH,
        );

        csg = csg.header(0).parent().expect("Failed to encode header");

        let value = SbeMessageType::from(self.message_type as u16);
        csg.message_type(value);

        let value = self.client_id;
        csg.client_id(value);

        let value = self.data_error_type as u8;
        csg.data_error_type(value);

        let limit = csg.get_limit();
        Ok((limit, buffer))
    }
}
