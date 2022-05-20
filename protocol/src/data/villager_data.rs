use crate::io::{MinecraftWritable, MinecraftWriter};
use registries::villager_profession::VillagerProfessionRegistry;
use registries::villager_type::VillagerTypeRegistry;

pub struct VillagerData {
    r#type: VillagerTypeRegistry,
    profession: VillagerProfessionRegistry,
    level: u8,
}

impl MinecraftWritable for VillagerData {
    fn serialize_to(&self, writer: &mut MinecraftWriter) {
        writer.write_varint(self.r#type as usize as i32);
        writer.write_varint(self.profession as usize as i32);
        writer.write_varint(self.level as i32);
    }
}
