pub enum FluidRegistry { r#empty = 0, r#flowing_lava = 3, r#flowing_water = 1, r#lava = 4, r#water = 2, } impl crate::Registry for FluidRegistry { fn get_protocol_id() -> u32 { return 2; } } impl Default for FluidRegistry {fn default() -> Self { return FluidRegistry::minecraft:empty; } }