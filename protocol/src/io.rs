use std::io::{Read, Write};
use std::net::TcpStream;

use macros::json::Json;

pub struct MinecraftReader<'t> {
    stream: &'t mut MinecraftStream,
}

pub struct MinecraftWriter<'t> {
    stream: &'t mut MinecraftStream,
}

pub struct MinecraftStream {
    pub stream: TcpStream,
}

// Constructor
impl<'t> MinecraftReader<'t> {
    pub fn wrap(stream: &mut MinecraftStream) -> MinecraftReader {
        return MinecraftReader { stream };
    }
}

// Constructor
impl<'t> MinecraftWriter<'t> {
    pub fn wrap(stream: &mut MinecraftStream) -> MinecraftWriter {
        return MinecraftWriter { stream };
    }
}

// Constructor
impl MinecraftStream {
    pub fn wrap(stream: TcpStream) -> MinecraftStream {
        return MinecraftStream { stream };
    }
}

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

// Reader
impl<'t> MinecraftReader<'t> {
    pub fn read_varint(&mut self) -> i32 {
        return self.stream.read_varint();
    }
    pub fn read_varlong(&mut self) -> i64 {
        return self.stream.read_varlong();
    }

    pub fn read_boolean(&mut self) -> bool {
        return self.stream.read_boolean();
    }

    pub fn read_byte(&mut self) -> i8 {
        return self.stream.read_byte();
    }

    pub fn read_unsigned_byte(&mut self) -> u8 {
        return self.stream.read_unsigned_byte();
    }

    pub fn read_short(&mut self) -> i16 {
        return self.stream.read_short();
    }

    pub fn read_unsigned_short(&mut self) -> u16 {
        return self.stream.read_unsigned_short();
    }

    pub fn read_int(&mut self) -> i32 {
        return self.stream.read_int();
    }

    pub fn read_long(&mut self) -> i64 {
        return self.stream.read_long();
    }

    pub fn read_float(&mut self) -> f32 {
        return self.stream.read_float();
    }

    pub fn read_double(&mut self) -> f64 {
        return self.stream.read_double();
    }

    pub fn read_utf(&mut self) -> String {
        return self.stream.read_utf();
    }

    pub fn read_json(&mut self) -> Json {
        return self.stream.read_json();
    }

    pub fn read_uuid(&mut self) -> u128 {
        return self.stream.read_uuid();
    }

    pub fn read_byte_vec(&mut self, length: usize) -> Vec<u8> {
        return self.stream.read_byte_vec(length);
    }

    // pub fn read_byte_slice(&mut self, length: usize) -> &[u8] {
    //     return self.stream.read_byte_slice(length);
    // }
}

// Writer
impl<'t> MinecraftWriter<'t> {
    // VarInt Special
    pub fn write_varint(&mut self, value: i32) {
        return self.stream.write_varint(value);
    }

    // VarLong Special
    pub fn write_varlong(&mut self, value: i64) {
        return self.stream.write_varlong(value);
    }

    pub fn write_boolean(&mut self, value: bool) {
        return self.stream.write_boolean(value);
    }

    pub fn write_byte(&mut self, value: i8) {
        return self.stream.write_byte(value);
    }

    pub fn write_unsigned_byte(&mut self, value: u8) {
        return self.stream.write_unsigned_byte(value);
    }

    pub fn write_short(&mut self, value: i16) {
        return self.stream.write_short(value);
    }

    pub fn write_unsigned_short(&mut self, value: u16) {
        return self.stream.write_unsigned_short(value);
    }

    pub fn write_int(&mut self, value: i32) {
        return self.stream.write_int(value);
    }

    pub fn write_long(&mut self, value: i64) {
        return self.stream.write_long(value);
    }

    pub fn write_float(&mut self, value: f32) {
        return self.stream.write_float(value);
    }

    pub fn write_double(&mut self, value: f64) {
        return self.stream.write_double(value);
    }

    pub fn write_utf(&mut self, value: &String) {
        return self.stream.write_utf(value);
    }

    pub fn write_json(&mut self, value: &Json) {
        return self.stream.write_json(value);
    }

    pub fn write_uuid(&mut self, value: u128) {
        return self.stream.write_uuid(value);
    }

    pub fn write_byte_vec(&mut self, value: &Vec<u8>) {
        return self.stream.write_byte_vec(value);
    }

    pub fn write_byte_slice(&mut self, value: &[u8]) {
        return self.stream.write_byte_slice(value);
    }
}

