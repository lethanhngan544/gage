#[derive(Debug, Clone)]
pub enum CVarValue {
    Int(i32),
    Float(f32),
    Str(String),
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

pub struct CVar {
    pub name: String,
    pub value: CVarValue,
}

pub type Subscriber = Box<dyn Fn(Event) + Send + Sync>;

#[derive(Debug, Clone)]
pub enum Event {
    // Renderer-related
    CreateBuffer { data: Vec<u8> },
    BufferCreated { id: u32 },

    // Scene-related
    SpawnStaticModel { path: String },
    StaticModelReady { id: u32 },

    // Logging/UI
    Log { level: log::Level, message: String },

    // Window 
    MouseMoved(u32, u32),
    WindowResized(u32, u32)
}