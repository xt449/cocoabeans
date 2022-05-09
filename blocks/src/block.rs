use crate::state::BlockState;

pub struct Block<S: BlockState> {
    pub state: S
}
