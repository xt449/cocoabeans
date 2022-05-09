pub enum GameEventRegistry { r#block_attach = 0, r#block_change = 1, r#block_close = 2, r#block_destroy = 3, r#block_detach = 4, r#block_open = 5, r#block_place = 6, r#block_press = 7, r#block_switch = 8, r#block_unpress = 9, r#block_unswitch = 10, r#container_close = 11, r#container_open = 12, r#dispense_fail = 13, r#drinking_finish = 14, r#eat = 15, r#elytra_free_fall = 16, r#entity_damaged = 17, r#entity_killed = 18, r#entity_place = 19, r#equip = 20, r#explode = 21, r#fishing_rod_cast = 22, r#fishing_rod_reel_in = 23, r#flap = 24, r#fluid_pickup = 25, r#fluid_place = 26, r#hit_ground = 27, r#lightning_strike = 29, r#minecart_moving = 30, r#mob_interact = 28, r#piston_contract = 31, r#piston_extend = 32, r#prime_fuse = 33, r#projectile_land = 34, r#projectile_shoot = 35, r#ravager_roar = 36, r#ring_bell = 37, r#shear = 38, r#shulker_close = 39, r#shulker_open = 40, r#splash = 41, r#step = 42, r#swim = 43, r#wolf_shaking = 44, } impl crate::Registry for GameEvent { fn get_protocol_id() -> u32 { return 0; } } impl Default for GameEvent {fn default() -> Self { return GameEvent::minecraft:step; } }