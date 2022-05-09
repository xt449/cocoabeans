use crate::state::BlockState;

pub trait Block {
    fn get_state(&self) -> dyn BlockState;
}
