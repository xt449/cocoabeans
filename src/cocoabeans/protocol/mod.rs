pub(crate) mod client_connection;
pub(crate) mod packet;
mod stream_wrapper;
pub(crate) mod version_manager;
pub(crate) mod versions;

pub enum ConnectionState {
    HANDSHAKING = 0,
    STATUS = 1,
    LOGIN = 2,
    PLAY = 3,
}
