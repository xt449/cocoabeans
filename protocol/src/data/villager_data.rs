use generated::registries::villager_type::VillagerTypeRegistry;
use generated::registries::villager_profession::VillagerProfessionRegistry;

pub struct VillagerData {
    r#type: VillagerTypeRegistry,
    profession: VillagerProfessionRegistry,
    level: u8,
}
