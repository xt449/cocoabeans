pub enum FloatProviderTypeRegistry { r#clamped_normal = 2, r#constant = 0, r#trapezoid = 3, r#uniform = 1, } impl crate::Registry for FloatProviderType { fn get_protocol_id() -> u32 { return 35; } }