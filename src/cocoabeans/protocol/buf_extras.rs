use crate::cocoabeans::protocol::types;
use crate::cocoabeans::protocol::types::Bytable;
use bytes;

pub fn get_var_int(buffer: &mut bytes::BytesMut) -> types::VarInt {
    types::VarInt::read(buffer)
}
