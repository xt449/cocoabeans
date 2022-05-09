#![allow(non_camel_case_types, unused)]
#[derive(Copy, Clone)] pub enum WorldgenMaterialConditionRegistry { r#above_preliminary_surface = 9, r#biome = 0, r#hole = 8, r#noise_threshold = 1, r#not = 7, r#steep = 6, r#stone_depth = 10, r#temperature = 5, r#vertical_gradient = 2, r#water = 4, r#y_above = 3, } impl crate::registries::Registry for WorldgenMaterialConditionRegistry { fn get_protocol_id() -> u32 { return 52; } }