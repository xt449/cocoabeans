use std::io::{Error, ErrorKind, Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt};

pub trait ResultToOption<T> {
    fn to_option(self) -> Option<T>;
}

impl<T, E> ResultToOption<T> for Result<T, E> {
    fn to_option(self) -> Option<T> {
        return self.map_or_else(|_err| None, |ok| Some(ok));
    }
}

pub trait OptionToResult<T> {
    fn to_result(self) -> Result<T, ()>;
}

impl<T> OptionToResult<T> for Option<T> {
    fn to_result(self) -> Result<T, ()> {
        return self.map_or_else(|| Err(()), |some| Ok(some));
    }
}

pub trait OptionFrom<T>: TryFrom<T> {
    fn option_from(i: T) -> Option<Self> {
        return Self::try_from(i).map_or_else(|_err| None, |ok| Some(ok));
    }
}

impl<T, U> OptionFrom<T> for U where U: TryFrom<T> {}

pub trait OptionInto<T>: TryInto<T> {
    fn option_into(self) -> Option<T> {
        return self.try_into().map_or_else(|_err| None, |ok| Some(ok));
    }
}

impl<T, U> OptionInto<T> for U where U: TryInto<T> {}

// Variable length "primitives"

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

pub trait VarIntRead: Read {
    fn read_varint(&mut self) -> std::io::Result<i32> {
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

impl<U> VarIntRead for U where U: Read {}

pub trait VarIntWrite: Write {
    fn write_varint(&mut self, mut value: i32) -> std::io::Result<()> {
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

impl<U> VarIntWrite for U where U: Write {}

pub trait VarLongRead: Read {
    fn read_varlong(&mut self) -> std::io::Result<i64> {
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

impl<U> VarLongRead for U where U: Read {}

pub trait VarLongWrite: Write {
    fn write_varlong(&mut self, mut value: i64) -> std::io::Result<()> {
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

impl<U> VarLongWrite for U where U: Write {}

