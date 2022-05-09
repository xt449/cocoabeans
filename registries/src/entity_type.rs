pub enum EntityTypeRegistry { r#area_effect_cloud = 0, r#armor_stand = 1, r#arrow = 2, r#axolotl = 3, r#bat = 4, r#bee = 5, r#blaze = 6, r#boat = 7, r#cat = 8, r#cave_spider = 9, r#chest_minecart = 51, r#chicken = 10, r#cod = 11, r#command_block_minecart = 52, r#cow = 12, r#creeper = 13, r#dolphin = 14, r#donkey = 15, r#dragon_fireball = 16, r#drowned = 17, r#egg = 89, r#elder_guardian = 18, r#end_crystal = 19, r#ender_dragon = 20, r#ender_pearl = 90, r#enderman = 21, r#endermite = 22, r#evoker = 23, r#evoker_fangs = 24, r#experience_bottle = 91, r#experience_orb = 25, r#eye_of_ender = 26, r#falling_block = 27, r#fireball = 43, r#firework_rocket = 28, r#fishing_bobber = 112, r#fox = 29, r#furnace_minecart = 53, r#ghast = 30, r#giant = 31, r#glow_item_frame = 32, r#glow_squid = 33, r#goat = 34, r#guardian = 35, r#hoglin = 36, r#hopper_minecart = 54, r#horse = 37, r#husk = 38, r#illusioner = 39, r#iron_golem = 40, r#item = 41, r#item_frame = 42, r#leash_knot = 44, r#lightning_bolt = 45, r#llama = 46, r#llama_spit = 47, r#magma_cube = 48, r#marker = 49, r#minecart = 50, r#mooshroom = 58, r#mule = 57, r#ocelot = 59, r#painting = 60, r#panda = 61, r#parrot = 62, r#phantom = 63, r#pig = 64, r#piglin = 65, r#piglin_brute = 66, r#pillager = 67, r#player = 111, r#polar_bear = 68, r#potion = 92, r#pufferfish = 70, r#rabbit = 71, r#ravager = 72, r#salmon = 73, r#sheep = 74, r#shulker = 75, r#shulker_bullet = 76, r#silverfish = 77, r#skeleton = 78, r#skeleton_horse = 79, r#slime = 80, r#small_fireball = 81, r#snow_golem = 82, r#snowball = 83, r#spawner_minecart = 55, r#spectral_arrow = 84, r#spider = 85, r#squid = 86, r#stray = 87, r#strider = 88, r#tnt = 69, r#tnt_minecart = 56, r#trader_llama = 94, r#trident = 93, r#tropical_fish = 95, r#turtle = 96, r#vex = 97, r#villager = 98, r#vindicator = 99, r#wandering_trader = 100, r#witch = 101, r#wither = 102, r#wither_skeleton = 103, r#wither_skull = 104, r#wolf = 105, r#zoglin = 106, r#zombie = 107, r#zombie_horse = 108, r#zombie_villager = 109, r#zombified_piglin = 110, } impl crate::Registry for EntityTypeRegistry { fn get_protocol_id() -> u32 { return 6; } } impl Default for EntityTypeRegistry {fn default() -> Self { return EntityTypeRegistry::minecraft:pig; } }