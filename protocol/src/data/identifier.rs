use std::fmt::{Display, Formatter};

use crate::io::{MinecraftReadable, MinecraftReader, MinecraftWritable, MinecraftWriter};

pub struct Identifier {
    namespace: String,
    key: String,
}

impl Identifier {
    pub fn new_minecraft(key: &str) -> Self {
        return Identifier {
            namespace: "minecraft".to_owned(),
            key: key.to_owned(),
        };
    }
    pub fn new_other(namespace: &str, key: &str) -> Self {
        return Identifier {
            namespace: namespace.to_owned(),
            key: key.to_owned(),
        };
    }
    pub fn from_format(identifier: String) -> Result<Self, ()> {
        let split = identifier.split(':').collect::<Vec<&str>>();
        if split.len() == 2 {
            return Ok(Identifier {
                namespace: split[0].to_owned(),
                key: split[1].to_owned(),
            });
        }
        return Err(());
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return f.write_fmt(format_args!("{}:{}", self.namespace, self.key));
    }
}

impl MinecraftReadable<Self> for Identifier {
    fn deserialize_from(reader: &mut MinecraftReader) -> Result<Self, ()> {
        return Self::from_format(reader.read_utf());
    }
}

impl MinecraftWritable for Identifier {
    fn serialize_to(&self, writer: &mut MinecraftWriter) {
        writer.write_utf(&self.to_string());
    }
}
