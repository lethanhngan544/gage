use crate::engine::types::CVar;
use crate::engine::types::CVarValue;

use std::collections::HashMap;
use log;

//Will be shared between all systems
pub struct State {
    cvars: HashMap<String, CVar>,
   
}

impl State {
    pub fn new() -> Self {
        Self { 
            cvars: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, value: CVarValue)
    {
        log::info!("Cvar registered: {}, with value: {}", name, &value);
        self.cvars.insert(name.to_string(), CVar {
            name: name.to_string(),
            value,
        });
    }

    pub fn set(&mut self, name: &str, value: CVarValue) {

        if let Some(var) = self.cvars.get_mut(name) {
            var.value = value.clone();
            log::warn!("Cvar mutated: {}, with value: {:?}", name, value);
        } else {
            log::error!("CVar '{}' not found!", name);
        }
    }

    pub fn get(&self, name: &str) -> &CVarValue {
        self.cvars
            .get(name)
            .map(|v| &v.value)
            .expect(format!("Unknown cvar: {}, register it first !", name).as_str())
    }

   

}


