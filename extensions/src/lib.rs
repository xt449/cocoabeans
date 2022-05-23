use std::io::{Read, Write};

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

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

pub trait VarIntRead: UnsignedByteRead {
    fn read_varint(&mut self) -> std::io::Result<i32> {
        let mut value: i32 = 0;
        let mut position: i32 = 0;

        loop {
            let current_byte = self.read_unsigned_byte()?;
            println!(">> Reading varint {}", current_byte);
            value |= ((current_byte & SEGMENT_BITS) as i32) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                println!(">> Finished varint {}", value);
                return Ok(value);
            }

            position += 7;

            if position >= 32 {
                // Too big
                println!(">> Overflowed varint {}", value);
                return Ok(value);
            }
        }
    }
}

impl<U> VarIntRead for U where U: Read {}

pub trait UnsignedByteRead: Read {
    fn read_unsigned_byte(&mut self) -> std::io::Result<u8> {
        let mut byte_in = [0];
        self.read_exact(&mut byte_in)?;
        return Ok(byte_in[0]);
    }
}

impl<U> UnsignedByteRead for U where U: Read {}

pub trait VarIntWrite: UnsignedByteWrite {
    fn write_varint(&mut self, mut value: i32) -> std::io::Result<()> {
        loop {
            if (value & !SEGMENT_BITS as i32) == 0 {
                self.write_unsigned_byte(value as u8)?;
                return Ok(());
            }

            self.write_unsigned_byte(((value & SEGMENT_BITS as i32) | CONTINUE_BIT as i32) as u8)?;

            value >>= 7;
        }
    }
}

impl<U> VarIntWrite for U where U: Write {}

pub trait UnsignedByteWrite: Write {
    fn write_unsigned_byte(&mut self, value: u8) -> std::io::Result<usize> {
        return self.write(&[value]);
    }
}

impl<U> UnsignedByteWrite for U where U: Write {}
