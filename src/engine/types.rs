use std::collections::{VecDeque, HashMap};

#[derive(Debug, Clone)]
pub enum CVarValue {
    Int(i32),
    Float(f32),
    Str(String),
}

impl CVarValue {
    pub fn as_int(&self) -> i32 {
        if let CVarValue::Int(v) = self {
            *v
        } else {
            panic!("Tried to get Int from {:?}", self);
        }
    }

    pub fn as_float(&self) -> f32 {
        if let CVarValue::Float(v) = self {
            *v
        } else {
            panic!("Tried to get Float from {:?}", self);
        }
    }

    pub fn as_str(&self) -> &str {
        if let CVarValue::Str(v) = self {
            v
        } else {
            panic!("Tried to get Str from {:?}", self);
        }
    }
}

impl std::fmt::Display for CVarValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CVarValue::Int(v) => write!(f, "{}", v),
            CVarValue::Float(v) => write!(f, "{}", v),
            CVarValue::Str(v) => write!(f, "{}", v),
        }
    }
}

pub type EngineCvar = HashMap<String, CVarValue>;
pub type MessageQueue = VecDeque<Message>;

#[derive(Debug, Clone)]
pub enum Message {

    //Renderer
    LoadStaticModel { path: String },
    StaticModelReady { id: uuid::Uuid },

    // Logging/UI
    Log { level: log::Level, message: String },

    // Window 
    MouseMoved(u32, u32),
    WindowResized(u32, u32),
    // Keyboard
    KeyPressed(u32, u32), // key, scancode
    KeyReleased(u32, u32)
}