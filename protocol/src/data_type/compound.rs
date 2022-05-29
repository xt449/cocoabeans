use crate::data::io::{ReadVarIntExt, WriteVarIntExt};
use crate::data_type::ProtocolDataType;
use std::io::{Error, ErrorKind, Read, Result, Write};

pub struct SizedString<const N: usize>(String);

impl<const N: usize> ProtocolDataType<String> for SizedString<N> {
    fn read(mut read: &mut dyn Read) -> Result<Self> {
        let length = read.read_varint()? as usize;
        if length > N {
            return Err(Error::new(ErrorKind::InvalidData, "String too long"));
        }

        let mut buf = Vec::with_capacity(length);
        read.take(length as u64).read_to_end(&mut buf)?;

        return Ok(Self(String::from_utf8(buf).map_err(|_| Error::new(ErrorKind::InvalidData, "String had invalid UTF8 format"))?));
    }

    fn write(&self, mut write: &mut dyn Write) -> Result<()> {
        let bytes = self.0.as_bytes();
        write.write_varint(bytes.len() as i32)?;
        return write.write_all(bytes);
    }

    fn unwrap(&self) -> String {
        self.0.to_string()
    }
}
