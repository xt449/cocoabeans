use crate::cocoabeans::protocol::packet::packet_listener::PacketListener;

pub enum ServerBoundPacket<'t> {
    Handshaking { payload: &'t dyn ServerBound },
    Status { payload: &'t dyn ServerBound },
    Login { payload: &'t dyn ServerBound },
    Play { payload: &'t dyn ServerBound },
}

impl ServerBoundPacket<'static> {
    fn handle(&self, listener: &mut PacketListener) {
        match self {
            ServerBoundPacket::Handshaking { payload } | ServerBoundPacket::Status { payload } => {
                payload.handle(listener);
            }
            //=> {payload.handle(listener);}
            ServerBoundPacket::Login { payload } => {
                payload.handle(listener);
            }
            ServerBoundPacket::Play { payload } => {
                payload.handle(listener);
            }
        }
    }
}

pub trait ServerBound {
    fn handle(&self, listener: &mut PacketListener);
}

pub mod handshaking {
    use crate::cocoabeans::protocol::packet::packet_listener::{IPacketListener, PacketListener};
    use crate::cocoabeans::protocol::packet::serverbound_packets::ServerBound;
    use crate::cocoabeans::protocol::types;

    pub struct HandshakingPacket {
        pub payload: dyn ServerBound,
    }

    impl ServerBound for HandshakingPacket {
        fn handle(&self, listener: &mut PacketListener) {
            self.payload.handle(listener);
        }
    }

    pub struct HandshakePayload {
        pub protocol_version: types::VarInt,
        pub address: String,
        pub port: u16,
        pub next_state: types::VarInt,
    }

    impl ServerBound for HandshakePayload {
        fn handle(&self, listener: &mut PacketListener) {
            listener.handle_handshaking_handshake(self);
        }
    }
}

pub mod status {
    use crate::cocoabeans::protocol::packet::packet_listener::{IPacketListener, PacketListener};
    use crate::cocoabeans::protocol::packet::serverbound_packets::ServerBound;

    pub struct StatusPacket {
        payload: dyn ServerBound,
    }

    impl ServerBound for StatusPacket {
        fn handle(&self, listener: &mut PacketListener) {
            self.payload.handle(listener);
        }
    }

    // payloads

    pub struct RequestPayload {
        // no fields
    }

    impl ServerBound for RequestPayload {
        fn handle(&self, listener: &mut PacketListener) {
            listener.handle_status_request(self);
        }
    }

    pub struct PingPayload {
        payload: i64,
    }

    impl ServerBound for PingPayload {
        fn handle(&self, listener: &mut PacketListener) {
            listener.handle_status_ping(self);
        }
    }
}

pub mod login {
    use crate::cocoabeans::protocol::packet::packet_listener::{IPacketListener, PacketListener};
    use crate::cocoabeans::protocol::packet::serverbound_packets::ServerBound;
    use bytes::Bytes;

    pub struct LoginPacket {
        pub payload: dyn ServerBound,
    }

    impl ServerBound for LoginPacket {
        fn handle(&self, listener: &mut PacketListener) {
            self.payload.handle(listener);
        }
    }

    // payloads

    pub struct StartPayload {
        username: String,
    }

    impl ServerBound for StartPayload {
        fn handle(&self, listener: &mut PacketListener) {
            listener.handle_login_start(self);
        }
    }

    pub struct EncryptionResponsePayload {
        secret: Bytes,
        verification_token: Bytes,
    }

    impl ServerBound for EncryptionResponsePayload {
        fn handle(&self, listener: &mut PacketListener) {
            listener.handle_login_encryption_response(self);
        }
    }

    pub struct PluginResponsePayload {
        message_id: i32,
        successful: bool,
        data: Bytes,
    }

    impl ServerBound for PluginResponsePayload {
        fn handle(&self, listener: &mut PacketListener) {
            listener.handle_login_plugin_response(self);
        }
    }
}

pub mod play {
    use crate::cocoabeans::protocol::packet::packet_listener::PacketListener;
    use crate::cocoabeans::protocol::packet::serverbound_packets::ServerBound;

    pub struct PlayPacket {
        pub payload: dyn ServerBound,
    }

    impl ServerBound for PlayPacket {
        fn handle(&self, listener: &mut PacketListener) {
            self.payload.handle(listener);
        }
    }

    // payloads
}
