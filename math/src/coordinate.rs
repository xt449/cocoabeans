pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

pub struct FPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<FPosition> for Position {
    fn from(other: FPosition) -> Self {
        return Position {
            x: other.x as i64,
            y: other.y as i64,
            z: other.z as i64,
        };
    }
}

impl From<Position> for FPosition {
    fn from(other: Position) -> Self {
        return FPosition {
            x: other.x as f64,
            y: other.y as f64,
            z: other.z as f64,
        };
    }
}
