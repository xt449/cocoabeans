pub enum RuleTestRegistry { r#always_true = 0, r#block_match = 1, r#blockstate_match = 2, r#random_block_match = 4, r#random_blockstate_match = 5, r#tag_match = 3, } impl crate::Registry for RuleTest { fn get_protocol_id() -> u32 { return 14; } }