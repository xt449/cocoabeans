use crate::comparison::min_max;
use crate::coordinate::BlockPosition;

pub struct BoundingBox {
    // Min Corner
    pub x: i64,
    pub y: i64,
    pub z: i64,
    // Delta
    pub dx: u64,
    pub dy: u64,
    pub dz: u64,
}

// Constructors
impl BoundingBox {
    fn new_corners(x1: i64, y1: i64, z1: i64, x2: i64, y2: i64, z2: i64) -> Self {
        let x = min_max(x1, x2);
        let y = min_max(y1, y2);
        let z = min_max(z1, z2);

        return BoundingBox { x: x.0, y: y.0, z: z.0, dx: (x.1 - x.0) as u64, dy: (y.1 - y.0) as u64, dz: (z.1 - z.0) as u64 };
    }

    fn new_corner_positions(pos1: &BlockPosition, pos2: &BlockPosition) -> Self {
        return BoundingBox::new_corners(pos1.x as i64, pos1.y as i64, pos1.z as i64, pos2.x as i64, pos2.y as i64, pos2.z as i64);
    }
}

pub struct BoundingBox2 {
    // Min
    pub x1: i64,
    pub y1: i64,
    pub z1: i64,
    // Max
    pub x2: i64,
    pub y2: i64,
    pub z2: i64,
    hidden: bool,
}

impl BoundingBox2 {
    fn new_corners(x1: i64, y1: i64, z1: i64, x2: i64, y2: i64, z2: i64) -> Self {
        let x = min_max(x1, x2);
        let y = min_max(y1, y2);
        let z = min_max(z1, z2);

        return BoundingBox2 { x1: x.0, y1: y.0, z1: z.0, x2: x.1, y2: y.1, z2: z.1, hidden: false };
    }

    fn new_corner_positions(pos1: &BlockPosition, pos2: &BlockPosition) -> Self {
        return BoundingBox2::new_corners(pos1.x as i64, pos1.y as i64, pos1.z as i64, pos2.x as i64, pos2.y as i64, pos2.z as i64);
    }
}

// Getters
impl BoundingBox2 {
    fn get_dx(&self) -> i64 {
        return self.x2 - self.x1;
    }

    fn get_dy(&self) -> i64 {
        return self.y2 - self.y1;
    }

    fn get_dz(&self) -> i64 {
        return self.z2 - self.z1;
    }
}

pub struct FBoundingBox {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}
