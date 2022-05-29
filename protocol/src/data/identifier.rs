use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind};

use crate::io::{MinecraftReadable, MinecraftReader, MinecraftWritable, MinecraftWriter};

pub struct Identifier {
    namespace: String,
    pub key: String,
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
    pub fn from_format(identifier: String) -> std::io::Result<Self> {
        let split = identifier.split(':').collect::<Vec<&str>>();
        return match split.len() {
            1 => Ok(Identifier {
                namespace: "minecraft".to_owned(),
                key: split[1].to_owned(),
            }),
            2 => Ok(Identifier {
                namespace: split[0].to_owned(),
                key: split[1].to_owned(),
            }),
            _ => Err(Error::new(ErrorKind::InvalidData, "Could not split identifer into 2 distince parts")),
        };
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return f.write_fmt(format_args!("{}:{}", self.namespace, self.key));
    }
}

impl MinecraftReadable<Self> for Identifier {
    fn deserialize_from(reader: &mut MinecraftReader) -> std::io::Result<Self> {
        return Self::from_format(reader.read_string()?);
    }
}

impl MinecraftWritable for Identifier {
    fn serialize_to(&self, writer: &mut MinecraftWriter) {
        writer.write_utf(&self.to_string());
    }
}
