use bytes;

use crate::cocoabeans::protocol::types;
use crate::cocoabeans::protocol::types::Bytable;

pub fn get_var_int(buffer: &mut bytes::BytesMut) -> types::VarInt {
    types::VarInt::read(buffer)
}
