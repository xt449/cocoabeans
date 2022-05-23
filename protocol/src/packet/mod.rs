pub mod clientbound;
pub mod packet_handler;
pub mod serverbound;

#[deprecated]
pub trait Packet {
    // TODO - used for checking correct 1.18.2 packet ids
    const ID: u8;
}
