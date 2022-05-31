use byteorder::{NetworkEndian, ReadBytesExt};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use nbt::Value;
use serde::Serialize;
use std::io::{Error, ErrorKind, Read, Take, Write};
use std::net::TcpStream;

pub trait MinecraftReadable<T> {
    fn deserialize_from(reader: &mut MinecraftReader) -> std::io::Result<T>;
}

pub trait MinecraftWritable {
    fn serialize_to(&self, writer: &mut MinecraftWriter);
}

pub struct MinecraftReader<'t> {
    taken: Take<&'t TcpStream>,
}

pub struct MinecraftWriter {
    pub buf: BytesMut,
}

// TODO: should be used for packet encryption
pub struct MinecraftBuffer {
    buf: BytesMut,
}

// Constructor
impl MinecraftReader<'_> {
    pub fn from(take: Take<&TcpStream>) -> MinecraftReader {
        return MinecraftReader { taken: take };
    }

    pub fn take_from(read: &TcpStream, size: u64) -> std::io::Result<MinecraftReader> {
        let take = read.take(size);
        if take.limit() < size {
            return Err(todo!());
        }

        return Ok(MinecraftReader { taken: take });
    }
}

// Constructor
impl MinecraftWriter {
    pub fn from(slice: &[u8]) -> MinecraftWriter {
        return MinecraftWriter { buf: BytesMut::from(slice) };
    }

    pub fn new() -> MinecraftWriter {
        return MinecraftWriter { buf: BytesMut::new() };
    }

    pub fn with_capacity(size: usize) -> MinecraftWriter {
        return MinecraftWriter { buf: BytesMut::with_capacity(size) };
    }
}

// Constructor
impl MinecraftBuffer {
    pub fn from(slice: &[u8]) -> MinecraftBuffer {
        return MinecraftBuffer { buf: BytesMut::from(slice) };
    }

    pub fn new() -> MinecraftBuffer {
        return MinecraftBuffer { buf: BytesMut::new() };
    }

    pub fn with_capacity(size: usize) -> MinecraftBuffer {
        return MinecraftBuffer { buf: BytesMut::with_capacity(size) };
    }

    pub fn copy_from(read: &mut dyn Read, size: usize) -> MinecraftBuffer {
        let mut vec_backing = Vec::<u8>::with_capacity(size);
        let slice: &mut [u8] = vec_backing.as_mut_slice();
        read.read_exact(slice).expect("MinecraftBuffer::copy_from");
        return MinecraftBuffer { buf: BytesMut::from(&slice[..]) };
    }
}

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

// Reader
impl MinecraftReader<'_> {
    // Check remining packet length
    pub fn remaining(&self) -> u64 {
        return self.taken.limit();
    }

    // VarInt Special
    pub fn read_varint(&mut self) -> std::io::Result<i32> {
        let mut value: i32 = 0;
        let mut position: i32 = 0;

        loop {
            let current_byte = self.taken.read_u8()?;
            value |= ((current_byte & SEGMENT_BITS) as i32) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                return Ok(value);
            }

            position += 7;

            if position >= 32 {
                return Err(todo!());
            }
        }
    }

    // VarLong Special
    pub fn read_varlong(&mut self) -> std::io::Result<i64> {
        let mut value: i64 = 0;
        let mut position: i32 = 0;

        loop {
            let current_byte = self.taken.read_u8()?;
            value |= ((current_byte & SEGMENT_BITS) as i64) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                return Ok(value);
            }

            position += 7;

            if position >= 64 {
                return Err(todo!());
            }
        }
    }

    pub fn read_boolean(&mut self) -> std::io::Result<bool> {
        return Ok(self.taken.read_u8()? != 0);
    }

    pub fn read_byte(&mut self) -> std::io::Result<i8> {
        return self.taken.read_i8();
    }

    pub fn read_unsigned_byte(&mut self) -> std::io::Result<u8> {
        return self.taken.read_u8();
    }

    pub fn read_short(&mut self) -> std::io::Result<i16> {
        return self.taken.read_i16::<NetworkEndian>();
    }

    pub fn read_unsigned_short(&mut self) -> std::io::Result<u16> {
        return self.taken.read_u16::<NetworkEndian>();
    }

    pub fn read_int(&mut self) -> std::io::Result<i32> {
        return self.taken.read_i32::<NetworkEndian>();
    }

    pub fn read_long(&mut self) -> std::io::Result<i64> {
        return self.taken.read_i64::<NetworkEndian>();
    }

    pub fn read_float(&mut self) -> std::io::Result<f32> {
        return self.taken.read_f32::<NetworkEndian>();
    }

    pub fn read_double(&mut self) -> std::io::Result<f64> {
        return self.taken.read_f64::<NetworkEndian>();
    }

    pub fn read_string(&mut self) -> std::io::Result<String> {
        let length = self.read_varint()? as u64;

        let mut take = self.taken.by_ref().take(length);
        if take.limit() < length {
            return Err(Error::new(ErrorKind::InvalidData, "Read too short"));
        }

        let mut buf = Vec::with_capacity(length as usize);
        take.read_to_end(&mut buf)?;

        return Ok(String::from_utf8(buf).map_err(|_| Error::new(ErrorKind::InvalidData, "String had invalid UTF8 format"))?);
    }

    pub fn read_limited_string(&mut self, size: u64) -> std::io::Result<String> {
        let length = self.read_varint()? as u64;
        if length > size {
            return Err(Error::new(ErrorKind::InvalidData, "String too long"));
        }

        let mut take = self.taken.by_ref().take(length);
        if take.limit() < length {
            return Err(Error::new(ErrorKind::InvalidData, "Read too short"));
        }

        let mut buf = Vec::with_capacity(length as usize);
        take.read_to_end(&mut buf)?;

        return Ok(String::from_utf8(buf).map_err(|_| Error::new(ErrorKind::InvalidData, "String had invalid UTF8 format"))?);
    }

    // pub fn read_json<T: DeserializeOwned>(&mut self) -> T {
    //     return serde_json::from_str::<T>(self.read_utf().as_str()).unwrap();
    // }

    pub fn read_uuid(&mut self) -> std::io::Result<u128> {
        return self.taken.read_u128::<NetworkEndian>();
    }

    pub fn read_byte_vec(&mut self, size: u64) -> std::io::Result<Vec<u8>> {
        let mut take = self.taken.by_ref().take(size);
        if take.limit() < size {
            return Err(todo!());
        }

        let mut vec = Vec::with_capacity(size as usize);
        take.read_to_end(&mut vec)?;
        return Ok(vec);
    }

    pub fn read<T: MinecraftReadable<T>>(&mut self) -> std::io::Result<T> {
        return T::deserialize_from(self);
    }

    pub fn read_option<T: MinecraftReadable<T>>(&mut self) -> std::io::Result<Option<T>> {
        return if self.read_boolean()? { T::deserialize_from(self).map(|v| Some(v)) } else { Ok(None) };
    }
}

