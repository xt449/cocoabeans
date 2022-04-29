pub mod packet;
pub mod stream_wrapper;
pub mod version_manager;
pub mod versions;

pub enum ConnectionState {
    HANDSHAKING = 0,
    STATUS = 1,
    LOGIN = 2,
    PLAY = 3,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
