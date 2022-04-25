use bytes::{Buf, BytesMut};

pub trait Bytable<T> {
    fn read(buffer: &mut BytesMut) -> T;
    fn write(&self, buffer: &mut BytesMut);
}

pub type VarInt = i32;

const SEGMENT_BITS: i8 = 0x7F;
const CONTINUE_BIT: i8 = -0x01;

impl Bytable<VarInt> for VarInt {
    fn read(buffer: &mut BytesMut) -> VarInt {
        let mut value: i32 = 0;
        let mut position = 0;

        loop {
            let current_byte = buffer.get_i8();
            value |= i32::from(current_byte & SEGMENT_BITS) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                return VarInt::from(value);
            }

            position += 7;

            if position >= 32 {
                return VarInt::from(value);
            }
        }
    }

    fn write(&self, buffer: &mut BytesMut) {
        todo!()
    }
}