// impl Read for MinecraftReader<'_> {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         self.take.copy_to_slice(buf);
//         return Ok(buf.len());
//     }
// }

// Writer
impl MinecraftWriter {
    // Easily get contents of buf to write elsewhere
    pub fn to_slice(&self) -> &[u8] {
        return self.buf.chunk();
    }

    // VarInt Special
    pub fn write_varint(&mut self, mut value: i32) {
        loop {
            if (value & !SEGMENT_BITS as i32) == 0 {
                self.write_unsigned_byte(value as u8);
                return;
            }

            self.write_unsigned_byte(((value & SEGMENT_BITS as i32) | CONTINUE_BIT as i32) as u8);

            value >>= 7;
        }
    }

    // VarLong Special
    pub fn write_varlong(&mut self, mut value: i64) {
        loop {
            if (value & !(SEGMENT_BITS as i64)) == 0 {
                self.write_unsigned_byte(value as u8);
                return;
            }

            self.write_unsigned_byte(((value & SEGMENT_BITS as i64) | CONTINUE_BIT as i64) as u8);

            value >>= 7;
        }
    }

    pub fn write_boolean(&mut self, value: bool) {
        self.buf.put_u8(value as u8);
    }

    pub fn write_byte(&mut self, value: i8) {
        self.buf.put_i8(value);
    }

    pub fn write_unsigned_byte(&mut self, value: u8) {
        self.buf.put_u8(value);
    }

    pub fn write_short(&mut self, value: i16) {
        self.buf.put_i16(value);
    }

    pub fn write_unsigned_short(&mut self, value: u16) {
        self.buf.put_u16(value);
    }

    pub fn write_int(&mut self, value: i32) {
        self.buf.put_i32(value);
    }

    pub fn write_long(&mut self, value: i64) {
        self.buf.put_i64(value);
    }

    pub fn write_float(&mut self, value: f32) {
        self.buf.put_f32(value);
    }

    pub fn write_double(&mut self, value: f64) {
        self.buf.put_f64(value);
    }

    pub fn write_utf(&mut self, value: &str) {
        let bytes = value.as_bytes();
        self.write_varint(bytes.len() as i32);
        self.buf.put_slice(bytes);
    }

    pub fn write_json<T: Serialize>(&mut self, value: &T) {
        self.write_utf(&serde_json::to_string(&value).unwrap());
    }

    pub fn write_uuid(&mut self, value: u128) {
        self.buf.put_u128(value);
    }

    pub fn write_byte_vec(&mut self, value: &Vec<u8>) {
        self.buf.put_slice(value.as_slice());
    }

    pub fn write_bytes(&mut self, value: Bytes) {
        self.buf.put_slice(value.chunk());
    }

    pub fn write_byte_slice(&mut self, value: &[u8]) {
        self.buf.put_slice(value);
    }

    pub fn write<T: MinecraftWritable>(&mut self, value: &T) {
        value.serialize_to(self);
    }

    pub fn write_option<T: MinecraftWritable>(&mut self, value: &Option<T>) {
        match value {
            None => self.write_unsigned_byte(0),
            Some(value) => {
                self.write_unsigned_byte(1);
                value.serialize_to(self);
            }
        }
    }
}

impl Write for MinecraftWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.put_slice(buf);
        return Ok(buf.len());
    }

    fn flush(&mut self) -> std::io::Result<()> {
        return Ok(());
    }
}

// Readable/Writable extras

impl MinecraftReadable<Value> for Value {
    fn deserialize_from(reader: &mut MinecraftReader) -> std::io::Result<Value> {
        if let Ok(value) = Value::from_reader(/*hard coded compound id*/ 0x0a, &mut reader.taken) {
            return Ok(value);
        }
        return Err(todo!());
    }
}

impl MinecraftWritable for Value {
    fn serialize_to(&self, writer: &mut MinecraftWriter) {
        self.to_writer(writer).unwrap();
    }
}
