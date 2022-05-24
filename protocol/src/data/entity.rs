pub enum Action {
    SneakingStart = 0,
    SneakingStop = 1,
    BedLeave = 2,
    SprintingStart = 3,
    SprintingStop = 4,
    HorseJumpStart = 5,
    HorseJumpStop = 6,
    HorseInventory = 7,
    ElytraFlyingStart = 8,
}

pub enum Status {
    DiggingStart = 0,
    DiggingCancel = 1,
    DiggingFinish = 2,
    DropItemStack = 3,
    DropItemSingle = 4,
    UseItem = 5,
    SetItemOffHand = 6,
}

pub enum InteractionType {
    Interact = 0,
    Attack = 1,
    InteractAt = 2,
}
