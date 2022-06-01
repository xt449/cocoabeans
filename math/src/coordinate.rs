pub struct BlockPosition {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<Position> for BlockPosition {
    fn from(other: Position) -> Self {
        return BlockPosition { x: other.x as i32, y: other.y as i16, z: other.z as i32 };
    }
}

impl From<BlockPosition> for Position {
    fn from(other: BlockPosition) -> Self {
        return Position { x: other.x as f64, y: other.y as f64, z: other.z as f64 };
    }
}
