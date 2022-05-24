pub enum Action {
    UpdateData = 0,
    SaveStructure = 1,
    LoadStructure = 2,
    DetectSize = 3,
}

pub enum Mode {
    Save = 0,
    Load = 1,
    Corner = 2,
    Data = 3,
}

pub enum Mirror {
    None = 0,
    LeftRight = 1,
    FrontBack = 2,
}

pub enum Rotation {
    None = 0,
    Clockwise90 = 1,
    Clockwise180 = 2,
    CounterClockwise90 = 3,
}
