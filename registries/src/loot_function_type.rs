pub enum LootFunctionTypeRegistry { r#apply_bonus = 15, r#copy_name = 12, r#copy_nbt = 20, r#copy_state = 21, r#enchant_randomly = 2, r#enchant_with_levels = 1, r#exploration_map = 10, r#explosion_decay = 17, r#fill_player_head = 19, r#furnace_smelt = 5, r#limit_count = 14, r#looting_enchant = 6, r#set_attributes = 8, r#set_banner_pattern = 22, r#set_contents = 13, r#set_count = 0, r#set_damage = 7, r#set_enchantments = 3, r#set_loot_table = 16, r#set_lore = 18, r#set_name = 9, r#set_nbt = 4, r#set_potion = 23, r#set_stew_effect = 11, } impl crate::Registry for LootFunctionTypeRegistry { fn get_protocol_id() -> u32 { return 30; } }