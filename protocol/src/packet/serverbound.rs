use crate::cocoabeans::protocol::packet::packet_handler::IPacketHandler;
use crate::cocoabeans::protocol::stream_wrapper::MinecraftStream;

pub type ServerBoundPacketBuilder = fn(stream: &MinecraftStream, length: i32) -> ServerBoundPacket;

pub type ServerBoundPacket = Box<dyn ServerBoundPayload>;

pub trait ServerBoundPayload {
    fn handle(&self, listener: &mut dyn IPacketHandler);
}

pub mod handshaking {
    use crate::cocoabeans::protocol::packet::packet_handler::IPacketHandler;
    use crate::cocoabeans::protocol::packet::serverbound::ServerBoundPayload;
    use crate::cocoabeans::protocol::ConnectionState;

    // payloads

    pub struct HandshakePayload {
        pub protocol_version: i32,
        pub address: String,
        pub port: u16,
        pub next_state: ConnectionState,
    }

    impl ServerBoundPayload for HandshakePayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_handshaking_handshake(self);
        }
    }

    impl HandshakePayload {}
}

pub mod status {
    use crate::cocoabeans::protocol::packet::packet_handler::IPacketHandler;
    use crate::cocoabeans::protocol::packet::serverbound::ServerBoundPayload;

    // payloads

    pub struct RequestPayload {
        // no fields
    }

    impl ServerBoundPayload for RequestPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_status_request(self);
        }
    }

    pub struct PingPayload {
        payload: i64,
    }

    impl ServerBoundPayload for PingPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_status_ping(self);
        }
    }
}

pub mod login {
    use crate::cocoabeans::protocol::packet::packet_handler::IPacketHandler;
    use crate::cocoabeans::protocol::packet::serverbound::ServerBoundPayload;

    // payloads

    pub struct StartPayload {
        username: String,
    }

    impl ServerBoundPayload for StartPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_login_start(self);
        }
    }

    pub struct EncryptionResponsePayload {
        secret: Vec<u8>,
        verification_token: Vec<u8>,
    }

    impl ServerBoundPayload for EncryptionResponsePayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_login_encryption_response(self);
        }
    }

    pub struct PluginResponsePayload {
        message_id: i32,
        successful: bool,
        data: Vec<u8>,
    }

    impl ServerBoundPayload for PluginResponsePayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_login_plugin_response(self);
        }
    }
}

pub mod play {
    // payloads

    // TODO
}
