use generated::registries::item::ItemRegistry;
use nbt::lib::Blob;

pub struct ItemStack {
    pub count: u8,
    pub id: Option<ItemRegistry>,
    nbt: Option<Blob>,
}

impl ItemStack {
    pub fn get_nbt(&self) -> &Option<Blob> {
        return &self.nbt;
    }
}
