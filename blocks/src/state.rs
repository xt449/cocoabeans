use crate::property::*;

pub trait BlockState {
    fn get_id(&self) -> u32;
}

pub trait Facing2D {
    fn get_facing_2d_state(&self) -> Facing2DState;
}

pub trait Facing3D {
    fn get_facing_3d_state(&self) -> Facing3DState;
}

pub trait Waterlogged {
    fn get_water_logged_state(&self) -> bool;
}

pub trait BambooAge {
    fn get_bamboo_age_state(&self) -> BambooAgeState;
}

pub trait BambooLeaves {
    fn get_bamboo_leaves_state(&self) -> BambooLeavesState;
}

pub trait BambooStage {
    fn get_bamboo_stage_state(&self) -> BambooStageState;
}

pub trait Rotation {
    fn get_rotation_state(&self) -> RotationState;
}

pub trait BarrelOpen {
    fn get_barrel_open_state(&self) -> BarrelOpenState;
}

pub trait Axis {
    fn get_axis_state(&self) -> AxisState;
}

pub trait BedOccupied {
    fn get_bed_occupied_state(&self) -> BedOccupiedState;
}

pub trait BedPart {
    fn get_bed_part_state(&self) -> BedPartState;
}

pub trait BeehiveHoneyLevel {
    fn get_beehive_honey_level_state(&self) -> BeehiveHoneyLevelState;
}

pub trait BeetrootsAge {
    fn get_beetroots_age_state(&self) -> BeetrootsAgeState;
}

pub trait BellAttachment {
    fn get_bell_attachment_state(&self) -> BellAttachmentState;
}

pub trait DripleafTilt {
    fn get_dripleaf_tilt_state(&self) -> DripleafTiltState;
}

pub trait FurnaceLit {
    fn get_furnace_lit_state(&self) -> FurnaceLitState;
}

pub trait BrewingStandHasBottle0 {
    fn get_brewing_stand_has_bottle_0_state(&self) -> BrewingStandHasBottle0State;
}

pub trait BrewingStandHasBottle1 {
    fn get_brewing_stand_has_bottle_1_state(&self) -> BrewingStandHasBottle1State;
}

pub trait BrewingStandHasBottle2 {
    fn get_brewing_stand_has_bottle_2_state(&self) -> BrewingStandHasBottle2State;
}

pub trait BubbleColumnDrag {
    fn get_bubble_column_drag_state(&self) -> BubbleColumnDragState;
}

pub trait Face {
    fn get_face_state(&self) -> FaceState;
}

pub trait CactusAge {
    fn get_cactus_age_state(&self) -> CactusAgeState;
}

pub trait CakeBites {
    fn get_cake_bites_state(&self) -> CakeBitesState;
}

pub trait CakeLit {
    fn get_cake_lit_state(&self) -> CakeLitState;
}

pub trait CampfireLit {
    fn get_campfire_lit_state(&self) -> CampfireLitState;
}

pub trait CampfireSignalFire {
    fn get_campfire_signal_fire_state(&self) -> CampfireSignalFireState;
}

pub trait Candles {
    fn get_candles_state(&self) -> CandlesState;
}

pub trait CandlesLit {
    fn get_candles_lit_state(&self) -> CandlesLitState;
}

pub trait CarrotsAge {
    fn get_carrots_age_state(&self) -> CarrotsAgeState;
}

pub trait CauldronLevel {
    fn get_cauldron_level_state(&self) -> CauldronLevelState;
}

pub trait CaveVinesAge {
    fn get_cave_vines_age_state(&self) -> CaveVinesAgeState;
}

pub trait CaveVinesPlantBerries {
    fn get_cave_vines_plant_berries_state(&self) -> CaveVinesPlantBerriesState;
}

pub trait ChestType {
    fn get_chest_type_state(&self) -> ChestTypeState;
}

pub trait ChorusFlowerAge {
    fn get_chorus_flower_age_state(&self) -> ChorusFlowerAgeState;
}

pub trait Down {
    fn get_down_state(&self) -> DownState;
}

pub trait East {
    fn get_east_state(&self) -> EastState;
}

pub trait North {
    fn get_north_state(&self) -> NorthState;
}

pub trait South {
    fn get_south_state(&self) -> SouthState;
}

pub trait Up {
    fn get_up_state(&self) -> UpState;
}

pub trait West {
    fn get_west_state(&self) -> WestState;
}
