pub enum LootNumberProviderTypeRegistry { r#binomial = 2, r#constant = 0, r#score = 3, r#uniform = 1, } impl crate::Registry for LootNumberProviderTypeRegistry { fn get_protocol_id() -> u32 { return 32; } }