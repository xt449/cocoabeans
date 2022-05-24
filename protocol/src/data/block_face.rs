pub enum BlockFace {
    Bottom = 0,
    Top = 1,
    North = 2,
    South = 3,
    West = 4,
    East = 5,
}

impl BlockFace {
    pub fn get_position_offset(&self) -> (i8, i8, i8) {
        return match self {
            BlockFace::Bottom => (0, -1, 0),
            BlockFace::Top => (0, 1, 0),
            BlockFace::North => (0, 0, -1),
            BlockFace::South => (0, 0, 1),
            BlockFace::West => (-1, 0, 0),
            BlockFace::East => (1, 0, 0),
        };
    }
}
