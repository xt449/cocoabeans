pub enum WorldgenFoliagePlacerTypeRegistry { r#acacia_foliage_placer = 3, r#blob_foliage_placer = 0, r#bush_foliage_placer = 4, r#dark_oak_foliage_placer = 8, r#fancy_foliage_placer = 5, r#jungle_foliage_placer = 6, r#mega_pine_foliage_placer = 7, r#pine_foliage_placer = 2, r#random_spread_foliage_placer = 9, r#spruce_foliage_placer = 1, } impl crate::Registry for WorldgenFoliagePlacerTypeRegistry { fn get_protocol_id() -> u32 { return 46; } }