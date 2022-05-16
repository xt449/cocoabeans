use serde::{Serialize, Serializer};
use macros::json::Json;

pub enum Color {
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
    Hex([u8; 3])
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
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
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
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
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
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
    tag: Json, // TODO: NBT
}

#[derive(Serialize)]
pub struct HoverEntity {
    name: Option<ChatComponent>,
    r#type: String,
    id: String, // TODO: UUID
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
    color: Option<Color>,
    //font: Option<Font>,
    bold: Option<bool>,
    italic: Option<bool>,
    underlined: Option<bool>,
    strikethrough: Option<bool>,
    obfuscated: Option<bool>,
    
    // Interaction
    insertion: Option<String>,
    clickEvent: Option<ClickEvent>,
    hoverEvent: Option<HoverEvent>,

    // Children
    extra: Option<Vec<ChatComponent>>
}

#[derive(Serialize)]
pub struct KeybindChatComponent {
    // Content
    keybind: String,

    // Format
    color: Option<Color>,
    //font: Option<Font>,
    bold: Option<bool>,
    italic: Option<bool>,
    underlined: Option<bool>,
    strikethrough: Option<bool>,
    obfuscated: Option<bool>,

    // Interaction
    insertion: Option<String>,
    clickEvent: Option<ClickEvent>,
    hoverEvent: Option<HoverEvent>,

    // Children
    extra: Option<Vec<ChatComponent>>
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
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        return match self {
            ChatComponent::Text(component) => component.serialize(serializer),
            ChatComponent::Keybind(component) => component.serialize(serializer),
        };
    }
}
