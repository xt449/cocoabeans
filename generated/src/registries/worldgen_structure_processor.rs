#![allow(non_camel_case_types, unused)]
#[derive(Copy, Clone)] pub enum WorldgenStructureProcessorRegistry { r#blackstone_replace = 7, r#block_age = 6, r#block_ignore = 0, r#block_rot = 1, r#gravity = 2, r#jigsaw_replacement = 3, r#lava_submerged_block = 8, r#nop = 5, r#protected_blocks = 9, r#rule = 4, } impl crate::registries::Registry for WorldgenStructureProcessorRegistry { fn get_protocol_id() -> u32 { return 55; } }