pub enum WorldgenChunkGeneratorRegistry { r#debug = 2, r#flat = 1, r#noise = 0, } impl crate::Registry for WorldgenChunkGeneratorRegistry { fn get_protocol_id() -> u32 { return 51; } }