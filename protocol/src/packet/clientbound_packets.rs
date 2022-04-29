use super::super::stream_wrapper::MinecraftStream;
use super::versions::ProtocolVersion;

pub trait ClientBoundPacket {
    fn write_to(&self, stream: &mut MinecraftStream, protocol_version: &dyn ProtocolVersion);
}

pub mod handshaking {
    // no client-bound handshaking packets
}

pub mod status {
    use cocoabeans::macros::json::Json;
    use super::ClientBoundPacket;
    use super::super::stream_wrapper::MinecraftStream;
    use super::super::versions::ProtocolVersion;

    pub struct ResponsePacket {
        pub json_payload: Json,
    }

    impl ClientBoundPacket for ResponsePacket {
        #[allow(unused_must_use)]
        fn write_to(&self, stream: &mut MinecraftStream, protocol_version: &dyn ProtocolVersion) {
            stream.write_unsigned_byte(protocol_version.get_status_response_id());
            stream.write_json(&self.json_payload);
        }
    }

    pub struct PongPacket {
        pub payload: i64,
    }

    impl ClientBoundPacket for PongPacket {
        #[allow(unused_must_use)]
        fn write_to(&self, stream: &mut MinecraftStream, protocol_version: &dyn ProtocolVersion) {
            stream.write_unsigned_byte(protocol_version.get_status_pong_id());
            stream.write_long(self.payload);
        }
    }
}

pub mod login {
    use crate::cocoabeans::json_macros::Json;
    use crate::cocoabeans::protocol::packet::clientbound_packets::ClientBoundPacket;
    use crate::cocoabeans::protocol::stream_wrapper::MinecraftStream;
    use crate::cocoabeans::protocol::versions::ProtocolVersion;

    pub struct DisconnectPacket {
        pub json_chat: Json,
    }

    impl ClientBoundPacket for DisconnectPacket {
        #[allow(unused_must_use)]
        fn write_to(&self, stream: &mut MinecraftStream, protocol_version: &dyn ProtocolVersion) {
            stream.write_unsigned_byte(protocol_version.get_status_pong_id());
            stream.write_json(&self.json_chat);
        }
    }

    pub struct EncryptionRequestPacket {
        pub server_id: String,
        pub key: Vec<u8>,
        pub verification_token: Vec<u8>,
    }

    impl ClientBoundPacket for EncryptionRequestPacket {
        #[allow(unused_must_use)]
        fn write_to(&self, stream: &mut MinecraftStream, protocol_version: &dyn ProtocolVersion) {
            stream.write_unsigned_byte(protocol_version.get_status_pong_id());
            stream.write_utf(&self.server_id);
            stream.write_byte_vec(&self.key);
            stream.write_byte_vec(&self.verification_token);
        }
    }

    pub struct SuccessPacket {
        pub uuid: u128,
        pub username: String,
    }

    impl ClientBoundPacket for SuccessPacket {
        #[allow(unused_must_use)]
        fn write_to(&self, stream: &mut MinecraftStream, protocol_version: &dyn ProtocolVersion) {
            stream.write_unsigned_byte(protocol_version.get_status_pong_id());
            stream.write_uuid(self.uuid);
            stream.write_utf(&self.username);
        }
    }

    pub struct SetCompressionPacket {
        pub compression_threshold: i32,
    }

    impl ClientBoundPacket for SetCompressionPacket {
        #[allow(unused_must_use)]
        fn write_to(&self, stream: &mut MinecraftStream, protocol_version: &dyn ProtocolVersion) {
            stream.write_unsigned_byte(protocol_version.get_status_pong_id());
            stream.write_varint(self.compression_threshold);
        }
    }

    pub struct PluginRequestPacket {
        pub message_id: i32,
        pub identifier: String,
        pub data: Vec<u8>,
    }

    impl ClientBoundPacket for PluginRequestPacket {
        #[allow(unused_must_use)]
        fn write_to(&self, stream: &mut MinecraftStream, protocol_version: &dyn ProtocolVersion) {
            stream.write_unsigned_byte(protocol_version.get_status_pong_id());
            stream.write_int(self.message_id);
            stream.write_utf(&self.identifier);
            stream.write_byte_vec(&self.data);
        }
    }
}

pub mod play {
    // TODO
}
