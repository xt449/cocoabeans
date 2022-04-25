use crate::cocoabeans::protocol::packet::packet_listener::PacketListener;

pub struct ServerBoundPacket {
    payload: dyn ServerBoundPayload,
}

impl ServerBoundPacket {
    fn handle(&self, listener: &mut PacketListener) {
        self.handle(listener);
    }
}

pub trait ServerBoundPayload {
    fn handle(&self, listener: &mut PacketListener);
}

pub mod handshaking {
    use crate::cocoabeans::protocol::packet::packet_listener::{IPacketListener, PacketListener};
    use crate::cocoabeans::protocol::packet::serverbound_packets::ServerBoundPayload;
    use crate::cocoabeans::protocol::types;

    // payloads

    pub struct HandshakePayload {
        pub protocol_version: types::VarInt,
        pub address: String,
        pub port: u16,
        pub next_state: types::VarInt,
    }

    impl ServerBoundPayload for HandshakePayload {
        fn handle(&self, listener: &mut PacketListener) {
            listener.handle_handshaking_handshake(self);
        }
    }
}

pub mod status {
    use crate::cocoabeans::protocol::packet::packet_listener::{IPacketListener, PacketListener};
    use crate::cocoabeans::protocol::packet::serverbound_packets::ServerBoundPayload;

    // payloads

    pub struct RequestPayload {
        // no fields
    }

    impl ServerBoundPayload for RequestPayload {
        fn handle(&self, listener: &mut PacketListener) {
            listener.handle_status_request(self);
        }
    }

    pub struct PingPayload {
        payload: i64,
    }

    impl ServerBoundPayload for PingPayload {
        fn handle(&self, listener: &mut PacketListener) {
            listener.handle_status_ping(self);
        }
    }
}

pub mod login {
    use bytes::Bytes;

    use crate::cocoabeans::protocol::packet::packet_listener::{IPacketListener, PacketListener};
    use crate::cocoabeans::protocol::packet::serverbound_packets::ServerBoundPayload;

    // payloads

    pub struct StartPayload {
        username: String,
    }

    impl ServerBoundPayload for StartPayload {
        fn handle(&self, listener: &mut PacketListener) {
            listener.handle_login_start(self);
        }
    }

    pub struct EncryptionResponsePayload {
        secret: Bytes,
        verification_token: Bytes,
    }

    impl ServerBoundPayload for EncryptionResponsePayload {
        fn handle(&self, listener: &mut PacketListener) {
            listener.handle_login_encryption_response(self);
        }
    }

    pub struct PluginResponsePayload {
        message_id: i32,
        successful: bool,
        data: Bytes,
    }

    impl ServerBoundPayload for PluginResponsePayload {
        fn handle(&self, listener: &mut PacketListener) {
            listener.handle_login_plugin_response(self);
        }
    }
}

pub mod play {
    // payloads

    // TODO
}
