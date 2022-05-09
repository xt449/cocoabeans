use crate::state::BlockState;
use math::coordinate::Position;

// TODO - unnecessary wrapper?
pub struct Block<S: BlockState> {
    pub state: S,
    pub position: Position,
}
