use extensions::{VarIntRead, VarIntWrite};
use serde_json::json;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::ops::Deref;

use crate::io::{MinecraftReader, MinecraftWriter};
use crate::packet::clientbound;
use crate::packet::clientbound::ClientBoundPacket;
use crate::packet::serverbound;
use crate::versions;
use crate::versions::ProtocolVersion;

pub enum State {
    HANDSHAKING = 0,
    STATUS = 1,
    LOGIN = 2,
    PLAY = 3,
}

impl TryFrom<usize> for State {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        return match value {
            x if x == State::HANDSHAKING as usize => Ok(State::HANDSHAKING),
            x if x == State::STATUS as usize => Ok(State::STATUS),
            x if x == State::LOGIN as usize => Ok(State::LOGIN),
            x if x == State::PLAY as usize => Ok(State::PLAY),
            _ => Err(()),
        };
    }
}

pub trait IPacketHandler {
    // Handshaking
    fn handle_handshaking_handshake(
        &mut self,
        payload: &serverbound::handshaking::HandshakePayload,
    );
    // Status
    fn handle_status_request(&mut self, payload: &serverbound::status::RequestPayload);
    fn handle_status_ping(&mut self, payload: &serverbound::status::PingPayload);
    // Login
    fn handle_login_start(&mut self, payload: &serverbound::login::StartPayload);
    fn handle_login_encryption_response(
        &mut self,
        payload: &serverbound::login::EncryptionResponsePayload,
    );
    fn handle_login_plugin_response(&mut self, payload: &serverbound::login::PluginResponsePayload);
    // Play - TODO
}

pub struct PacketHandler {
    pub stream: TcpStream,
    pub state: State,
    pub protocol_version: &'static dyn ProtocolVersion,
    pub compression: bool,
    pub encryption: Option<u64>,
}

// Constructor
impl PacketHandler {
    pub fn new(stream: TcpStream) -> PacketHandler {
        return PacketHandler {
            stream: stream,
            state: State::HANDSHAKING,
            protocol_version: &versions::V758 {},
            compression: false,
            encryption: None,
        };
    }
}

// Packet stuff
impl PacketHandler {
    pub fn write_packet<T: ClientBoundPacket>(&mut self, packet: T) {
        let mut buffer = MinecraftWriter::new();
        packet.write_to(&mut buffer, self.protocol_version);
        let mut bytes = Vec::from(buffer.to_array());
        println!(
            "Sending packet #{} with total length {}",
            bytes[0],
            bytes.len()
        );

        //bytes.insert(0, bytes.len() as u8);
        println!(
            "[ {} ]",
            bytes
                .iter()
                .map(|v| format!("{:02X}", v))
                .collect::<Vec<String>>()
                .join(" ")
        );

        let mut buffer = MinecraftReader::from(buffer.to_array());
        println!(
            "id: {}, string: {}",
            buffer.read_unsigned_byte(),
            buffer.read_utf()
        );

        self.stream.write_varint(bytes.len() as i32).unwrap();
        self.stream.write_all(bytes.deref()).unwrap();
    }

    pub fn read_next_packet(&mut self) -> Option<serverbound::ServerBoundPacket> {
        //self.stream.peek(&mut [0, 0]).expect("HOW THOUGH"); // TODO

        let length = self.stream.read_varint();
        if length == 0 {
            return None;
        }

        let mut vec = Vec::<u8>::with_capacity(length as usize);
        let read_result = Read::by_ref(&mut self.stream)
            .take(length as u64)
            .read_to_end(&mut vec);
        if let Err(_) = read_result {
            return None;
        }

        return match self.encryption {
            None => PacketHandler::decode_packet(
                &self.state,
                self.protocol_version,
                MinecraftReader::from(vec.as_slice()),
            ),
            Some(cipher) => PacketHandler::decode_packet(
                &self.state,
                self.protocol_version,
                PacketHandler::decrypt_packet(cipher, vec),
            ),
        };
    }

    fn decrypt_packet(cipher: u64, vec: Vec<u8>) -> MinecraftReader {
        // TODO
        return MinecraftReader::from(vec.as_slice());
    }

    fn decode_packet<'a>(
        state: &State,
        protocol_version: &'a dyn ProtocolVersion,
        mut reader: MinecraftReader,
    ) -> Option<serverbound::ServerBoundPacket> {
        println!("Decoding packet {} bytes long...", reader.remaining());
        let id = reader.read_varint();
        println!(
            "Decoding packet id#{} in state {}",
            id,
            match state {
                State::HANDSHAKING => {
                    "HANDSHAKING"
                }
                State::STATUS => {
                    "STATUS"
                }
                State::LOGIN => {
                    "LOGIN"
                }
                State::PLAY => {
                    "PLAY"
                }
            }
        );
        return match protocol_version.get_packet_builder_from_id(state, id as u8) {
            None => None,
            Some(builder) => builder(reader),
        };
    }
}

impl IPacketHandler for PacketHandler {
    // Handshaking

    fn handle_handshaking_handshake(
        &mut self,
        payload: &serverbound::handshaking::HandshakePayload,
    ) {
        println!("Received HandshakePayload");
        // TODO - magic value
        if payload.protocol_version != 758 {
            return;
        }

        match payload.next_state {
            State::STATUS => {
                self.state = State::STATUS;
            }
            State::LOGIN => {
                self.state = State::LOGIN;
            }
            _ => {}
        }
    }

    // Status

    fn handle_status_request(&mut self, payload: &serverbound::status::RequestPayload) {
        self.write_packet(clientbound::status::ResponsePacket {
            json_payload: json!({
                "version": {
                    "name": "1.8.7",
                    "protocol": self.protocol_version.get_id()
                },
                "players": {
                    "max": 14,
                    "online": 3
                },
                "description": {
                    "text": "Hello world",
                    "color": "aqua"
                }
            }),
        });
    }

    fn handle_status_ping(&mut self, payload: &serverbound::status::PingPayload) {
        self.write_packet(clientbound::status::PongPacket {
            payload: payload.payload,
        });
    }

    // Login

    fn handle_login_start(&mut self, payload: &serverbound::login::StartPayload) {
        todo!()
    }

    fn handle_login_encryption_response(
        &mut self,
        payload: &serverbound::login::EncryptionResponsePayload,
    ) {
        todo!()
    }

    fn handle_login_plugin_response(
        &mut self,
        payload: &serverbound::login::PluginResponsePayload,
    ) {
        todo!()
    }

    // Play - TODO
}
