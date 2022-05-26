use crate::data::io::{ReadVarIntExt, ReadVarLongExt, WriteVarIntExt, WriteVarLongExt};
use crate::data_type::ProtocolDataType;
use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Result, Write};

pub struct Boolean(bool);

impl ProtocolDataType<bool> for Boolean {
    fn read(read: &mut dyn Read) -> Result<Self> {
        return Ok(Self(read.read_u8()? != 0));
    }

    fn write(&self, write: &mut dyn Write) -> Result<()> {
        return write.write_u8(self.0 as u8);
    }

    fn unwrap(&self) -> bool {
        self.0
    }
}

pub struct Byte(i8);

impl ProtocolDataType<i8> for Byte {
    fn read(read: &mut dyn Read) -> Result<Self> {
        return Ok(Self(read.read_i8()?));
    }

    fn write(&self, write: &mut dyn Write) -> Result<()> {
        return write.write_i8(self.0);
    }

    fn unwrap(&self) -> i8 {
        self.0
    }
}

pub struct UnsignedByte(u8);

impl ProtocolDataType<u8> for UnsignedByte {
    fn read(read: &mut dyn Read) -> Result<Self> {
        return Ok(Self(read.read_u8()?));
    }

    fn write(&self, write: &mut dyn Write) -> Result<()> {
        return write.write_u8(self.0);
    }

    fn unwrap(&self) -> u8 {
        self.0
    }
}

pub struct Short(i16);

impl ProtocolDataType<i16> for Short {
    fn read(read: &mut dyn Read) -> Result<Self> {
        return Ok(Self(read.read_i16::<NetworkEndian>()?));
    }

    fn write(&self, write: &mut dyn Write) -> Result<()> {
        return write.write_i16::<NetworkEndian>(self.0);
    }

    fn unwrap(&self) -> i16 {
        self.0
    }
}

pub struct UnsignedShort(u16);

impl ProtocolDataType<u16> for UnsignedShort {
    fn read(read: &mut dyn Read) -> Result<Self> {
        return Ok(Self(read.read_u16::<NetworkEndian>()?));
    }

    fn write(&self, write: &mut dyn Write) -> Result<()> {
        return write.write_u16::<NetworkEndian>(self.0);
    }

    fn unwrap(&self) -> u16 {
        self.0
    }
}

pub struct Int(i32);

impl ProtocolDataType<i32> for Int {
    fn read(read: &mut dyn Read) -> Result<Self> {
        return Ok(Self(read.read_i32::<NetworkEndian>()?));
    }

    fn write(&self, write: &mut dyn Write) -> Result<()> {
        return write.write_i32::<NetworkEndian>(self.0);
    }

    fn unwrap(&self) -> i32 {
        self.0
    }
}

pub struct Long(i64);

impl ProtocolDataType<i64> for Long {
    fn read(read: &mut dyn Read) -> Result<Self> {
        return Ok(Self(read.read_i64::<NetworkEndian>()?));
    }

    fn write(&self, write: &mut dyn Write) -> Result<()> {
        return write.write_i64::<NetworkEndian>(self.0);
    }

    fn unwrap(&self) -> i64 {
        self.0
    }
}

pub struct Float(f32);

impl ProtocolDataType<f32> for Float {
    fn read(read: &mut dyn Read) -> Result<Self> {
        return Ok(Self(read.read_f32::<NetworkEndian>()?));
    }

    fn write(&self, write: &mut dyn Write) -> Result<()> {
        return write.write_f32::<NetworkEndian>(self.0);
    }

    fn unwrap(&self) -> f32 {
        self.0
    }
}

pub struct Double(f64);

impl ProtocolDataType<f64> for Double {
    fn read(read: &mut dyn Read) -> Result<Self> {
        return Ok(Self(read.read_f64::<NetworkEndian>()?));
    }

    fn write(&self, write: &mut dyn Write) -> Result<()> {
        return write.write_f64::<NetworkEndian>(self.0);
    }

    fn unwrap(&self) -> f64 {
        self.0
    }
}

// Variable length "primitives"

pub struct VarInt(i32);

impl ProtocolDataType<i32> for VarInt {
    fn read(read: &mut dyn Read) -> Result<Self> {
        return Ok(Self(read.read_varint()?));
    }

    fn write(&self, write: &mut dyn Write) -> Result<()> {
        return write.write_varint(self.0);
    }

    fn unwrap(&self) -> i32 {
        self.0
    }
}

pub struct VarLong(i64);

impl ProtocolDataType<i64> for VarLong {
    fn read(read: &mut dyn Read) -> Result<Self> {
        return Ok(Self(read.read_varlong()?));
    }

    fn write(&self, write: &mut dyn Write) -> Result<()> {
        return write.write_varlong(self.0);
    }

    fn unwrap(&self) -> i64 {
        self.0
    }
}
