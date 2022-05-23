use extensions::{VarIntRead, VarIntWrite};
use serde_json::json;
use std::io::{Error, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::ops::Deref;

use crate::io::{MinecraftReader, MinecraftWriter};
use crate::packet::clientbound;
use crate::packet::clientbound::ClientBoundPacket;
use crate::packet::serverbound;
use crate::version_manager;
use crate::version::ProtocolVersion;

#[derive(Copy, Clone)]
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

pub struct PacketHandler<'a> {
    stream: TcpStream,
    state: State,
    protocol_version: &'a dyn ProtocolVersion,
    compression: bool,
    encryption: Option<u64>,
}

// Constructor
impl<'a> PacketHandler<'a> {
    pub fn new(stream: TcpStream) -> PacketHandler<'a> {
        stream
            .set_nonblocking(false)
            .expect("Unable to make TcpStream blocking");
        return PacketHandler {
            stream: stream,
            state: State::HANDSHAKING,
            protocol_version: &version_manager::V758 {},
            compression: false,
            encryption: None,
        };
    }
}

// Packet Accessors
impl<'a> PacketHandler<'a> {
    pub fn write_packet<T: ClientBoundPacket>(&mut self, packet: T) {
        let mut buffer = MinecraftWriter::new();
        packet.write_to(&mut buffer, self.protocol_version);
        let bytes = buffer.to_array();
        println!(
            "DEBUG Sending packet #{} with total length {}",
            bytes[0],
            bytes.len()
        );

        //bytes.insert(0, bytes.len() as u8);
        println!(
            "DEBUG [ {} ]",
            bytes
                .iter()
                .map(|v| format!("{:02X}", v))
                .collect::<Vec<String>>()
                .join(" ")
        );

        let mut buffer = MinecraftReader::from(buffer.to_array());
        println!(
            "DEBUG id: {}, string: {}",
            buffer.read_unsigned_byte(),
            buffer.read_utf()
        );

        self.stream.write_varint(bytes.len() as i32).unwrap();
        self.stream.write_all(bytes.deref()).unwrap();
    }

    pub fn next(&mut self) -> Result<(), Error> {
        let packet = self.read_next_packet()?;
        packet.handle(self);
        return Ok(());
    }

    fn read_next_packet(&mut self) -> Result<serverbound::ServerBoundPacket, Error> {
        let mut length: i32;
        loop {
            // do while
            match self.stream.read_varint() {
                Ok(v) => {
                    length = v;
                }
                Err(e) => {
                    return Err(e);
                }
            }
            if length != 0 {
                break;
            }
        }

        let mut vec = Vec::<u8>::with_capacity(length as usize);
        (&self.stream)
            .take(length as u64)
            .read_to_end(&mut vec)
            .expect("Error reading from Take of TcpStream");

        let packet_data = match self.encryption {
            None => MinecraftReader::from(vec.as_slice()),
            Some(cipher) => PacketHandler::decrypt_packet(cipher, vec),
        };

        return match self.decode_packet(packet_data) {
            None => Err(Error::new(
                ErrorKind::InvalidData,
                "Unable to decrypt packet",
            )),
            Some(packet) => Ok(packet),
        };
    }

    fn decrypt_packet(cipher: u64, vec: Vec<u8>) -> MinecraftReader {
        // TODO
        return MinecraftReader::from(vec.as_slice());
    }

    fn decode_packet(&self, mut reader: MinecraftReader) -> Option<serverbound::ServerBoundPacket> {
        println!("Decoding packet {} bytes long...", reader.remaining());
        let id = reader.read_varint();
        println!(
            "Decoding packet id#{} in state {}",
            id,
            match self.state {
                State::HANDSHAKING => "HANDSHAKING",
                State::STATUS => "STATUS",
                State::LOGIN => "LOGIN",
                State::PLAY => "PLAY",
            }
        );
        return match self.protocol_version.get_packet_builder_from_id(self.state.clone(), id as u8) {
            None => None,
            Some(builder) => builder(reader),
        };
    }
}

// Packet Handler
impl<'a> IPacketHandler for PacketHandler<'a> {
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
                    "name": "1.18.2 or not",
                    "protocol": self.protocol_version.get_id()
                },
                "players": {
                    "max": 1000000000,
                    "online": -1
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
