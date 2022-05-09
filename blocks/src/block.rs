use math::coordinate::Position;
use crate::state::BlockState;

// TODO - unnecessary wrapper?
pub struct Block<S: BlockState> {
    pub state: S,
    pub position: Position,
}
