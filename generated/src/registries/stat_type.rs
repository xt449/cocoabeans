#![allow(non_camel_case_types, unused)]
#[derive(Copy, Clone)] pub enum StatTypeRegistry { r#broken = 3, r#crafted = 1, r#custom = 8, r#dropped = 5, r#killed = 6, r#killed_by = 7, r#mined = 0, r#picked_up = 4, r#used = 2, } impl crate::registries::Registry for StatTypeRegistry { fn get_protocol_id() -> u32 { return 21; } }