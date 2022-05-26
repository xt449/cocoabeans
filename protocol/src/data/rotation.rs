use crate::io::{MinecraftReadable, MinecraftReader, MinecraftWritable, MinecraftWriter};

pub struct Rotation {
    pub x: f32,
    pub z: f32,
    pub y: f32,
}

impl MinecraftReadable<Self> for Rotation {
    fn deserialize_from(reader: &mut MinecraftReader) -> std::io::Result<Self> {
        return Ok(Rotation {
            x: reader.read_float()?,
            y: reader.read_float()?,
            z: reader.read_float()?,
        });
    }
}

impl MinecraftWritable for Rotation {
    fn serialize_to(&self, writer: &mut MinecraftWriter) {
        writer.write_float(self.x);
        writer.write_float(self.y);
        writer.write_float(self.z);
    }
}
