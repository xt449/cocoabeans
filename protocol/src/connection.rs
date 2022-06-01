use std::io::{Read, Result, Write};
use std::net::{SocketAddr, TcpStream};
use std::ops::Deref;
use packets::data::io::{ReadVarIntExt, WriteVarIntExt};
use packets::State;
use packets::wrapped::{build_packet, clientbound, serverbound};

use crate::{MinecraftReader, MinecraftWriter, PacketHandler};

pub struct Connection<'c> {
    address: SocketAddr,
    stream: TcpStream,
    pub packet_handler: PacketHandler<'c>,
    compression: bool,
    encryption: Option<u64>,
}

// Constructor
impl<'c> Connection<'c> {
    pub fn new(address: SocketAddr, stream: TcpStream) -> Connection<'c> {
        stream.set_nonblocking(false).expect("Unable to make TcpStream blocking");
        let mut conn = Connection {
            address: address,
            stream: stream,
            packet_handler: PacketHandler::new(),
            compression: false,
            encryption: None
        };
        conn.packet_handler.init(&conn);

        return conn;
    }
}

// Methods
impl Connection<'_> {
    pub fn write_packet<T: clientbound::Packet>(&mut self, packet: T) {
        let mut buffer = MinecraftWriter::new();
        packet.write_to(&mut buffer);
        let bytes = buffer.to_slice();
        println!("DEBUG Sending packet #{} with total length {}", bytes[0], bytes.len());

        //bytes.insert(0, bytes.len() as u8);
        println!("DEBUG [ {} ]", bytes.iter().map(|v| format!("{:02X}", v)).collect::<Vec<String>>().join(" "));

        // let mut buffer = MinecraftReader::from(buffer.to_slice());
        // println!(
        //     "DEBUG id: {}, string: {}",
        //     buffer.read_unsigned_byte()?,
        //     buffer.read_string()?
        // );

        self.stream.write_varint(bytes.len() as i32).unwrap();
        self.stream.write_all(bytes.deref()).unwrap();
    }

    pub fn next_packet(&mut self) -> Result<()> {
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

        let taken = (&self.stream).take(length as u64);

        let packet_data = match self.encryption {
            None => MinecraftReader::from(taken),
            Some(cipher) => Self::decrypt_packet(cipher, MinecraftReader::from(taken)),
        };

        return Ok(self.decode_packet(packet_data)?.handle(&mut self.packet_handler));
    }

    fn decode_packet(&self, mut reader: MinecraftReader) -> Result<Box<dyn serverbound::Packet>> {
        println!("Decoding packet {} bytes long...", reader.remaining());
        let id = reader.read_varint()?;
        println!(
            "Decoding packet id#{} in state {}",
            id,
            match self.packet_handler.state {
                State::HANDSHAKING => "HANDSHAKING",
                State::STATUS => "STATUS",
                State::LOGIN => "LOGIN",
                State::PLAY => "PLAY",
            }
        );

        return build_packet(self.packet_handler.state, id as u32, reader);
    }
}

// Functions
impl Connection<'_> {
    fn decrypt_packet(cipher: u64, mut reader: MinecraftReader) -> MinecraftReader {
        // TODO
        return reader;
    }
}
