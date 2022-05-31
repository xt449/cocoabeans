use blocks::BlockState;
use math::coordinate::{BlockPosition, Position};
use nbt::Value;
use registries::item::ItemRegistry;
use registries::particle_type::ParticleTypeRegistry;
use registries::villager_profession::VillagerProfessionRegistry;
use registries::villager_type::VillagerTypeRegistry;
use serde::{Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind};

pub mod io;

// Destination

pub enum Destination {
    Entity { id: u32 },
    Block { position: BlockPosition },
}

// Particle

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
            Particle::SimpleParticle(p) => *p as i32,
            Particle::BlockStateParticle(p, _) => *p as i32,
            Particle::DustParticle { .. } => ParticleTypeRegistry::dust as i32,
            Particle::DustColorTransitionParticle { .. } => ParticleTypeRegistry::dust_color_transition as i32,
            Particle::ItemParticle(_) => ParticleTypeRegistry::item as i32,
            Particle::BlockVibrationParticle { .. } => ParticleTypeRegistry::vibration as i32,
            Particle::EntityVibrationParticle { .. } => ParticleTypeRegistry::vibration as i32,
        };
    }
}

// Rotation

pub struct Rotation {
    pub x: f32,
    pub z: f32,
    pub y: f32,
}

// VillagerData

pub struct VillagerData {
    r#type: VillagerTypeRegistry,
    profession: VillagerProfessionRegistry,
    level: u8,
}

// ItemStack

pub struct ItemStack {
    pub count: u8,
    pub id: ItemRegistry,
    pub nbt: Value,
}

impl ItemStack {
    pub fn empty() -> Self {
        return ItemStack { count: 0, id: ItemRegistry::air, nbt: Value::Compound(HashMap::new()) };
    }

    // pub fn new(item: ItemRegistry, size: u8) -> Self {
    //     return ItemStack {
    //         count: size,
    //         id: item,
    //         nbt: None,
    //     };
    // }
    //
    // pub fn new_single(item: ItemRegistry) -> Self {
    //     return ItemStack {
    //         count: 1,
    //         id: item,
    //         nbt: None,
    //     };
    // }
}

// Identifier

pub struct Identifier {
    namespace: String,
    pub key: String,
}

impl Identifier {
    pub fn new_minecraft(key: &str) -> Self {
        return Identifier { namespace: "minecraft".to_owned(), key: key.to_owned() };
    }
    pub fn new_other(namespace: &str, key: &str) -> Self {
        return Identifier { namespace: namespace.to_owned(), key: key.to_owned() };
    }
    pub fn from_format(identifier: String) -> std::io::Result<Self> {
        let split = identifier.split(':').collect::<Vec<&str>>();
        return match split.len() {
            1 => Ok(Identifier { namespace: "minecraft".to_owned(), key: split[1].to_owned() }),
            2 => Ok(Identifier { namespace: split[0].to_owned(), key: split[1].to_owned() }),
            _ => Err(Error::new(ErrorKind::InvalidData, "Could not split identifer into 2 distince parts")),
        };
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return f.write_fmt(format_args!("{}:{}", self.namespace, self.key));
    }
}

// EntityMetadata

pub struct EntityMetadata {
    data: HashMap<u8, Box<EntityMetadataEntry>>,
}

const END_OF_ARRAY: u8 = 0xFF;

pub enum EntityMetadataEntry {
    Byte(i8),
    VarInt(i32),
    Float(f32),
    String(String),
    Chat(ChatComponent),
    OptionChat(Option<ChatComponent>),
    ItemStack(ItemStack),
    Boolean(bool),
    Rotation(Rotation),
    Position(Position),
    OptionPosition(Option<Position>),
    Direction(i32 /*VarInt*/),
    OptionUUID(Option<u128>),
    BlockState(Box<dyn BlockState>),
    NBT(Value),
    Particle(Particle),
    VillagerData(VillagerData),
    OptionVarInt(i32),
    Pose(i32 /*VarInt*/),
}

impl EntityMetadataEntry {
    pub const fn get_id(&self) -> i32 {
        return match self {
            EntityMetadataEntry::Byte(_) => 0,
            EntityMetadataEntry::VarInt(_) => 1,
            EntityMetadataEntry::Float(_) => 2,
            EntityMetadataEntry::String(_) => 3,
            EntityMetadataEntry::Chat(_) => 4,
            EntityMetadataEntry::OptionChat(_) => 5,
            EntityMetadataEntry::ItemStack(_) => 6,
            EntityMetadataEntry::Boolean(_) => 7,
            EntityMetadataEntry::Rotation(_) => 8,
            EntityMetadataEntry::Position(_) => 9,
            EntityMetadataEntry::OptionPosition(_) => 10,
            EntityMetadataEntry::Direction(_) => 11,
            EntityMetadataEntry::OptionUUID(_) => 12,
            EntityMetadataEntry::BlockState(_) => 13,
            EntityMetadataEntry::NBT(_) => 14,
            EntityMetadataEntry::Particle(_) => 15,
            EntityMetadataEntry::VillagerData(_) => 16,
            EntityMetadataEntry::OptionVarInt(_) => 17,
            EntityMetadataEntry::Pose(_) => 18,
        };
    }
}

