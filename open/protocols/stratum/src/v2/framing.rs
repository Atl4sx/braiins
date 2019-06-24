//! This module defines basic framing and all protocol message types

use packed_struct::prelude::*;
use packed_struct_codegen::PackedStruct;
use packed_struct_codegen::PrimitiveEnum_u8;

pub mod codec;

/// Header of the protocol message
#[derive(PackedStruct, Debug)]
#[packed_struct(endian = "lsb")]
pub struct Header {
    #[packed_field(size_bytes = "1", ty = "enum")]
    pub msg_type: MessageType,
    pub msg_length: Integer<u32, packed_bits::Bits24>,
}

impl Header {
    pub const SIZE: usize = 4;

    pub fn new(msg_type: MessageType, msg_length: usize) -> Header {
        assert!(msg_length <= 0xffffff, "Message too large");
        let msg_length = (msg_length as u32).into();

        Header {
            msg_type,
            msg_length,
        }
    }
}

/// All message recognized by the protocol
//#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
#[derive(PrimitiveEnum_u8, Clone, Copy, PartialEq, Eq, Debug)]
pub enum MessageType {
    SetupMiningConnection = 0x00,
    SetupMiningConnectionSuccess = 0x01,
    SetupMiningConnectionError = 0x02,
    OpenChannel = 0x03,
    OpenChannelSuccess = 0x04,
    OpenChannelError = 0x05,
    UpdateChannel = 0x06,
    UpdateChannelError = 0x07,
    NewMiningJob = 0x08,
    SetTarget = 0x09,
    SetNewPrevHash = 0x0a,
    SubmitShares = 0x0b,
    SubmitSharesSuccess = 0x0c,
    SubmitSharesError = 0x0d,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_header_pack() {
        let expected_bytes = [0x00u8, 0xcc, 0xbb, 0xaa];
        let header = Header::new(MessageType::SetupMiningConnection, 0xaabbcc);
        let header_bytes = header.pack();
        assert_eq!(
            expected_bytes, header_bytes,
            "Packing test header failed, message being \
             serialized: {:#08x?}",
            header
        );
    }

    /// This test relies on the fact that there is at least one message type identifier (0xff) is
    /// not used in the protocol.
    #[test]
    fn test_unknown_message_type() {
        let broken_header = [0xffu8, 0xaa, 0xbb, 0xcc];
        let header = Header::unpack_from_slice(&broken_header);
        assert!(
            header.is_err(),
            "Unpacking should have failed to non-existing header type, \
             parsed: {:#04x?}, sliced view {:#04x?}",
            header,
            broken_header
        );
    }
}