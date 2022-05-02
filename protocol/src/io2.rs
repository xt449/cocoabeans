use std::io::{Read, Write};
use std::net::TcpStream;

use macros::json::Json;

pub struct MinecraftReader {
    read: Box<dyn Read>,
}

pub struct MinecraftWriter {
    write: Box<dyn Write>,
}

pub struct MinecraftStream {
    pub reader: MinecraftReader,
    pub writer: MinecraftWriter,
}

// Constructor
impl MinecraftReader {
    pub fn wrap(read: Box<dyn Read>) -> MinecraftReader {
        return MinecraftReader { read };
    }
}

// Constructor
impl MinecraftWriter {
    pub fn wrap(write: Box<dyn Write>) -> MinecraftWriter {
        return MinecraftWriter {  write };
    }
}

// Constructors
impl MinecraftStream {
    pub fn wrap(stream: &'static TcpStream) -> MinecraftStream {
        return MinecraftStream {
            reader: MinecraftReader { read: Box::new(stream) },
            writer: MinecraftWriter { write: Box::new(stream) },
        };
    }
    pub fn wrap_twins(read: Box<dyn Read>, write: Box<dyn Write>) -> MinecraftStream {
        return MinecraftStream {
            reader: MinecraftReader { read },
            writer: MinecraftWriter { write },
        };
    }
}

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

// Reader
impl MinecraftReader {
    // Take
    pub fn take(self, length: u64) -> MinecraftReader {
        return MinecraftReader { read: Box::new(self.read.take(length)) };
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
        let byte_in: u8 = 0u8;
        self.read.read_exact(&mut [byte_in]);
        return byte_in != 0;
    }

    pub fn read_byte(&mut self) -> i8 {
        let byte_in: u8 = 0u8;
        self.read.read_exact(&mut [byte_in]);
        return byte_in as i8;
    }

    pub fn read_unsigned_byte(&mut self) -> u8 {
        let byte_in: u8 = 0u8;
        self.read.read_exact(&mut [byte_in]);
        return byte_in;
    }

    pub fn read_short(&mut self) -> i16 {
        let mut bytes_in: [u8; 2] = [0u8, 0u8];
        self.read.read_exact(&mut bytes_in);
        return i16::from_be_bytes(bytes_in);
    }

    pub fn read_unsigned_short(&mut self) -> u16 {
        let mut bytes_in: [u8; 2] = [0u8, 0u8];
        self.read.read_exact(&mut bytes_in);
        return u16::from_be_bytes(bytes_in);
    }

    pub fn read_int(&mut self) -> i32 {
        let mut bytes_in: [u8; 4] = [0u8, 0u8, 0u8, 0u8];
        self.read.read_exact(&mut bytes_in);
        return i32::from_be_bytes(bytes_in);
    }

    pub fn read_long(&mut self) -> i64 {
        let mut bytes_in: [u8; 8] = [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
        self.read.read_exact(&mut bytes_in);
        return i64::from_be_bytes(bytes_in);
    }

    pub fn read_float(&mut self) -> f32 {
        let mut bytes_in: [u8; 4] = [0u8, 0u8, 0u8, 0u8];
        self.read.read_exact(&mut bytes_in);
        return f32::from_be_bytes(bytes_in);
    }

    pub fn read_double(&mut self) -> f64 {
        let mut bytes_in: [u8; 8] = [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
        self.read.read_exact(&mut bytes_in);
        return f64::from_be_bytes(bytes_in);
    }

    pub fn read_utf(&mut self) -> String {
        let length: i32 = self.read_varint();
        let mut bytes_in: Vec<u8> = Vec::with_capacity(length as usize);
        self.read.read_exact(&mut bytes_in);
        return String::from_utf8(bytes_in).unwrap();
    }

    pub fn read_json(&mut self) -> Json {
        return match serde_json::from_str::<Json>(&self.read_utf()) {
            Ok(json) => json,
            Err(_) => Json::new(),
        };
    }

    pub fn read_uuid(&mut self) -> u128 {
        let mut bytes_in: [u8; 16] = [
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        ];
        self.read.read_exact(&mut bytes_in);
        return u128::from_be_bytes(bytes_in);
    }

    pub fn read_byte_vec(&mut self, length: usize) -> Vec<u8> {
        let mut bytes_in: Vec<u8> = Vec::with_capacity(length);
        self.read.read_exact(&mut bytes_in);
        return bytes_in;
    }

    // pub fn read_byte_slice(&mut self, length: usize) -> &[u8] {
    //     return &self.read_byte_vec(length)[..];
    // }
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
        self.write.write_all(&mut [value as u8]);
    }

    pub fn write_byte(&mut self, value: i8) {
        self.write.write_all(&mut [value as u8]);
    }

    pub fn write_unsigned_byte(&mut self, value: u8) {
        self.write.write_all(&mut [value]);
    }

    pub fn write_short(&mut self, value: i16) {
        self.write.write(&value.to_be_bytes());
    }

    pub fn write_unsigned_short(&mut self, value: u16) {
        self.write.write(&value.to_be_bytes());
    }

    pub fn write_int(&mut self, value: i32) {
        self.write.write(&value.to_be_bytes());
    }

    pub fn write_long(&mut self, value: i64) {
        self.write.write(&value.to_be_bytes());
    }

    pub fn write_float(&mut self, value: f32) {
        self.write.write(&value.to_be_bytes());
    }

    pub fn write_double(&mut self, value: f64) {
        self.write.write(&value.to_be_bytes());
    }

    pub fn write_utf(&mut self, value: &String) {
        self.write.write(value.as_bytes());
    }

    pub fn write_json(&mut self, value: &Json) {
        self.write_utf(&serde_json::to_string(&value).unwrap());
    }

    pub fn write_uuid(&mut self, value: u128) {
        self.write.write(&value.to_be_bytes());
    }

    pub fn write_byte_vec(&mut self, value: &Vec<u8>) {
        self.write.write_all(&value[..]);
    }

    pub fn write_byte_slice(&mut self, value: &[u8]) {
        self.write.write_all(value);
    }
}
