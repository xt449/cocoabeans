pub(crate) mod clientbound_packets;
pub(crate) mod packet_listener;
pub(crate) mod serverbound_packets;

struct Packet {
    state: crate::cocoabeans::protocol::definition::ConnectionState,
    direction: crate::cocoabeans::protocol::definition::Direction,
}
