pub enum WorldgenTrunkPlacerTypeRegistry { r#bending_trunk_placer = 6, r#dark_oak_trunk_placer = 4, r#fancy_trunk_placer = 5, r#forking_trunk_placer = 1, r#giant_trunk_placer = 2, r#mega_jungle_trunk_placer = 3, r#straight_trunk_placer = 0, } impl crate::Registry for WorldgenTrunkPlacerTypeRegistry { fn get_protocol_id() -> u32 { return 47; } }