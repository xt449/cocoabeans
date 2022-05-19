use registries::particle_type::ParticleTypeRegistry;
use blocks::BlockState;
use crate::data::item_stack::ItemStack;
use crate::data::position::Position;

pub enum Particle {
    SimpleParticle(ParticleTypeRegistry),
    BlockStateParticle(ParticleTypeRegistry, Box<dyn BlockState>),
    DustParticle{ red: f32, blue: f32, green: f32, scale: f32 },
    DustColorTransitionParticle{ red: f32, blue: f32, green: f32, scale: f32 },
    ItemParticle(ItemStack),
    BlockVibrationParticle{ origin: Position, block_position: Position, ticks: u32 },
    EntityVibrationParticle{ origin: Position, entity_id: i32, ticks: u32 },
}