// Readers
impl MinecraftStream {
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
        self.stream.read_exact(&mut [byte_in]);
        return byte_in != 0;
    }

    pub fn read_byte(&mut self) -> i8 {
        let byte_in: u8 = 0u8;
        self.stream.read_exact(&mut [byte_in]);
        return byte_in as i8;
    }

    pub fn read_unsigned_byte(&mut self) -> u8 {
        let byte_in: u8 = 0u8;
        self.stream.read_exact(&mut [byte_in]);
        return byte_in;
    }

    pub fn read_short(&mut self) -> i16 {
        let mut bytes_in: [u8; 2] = [0u8, 0u8];
        self.stream.read_exact(&mut bytes_in);
        return i16::from_be_bytes(bytes_in);
    }

    pub fn read_unsigned_short(&mut self) -> u16 {
        let mut bytes_in: [u8; 2] = [0u8, 0u8];
        self.stream.read_exact(&mut bytes_in);
        return u16::from_be_bytes(bytes_in);
    }

    pub fn read_int(&mut self) -> i32 {
        let mut bytes_in: [u8; 4] = [0u8, 0u8, 0u8, 0u8];
        self.stream.read_exact(&mut bytes_in);
        return i32::from_be_bytes(bytes_in);
    }

    pub fn read_long(&mut self) -> i64 {
        let mut bytes_in: [u8; 8] = [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
        self.stream.read_exact(&mut bytes_in);
        return i64::from_be_bytes(bytes_in);
    }

    pub fn read_float(&mut self) -> f32 {
        let mut bytes_in: [u8; 4] = [0u8, 0u8, 0u8, 0u8];
        self.stream.read_exact(&mut bytes_in);
        return f32::from_be_bytes(bytes_in);
    }

    pub fn read_double(&mut self) -> f64 {
        let mut bytes_in: [u8; 8] = [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
        self.stream.read_exact(&mut bytes_in);
        return f64::from_be_bytes(bytes_in);
    }

    pub fn read_utf(&mut self) -> String {
        let length: i32 = self.read_varint();
        let mut bytes_in: Vec<u8> = Vec::with_capacity(length as usize);
        self.stream.read_exact(&mut bytes_in);
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
        self.stream.read_exact(&mut bytes_in);
        return u128::from_be_bytes(bytes_in);
    }

    pub fn read_byte_vec(&mut self, length: usize) -> Vec<u8> {
        let mut bytes_in: Vec<u8> = Vec::with_capacity(length);
        self.stream.read_exact(&mut bytes_in);
        return bytes_in;
    }

    // pub fn read_byte_slice(&mut self, length: usize) -> &[u8] {
    //     return &self.read_byte_vec(length)[..];
    // }
}

// Writers
impl MinecraftStream {
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
        self.stream.write_all(&mut [value as u8]);
    }

    pub fn write_byte(&mut self, value: i8) {
        self.stream.write_all(&mut [value as u8]);
    }

    pub fn write_unsigned_byte(&mut self, value: u8) {
        self.stream.write_all(&mut [value]);
    }

    pub fn write_short(&mut self, value: i16) {
        self.stream.write(&value.to_be_bytes());
    }

    pub fn write_unsigned_short(&mut self, value: u16) {
        self.stream.write(&value.to_be_bytes());
    }

    pub fn write_int(&mut self, value: i32) {
        self.stream.write(&value.to_be_bytes());
    }

    pub fn write_long(&mut self, value: i64) {
        self.stream.write(&value.to_be_bytes());
    }

    pub fn write_float(&mut self, value: f32) {
        self.stream.write(&value.to_be_bytes());
    }

    pub fn write_double(&mut self, value: f64) {
        self.stream.write(&value.to_be_bytes());
    }

    pub fn write_utf(&mut self, value: &String) {
        self.stream.write(value.as_bytes());
    }

    pub fn write_json(&mut self, value: &Json) {
        self.write_utf(&serde_json::to_string(&value).unwrap());
    }

    pub fn write_uuid(&mut self, value: u128) {
        self.stream.write(&value.to_be_bytes());
    }

    pub fn write_byte_vec(&mut self, value: &Vec<u8>) {
        self.stream.write_all(&value[..]);
    }

    pub fn write_byte_slice(&mut self, value: &[u8]) {
        self.stream.write_all(value);
    }
}

// Restrictors
impl MinecraftStream {
    #[warn(unused_allocation)]
    pub fn get_reader(&mut self) -> MinecraftReader {
        return MinecraftReader::wrap(self);
    }

    #[warn(unused_allocation)]
    pub fn get_writer(&mut self) -> MinecraftWriter {
        return MinecraftWriter::wrap(self);
    }
}
