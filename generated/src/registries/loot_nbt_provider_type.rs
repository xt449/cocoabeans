#![allow(non_camel_case_types, unused)]
#[derive(Copy, Clone)] pub enum LootNbtProviderTypeRegistry { r#context = 1, r#storage = 0, } impl crate::registries::Registry for LootNbtProviderTypeRegistry { fn get_protocol_id() -> u32 { return 33; } }