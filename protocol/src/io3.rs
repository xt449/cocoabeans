use std::borrow::Borrow;
use std::io::{Read};
use std::ops::Deref;
use bytes::{Buf, BufMut, Bytes, BytesMut};

use macros::json::Json;

pub struct MinecraftReader {
    buf: Bytes,
}

pub struct MinecraftWriter {
    buf: BytesMut,
}

pub struct MinecraftBuffer {
    buf: BytesMut,
}

// Constructor
impl MinecraftReader {
    pub fn wrap(slice: &[u8]) -> MinecraftReader {
        return MinecraftReader { buf: Bytes::copy_from_slice(slice) };
    }

    pub fn read_from(read: &mut dyn Read, size: usize) -> MinecraftReader {
        let mut vec_backing = Vec::<u8>::with_capacity(size);
        let slice: &mut [u8] = vec_backing.as_mut_slice();
        read.read_exact(slice);
        return MinecraftReader { buf: Bytes::copy_from_slice(slice) };
    }
}

// Constructor
impl MinecraftWriter {
    pub fn wrap(slice: &[u8]) -> MinecraftWriter {
        return MinecraftWriter { buf: BytesMut::from(slice) };
    }

    pub fn new(size: usize) -> MinecraftWriter {
        return MinecraftWriter { buf: BytesMut::with_capacity(size) };
    }
}

// Constructor
impl MinecraftBuffer {
    pub fn wrap(slice: &[u8]) -> MinecraftBuffer {
        return MinecraftBuffer { buf: BytesMut::from(slice) };
    }

    pub fn new(size: usize) -> MinecraftBuffer {
        return MinecraftBuffer { buf: BytesMut::with_capacity(size) };
    }

    pub fn copy_from(read: &mut dyn Read, size: usize) -> MinecraftBuffer {
        let mut vec_backing = Vec::<u8>::with_capacity(size);
        let slice: &mut [u8] = vec_backing.as_mut_slice();
        read.read_exact(slice);
        return MinecraftBuffer { buf: BytesMut::from(&slice[..]) };
    }
}

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

// Reader
impl MinecraftReader {
    // Take
    pub fn take(self, length: usize) -> MinecraftReader {
        return MinecraftReader { buf: self.buf.take(length).into_inner() };
    }

    // VarInt Special
    pub fn read_varint(&mut self) -> i32 {
        let mut value: i32 = 0;
        let mut position: i32 = 0;

        loop {
            let current_byte = self.read_unsigned_byte();
            value |= ((current_byte & SEGMENT_BITS) as i32) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                return value;
            }

            position += 7;

            if position >= 32 {
                // Too big
                return value;
            }
        }
    }

    // VarLong Special
    pub fn read_varlong(&mut self) -> i64 {
        let mut value: i64 = 0;
        let mut position: i32 = 0;

        loop {
            let current_byte = self.read_unsigned_byte();
            value |= ((current_byte & SEGMENT_BITS) as i64) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                return value;
            }

            position += 7;

            if position >= 64 {
                // Too big
                return value;
            }
        }
    }

    pub fn read_boolean(&mut self) -> bool {
        return self.buf.get_u8() != 0;
    }

    pub fn read_byte(&mut self) -> i8 {
        return self.buf.get_i8();
    }

    pub fn read_unsigned_byte(&mut self) -> u8 {
        return self.buf.get_u8();
    }

    pub fn read_short(&mut self) -> i16 {
        return self.buf.get_i16();
    }

    pub fn read_unsigned_short(&mut self) -> u16 {
        return self.buf.get_u16();
    }

    pub fn read_int(&mut self) -> i32 {
        return self.buf.get_i32();
    }

    pub fn read_long(&mut self) -> i64 {
        return self.buf.get_i64();
    }

    pub fn read_float(&mut self) -> f32 {
        return self.buf.get_f32();
    }

    pub fn read_double(&mut self) -> f64 {
        return self.buf.get_f64();
    }

    pub fn read_utf(&mut self) -> String {
        let length: i32 = self.read_varint();
        return String::from_utf8(self.buf.copy_to_bytes(length as usize).to_vec()).unwrap();
    }

    pub fn read_json(&mut self) -> Json {
        return match serde_json::from_str::<Json>(&self.read_utf()) {
            Ok(json) => json,
            Err(_) => Json::new(),
        };
    }

    pub fn read_uuid(&mut self) -> u128 {
        return self.buf.get_u128();
    }

    pub fn read_byte_vec(&mut self, length: usize) -> Vec<u8> {
        return self.buf.copy_to_bytes(length).to_vec();
    }

    pub fn read_bytes(&mut self, length: usize) -> Bytes {
        return self.buf.copy_to_bytes(length);
    }
}

// Writer
impl MinecraftWriter {
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

    pub fn write_utf(&mut self, value: &String) {
        self.buf.put_slice(value.as_bytes());
    }

    pub fn write_json(&mut self, value: &Json) {
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
}
