use registries::item::ItemRegistry;

use nbt::lib::Blob;

use crate::io::{MinecraftReadable, MinecraftReader, MinecraftWritable, MinecraftWriter};

pub struct ItemStack {
    pub count: u8,
    pub id: ItemRegistry,
    nbt: Option<Blob>,
}

impl ItemStack {
    pub fn empty() -> Self {
        return ItemStack {
            count: 0,
            id: ItemRegistry::air,
            nbt: None,
        };
    }

    pub fn new(item: ItemRegistry, size: u8) -> Self {
        return ItemStack {
            count: size,
            id: item,
            nbt: None,
        };
    }

    pub fn new_single(item: ItemRegistry) -> Self {
        return ItemStack {
            count: 1,
            id: item,
            nbt: None,
        };
    }
}

impl ItemStack {
    pub fn init_nbt(&mut self) {
        if let None = self.nbt {
            self.nbt = Some(Blob::new());
        }
    }

    pub fn get_nbt(&self) -> &Option<Blob> {
        return &self.nbt;
    }
}

impl MinecraftReadable<Self> for ItemStack {
    fn deserialize_from(reader: &mut MinecraftReader) -> Result<Self, ()> {
        todo!()
    }
}

impl MinecraftWritable for ItemStack {
    fn serialize_to(&self, writer: &mut MinecraftWriter) {
        writer.write_boolean(self.count > 0);
        if self.count > 0 {
            writer.write_varint(self.id as usize as i32);
            writer.write_unsigned_byte(self.count);
            writer.write_json(&self.nbt);
        }
    }
}
