use std::io::{Read, Result, Write};

pub mod compound;
pub mod primitive;

pub trait ProtocolDataType<T>: Sized {
    fn read(read: &mut dyn Read) -> Result<Self>;
    fn write(&self, write: &mut dyn Write) -> Result<()>;
    fn unwrap(&self) -> T;
}
