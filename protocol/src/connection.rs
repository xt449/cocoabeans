use std::io::{Error, ErrorKind, Read, Result};
use std::net::{SocketAddr, TcpStream};

use packets::data::io::ReadVarIntExt;
use packets::wrapped::{build_packet, serverbound};
use packets::State;

use crate::PacketHandler;

pub struct Connection<'c> {
    address: SocketAddr,
    stream: &'c TcpStream,
    pub packet_handler: PacketHandler<'c>,
    compression: bool,
    /*encryption: Option<u64>,*/
}

// Constructor
impl<'c> Connection<'c> {
    pub fn new(address: SocketAddr, stream: &'c TcpStream) -> Connection<'c> {
        stream.set_nonblocking(false).expect("Unable to make TcpStream blocking");

        return Connection {
            address: address,
            stream: stream,
            packet_handler: PacketHandler::new(stream),
            compression: false,
            /*encryption: None,*/
        };
    }
}

// Methods
impl Connection<'_> {
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

        let mut take = (&self.stream).take(length as u64);
        if (take.limit() as i32) < length {
            return Err(Error::new(ErrorKind::InvalidData, "Unexpected early end of stream while reading packet bytes"));
        }

        let mut array = Vec::with_capacity(length as usize);
        take.read_to_end(&mut array)?;

        /*let packet_data = match self.encryption {
            None => (&mut self.stream).take(length as u64),
            Some(cipher) => Self::decrypt_packet(cipher, (&mut self.stream).take(length as u64)),
        };*/

        self.decode_packet(array.as_slice())?.handle(&mut self.packet_handler);

        return Ok(());
    }

    fn decode_packet(&self, mut packet_bytes: &[u8]) -> Result<Box<dyn serverbound::Packet>> {
        println!("Decoding packet {} bytes long...", packet_bytes.len());
        let id = packet_bytes.read_varint()?;
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

        return build_packet(self.packet_handler.state, id as u32, packet_bytes);
    }
}

// Functions
impl Connection<'_> {
    fn decrypt_packet(cipher: u64, mut packet_bytes: &[u8]) -> &[u8] {
        todo!()
    }
}
