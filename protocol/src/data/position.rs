use crate::io::{MinecraftReadable, MinecraftReader, MinecraftWritable, MinecraftWriter};

pub struct Position {
    pub x: i32,
    pub z: i32,
    pub y: i16,
}

impl MinecraftReadable<Self> for Position {
    fn deserialize_from(reader: &mut MinecraftReader) -> Result<Self, ()> {
        todo!()
    }
}

impl MinecraftWritable for Position {
    fn serialize_to(&self, writer: &mut MinecraftWriter) {
        writer.write_long(
            (((self.x & 0x3FFFFFF) as i64) << 38)
                | (((self.z & 0x3FFFFFF) as i64) << 12)
                | (self.y & 0xFFF) as i64,
        );
    }
}
