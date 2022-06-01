use std::io::{Error, ErrorKind, Read, Result, Write};
use std::net::{SocketAddr, TcpStream};

use crate::packets::data::io::{ReadVarIntExt, WriteVarIntExt};
use crate::packets::identifier::serverbound::build_packet;
use crate::packets::{clientbound, serverbound, State};

pub struct PacketIO<'c> {
    stream: &'c TcpStream,
    pub state: State,
    // TODO: compression and encryption
    /*compression_threshold: i32,
    encryption: Option<u64>,*/
}

// Constructor
impl<'c> PacketIO<'c> {
    pub fn new(stream: &'c TcpStream) -> Self {
        stream.set_nonblocking(false).expect("Unable to make TcpStream blocking");

        return Self { stream: stream, state: State::STATUS };
    }
}

// Methods
impl PacketIO<'_> {
    pub fn read_packet(&mut self) -> Result<Box<dyn serverbound::Packet>> {
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

        // Take specified length

        let mut take = self.stream.take(length as u64);
        if (take.limit() as i32) < length {
            return Err(Error::new(ErrorKind::InvalidData, "Unexpected early end of stream while reading packet bytes"));
        }

        let mut array = Vec::with_capacity(length as usize);
        take.read_to_end(&mut array)?;

        let mut packet_bytes = array.as_slice();

        // Decode

        println!("Decoding packet {} bytes long...", packet_bytes.len());
        let id = packet_bytes.read_varint()?;
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

        return build_packet(self.state, id as u32, packet_bytes);
    }

    pub fn write_packet<T: clientbound::Packet>(&mut self, packet: T) -> Result<()> {
        let mut buffer = Vec::with_capacity(128);
        buffer.write_varint(T::get_id() as i32)?;
        packet.write_to(&mut buffer);

        println!("DEBUG Sending packet #{} with total length {}", buffer[0], buffer.len());
        println!("DEBUG [ {} ]", buffer.iter().map(|v| format!("{:02X}", v)).collect::<Vec<String>>().join(" "));

        self.stream.write_varint(buffer.len() as i32)?;
        return self.stream.write_all(&buffer);
    }
}
