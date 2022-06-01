use std::io::{Error, ErrorKind, Read, Result, Take};
use crate::{Packet, State};

pub mod clientbound;
pub mod serverbound;

pub fn build_packet(connection_state: State, packet_id: u32, reader: &mut Take<&mut dyn Read>) -> Result<Box<dyn Packet>> {
    return match connection_state {
        State::HANDSHAKING => HANDSHAKING.get(packet_id as usize).ok_or(Error::new(ErrorKind::InvalidData, "Unknown packet id"))?(reader),
        State::PLAY => PLAY.get(packet_id as usize).ok_or(Error::new(ErrorKind::InvalidData, "Unknown packet id"))?(reader),
        State::STATUS => STATUS.get(packet_id as usize).ok_or(Error::new(ErrorKind::InvalidData, "Unknown packet id"))?(reader),
        State::LOGIN => LOGIN.get(packet_id as usize).ok_or(Error::new(ErrorKind::InvalidData, "Unknown packet id"))?(reader),
    }
}

const HANDSHAKING: [serverbound::PacketBuilder; 1] = [
    serverbound::LoginPluginResponsePacket::BUILDER,
];
const PLAY: [serverbound::PacketBuilder; 2] = [
    serverbound::StatusRequestPacket::BUILDER,
    serverbound::StatusPingPacket::BUILDER,
];
const STATUS: [serverbound::PacketBuilder; 3] = [
    serverbound::LoginStartPacket::BUILDER,
    serverbound::LoginEncryptionResponsePacket::BUILDER,
    serverbound::LoginPluginResponsePacket::BUILDER,
];
const LOGIN: [serverbound::PacketBuilder; 48] = [
    serverbound::PlayTeleportConfirmPacket::BUILDER,
    serverbound::PlayQueryBlockNBTPacket::BUILDER,
    serverbound::PlaySetDifficultyPacket::BUILDER,
    serverbound::PlayChatMessagePacket::BUILDER,
    serverbound::PlayClientStatusPacket::BUILDER,
    serverbound::PlayClientSettingsPacket::BUILDER,
    serverbound::PlayTabCompletePacket::BUILDER,
    serverbound::PlayClickWindowButtonPacket::BUILDER,
    serverbound::PlayClickWindowPacket::BUILDER,
    serverbound::PlayCloseWindowPacket::BUILDER,
    serverbound::PlayPluginMessagePacket::BUILDER,
    serverbound::PlayEditBookPacket::BUILDER,
    serverbound::PlayQueryEntityNBTPacket::BUILDER,
    serverbound::PlayInteractEntityPacket::BUILDER,
    serverbound::PlayGenerateStructurePacket::BUILDER,
    serverbound::PlayKeepAlivePacket::BUILDER,
    serverbound::PlayLockDifficultyPacket::BUILDER,
    serverbound::PlayPlayerPositionPacket::BUILDER,
    serverbound::PlayPlayerPositionAndRotationPacket::BUILDER,
    serverbound::PlayPlayerRotationPacket::BUILDER,
    serverbound::PlayPlayerMovementPacket::BUILDER,
    serverbound::PlayVehicleMovePacket::BUILDER,
    serverbound::PlaySteerBoatPacket::BUILDER,
    serverbound::PlayPickItemPacket::BUILDER,
    serverbound::PlayCraftRecipeRequestPacket::BUILDER,
    serverbound::PlayPlayerAbilitiesPacket::BUILDER,
    serverbound::PlayPlayerDiggingPacket::BUILDER,
    serverbound::PlayEntityActionPacket::BUILDER,
    serverbound::PlaySteerVehiclePacket::BUILDER,
    serverbound::PlayPongPacket::BUILDER,
    serverbound::PlaySetRecipeBookStatePacket::BUILDER,
    serverbound::PlaySetDisplayedRecipePacket::BUILDER,
    serverbound::PlayNameItemPacket::BUILDER,
    serverbound::PlayResourcePackStatusPacket::BUILDER,
    serverbound::PlayAdvancementTabPacket::BUILDER,
    serverbound::PlaySelectTradePacket::BUILDER,
    serverbound::PlaySetBeaconEffectPacket::BUILDER,
    serverbound::PlayHeldItemChangePacket::BUILDER,
    serverbound::PlayUpdateCommandBlockPacket::BUILDER,
    serverbound::PlayUpdateCommandBlockMinecartPacket::BUILDER,
    serverbound::PlayCreativeInventoryActionPacket::BUILDER,
    serverbound::PlayUpdateJigsawBlockPacket::BUILDER,
    serverbound::PlayUpdateStructureBlockPacket::BUILDER,
    serverbound::PlayUpdateSignBlockPacket::BUILDER,
    serverbound::PlayAnimationPacket::BUILDER,
    serverbound::PlaySpectatePacket::BUILDER,
    serverbound::PlayPlayerBlockPlacementPacket::BUILDER,
    serverbound::PlayUseItemPacket::BUILDER,
];
