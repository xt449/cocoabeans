#![allow(non_camel_case_types, unused)]
#[derive(Copy, Clone)] pub enum WorldgenBiomeSourceRegistry { r#checkerboard = 2, r#fixed = 0, r#multi_noise = 1, r#the_end = 3, } impl crate::registries::Registry for WorldgenBiomeSourceRegistry { fn get_protocol_id() -> u32 { return 50; } }