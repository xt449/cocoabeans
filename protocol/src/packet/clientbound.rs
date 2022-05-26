use crate::io::MinecraftWriter;
use crate::version::ProtocolVersion;

pub trait ClientBoundPacket {
    fn write_to(&self, stream: &mut MinecraftWriter, protocol_version: &dyn ProtocolVersion);
}

pub mod handshaking {
    // no client-bound handshaking packets
}

pub mod status {
    use serde_json::Value;

    use crate::io::MinecraftWriter;
    use crate::version::ProtocolVersion;

    use super::ClientBoundPacket;

    pub struct ResponsePacket {
        pub json_payload: Value,
    }

    impl ClientBoundPacket for ResponsePacket {
        fn write_to(&self, stream: &mut MinecraftWriter, protocol_version: &dyn ProtocolVersion) {
            stream.write_unsigned_byte(protocol_version.get_status_response_id());
            stream.write_json(&self.json_payload);
        }
    }

    pub struct PongPacket {
        pub payload: i64,
    }

    impl ClientBoundPacket for PongPacket {
        fn write_to(&self, stream: &mut MinecraftWriter, protocol_version: &dyn ProtocolVersion) {
            stream.write_unsigned_byte(protocol_version.get_status_pong_id());
            stream.write_long(self.payload);
        }
    }
}

pub mod login {
    use serde_json::Value;

    use crate::data::identifier::Identifier;
    use crate::io::MinecraftWriter;
    use crate::packet::clientbound::ClientBoundPacket;
    use crate::version::ProtocolVersion;

    pub struct DisconnectPacket {
        pub json_chat: Value,
    }

    impl ClientBoundPacket for DisconnectPacket {
        fn write_to(&self, stream: &mut MinecraftWriter, protocol_version: &dyn ProtocolVersion) {
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
        fn write_to(&self, stream: &mut MinecraftWriter, protocol_version: &dyn ProtocolVersion) {
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
        fn write_to(&self, stream: &mut MinecraftWriter, protocol_version: &dyn ProtocolVersion) {
            stream.write_unsigned_byte(protocol_version.get_status_pong_id());
            stream.write_uuid(self.uuid);
            stream.write_utf(&self.username);
        }
    }

    pub struct SetCompressionPacket {
        pub compression_threshold: i32,
    }

    impl ClientBoundPacket for SetCompressionPacket {
        fn write_to(&self, stream: &mut MinecraftWriter, protocol_version: &dyn ProtocolVersion) {
            stream.write_unsigned_byte(protocol_version.get_status_pong_id());
            stream.write_varint(self.compression_threshold);
        }
    }

    pub struct PluginRequestPacket {
        pub message_id: i32,
        pub identifier: Identifier,
        pub data: Vec<u8>,
    }

    impl ClientBoundPacket for PluginRequestPacket {
        fn write_to(&self, stream: &mut MinecraftWriter, protocol_version: &dyn ProtocolVersion) {
            stream.write_unsigned_byte(protocol_version.get_status_pong_id());
            stream.write_int(self.message_id);
            stream.write(&self.identifier);
            stream.write_byte_vec(&self.data);
        }
    }
}

pub mod play {
    // TODO
}
