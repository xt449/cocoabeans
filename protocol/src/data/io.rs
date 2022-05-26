use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use math::coordinate::BlockPosition;
use nbt::lib::Value;
use std::io::{Error, ErrorKind, Read, Result, Write};

// Bool

pub trait ReadBoolExt: Read {
    fn read_bool(&mut self) -> Result<bool> {
        return Ok(self.read_u8()? != 0);
    }
}

impl<U> ReadBoolExt for U where U: Read {}

pub trait WriteBoolExt: Write {
    fn write_bool(&mut self, value: bool) -> Result<()> {
        return self.write_u8(value as u8);
    }
}

// constants for the follow variable length "primitives"

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

// VarInt

pub trait ReadVarIntExt: Read {
    fn read_varint(&mut self) -> Result<i32> {
        let mut value: i32 = 0;
        let mut position: u8 = 0;

        loop {
            let current_byte = self.read_u8()?;
            value |= ((current_byte & SEGMENT_BITS) as i32) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                return Ok(value);
            }

            position += 7;

            if position >= 32 {
                return Err(Error::new(ErrorKind::InvalidData, "VarInt is too big"));
            }
        }
    }
}

impl<U> ReadVarIntExt for U where U: Read + ?Sized {}

pub trait WriteVarIntExt: Write {
    fn write_varint(&mut self, mut value: i32) -> Result<()> {
        loop {
            if (value & !SEGMENT_BITS as i32) == 0 {
                self.write_u8(value as u8)?;
                return Ok(());
            }

            self.write_u8(((value & SEGMENT_BITS as i32) | CONTINUE_BIT as i32) as u8)?;

            value >>= 7;
        }
    }
}

impl<U> WriteVarIntExt for U where U: Write + ?Sized {}

// VarLong

pub trait ReadVarLongExt: Read {
    fn read_varlong(&mut self) -> Result<i64> {
        let mut value: i64 = 0;
        let mut position: u8 = 0;

        loop {
            let current_byte = self.read_u8()?;
            value |= ((current_byte & SEGMENT_BITS) as i64) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                return Ok(value);
            }

            position += 7;

            if position >= 64 {
                return Err(Error::new(ErrorKind::InvalidData, "VarLong is too big"));
            }
        }
    }
}

impl<U> ReadVarLongExt for U where U: Read + ?Sized {}

pub trait WriteVarLongExt: Write {
    fn write_varlong(&mut self, mut value: i64) -> Result<()> {
        loop {
            if (value & !SEGMENT_BITS as i64) == 0 {
                self.write_u8(value as u8)?;
                return Ok(());
            }

            self.write_u8(((value & SEGMENT_BITS as i64) | CONTINUE_BIT as i64) as u8)?;

            value >>= 7;
        }
    }
}

impl<U> WriteVarLongExt for U where U: Write + ?Sized {}

// String

pub trait ReadSizedStringExt: Read {
    fn read_sized_string(&mut self, size: usize) -> Result<String> {
        let length = self.read_varint()? as usize;
        if length > size {
            return Err(Error::new(ErrorKind::InvalidData, "String too long"));
        }

        let mut buf = Vec::with_capacity(length);
        let mut take = self.take(length as u64);
        if take.limit() < size as u64 {
            return Err(Error::new(ErrorKind::InvalidData, "Read too short"));
        }
        take.read_to_end(&mut buf)?;

        return Ok(String::from_utf8(buf)
            .map_err(|_| Error::new(ErrorKind::InvalidData, "String had invalid UTF8 format"))?);
    }
}

impl<U> ReadSizedStringExt for U where U: Read {}

pub trait WriteSizedStringExt: Write {
    fn write_sized_string(&mut self, value: &String, size: usize) -> Result<()> {
        let bytes = value.as_bytes();
        if bytes.len() > i16::MAX as usize || bytes.len() > size {
            return Err(Error::new(ErrorKind::InvalidData, "String too long"));
        }

        self.write_varint(bytes.len() as i32)?;
        return self.write_all(bytes);
    }
}

impl<U> WriteSizedStringExt for U where U: Write {}

// NBT

trait ReadNBTExt: Read + Sized {
    fn read_nbt(&mut self) -> nbt::lib::Result<Value> {
        return Value::from_reader(/*hard coded compound id*/ 0x0a, self);
    }
}

impl<U> ReadNBTExt for U where U: Read {}

trait WriteNBTExt: Write + Sized {
    fn write_nbt(&mut self, value: &Value) -> nbt::lib::Result<()> {
        return value.to_writer(self);
    }
}

impl<U> WriteNBTExt for U where U: Write {}

// Byte Vec

trait ReadByteVecExt: Read {
    fn read_byte_vec(&mut self, size: u64) -> Result<Vec<u8>> {
        let mut take = self.take(size);
        if take.limit() < size {
            return Err(Error::new(ErrorKind::InvalidData, "Read too short"));
        }

        let mut array = Vec::with_capacity(size as usize);
        take.read_to_end(&mut array)?;

        return Ok(array);
    }
}

impl<U> ReadByteVecExt for U where U: Read {}

trait WriteByteVecExt: Write {
    fn write_byte_vec(&mut self, value: Vec<u8>, size: usize) -> Result<()> {
        if value.len() > size {
            return Err(Error::new(ErrorKind::InvalidData, "Vec too long"));
        }
        return self.write_all(&value);
    }
}

impl<U> WriteByteVecExt for U where U: Write {}

// BlockPosition

trait ReadBlockPositionExt: Read {
    fn read_block_position(&mut self) -> Result<BlockPosition> {
        let long = self.read_u64::<NetworkEndian>()?;
        return Ok(BlockPosition {
            x: (long >> 38) as i32,
            y: (long & 0xFFF) as i16,
            z: ((long >> 12) & 0x3FFFFFF) as i32,
        });
    }
}

impl<U> ReadBlockPositionExt for U where U: Read {}

trait WriteBlockPositionExt: Write {
    fn write_block_position(&mut self, value: &BlockPosition) -> Result<()> {
        return self.write_u64::<NetworkEndian>(
            (((value.x & 0x3FFFFFF) as u64) << 38)
                | (((value.z & 0x3FFFFFF) as u64) << 12)
                | (value.y & 0xFFF) as u64,
        );
    }
}

impl<U> WriteBlockPositionExt for U where U: Write {}
