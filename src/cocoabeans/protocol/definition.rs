pub enum ConnectionState {
    HANDSHAKING = 0,
    STATUS = 1,
    LOGIN = 2,
    PLAY = 3,
}

pub enum Direction {
    SERVERBOUND = 0,
    CLIENTBOUND = 1,
}