// Chat

pub enum ChatColor {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
    Reset,
    Hex([u8; 3]),
}

impl Serialize for ChatColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return match self {
            Self::Black => serializer.serialize_str("black"),
            Self::DarkBlue => serializer.serialize_str("dark_blue"),
            Self::DarkGreen => serializer.serialize_str("dark_green"),
            Self::DarkAqua => serializer.serialize_str("dark_aqua"),
            Self::DarkRed => serializer.serialize_str("dark_red"),
            Self::DarkPurple => serializer.serialize_str("dark_purple"),
            Self::Gold => serializer.serialize_str("gold"),
            Self::Gray => serializer.serialize_str("gray"),
            Self::DarkGray => serializer.serialize_str("dark_gray"),
            Self::Blue => serializer.serialize_str("blue"),
            Self::Green => serializer.serialize_str("green"),
            Self::Aqua => serializer.serialize_str("aqua"),
            Self::Red => serializer.serialize_str("red"),
            Self::LightPurple => serializer.serialize_str("light_purple"),
            Self::Yellow => serializer.serialize_str("yellow"),
            Self::White => serializer.serialize_str("white"),
            Self::Reset => serializer.serialize_str("reset"),
            Self::Hex(hex) => serializer.serialize_str(&format!("#{:02X}{:02X}{:02X}", hex[0], hex[1], hex[2])),
        };
    }
}

pub enum ClickAction {
    OpenUrl,
    OpenFile,
    RunCommand,
    SuggestCommand,
    ChangePage,
    CopyToClipboard,
}

impl Serialize for ClickAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return match self {
            Self::OpenUrl => serializer.serialize_str("open_url"),
            Self::OpenFile => serializer.serialize_str("open_file"),
            Self::RunCommand => serializer.serialize_str("run_command"),
            Self::SuggestCommand => serializer.serialize_str("suggest_command"),
            Self::ChangePage => serializer.serialize_str("change_page"),
            Self::CopyToClipboard => serializer.serialize_str("copy_to_clipboard"),
        };
    }
}

#[derive(Serialize)]
pub struct ClickEvent {
    action: ClickAction,
    value: String,
}

pub enum HoverAction {
    ShowText,
    ShowItem,
    ShowEntity,
}

impl Serialize for HoverAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return match self {
            Self::ShowText => serializer.serialize_str("show_text"),
            Self::ShowItem => serializer.serialize_str("show_item"),
            Self::ShowEntity => serializer.serialize_str("show_entity"),
        };
    }
}

#[derive(Serialize)]
pub struct HoverItem {
    id: String,
    count: Option<u8>,
    tag: Value,
}

#[derive(Serialize)]
pub struct HoverEntity {
    name: Option<ChatComponent>,
    r#type: String,
    id: String, // This is a UUID
}

#[derive(Serialize)]
pub struct HoverContents {
    show_text: Option<ChatComponent>,
    show_item: Option<HoverItem>,
    show_entity: Option<HoverEntity>,
}

#[derive(Serialize)]
pub struct HoverEvent {
    action: HoverAction,
    contents: String,
}

#[derive(Serialize)]
pub struct TextChatComponent {
    // Content
    text: String,

    // Format
    color: Option<ChatColor>,
    //font: Option<Font>,
    bold: Option<bool>,
    italic: Option<bool>,
    underlined: Option<bool>,
    strikethrough: Option<bool>,
    obfuscated: Option<bool>,

    // Interaction
    insertion: Option<String>,
    click_event: Option<ClickEvent>,
    hover_event: Option<HoverEvent>,

    // Children
    extra: Option<Vec<ChatComponent>>,
}

#[derive(Serialize)]
pub struct KeybindChatComponent {
    // Content
    keybind: String,

    // Format
    color: Option<ChatColor>,
    //font: Option<Font>,
    bold: Option<bool>,
    italic: Option<bool>,
    underlined: Option<bool>,
    strikethrough: Option<bool>,
    obfuscated: Option<bool>,

    // Interaction
    insertion: Option<String>,
    click_event: Option<ClickEvent>,
    hover_event: Option<HoverEvent>,

    // Children
    extra: Option<Vec<ChatComponent>>,
}

// Generalized

pub enum ChatComponent {
    Text(TextChatComponent),
    Keybind(KeybindChatComponent),
    // Selector(SelectorChatComponent),
    // Score(ScoreChatComponent),
    // Translate(TranslateChatComponent),
}

impl Serialize for ChatComponent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return match self {
            ChatComponent::Text(component) => component.serialize(serializer),
            ChatComponent::Keybind(component) => component.serialize(serializer),
        };
    }
}
