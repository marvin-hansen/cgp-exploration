use crate::ClientLogoutMessage;
use sbe_bindings::message_type::MessageType as SbeMessageType;
use sbe_bindings::{
    Encoder, WriteBuf, client_logout_codec::ClientLogoutEncoder, message_header_codec,
};
use sbe_types::SbeEncodeError;

impl ClientLogoutMessage {
    /// Encodes a `ClientLogoutMessage` to a byte buffer.
    ///
    /// # Arguments
    ///
    /// * `self` - `ClientLogoutMessage` to encode
    ///
    /// # Returns
    ///
    /// (usize, `Vec<u8>`) - Tuple of encoded size and byte buffer
    ///
    /// # Errors
    ///
    /// Returns Err if encoding fails
    ///
    /// # Process
    ///
    /// - Create a 12 byte buffer
    /// - Create default `ClientLogoutEncoder`
    /// - Wrap buffer in `WriteBuf`
    /// - Encode header
    /// - Encode `message_type`
    /// - Encode `client_id`
    /// - Return encoded size and buffer
    ///
    pub fn encode(&self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        // precise buffer size is 12 bytes for the entire message.
        let mut buffer = vec![0u8; 12];

        let mut csg = ClientLogoutEncoder::default();

        csg = csg.wrap(
            WriteBuf::new(buffer.as_mut_slice()),
            message_header_codec::ENCODED_LENGTH,
        );

        csg = csg.header(0).parent().expect("Failed to encode header");

        let value = SbeMessageType::from(self.message_type as u16);
        csg.message_type(value);

        let value = self.client_id;
        csg.client_id(value);

        let limit = csg.get_limit();
        Ok((limit, buffer))
    }
}
