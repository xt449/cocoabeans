pub enum CustomStatRegistry { r#animals_bred = 32, r#aviate_one_cm = 18, r#bell_ring = 68, r#boat_one_cm = 15, r#clean_armor = 40, r#clean_banner = 41, r#clean_shulker_box = 42, r#climb_one_cm = 11, r#crouch_one_cm = 7, r#damage_absorbed = 28, r#damage_blocked_by_shield = 27, r#damage_dealt = 23, r#damage_dealt_absorbed = 24, r#damage_dealt_resisted = 25, r#damage_resisted = 29, r#damage_taken = 26, r#deaths = 30, r#drop = 22, r#eat_cake_slice = 37, r#enchant_item = 53, r#fall_one_cm = 10, r#fill_cauldron = 38, r#fish_caught = 34, r#fly_one_cm = 12, r#horse_one_cm = 17, r#inspect_dispenser = 47, r#inspect_dropper = 45, r#inspect_hopper = 46, r#interact_with_anvil = 71, r#interact_with_beacon = 44, r#interact_with_blast_furnace = 61, r#interact_with_brewingstand = 43, r#interact_with_campfire = 64, r#interact_with_cartography_table = 65, r#interact_with_crafting_table = 56, r#interact_with_furnace = 55, r#interact_with_grindstone = 72, r#interact_with_lectern = 63, r#interact_with_loom = 66, r#interact_with_smithing_table = 74, r#interact_with_smoker = 62, r#interact_with_stonecutter = 67, r#jump = 21, r#leave_game = 0, r#minecart_one_cm = 14, r#mob_kills = 31, r#open_barrel = 60, r#open_chest = 57, r#open_enderchest = 52, r#open_shulker_box = 59, r#pig_one_cm = 16, r#play_noteblock = 48, r#play_record = 54, r#play_time = 1, r#player_kills = 33, r#pot_flower = 50, r#raid_trigger = 69, r#raid_win = 70, r#sleep_in_bed = 58, r#sneak_time = 5, r#sprint_one_cm = 8, r#strider_one_cm = 20, r#swim_one_cm = 19, r#talked_to_villager = 35, r#target_hit = 73, r#time_since_death = 3, r#time_since_rest = 4, r#total_world_time = 2, r#traded_with_villager = 36, r#trigger_trapped_chest = 51, r#tune_noteblock = 49, r#use_cauldron = 39, r#walk_on_water_one_cm = 9, r#walk_one_cm = 6, r#walk_under_water_one_cm = 13, } impl crate::Registry for CustomStat { fn get_protocol_id() -> u32 { return 12; } }