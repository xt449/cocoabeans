pub enum RecipeTypeRegistry { r#blasting = 2, r#campfire_cooking = 4, r#crafting = 0, r#smelting = 1, r#smithing = 6, r#smoking = 3, r#stonecutting = 5, } impl crate::Registry for RecipeTypeRegistry { fn get_protocol_id() -> u32 { return 17; } }