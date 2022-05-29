use registries::particle_type::ParticleTypeRegistry;
use std::ops::Deref;

use blocks::BlockState;
use math::coordinate::BlockPosition;

use crate::data::item_stack::ItemStack;
use crate::io::{MinecraftWritable, MinecraftWriter};

pub enum Particle {
    SimpleParticle(ParticleTypeRegistry),
    BlockStateParticle(ParticleTypeRegistry, Box<dyn BlockState>),
    DustParticle { red: f32, blue: f32, green: f32, scale: f32 },
    DustColorTransitionParticle { red: f32, blue: f32, green: f32, scale: f32 },
    ItemParticle(ItemStack),
    BlockVibrationParticle { origin: BlockPosition, block_position: BlockPosition, ticks: u32 },
    EntityVibrationParticle { origin: BlockPosition, entity_id: i32, ticks: u32 },
}

const VIBRATION_BLOCK: &str = "minecraft:block";
const VIBRATION_ENTITY: &str = "minecraft:entity";

impl Particle {
    pub fn get_id(&self) -> i32 {
        return match self {
            Particle::SimpleParticle(p) => *p as usize as i32,
            Particle::BlockStateParticle(p, _) => *p as usize as i32,
            Particle::DustParticle { .. } => ParticleTypeRegistry::dust as usize as i32,
            Particle::DustColorTransitionParticle { .. } => ParticleTypeRegistry::dust_color_transition as usize as i32,
            Particle::ItemParticle(_) => ParticleTypeRegistry::item as usize as i32,
            Particle::BlockVibrationParticle { .. } => ParticleTypeRegistry::vibration as usize as i32,
            Particle::EntityVibrationParticle { .. } => ParticleTypeRegistry::vibration as usize as i32,
        };
    }
}

impl MinecraftWritable for Particle {
    fn serialize_to(&self, writer: &mut MinecraftWriter) {
        writer.write_varint(self.get_id());
        match self {
            Particle::SimpleParticle(_) => { /*nothing extra*/ }
            Particle::BlockStateParticle(_, blockstate) => writer.write_varint(blocks::get_id_from_blockstate(blockstate.deref()) as usize as i32),
            Particle::DustParticle { red, green, blue, scale } => {
                writer.write_float(*red);
                writer.write_float(*green);
                writer.write_float(*blue);
                writer.write_float(*scale);
            }
            Particle::DustColorTransitionParticle { red, green, blue, scale } => {
                writer.write_float(*red);
                writer.write_float(*green);
                writer.write_float(*blue);
                writer.write_float(*scale);
            }
            Particle::ItemParticle(item) => writer.write(item),
            Particle::BlockVibrationParticle { origin, block_position, ticks } => {
                // writer.write(origin);// TODO
                writer.write_utf(VIBRATION_BLOCK);
                // writer.write(block_position);// TODO
                writer.write_varint(*ticks as i32);
            }
            Particle::EntityVibrationParticle { origin, entity_id, ticks } => {
                // writer.write(origin);// TODO
                writer.write_utf(VIBRATION_ENTITY);
                writer.write_varint(*entity_id);
                writer.write_varint(*ticks as i32);
            }
        }
    }
}
